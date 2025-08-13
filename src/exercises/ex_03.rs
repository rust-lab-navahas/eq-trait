use eq_trait::*;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct PointF {
    x: f32,
    y: f32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x, self.y) == (other.x, other.y) && (self.x != 0 || self.y != 0)
    }
}

impl PartialEq for PointF {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn run() {
    let p1 = Point { x: 1, y: 3 };
    let p2 = Point { x: 0, y: 0 };

    compare_peq("03: 01", &p1, &p1);
    compare_peq("03: 02", &p2, &p2);

    let fp1 = PointF { x: 1.1, y: 0.1 };
    let fp2 = PointF { x: 1.1, y: 0.1 };
    let fp3 = PointF {
        x: f32::NAN,
        y: f32::NAN,
    };

    compare_peq("03: 03", &fp1, &fp2);
    compare_peq("03: 04", &fp1, &fp3);
    compare_peq("03: 05", fp3.clone(), fp3);
}
