use scx_truther::astrology::Astrology;
use scx_truther::numerology::LifePath;
use scx_truther::runes::RuneCast;
use scx_truther::tarot::TarotReading;
use scx_truther::truth::Mode;
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

#[test]
fn tarot_draw_is_deterministic_for_seed() {
    let mut r1 = Rng::new(7);
    let mut r2 = Rng::new(7);

    let t1 = TarotReading::draw(&mut r1);
    let t2 = TarotReading::draw(&mut r2);

    assert_eq!(t1.index(), t2.index());
    assert_eq!(t1.reversed(), t2.reversed());
}

#[test]
fn rune_cast_is_deterministic_for_seed() {
    let mut r1 = Rng::new(99);
    let mut r2 = Rng::new(99);

    assert_eq!(
        RuneCast::cast(&mut r1).index(),
        RuneCast::cast(&mut r2).index()
    );
}

#[test]
fn mode_parsing_accepts_known_names_and_rejects_unknown() {
    assert_eq!(Mode::parse("tarot"), Some(Mode::Tarot));
    assert_eq!(Mode::parse("8ball"), Some(Mode::EightBall));
    assert_eq!(Mode::parse("astrology-please"), None);
}
