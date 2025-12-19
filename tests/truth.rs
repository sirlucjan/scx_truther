use scx_truther::truth::Truth;

#[test]
fn zodiac_is_stable_for_seed() {
    let t1 = Truth::new(42);
    let t2 = Truth::new(42);

    assert_eq!(t1.index(), t2.index());
}
