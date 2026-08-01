//! The rocker plate — the tour's arc-leg fillet stop (M5 S2 + S8).
//!
//! Every corner of this part is authored with `LoopBuilder`'s fillet
//! sugar, and between them they cover the whole corner taxonomy the S2
//! unit opened: **arc×line** (hub → lower flank), **line×line** (the
//! keel knee), **line×arc** (flank → boss), **arc×line** again (boss →
//! upper flank), **line×arc** (flank → hub), and — in the eye-shaped
//! slot through the hub — **arc×arc**, at a corner where TWO tangent
//! circles of the authored radius fit and the S8 rule picks the one
//! nearest the corner the author wrote down.
//!
//! What the sugar buys, in one line: the fillet arc's tangent points
//! and bulge are CONSTRUCTED from the legs (offset carriers, closed
//! form), so both junctions are tangent by construction and are
//! DECLARED as such — and validation then verifies every declaration
//! (`TangencyContradicted` if one is off). Hand-authored via points
//! are what the #99/#100 escalation was about; nothing here is a
//! typed-in tangent point.
//!
//! Constructors are generic over [`Scalar`] (M4 PR 8b): the f64 tour
//! and the Probe K-telemetry sweep build the SAME geometry.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tolerance};
use profile::{
    ArcSweep, FilletLegShape, LoopBuilder, Profile, ProfileLoop, SegmentKind, SketchPlane,
    ValidatedProfile,
};
use sweep::{Extrusion, extrude};

use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};

fn p2<S: Scalar>(x: f64, y: f64) -> Point2<S> {
    Point2::new(S::from_f64(x), S::from_f64(y))
}

fn arc<S: Scalar>(cx: f64, cy: f64) -> FilletLegShape<S> {
    FilletLegShape::Arc {
        center: p2(cx, cy),
        sweep: ArcSweep::Ccw,
    }
}

/// Hub circle: centre (0, 0), R = 2.5 — the plate's big bearing boss.
const HUB: (f64, f64, f64) = (0.0, 0.0, 2.5);
/// Boss circle: centre (7, 0), R = 1.5 — the arm's small end.
const BOSS: (f64, f64, f64) = (7.0, 0.0, 1.5);
/// Blend radius at the four flank/circle junctions.
const R_BLEND: f64 = 0.5;
/// Radius of the knee fillet on the keel (the line×line corner).
const R_KNEE: f64 = 0.5;
/// Radius of the eye slot's rounded tip (the arc×arc corner).
const R_EYE: f64 = 0.25;
/// Half the eye slot's tip separation: the two R = 1 slot carriers sit
/// at (∓1/2, 0), so they cross at (0, ±√(1 − 1/4)) — the vesica of the
/// S8 branch-selection fixture, at half its size.
fn eye_tip() -> f64 {
    0.75f64.sqrt()
}

/// The picked eye fillet's centre, in closed form: the offset carriers
/// are the R − r = 3/4 circles about (∓1/2, 0), so they cross at
/// (0, ±√(9/16 − 1/4)). The NEAR candidate — the one in the same
/// pocket as the authored corner (0, +tip) — is the `+` root; the `−`
/// root is the far pocket, deliberately authorable as the fillet of
/// the slot's OTHER tip (S8 §2).
fn eye_fillet_center_y() -> f64 {
    0.3125f64.sqrt()
}

/// The plate outline, walked counterclockwise from the hub's west
/// point — a start deliberately placed mid-arc, so the loop's first
/// and last segments continue the SAME hub carrier (a same-carrier
/// joint, which is not a tangency and needs no declaration).
///
/// Corner order: hub→keel (arc×line), the keel knee (line×line),
/// keel→boss (line×arc), boss→upper flank (arc×line), flank→hub
/// (line×arc). Every one of them is a fillet; not a single tangent
/// point in this function was computed by hand.
fn outline<S: Scalar>() -> ProfileLoop<S> {
    let (hx, hy, _) = HUB;
    let (bx, by, _) = BOSS;
    let tol = Tolerance::get();
    let start = p2::<S>(-2.5, 0.0);
    LoopBuilder::start(start)
        // hub → keel: the concave blend a web makes into its boss.
        .fillet_corner(
            arc(hx, hy),
            p2(2.0, -1.5),
            FilletLegShape::Line,
            p2(4.0, -2.0),
            S::from_f64(R_BLEND),
            tol,
        )
        .expect("hub→keel blend fits")
        // the keel knee: two straight legs — the line×line door the
        // bracket has used since #101, reached through the same
        // `fillet_corner` call as its arc-leg siblings (two straight
        // legs delegate, so the geometry is bit-identical).
        .fillet_corner(
            FilletLegShape::Line,
            p2(4.0, -2.0),
            FilletLegShape::Line,
            p2(6.1, -1.2),
            S::from_f64(R_KNEE),
            tol,
        )
        .expect("keel knee fillet fits")
        // keel → boss.
        .fillet_corner(
            FilletLegShape::Line,
            p2(6.1, -1.2),
            arc(bx, by),
            p2(6.1, 1.2),
            S::from_f64(R_BLEND),
            tol,
        )
        .expect("keel→boss blend fits")
        // boss → upper flank (the boss arc between the two corners is
        // emitted by the constructors as the trimmed leg).
        .fillet_corner(
            arc(bx, by),
            p2(6.1, 1.2),
            FilletLegShape::Line,
            p2(2.0, 1.5),
            S::from_f64(R_BLEND),
            tol,
        )
        .expect("boss→flank blend fits")
        // upper flank → hub, closing on the hub carrier.
        .fillet_corner(
            FilletLegShape::Line,
            p2(2.0, 1.5),
            arc(hx, hy),
            start,
            S::from_f64(R_BLEND),
            tol,
        )
        .expect("flank→hub blend fits")
        .close_arc_center(p2(hx, hy), ArcSweep::Ccw)
}

/// The eye slot through the hub: the lens of two R = 1 circles about
/// (∓1/2, 0), its TOP tip rounded and its bottom tip left sharp.
///
/// This is the S8 corner. Both legs run tip to tip, so both tangent
/// circles of radius `R_EYE` — one in each tip's pocket — clear the
/// legs' extents and survive the corner-side test. The rule picks the
/// one nearest the authored corner; the sharp bottom tip is where its
/// rival sat.
fn eye<S: Scalar>() -> ProfileLoop<S> {
    let tip = eye_tip();
    LoopBuilder::start(p2(0.0, -tip))
        .fillet_corner(
            arc(-0.5, 0.0),
            p2(0.0, tip),
            arc(0.5, 0.0),
            p2(0.0, -tip),
            S::from_f64(R_EYE),
            Tolerance::get(),
        )
        .expect("the near candidate resolves the eye slot's tip")
        .close_arc_center(p2(0.5, 0.0), ArcSweep::Ccw)
}

/// The validated rocker profile: outline + eye slot.
pub fn profile<S: Scalar>() -> ValidatedProfile<S> {
    Profile::new(SketchPlane::xy(), vec![outline(), eye()])
        .validate(Tolerance::get())
        .expect("the fillet-authored rocker profile validates")
}

/// The plate: profile extruded 1/2 m.
pub fn rocker<S: Scalar>() -> topo::Body<S> {
    extrude(&profile::<S>(), Extrusion::Distance(S::from_f64(0.5)))
        .expect("extrude rocker")
        .body
}

/// The S8 witness, read back off the VALIDATED profile: the one arc
/// segment whose radius is `R_EYE` is the eye fillet, and its centre
/// must be the near root (0, +√(9/16 − 1/4)). Returns the narration
/// line; panics if the far pocket was picked (that would be the branch
/// rule silently changing under the demo).
fn eye_pick_narration(vp: &ValidatedProfile<f64>) -> String {
    let center = vp
        .loops()
        .iter()
        .flat_map(|lp| lp.segments().iter())
        .find_map(|s| match s.kind {
            SegmentKind::Arc { center, radius, .. } if (radius - R_EYE).abs() < 1e-12 => {
                Some(center)
            }
            _ => None,
        })
        .expect("the eye fillet classifies at its authored radius");
    let want = eye_fillet_center_y();
    assert!(
        center.x.abs() < 1e-12 && (center.y - want).abs() < 1e-12,
        "the eye fillet must be the NEAR candidate (0, {want:.6}), got {center:?}"
    );
    format!(
        "the eye slot's top tip had TWO fits of r = {R_EYE}: centres \
         (0, ±{want:.6}). The kernel took the one nearest the corner as \
         authored — centre (0, +{:.6}) — and left the other pocket at the \
         sharp bottom tip, where it is still authorable as THAT tip's own \
         fillet. A pick, never a guess.",
        center.y
    )
}

/// The stop, in tour order (standalone render — the montage sheet is
/// refreshed at the PR 11 demo moment, `curvedcut`).
pub fn stops() -> Vec<Stop> {
    let note = eye_pick_narration(&profile::<f64>());
    vec![Stop {
        name: "rocker",
        caption: "rocker plate — every corner filleted (arc legs included)".to_string(),
        montage: false,
        story: "rocker plate — SIX filleted corners covering the whole taxonomy: \
                arc x line (hub blend), line x line (keel knee), line x arc (boss \
                blend), arc x line (boss exit), line x arc (hub return), and arc x \
                arc at the eye slot's rounded tip",
        ops: "LoopBuilder::fillet_corner on line/arc legs -> Profile::validate \
              -> extrude(Distance), genus 1",
        delta: 5e-3,
        note: Some(note),
        // A plan-leaning camera on purpose: the fillets ARE the stop,
        // and a near-overhead view is where a blend radius reads.
        view: View {
            elev: 68.0,
            azim: -70.0,
            up: 'z',
        },
        bodies: vec![SceneBody::plain("rocker", [0.85, 0.72, 0.32], rocker())],
    }]
}
