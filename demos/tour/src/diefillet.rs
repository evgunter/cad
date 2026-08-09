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

use pncad::geom_core::{Affine3, Band, Point2, Point3, Tolerance, Vec2, Vec3};
use pncad::prelude::{CurveKind, CurveKindSet, Open, Start, SurfaceKind, SurfaceKindSet};
use pncad::profile::{Profile, SketchPlane};
use pncad::sweep::fillet::build::fillet_edges;
use pncad::sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use pncad::topo::Body;
use pncad::topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};

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
    // Algebra-authored (LIB-U2 PR-2).
    let lp = crate::paths::path_polygon(&[(0.0, 0.0), (L, 0.0), (L, L), (0.0, L)]);
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
    // Algebra-authored (LIB-U2 PR-2): the same half-disc — a bulge-1
    // semicircular arc leg to the far pole, straight seam back along
    // the axis (both pole junctions are 90-degree sharp corners; the
    // authored endpoints and the bulge literal are emitted verbatim,
    // so the lowered loop is bit-identical to the raw chain it
    // replaces).
    let lp = Open
        .at(p2(0.0, -PIP_R))
        .arc_to(p2(0.0, PIP_R), S::from_f64(1.0))
        .expect("pip arc leg")
        .line_to(Start)
        .expect("pip seam")
        .into();
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
            pncad::topo::transform_rigid(
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
        pncad::topo::transform_rigid(
            &b,
            &Affine3::rotation_about_axis(origin, rot.normalize(), y.dot(pole).acos()),
        )
        .unwrap()
    };
    pncad::topo::transform_rigid(&placed, &Affine3::translation(c)).unwrap()
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
            &pncad::topo::BooleanDeclarations::none(),
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
        &pncad::topo::BooleanDeclarations::none(),
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
///
/// The two filters below are P10's alphabet, and since LIB-SEL1 they
/// are said in the RATIFIED vocabulary rather than hand-rolled: a
/// [`CurveKindSet`] over the carrier tag, and an unordered
/// [`SurfaceKindSet`] pair across the edge (SELECT-DESIGN §1's two
/// EXACT atoms — tag reads, no funnel, no margin). The ad-hoc `0u8 /
/// 1 / 2` surface encoding this used to carry is gone: these are the
/// same values `select_where` filters with, so the demo and the
/// library can no longer disagree about what "a plane/sphere rim" is.
///
/// **They are still not a `select_where` CALL, and the reason is
/// mechanical, not a deferral.** `select_where` answers against an
/// `Evaluation`, so the die would have to be a RECIPE — and this die
/// cannot be one at byte-identity:
///
/// - the pip ball's meridian is a two-vertex, all-on-axis loop, which
///   `names::name_revolve` refuses typed ("revolve vertex resolution
///   exceeded elimination"); the corpus `die_pips` document works
///   around it with a split-at-equator two-quarter-arc meridian
///   derived from `tan(π/8)`, which is a DIFFERENT body (two band
///   faces, different circle data);
/// - `ball()` picks its placement with a runtime branch that, for the
///   `+y` pole, applies no transform at all, while `Node::Transform`
///   is one unconditional fused `(R, t)` pass — a different
///   re-certification chain, and `transform_rigid` re-mints every
///   curve witness each time it runs.
///
/// So the geometric SELECTOR's acceptance lives where a die IS a
/// recipe: `die_composed`, whose fourteen fillet names these same two
/// atoms materialize exactly (`editor-core/tests/lib_sel1_geoselect.rs`),
/// with the two co-surface meridians falling out as `(Sphere, Sphere)`
/// — the exclusion this comment's ancestor called "standing in for
/// concave rim".
pub fn composed<S: Scalar>() -> Body<S> {
    // The predicate VALUES, stated once — inspectable data, not a
    // closure and not a `matches!` arm buried in a filter.
    let straight = CurveKindSet::just(CurveKind::Line);
    let (rim_a, rim_b) = (
        SurfaceKindSet::just(SurfaceKind::Plane),
        SurfaceKindSet::just(SurfaceKind::Sphere),
    );

    let pipped = pipped::<S>();
    let box_edges: Vec<_> = pipped
        .edges()
        .filter(|(_, e)| {
            pipped
                .get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .is_some_and(|c| straight.contains(CurveKind::of(c.carrier())))
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
                    .map(SurfaceKind::of)
            };
            // UNORDERED, exactly as the atom is: the pair matches
            // whichever half-edge carries which face.
            match (face_kind(e.he_plus), face_kind(e.he_minus)) {
                (Some(p), Some(m)) => {
                    (rim_a.contains(p) && rim_b.contains(m))
                        || (rim_a.contains(m) && rim_b.contains(p))
                }
                _ => false,
            }
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
    let vol = pncad::topo::mass_properties(&blank).unwrap().volume;
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
    let pip_vol = pncad::topo::mass_properties(&pipped).unwrap().volume;
    let composed = composed::<f64>();
    let comp = pncad::topo::mass_properties(&composed).unwrap();
    assert_eq!(
        pncad::topo::validate_geometric(&composed),
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
            // Standalone since the montage-v2 curation (Evan, #218
            // follow-up): the two PARTIAL dice — the blank and the
            // pipped cube — are interesting for how they work (the
            // battery, the closed-group cut), but without that context
            // they read as near-duplicates of the composed die, which
            // remains the sheet's die. Scenes and narration stay alive.
            montage: false,
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
            // Standalone with `diefillet` (see the note there).
            montage: false,
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
