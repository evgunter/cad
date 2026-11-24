//! **The general-pose cone rows**: the classifier on the unit cone
//! (apex at the origin, axis `z`, `ρ = |z|`) against oblique cylinders
//! and tilted or parallel-axis cones, each row both ways round. Each
//! answered row's class is the one `scripts/oracles/cone_pair_sections_mpmath.py`
//! reads at 40 digits (the dump in [`dump_the_rows_for_the_mpmath_oracle`]),
//! and each row traces the section on both carriers' charts
//! (`cone_pair_search::traced_mismatches`): every witness must lie on a
//! traced component of its part's class. Every row names the mutant
//! that turns it red.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

use super::search_kit::band;
use super::section_cert_rows::{LEVER, cylinder, p, tangent, unit_cone, v};
use super::*;
use core::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_4, PI};
use geom::Surface;
use geom_core::{Point3, Vec3};

/// **The region the section value reads every row over**: wide enough
/// to hold whole every bounded component of these poses that the trace
/// sees, as the operation's region holds its faces.
const REGION: f64 = 12.0;

/// The classification over [`REGION`] about the origin, its closed forms
/// levered over the reach the other rows read at.
fn classify(f: &Surface<f64>, g: &Surface<f64>) -> Section<f64> {
    super::classify(
        f,
        g,
        &geom_brep::Reach::Ball(geom_brep::ExtentBall::fixture(
            p(0.0, 0.0, 0.0),
            LEVER / 2.0,
        )),
        Some(geom_brep::ExtentBall::fixture(p(0.0, 0.0, 0.0), REGION)),
        Some(geom_brep::SsiLane::marched()),
        band(),
    )
}

/// The cone `(apex, axis, α)` with a seam square to its axis.
fn cone_about(apex: Point3<f64>, axis: Vec3<f64>, alpha: f64) -> Surface<f64> {
    let axis = axis.normalize();
    Surface::Cone {
        frame: geom::Frame::new(apex, axis, axis.orthonormal_basis().0),
        half_angle: alpha,
    }
}

/// `(bounded parts as (essential on the cone, on the partner), the
/// unbounded count, single)`, with the bounded parts sorted.
type Classes = (Vec<(bool, bool)>, usize, bool);

/// The classification with the cone as `F`, checked to be the same with
/// the roles exchanged, every bounded part witnessed on both carriers
/// and on a traced component of its class.
fn classes(cone: &Surface<f64>, partner: &Surface<f64>) -> Classes {
    let s = classify(cone, partner);
    let t = classify(partner, cone);
    let (Section::Components { parts, single }, Section::Components { parts: back, .. }) = (&s, &t)
    else {
        panic!("not classified both ways: {s:?}, {t:?}");
    };
    assert_eq!(parts.len(), back.len(), "the count, both ways");
    let mut bounded = Vec::new();
    let mut unbounded = 0;
    for (x, y) in parts.iter().zip(back) {
        assert_eq!(
            (x.unbounded, x.essential_f, x.essential_g),
            (y.unbounded, y.essential_g, y.essential_f),
            "a component's flags, both ways"
        );
        if x.unbounded {
            unbounded += 1;
            continue;
        }
        let w = x.witness.expect("a bounded part carries a witness");
        for surf in [cone, partner] {
            let r = geom_brep::implicit_residual(surf, w);
            assert!(r.abs() < 1e-12, "the witness {w:?} is {r} off {surf:?}");
        }
        bounded.push((x.essential_f, x.essential_g));
    }
    let traced = super::cone_pair_search::traced_mismatches(cone, partner, 1.0, parts, *single);
    assert!(traced.is_empty(), "against the trace: {traced:#?}");
    bounded.sort_unstable();
    (bounded, unbounded, *single)
}

/// Both orders refuse R-tan, under `row`.
fn refuses(cone: &Surface<f64>, partner: &Surface<f64>, row: &str) {
    assert_eq!(tangent(&classify(cone, partner)), row);
    assert_eq!(tangent(&classify(partner, cone)), row);
}

// -------------------------------------------------------------------
// Cone × oblique cylinder
// -------------------------------------------------------------------

/// The rod about the line `y = 0, z = 1` along `x`, radius `0.2`: it
/// passes through the cone, entering and leaving its upper nappe.
fn rod_through() -> Surface<f64> {
    cylinder(p(0.0, 0.0, 1.0), Vec3::unit_x(), 0.2)
}

/// **A rod through the cone cuts two loops, each round the rod.** Null
/// on the cone, essential on the wall. The mutant flagging the wall's
/// class from the cone's chart: red; the mutant dropping a fold of the
/// cone's chart: red (it refuses).
#[test]
fn a_rod_through_the_cone_cuts_two_loops_round_the_rod() {
    let want: Classes = (vec![(false, true), (false, true)], 0, false);
    assert_eq!(classes(&unit_cone(), &rod_through()), want);
}

/// **A rod biting the cone's side cuts one null loop**, certified
/// single: the rod about `y = 1.3, z = 1` along `x`, radius `0.4`, its
/// axis outside the cone (`ρ ≥ 1.3 > z`). The arm the preview's bite
/// (`docs/doc-ledger/germ-verbs-cone-spec.md`, P1) reaches. The mutant
/// flagging the loop essential on either carrier: red.
#[test]
fn a_rod_biting_the_side_cuts_one_null_loop() {
    let bite = cylinder(p(0.0, 1.3, 1.0), Vec3::unit_x(), 0.4);
    assert_eq!(
        classes(&unit_cone(), &bite),
        (vec![(false, false)], 0, true)
    );
}

/// **A fat wall about the apex, its axis steeper than the generators,
/// cuts one curve round each nappe**, essential on both: the axis leans
/// `0.3` rad off `z` through `(0.1, 0, 0)`, radius `1`. The mutant
/// reading the essential class off a full-turn chart as null: red.
#[test]
fn a_fat_steep_wall_about_the_apex_cuts_two_curves_essential_on_both() {
    let wall = cylinder(p(0.1, 0.0, 0.0), v(0.3f64.sin(), 0.0, 0.3f64.cos()), 1.0);
    assert_eq!(
        classes(&unit_cone(), &wall),
        (vec![(true, true), (true, true)], 0, false)
    );
}

/// **A wall about the apex lying across the axis cuts one curve round
/// each nappe, null on the wall**: the axis `x` through the origin,
/// radius `0.5`; every ruling is shallower than the generators and
/// meets one nappe or none. The mutant reading the wall's class from
/// the cone's chart: red.
#[test]
fn a_wall_across_the_axis_about_the_apex_is_null_on_the_wall() {
    let wall = cylinder(p(0.0, 0.0, 0.0), Vec3::unit_x(), 0.5);
    assert_eq!(
        classes(&unit_cone(), &wall),
        (vec![(true, false), (true, false)], 0, false)
    );
}

/// **The degenerate cylinder poses, as the section value decides them**:
/// the apex on the wall (the rod about `y = 0, z = 0.2`, radius `0.2`,
/// its wall through the apex, the cone's form definite on its tangent
/// plane) is a touch at the apex beside the loop round the upper nappe,
/// R-tan on the apex's margin; the axis along a generator (`x = z`)
/// refuses on its own margin; and the rod about `y = 0, z = 1` of radius
/// `1/√2`, which touches the cone along the circle of its own section
/// `x = 0` at `y = ±z`, is decided a crossing tangency: two arcs through
/// each of two nodes, one node-joined set, one bounded component that
/// claims no winding. A reading that refuses the graze, or reads the
/// apex as a tangency: red.
#[test]
fn the_degenerate_cylinder_poses_read_as_the_value_decides_them() {
    let cone = unit_cone();
    refuses(
        &cone,
        &cylinder(p(0.0, 0.0, 0.2), Vec3::unit_x(), 0.2),
        "kc_apex_on_partner",
    );
    refuses(
        &cone,
        &cylinder(p(0.5, 0.0, 0.0), v(1.0, 0.0, 1.0), 0.3),
        "kc_generator_along_axis",
    );
    let rod = cylinder(p(0.0, 0.0, 1.0), Vec3::unit_x(), FRAC_1_SQRT_2);
    for graze in [classify(&cone, &rod), classify(&rod, &cone)] {
        let Section::Components { parts, single } = &graze else {
            panic!("the grazing rod is classified: {graze:?}")
        };
        assert!(*single, "one node-joined set: {graze:?}");
        assert!(
            parts
                .iter()
                .all(|c| !c.unbounded && !c.essential_f && !c.essential_g),
            "a bounded set claiming no winding: {graze:?}"
        );
    }
}

// -------------------------------------------------------------------
// Cone × tilted or parallel-axis cone
// -------------------------------------------------------------------

/// **A narrow cone through the side cuts two loops, each round it**:
/// apex `(−5, 0, 2)`, axis `x`, `α = 0.2`. Its directions all lie
/// within `0.2` rad of `x`, outside the unit cone's (`π/4` of `±z`), so
/// nothing runs to infinity. The mutant flagging the partner's class
/// from the first chart: red; the mutant dropping a fold: red.
#[test]
fn a_narrow_cone_through_the_side_cuts_two_loops_round_it() {
    let narrow = cone_about(p(-5.0, 0.0, 2.0), Vec3::unit_x(), 0.2);
    assert_eq!(
        classes(&unit_cone(), &narrow),
        (vec![(false, true), (false, true)], 0, false)
    );
}

/// **A parallel-axis cone cuts one loop per side of the apexes' plane,
/// round the partner only**: apex `(1, 0, 0)`, axis `z`,
/// `tan α = 1/2`. The parallels `ρ₁ = |h|`, `ρ₂ = |h|/2` at offset `1`
/// meet over `2/3 ≤ |h| ≤ 2`, a bounded interval on each side; at
/// `|h| = 2/3` the circles touch between the axes (angle `0` about
/// both), at `|h| = 2` beyond the partner's axis (angle `0` about the
/// unit cone's, `π` about the partner's). The mutant flagging the
/// partner's class from the first chart: red.
#[test]
fn a_parallel_axis_cone_cuts_one_loop_each_side_round_the_partner() {
    let partner = cone_about(p(1.0, 0.0, 0.0), Vec3::unit_z(), 0.5f64.atan());
    assert_eq!(
        classes(&unit_cone(), &partner),
        (vec![(false, true), (false, true)], 0, false)
    );
}

/// **A tilted cone sharing asymptotic directions cuts unbounded
/// branches, and here one bounded curve round the unit cone**: apex
/// `(3, 0, 0)`, axis `z` turned `1` rad about `y`, `α = π/4`. The unit
/// cone's generators along the partner's asymptotic directions are where
/// a branch leaves to infinity: two unbounded components, and the curve
/// that never does, null on the partner. The mutant dropping an
/// asymptote: red.
#[test]
fn a_tilted_cone_sharing_asymptotic_directions_cuts_unbounded_branches() {
    let tilted = cone_about(p(3.0, 0.0, 0.0), v(1f64.sin(), 0.0, 1f64.cos()), FRAC_PI_4);
    assert_eq!(
        classes(&unit_cone(), &tilted),
        (vec![(true, false)], 2, false)
    );
}

/// **Two side-by-side cones of one aperture meet in a hyperbola**: apex
/// `(1, 0, 0)`, axis `z`, `α = π/4`. The forms share their quadratic
/// part, so the section is the plane `x = 1/2`'s (`z² = y² + 1/4`): two
/// unbounded branches, where the charts would read `p₂ ≡ 0`. The mutant
/// sending the pose to the charts: red (it refuses `_fold`).
#[test]
fn twin_cones_side_by_side_meet_in_a_hyperbola() {
    let twin = cone_about(p(1.0, 0.0, 0.0), Vec3::unit_z(), FRAC_PI_4);
    assert_eq!(classes(&unit_cone(), &twin), (vec![], 2, false));
    let flipped = cone_about(p(1.0, 0.0, 0.0), -Vec3::unit_z(), FRAC_PI_4);
    assert_eq!(
        classes(&unit_cone(), &flipped),
        (vec![], 2, false),
        "flipped"
    );
}

/// **A twin cone with its apex inside meets the unit cone in one
/// ellipse**, essential on both: apex `(0.2, 0, −1)`, axis `z`,
/// `α = π/4`, on the lower nappe; the plane `0.1x + 0.5z = −0.24` is
/// steeper than the generators. The mutant reading the partner's class off the plane
/// (`essential_g` false): red.
#[test]
fn a_twin_cone_with_its_apex_inside_meets_in_one_ellipse() {
    let twin = cone_about(p(0.2, 0.0, -1.0), Vec3::unit_z(), FRAC_PI_4);
    assert_eq!(classes(&unit_cone(), &twin), (vec![(true, true)], 0, true));
}

/// **Two cones through one apex meet in the rulings both hold**: a
/// partner on an axis tilted far off the unit cone's shares no ruling,
/// a touch at the apex; one tilted less crosses it in two lines through
/// the apex, each an unbounded component.
#[test]
fn two_cones_through_one_apex_meet_in_their_common_rulings() {
    let cone = unit_cone();
    let s = classify(&cone, &cone_about(p(0.0, 0.0, 0.0), v(0.2, 1.0, 0.3), 0.4));
    let Section::Touch(t) = s else {
        panic!("not a touch: {s:?}")
    };
    assert_eq!(t.name, "bool_cones_share_apex");
    assert_eq!((t.at - Point3::origin()).norm(), 0.0);
    // Tilted 0.5 off the unit cone's axis, half-angle 0.4: the nappes
    // facing one way cross, `|π/4 − 0.4| < 0.5 < π/4 + 0.4`; the axes
    // turned apart, `π − 0.5`, are past the sum.
    let tilted = cone_about(p(0.0, 0.0, 0.0), v(0.5f64.sin(), 0.0, 0.5f64.cos()), 0.4);
    // Read both ways in closed form: lines through an apex resolve on no
    // marched grid.
    for s in [classify(&cone, &tilted), classify(&tilted, &cone)] {
        let Section::Components { parts, single } = &s else {
            panic!("not classified: {s:?}")
        };
        assert!(!single, "{s:?}");
        assert_eq!(parts.len(), 2, "{s:?}");
        assert!(
            parts
                .iter()
                .all(|c| c.unbounded && !c.essential_f && !c.essential_g),
            "{s:?}"
        );
    }
}

/// **The degenerate cone poses, as the section value decides them**:
/// the partner's apex on the unit cone (`(1, 0, 1)`), its form
/// indefinite on the unit cone's tangent plane there, is a node at that
/// apex, its arcs one bounded node-joined set; a partner whose direction
/// cone touches the unit cone's (axis `5π/12` off `z`, `α = π/6`: one
/// asymptotic direction shared doubly) is a branch leaving every bound,
/// cut by the region; and a parallel-axis partner whose aperture is in
/// the band's gap off the unit cone's refuses R-tan on the sweep's own
/// discriminant. A reading that refuses the apex or the doubled
/// asymptote, or answers the aperture in the band: red.
#[test]
fn the_degenerate_cone_poses_read_as_the_value_decides_them() {
    let cone = unit_cone();
    let apex = cone_about(p(1.0, 0.0, 1.0), v(0.2, 1.0, 0.3), 0.4);
    for s in [classify(&cone, &apex), classify(&apex, &cone)] {
        assert!(
            matches!(&s, Section::Components { parts, single: true }
                if parts.iter().all(|c| !c.unbounded)),
            "the apex on the unit cone: one bounded set, {s:?}"
        );
    }
    let lean = 5.0 * PI / 12.0;
    let doubled = cone_about(p(0.3, 2.0, 0.5), v(lean.sin(), 0.0, lean.cos()), PI / 6.0);
    // The branch leaves along the doubled asymptote nearly tangent to the
    // region's sphere, so at a finer ε its cut end may straddle it.
    for s in [classify(&cone, &doubled), classify(&doubled, &cone)] {
        assert!(
            matches!(&s, Section::Components { parts, .. }
                if parts.len() == 1 && parts[0].unbounded)
                || matches!(s, Section::Tangent(name) if super::search_kit::region_cut(name)),
            "the doubled asymptote: one branch the region cuts, {s:?}"
        );
    }
    let b = band();
    // The aperture gap the retired charts levered over the reach: the
    // pose is the one they refused.
    let lever = 2.0 * (1.0 + LEVER / 2.0);
    let gap = (b.zero() + b.escalate()) / 2.0 / lever;
    let alpha = (0.5f64 - gap).sqrt().acos();
    refuses(
        &cone,
        &cone_about(p(1.0, 0.0, 0.0), Vec3::unit_z(), alpha),
        "ruling_discriminant",
    );
}

/// **The rows, for the high-precision oracle**: every answered pose
/// above, one JSON line each, tagged `CPDUMP`, read by
/// `scripts/oracles/cone_pair_sections_mpmath.py`. Run:
/// `cargo nextest run -p topo --lib --run-ignored only
/// dump_the_rows_for_the_mpmath_oracle --no-capture | sed -n
/// 's/^CPDUMP //p' > /tmp/rows.jsonl`, then
/// `python3 scripts/oracles/cone_pair_sections_mpmath.py /tmp/rows.jsonl`.
#[test]
#[ignore = "an oracle dump; run command in the docs"]
fn dump_the_rows_for_the_mpmath_oracle() {
    let lean = 1f64;
    let rows: [(&str, Surface<f64>); 9] = [
        ("rod through", rod_through()),
        ("bite", cylinder(p(0.0, 1.3, 1.0), Vec3::unit_x(), 0.4)),
        (
            "fat steep wall",
            cylinder(p(0.1, 0.0, 0.0), v(0.3f64.sin(), 0.0, 0.3f64.cos()), 1.0),
        ),
        (
            "wall across",
            cylinder(p(0.0, 0.0, 0.0), Vec3::unit_x(), 0.5),
        ),
        (
            "narrow cone",
            cone_about(p(-5.0, 0.0, 2.0), Vec3::unit_x(), 0.2),
        ),
        (
            "parallel cone",
            cone_about(p(1.0, 0.0, 0.0), Vec3::unit_z(), 0.5f64.atan()),
        ),
        (
            "tilted cone",
            cone_about(p(3.0, 0.0, 0.0), v(lean.sin(), 0.0, lean.cos()), FRAC_PI_4),
        ),
        (
            "twin side by side",
            cone_about(p(1.0, 0.0, 0.0), Vec3::unit_z(), FRAC_PI_4),
        ),
        (
            "twin inside",
            cone_about(p(0.2, 0.0, -1.0), Vec3::unit_z(), FRAC_PI_4),
        ),
    ];
    let cone = unit_cone();
    for (name, partner) in rows {
        let sec = classify(&cone, &partner);
        println!(
            "CPDUMP {}",
            super::cone_pair_search::dump_line(name, 1.0, &cone, &partner, &sec)
        );
    }
}

/// The classification at ε `eps` (the run's K), about `centre`.
fn classify_at(
    f: &Surface<f64>,
    g: &Surface<f64>,
    centre: Point3<f64>,
    radius: f64,
    eps: f64,
) -> Section<f64> {
    let band = geom_core::Band::linear_at(geom_core::Tol::witness(), eps).unwrap();
    super::classify(
        f,
        g,
        &geom_brep::Reach::Ball(geom_brep::ExtentBall::fixture(centre, radius)),
        None,
        Some(geom_brep::SsiLane::marched()),
        band,
    )
}

/// **A far rod and a slender cone answer at every ε**: the rod through
/// the cone at 300 m (the reach 750 m) and the rod through a slender cone
/// (`α = 0.02`, the rod about `y = 0, z = 10` along `x`, radius `0.2`),
/// which the retired ruling charts refused at `1e-12` on their
/// `_precision` rows, read on the value at both `1e-9` and `1e-12`: two
/// loops each, round the rod and round the slender cone respectively. A
/// reading that refuses either at the finer ε: red.
#[test]
fn a_far_rod_and_a_slender_cone_answer_at_every_eps() {
    let origin = p(0.0, 0.0, 0.0);
    let rod = |s: f64| cylinder(p(0.0, 0.0, s), Vec3::unit_x(), 0.2 * s);
    let classes = |x: &Section<f64>| match x {
        Section::Components { parts, .. } => parts
            .iter()
            .map(|c| (c.unbounded, c.essential_f, c.essential_g))
            .collect::<Vec<_>>(),
        other => panic!("not classified: {other:?}"),
    };
    let slender = cone_about(origin, Vec3::unit_z(), 0.02);
    let thin = cylinder(p(0.0, 0.0, 10.0), Vec3::unit_x(), 0.2);
    for eps in [1e-9, 1e-12] {
        let far = classify_at(&unit_cone(), &rod(300.0), origin, 750.0, eps);
        assert_eq!(
            classes(&far),
            vec![(false, false, true); 2],
            "300 m at {eps:e}"
        );
        let near = classify_at(&slender, &thin, p(0.0, 0.0, 10.0), 2.5, eps);
        assert_eq!(
            classes(&near),
            vec![(false, true, false); 2],
            "slender at {eps:e}"
        );
    }
}

/// **Coaxial cones whose apexes stand a hair apart refuse on the value's
/// sweep**: the pose `cone_search` drew (seed `0xbe01201603b110c4`), the
/// partner's apex `2.1·10⁻⁷` m up the first cone's axis, at about `80` m
/// from the origin. The retired coaxial arm read the parallels in closed
/// form there; the section value's sweep meets the discriminant in the
/// band, the sheets passing all but through the near apex. A narrowing
/// filed as
/// `work/sectcone/coaxial-cones-a-hair-apart-refuse-on-the-sweep.md`. A
/// reading that answers here, or refuses on another row: red.
#[test]
fn coaxial_cones_a_hair_apart_refuse_on_the_sweep() {
    let apex = p(11.469147488760456, -61.43428644076939, -58.126021681816255);
    let axis = v(-0.6257110708896446, -0.5303880180400604, 0.5719914388220076).normalize();
    let first = cone_about(apex, axis, 0.9444403477437026);
    let second = cone_about(
        apex + axis * 2.120791546399104e-7,
        axis,
        0.32689233645338184,
    );
    let reach = geom_brep::Reach::Ball(geom_brep::ExtentBall::fixture(apex, 5.0 * 80.0));
    let read = |f: &Surface<f64>, g: &Surface<f64>| {
        super::classify(
            f,
            g,
            &reach,
            Some(geom_brep::ExtentBall::fixture(apex, 100.0)),
            Some(geom_brep::SsiLane::marched()),
            band(),
        )
    };
    assert_eq!(tangent(&read(&first, &second)), "ruling_discriminant");
    assert_eq!(tangent(&read(&second, &first)), "ruling_discriminant");
}
