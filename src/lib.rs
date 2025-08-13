pub fn compare_peq<T: PartialEq + std::fmt::Debug>(label: &str, a: T, b: T) -> bool {
    let eq = a == b;
    println!("[{}] {:?} == {:?} -> {}", label, a, b, eq);
    eq
}

// requires a full equivalence relation (reflexive, symmetric, transitive)
pub fn compare_eq<T: Eq + std::fmt::Debug>(label: &str, a: T, b: T) -> bool {
    let eq = a == b;
    println!("[{}] {:?} == {:?} -> {}", label, a, b, eq);
    eq
}
