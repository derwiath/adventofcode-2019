use std::env;
use std::fs;

struct Range {
    from: i32,
    to: i32,
}

impl Range {
    #[allow(dead_code)]
    pub fn new(from: i32, to: i32) -> Range {
        Range { from, to }
    }
}

#[allow(dead_code)]
fn intersects(range1: Range, range2: Range) -> bool {
    !(range1.to < range2.from || range1.from > range2.to)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day3 input-filename");

    let contents = fs::read_to_string(filename).expect("Failed to read file");
    println!("Input contains {} lines", contents.lines().count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_range_intersects_when_equal() {
        assert_eq!(intersects(Range::new(0, 0), Range::new(0, 0)), true);
        assert_eq!(intersects(Range::new(0, 0), Range::new(1, 1)), false);
    }

    #[test]
    fn disjoint_ranges_do_not_intersect() {
        assert_eq!(intersects(Range::new(0, 1), Range::new(2, 3)), false);
        assert_eq!(intersects(Range::new(2, 3), Range::new(0, 1)), false);
    }

    #[test]
    fn overlapping_ranges_intersect() {
        assert_eq!(intersects(Range::new(0, 2), Range::new(1, 3)), true);
        assert_eq!(intersects(Range::new(1, 3), Range::new(0, 2)), true);
    }

    #[test]
    fn enclosing_ranges_intersects() {
        assert_eq!(intersects(Range::new(0, 3), Range::new(1, 2)), true);
        assert_eq!(intersects(Range::new(1, 2), Range::new(0, 3)), true);
    }
}
