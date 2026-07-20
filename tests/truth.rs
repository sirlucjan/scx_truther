use scx_truther::astrology::Astrology;
use scx_truther::numerology::LifePath;
use scx_truther::util::Rng;

#[test]
fn astrology_is_deterministic_for_seed() {
    let a1 = Astrology::from_seed(42);
    let a2 = Astrology::from_seed(42);

    assert_eq!(a1.index(), a2.index());
}

#[test]
fn rng_is_deterministic_for_seed() {
    let mut r1 = Rng::new(1234);
    let mut r2 = Rng::new(1234);

    assert_eq!(r1.next_u64(), r2.next_u64());
    assert_eq!(r1.next_u64(), r2.next_u64());
}

#[test]
fn life_path_preserves_master_numbers() {
    // 2 + 9 = 11, a master number that should not be reduced further.
    assert_eq!(LifePath::from_pid(29).number(), 11);
    // 4 + 4 = 8, an ordinary number that should reduce normally.
    assert_eq!(LifePath::from_pid(44).number(), 8);
}
