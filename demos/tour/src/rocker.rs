//! The rocker plate — the tour's fillet stop: a plate rounded in the
//! profile and on the solid.
//!
//! Every corner of the outline and the eye slot is authored through the PATHS lattice's
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
//! The keyhole through the arm is the 3-D counterpart: extruded sharp,
//! then its two convex disc/slot creases are rounded by `fillet_edges`
//! on the solid, selected by description (a line between a cylinder
//! and a plane) and checked against the closed form for the section a
//! convex ruled crease's rolling ball removes.
//!
//! Constructors are generic over [`Scalar`] (M4 PR 8b): the f64 tour
//! and the Probe K-telemetry sweep build the SAME geometry.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::prelude::{
    BlendError, Body, Convexity, CurveKind, CurveKindSet, EdgeKey, SurfaceKind, SurfaceKindSet,
    fillet_edges, mass_properties, query, validate_geometric,
};
use pncad::profile::{
    ArcSide, ArcSweep, Center, Open, Profile, ProfileLoop, Radius, SegmentKind, SketchPlane, Start,
    ValidatedProfile,
};
use pncad::sweep::{Extruded, Extrusion, extrude};
use pncad::topo::readback::euler_counts;

use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::authoring::p2;
use pncad::geom_core::{Arc2, Tol};

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
/// Keyhole disc: centre (3.5, −1/4), R = 1/2 — in the arm's web,
/// between the hub and the knee.
const KEY: (f64, f64, f64) = (3.5, -0.25, 0.5);
/// The keyhole slot's half-width.
const KEY_W: f64 = 0.2;
/// How far east of the disc's centre the slot ends.
const KEY_SLOT: f64 = 0.8;
/// The plate's thickness.
const DEPTH: f64 = 0.5;
/// Radius of the 3-D fillet on the keyhole's two convex creases: the
/// eye tip's radius, the largest of the plate's radii the keyhole
/// admits (walls 1 and 2 in [`crease_narration`]).
const R_CREASE: f64 = R_EYE;
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

/// The keyhole through the arm: the [`KEY`] disc and a slot of
/// half-width [`KEY_W`] running east to [`KEY_SLOT`] past the disc's
/// centre.
fn keyhole<S: Scalar>(tol: Tol) -> ProfileLoop<S> {
    let (kx, ky, kr) = KEY;
    let x0 = kx + (kr * kr - KEY_W * KEY_W).sqrt();
    let x1 = kx + KEY_SLOT;
    Open.at(p2(x0, ky + KEY_W))
        .arc_to(
            Center {
                c: p2(kx, ky),
                winding: ArcSweep::Ccw,
                p: p2(x0, ky - KEY_W),
            },
            tol,
        )
        .expect("the disc runs the long way round to the slot's lower wall")
        .line_to(p2(x1, ky - KEY_W), tol)
        .expect("the slot's lower wall")
        .line_to(p2(x1, ky + KEY_W), tol)
        .expect("the slot's end")
        .line_to(Start, tol)
        .expect("the slot's upper wall closes the keyhole")
        .into()
}

/// The validated rocker profile: outline, eye slot, keyhole.
pub fn profile<S: Scalar>(tol: Tol) -> ValidatedProfile<S> {
    Profile::new(
        SketchPlane::xy(),
        vec![outline(tol), eye(tol), keyhole(tol)],
    )
    .validate(tol)
    .expect("the fillet-authored rocker profile validates")
}

/// The plate: the profile extruded [`DEPTH`], every vertical edge
/// still as the extrude left it.
fn plate<S: Scalar>(tol: Tol) -> Extruded<S> {
    extrude(
        &profile::<S>(tol),
        Extrusion::Distance(S::from_f64(DEPTH)),
        tol,
    )
    .expect("extrude rocker")
}

/// Whether `e` is a line between a cylinder and a plane — the
/// kind-pair description of a disc-meets-slot crease, said with the
/// kernel's own query predicates.
fn cylinder_plane_line<S: Scalar>(body: &Body<S>, e: EdgeKey) -> bool {
    query::edge_carrier_matches(body, e, CurveKindSet::just(CurveKind::Line))
        && query::edge_adjacent_matches(
            body,
            e,
            SurfaceKindSet::just(SurfaceKind::Cylinder),
            SurfaceKindSet::just(SurfaceKind::Plane),
        )
}

/// Every edge of the plate the kind-pair description matches: the
/// keyhole's two creases, and the six vertical seams where a profile
/// fillet meets a straight side — which are TANGENT, so the
/// description alone over-selects (see [`stops`]).
fn cylinder_plane_lines<S: Scalar>(body: &Body<S>) -> Vec<EdgeKey> {
    query::all_edges(body)
        .into_iter()
        .filter(|&e| cylinder_plane_line(body, e))
        .collect()
}

/// The keyhole's convex creases: the kind-pair description, scoped to
/// the struts the extrude stood up along the keyhole loop (the last
/// hole, so the last entry of [`Extruded::walls`]). Of the keyhole's
/// four struts, the two where the disc meets a slot wall match; the
/// slot end's two corners are plane×plane.
fn keyhole_creases<S: Scalar>(plate: &Extruded<S>) -> Vec<EdgeKey> {
    let keyhole = plate.walls.last().expect("the plate has hole loops");
    keyhole
        .iter()
        .map(|wall| wall.strut)
        .filter(|&e| cylinder_plane_line(&plate.body, e))
        .collect()
}

/// The rocker: the plate with its keyhole's two convex creases
/// rounded at [`R_CREASE`] by `fillet_edges`.
pub fn rocker<S: Scalar>(tol: Tol) -> Body<S> {
    let plate = plate::<S>(tol);
    fillet_edges(
        &plate.body,
        &keyhole_creases(&plate),
        S::from_f64(R_CREASE),
        tol,
    )
    .expect("the keyhole's creases round at R_CREASE")
    .body
}

/// The area one keyhole crease's fillet removes from the plate's
/// section, in closed form. In the section, about the disc's centre:
/// the crease is `V = (x0, w)` with `x0 = √(R² − w²)`; the rolling
/// ball's centre `c = (cx, w + r)` sits at `R + r` from the centre
/// (outside the disc) and `r` from the slot wall, so
/// `cx = √((R + r)² − (w + r)²)`. Its feet are `F_b = (cx, w)` on the
/// wall and `F_a = c·R/(R + r)` on the disc. The removed region is the
/// quadrilateral `V F_b c F_a` less the ball's sector between its feet
/// and the disc's segment between `V` and `F_a`.
fn crease_cut(r: f64) -> f64 {
    let (_, _, big_r) = KEY;
    let w = KEY_W;
    let x0 = (big_r * big_r - w * w).sqrt();
    let cy = w + r;
    let cx = ((big_r + r).powi(2) - cy * cy).sqrt();
    let s = big_r / (big_r + r);
    let quad = [(x0, w), (cx, w), (cx, cy), (cx * s, cy * s)];
    let twice: f64 = (0..4)
        .map(|i| {
            let (p, q) = (quad[i], quad[(i + 1) % 4]);
            p.0 * q.1 - q.0 * p.1
        })
        .sum();
    // The ball's sector runs from F_b (straight down from c) round to
    // F_a (towards the disc's centre).
    let to_fb = -core::f64::consts::FRAC_PI_2;
    let to_fa = (-cy).atan2(-cx);
    let sector = 0.5 * r * r * (to_fb - to_fa).abs();
    let phi = cy.atan2(cx) - w.atan2(x0);
    let segment = 0.5 * big_r * big_r * (phi - phi.sin());
    0.5 * twice.abs() - sector - segment
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
    let eye_loop = vp.loops().get(1).expect("the eye is the profile's second loop");
    let blends = eye_loop.blend_arcs();
    let [blend] = blends.as_slice() else {
        panic!(
            "the eye slot has exactly one filleted corner, found {}",
            blends.len()
        )
    };
    let SegmentKind::Arc {
        arc: Arc2 { centre, radius, .. },
        ..
    } = blend.kind
    else {
        panic!("a fillet is an arc")
    };
    assert!(
        (radius - R_EYE).abs() < 1e-12,
        "the eye fillet is authored at R_EYE"
    );
    let want = eye_fillet_center_y();
    assert!(
        centre.x.abs() < 1e-12 && (centre.y - want).abs() < 1e-12,
        "the eye fillet must be the NEAR candidate (0, {want:.6}), got {centre:?}"
    );
    format!(
        "the eye slot's top tip had TWO fits of r = {R_EYE}: centres \
         (0, ±{want:.6}). The kernel took the one nearest the corner as \
         authored — centre (0, +{:.6}) — and left the other pocket at the \
         sharp bottom tip, where it is still authorable as THAT tip's own \
         fillet. A pick, never a guess.",
        centre.y
    )
}

/// The keyhole's 3-D fillet, checked against its closed form, and the
/// two radii the plate would naturally take that the kernel refuses,
/// pinned live. Returns the narration line.
fn crease_narration(tol: Tol) -> String {
    let volume = |b: &Body<f64>| {
        mass_properties(b, tol)
            .expect("the rocker's mass properties are closed-form")
            .volume
    };
    let plate = plate::<f64>(tol);

    // The kind-pair description alone also matches the six vertical
    // seams where a profile fillet meets a straight side. Those are
    // tangent, and the selector has no convexity atom to leave them
    // out, so the door refuses the whole request.
    let described = cylinder_plane_lines(&plate.body);
    assert_eq!(
        described.len(),
        8,
        "the description matches the keyhole's two creases and the outline's six tangent seams"
    );
    let over = fillet_edges(&plate.body, &described, R_CREASE, tol);
    assert!(
        matches!(&over, Err(e) if matches!(e.error, BlendError::TangentialEdge { .. })),
        "the description alone hands the door a tangent seam: {:?}",
        over.as_ref().err()
    );

    let creases = keyhole_creases(&plate);
    assert_eq!(creases.len(), 2, "the keyhole's two disc/slot creases");

    // The plate's own blend radius: the ball rolls OUTSIDE the disc's
    // wall, where its curvature sets no limit, but the headroom
    // predicate reads `(1 − r/R)·r` whichever side the ball is on.
    crate::walls::wall(
        "rocker",
        1,
        "round the keyhole's creases at the outline's blend radius R_BLEND = R_disc",
        fillet_edges(&plate.body, &creases, R_BLEND, tol),
        |e| matches!(e.error, BlendError::RadiusHeadroom { .. }),
        "round the creases at R_BLEND",
    );
    // Above r ≈ 0.32 the slot end's corner enters the region the cap
    // meter encloses the sliver with (an annulus about the ball's
    // centre out to the crease), though it stays clear of the sliver
    // itself: the margin is exactly ‖corner − c‖ − ‖V − c‖.
    crate::walls::wall(
        "rocker",
        2,
        "round the keyhole's creases at r = 0.4, where the sliver stays clear of the slot's end",
        fillet_edges(&plate.body, &creases, 0.4, tol),
        |e| {
            matches!(
                e.error,
                BlendError::RingClearance {
                    face,
                    chain: Convexity::Convex,
                    bounded: false,
                    ..
                } if face == plate.bottom || face == plate.top
            )
        },
        "round the creases at the largest radius the slot admits",
    );

    let rounded = fillet_edges(&plate.body, &creases, R_CREASE, tol)
        .expect("the keyhole's creases round at R_CREASE");
    validate_geometric(&rounded.body, tol).expect("the rounded rocker is tier-3 valid");
    let cut = crease_cut(R_CREASE);
    let dv = volume(&rounded.body) - volume(&plate.body);
    let want = -2.0 * cut * DEPTH;
    assert!(
        (dv - want).abs() < 1e-12,
        "the two creases remove 2·A·depth = {want:e}, measured ΔV = {dv:e}"
    );
    let counts = euler_counts(&rounded.body);
    // Each crease's fillet face adds a face, splits its vertical edge
    // in two and both cap vertices in two, joined by a cap arc: +2
    // vertices, +3 edges, +1 face per crease on the plate's 34/51/19.
    assert_eq!(
        (counts.v, counts.e, counts.f, counts.r, counts.genus()),
        (38, 57, 21, 4, Ok(2)),
        "census: the eye and the keyhole make genus 2"
    );
    format!(
        "The keyhole is rounded on the SOLID: after the extrude, `fillet_edges` at \
         r = {R_CREASE} on its two convex disc/slot creases. Selected as `Line` edges \
         between a `Cylinder` and a `Plane`, scoped to the keyhole loop's struts — the \
         description alone also matches the outline's six tangent seams, and the door \
         refuses those (`TangentialEdge`): the selector has no convexity atom. Each crease \
         removes A = {cut:.6e} m² of section, so ΔV = −2·A·{DEPTH} = {want:.6e} m³, \
         measured {dv:.6e}. The outline's blend radius ({R_BLEND}) and r = 0.4 are \
         refused (walls 1 and 2)."
    )
}

/// The stop, in tour order — the montage's fillet cell: the plate
/// rounded in the profile (six corners, 2-D) and on the solid (the
/// keyhole's creases, 3-D).
pub fn stops(tol: Tol) -> Vec<Stop> {
    let note = format!(
        "{} {}",
        eye_pick_narration(&profile::<f64>(tol)),
        crease_narration(tol)
    );
    vec![Stop {
        name: "rocker",
        // Short — montage captions share the panel's width.
        caption: "rocker plate — filleted in 2-D and in 3-D".to_string(),
        montage: true,
        story: "rocker plate — SIX corners filleted in the profile, covering the whole \
                taxonomy: arc x line (hub blend), line x line (keel knee), line x arc \
                (boss blend), arc x line (boss exit), line x arc (hub return), and arc x \
                arc at the eye slot's rounded tip; then a keyhole whose two convex \
                disc/slot creases are filleted on the solid",
        ops: "PATHS fillet doors on line/arc carriers -> Profile::validate \
              -> extrude(Distance) -> query (Line, Cylinder|Plane) on the keyhole's \
              struts -> fillet_edges, genus 2",
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
