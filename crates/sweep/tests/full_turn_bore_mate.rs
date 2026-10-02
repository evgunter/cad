//! **A shaft set into a full-turn bore, its cylindrical `Rest`
//! declared, unions.**
//!
//! The collar is the rectangle `ρ ∈ [0.5, 1.5]`, `y ∈ [1, 2]` revolved
//! a full turn about `y`: its bore is ONE face with a self-mated seam
//! ruling at azimuth 0, bounded by two rim circles that each carry one
//! vertex. The shaft is the three-arc peg of radius 0.5 (three wall
//! thirds, three seam rulings) turned so its axis is `y`, its first
//! ruling at azimuth `a`. Every bore × peg-wall pair is declared `Rest`.
//!
//! What the mate makes the kernel do, by the shaft's azimuth and span:
//!
//! - a peg ruling passing a rim away from the rim's vertex crosses the
//!   bore's boundary in its own interior (both its ends past the rims
//!   when the shaft runs through);
//! - the bore is one face where the peg is three, so the two solids
//!   divide the contact band differently;
//! - at an azimuth off the collar's seam, the collar's seam vertices sit
//!   inside a peg wall third, with the collar's flat caps leaving them
//!   radially.
//!
//! Every pose of [`crate::common::poses::poses`] moves both operands.
//! The oracle is closed form: the interiors are disjoint, so the union
//! is the collar's annulus volume plus the shaft's disc volume, the
//! intersection is empty, and each difference is its minuend whole.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::mate2_common::{peg_at, wall_decls};
use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, BooleanOp, BooleanResult, mass_properties};

/// The collar's bore and outer radii and its span in `y`.
const BORE: f64 = 0.5;
const OUTER: f64 = 1.5;
const COLLAR: (f64, f64) = (1.0, 2.0);

fn collar() -> Body<f64> {
    let (y0, y1) = COLLAR;
    let lp = ProfileLoop::polygon([
        Point2::new(BORE, y0),
        Point2::new(OUTER, y0),
        Point2::new(OUTER, y1),
        Point2::new(BORE, y1),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// The three-arc peg, first ruling at azimuth `deg` (from `+x`, the
/// collar's seam), spanning `y ∈ [y0, y0 + h]`.
fn shaft(deg: f64, y0: f64, h: f64) -> Body<f64> {
    // A quarter turn about `x` takes the peg's `z` axis to `y` and its
    // sketch azimuth `θ` (from `+x` toward `+y`) to the azimuth `θ` from
    // `+x` toward `−z`, which is the revolve's own sense about `+y`.
    let up = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        -FRAC_PI_2,
    );
    topo::transform_rigid(&peg_at(deg, y0, h), &up, Tol::witness()).unwrap()
}

fn placed(b: &Body<f64>, pose: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, pose, Tol::witness()).unwrap()
}

fn volume(b: &Body<f64>) -> f64 {
    mass_properties(b, Tol::witness()).unwrap().volume
}

/// The collar's closed-form volume, `π(R² − r²)·h`.
fn collar_volume() -> f64 {
    PI * (OUTER * OUTER - BORE * BORE) * (COLLAR.1 - COLLAR.0)
}

/// The shaft's closed-form volume, `π r² h`: its three arcs close one
/// circle.
fn shaft_volume(h: f64) -> f64 {
    PI * BORE * BORE * h
}

/// The shaft spans, by name: through both rims, and exactly the bore.
/// A shaft ending inside the bore is
/// `work/reach/blind-shaft-in-a-full-turn-bore-revisits-the-seam-vertex.md`.
const SPANS: [(&str, f64, f64); 2] = [("through", 0.5, 2.0), ("flush", 1.0, 1.0)];

/// `got` against the closed form, relative to the larger operand.
fn agrees(got: f64, want: f64) -> bool {
    (got - want).abs() <= 1e-12 * collar_volume()
}

/// **The union** at every azimuth, span and pose: a body, its volume
/// the closed form, tier 3 and the pseudomanifold census clean.
fn unions_at(deg: f64) {
    let tol = Tol::witness();
    for (pose_name, pose) in crate::common::poses::poses() {
        let c = placed(&collar(), &pose);
        for (span, y0, h) in SPANS {
            let p = placed(&shaft(deg, y0, h), &pose);
            let tag = format!("azimuth {deg}, {span}, pose {pose_name}");
            let decls = wall_decls(&c, &p);
            assert_eq!(
                decls.coincident_faces.len(),
                3,
                "{tag}: one bore, three walls"
            );
            let bb = match topo::union_with(&c, &p, &decls, tol) {
                Ok(BooleanResult::Body(bb)) => bb,
                other => panic!("{tag}: the mate does not union: {:?}", other.err()),
            };
            let (got, want) = (volume(&bb.body), collar_volume() + shaft_volume(h));
            assert!(agrees(got, want), "{tag}: union volume {got} vs {want}");
            assert_eq!(bb.body.shells().count(), 1, "{tag}: one shell");
            assert_eq!(
                topo::validate_geometric(&bb.body, tol),
                Ok(()),
                "{tag}: tier 3"
            );
            assert_eq!(
                topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol),
                Ok(()),
                "{tag}: the census"
            );
        }
    }
}

#[test]
fn a_shaft_on_the_bores_seam_unions() {
    unions_at(0.0);
}

#[test]
fn a_shaft_off_the_bores_seam_unions() {
    unions_at(60.0);
}

#[test]
fn a_shaft_a_quarter_turn_off_the_bores_seam_unions() {
    unions_at(90.0);
}

/// **The other three ops are never silently wrong.** Each either
/// refuses typed or answers the closed form: `∩` empty, each
/// difference its minuend whole.
#[test]
fn intersect_and_differences_answer_the_closed_form_or_refuse() {
    let tol = Tol::witness();
    let c = collar();
    for deg in [0.0, 60.0] {
        for (span, y0, h) in SPANS {
            let p = shaft(deg, y0, h);
            let tag = format!("azimuth {deg}, {span}");
            let ab = wall_decls(&c, &p);
            let ba = wall_decls(&p, &c);
            let rows = [
                (BooleanOp::Intersect, &c, &p, &ab, None),
                (BooleanOp::Subtract, &c, &p, &ab, Some(collar_volume())),
                (BooleanOp::Subtract, &p, &c, &ba, Some(shaft_volume(h))),
            ];
            for (op, a, b, decls, want) in rows {
                let out =
                    topo::boolean_op_with(op, a, b, decls, topo::SweepStrategy::Realized, tol);
                match (out, want) {
                    (Err(e), _) => eprintln!("{tag}: {op:?} refused {e:?}"),
                    (Ok(BooleanResult::Empty), None) => {}
                    (Ok(BooleanResult::Body(bb)), Some(want)) => {
                        let got = volume(&bb.body);
                        assert!(agrees(got, want), "{tag}: {op:?} volume {got} vs {want}");
                        assert_eq!(topo::validate_geometric(&bb.body, tol), Ok(()), "{tag}");
                    }
                    (Ok(r), _) => panic!("{tag}: {op:?} answered wrong: {:?}", r.body().is_some()),
                }
            }
        }
    }
}
