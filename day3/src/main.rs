#![allow(dead_code)]

use regex;
use std::cmp;
use std::env;
use std::fs;

#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
struct Range {
    from: i32,
    to: i32,
}

impl Range {
    pub fn new(from: i32, to: i32) -> Range {
        Range {
            from: cmp::min(from, to),
            to: cmp::max(from, to),
        }
    }
}

fn intersects(range1: &Range, range2: &Range) -> Option<i32> {
    if range1.to < range2.from {
        None
    } else if range1.from > range2.to {
        None
    } else {
        let from = cmp::max(range1.from, range2.from);
        let to = cmp::min(range1.to, range2.to);
        if from.signum() == to.signum() {
            Some(cmp::min(from.abs(), to.abs()))
        } else {
            Some(0)
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct HorzLine {
    x: Range,
    y: i32,
}

impl HorzLine {
    pub fn new(x: Range, y: i32) -> HorzLine {
        HorzLine { x, y }
    }
}

#[derive(Debug)]
struct VertLine {
    x: i32,
    y: Range,
}

impl VertLine {
    pub fn new(x: i32, y: Range) -> VertLine {
        VertLine { x, y }
    }
}

fn intersects_hv(horz_line: &HorzLine, vert_line: &VertLine) -> Option<i32> {
    if !(horz_line.y < vert_line.y.from || horz_line.y > vert_line.y.to)
        && !(vert_line.x < horz_line.x.from || vert_line.x > horz_line.x.to)
    {
        Some(vert_line.x.abs() + horz_line.y.abs())
    } else {
        None
    }
}

#[derive(Debug)]
enum Move {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

impl Move {
    pub fn parse(s: &str) -> Move {
        lazy_static! {
            static ref MOVE_RE: regex::Regex =
                regex::Regex::new(r"(?P<d>[UDLR]{1})(?P<s>[0-9]*)").unwrap();
        }
        let captures = MOVE_RE.captures(s).unwrap();
        let steps = captures.name("s").unwrap().as_str();
        let steps = steps.parse::<i32>().unwrap();
        match captures.name("d").unwrap().as_str() {
            "U" => Move::Up(steps),
            "D" => Move::Down(steps),
            "L" => Move::Left(steps),
            "R" => Move::Right(steps),
            s => panic!("Unexpected move string {}", s),
        }
    }
}

fn parse_moves(line: &str) -> Vec<Move> {
    line.trim_end()
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| Move::parse(s))
        .collect::<Vec<Move>>()
}

#[derive(Debug)]
struct Lines {
    pub horz_lines: Vec<HorzLine>,
    pub vert_lines: Vec<VertLine>,
}

impl Lines {
    pub fn new(moves: &[Move]) -> Lines {
        let mut cursor = Point::new(0, 0);
        let mut horz_lines = Vec::<HorzLine>::new();
        let mut vert_lines = Vec::<VertLine>::new();
        for mov in moves.iter() {
            match mov {
                Move::Up(dist_y) => {
                    vert_lines.push(VertLine::new(
                        cursor.x,
                        Range::new(cursor.y, cursor.y + dist_y),
                    ));
                    cursor.y += dist_y;
                }
                Move::Right(dist_x) => {
                    horz_lines.push(HorzLine::new(
                        Range::new(cursor.x, cursor.x + dist_x),
                        cursor.y,
                    ));
                    cursor.x += dist_x;
                }
                Move::Down(dist_y) => {
                    vert_lines.push(VertLine::new(
                        cursor.x,
                        Range::new(cursor.y, cursor.y - dist_y),
                    ));
                    cursor.y -= dist_y;
                }
                Move::Left(dist_x) => {
                    horz_lines.push(HorzLine::new(
                        Range::new(cursor.x, cursor.x - dist_x),
                        cursor.y,
                    ));
                    cursor.x -= dist_x;
                }
            }
        }
        Lines {
            horz_lines,
            vert_lines,
        }
    }
}

fn find_closest_intersection_hv(horz_lines: &[HorzLine], vert_lines: &[VertLine]) -> Option<i32> {
    let mut closest_distance: Option<i32> = None;
    for line1 in horz_lines.iter() {
        for line2 in vert_lines.iter() {
            if let Some(intersection_distance) = intersects_hv(line1, line2) {
                if intersection_distance > 0 {
                    closest_distance = Some(cmp::min(
                        closest_distance.unwrap_or(intersection_distance),
                        intersection_distance,
                    ))
                }
            }
        }
    }
    closest_distance
}

fn dist_to_closest_intersection(path1: &[Move], path2: &[Move]) -> Option<i32> {
    let lines1 = Lines::new(path1);
    let lines2 = Lines::new(path2);

    let distances = [
        find_closest_intersection_hv(&lines1.horz_lines, &lines2.vert_lines),
        find_closest_intersection_hv(&lines2.horz_lines, &lines1.vert_lines),
    ];

    let closest = distances.iter().min_by(|a, b| {
        if let Some(a) = a {
            if let Some(b) = b {
                a.cmp(b)
            } else {
                cmp::Ordering::Less
            }
        } else if let Some(_b) = b {
            cmp::Ordering::Greater
        } else {
            cmp::Ordering::Equal
        }
    });
    *closest.unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day3 input-filename");

    let contents = fs::read_to_string(filename).expect("Failed to read file");
    let moves: Vec<Vec<Move>> = contents.lines().map(|line| parse_moves(line)).collect();

    let dist = dist_to_closest_intersection(&moves[0][..], &moves[1][..]).unwrap();
    println!("Distance to closest intersection: {}", dist);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_example_1() {
        let path1 = [Move::Right(8), Move::Up(5), Move::Left(5), Move::Down(3)];
        let path2 = [Move::Up(7), Move::Right(6), Move::Down(4), Move::Left(4)];
        assert_eq!(dist_to_closest_intersection(&path1, &path2), Some(6));
    }

    #[test]
    fn verify_example_2() {
        let path1 = [
            Move::Right(75),
            Move::Down(30),
            Move::Right(83),
            Move::Up(83),
            Move::Left(12),
            Move::Down(49),
            Move::Right(71),
            Move::Up(7),
            Move::Left(72),
        ];
        let path2 = [
            Move::Up(62),
            Move::Right(66),
            Move::Up(55),
            Move::Right(34),
            Move::Down(71),
            Move::Right(55),
            Move::Down(58),
            Move::Right(83),
        ];
        assert_eq!(dist_to_closest_intersection(&path1, &path2), Some(159));
    }

    #[test]
    fn verify_example_3() {
        let path1 = [
            Move::Right(98),
            Move::Up(47),
            Move::Right(26),
            Move::Down(63),
            Move::Right(33),
            Move::Up(87),
            Move::Left(62),
            Move::Down(20),
            Move::Right(33),
            Move::Up(53),
            Move::Right(51),
        ];
        let path2 = [
            Move::Up(98),
            Move::Right(91),
            Move::Down(20),
            Move::Right(16),
            Move::Down(67),
            Move::Right(40),
            Move::Up(7),
            Move::Right(15),
            Move::Up(6),
            Move::Right(7),
        ];
        assert_eq!(dist_to_closest_intersection(&path1, &path2), Some(135));
    }

    #[test]
    fn empty_range_intersects_when_equal() {
        assert_eq!(intersects(&Range::new(0, 0), &Range::new(0, 0)), Some(0));
        assert_eq!(intersects(&Range::new(0, 0), &Range::new(1, 1)), None);
    }

    #[test]
    fn disjoint_ranges_do_not_intersect() {
        assert_eq!(intersects(&Range::new(0, 1), &Range::new(2, 3)), None);
        assert_eq!(intersects(&Range::new(2, 3), &Range::new(0, 1)), None);
    }

    #[test]
    fn overlapping_ranges_intersect() {
        assert_eq!(intersects(&Range::new(0, 2), &Range::new(1, 3)), Some(1));
        assert_eq!(intersects(&Range::new(1, 3), &Range::new(0, 2)), Some(1));
    }

    #[test]
    fn enclosing_ranges_intersects() {
        assert_eq!(intersects(&Range::new(0, 3), &Range::new(1, 2)), Some(1));
        assert_eq!(intersects(&Range::new(1, 2), &Range::new(0, 3)), Some(1));
    }
}
