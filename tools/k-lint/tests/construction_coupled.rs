//! **Rule (5), the construction-coupled names** (`k_lint`'s module docs,
//! "The construction-coupled families").
//!
//! The fitted circle envelope (`pcurve_envelope_hermite`) is a certified
//! bound refined to a quarter of the band, so its `zero` rows land just
//! under `ε/4` at every ε. These rows pin three things:
//! - the rows the first sweep recorded pass;
//! - a fitted envelope that reaches the band flags, on either side of
//!   the coincidence threshold;
//! - the name's closed-form sites, which keep `pcurve_envelope`, still
//!   answer to the metre rules.
//!
//! The last row pins the premise at its source: the mint spells the name,
//! and the lane refines to the target this list states.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use k_lint::{
    BASELINE_FLOOR_MARGIN, CONSTRUCTION_CEILING_FACTOR, CONSTRUCTION_COUPLED, Reason,
    construction_ceiling, lint_sample,
};
use test_utils::source;

const HERMITE: &str = "pcurve_envelope_hermite";

/// `(ε, margin)` of every `pcurve_envelope_hermite` row the first sweep
/// recorded (`demo/lily_walls`, wall 7's tilted carve of the lantern's
/// zone sphere), each recorded twice per ε row, all `zero`.
const MEASURED: [(f64, f64); 12] = [
    (1e-6, 1.942_112_444_076_099_4e-7),
    (1e-6, 2.289_962_663_528_326e-7),
    (1e-6, 2.289_962_662_164_088_2e-7),
    (1e-6, 1.942_112_444_075_969_2e-7),
    (1e-9, 2.408_689_184_193_787e-10),
    (1e-9, 2.123_731_480_047_977_2e-10),
    (1e-9, 2.123_730_128_956_737_7e-10),
    (1e-9, 2.408_689_342_483_723e-10),
    (1e-12, 2.470_405_863_006_701e-13),
    (1e-12, 2.497_967_830_513_850_7e-13),
    (1e-12, 2.497_967_830_513_072e-13),
    (1e-12, 2.470_405_863_006_662e-13),
];

#[test]
fn the_fitted_rows_the_sweep_recorded_pass_rule_5() {
    for (eps, m) in MEASURED {
        assert_eq!(
            lint_sample(HERMITE, m, eps, 10.0 * eps, "zero"),
            Vec::<Reason>::new(),
            "a measured fitted envelope {m:e} at ε {eps:e}"
        );
        // The same row under the old shared name is what flagged: rule
        // (2)'s zero arm.
        assert_eq!(
            lint_sample("pcurve_envelope", m, eps, 10.0 * eps, "zero"),
            vec![Reason::NearBandBelow],
            "the measured row under the closed-form name, ε {eps:e}"
        );
    }
}

#[test]
fn a_fitted_envelope_that_reaches_the_band_is_flagged() {
    for eps in [1e-6, 1e-9, 1e-12] {
        let esc = 10.0 * eps;
        // Above the ceiling and still classified zero: the construction
        // no longer meets its own target.
        for ratio in [0.31, 0.5, 0.99] {
            assert_eq!(
                lint_sample(HERMITE, ratio * eps, eps, esc, "zero"),
                vec![Reason::AboveConstructionTarget],
                "a zero envelope at {ratio}·ε, ε {eps:e}"
            );
        }
        // Past the band: the certificate refused.
        for (m, outcome) in [(1.5 * esc, "positive"), (-1.5 * esc, "negative")] {
            assert_eq!(
                lint_sample(HERMITE, m, eps, esc, outcome),
                vec![Reason::ConstructionRefused],
                "a {outcome} envelope, ε {eps:e}"
            );
        }
        // In the band: rule (1), whatever the name.
        assert_eq!(
            lint_sample(HERMITE, 3.0 * eps, eps, esc, "indeterminate"),
            vec![Reason::InBand]
        );
    }
}

#[test]
fn a_closed_form_envelope_stays_under_the_metre_rules() {
    assert_eq!(construction_ceiling("pcurve_envelope"), None);
    for eps in [1e-6, 1e-9, 1e-12] {
        let esc = 10.0 * eps;
        // Rounding-sized, as the closed-form lanes record: clean.
        assert!(lint_sample("pcurve_envelope", 1.5e-15, eps, esc, "zero").is_empty());
        // Crowding the coincidence threshold: rule (2)'s zero arm.
        assert_eq!(
            lint_sample("pcurve_envelope", 0.2 * eps, eps, esc, "zero"),
            vec![Reason::NearBandBelow]
        );
        // A definite closed-form envelope crowding the metre floor:
        // rule (3), and rule (2) wherever its arm discriminates.
        let crowding = lint_sample("pcurve_envelope", 2.0 * esc, eps, esc, "positive");
        assert!(
            crowding.contains(&Reason::BelowBaselineFloor),
            "ε {eps:e}: {crowding:?}"
        );
    }
}

/// The body of `fn {name}` in `code`, the comments-and-literals-blanked
/// view of `path`, as a byte range valid in every blanked view.
fn item(code: &str, path: &str, name: &str) -> std::ops::Range<usize> {
    let needle = format!("fn {name}");
    let heads = source::required_matches(code, path, &needle);
    assert_eq!(
        heads.len(),
        1,
        "{path} declares `{needle}` {} times",
        heads.len()
    );
    let source::ItemBody::Body(body) = source::item_body(code, heads[0]) else {
        panic!("`{needle}` in {path} has no body");
    };
    body
}

/// **The rostered name is minted for the construction whose target the
/// roster states, and nowhere else.** Three places, read through the
/// shared lexer: the name is spelled once, as the envelope name of the
/// `MapResidualHermite` statement inside `run_fitted_checks`; that
/// statement is stated once, by `fitted_lane`; and
/// `sphere_circle_image_lane`, the image that statement certifies,
/// refines to the roster's target. A second construction that reused
/// the name, or a lane that stated the statement elsewhere, reds here.
#[test]
fn the_rostered_target_is_the_lanes_own() {
    const PATH: &str = "crates/geom-brep/src/pcurve_cache.rs";
    const CACHE: &str = include_str!("../../../crates/geom-brep/src/pcurve_cache.rs");
    // Blanked in place so offsets agree across views: the code view
    // locates items, the literal view reads names and numbers, and
    // neither answers to a mention in prose.
    let code = source::blanked(source::code_only, PATH, CACHE);
    let lits = source::blanked(source::code_and_literals, PATH, CACHE);
    assert_eq!(CONSTRUCTION_COUPLED.len(), 1);
    let (name, target, _) = CONSTRUCTION_COUPLED[0];
    assert_eq!(name, HERMITE);

    // Where the name is minted.
    let minted = source::required_matches(&lits, PATH, &format!("\"{name}\""));
    assert_eq!(
        minted.len(),
        1,
        "{PATH} spells {name:?} {} times",
        minted.len()
    );
    let checks = item(&code, PATH, "run_fitted_checks");
    assert!(
        checks.contains(&minted[0]),
        "{name:?} is minted outside run_fitted_checks in {PATH}"
    );
    let arm = format!("EnvelopeStatement::MapResidualHermite => \"{name}\"");
    assert!(
        lits[checks.clone()].contains(&arm),
        "run_fitted_checks no longer keys {name:?} on the MapResidualHermite statement"
    );

    // Who states that statement.
    let stated = source::required_matches(
        &code,
        PATH,
        "statement: EnvelopeStatement::MapResidualHermite",
    );
    assert_eq!(
        stated.len(),
        1,
        "{PATH} states MapResidualHermite {} times",
        stated.len()
    );
    assert!(
        item(&code, PATH, "fitted_lane").contains(&stated[0]),
        "MapResidualHermite is stated outside fitted_lane in {PATH}"
    );

    // And the target that construction refines to.
    let lane = item(&code, PATH, "sphere_circle_image_lane");
    assert!(
        lits[lane].contains(&format!("{target} * band.zero()")),
        "sphere_circle_image_lane no longer refines to {target} * band.zero(): rule (5)'s \
         target for {name:?} is stale"
    );
    assert!(CONSTRUCTION_CEILING_FACTOR * target < 1.0);
}

#[test]
fn rule_5_tallies_under_its_own_number() {
    assert_eq!(Reason::AboveConstructionTarget.rule(), 5);
    assert_eq!(Reason::ConstructionRefused.rule(), 5);
}

/// **The sweep row that keeps the clearance charge out of the K
/// population holds it to this lint's own floor.**
/// `crates/sweep/tests/tilted_sphere_pair_k_rows.rs` cannot depend on
/// this crate (it is a separate cargo root), so it spells the floor as
/// `METRE_FLOOR`. That spelling is pinned here to
/// [`BASELINE_FLOOR_MARGIN`], so the two cannot drift apart.
#[test]
fn the_sweep_rows_metre_floor_is_the_lints() {
    const PATH: &str = "crates/sweep/tests/tilted_sphere_pair_k_rows.rs";
    const ROW: &str = include_str!("../../../crates/sweep/tests/tilted_sphere_pair_k_rows.rs");
    let code = source::blanked(source::code_only, PATH, ROW);
    let at = source::required_matches(&code, PATH, "const METRE_FLOOR: f64 =");
    assert_eq!(
        at.len(),
        1,
        "{PATH} declares METRE_FLOOR {} times",
        at.len()
    );
    let value: f64 = code[at[0]..]
        .split_once('=')
        .and_then(|(_, rest)| rest.split_once(';'))
        .map(|(v, _)| {
            v.trim()
                .parse()
                .expect("METRE_FLOOR is a plain f64 literal")
        })
        .expect("METRE_FLOOR has a value");
    assert_eq!(
        value, BASELINE_FLOOR_MARGIN,
        "{PATH}'s METRE_FLOOR is not k-lint's BASELINE_FLOOR_MARGIN"
    );
}
