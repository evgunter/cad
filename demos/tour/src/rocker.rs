//! The rocker plate — the tour's arc-leg fillet stop (M5 S2 + S8).
//!
//! Every corner of this part is authored through the PATHS lattice's
//! fillet doors, and between them they cover the whole corner taxonomy
//! the S2 unit opened: **arc×line** (hub → lower flank), **line×line** (the
//! keel knee), **line×arc** (flank → boss), **arc×line** again (boss →
//! upper flank), **line×arc** (flank → hub), and — in the eye-shaped
//! slot through the hub — **arc×arc**, at a corner where TWO tangent
//! circles of the authored radius fit and the S8 rule picks the one
//! nearest the corner the author wrote down.
//!
//! What the fillet doors buy, in one line: the fillet arc's tangent points
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

use pncad::profile::{
    ArcSide, ArcSweep, Center, Open, Profile, ProfileLoop, Radius, SegmentKind, SketchPlane, Start,
    ValidatedProfile,
};
use pncad::sweep::{Extrusion, extrude};

use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::authoring::p2;
use pncad::geom_core::Tol;

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

/// The plate outline, authored through the PATHS lattice: five fillets
/// between the two circles and the three straight sides, walked
/// counterclockwise from a point on the keel.
///
/// Corner order from the entry: keel→boss (line×arc), boss→upper flank
/// (arc×line), flank→hub (line×arc), hub→lower flank (arc×line), and
/// the keel knee (line×line), which is the SEAM. Every one of them is a
/// fillet; not a single tangent point — and, since the migration, not a
/// single CORNER — in this function was written down. What is authored
/// is what the part is: two circles, three straight sides (each an
/// on-path anchor plus its direction), and the blend radii.
///
/// **Topology, stated as the invariant it now is (LB5).** The outline
/// has TEN vertices: five fillet arcs, each with a trim point ahead of
/// it. The hub arc is ONE segment, and the seam sits on the keel, where
/// `.to(Start)` retrims the entry anchor away. The raw spelling this
/// replaced started mid-hub-arc and so carried an eleventh vertex — a
/// same-carrier joint splitting the hub arc in two, hence one extra
/// lateral face after extrusion. That vertex was the raw builder's
/// seam, not the plate's shape, and the ratified LB5 disposition
/// re-anchors it: the demo's point is the rocker's SHAPE.
///
/// **Derived corners are not authored ones, to the ulp (LB4).** Four of
/// these five corners are arc↔line, where the derived corner lands 0–4
/// ulps off the point a hand author would have transcribed (only the
/// eye's circle×circle corner is structurally exact, by the
/// squared-radius rule). The contract here is #289's — the scene's
/// oracle, verified at its own tolerances — never byte-identity with a
/// previous spelling, and nothing in this file is fitted to make a
/// rounding cancel.
fn outline<S: Scalar>(tol: Tol) -> ProfileLoop<S> {
    let (hx, hy, hr) = HUB;
    let (bx, by, br) = BOSS;
    let blend = S::from_f64(R_BLEND);
    // The three straight sides, each as an on-path anchor and the
    // direction it runs: the keel (entry, heading east into the boss),
    // the upper flank (heading back west into the hub), the lower flank
    // (heading east into the knee). Only the RATIO of a direction is
    // read, so these are the sides' slopes, written exactly.
    Open.at(p2::<S>(5.05, -1.6))
        .toward(S::from_f64(2.1), S::from_f64(0.8), tol)
        .expect("the keel runs east")
        // keel → boss: a straight incoming side, an ARC arrival that
        // rides the boss circle and enters it at its east point — one
        // fused verb, radius and arrival authored together.
        .fillet_arc(
            blend,
            Center {
                c: p2(bx, by),
                winding: ArcSweep::Ccw,
                p: p2(bx + br, by),
            },
            tol,
        )
        .expect("keel→boss blend fits")
        // boss → upper flank: the incoming side runs ON the boss
        // carrier, so the verb re-authors it from the tip's own bits —
        // the carrier radius and the side its centre sits on (left of
        // travel is CCW) — and the arrival is straight.
        .arc_fillet(
            Radius {
                r: S::from_f64(br),
                side: ArcSide::Left,
            },
            blend,
            tol,
        )
        .expect("boss→flank blend fits")
        .at(p2(4.05, 1.35), tol)
        .expect("the upper flank's anchor fits the blend")
        .toward(S::from_f64(-4.1), S::from_f64(0.3), tol)
        .expect("the upper flank runs back west")
        // upper flank → hub: back onto the big circle, entered at its
        // west point — the very point the raw spelling used as its seam,
        // now an anchor and not a vertex.
        .fillet_arc(
            blend,
            Center {
                c: p2(hx, hy),
                winding: ArcSweep::Ccw,
                p: p2(hx - hr, hy),
            },
            tol,
        )
        .expect("flank→hub blend fits")
        // hub → lower flank: arc incoming off the hub carrier, straight
        // arrival, again in one verb.
        .arc_fillet(
            Radius {
                r: S::from_f64(hr),
                side: ArcSide::Left,
            },
            blend,
            tol,
        )
        .expect("hub→keel blend fits")
        .at(p2(3.0, -1.75), tol)
        .expect("the lower flank's anchor fits the blend")
        .toward(S::from_f64(2.0), S::from_f64(-0.5), tol)
        .expect("the lower flank runs east")
        // the keel knee, and the seam: two straight legs, so this is the
        // line×line door the bracket has used since #101, closing onto
        // side 1 through `Start`.
        .fillet(S::from_f64(R_KNEE), tol)
        .expect("a definitely positive knee radius")
        .to(Start, tol)
        .expect("keel knee fillet fits")
        .into()
}

/// The eye slot through the hub: the lens of two R = 1 circles about
/// (∓1/2, 0), its TOP tip rounded and its bottom tip left sharp.
///
/// This is the S8 corner. Both legs run tip to tip, so both tangent
/// circles of radius `R_EYE` — one in each tip's pocket — clear the
/// legs' extents and survive the corner-side test. The rule picks the
/// one nearest the authored corner; the sharp bottom tip is where its
/// rival sat.
///
/// **Authored through the PATHS algebra (LIB-G2 §4)** — the whole loop
/// is ONE fused verb, and NEITHER tip is written down.
///
/// `arc_fillet_arc` states the corner's three parts in a single
/// authoring act: the incoming side ON the right lobe (`Center`, so
/// anchor + centre + winding, the tangent derived), the blend radius,
/// and the arrival on the LEFT lobe closing through `Start`. The top
/// corner is DERIVED as the two carriers' circle×circle intersection —
/// the squared-radius form lands it bitwise on the `(0, √¾)` a hand
/// author would type — and the bottom tip is kept, because the arrival
/// closes on a *different* carrier and that vertex is a genuine
/// two-carrier junction (`to(Start)` would retrim it away).
/// Bit-identity with the raw `fillet_corner` chain is pinned in
/// `profile`'s differential suite.
fn eye<S: Scalar>(tol: Tol) -> ProfileLoop<S> {
    let tip = eye_tip();
    Open.arc_fillet_arc(
        Center {
            c: p2(-0.5, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(0.0, -tip),
        },
        S::from_f64(R_EYE),
        Center {
            c: p2(0.5, 0.0),
            winding: ArcSweep::Ccw,
            p: Start,
        },
        tol,
    )
    .expect("the near candidate resolves the eye slot's tip")
    .into()
}

/// The validated rocker profile: outline + eye slot.
pub fn profile<S: Scalar>(tol: Tol) -> ValidatedProfile<S> {
    Profile::new(SketchPlane::xy(), vec![outline(tol), eye(tol)])
        .validate(tol)
        .expect("the fillet-authored rocker profile validates")
}

/// The plate: profile extruded 1/2 m.
pub fn rocker<S: Scalar>(tol: Tol) -> pncad::topo::Body<S> {
    extrude(
        &profile::<S>(tol),
        Extrusion::Distance(S::from_f64(0.5)),
        tol,
    )
    .expect("extrude rocker")
    .body
}

/// The S8 witness, read back off the VALIDATED profile: the one arc
/// segment whose radius is `R_EYE` is the eye fillet, and its centre
/// must be the near root (0, +√(9/16 − 1/4)). Returns the narration
/// line; panics if the far pocket was picked (that would be the branch
/// rule silently changing under the demo).
fn eye_pick_narration(vp: &ValidatedProfile<f64>) -> String {
    // Which segment is the fillet at the eye's top corner? ASKED
    // (LIB-U5 deliverable 4), not fished for. `blend_arcs` reads the
    // loop's DECLARED tangent joints — an arc tangent at both ends is
    // what a corner fillet leaves behind — so no float is compared to
    // find it. This used to scan every segment of every loop for "the
    // arc whose radius equals R_EYE", which would find the wrong arc
    // the moment two blends shared a radius, and nothing at all if
    // the stored radius drifted an ulp.
    let eye_loop = vp.loops().last().expect("the profile has loops");
    let blends = eye_loop.blend_arcs();
    let [blend] = blends.as_slice() else {
        panic!(
            "the eye slot has exactly one filleted corner, found {}",
            blends.len()
        )
    };
    let SegmentKind::Arc { center, radius, .. } = blend.kind else {
        panic!("a fillet is an arc")
    };
    assert!(
        (radius - R_EYE).abs() < 1e-12,
        "the eye fillet is authored at R_EYE"
    );
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

/// The stop, in tour order — a MONTAGE panel since the M6 curation
/// unit. The `montage: false` it shipped with was a staging leftover
/// from the demo unit ("the sheet refresh rides PR 11"); PR 11
/// refreshed the sheet without flipping the flag back. The rocker is
/// now the sheet's profile-fillet cell, which is what let the bracket
/// cell retire.
pub fn stops(tol: Tol) -> Vec<Stop> {
    let note = eye_pick_narration(&profile::<f64>(tol));
    vec![Stop {
        name: "rocker",
        // Short — montage captions share the panel's width, and this
        // stop became a panel at the M6 curation pass.
        caption: "rocker plate — every corner filleted".to_string(),
        montage: true,
        story: "rocker plate — SIX filleted corners covering the whole taxonomy: \
                arc x line (hub blend), line x line (keel knee), line x arc (boss \
                blend), arc x line (boss exit), line x arc (hub return), and arc x \
                arc at the eye slot's rounded tip",
        ops: "PATHS fillet doors on line/arc carriers -> Profile::validate \
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
        bodies: vec![SceneBody::plain("rocker", [0.85, 0.72, 0.32], rocker(tol))],
    }]
}
