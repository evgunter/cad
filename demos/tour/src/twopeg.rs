//! **The two-peg plate** — the join lane's canonical cell (M9-3 PR-B
//! acceptance fixture (i), here as the scene it was always meant to
//! be): two plates that locate on each other three ways at once, and
//! then become ONE body.
//!
//! Plate P is a 6×4×1 plate with two radius-0.5 pegs standing proud of
//! it; plate Q is the same plate with two through-bores on the same
//! centres. Both outlines are SHARP: the montage-v3 pass attempted the
//! profile fillets Evan asked for and met a wall — see [`outline`],
//! which states it and the controlled pair that isolates it. Set Q down on P and the two parts touch on THREE declared
//! contacts: the mating plane, and each peg's wall against its own
//! bore's wall. One is planar; two are CYLINDRICAL.
//!
//! What the cell shows, at demo altitude:
//!
//! - **P is a boolean result; Q is authored.** P is the plate unioned
//!   with two pegs (transverse curved unions — `bossplate`'s lane). Q's
//!   bores are INNER LOOPS of its sketch: one extrude, no boolean,
//!   genus 2 by construction. Q used to be two subtracts, and the
//!   montage-v3 curation changed it for two reasons — it is what a
//!   drawing says (a plate with two holes in it), and it carries the
//!   fact the retired `plate` cell used to, a profile whose inner loops
//!   extrude to a genus-2 body. It also deletes a dodge: the bore
//!   cutters had to overshoot both faces so no cutter plane would
//!   coincide with a plate plane (#91's design rule), and a
//!   configuration arranged to avoid a refusal is not one a drawing
//!   would ever describe. What it costs, plainly: the mate is now a
//!   boolean of ONE boolean rather than of two. The `subtract` it gave
//!   up is on the sheet several times over — `projectbox` runs
//!   fourteen.
//! - **One rim helper, both sides of the fit.** The peg extrudes the
//!   three-arc `circle_split` loop and Q takes the SAME loop as an
//!   inner ring, so peg wall and bore wall are three faces each and
//!   [`declarations`]'s 3×3 pairing is a fact about the loop rather
//!   than a coincidence between two spellings.
//! - **The mate is DECLARED, never inferred.** The author knows Q is
//!   located on P — the kernel is told, in the author's own words,
//!   which face pairs are in contact and that each contact is a
//!   `Rest`. Value equality never glues; a declaration is what
//!   unlocks the arm, and verification still happens inside the op.
//!   Undeclared, this mate does not even reach the coincidence ladder
//!   the cross-lap's is turned away at — it is refused one stage
//!   earlier, in the reduction's curved-face arm, and the live
//!   narration prints which refusal it actually got.
//! - **The union is exactly additive.** vol(P) + vol(Q) = vol(mated):
//!   the interiors are disjoint, so the glue discards nothing, and the
//!   pegs' π-terms cancel the bores' exactly. The claim is asked of
//!   THREE kernel answers rather than of one answer against a
//!   hand-written constant — which is both its actual content and the
//!   form that would survive corner fillets, since with them a plate
//!   would be `24 − (4 − π)r²` and the mated total would stop being a
//!   dyadic 48.
//!   Each narrated constant is separately pinned to the body it
//!   describes.
//! - **Full engagement deletes the walls.** Each peg fills its bore
//!   completely, so all four cylindrical contact patches are interior
//!   in the result and the bore walls are removed rather than merged:
//!   the finished body carries NO cylindrical face at all. What
//!   survives of each peg is its rim circle, as an inner ring on the
//!   plate's top face.
//!
//! The apart framing is the same two parts, Q lifted by a rigid
//! transform, so a reader can see the three contacts before they
//! become interior.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::geom_core::{Affine3, Point2, Tol, Vec3};
use pncad::profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile, circle_split};
use pncad::sweep::{Extrusion, extrude};
use pncad::topo::{Body, BooleanBody, BooleanDeclarations, ContactClass, FacePairDeclaration};

use crate::booleans::{check, expect_seamed, try_union};
use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};

/// Plate footprint and thickness, peg radius, and the two peg centres
/// — the part's whole dimension set, named once so the closed-form
/// oracle below reads as arithmetic on them rather than as a magic
/// number.
const PLATE: (f64, f64, f64) = (6.0, 4.0, 1.0);
const PEG_R: f64 = 0.5;
/// How near two stored cylinder carriers must agree to be read as ONE.
/// A SELECTION tolerance, not a geometric decision: it picks which face
/// pairs the author means to declare, and the kernel then verifies
/// every declaration it is handed. The neighbouring `plane_face` reads
/// its stations the same way. Both are the plain-body door's missing
/// selector showing through (#1345); a document would say this with a
/// `GeoSelect`.
const SAME_CARRIER: f64 = 1e-12;
/// How near two stored cylinder carriers must agree to be read as ONE.
/// A SELECTION tolerance, not a geometric decision: it picks which face
/// pairs the author means to declare, and the kernel then verifies
/// every declaration it is handed. The neighbouring `plane_face` reads
/// its stations the same way. Both are the plain-body door's missing
/// selector showing through (#1345); a document would say this with a
/// `GeoSelect`.
const PEG_X: [f64; 2] = [2.0, 4.0];
const PEG_Y: f64 = 2.0;
/// How far each peg stands proud of its plate — and, equally, how deep
/// its bore is. FULL ENGAGEMENT: the peg fills the bore exactly, which
/// is what makes every cylindrical contact patch interior.
const ENGAGE: f64 = 1.0;

/// The footprint's area. Sharp corners — see [`outline`] for the
/// profile fillets this scene attempted and the wall they met; with a
/// corner radius r it would carry a `− (4 − π)r²` term.
const PLATE_AREA: f64 = PLATE.0 * PLATE.1;
/// One bare plate.
const PLATE_VOL: f64 = PLATE_AREA * PLATE.2;
/// What two pegs add to P — and, since [`ENGAGE`] equals the plate's
/// own thickness, exactly what two bores take from Q.
const PEG_VOL: f64 = 2.0 * core::f64::consts::PI * PEG_R * PEG_R * ENGAGE;
/// vol(P) = plate + the two pegs' proud stubs; vol(Q) = plate − the
/// two bores. The π-terms are equal and opposite, so the mated pair is
/// exactly two plates' worth of material.
///
/// **Each is asserted against the body it describes** ([`build`]), not
/// only against their sum. A per-part constant that is printed but
/// pinned by nothing can be wrong by a factor and stay green as long as
/// the error cancels in the total — which is the shape of the only bug
/// this scene has had.
const V_P: f64 = PLATE_VOL + PEG_VOL;
const V_Q: f64 = PLATE_VOL - PEG_VOL;
const V_MATED: f64 = 2.0 * PLATE_VOL;

/// The plate outline.
///
/// # The corner fillets, attempted and REFUSED (#1352)
///
/// Evan's montage-v3 note asked for the extruded PROFILE to be
/// filleted — `bracket`/`rocker`'s PATHS line×line door, four rounded
/// corners, top and bottom edges left sharp so both mating faces stay
/// the SAME rounded rectangle. It authors cleanly and both plates
/// build. **The MATE then refuses, declared or not**, and the reason is
/// a fact about this kernel worth stating rather than a fact about this
/// scene:
///
/// P spans `z ∈ [0, 1]` and Q spans `[1, 2]` on the same footprint, so
/// the two parts' outer walls are COSURFACE with DISJOINT extents —
/// same carrier, same outward sense, meeting only along the mating
/// plane. While those walls are PLANES that passes the reduction with
/// no declaration at all, which is why three declared contacts used to
/// be enough. Round the corners and four of them become CYLINDERS, and
/// the curved arm refuses the same configuration:
/// `CurvedPierceUnsupported`, with the identical payload the UNDECLARED
/// mate gets — and declaring them does not help, because
/// [`ContactClass::Rest`] means *opposed* senses and these two walls
/// face the same way. There is no class in the vocabulary for a
/// cosurface CONTINUATION, and the curved arm has no arm for one.
///
/// Measured as a controlled pair, both halves run in this scene's own
/// history: sharp outline + the same everything else GLUES; filleted
/// outline + 22 declared cylindrical `Rest`s (the four corner pairs
/// included, matched by carrier) REFUSES. The only difference is the
/// four corner cylinders.
///
/// Recorded, not worked around (`memories/demo-purpose.md`): the plate
/// ships SHARP, the fillets wait on the wall, and the scene does not
/// contort itself — no mismatched radii between the two plates, no
/// one-plate-only rounding — to manufacture a shape that dodges it.
fn outline<S: Scalar>(tol: Tol) -> ProfileLoop<S> {
    crate::paths::path_polygon(
        &[
            (0.0, 0.0),
            (PLATE.0, 0.0),
            (PLATE.0, PLATE.1),
            (0.0, PLATE.1),
        ],
        tol,
    )
}

/// The radius-0.5 circle about `(cx, PEG_Y)`, authored as THREE 120°
/// arcs of one carrier — `circle_split`, as `bossplate` does, because
/// the split count is part of what the seam looks like and must be said
/// out loud.
///
/// ONE helper for BOTH sides of the fit, deliberately: the peg extrudes
/// this loop, and plate Q takes the same loop as an inner ring, so peg
/// wall and bore wall are three faces each and [`declarations`]'s 3×3
/// pairing is a fact about the loop rather than a coincidence between
/// two spellings.
fn rim<S: Scalar>(cx: f64, tol: Tol) -> ProfileLoop<S> {
    circle_split(
        Point2::new(S::from_f64(cx), S::from_f64(PEG_Y)),
        S::from_f64(PEG_R),
        3,
        S::from_f64(0.0),
        tol,
    )
    .expect("the three-arc peg rim authors")
    .into()
}

/// The plate's sketch at `z0` — the outline, plus a bore rim per peg
/// centre when `bores` is set.
fn plate_profile<S: Scalar>(z0: f64, bores: bool, tol: Tol) -> ValidatedProfile<S> {
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(
        S::from_f64(0.0),
        S::from_f64(0.0),
        S::from_f64(z0),
    )));
    let mut loops = vec![outline::<S>(tol)];
    if bores {
        loops.extend(PEG_X.into_iter().map(|cx| rim::<S>(cx, tol)));
    }
    Profile::new(plane, loops)
        .validate(tol)
        .expect("the plate profile validates")
}

/// A plate: the 6×4 footprint, thickness 1, sketched at `z0`.
fn plate<S: Scalar>(z0: f64, tol: Tol) -> Body<S> {
    extrude(
        &plate_profile::<S>(z0, false, tol),
        Extrusion::Distance(S::from_f64(PLATE.2)),
        tol,
    )
    .expect("the plate extrudes")
    .body
}

/// A peg: [`rim`] extruded `h` from `z0`.
fn peg<S: Scalar>(cx: f64, z0: f64, h: f64, tol: Tol) -> Body<S> {
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(
        S::from_f64(0.0),
        S::from_f64(0.0),
        S::from_f64(z0),
    )));
    let profile = Profile::new(plane, vec![rim::<S>(cx, tol)])
        .validate(tol)
        .expect("the peg profile validates");
    extrude(&profile, Extrusion::Distance(S::from_f64(h)), tol)
        .expect("the peg extrudes")
        .body
}

/// Plate P: the plate, with a peg unioned on at each centre. Each peg
/// is sketched INSIDE the plate and extruded through its top, so both
/// unions are TRANSVERSE curved booleans — the same op `bossplate`
/// shows on its own.
fn plate_with_pegs<S: Scalar>(tol: Tol) -> Body<S> {
    let plain = PLATE_VOL;
    let stub = core::f64::consts::PI * PEG_R * PEG_R * ENGAGE;
    let mut body = plate::<S>(0.0, tol);
    for (i, cx) in PEG_X.into_iter().enumerate() {
        // Sketched at z = 0.4 (strictly inside the plate) and run to
        // z = 1 + ENGAGE, so the part standing proud of the plate is
        // exactly the bore's depth.
        let boss = peg::<S>(cx, 0.4, PLATE.2 - 0.4 + ENGAGE, tol);
        let want = plain + (i as f64 + 1.0) * stub;
        body = expect_seamed(
            "peg boss union (transverse curved)",
            check(try_union(&body, &boss, tol), want, tol),
            want,
        )
        .body;
    }
    body
}

/// Plate Q: the same plate a storey up, its two bores authored as
/// INNER LOOPS of the sketch — one extrude, no boolean.
///
/// **Why this is not two subtracts** (montage-v3, Evan): it is what a
/// machinist's drawing says — a plate with two holes in it — and it is
/// the one thing the retired `plate` cell was carrying, a profile whose
/// inner loops extrude to a genus-2 body. It also deletes a dodge: the
/// bore cutters used to overshoot both faces so that no cutter plane
/// would coincide with a plate plane (the #91 design rule), and a
/// configuration that has to be arranged to avoid a refusal is not one
/// a drawing would ever describe.
///
/// What it costs, said plainly: P is still a boolean result (two
/// transverse curved unions) but Q no longer is, so the mate is a
/// boolean of ONE boolean rather than of two. The `subtract` op it gave
/// up is on the sheet several times over — `projectbox` alone runs
/// fourteen.
fn plate_with_holes<S: Scalar>(tol: Tol) -> Body<S> {
    extrude(
        &plate_profile::<S>(PLATE.2, true, tol),
        Extrusion::Distance(S::from_f64(PLATE.2)),
        tol,
    )
    .expect("the holed plate extrudes")
    .body
}

/// Every cylindrical face of `body`, with its carrier's stored axis
/// origin in `x`/`y` and its radius — enough to tell two coaxial walls
/// from two parallel ones, since every cylinder in this scene is
/// vertical.
///
/// **A library finding, recorded at the site it was met** (the demos'
/// purpose rule): the author knows perfectly well WHICH contacts he
/// means — "each peg against its own bore, and the two plates' outer
/// walls where they continue across the joint" — but there is no
/// selector on the plain body API to say it with. The intent has to be
/// re-derived from stored surface parameters by walking every face in
/// the arena, and it comes back as THREE faces per peg side (the
/// three-arc split), so ONE contact the author has in mind is spelled
/// as NINE `FacePairDeclaration`s. The document layer has selection
/// (`GeoSelect`); the kernel-level `Body` does not, and a declared
/// contact is a kernel-level object — the two-doors gap, #1345.
/// `crate::booleans::flush_declarations` is the same gap answered for
/// PLANES only, and deliberately not widened here: it is called by
/// scenes whose contacts must keep refusing (the lily's stem glue), so
/// a curved arm on it would move a wall rather than build a part.
fn cylinders<S: Scalar>(body: &Body<S>) -> Vec<(pncad::topo::FaceKey, f64, f64, f64)> {
    body.faces()
        .filter_map(|(k, f)| match body.get_surface(f.surface) {
            Some(&pncad::geom::Surface::Cylinder { origin, radius, .. }) => {
                Some((k, origin.x.f(), origin.y.f(), radius.f()))
            }
            _ => None,
        })
        .collect()
}

/// The one planar face of `body` at height `z` whose outward normal
/// points up (`up`) or down. Same finding as [`cylinders`].
fn plane_face<S: Scalar>(body: &Body<S>, z: f64, up: bool) -> pncad::topo::FaceKey {
    let hits: Vec<_> = body
        .faces()
        .filter(|(_, f)| match body.get_surface(f.surface) {
            Some(pncad::geom::Surface::Plane { origin, normal, .. }) => {
                (origin.z.f() - z).abs() < 1e-12 && (normal.z.f() > 0.5) == up
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .collect();
    let [f] = hits[..] else {
        panic!("expected exactly one z = {z} face (up = {up}), got {hits:?}");
    };
    f
}

/// The mate, in the author's own words: the mating plane, plus every
/// place the two parts' walls lie on a COMMON cylinder — each a `Rest`.
///
/// Contacts are matched by CARRIER, not by name, which is both the
/// honest statement and the one that stays right as the part changes.
/// Cross-peg pairs are excluded for free: peg 1 and bore 2 sit on
/// DISTINCT carriers, and declaring them would be a false statement the
/// kernel would contradict — correctly.
///
/// Matching by carrier rather than by peg centre is what let the
/// montage-v3 pass MEASURE the corner-fillet wall [`outline`] records:
/// with the fillets on, this function declares the four corner-wall
/// pairs too — 22 cylindrical `Rest`s rather than 18 — and the mate
/// refuses anyway, which is what makes the wall a kernel fact rather
/// than a missing declaration.
fn declarations<S: Scalar>(p: &Body<S>, q: &Body<S>) -> BooleanDeclarations {
    let mut decls = BooleanDeclarations::none();
    // 1. The mating plane: P's top face against Q's bottom face.
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(p, PLATE.2, true),
        plane_face(q, PLATE.2, false),
        ContactClass::Rest,
    ));
    // 2. Every shared-carrier cylinder pair: the two peg fits (three
    // faces a side, so nine declarations each) and the four corner
    // walls the profile fillets mint.
    let (cp, cq) = (cylinders(p), cylinders(q));
    for &(fa, ax, ay, ar) in &cp {
        for &(fb, bx, by, br) in &cq {
            let same = (ax - bx).abs() < SAME_CARRIER
                && (ay - by).abs() < SAME_CARRIER
                && (ar - br).abs() < SAME_CARRIER;
            if same {
                decls
                    .coincident_faces
                    .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
            }
        }
    }
    decls
}

/// The cell's boolean work, generic (the K-probe sweep runs the same
/// ops): both parts, the UNDECLARED refusal, the DECLARED mate, and
/// the lifted copy for the apart framing. Returns the undeclared
/// refusal's narration for the f64 captions.
pub(crate) fn build<S: Scalar>(tol: Tol) -> (Body<S>, Body<S>, BooleanBody<S>, Body<S>, String) {
    let p = plate_with_pegs::<S>(tol);
    let q = plate_with_holes::<S>(tol);

    // UNDECLARED, the mate refuses — and it refuses EARLIER than the
    // cross-lap's does, which is worth saying because the two look
    // alike. The cross-lap's planar mate reaches the coincidence
    // ladder and is turned away there (value equality never
    // classifies); this one never gets that far. The reduction's
    // curved-face arm meets a bore rim circle sitting ON the peg's
    // carrier, decides zero clearance, and takes
    // `CurvedPierceUnsupported` before a single patch is discovered.
    // What the declaration unlocks is therefore that ARM, not just
    // the front door — M9-3 PR-A's rung, seen from the outside.
    let naive = check(try_union(&p, &q, tol), V_MATED, tol);
    let refusal = crate::booleans::describe(&naive, V_MATED);
    if !matches!(naive, crate::booleans::Verdict::Refused(_)) {
        panic!(
            "the UNDECLARED two-peg mate no longer refuses ({refusal}) — \
             a declaration must be what unlocks the arm, never a measurement); regression"
        );
    }
    println!("   two-peg mate WITHOUT declarations: {refusal}");

    let decls = declarations(&p, &q);
    println!(
        "   declared: {} face pairs — the mating plane, and every shared-carrier \
         cylinder pair (P has {} cylinder faces, Q has {}); cross-peg pairs never \
         arise, since peg 1 and bore 2 sit on distinct carriers",
        decls.coincident_faces.len(),
        cylinders(&p).len(),
        cylinders(&q).len()
    );
    let mated = expect_seamed(
        "declared two-peg mate (M9-3: the mating plane, and every shared-carrier \
         cylinder pair)",
        check(pncad::topo::union_with(&p, &q, &decls, tol), V_MATED, tol),
        V_MATED,
    );
    // THE ADDITIVITY CLAIM, ASKED OF THREE KERNEL ANSWERS rather than of
    // one answer against a hand-written constant. That is the claim's
    // actual content — the interiors are disjoint, so the glue discards
    // nothing — and asking it of the bodies rather than of 48 is what
    // makes it a statement about the OP instead of about this
    // footprint's arithmetic. It is asserted BITWISE, which is what the
    // kernel delivers here.
    let vol = |b: &Body<S>| {
        pncad::topo::mass_properties(b, tol)
            .expect("mass properties")
            .volume
            .f()
    };
    let (vp, vq, v) = (vol(&p), vol(&q), vol(&mated.body));
    let additive = vp + vq;
    assert_eq!(
        v, additive,
        "the two-peg mate is EXACTLY additive: vol(mated) must equal \
         vol(P) + vol(Q) = {vp} + {vq} BITWISE — the interiors are disjoint, so the \
         glue discards nothing and the pegs' pi-terms cancel the bores'"
    );
    // AND every constant this scene NARRATES, each pinned against the
    // body it describes rather than against the total — because an
    // error in a per-part constant cancels in the sum, so the total
    // cannot catch one (see [`V_P`]).
    for (name, measured, narrated) in [("P", vp, V_P), ("Q", vq, V_Q), ("the mate", v, V_MATED)] {
        assert!(
            (measured - narrated).abs() <= 1e-9,
            "{name}: the scene narrates {narrated} and the body measures {measured}"
        );
    }
    println!(
        "   volumes: P = {vp}, Q = {vq}, mated = {v}; additive to {:.2e} \
         (exactly {}), and each against its own closed form",
        (v - additive).abs(),
        if v == additive {
            "bitwise"
        } else {
            "within the gate"
        }
    );
    // Full engagement: every cylindrical patch is interior, so the
    // walls are REMOVED rather than merged and no cylinder survives.
    assert!(
        mated.body.faces().all(|(_, f)| !matches!(
            mated.body.get_surface(f.surface),
            Some(pncad::geom::Surface::Cylinder { .. })
        )),
        "full-engagement patch removal deletes every bore wall"
    );
    println!(
        "   two-peg mate WITH the three contacts declared: GLUED — volume {V_MATED} \
         exactly (vol P + vol Q = {V_P} + {V_Q}), every bore wall interior"
    );

    // The apart framing: Q lifted by a rigid transform (#84 — every
    // moved edge witness is re-minted, and the moved body revalidates).
    let lift = Affine3::translation(Vec3::new(
        S::from_f64(0.0),
        S::from_f64(0.0),
        S::from_f64(1.6),
    ));
    let q_lifted = pncad::topo::transform_rigid(&q, &lift, tol).expect("lift plate Q");
    (p, q, mated, q_lifted, refusal)
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let (p, _q, mated, q_lifted, refusal) = build::<f64>(tol);
    let note = format!(
        "the mate declared in the author's terms — the mating plane, and each peg \
         against its own bore — which the plain-body door can only spell as ONE planar \
         Rest and EIGHTEEN cylindrical ones, three faces a side per fit (#1345); \
         undeclared the mate \
         refuses ({refusal}); declared, the M9-3 zip GLUES it: volume {V_MATED} \
         exactly, and exactly additive — vol(P) + vol(Q) = ({V_P}) + ({V_Q}) = \
         {V_MATED}, the pegs' pi-terms cancelling the bores' bitwise. Full \
         engagement removes all four cylindrical patches, so the finished body \
         carries NO cylinder face; each peg survives as a rim circle, an inner \
         ring on the plate's top"
    );
    vec![
        Stop {
            name: "twopeg",
            caption: "two-peg plate (mated)".to_string(),
            montage: true,
            story: "two plates located on each other three ways — one planar and two \
                    CYLINDRICAL declared Rest contacts — and UNIONED into one body \
                    through the M9-3 zip; the peg-in-hole join this tour used to say \
                    it could not build",
            ops: "extrude plate + 2 x extrude three-arc peg -> 2 transverse unions (P); \
                  extrude one profile whose two inner loops are the bores (Q); declare \
                  every shared-carrier face pair Rest -> union_with",
            delta: 1e-2,
            note: Some(note),
            view: View {
                elev: 22.0,
                azim: -58.0,
                up: 'z',
            },
            bodies: vec![SceneBody::seamed(
                "twopeg_mated",
                [0.62, 0.66, 0.72],
                mated.body,
                mated.contacts,
            )],
        },
        Stop {
            name: "twopeg_apart",
            caption: "two-peg plate (apart)".to_string(),
            montage: true,
            story: "the same two parts apart: plate P with its two pegs, plate Q with \
                    its two bores lifted clear, so the three contacts are visible \
                    before the union makes them interior",
            ops: "transform_rigid(plate Q, +1.6 z) — transform witnesses re-minted",
            delta: 1e-2,
            note: None,
            view: View {
                elev: 22.0,
                azim: -58.0,
                up: 'z',
            },
            bodies: vec![
                SceneBody::plain("twopeg_apart_p", [0.62, 0.66, 0.72], p),
                SceneBody::plain("twopeg_apart_q", [0.78, 0.60, 0.42], q_lifted),
            ],
        },
    ]
}
