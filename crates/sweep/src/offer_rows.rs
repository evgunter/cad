//! **Every tolerance a sphere or revolved refusal offers, executed**
//! (D4 ¶1 (i)): the cases `topo`'s executed-offer census counts as run
//! here (`topo::test_support::OFFERS_EXECUTED_IN_SWEEP`), whose raises
//! need the balls and the revolved solids this crate builds. Each is a
//! public Boolean at a fixed margin chosen against the band at
//! [`DESIGN_EPS`]; its child row raises it at whatever tolerance its
//! process runs at, and [`test_utils::offer::execute`] re-runs it just
//! below the value the refusal offered (the harness's module docs state
//! what is true of an offer and what is false). Each case states the
//! margin its geometry gives, and the value it quotes and offers is
//! checked against it.
//!
//! The poses are the coincfr3 review's C1 sphere probes and the coincfr4
//! review's own (`zz_fr4_sweep_probe`): its off-axis spheres, its balls
//! under and beside a slab, its brick below a tube, and its dome on a
//! tube declared `Tangent`, whose rim offer is withdrawn.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::test_support::{ball_poled, ball_poled_y, brick, dome, finished, revolved_about_y};
use geom_core::{Band, Point2, Tol, Vec3};
use test_utils::offer::{DESIGN_EPS, Executed, Outcome, Verdict, execute, report, run};
use topo::{BooleanDeclarations, ContactClass, FacePairDeclaration};

/// A margin in the middle of the band at [`DESIGN_EPS`].
const D: f64 = 5.5e-9;

/// A margin in the zero band at [`DESIGN_EPS`].
const Z: f64 = 5e-10;

fn outcome(got: Result<(), topo::BooleanError>) -> Outcome {
    match got {
        Ok(()) => Outcome::Pass,
        Err(err) => {
            let (key, defect) = topo::test_support::offer_key(&err);
            Outcome::Refused {
                key,
                defect,
                text: err.to_string(),
            }
        }
    }
}

/// The public op `k` (0 union, 1 subtract, else intersect).
fn op(
    k: u8,
    a: &topo::AtRestBody<f64>,
    b: &topo::AtRestBody<f64>,
) -> Result<(), topo::BooleanError> {
    let tol = Tol::witness();
    match k {
        0 => topo::union(a, b, tol).map(|_| ()),
        1 => topo::subtract(a, b, tol).map(|_| ()),
        _ => topo::intersect(a, b, tol).map(|_| ()),
    }
}

/// A unit ball about `(2, 2, 0.5)` and a half-unit ball on its axis at
/// height `z` above that center: their union.
fn two_balls(z: f64) -> Result<(), topo::BooleanError> {
    let tol = Tol::witness();
    let big = finished(
        "the big ball",
        ball_poled_y(1.0, Vec3::new(2.0, 2.0, 0.5), tol),
        tol,
    );
    let small = finished(
        "the small ball",
        ball_poled_y(0.5, Vec3::new(2.0, 2.0, 0.5 + z), tol),
        tol,
    );
    topo::union(&big, &small, tol).map(|_| ())
}

/// A half-unit ball whose bottom stands `gap` above the top of the slab
/// `[0, 4]² × [0, 1]`: their union.
fn ball_over_a_slab(gap: f64) -> Result<(), topo::BooleanError> {
    let tol = Tol::witness();
    let slab = finished(
        "the slab",
        brick::<f64>((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol),
        tol,
    );
    let ball = finished(
        "the ball",
        ball_poled_y(0.5, Vec3::new(2.0, 2.0, 1.5 + gap), tol),
        tol,
    );
    topo::union(&slab, &ball, tol).map(|_| ())
}

/// The coincfr4 review's `slanted_balls`: a ball of radius 1.3 and one of
/// 0.4 at centre distance `d` along `(1, 1, 1)`, by op `k`.
fn slanted_balls(k: u8, d: f64) -> Result<(), topo::BooleanError> {
    let tol = Tol::witness();
    let c0 = Vec3::new(3.0, 3.0, 3.0);
    let dir = Vec3::new(1.0, 1.0, 1.0).normalize();
    op(
        k,
        &finished("the big ball", ball_poled_y(1.3, c0, tol), tol),
        &finished("the small ball", ball_poled_y(0.4, c0 + dir * d, tol), tol),
    )
}

/// The coincfr4 review's `nested_balls`: the same two balls, the small
/// one's centre `d` off the big one's along `(1, −1, 2)`, by op `k`.
fn nested_balls(k: u8, d: f64) -> Result<(), topo::BooleanError> {
    let tol = Tol::witness();
    let c0 = Vec3::new(3.0, 3.0, 3.0);
    let dir = Vec3::new(1.0, -1.0, 2.0).normalize();
    op(
        k,
        &finished("the big ball", ball_poled_y(1.3, c0, tol), tol),
        &finished("the small ball", ball_poled_y(0.4, c0 + dir * d, tol), tol),
    )
}

/// The coincfr4 review's `ball_under_slab`: a ball of radius 0.7 under
/// the slab `[0, 4]² × [0, 1]`, its top `gap` below the bottom face,
/// poled along `z`, by op `k`.
fn ball_under_a_slab(k: u8, gap: f64) -> Result<(), topo::BooleanError> {
    let tol = Tol::witness();
    let slab = finished(
        "the slab",
        brick::<f64>((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol),
        tol,
    );
    let ball = finished(
        "the ball",
        ball_poled(
            0.7,
            Vec3::new(1.7, 2.2, -0.7 - gap),
            Vec3::new(0.0, 0.0, 1.0),
            tol,
        ),
        tol,
    );
    op(k, &slab, &ball)
}

/// The coincfr4 review's `ball_beside_slab`: a ball of radius 0.6 `gap`
/// off the slab's face `x = 4`, poled along `x`, by op `k`.
fn ball_beside_a_slab(k: u8, gap: f64) -> Result<(), topo::BooleanError> {
    let tol = Tol::witness();
    let slab = finished(
        "the slab",
        brick::<f64>((0.0, 4.0), (0.0, 4.0), (0.0, 2.0), tol),
        tol,
    );
    let ball = finished(
        "the ball",
        ball_poled(
            0.6,
            Vec3::new(4.6 + gap, 2.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
            tol,
        ),
        tol,
    );
    op(k, &slab, &ball)
}

/// A tube about `y` (radii 0.3 to `outer`, `y` over `[lo, hi]`).
fn tube(outer: f64, lo: f64, hi: f64) -> topo::AtRestBody<f64> {
    let tol = Tol::witness();
    let profile = vec![
        (Point2::new(0.3, lo), 0.0),
        (Point2::new(outer, lo), 0.0),
        (Point2::new(outer, hi), 0.0),
        (Point2::new(0.3, hi), 0.0),
    ];
    finished(
        "the tube",
        revolved_about_y(profile, crate::Revolution::Full, tol),
        tol,
    )
}

/// The coincfr4 review's `brick_by_tube` (below): a brick whose top face
/// stands `gap` below a unit tube's wall, its edges across the axis, by
/// op `k`.
fn brick_below_a_tube(k: u8, gap: f64) -> Result<(), topo::BooleanError> {
    let tol = Tol::witness();
    let b = finished(
        "the brick",
        brick::<f64>((-0.5, 0.5), (0.2, 0.8), (-2.0, -1.0 - gap), tol),
        tol,
    );
    op(k, &tube(1.0, 0.0, 1.0), &b)
}

/// The coincfr4 review's `dome_on_tube`: a unit dome declared `Tangent`
/// to a tube whose outer wall has radius `r` and ends at the dome's
/// equator, through `topo::union_with`: the shared rim read at the
/// declaration door.
fn dome_on_a_tube(r: f64) -> Result<(), topo::BooleanError> {
    let tol = Tol::witness();
    let a = finished("the dome", dome(1.0, tol), tol);
    let b = tube(r, -1.0, 0.0);
    let spheres: Vec<topo::FaceKey> = a
        .faces()
        .filter(|(_, f)| matches!(a.get_surface(f.surface), Some(geom::Surface::Sphere { .. })))
        .map(|(k, _)| k)
        .collect();
    let walls: Vec<topo::FaceKey> = b
        .faces()
        .filter(|(_, f)| {
            matches!(b.get_surface(f.surface),
                Some(geom::Surface::Cylinder { radius, .. }) if *radius > 0.9)
        })
        .map(|(k, _)| k)
        .collect();
    let mut decls = BooleanDeclarations::none();
    for &fa in &spheres {
        for &fb in &walls {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Tangent));
        }
    }
    topo::union_with(&a, &b, &decls, tol).map(|_| ())
}

macro_rules! cases {
    ($($name:ident: $key:literal, $margin:expr => $raise:expr;)*) => {
        $(
            #[test]
            #[ignore = "a child row: every_sphere_offer_passes_just_below_it runs it"]
            fn $name() {
                report(stringify!($name), &outcome($raise));
            }
        )*
        /// Each case: its key, the margin its geometry gives, its name.
        fn cases() -> Vec<(&'static str, f64, &'static str)> {
            vec![$(($key, $margin, stringify!($name))),*]
        }
    };
}

cases! {
    apart_in_band: "Sphere(Apart)", D => two_balls(1.5 + D);
    // A decided zero the spheres' gap reads within the band: it refuses
    // as the in-band arm does.
    apart_in_the_zero_band: "Sphere(Apart)", Z => two_balls(1.5 + Z);
    nested_in_band: "Sphere(Nested)", D => two_balls(0.5 - D);
    nested_in_the_zero_band: "Sphere(Nested)", Z => two_balls(0.5 - Z);
    clear_of_a_slab_in_band: "Sphere(AgainstPlane)", -D => ball_over_a_slab(D);
    into_a_slab_in_band: "Sphere(AgainstPlane)", D => ball_over_a_slab(-D);
    // The coincfr4 review's off-axis spheres, through the three ops.
    apart_off_axis_in_the_zero_band_union: "Sphere(Apart)", Z => slanted_balls(0, 1.7 + Z);
    apart_off_axis_in_the_zero_band_subtract: "Sphere(Apart)", Z => slanted_balls(1, 1.7 + Z);
    apart_off_axis_in_the_zero_band_intersect: "Sphere(Apart)", Z => slanted_balls(2, 1.7 + Z);
    apart_off_axis_in_band_union: "Sphere(Apart)", D => slanted_balls(0, 1.7 + D);
    apart_off_axis_in_band_subtract: "Sphere(Apart)", D => slanted_balls(1, 1.7 + D);
    nested_off_axis_in_the_zero_band_union: "Sphere(Nested)", Z => nested_balls(0, 0.9 - Z);
    nested_off_axis_in_the_zero_band_subtract: "Sphere(Nested)", Z => nested_balls(1, 0.9 - Z);
    nested_off_axis_in_band_subtract: "Sphere(Nested)", D => nested_balls(1, 0.9 - D);
    // The review's balls under and beside a slab: a rim of the ball's
    // charts against the slab's face.
    ball_under_a_slab_in_band_union: "Coincidence(EdgeOnPlane)", -D => ball_under_a_slab(0, D);
    ball_under_a_slab_in_band_subtract: "Coincidence(EdgeOnPlane)", D =>
        ball_under_a_slab(1, -D);
    ball_beside_a_slab_in_band_union: "Coincidence(EdgeOnPlane)", -D =>
        ball_beside_a_slab(0, D);
    ball_beside_a_slab_in_band_subtract: "Coincidence(EdgeOnPlane)", D =>
        ball_beside_a_slab(1, -D);
    // The review's brick below a tube: its re-run meets the containment,
    // whose own story is CONTACT's row.
    brick_below_a_tube_union: "Coincidence(EdgeOnCurvedFace)", D => brick_below_a_tube(0, D);
}

/// The band every case's first raise runs at.
fn design_band() -> Band {
    Band::new(DESIGN_EPS, geom_core::tolerance::DEFAULT_K * DESIGN_EPS).unwrap()
}

/// Whether `got` is `want` to the precision a fixed pose's margin is
/// computed to.
fn near(got: f64, want: f64) -> bool {
    (got - want).abs() <= 1e-4 * want.abs() + 1e-15
}

/// **Every sphere offer is true of its decision**, and the cases here
/// are the ones `topo`'s census counts as run here: the value each quotes
/// and offers is its computed margin's, and re-run just below it, the
/// raise passes (T1) or refuses on a different decision past which it
/// passes, every refusal met further along telling its own true story or
/// logged under the row that owns it.
#[test]
fn every_sphere_offer_passes_just_below_it() {
    let cases = cases();
    let listed: Vec<(&str, bool, &str)> = cases
        .iter()
        .map(|&(key, margin, name)| (key, margin > 0.0, name))
        .collect();
    assert_eq!(
        listed,
        topo::test_support::OFFERS_EXECUTED_IN_SWEEP,
        "the cases topo's census counts as run here"
    );
    let module = module_path!()
        .split_once("::")
        .map_or(module_path!(), |(_, m)| m);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut false_offers = Vec::new();
    for (key, margin, name) in cases {
        let Executed { chain, verdict } = match execute(
            &format!("{module}::{name}"),
            key,
            topo::test_support::offer_same_decision,
        ) {
            Ok(executed) => executed,
            Err(why) => {
                false_offers.push(why);
                continue;
            }
        };
        let Outcome::Refused { text, .. } = &chain[0].outcome else {
            unreachable!("execute returns a chain that starts with its refusal");
        };
        // `SpheresMeet` states its verdict, not its margin.
        let quoted = text
            .split_once("margin ")
            .and_then(|(_, t)| t.split_whitespace().next())
            .and_then(|m| m.parse::<f64>().ok());
        let offered = test_utils::offer::offered_below(text);
        if !quoted.is_none_or(|m| near(m, margin))
            || !offered.is_some_and(|v| near(v, design_band().tolerance_deciding(margin)))
        {
            false_offers.push(format!(
                "{name}: quotes {quoted:?} and offers {offered:?}, computed margin {margin:e}: \
                 {text}"
            ));
            continue;
        }
        let logs = match &verdict {
            Verdict::T1 => Vec::new(),
            Verdict::T2 { laters } => match test_utils::offer::judge_laters(
                laters,
                geom_core::COINCIDENCE_RECOURSE,
                topo::test_support::LATER_STORIES_OWNED,
            ) {
                Ok(logged) => logged
                    .into_iter()
                    .map(|(key, row)| {
                        assert!(root.join(row).is_file(), "{key}: {row} is not a file");
                        format!("{key} logs {row}")
                    })
                    .collect(),
                Err(why) => {
                    false_offers.push(format!("{name}: {why}"));
                    continue;
                }
            },
        };
        let path: Vec<String> = chain
            .iter()
            .map(|l| match &l.outcome {
                Outcome::Pass => format!("{:e}: pass", l.eps),
                Outcome::Refused { key, .. } => format!("{:e}: {key}", l.eps),
            })
            .collect();
        println!(
            "OFFER {name}: {} {} [the public Boolean]",
            path.join(" -> "),
            logs.join("; ")
        );
    }
    assert!(
        false_offers.is_empty(),
        "false offers:\n{}",
        false_offers.join("\n\n")
    );
}

/// The withdrawn rim offer's child rows: the dome on a tube a hair wider
/// and a hair narrower.
macro_rules! withdrawn {
    ($($name:ident: $margin:expr => $raise:expr;)*) => {
        $(
            #[test]
            #[ignore = "a child row: a_declared_tangent_rim_offers_no_tolerance runs it"]
            fn $name() {
                report(stringify!($name), &outcome($raise));
            }
        )*
        const WITHDRAWN: &[(&str, f64)] = &[$((stringify!($name), $margin)),*];
    };
}

withdrawn! {
    dome_on_a_tube_a_hair_wider: -D => dome_on_a_tube(1.0 + D);
    dome_on_a_tube_a_hair_narrower: D => dome_on_a_tube(1.0 - D);
}

/// **A declared-`Tangent` pair's rim offers no tolerance, since none
/// passes it** (the coincfr4 review's MAJOR-2, on its pose at the
/// door): the dome on a tube declared `Tangent`, the tube's wall a hair
/// wider or narrower than the dome's equator, refuses at the rim's
/// identity, `Coincidence(Rim)`, quoting its margin with no tolerance
/// and naming what the Boolean cannot yet do. The offer it withdrew,
/// executed at 0.9 × `|m|/K`, meets the class refused at the door, and
/// no tolerance down to the harness's floor passes.
#[test]
fn a_declared_tangent_rim_offers_no_tolerance() {
    let module = module_path!()
        .split_once("::")
        .map_or(module_path!(), |(_, m)| m);
    for &(name, margin) in WITHDRAWN {
        let row = format!("{module}::{name}");
        let Outcome::Refused { key, defect, text } = run(&row, DESIGN_EPS) else {
            panic!("{name}: passes at the design tolerance");
        };
        let quoted = text
            .split_once("margin ")
            .and_then(|(_, t)| t.split_whitespace().next())
            .and_then(|m| m.parse::<f64>().ok());
        assert!(
            key == "Coincidence(Rim)"
                && !defect
                && quoted.is_some_and(|m| near(m, margin))
                && !text.contains("tighten")
                && !text.contains("Recourse")
                && text.ends_with(geom_core::NOT_YET_ENDING),
            "{name}: {text}"
        );
        let mut eps = test_utils::offer::BELOW * design_band().tolerance_deciding(margin);
        while eps >= test_utils::offer::FLOOR_EPS {
            let below = run(&row, eps);
            assert!(
                matches!(&below, Outcome::Refused { key, .. } if key == "UnsupportedDeclarationClass"),
                "{name}: at {eps:e} the withdrawn offer meets the class refused: {below:?}"
            );
            eps *= 0.01;
        }
    }
}
