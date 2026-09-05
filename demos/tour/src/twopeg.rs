//! **The two-peg plate** — the join lane's canonical cell (M9-3 PR-B
//! acceptance fixture (i), here as the scene it was always meant to
//! be): two plates that locate on each other three ways at once, and
//! then become ONE body.
//!
//! Plate P is a 6×4×1 plate with two radius-0.5 pegs standing proud of
//! it; plate Q is the same plate with two through-bores on the same
//! centres. Both outlines are SHARP: the montage-v3 pass attempted the
//! profile fillets Ev asked for and met a wall — see [`outline`],
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

use pncad::geom_brep::SurfaceKind;
use pncad::geom_core::{Affine3, Point2, Tol, Vec3};
use pncad::prelude::{SurfaceKindSet, query};
use pncad::profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile, circle_split};
use pncad::sweep::{Extrusion, extrude};
use pncad::topo::{Body, BooleanBody, BooleanDeclarations};

use crate::booleans::{check, expect_seamed, try_union};
use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};

/// Plate footprint and thickness, peg radius, and the two peg centres
/// — the part's whole dimension set, named once so the closed-form
/// oracle below reads as arithmetic on them rather than as a magic
/// number.
const PLATE: (f64, f64, f64) = (6.0, 4.0, 1.0);
const PEG_R: f64 = 0.5;
/// How far along +x the apart framing sits from the mated body. The
/// plate is 6 wide, so 8 leaves 2 of clear air between the two
/// framings at the shared camera.
const APART_GAP: f64 = 8.0;
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
/// Ev's montage-v3 note asked for the extruded PROFILE to be
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
/// **Why this is not two subtracts** (montage-v3, Ev): it is what a
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

/// Every cylindrical face of `body` — the kernel's own kind read
/// (`query::face_surface_matches`), for the narration's face counts
/// and for saying WHICH of the detector's findings the author means.
///
/// **A library finding, recorded at the site it was met** (the demos'
/// purpose rule), and it is now the SELECTION half alone. The author
/// knows perfectly well which contacts he means — "each peg against
/// its own bore, and the two plates' outer walls where they continue
/// across the joint" — but there is no selector on the plain body API
/// to say it with, so the intent is re-derived by walking the arena
/// and filtering. What retired with the flush detector's curved rungs
/// is the other half: this scene used to MATCH the peg walls to the
/// bore walls itself, comparing stored axis origins and radii against
/// a hand-picked `1e-12`, and the kernel's own carrier ladder decides
/// that now — cross-peg pairs come back `Distinct` from the same door
/// that verifies the declaration, rather than from a tolerance this
/// file chose. One contact the author has in mind is still spelled as
/// NINE `FacePairDeclaration`s (three faces a side, the three-arc
/// split), because the split is the loop's and no door groups faces
/// by contact. The document layer has selection (`GeoSelect`); the
/// kernel-level `Body` does not, and a declared contact is a
/// kernel-level object — the two-doors gap, #1345.
fn cylinders<S: Scalar>(body: &Body<S>) -> Vec<pncad::topo::FaceKey> {
    query::all_faces(body)
        .into_iter()
        .filter(|&k| {
            query::face_surface_matches(body, k, SurfaceKindSet::just(SurfaceKind::Cylinder))
        })
        .collect()
}

/// The one planar face of `body` at height `z` whose outward normal
/// points up (`up`) or down — a POSITIONAL pick, by stored plane
/// parameters, and it stays one.
///
/// Same finding as [`cylinders`], at the half of it the flush detector
/// does not close: the detector says which face pairs WOULD verify,
/// and this says which of them the author meant. Nothing on the plain
/// body API says "the mating face" — a document would say it with a
/// `GeoSelect` — so the scene says it in coordinates and the kernel
/// verifies the declaration that results.
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
/// **Every declaration here is a finding the kernel vouched for.**
/// `find_flush_candidates` reports the cross-body pairs its own verify
/// door would accept — planar AND curved, since the detector's reach
/// is the `Rest` ladder's reach — and this function DECLARES a subset
/// of that report: the mating plane, picked positionally because
/// nothing on the plain-body API says "the mating face", and every
/// cylindrical finding, which is what "each peg against its own bore"
/// means here. Nothing is asserted that the detector did not first
/// report, which is the no-fusion boundary doing its job (the findings
/// are values, inspected in between).
///
/// Contacts are matched by CARRIER, and now by the KERNEL'S carrier
/// ladder rather than by this file's arithmetic: cross-peg pairs are
/// excluded because peg 1 and bore 2 are `Distinct` at the same door
/// that verifies the declaration — the scene no longer compares stored
/// axis origins against a hand-picked tolerance to reach the same
/// answer. What the scene still chooses is WHICH findings it means.
///
/// Matching by carrier rather than by peg centre is what let the
/// montage-v3 pass MEASURE the corner-fillet wall [`outline`] records:
/// with the fillets on, the detector reports the four corner-wall
/// pairs too — 22 cylindrical `Rest`s rather than 18 — and the mate
/// refuses anyway, which is what makes the wall a kernel fact rather
/// than a missing declaration.
fn declarations<S: Scalar>(p: &Body<S>, q: &Body<S>, tol: Tol) -> BooleanDeclarations {
    let found = pncad::topo::flush::find_flush_candidates(p, q, tol)
        .expect("the plates' pairs are authored exactly, so they decide definitely");
    // The mating plane: P's top face against Q's bottom face. The
    // report holds the plates' other real contacts too — the flush
    // side walls, the peg tops flush with Q's top face — which this
    // part does not mate on, and nothing but the author knows that.
    // Detection and selection are two different missing doors and only
    // the first one shipped.
    let mating = (plane_face(p, PLATE.2, true), plane_face(q, PLATE.2, false));
    let cyl = cylinders(p);
    let picked: Vec<_> = found
        .into_iter()
        .filter(|f| f.pair == mating || cyl.contains(&f.pair.0))
        .collect();
    assert!(
        picked.iter().any(|f| f.pair == mating),
        "the mating plane must be a finding: P's top face rests on Q's bottom face"
    );
    pncad::topo::flush::declare_all(&picked)
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

    let decls = declarations(&p, &q, tol);
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
    // The apart framing is placed BESIDE the mated one, not in a cell
    // of its own: the two are one statement — these parts, and what
    // becomes of their three contacts when the union makes them
    // interior — and `compose_montage.py` scales every cell
    // independently, so as two panels they arrive at two different
    // sizes and the reader cannot lay one over the other.
    //
    // The MATED body stays at the origin because the scene's closed
    // forms are stated in its coordinates. The apart pair is already a
    // FRAMING rather than a part — Q is there by a rigid lift — so
    // moving it again is the same kind of act, not a new claim.
    //
    // (`transform_rigid` would carry either: it leaves the topology
    // "and every arena key untouched", so a contact-carrying body
    // survives it. What it re-mints is each moved edge's WITNESS, #84.)
    let aside = Affine3::translation(Vec3::new(APART_GAP, 0.0, 0.0));
    let p_aside = pncad::topo::transform_rigid(&p, &aside, tol).expect("place P aside");
    let q_aside = pncad::topo::transform_rigid(&q_lifted, &aside, tol).expect("place Q aside");
    vec![Stop {
        name: "twopeg",
        caption: "two-peg plate — mated, and apart".to_string(),
        montage: true,
        story: "two plates located on each other three ways — one planar and two \
                CYLINDRICAL declared Rest contacts — and UNIONED into one body through \
                the M9-3 zip; the peg-in-hole join this tour used to say it could not \
                build. Beside it the same two parts apart, Q lifted clear, so the three \
                contacts are visible before the union makes them interior",
        ops: "extrude plate + 2 x extrude three-arc peg -> 2 transverse unions (P); \
              extrude one profile whose two inner loops are the bores (Q); \
              find_flush_candidates -> pick the mating plane and the two peg fits \
              out of the report -> declare_all -> union_with; transform_rigid for \
              the apart framing",
        delta: 1e-2,
        note: Some(note),
        view: View {
            elev: 22.0,
            azim: -58.0,
            up: 'z',
        },
        bodies: vec![
            SceneBody::seamed(
                "twopeg_mated",
                [0.62, 0.66, 0.72],
                mated.body,
                mated.contacts,
            ),
            SceneBody::plain("twopeg_apart_p", [0.62, 0.66, 0.72], p_aside),
            SceneBody::plain("twopeg_apart_q", [0.78, 0.60, 0.42], q_aside),
        ],
    }]
}

/// **What the flush detector reaches on this mate, measured** — the
/// evidence behind [`declarations`]' note, kept as a test so the
/// sentence is re-derived rather than believed.
///
/// Two claims, and the second is the one the scene's declaration
/// assembly rests on:
///
/// 1. The detector produces the mating plane, and it also produces
///    contacts this part does not mate on — so the scene picks its
///    findings out of the report rather than declaring the report.
/// 2. The cylindrical peg/bore pairs are IN the report: three faces a
///    side per fit, eighteen findings, and not one cross-peg pair —
///    the carrier ladder decides sameness, so the scene's old
///    hand-match on stored axis origins is not doing that work any
///    more. This is the measurement that replaced the one recorded
///    here while the detector was planar (then: the pairs verify
///    under a declaration and report as would-verify-if-declared,
///    but the detector's own door did not carry them).
#[cfg(test)]
mod flush_detector_measurements {
    use super::*;
    use pncad::geom_core::Tol;

    #[test]
    fn the_detector_finds_the_mating_plane_among_other_real_contacts() {
        let tol = Tol::witness();
        let p = plate_with_pegs::<f64>(tol);
        let q = plate_with_holes::<f64>(tol);
        let found = pncad::topo::flush::find_flush_candidates(&p, &q, tol)
            .expect("the plates' pairs decide definitely");
        let mating = (
            plane_face(&p, PLATE.2, true),
            plane_face(&q, PLATE.2, false),
        );
        assert!(
            found.iter().any(|f| f.pair == mating),
            "the mate's own plane must be a finding: {found:?}"
        );
        let planar: Vec<_> = found
            .iter()
            .filter(|f| !cylinders(&p).contains(&f.pair.0))
            .collect();
        assert!(
            planar.len() > 1,
            "the parts share more planar contacts than they mate on (flush side walls, \
             peg tops flush with Q's top face), which is why the scene picks: {planar:?}"
        );
        assert_eq!(
            declarations(&p, &q, tol)
                .coincident_faces
                .iter()
                .filter(|d| d.a == mating.0 && d.b == mating.1)
                .count(),
            1,
            "and picks exactly the one it means"
        );
    }

    #[test]
    fn the_cylindrical_fits_are_findings_and_no_cross_peg_pair_is() {
        let tol = Tol::witness();
        let p = plate_with_pegs::<f64>(tol);
        let q = plate_with_holes::<f64>(tol);
        let (cp, cq) = (cylinders(&p), cylinders(&q));
        assert_eq!((cp.len(), cq.len()), (6, 6), "three faces a side, two fits");
        let found = pncad::topo::flush::find_flush_candidates(&p, &q, tol)
            .expect("the plates' pairs decide definitely");
        let curved: Vec<_> = found
            .iter()
            .filter(|f| cp.contains(&f.pair.0))
            .cloned()
            .collect();
        assert_eq!(
            curved.len(),
            18,
            "each peg against its own bore is 3 x 3, and the two fits are all of it — \
             the cross-peg pairs sit on DISTINCT carriers and the ladder says so, \
             which is why nothing here matches carriers by hand: {curved:?}"
        );
        for f in &curved {
            assert!(cq.contains(&f.pair.1));
            assert_eq!(f.class, pncad::topo::ContactClass::Rest);
            assert_eq!(
                f.evidence.relation,
                pncad::topo::CarrierRelation::SameOpposite,
                "a peg's convex wall against its bore's concave wall is Rest: {f:?}"
            );
        }
        // And the scene declares exactly these, plus the mating plane.
        assert_eq!(
            declarations(&p, &q, tol).coincident_faces.len(),
            19,
            "one planar Rest and eighteen cylindrical ones"
        );
    }
}
