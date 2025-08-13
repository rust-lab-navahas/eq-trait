use eq_trait::*;

pub fn _needs_eq<T: Eq>(_t: T) {}

pub fn run() {
    let x = std::f32::NAN;
    // false
    compare_peq("01: 01", x, x);

    // true
    let y = std::f32::INFINITY;
    compare_peq("01: 02", y, y);
    
    // true
    let bar = std::f32::MAX;
    compare_peq("01: 03", bar, bar);

    // true
    let foo: f32 = 2.32;
    compare_peq("01: 05", foo, foo);

    //needs_eq(x);
}
