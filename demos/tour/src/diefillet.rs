//! **The die** (M5 PR 12, acceptance shape (v)): constant-radius
//! rolling-ball fillets, made visible.
//!
//! Three stops. At M5 the die was honestly two bodies; M6's
//! composition surgery joins them, and the third stop IS the joined
//! die. The first two stay as the record of the parts:
//!
//! - **the blank** — a unit cube with all twelve edges filleted. Six
//!   shrunk planar faces, twelve quarter-cylinder blends, eight
//!   sphere-octant corner patches: 26 faces, 48 edges, 24 vertices.
//!   Every open chain terminates in a three-convex-edge corner (OQ6's
//!   one in-scope configuration), every trimline is stored
//!   `TangentIntersection` from birth, and the corner trimlines are
//!   CIRCLES — a jet-certificate class this unit had to retire into
//!   the lane before it could store them honestly.
//! - **the pips** — 21 spherical dimples on the six faces of a sharp
//!   cube, cut in ONE certified group operation (S13's closed-group
//!   arm), each ball charted with its pole along the face it is cut
//!   by.
//!
//! - **the composed die** (M6 unit 1) — the pipped cube's twelve box
//!   edges blended IN PLACE (the pip rims carried through as rings)
//!   and all 21 rims replaced by torus bands: one body, tier-3
//!   certified, closed-form volume, watertight. The M5 frontier rows
//!   (`deviation_1_*`) are flipped in
//!   `crates/sweep/tests/m5_pr12_die.rs`; the full ladder lives in
//!   `crates/sweep/tests/m6_surgery.rs`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Band, Point2, Point3, Tolerance, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::fillet::build::fillet_edges;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::Body;
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};

use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};

const L: f64 = 1.0;
const R: f64 = 0.12;
/// The pip-rim blend radius (the composed stop's second call).
const RIM_R: f64 = 0.02;
const PIP_R: f64 = 0.09;
const PIP_H: f64 = 0.05;
const PIP_D: f64 = 0.22;

fn band() -> Band {
    let tol = Tolerance::get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

fn cube<S: Scalar>() -> Body<S> {
    let p2 = |x: f64, y: f64| Point2::new(S::from_f64(x), S::from_f64(y));
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(L, 0.0), p2(L, L), p2(0.0, L)]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&profile, Extrusion::Distance(S::from_f64(L)))
        .unwrap()
        .body
}

/// The blank: every edge of the cube, filleted at radius `R`.
pub fn blank<S: Scalar>() -> Body<S> {
    let body = cube::<S>();
    let edges: Vec<_> = body.edges().map(|(k, _)| k).collect();
    fillet_edges(&body, &edges, S::from_f64(R), band())
        .expect("the die blank fillets")
        .body
}

/// A radius-`PIP_R` ball centred at `c` with its polar axis along
/// `pole` (the chart discipline the plane×sphere section needs).
fn ball<S: Scalar>(c: Vec3<S>, pole: Vec3<S>) -> Body<S> {
    let p2 = |x: f64, y: f64| Point2::new(S::from_f64(x), S::from_f64(y));
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(0.0, -PIP_R),
            bulge: S::from_f64(1.0),
        },
        ProfileVertex {
            pos: p2(0.0, PIP_R),
            bulge: S::from_f64(0.0),
        },
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(S::from_f64(0.0), S::from_f64(1.0)),
    };
    let b = revolve(&vp, axis, Revolution::Full).unwrap().body;
    let y = Vec3::new(S::from_f64(0.0), S::from_f64(1.0), S::from_f64(0.0));
    let rot = y.cross(pole);
    let origin = Point3::new(S::from_f64(0.0), S::from_f64(0.0), S::from_f64(0.0));
    let placed = if rot.norm().lo() < 1e-12 {
        if y.dot(pole).lo() > 0.0 {
            b
        } else {
            topo::transform_rigid(
                &b,
                &Affine3::rotation_about_axis(
                    origin,
                    Vec3::new(S::from_f64(1.0), S::from_f64(0.0), S::from_f64(0.0)),
                    S::from_f64(PI),
                ),
            )
            .unwrap()
        }
    } else {
        topo::transform_rigid(
            &b,
            &Affine3::rotation_about_axis(origin, rot.normalize(), y.dot(pole).acos()),
        )
        .unwrap()
    };
    topo::transform_rigid(&placed, &Affine3::translation(c)).unwrap()
}

fn layout(n: u32) -> Vec<(f64, f64)> {
    let c = vec![(0.0, 0.0)];
    let diag = vec![(-1.0, -1.0), (1.0, 1.0)];
    let anti = vec![(-1.0, 1.0), (1.0, -1.0)];
    let sides = vec![(-1.0, 0.0), (1.0, 0.0)];
    match n {
        1 => c,
        2 => diag,
        3 => [diag.clone(), c].concat(),
        4 => [diag.clone(), anti.clone()].concat(),
        5 => [diag.clone(), anti.clone(), c].concat(),
        _ => [diag, anti, sides].concat(),
    }
}

fn placements<S: Scalar>() -> Vec<(Vec3<S>, Vec3<S>)> {
    let v = |x: f64, y: f64, z: f64| Vec3::new(S::from_f64(x), S::from_f64(y), S::from_f64(z));
    let h = L / 2.0;
    let faces = [
        (1u32, v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0)),
        (6, v(0.0, 0.0, -1.0), v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0)),
        (2, v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)),
        (5, v(-1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)),
        (3, v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0)),
        (4, v(0.0, -1.0, 0.0), v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0)),
    ];
    let mut out = Vec::new();
    for (n, normal, ex, ey) in faces {
        let base = v(h, h, h) + normal * S::from_f64(h + (PIP_R - PIP_H));
        for (u, w) in layout(n) {
            out.push((
                base + ex * S::from_f64(u * PIP_D) + ey * S::from_f64(w * PIP_D),
                normal,
            ));
        }
    }
    out
}

/// The pipped cube: 21 dimples, ONE group subtraction.
pub fn pipped<S: Scalar>() -> Body<S> {
    let places = placements::<S>();
    let mut tool = ball::<S>(places[0].0, places[0].1);
    for (c, n) in &places[1..] {
        tool = boolean_op_with(
            BooleanOp::Union,
            &tool,
            &ball::<S>(*c, *n),
            &topo::BooleanDeclarations::none(),
            SweepStrategy::Realized,
        )
        .expect("the pip tool assembles")
        .body()
        .expect("a body")
        .body
        .clone();
    }
    boolean_op_with(
        BooleanOp::Subtract,
        &cube::<S>(),
        &tool,
        &topo::BooleanDeclarations::none(),
        SweepStrategy::Realized,
    )
    .expect("the pips cut")
    .body()
    .expect("a body")
    .body
    .clone()
}

/// The composed die: pips first (one group cut), then the twelve box
/// edges in place, then all 21 rims as closed chains in one call.
pub fn composed<S: Scalar>() -> Body<S> {
    let pipped = pipped::<S>();
    let box_edges: Vec<_> = pipped
        .edges()
        .filter(|(_, e)| {
            pipped
                .get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .is_some_and(|c| matches!(c.carrier(), geom_curves::Curve3::Line { .. }))
        })
        .map(|(k, _)| k)
        .collect();
    let blanked = fillet_edges(&pipped, &box_edges, S::from_f64(R), band())
        .expect("the box edges blend in place")
        .body;
    let rims: Vec<_> = blanked
        .edges()
        .filter(|(_, e)| {
            let face_kind = |he| {
                let h = blanked.get_half_edge(he)?;
                let f = blanked.get_loop(h.parent_loop)?.face;
                blanked
                    .get_surface(blanked.get_face(f)?.surface)
                    .map(|s| match s {
                        geom_surfaces::Surface::Plane { .. } => 0u8,
                        geom_surfaces::Surface::Sphere { .. } => 1,
                        _ => 2,
                    })
            };
            matches!(
                (face_kind(e.he_plus), face_kind(e.he_minus)),
                (Some(0), Some(1)) | (Some(1), Some(0))
            )
        })
        .map(|(k, _)| k)
        .collect();
    fillet_edges(&blanked, &rims, S::from_f64(RIM_R), band())
        .expect("the rims blend to torus bands")
        .body
}

/// The blank's closed-form volume: core + 6 slabs + 12
/// quarter-cylinders + 8 octants (which sum to one whole ball).
fn blank_volume() -> f64 {
    let core = L - 2.0 * R;
    core.powi(3)
        + 6.0 * R * core.powi(2)
        + 12.0 * (PI * R * R / 4.0) * core
        + (4.0 / 3.0) * PI * R.powi(3)
}

pub fn stops() -> Vec<Stop> {
    let blank = blank::<f64>();
    let vol = topo::mass_properties(&blank).unwrap().volume;
    let want = blank_volume();
    assert!(
        (vol - want).abs() < 1e-9 * want,
        "the blank's volume is a closed form: {vol} vs {want}"
    );
    let (f, e, v) = (
        blank.faces().count(),
        blank.edges().count(),
        blank.vertices().count(),
    );
    assert_eq!((f, e, v), (26, 48, 24));
    let pipped = pipped::<f64>();
    let pip_vol = topo::mass_properties(&pipped).unwrap().volume;
    let composed = composed::<f64>();
    let comp = topo::mass_properties(&composed).unwrap();
    assert_eq!(
        topo::validate_geometric(&composed),
        Ok(()),
        "the composed die is tier-3 valid"
    );
    let (cf, ce, cv) = (
        composed.faces().count(),
        composed.edges().count(),
        composed.vertices().count(),
    );
    assert_eq!((cf, ce, cv), (26 + 21 * 3, 48 + 21 * 7, 24 + 21 * 5));

    vec![
        Stop {
            name: "diefillet",
            caption: "the die blank (rolling-ball fillets)".to_string(),
            montage: true,
            story: "every edge of a cube blended at one radius — twelve quarter-cylinders \
                    and eight sphere-octant corners",
            ops: "fillet_edges(cube, all 12 edges, r = 0.12): battery first, then \
                  plane–plane → cylinder blends with sphere-octant corner patches",
            delta: 5e-3,
            note: Some(format!(
                "the validity battery runs over the INPUTS before a surface exists — six \
                 named Q1 trileans (radius headroom, face consumption, spine regularity, \
                 chain G1, convexity sign, corner configuration); the result is {f} faces, \
                 {e} edges, {v} vertices, every trimline stored TangentIntersection from \
                 birth (the corner ones on CIRCLE carriers, a jet-certificate class this \
                 unit retired into the lane by equivariance), and V = {vol:.6} m³ on the \
                 closed form"
            )),
            view: View {
                elev: 26.0,
                azim: -50.0,
                up: 'z',
            },
            bodies: vec![SceneBody::plain("diefillet", [0.80, 0.72, 0.55], blank)],
        },
        Stop {
            name: "diepips",
            caption: "the die's pips (one group cut)".to_string(),
            montage: true,
            story: "21 spherical dimples on six faces, subtracted as a single 21-shell \
                    operand",
            ops: "cube ∖ (21 disjoint balls): S13's closed-group extent arm, each ball \
                  charted with its pole along the face it is cut by",
            delta: 5e-3,
            note: Some(format!(
                "cutting the pips one at a time would present a TRIMMED sphere face as the \
                 next operand, which S13 refuses typed (the extent certificate needs the \
                 closed-group discipline); charting a ball with a tilted pole would make the \
                 plane×sphere section non-polar, which the split-join refuses typed. Doing \
                 both right makes it one certified operation: V = {pip_vol:.6} m³. At M5 \
                 this and the blank were the die's two halves; the next stop is M6's \
                 composition surgery joining them"
            )),
            view: View {
                elev: 26.0,
                azim: -50.0,
                up: 'z',
            },
            bodies: vec![SceneBody::plain("diepips", [0.62, 0.66, 0.78], pipped)],
        },
        Stop {
            name: "diecomposed",
            caption: "THE COMPOSED DIE (in-place surgery)".to_string(),
            montage: true,
            story: "the filleted blank, the 21 pips and the filleted pip rims in ONE body \
                    — M6's in-place edge-blend composition surgery",
            ops: "cube ∖ pips, then fillet_edges twice IN PLACE: the twelve box edges \
                  (faces split along stored trimlines, rings carried through, octant \
                  corners grafted), then all 21 rims as closed chains → torus bands \
                  (donut-style slit-seamed annuli)",
            delta: 5e-3,
            note: Some(format!(
                "M5 exited with the die as two bodies pinned at two frontiers; both are \
                 retired — door B by the surgery, door A by the circle-carrier \
                 definite-miss rider. The composed body: {cf} faces, {ce} edges, {cv} \
                 vertices, tier-3 certified, V = {:.6} m³ on the closed form \
                 (Steiner blank − 21·(cap + rim-torus term)) at zero enclosure pad",
                comp.volume
            )),
            view: View {
                elev: 26.0,
                azim: -50.0,
                up: 'z',
            },
            bodies: vec![SceneBody::plain(
                "diecomposed",
                [0.85, 0.63, 0.46],
                composed,
            )],
        },
    ]
}
