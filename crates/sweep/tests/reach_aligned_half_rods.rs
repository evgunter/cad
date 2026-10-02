//! Two stacked rods whose walls are two half-cylinders each
//! (`circle_split(.., 2, ..)`), the upper one's seams turned by θ about
//! the shared axis. The mating disc is a `Rest` contact and the four
//! wall-half pairs that meet across the mating circle are
//! continuations (`crates/topo/README.md`, C4's continuation clause).
//!
//! The declared union builds at every θ through the declared-REST zip
//! (`topo::boolean::rest`). With the seams aligned (θ = 0, and θ = π
//! with the halves swapped) the mating circle carries two sites a half
//! turn apart: each seam segment is a semicircle, so its two end germs
//! are perpendicular to their chord, and the two arcs between the
//! sites are parallel edges told apart by the faces the germs lie on.
//! The oracle is closed form: πr²h per rod.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::circle_split;
use sweep::test_support::{extruded, sketch_at};
use topo::{
    Body, BooleanBody, BooleanCoincidence, BooleanDeclarations, BooleanError, BooleanResult,
    Operand, PlaneRelation,
};

use core::f64::consts::{FRAC_PI_2, PI};

/// The seam turns of the upper rod: aligned, turned, quarter-turned,
/// and aligned again with the two halves exchanged.
const POSES: [(&str, f64); 4] = [
    ("aligned", 0.0),
    ("turned 0.7", 0.7),
    ("quarter turn", FRAC_PI_2),
    ("half turn", PI),
];

fn tol() -> Tol {
    Tol::witness()
}

/// A rod of radius 1 and height 1 standing on `z0`, its wall the two
/// half-cylinders of a rim split at `phase`.
fn rod(z0: f64, phase: f64) -> Body<f64> {
    let rim =
        circle_split(Point2::new(0.0, 0.0), 1.0, 2, phase, tol()).expect("the two-arc rim authors");
    extruded(sketch_at(z0), vec![rim.into()], 1.0, tol())
}

/// Every finding the flush detector offers between `a` and `b`, declared.
fn declared(a: &Body<f64>, b: &Body<f64>) -> BooleanDeclarations {
    let found = topo::flush::find_flush_candidates(a, b, tol()).expect("the rods decide");
    topo::flush::declare_all(&found)
}

fn volume(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, tol()).unwrap().volume
}

/// A boolean that must build a body of volume `expect` (closed form),
/// valid at tier 3 and 3′.
fn builds(
    label: &str,
    out: Result<BooleanResult<f64>, BooleanError>,
    expect: f64,
) -> BooleanBody<f64> {
    let Ok(BooleanResult::Body(bb)) = out else {
        panic!("{label}: {out:?}");
    };
    let v = volume(&bb.body);
    assert!((v - expect).abs() <= 1e-12, "{label}: {v} vs {expect}");
    assert_eq!(
        topo::validate_geometric(&bb.body, tol()),
        Ok(()),
        "{label}: tier 3"
    );
    assert_eq!(
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
        Ok(()),
        "{label}: tier 3′"
    );
    bb
}

/// **The declared union builds at every seam turn**: six faces (top,
/// bottom and the four wall halves, unmerged across the mating circle
/// with the skips recorded), volume 2π.
#[test]
fn a_declared_half_rod_stack_unions_at_every_seam_turn() {
    for (label, theta) in POSES {
        let (a, b) = (rod(0.0, 0.0), rod(1.0, theta));
        let d = declared(&a, &b);
        assert_eq!(
            d.coincident_faces
                .iter()
                .map(|f| f.class == BooleanCoincidence::REST)
                .fold((0, 0), |(r, c), rest| if rest {
                    (r + 1, c)
                } else {
                    (r, c + 1)
                }),
            (1, 4),
            "{label}: the mating disc and four wall continuations"
        );
        let bb = builds(label, topo::union_with(&a, &b, &d, tol()), 2.0 * PI);
        assert_eq!(bb.body.faces().count(), 6, "{label}: faces");
        assert_eq!(
            bb.naming.merge_skipped.len(),
            3,
            "{label}: the declared cylinder pairs and each wall's period closure: {:?}",
            bb.naming.merge_skipped
        );
    }
}

/// **The other three ops keep their typed refusal at every seam
/// turn**: ∩, A ∖ B and B ∖ A refuse `FallbackExtentUnsupported` at the
/// no-crossings path's section certificate, the coaxial wall halves
/// touching across the mating circle with no crossing event — the
/// rounded plate stack's refusal
/// (`work/reach/rounded-stack-subtract-and-intersect-refuse-fallback-extent.md`).
/// Their oracle once built: ∩ empty, each difference its minuend, π.
#[test]
fn a_declared_half_rod_stack_keeps_its_intersect_and_subtract_refusals() {
    for (label, theta) in POSES {
        let (a, b) = (rod(0.0, 0.0), rod(1.0, theta));
        let (ab, ba) = (declared(&a, &b), declared(&b, &a));
        for (op, out) in [
            ("A ∩ B", topo::intersect_with(&a, &b, &ab, tol())),
            ("A ∖ B", topo::subtract_with(&a, &b, &ab, tol())),
            ("B ∖ A", topo::subtract_with(&b, &a, &ba, tol())),
        ] {
            assert!(
                matches!(out, Err(BooleanError::FallbackExtentUnsupported { .. })),
                "{label}: {op}: {out:?}"
            );
        }
    }
}

/// **The union is a legal operand**: a third rod, aligned with the
/// lower one's seams, stacks on top with its own findings declared.
#[test]
fn a_half_rod_stack_takes_a_third_rod() {
    for (label, theta) in POSES {
        let (a, b) = (rod(0.0, 0.0), rod(1.0, theta));
        let stack = builds(
            label,
            topo::union_with(&a, &b, &declared(&a, &b), tol()),
            2.0 * PI,
        )
        .body;
        let c = rod(2.0, 0.0);
        builds(
            &format!("{label}, third rod"),
            topo::union_with(&stack, &c, &declared(&stack, &c), tol()),
            3.0 * PI,
        );
    }
}

/// **Undeclared, or with only the mate declared, the walls refuse** as
/// an undeclared continuation, at every seam turn.
#[test]
fn an_undeclared_half_rod_wall_refuses_at_every_seam_turn() {
    for (label, theta) in POSES {
        let (a, b) = (rod(0.0, 0.0), rod(1.0, theta));
        let all = declared(&a, &b);
        let mut mate = all.clone();
        mate.coincident_faces
            .retain(|f| f.class == BooleanCoincidence::REST);
        for (what, d) in [
            ("undeclared", BooleanDeclarations::default()),
            ("mate only", mate),
        ] {
            let err = topo::union_with(&a, &b, &d, tol()).expect_err("a wall pair is undeclared");
            assert!(
                matches!(
                    err,
                    BooleanError::UndeclaredCoincidence {
                        pair: [(Operand::A, _), (Operand::B, _)],
                        relation: PlaneRelation::SameOriented,
                        ..
                    }
                ),
                "{label}, {what}: an undeclared continuation: {err:?}"
            );
        }
    }
}
