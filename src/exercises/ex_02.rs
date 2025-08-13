use eq_trait::*;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

pub fn run() {
    let p1 = Point { x: 1, y: 4 };
    let p2 = Point { x: 1, y: 4 };
    let p3 = p2.clone();

    compare_peq("02: 01", &p1, &p2);
    compare_eq("02: 02", &p2, &p1);
    compare_peq("02: 03", &p1, &p3);
}
