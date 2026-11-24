//! Unit tests: table pins, construction identities, formatter pins,
//! and the crate-local half of the parse/format round-trip (the full
//! pin through the real text parser lives in editor-core's suite).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use proptest::prelude::*;

use crate::{
    Angle, CENTI, CM, DECIDING_FIGURES, DEG, Decimal, FmtQuantityError, IN, Length, M, MILLI, MM,
    ONE, PI, RAD, ROUND_TRIP_FIGURES, UNITS, UnitDef, UnitQuantity, WrittenAngle, WrittenLength,
    fmt_angle, fmt_length, unit_by_symbol,
};

/// The decimal `text` spells.
fn decimal(text: &str) -> Decimal {
    Decimal::parse(text).unwrap_or_else(|| panic!("{text:?} is a decimal"))
}

#[test]
fn the_unit_table_is_the_whole_closed_set_and_reads_as_data() {
    let symbols: Vec<&str> = UNITS.iter().map(|u| u.symbol()).collect();
    // The dimensionless row is LAST and its symbol is the empty string
    // — the notation of a number written with no suffix. Last so that
    // the rows a user can pick are a prefix of the table.
    assert_eq!(symbols, ["mm", "cm", "m", "in", "deg", "rad", "pi rad", ""]);
    assert_eq!(MM.factor(), MILLI);
    assert_eq!(MILLI, 1e-3);
    assert_eq!(CM.factor(), CENTI);
    assert_eq!(CENTI, 1e-2);
    assert_eq!(M.factor(), 1.0);
    assert_eq!(IN.factor(), 0.0254);
    assert_eq!(RAD.factor(), 1.0);
    // The float nearest π/180, pinned by bits.
    assert_eq!(DEG.factor(), 0.017_453_292_519_943_295_f64);
    // The half-turn row's float is π's, pinned the same way. Its symbol
    // is two words — the only such row — and the table carries exactly
    // that spelling.
    assert_eq!(PI.factor(), core::f64::consts::PI);
    assert_eq!(
        unit_by_symbol("pi rad").expect("pi rad row").quantity(),
        UnitQuantity::Angle
    );
    assert_eq!(
        unit_by_symbol("pi"),
        None,
        "the half-turn row is spelled `pi rad`; the table is closed over one spelling"
    );
    let mm = unit_by_symbol("mm").expect("mm row");
    assert_eq!(mm.quantity(), UnitQuantity::Length);
    assert_eq!(mm.factor(), MILLI);
    assert_eq!(
        unit_by_symbol("deg").expect("deg row").quantity(),
        UnitQuantity::Angle
    );
    assert_eq!(unit_by_symbol("furlong"), None);
    // Each row's float is what one of the unit is.
    for row in UNITS {
        assert_eq!(
            row.canonical(&decimal("1")).to_bits(),
            row.factor().to_bits(),
            "1 {}",
            row.symbol()
        );
    }
    assert_eq!(
        unit_by_symbol("MM"),
        None,
        "symbols are case-sensitive data"
    );
}

#[test]
fn unit_constant_construction_lands_on_canonical_units() {
    assert_eq!((25.0 * MM).meters(), 0.025);
    assert_eq!((25.0 * MM).meters(), (MM * 25.0).meters());
    assert_eq!((1.0 * M).meters(), 1.0);
    assert_eq!((1.0 * IN).meters(), 0.0254);
    assert_eq!((90.0 * DEG).radians(), core::f64::consts::FRAC_PI_2);
    assert_eq!((1.5 * RAD).radians(), 1.5);
    // The heatsink migration's three literals: prefixed-metre decimal
    // values whose f64 multiply lands on the exact dyadic the tour
    // hand-wrote in meters (the SAID change's bit-identity basis).
    assert_eq!((250.0 * MM).meters(), 0.25);
    assert_eq!((812.5 * MM).meters(), 0.8125);
    assert_eq!((312.5 * MM).meters(), 0.3125);
    // Display-side conversion.
    assert_eq!((25.0 * MM).in_unit(MM), 25.0);
}

#[test]
fn quantity_arithmetic_is_the_infallible_subset() {
    assert_eq!((1.0 * M + 50.0 * CM).meters(), 1.5);
    assert_eq!((1.0 * M - 250.0 * MM).meters(), 0.75);
    assert_eq!((-(1.0 * M)).meters(), -1.0);
    assert_eq!((2.0 * (1.0 * M) * 3.0).meters(), 6.0);
    assert_eq!(((1.0 * M) / 4.0).meters(), 0.25);
    assert!(90.0 * DEG < 2.0 * RAD);
    assert_eq!(crate::Count::new(5).get(), 5);
}

#[test]
fn formatter_pins_the_spec_surface_shape() {
    assert_eq!(fmt_length(0.25, MM).unwrap(), "250 mm");
    assert_eq!(fmt_length(0.025, MM).unwrap(), "25 mm");
    assert_eq!(fmt_length(0.25, M).unwrap(), "0.25 m");
    assert_eq!(fmt_length(0.0254, IN).unwrap(), "1 in");
    assert_eq!(fmt_length(-0.25, MM).unwrap(), "-250 mm");
    assert_eq!(fmt_length(0.0, MM).unwrap(), "0 mm");
    assert_eq!(fmt_angle(DEG.factor(), DEG).unwrap(), "1 deg");
    assert_eq!(fmt_angle(1.5, RAD).unwrap(), "1.5 rad");
    assert!(matches!(
        fmt_length(f64::NAN, MM),
        Err(FmtQuantityError::NonFinite { value }) if value.is_nan()
    ));
    assert!(matches!(
        fmt_angle(f64::INFINITY, DEG),
        Err(FmtQuantityError::NonFinite { value }) if value == f64::INFINITY
    ));
}

/// The crate-local mirror of the parser's literal semantics: the
/// decimal before the symbol, through the unit's one door. Kept in
/// lockstep with `editor-core`'s parser by the round-trip suite there.
fn parse_back(text: &str, symbol: &str) -> f64 {
    let (num, sym) = text.split_once(' ').expect("value then symbol");
    assert_eq!(sym, symbol, "the suffix of {text:?}");
    unit_by_symbol(sym)
        .expect("table row")
        .canonical(&decimal(num))
}

proptest! {
    #[test]
    // "Sampled" is honest (NOTE-2 of the PR #267 review): the DOMAIN
    // is all finite f64 × all seven FORMATTABLE units; a run samples
    // proptest's configured case count from it. The dimensionless row
    // is excluded because it has no formatter and no suffix — its
    // "text" is the bare number, which `expr`'s writer produces.
    fn fmt_round_trip_bit_exact_sampled_over_finite_values_and_units(
        value in proptest::num::f64::ANY.prop_filter("finite", |v| v.is_finite()),
        unit_idx in 0usize..7,
    ) {
        let u = UNITS[unit_idx];
        let text = match u.quantity() {
            UnitQuantity::Length => {
                fmt_length(value, u.as_length().expect("a Length row has a length view"))
                    .unwrap()
            }
            UnitQuantity::Angle => {
                fmt_angle(value, u.as_angle().expect("an Angle row has an angle view"))
                    .unwrap()
            }
            UnitQuantity::Scalar => unreachable!(
                "the dimensionless row is index {} and this generator stops before it",
                UNITS.len() - 1
            ),
        };
        let back = parse_back(&text, u.symbol());
        prop_assert_eq!(back.to_bits(), value.to_bits(), "text {:?}", text);
    }
}

/// A value the float multiply by `MM`'s factor stepped over, which had
/// no millimetre spelling under it, is said in millimetres: the
/// multiply is no door.
#[test]
fn a_value_a_float_multiply_steps_over_is_said_in_its_unit() {
    let f = MM.factor();
    let mut d = 1024.0f64;
    for _ in 0..200_000 {
        let (x1, x2) = (d * f, d.next_up() * f);
        if x2 > x1.next_up() {
            let skipped = x1.next_up();
            let text = fmt_length(skipped, MM).unwrap();
            assert_eq!(
                parse_back(&text, "mm").to_bits(),
                skipped.to_bits(),
                "{text}"
            );
            return;
        }
        d = d.next_up();
    }
    panic!("no 2-ulp step found in 200k quotients");
}

/// What a decimal written in a unit is, against exact arithmetic done
/// elsewhere (Python's `Fraction`, whose float is correctly rounded,
/// and π by Machin at 4000 bits): ties to even, both ends of the
/// format, signed zeros, decimals of more figures than a float holds,
/// and decimals from a seeded draw over the whole format.
#[test]
fn a_number_in_a_unit_is_its_decimal_times_the_exact_factor_rounded_once() {
    const DOOR: [(&str, &str, u64); 127] = [
        ("300.04", "mm", 0x3fd333daf8df7a4e),
        ("0.30004", "m", 0x3fd333daf8df7a4e),
        ("0.13", "mm", 0x3f210a137f38c543),
        ("25", "mm", 0x3f9999999999999a),
        ("250", "mm", 0x3fd0000000000000),
        ("812.5", "mm", 0x3fea000000000000),
        ("312.5", "mm", 0x3fd4000000000000),
        ("1", "in", 0x3f9a027525460aa6),
        ("2.5", "in", 0x3fb04189374bc6a8),
        ("2.5", "mm", 0x3f647ae147ae147b),
        ("2.5", "cm", 0x3f9999999999999a),
        ("2.5", "deg", 0x3fa657184ae74487),
        ("2.5", "pi rad", 0x401f6a7a2955385e),
        ("3", "mm", 0x3f689374bc6a7efa),
        ("3", "cm", 0x3f9eb851eb851eb8),
        ("3", "in", 0x3fb381d7dbf487fd),
        ("3", "deg", 0x3faacee9f37bebd6),
        ("3", "pi rad", 0x4022d97c7f3321d2),
        ("1", "deg", 0x3f91df46a2529d39),
        ("30", "deg", 0x3fe0c152382d7366),
        ("90", "deg", 0x3ff921fb54442d18),
        ("180", "deg", 0x400921fb54442d18),
        ("360", "deg", 0x401921fb54442d18),
        ("45", "deg", 0x3fe921fb54442d18),
        ("1", "pi rad", 0x400921fb54442d18),
        ("0.5", "pi rad", 0x3ff921fb54442d18),
        ("2", "pi rad", 0x401921fb54442d18),
        ("-0.25", "pi rad", 0xbfe921fb54442d18),
        ("1e309", "mm", 0x7f76c8e5ca239029),
        ("1e400", "mm", 0x7ff0000000000000),
        ("-1e400", "deg", 0xfff0000000000000),
        ("1e-330", "pi rad", 0x0000000000000000),
        ("4.94e-321", "mm", 0x0000000000000001),
        ("5e-324", "m", 0x0000000000000001),
        ("2.4703282292062328e-324", "m", 0x0000000000000001),
        ("2.4703282292062327e-324", "m", 0x0000000000000000),
        ("1.7976931348623157e311", "mm", 0x7fefffffffffffff),
        ("1.7976931348623158e311", "mm", 0x7fefffffffffffff),
        (
            "1.79769313486231580793728971405301e311",
            "mm",
            0x7fefffffffffffff,
        ),
        (
            "1000.00000000000011102230246251565404236316680908203125",
            "mm",
            0x3ff0000000000000,
        ),
        (
            "1000.000000000000111022302462515654042363166809082031251",
            "mm",
            0x3ff0000000000001,
        ),
        (
            "1000.00000000000011102230246251565404236316680908203124",
            "mm",
            0x3ff0000000000000,
        ),
        (
            "1000.00000000000033306690738754696212708950042724609375",
            "mm",
            0x3ff0000000000002,
        ),
        (
            "179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497792000",
            "mm",
            0x7ff0000000000000,
        ),
        (
            "179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791999",
            "mm",
            0x7fefffffffffffff,
        ),
        ("-0", "mm", 0x8000000000000000),
        ("0", "deg", 0x0000000000000000),
        ("31250919908e69", "mm", 0x4fd145d934d7ee91),
        ("521610615998048273e261", "deg", 0x79706f25baf75f41),
        ("-1629072e-247", "in", 0x8d9c40ddd2ae25b5),
        ("-346e-243", "mm", 0x8d62e668982f7369),
        ("9738681121152269347e-107", "deg", 0x2d4bb2fa7af1d400),
        ("1457380681191578132e-109", "deg", 0x2cb5391dbb06c908),
        ("47959e94", "deg", 0x540f599fd34bc1f2),
        ("6054e238", "mm", 0x71829818d5358970),
        ("-6892936625691456361e-143", "mm", 0xa587e3c9a517ebe9),
        ("172390762004538402e242", "deg", 0x756409daa56501e8),
        ("-9328453e209", "pi rad", 0xece100a35ef529a7),
        ("-89309156112e129", "m", 0xdcfdff96a34be75f),
        ("34127884e-86", "deg", 0x2f4699b1ddc2f655),
        ("-7550669089e16", "m", 0xc54f3a94f51eee9d),
        ("-11350317716114554168e189", "cm", 0xeab6a04c68552b96),
        ("67934655362e-295", "pi rad", 0x0509639b6ebdd629),
        ("-492134380942901708e23", "deg", 0xc804318944acc5b7),
        ("633788284861825e-269", "rad", 0x0b27ca772e956682),
        ("-8825107365e-44", "rad", 0xb8dd539230bdc947),
        ("488530022802e-163", "in", 0x2030a325dc3f0ea7),
        ("37e-41", "rad", 0x37c01da34ae7fdd7),
        ("-63404922e65", "mm", 0xce6d65d257df3ae7),
        ("-571007e76", "cm", 0xd07ed219ae605929),
        ("-87437676164617e-50", "m", 0xb872988f55e75f85),
        ("1777547250534441977855e-155", "cm", 0x23c08997e99fab0a),
        ("5575015054520851622378e268", "m", 0x7c16e202bff4d8fe),
        ("-1017581913e94", "deg", 0xd4f44cb4bf712c78),
        ("9790776747417176309e-207", "in", 0x1891ba749b0ffde0),
        ("-4699665557397008640419e72", "in", 0xd30d4ce00ddf93e2),
        ("-8889e314", "mm", 0xfff0000000000000),
        ("-2129905e-122", "cm", 0xa7812ff5216cb476),
        ("6571e280", "mm", 0x7a3cf5b528b4d47d),
        ("3e214", "m", 0x6c76474652aa8962),
        ("-76667805751908262684e-183", "m", 0x9e11a8f5ac22b9d2),
        ("-18746550756401336488e-217", "in", 0x968d2886557335f5),
        ("3808914984605323e-248", "mm", 0x0f0f00d801dce54f),
        ("47539522661e155", "deg", 0x61f70daa05a278fb),
        ("-4e205", "cm", 0xea3469ae42c8560c),
        ("-131177867312592195e205", "rad", 0xee0d082315a846fb),
        ("-812e-68", "cm", 0xb201835fda708f84),
        ("-348128583958e210", "rad", 0xedeed18f860dc41c),
        ("92306098e-136", "pi rad", 0x257419dd66181a85),
        ("-4514257086821e195", "m", 0xeb0c1f1deb41ed55),
        ("-1e-49", "m", 0xb5c2b50c6ec4f313),
        ("-6776075e122", "m", 0xdaaf481c09303147),
        ("325e-231", "in", 0x1029a1cf950ed8ad),
        ("-6666294e-126", "deg", 0xa6d33a6f7e0d61b0),
        ("65375434412351473771e17", "pi rad", 0x47aee710968f5dea),
        ("-4669237752600789647625e-131", "cm", 0xa8d1f76ef16f701e),
        ("99489032923039e5", "pi rad", 0x43fb1c17e0f98919),
        ("8062915493999e-249", "cm", 0x0e80cd17488d51a3),
        ("-13610e-181", "pi rad", 0x9b515387606dcecf),
        ("671485775095843714043e23", "deg", 0x48aae82f15f88110),
        ("124668690687993982e-321", "deg", 0x00787280311a4841),
        ("66860e-136", "mm", 0x2413704670dda6a1),
        ("328470563e-36", "pi rad", 0x3a547075a981ba66),
        ("-3392217998392035586e222", "pi rad", 0xf1f99251328281d3),
        ("-17982e27", "rad", 0xc66c5edf38cc36f2),
        ("-9340665568497953815e194", "pi rad", 0xec416eefc55ce450),
        ("86644052735996797e-180", "pi rad", 0x1e2f59938e58216f),
        ("306161765276562e288", "pi rad", 0x7ed670ef6a9eed49),
        ("-32589e-191", "deg", 0x98d9579f9af337e1),
        ("-2011e-2", "pi rad", 0xc04f96b5f826f80a),
        ("1930e-81", "m", 0x2fcc9af37b22ac77),
        ("-22e184", "deg", 0xe60ceabe0dd46ef3),
        ("-2e118", "deg", 0xd821b7da6a80da6c),
        ("-83810248240715456e-131", "in", 0xa7ead74da8a9a94f),
        ("-83174755739473539e-82", "deg", 0xb20f4f43d3b26199),
        ("-8508277e-195", "mm", 0x984368c4e38b7fdc),
        ("6559586563763e-261", "in", 0x0c131625a5425306),
        ("317e-25", "pi rad", 0x3b5e194935e34a6d),
        ("94339e39", "m", 0x4910ebd59d996b42),
        ("71307e-111", "in", 0x299103c36bc1051c),
        ("9915966258639269e-106", "rad", 0x2d4028cac4d39fda),
        ("-82567741975268e78", "in", 0xd2b0790467c83bd0),
        ("6983003e-9", "rad", 0x3f7c9a359834b70e),
        ("-469450866005e232", "in", 0xf231e1e914ce0ac9),
        ("-7e4", "deg", 0xc09316ec02011dcf),
        ("215e-101", "mm", 0x2ad34305a363c208),
        ("391972375e-295", "m", 0x0477dfd48851cbf4),
    ];
    for (text, symbol, want) in DOOR {
        let unit = unit_by_symbol(symbol).expect("a table row");
        let got = unit.canonical(&decimal(text));
        assert_eq!(
            got.to_bits(),
            want,
            "{text} {symbol}: {got:e} against {:e}",
            f64::from_bits(want)
        );
    }
    // A half-turn is π's float said three ways.
    let pi = core::f64::consts::PI.to_bits();
    assert_eq!((180.0 * DEG).radians().to_bits(), pi, "180 deg");
    assert_eq!((1.0 * PI).radians().to_bits(), pi, "1 pi rad");
    // And a float is read as its shortest decimal: `300.04 * MM` is
    // what `300.04 mm` is, where the float multiply lands one over.
    assert_eq!((300.04 * MM).meters(), 0.300_04);
}

/// A decimal of more figures than can decide its rounding is read as
/// its first [`DECIDING_FIGURES`] and a sticky rest, against exact
/// arithmetic done elsewhere: ties at one metre and one thousand
/// millimetres, where the 800th figure (the last kept) or the 801st
/// (the first dropped) is the one that decides; and a thousand-figure
/// degree decimal that agrees with a rounding boundary through all its
/// figures, which no head of 800 figures decides.
#[test]
fn a_long_decimal_is_decided_by_its_deciding_figures() {
    // 1 + 2^-53, whose even neighbour is 1; and 1 + 3·2^-53, whose even
    // neighbour is 1 + 2^-51.
    let low_tie = "100000000000000011102230246251565404236316680908203125";
    let high_tie = "100000000000000033306690738754696212708950042724609375";
    let pad = DECIDING_FIGURES - low_tie.len();
    let below_high_tie = format!("{}4{}1", &high_tie[..high_tie.len() - 1], "9".repeat(pad));
    for (unit, shift) in [(M.def(), 0), (MM.def(), 3)] {
        let e = -53 + shift;
        let pad_e = pad as i64;
        for (text, want) in [
            (format!("{low_tie}e{e}"), 0x3ff0_0000_0000_0000_u64),
            (
                format!("{low_tie}{}1e{}", "0".repeat(pad), e - pad_e - 1),
                0x3ff0_0000_0000_0001,
            ),
            (
                format!("{below_high_tie}e{}", e - pad_e - 1),
                0x3ff0_0000_0000_0001,
            ),
            (
                format!("{high_tie}{}e{}", "0".repeat(pad), e - pad_e),
                0x3ff0_0000_0000_0002,
            ),
        ] {
            let number = decimal(&text);
            assert_eq!(
                unit.canonical(&number).to_bits(),
                want,
                "{} figures in {}",
                number.figures(),
                unit.symbol()
            );
        }
    }
    let near_a_boundary = decimal(DEG_NEAR_A_BOUNDARY);
    assert!(near_a_boundary.figures() > DECIDING_FIGURES);
    assert_eq!(
        DEG.def().canonical(&near_a_boundary).to_bits(),
        0x3fe0_c152_382d_7365
    );
}

/// A thousand-figure decimal of degrees that agrees with the midpoint
/// between the two floats beside thirty degrees, times 180/π, through
/// every figure (Python, π by Machin at 12 000 bits).
const DEG_NEAR_A_BOUNDARY: &str = "29.999999999999999890733344065359585131539391042633019363235952349336250314890346797641898303507789549444447461311432902501099307609702394647419527326352379107478243619992777347373585630059055324797483883989948801774554940086215256077606896806210522948065133322448296934861516751847914307310899767513218075467548580465590715973368656485769846690076504351116020537288456522278174071723919074369053092790207549438395225094593160326941140419936116546186957050493881371391071535605523382088190996271112073140752184442017925229867732539939468349341948033526275819949609954494172254375160882789921935272533688812880996683801319433594995142577158051446975579725169098732767166135299206032204141339351575950270627663403642768516220705502298628501938865622566371172492649533927502869963974247147495442984464835954876282063207077771448389366999758246633234000636309610955453434336636805759963615683744548660950428217773117542239752139727972716209070525680070225638894931642411018758812552477043646000778239571857";

/// A run of zeros between a short number and a far digit is not a half:
/// the figures that decide are positional, so a zero among the first
/// [`DECIDING_FIGURES`] is one of them.
#[test]
fn a_run_of_zeros_before_a_far_digit_is_not_a_half() {
    let far = format!("1.{}1", "0".repeat(1000));
    for unit in [M.def(), RAD.def(), ONE.def()] {
        assert_eq!(unit.canonical(&decimal(&far)), 1.0, "{}", unit.symbol());
    }
    let mm = format!("1000.{}1", "0".repeat(1000));
    assert_eq!(MM.def().canonical(&decimal(&mm)), 1.0);
    let nines = format!("0.3{}7", "0".repeat(900));
    assert_eq!(ONE.def().canonical(&decimal(&nines)), 0.3);
}

/// The shapes of long decimal that a misplaced deciding figure gets
/// wrong, in every unit, against exact arithmetic done elsewhere
/// (Python's `Fraction`, π by Machin at 30 000 bits): a short number (1,
/// 3, and the tie between 1 and the float after it), a run of zeros or
/// nines across the 800th figure or far past it, and a far digit of 1,
/// 5 or 9 or none.
#[test]
fn a_long_run_reads_as_its_exact_value_in_every_unit() {
    let mut want = RUNS.iter();
    for symbol in ["m", "mm", "rad", "", "in", "deg", "pi rad"] {
        let unit = unit_by_symbol(symbol).unwrap();
        for lead in [
            "1",
            "3",
            "1000000000000000111022302462515654042363166809082031",
        ] {
            for fill in ['0', '9'] {
                for n in [798_usize, 800, 801, 5000] {
                    for tail in ["1", "5", "9", ""] {
                        if tail.is_empty() && fill == '0' {
                            continue;
                        }
                        let e = -((lead.len() - 1 + n + tail.len()) as i64);
                        let text = format!("{lead}{}{tail}e{e}", fill.to_string().repeat(n));
                        let got = unit.canonical(&decimal(&text)).to_bits();
                        let Some(&expected) = want.next() else {
                            panic!("the table is shorter than the sweep")
                        };
                        assert_eq!(
                            got, expected,
                            "{lead}, {n} of {fill}, then {tail:?}, in {symbol:?}"
                        );
                    }
                }
            }
        }
    }
    assert!(want.next().is_none(), "the table is longer than the sweep");
}

/// In `in`, `deg` and `pi rad` a boundary is a repeating or irrational
/// decimal, and a decimal just above one, through 900 figures, is
/// decided on its full length: a door that read the 800-figure head
/// with a `5` after it (right only where the factor is a power of ten)
/// reads each of these one float low.
#[test]
fn a_decimal_just_past_a_boundary_is_read_past_it_where_the_factor_is_not_a_power_of_ten() {
    for (symbol, (text, want)) in [("in", NEAR_IN), ("deg", NEAR_DEG), ("pi rad", NEAR_PI_RAD)] {
        let number = decimal(text);
        assert!(number.figures() > DECIDING_FIGURES, "{symbol}");
        let unit = unit_by_symbol(symbol).unwrap();
        assert_eq!(unit.canonical(&number).to_bits(), want, "{symbol}");
    }
}

const RUNS: [u64; 588] = [
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f60624dd2f1a9fc,
    0x3f689374bc6a7efa,
    0x3f689374bc6a7efa,
    0x3f689374bc6a7efa,
    0x3f689374bc6a7efa,
    0x3f689374bc6a7efa,
    0x3f689374bc6a7efa,
    0x3f689374bc6a7efa,
    0x3f689374bc6a7efa,
    0x3f689374bc6a7efa,
    0x3f689374bc6a7efa,
    0x3f689374bc6a7efa,
    0x3f689374bc6a7efa,
    0x3f70624dd2f1a9fc,
    0x3f70624dd2f1a9fc,
    0x3f70624dd2f1a9fc,
    0x3f70624dd2f1a9fc,
    0x3f70624dd2f1a9fc,
    0x3f70624dd2f1a9fc,
    0x3f70624dd2f1a9fc,
    0x3f70624dd2f1a9fc,
    0x3f70624dd2f1a9fc,
    0x3f70624dd2f1a9fc,
    0x3f70624dd2f1a9fc,
    0x3f70624dd2f1a9fc,
    0x3f70624dd2f1a9fc,
    0x3f70624dd2f1a9fc,
    0x3f70624dd2f1a9fc,
    0x3f70624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3f50624dd2f1a9fc,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4000000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4008000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x4010000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000000,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3ff0000000000001,
    0x3f9a027525460aa6,
    0x3f9a027525460aa6,
    0x3f9a027525460aa6,
    0x3f9a027525460aa6,
    0x3f9a027525460aa6,
    0x3f9a027525460aa6,
    0x3f9a027525460aa6,
    0x3f9a027525460aa6,
    0x3f9a027525460aa6,
    0x3f9a027525460aa6,
    0x3f9a027525460aa6,
    0x3f9a027525460aa6,
    0x3faa027525460aa6,
    0x3faa027525460aa6,
    0x3faa027525460aa6,
    0x3faa027525460aa6,
    0x3faa027525460aa6,
    0x3faa027525460aa6,
    0x3faa027525460aa6,
    0x3faa027525460aa6,
    0x3faa027525460aa6,
    0x3faa027525460aa6,
    0x3faa027525460aa6,
    0x3faa027525460aa6,
    0x3faa027525460aa6,
    0x3faa027525460aa6,
    0x3faa027525460aa6,
    0x3faa027525460aa6,
    0x3fb381d7dbf487fd,
    0x3fb381d7dbf487fd,
    0x3fb381d7dbf487fd,
    0x3fb381d7dbf487fd,
    0x3fb381d7dbf487fd,
    0x3fb381d7dbf487fd,
    0x3fb381d7dbf487fd,
    0x3fb381d7dbf487fd,
    0x3fb381d7dbf487fd,
    0x3fb381d7dbf487fd,
    0x3fb381d7dbf487fd,
    0x3fb381d7dbf487fd,
    0x3fba027525460aa6,
    0x3fba027525460aa6,
    0x3fba027525460aa6,
    0x3fba027525460aa6,
    0x3fba027525460aa6,
    0x3fba027525460aa6,
    0x3fba027525460aa6,
    0x3fba027525460aa6,
    0x3fba027525460aa6,
    0x3fba027525460aa6,
    0x3fba027525460aa6,
    0x3fba027525460aa6,
    0x3fba027525460aa6,
    0x3fba027525460aa6,
    0x3fba027525460aa6,
    0x3fba027525460aa6,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f9a027525460aa7,
    0x3f91df46a2529d39,
    0x3f91df46a2529d39,
    0x3f91df46a2529d39,
    0x3f91df46a2529d39,
    0x3f91df46a2529d39,
    0x3f91df46a2529d39,
    0x3f91df46a2529d39,
    0x3f91df46a2529d39,
    0x3f91df46a2529d39,
    0x3f91df46a2529d39,
    0x3f91df46a2529d39,
    0x3f91df46a2529d39,
    0x3fa1df46a2529d39,
    0x3fa1df46a2529d39,
    0x3fa1df46a2529d39,
    0x3fa1df46a2529d39,
    0x3fa1df46a2529d39,
    0x3fa1df46a2529d39,
    0x3fa1df46a2529d39,
    0x3fa1df46a2529d39,
    0x3fa1df46a2529d39,
    0x3fa1df46a2529d39,
    0x3fa1df46a2529d39,
    0x3fa1df46a2529d39,
    0x3fa1df46a2529d39,
    0x3fa1df46a2529d39,
    0x3fa1df46a2529d39,
    0x3fa1df46a2529d39,
    0x3faacee9f37bebd6,
    0x3faacee9f37bebd6,
    0x3faacee9f37bebd6,
    0x3faacee9f37bebd6,
    0x3faacee9f37bebd6,
    0x3faacee9f37bebd6,
    0x3faacee9f37bebd6,
    0x3faacee9f37bebd6,
    0x3faacee9f37bebd6,
    0x3faacee9f37bebd6,
    0x3faacee9f37bebd6,
    0x3faacee9f37bebd6,
    0x3fb1df46a2529d39,
    0x3fb1df46a2529d39,
    0x3fb1df46a2529d39,
    0x3fb1df46a2529d39,
    0x3fb1df46a2529d39,
    0x3fb1df46a2529d39,
    0x3fb1df46a2529d39,
    0x3fb1df46a2529d39,
    0x3fb1df46a2529d39,
    0x3fb1df46a2529d39,
    0x3fb1df46a2529d39,
    0x3fb1df46a2529d39,
    0x3fb1df46a2529d39,
    0x3fb1df46a2529d39,
    0x3fb1df46a2529d39,
    0x3fb1df46a2529d39,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x3f91df46a2529d3a,
    0x400921fb54442d18,
    0x400921fb54442d18,
    0x400921fb54442d18,
    0x400921fb54442d18,
    0x400921fb54442d18,
    0x400921fb54442d18,
    0x400921fb54442d18,
    0x400921fb54442d18,
    0x400921fb54442d18,
    0x400921fb54442d18,
    0x400921fb54442d18,
    0x400921fb54442d18,
    0x401921fb54442d18,
    0x401921fb54442d18,
    0x401921fb54442d18,
    0x401921fb54442d18,
    0x401921fb54442d18,
    0x401921fb54442d18,
    0x401921fb54442d18,
    0x401921fb54442d18,
    0x401921fb54442d18,
    0x401921fb54442d18,
    0x401921fb54442d18,
    0x401921fb54442d18,
    0x401921fb54442d18,
    0x401921fb54442d18,
    0x401921fb54442d18,
    0x401921fb54442d18,
    0x4022d97c7f3321d2,
    0x4022d97c7f3321d2,
    0x4022d97c7f3321d2,
    0x4022d97c7f3321d2,
    0x4022d97c7f3321d2,
    0x4022d97c7f3321d2,
    0x4022d97c7f3321d2,
    0x4022d97c7f3321d2,
    0x4022d97c7f3321d2,
    0x4022d97c7f3321d2,
    0x4022d97c7f3321d2,
    0x4022d97c7f3321d2,
    0x402921fb54442d18,
    0x402921fb54442d18,
    0x402921fb54442d18,
    0x402921fb54442d18,
    0x402921fb54442d18,
    0x402921fb54442d18,
    0x402921fb54442d18,
    0x402921fb54442d18,
    0x402921fb54442d18,
    0x402921fb54442d18,
    0x402921fb54442d18,
    0x402921fb54442d18,
    0x402921fb54442d18,
    0x402921fb54442d18,
    0x402921fb54442d18,
    0x402921fb54442d18,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
    0x400921fb54442d19,
];
const NEAR_IN: (&str, u64) = (
    "3.00000000000000042441990429567992155564816917960099347933070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433070866141732283464566929133858267716535433071",
    0x3fb381d7dbf487fe,
);
const NEAR_DEG: (&str, u64) = (
    "29.9999999999999998907333440653595851315393910426330193632359523493362503148903467976418983035077895494444474613114329025010993076097023946474195273263523791074782436199927773473735856300590553247974838839899488017745549400862152560776068968062105229480651333224482969348615167518479143073108997675132180754675485804655907159733686564857698466900765043511160205372884565222781740717239190743690530927902075494383952250945931603269411404199361165461869570504938813713910715356055233820881909962711120731407521844420179252298677325399394683493419480335262758199496099544941722543751608827899219352725336888128809966838013194335949951425771580514469755797251690987327671661352992060322041413393515759502706276634036427685162207055022986285019388656225663711724926495339275028699639742471474954429844648359548762820632070777714483893669997582466332340006363096109554534343366368057599636156837445486609505",
    0x3fe0c152382d7366,
);
const NEAR_PI_RAD: (&str, u64) = (
    "0.33333333333333333211925937850399539035043767825147799292484391499262500349878163108490998115008655054938274957012703225001221452899669327386021697029280421230531381799991974830415095144510061471997204315544387557527283266762461395641785440895789469942294592580498107705401685279831015897012110852792464528297276200517323017748187396095299829655640560390128911708098285024753526746359910082632281214211341721598216916771770178141045711577706796162429952278326534857101190595117248202320212218079013414600835760491131028033186369488821631499268831148362528688832899949437969171527956536433246594747259654236534440759779243815105550158419064501607750644139076776369741295705888006702449045932612862166967364070448491965018023006113665142779932072913962634636102943926586114299959971385719439381093849817727640313403563419746053765963333064718481371111818121789950503815929596450844404017426382831845501",
    0x3ff0c152382d7366,
);

/// A float the old door made (the decimal's float times the factor's
/// float) is spelled as the float it is, in its unit; the door's own
/// float is spelled short.
#[test]
fn a_float_the_old_multiply_made_is_spelled_exactly() {
    assert_eq!(
        fmt_length(4.5 * MILLI, MM).unwrap(),
        "4.5000000000000005 mm"
    );
    assert_eq!(
        fmt_angle(30.0 * DEG.factor(), DEG).unwrap(),
        "29.999999999999997 deg"
    );
    assert_eq!(fmt_length((4.5 * MM).meters(), MM).unwrap(), "4.5 mm");
    assert_eq!(fmt_angle((30.0 * DEG).radians(), DEG).unwrap(), "30 deg");
}

/// The text door and the float door read a decimal of sixteen figures
/// apart: the float the literal already is has lost what the text kept
/// (`work/told/a-float-door-loses-what-the-text-door-keeps.md`).
#[test]
fn a_sixteen_figure_decimal_reads_apart_at_the_two_doors() {
    let text = IN.def().canonical(&decimal("9683.884588090202"));
    assert_eq!(text.to_bits(), 0x406e_bf0f_b776_f8fa);
    assert_eq!(
        (9683.884588090202 * IN).meters().to_bits(),
        0x406e_bf0f_b776_f8fb
    );
}

/// The shortest decimal is the shortest: in the canonical units it is
/// std's own digits, and in a scaled unit it is what an independent
/// exact oracle (BBP π) found.
#[test]
fn the_shortest_decimal_is_the_shortest() {
    for x in [
        5.426_657_103_235_053e-166,
        0.1,
        0.300_04,
        1.0 / 3.0,
        1e23,
        5e-324,
        f64::MAX,
    ] {
        assert_eq!(Some(M.def().shortest(x)), Decimal::of_float(x), "{x:e}");
    }
    assert_eq!(MM.def().shortest(0.300_04).to_string(), "300.04");
    assert_eq!(DEG.def().shortest((30.0 * DEG).radians()).to_string(), "30");
    assert_eq!(IN.def().shortest(0.0762).to_string(), "3");
    for (symbol, bits, want) in [
        ("deg", 0x0000_0000_0000_0001_u64, "3e-322"),
        ("deg", 0x0010_0000_0000_0000, "12748734119735194e-322"),
        ("deg", 0x4009_21fb_5444_2d18, "180"),
        ("in", 0x3fd3_3333_3333_3333, "11811023622047244e-15"),
        ("mm", 0x0010_0000_0000_0000, "22250738585072014e-321"),
        ("pi rad", 0x7fef_ffff_ffff_ffff, "57222349715140557e291"),
    ] {
        let unit = unit_by_symbol(symbol).unwrap();
        assert_eq!(
            unit.shortest(f64::from_bits(bits)),
            decimal(want),
            "{symbol} {bits:#x}"
        );
    }
}

/// Every finite float has a decimal of at most seventeen figures that,
/// written in any unit, is that float, and in the canonical units it is
/// the float's own shortest digits: every fifth binade from the
/// subnormals to `f64::MAX` and the eight at each end, at its first
/// float, its neighbours and a float inside it, of both signs.
#[test]
fn every_float_has_a_shortest_decimal_in_every_unit() {
    let binades: Vec<f64> = core::iter::successors(Some(f64::from_bits(1)), |x| Some(x * 2.0))
        .take_while(|x| x.is_finite())
        .collect();
    let ends = (0..8).chain(binades.len() - 8..binades.len());
    let mut values = vec![f64::MAX, f64::MIN_POSITIVE, 0.300_04, 0.1];
    for k in (0..binades.len()).step_by(5).chain(ends) {
        let x = binades[k];
        values.extend([x, x.next_up(), x.next_down(), x * 1.374_218_75]);
    }
    for unit in UNITS {
        for &v in &values {
            for v in [v, -v] {
                if !v.is_finite() || v == 0.0 {
                    continue;
                }
                let shortest = unit.shortest(v);
                assert!(
                    shortest.figures() <= ROUND_TRIP_FIGURES,
                    "{v:e} in {}",
                    unit.symbol()
                );
                assert_eq!(
                    unit.canonical(&shortest).to_bits(),
                    v.to_bits(),
                    "{v:e} in {}: {shortest}",
                    unit.symbol()
                );
                if unit.factor() == 1.0 {
                    assert_eq!(Some(shortest), Decimal::of_float(v), "{v:e}");
                }
            }
        }
    }
}

/// **The half-turn row does the job it was added for**: an angle
/// authored as a multiple of π comes back written that way.
///
/// This is the whole content of "π as a unit" — the row is not a
/// physical unit and claims nothing physical. What it buys is the same
/// thing every other row buys: `fmt(parse(text)) == text` for text a
/// human wrote, so an angle a user chose to say in half-turns is not
/// silently re-rendered in radians. The values below are the ones a
/// user actually writes; the general bit-exact statement over random
/// f64 is `fmt_round_trip_bit_exact_sampled_over_finite_values_and_units`,
/// which covers this row with every other.
///
/// The last row is the point of the pin. `0.5 pi rad` and `90 deg` are
/// the SAME canonical value, the float nearest a quarter turn, so
/// nothing but the authored unit can distinguish them — which is why
/// the unit is stored per literal rather than derived from the number.
#[test]
fn the_half_turn_row_writes_angles_the_way_they_were_authored() {
    for (multiple, text) in [
        (1.0, "1 pi rad"),
        (0.5, "0.5 pi rad"),
        (2.0, "2 pi rad"),
        (-0.25, "-0.25 pi rad"),
    ] {
        let angle = multiple * PI;
        assert_eq!(
            angle.radians().to_bits(),
            (multiple * core::f64::consts::PI).to_bits(),
            "{text} is not multiple × π"
        );
        assert_eq!(fmt_angle(angle.radians(), PI).unwrap(), text);
    }
    // A right angle, said both ways: same quarter turn, each rendered
    // in the unit it was authored in and neither converted to the
    // other.
    let quarter_in_pi = 0.5 * PI;
    let quarter_in_deg = 90.0 * DEG;
    assert_eq!(quarter_in_pi.radians(), quarter_in_deg.radians());
    assert_eq!(
        fmt_angle(quarter_in_pi.radians(), PI).unwrap(),
        "0.5 pi rad"
    );
    assert_eq!(fmt_angle(quarter_in_deg.radians(), DEG).unwrap(), "90 deg");
}

/// The seal's own claim, as an assertion rather than as prose: the
/// symbol DETERMINES the row. Every row a caller can obtain comes from
/// the table, so resolving its symbol must return the same row — which
/// is the premise `editor-core`'s symbol-only `UnitSym::from_def`
/// lookup relies on (issue #650).
///
/// It goes red if the table ever gains two rows sharing a symbol, which
/// is the one way a sealed `UnitDef` could still be ambiguous.
#[test]
fn the_symbol_determines_the_row() {
    for row in UNITS {
        let resolved = unit_by_symbol(row.symbol()).expect("a table row resolves by its symbol");
        assert_eq!(
            resolved,
            row,
            "unit_by_symbol({:?}) is not the row it came from",
            row.symbol()
        );
    }
}

/// **The #669 seal, as the property it exists to buy: every typed view
/// a caller can hold pairs its symbol with the TABLE's factor.**
///
/// The mint is closed by the type — `LengthUnit`/`AngleUnit` wrap a
/// `UnitDef` and have no public constructor, pinned by `compile_fail`
/// doctests on the two types (a runtime test cannot assert that a
/// struct literal does not compile). What this test adds is the other
/// half: that the doors which REPLACE the literal cannot reintroduce
/// the illegal pairing, stated over the whole table so a unit added to
/// `quantity` is covered the day it lands.
///
/// Every route in is here: the six constants, `UnitDef::as_length` /
/// `as_angle`, and — for the constants — the round trip back through
/// `unit_by_symbol`. The three consequences #669 named are asserted at
/// the boundary itself rather than on the fields: the `Mul` value, the
/// `in_unit` value, and the formatter's suffix.
#[test]
fn every_obtainable_typed_view_pairs_its_symbol_with_the_tables_factor() {
    let mut lengths = 0_u32;
    let mut angles = 0_u32;
    for row in UNITS {
        match row.quantity() {
            UnitQuantity::Scalar => {
                let view = row.as_scalar().expect("the Scalar row has a scalar view");
                assert_eq!(
                    row.as_length(),
                    None,
                    "the dimensionless row is not a length"
                );
                assert_eq!(row.as_angle(), None, "nor an angle");
                assert_eq!(view.symbol(), "", "its notation is the ABSENCE of a suffix");
                assert_eq!(view.factor(), 1.0, "and there is nothing to convert");
            }
            UnitQuantity::Length => {
                let view = row.as_length().expect("a Length row has a length view");
                assert_eq!(row.as_angle(), None, "{} is not an angle", row.symbol());
                assert_eq!(view.symbol(), row.symbol());
                assert_eq!(view.factor(), row.factor());
                // The D6 boundary: 2.5 rather than 1.0, since `1.0 * f`
                // is `f` bitwise and cannot tell "applied the factor"
                // from "returned it".
                assert_eq!(
                    (2.5 * view).meters().to_bits(),
                    row.canonical(&decimal("2.5")).to_bits(),
                    "{} read in the wrong unit",
                    row.symbol()
                );
                // `in_unit` is the row's quotient.
                assert_eq!(
                    Some(Length::from_meters(2.5).in_unit(view).to_bits()),
                    row.number_of(2.5).map(f64::to_bits),
                    "{} divided by the wrong factor",
                    row.symbol()
                );
                // The suffix names the unit that was applied — the
                // `parse(fmt(x, unit))` pin's other half.
                assert_eq!(
                    fmt_length((2.5 * view).meters(), view).unwrap(),
                    format!("2.5 {}", row.symbol())
                );
                assert_eq!(
                    unit_by_symbol(view.symbol()).and_then(UnitDef::as_length),
                    Some(view),
                    "{} does not resolve back to itself",
                    row.symbol()
                );
                lengths += 1;
            }
            UnitQuantity::Angle => {
                let view = row.as_angle().expect("an Angle row has an angle view");
                assert_eq!(row.as_length(), None, "{} is not a length", row.symbol());
                assert_eq!(view.symbol(), row.symbol());
                assert_eq!(view.factor(), row.factor());
                assert_eq!(
                    (2.5 * view).radians().to_bits(),
                    row.canonical(&decimal("2.5")).to_bits(),
                    "{} read in the wrong unit",
                    row.symbol()
                );
                assert_eq!(
                    Some(Angle::from_radians(2.5).in_unit(view).to_bits()),
                    row.number_of(2.5).map(f64::to_bits),
                    "{} divided by the wrong factor",
                    row.symbol()
                );
                assert_eq!(
                    fmt_angle((2.5 * view).radians(), view).unwrap(),
                    format!("2.5 {}", row.symbol())
                );
                assert_eq!(
                    unit_by_symbol(view.symbol()).and_then(UnitDef::as_angle),
                    Some(view),
                    "{} does not resolve back to itself",
                    row.symbol()
                );
                angles += 1;
            }
        }
    }
    // Counts, so a table that quietly stopped being exercised cannot
    // pass as green — and so that both arms above are known to have run.
    assert_eq!(lengths, 4, "four length rows");
    assert_eq!(angles, 3, "three angle rows");
    // The exported constants are exactly the rows, so the loop above
    // covers all of them. What makes that a closure rather than a
    // coincidence is upstream of this assertion: a typed view is an
    // INDEX into `UNITS`, so a constant naming a unit the table does
    // not have fails to compile, and one naming a row it does have is
    // already in the loop. This row catches the remaining case — a row
    // added to the table with no constant exported for it.
    let constants: [&str; 8] = [
        MM.symbol(),
        CM.symbol(),
        M.symbol(),
        IN.symbol(),
        DEG.symbol(),
        RAD.symbol(),
        PI.symbol(),
        ONE.symbol(),
    ];
    let mut tabled: Vec<&str> = UNITS.iter().map(UnitDef::symbol).collect();
    let mut named = constants.to_vec();
    tabled.sort_unstable();
    named.sort_unstable();
    assert_eq!(named, tabled, "the constants must cover exactly the table");
}

test_utils::f6_variants! {
    /// `FmtQuantityError`'s census: one ident per variant, feeding
    /// both the wildcard-free `match` rustc checks and the identifier
    /// roster the weld bans from every rendering. An arm added to the
    /// enum stops this file compiling until it is named here, and
    /// naming it is also rostering it, so it then reds until it has a
    /// case below. The mechanism and what it does NOT weld are
    /// documented on [`test_utils::f6::assert_f6_every_variant`].
    const FMT_QUANTITY_ERROR: FmtQuantityError = [NonFinite];
}

/// The `Debug` field-name token this enum's payload would leak. `{` is
/// [`test_utils::f6::assert_f6`]'s own and is banned whatever this
/// list says; what this entry buys over it is a brace-free `value: `
/// written into the sentence by hand.
const FMT_QUANTITY_ERROR_FIELDS: &[&str] = &["value:"];

/// The Display contract (#1111): a façade consumer renders a
/// `FmtQuantityError` through this module's own words — the refused
/// value and why it has no display form — and never as the `Debug`
/// struct dump. The variant identifier and the field-name punctuation
/// are the dump's fingerprints; asserting their ABSENCE is what keeps
/// a future `write!(f, "{self:?}")` from passing this test.
///
/// **The ban is the ENUM's roster, not this arm's own**
/// ([`FMT_QUANTITY_ERROR`]). A rendering that leaks a SIBLING arm's
/// identifier is a dump as surely as one that leaks its own, and the
/// per-arm string this row used to spell could see neither that nor an
/// arm added tomorrow.
#[test]
fn fmt_quantity_error_display_names_its_content_not_its_struct() {
    let values = [f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
    // The refused value is `want`ed as the text the sentence must
    // carry, so the rendered decimal has to outlive the borrow.
    let rendered: Vec<String> = values.iter().map(f64::to_string).collect();
    let cases: Vec<(FmtQuantityError, Vec<&str>)> = values
        .iter()
        .zip(&rendered)
        .map(|(&value, text)| {
            (
                FmtQuantityError::NonFinite { value },
                vec!["no display form", "poison", text.as_str()],
            )
        })
        .collect();
    test_utils::f6::assert_f6_every_variant(
        &cases,
        &FMT_QUANTITY_ERROR,
        &[],
        FMT_QUANTITY_ERROR_FIELDS,
    );
}

/// The authored carriers do what the plain newtypes cannot: keep the
/// unit attached. The value half is BIT-IDENTICAL to what `25.0 * MM`
/// produces — the carrier reads the number through the same door, so
/// nothing about it depends on which door built it.
#[test]
fn an_authored_quantity_keeps_the_unit_the_multiply_would_have_erased() {
    let written = WrittenLength::in_unit(25.0, MM);
    assert_eq!(written.unit(), MM);
    assert_eq!(
        written.length().meters().to_bits(),
        (25.0 * MM).meters().to_bits()
    );
    assert_eq!(written.meters(), 0.025);

    let angle = WrittenAngle::in_unit(90.0, DEG);
    assert_eq!(angle.unit(), DEG);
    assert_eq!(
        angle.angle().radians().to_bits(),
        (90.0 * DEG).radians().to_bits()
    );

    // The half-turn row is a notation carried as a unit, and it rides
    // this door like any other row: two half-turns is a full turn.
    let turn = WrittenAngle::in_unit(2.0, PI);
    assert_eq!(turn.unit(), PI);
    assert_eq!(turn.radians(), core::f64::consts::TAU);
}

/// The plain spelling NAMES the canonical unit rather than declining to
/// name one — there is no unmarked state here, which is what lets a
/// document say how it is written instead of leaning on its reader's
/// fallback. These compare equal.
#[test]
fn the_plain_spelling_is_the_canonical_unit_said_out_loud() {
    assert_eq!(
        WrittenLength::from_meters(0.025),
        WrittenLength::in_unit(0.025, M)
    );
    assert_eq!(WrittenLength::from_meters(0.025).unit(), M);
    assert_eq!(
        WrittenAngle::from_radians(1.5),
        WrittenAngle::in_unit(1.5, RAD)
    );
    assert_eq!(WrittenAngle::from_radians(1.5).unit(), RAD);

    // Same magnitude, different authorings — the carrier tells them
    // apart, unlike the stored literal it feeds, where the display unit
    // is excluded from expression identity.
    let metres = WrittenLength::from_meters(0.025);
    let inches = WrittenLength::canonical_in(0.025, IN);
    assert_eq!(metres.meters().to_bits(), inches.meters().to_bits());
    assert_ne!(metres, inches);
}

/// The form's door: the draft is ALREADY canonical (a picker re-writes
/// what is on screen and changes no value), so this one attaches the
/// notation without reading the number in it. `canonical_in(x, u)` and
/// `in_unit(x, u)` are therefore different values, and deliberately so.
#[test]
fn the_already_canonical_door_attaches_notation_without_applying_it() {
    let form = WrittenLength::canonical_in(0.025, MM);
    assert_eq!(form.meters(), 0.025, "no factor applied");
    assert_eq!(form.unit(), MM, "the notation still rides");
    assert_eq!(WrittenLength::in_unit(0.025, MM).meters(), 2.5e-5);

    assert_eq!(
        WrittenLength::canonical_in(0.025, M),
        WrittenLength::from_meters(0.025)
    );
    assert_eq!(
        WrittenAngle::canonical_in(1.5, RAD),
        WrittenAngle::from_radians(1.5)
    );
    assert_eq!(WrittenAngle::canonical_in(1.5, DEG).radians(), 1.5);
}

/// Every row of the closed table is reachable through its OWN carrier
/// and through no other — the #650/#669 seals, one layer out. The
/// pairing is not checked here because there is no spelling to check:
/// `WrittenLength::in_unit(1.0, DEG)` does not compile.
#[test]
fn each_carrier_admits_exactly_its_own_half_of_the_table() {
    for row in UNITS {
        match row.quantity() {
            // The dimensionless row has no carrier: there is only one
            // way to write a dimensionless number, so there is nothing
            // for an authored value to remember beyond its dimension.
            UnitQuantity::Scalar => assert_eq!(row.symbol(), ""),
            UnitQuantity::Length => {
                let unit = row.as_length().expect("a Length row has the length view");
                let written = WrittenLength::in_unit(3.0, unit);
                assert_eq!(written.unit().symbol(), row.symbol());
                assert_eq!(written.meters(), row.canonical(&decimal("3")));
                assert_eq!(row.as_angle(), None);
            }
            UnitQuantity::Angle => {
                let unit = row.as_angle().expect("an Angle row has the angle view");
                let written = WrittenAngle::in_unit(3.0, unit);
                assert_eq!(written.unit().symbol(), row.symbol());
                assert_eq!(written.radians(), row.canonical(&decimal("3")));
                assert_eq!(row.as_length(), None);
            }
        }
    }
}

/// A unit is a key, and its hash is its symbol.
///
/// The claim a bug could break is the pair the hash contract is about:
/// two INDEPENDENTLY reached values of one table row hash alike, and
/// `Eq`'s answer is `PartialEq`'s answer on every ordered pair of the
/// table. `def()` and `as_length()`/`as_angle()` are the two ways to
/// cross between a row and its view, so a hash that read the index
/// rather than the row, or an `Eq` that disagreed with the derived
/// comparison, would fail here.
#[test]
fn a_unit_row_and_its_view_are_keys_that_hash_by_symbol() {
    fn digest<T: core::hash::Hash>(value: &T) -> u64 {
        use core::hash::Hasher;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    for row in UNITS {
        let reached = unit_by_symbol(row.symbol()).expect("every row is reachable by its symbol");
        assert_eq!(row, reached);
        assert_eq!(digest(&row), digest(&reached));
        if let Some(view) = row.as_length() {
            assert_eq!(
                digest(&view),
                digest(&view.def().as_length().expect("a length row"))
            );
            assert_eq!(digest(&view), digest(&row));
        }
        if let Some(view) = row.as_angle() {
            assert_eq!(
                digest(&view),
                digest(&view.def().as_angle().expect("an angle row"))
            );
            assert_eq!(digest(&view), digest(&row));
        }
    }

    // `Eq` follows `PartialEq`'s answer, over every ordered pair.
    for left in UNITS {
        for right in UNITS {
            assert_eq!(left.eq(&right), left.symbol() == right.symbol());
        }
    }

    // The two views the bindings mirror, used as keys: one entry per
    // row, read back by a value the table minted rather than the
    // constant that keyed it.
    let mut tally = std::collections::HashMap::new();
    for unit in [MM, CM, M, IN] {
        tally.insert(unit, unit.symbol());
    }
    assert_eq!(tally.len(), 4);
    assert_eq!(
        tally[&unit_by_symbol("in")
            .expect("in is a row")
            .as_length()
            .expect("in is a length")],
        "in"
    );
    let angles: std::collections::HashSet<_> = [DEG, RAD, PI, DEG].into_iter().collect();
    assert_eq!(angles.len(), 3);
}
