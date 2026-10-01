//! **The two-peg plate** — the join lane's canonical cell (M9-3 PR-B
//! acceptance fixture (i), here as the scene it was always meant to
//! be): two plates that locate on each other three ways at once, and
//! then become ONE body.
//!
//! Plate P is a 6×4×1 plate with two radius-0.5 pegs standing proud of
//! it; plate Q is the same plate with two through-bores on the same
//! centres. Both share one outline, its four corners filleted (the
//! montage-v3 ask). Set Q down on P and the two parts touch on THREE
//! declared contacts: the mating plane, and each peg's wall against
//! its own bore's wall. One is planar; two are CYLINDRICAL. Where the
//! two parts' surfaces carry on across the seam instead — the outer
//! walls and their corner fillets, each peg's end flush with Q's top —
//! the pair is a CONTINUATION, declared as well, and the union merges
//! it.
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
//!   which face pairs are in contact (each a `Rest`) and which carry
//!   on across the seam (each a continuation). Value equality never
//!   glues; a declaration is what unlocks the arm, and verification
//!   still happens inside the op. Undeclared, the mate refuses before
//!   its crossing layer runs, naming the first undeclared continuation
//!   it meets, and the live narration prints that refusal.
//! - **The union is additive, to 4 ULP.** vol(P) + vol(Q) = vol(mated):
//!   the interiors are disjoint, so the glue discards nothing, and the
//!   pegs' π-terms cancel the bores'. The claim is asked of
//!   THREE kernel answers rather than of one answer against a
//!   hand-written constant — which is both its actual content and the
//!   form that survives the corner fillets: a plate is `24 − (4 − π)r²`,
//!   so the mated total is not a dyadic 48.
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

use pncad::authoring::{p2, v3, validated};
use pncad::geom_brep::SurfaceKind;
use pncad::geom_core::{Affine3, Tol, Vec3};
use pncad::prelude::{SurfaceKindSet, query};
use pncad::profile::{ConstructedLoop, Open, SketchPlane, Start, ValidatedProfile, circle_split};
use pncad::sweep::{Extrusion, extrude};
use pncad::topo::{Body, BooleanBody, BooleanCoincidence, BooleanDeclarations};

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

/// The footprint's corner-fillet radius — the montage-v3 ask, the same
/// on both plates so their outer walls carry on across the seam.
const CORNER_R: f64 = 0.5;
/// The footprint's area: the 6×4 rectangle less what the four corner
/// fillets round off, `(4 − π)r²`.
const PLATE_AREA: f64 = PLATE.0 * PLATE.1 - (4.0 - core::f64::consts::PI) * CORNER_R * CORNER_R;
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

/// The plate outline: the 6×4 footprint, its four corners filleted at
/// [`CORNER_R`] through the PATHS lattice's line×line fillet door —
/// each side an on-path anchor and the direction it runs, each corner
/// a radius, so no tangent point is written down.
///
/// Both plates extrude this one outline, top and bottom edges left
/// sharp, so the mating faces are the SAME rounded rectangle and every
/// outer wall of P — the four flat sides and the four fillets — carries
/// on into Q's above it: one carrier, the same outward sense, meeting
/// only along the mating plane. That is a continuation, and
/// [`declarations`] says so.
fn outline<S: Scalar>(tol: Tol) -> ConstructedLoop<S> {
    let r = S::from_f64(CORNER_R);
    let (w, h) = (PLATE.0, PLATE.1);
    Open.at(p2::<S>(w / 2.0, 0.0))
        .toward(S::from_f64(1.0), S::from_f64(0.0), tol)
        .expect("the south side runs east")
        .fillet(r, tol)
        .expect("a definitely positive corner radius")
        .at(p2(w, h / 2.0), tol)
        .expect("the south-east fillet fits")
        .toward(S::from_f64(0.0), S::from_f64(1.0), tol)
        .expect("the east side runs north")
        .fillet(r, tol)
        .expect("a definitely positive corner radius")
        .at(p2(w / 2.0, h), tol)
        .expect("the north-east fillet fits")
        .toward(S::from_f64(-1.0), S::from_f64(0.0), tol)
        .expect("the north side runs west")
        .fillet(r, tol)
        .expect("a definitely positive corner radius")
        .at(p2(0.0, h / 2.0), tol)
        .expect("the north-west fillet fits")
        .toward(S::from_f64(0.0), S::from_f64(-1.0), tol)
        .expect("the west side runs south")
        .fillet(r, tol)
        .expect("a definitely positive corner radius")
        .to(Start, tol)
        .expect("the south-west fillet closes onto the south side")
        .into()
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
fn rim<S: Scalar>(cx: f64, tol: Tol) -> ConstructedLoop<S> {
    circle_split(p2(cx, PEG_Y), S::from_f64(PEG_R), 3, S::from_f64(0.0), tol)
        .expect("the three-arc peg rim authors")
        .into()
}

/// The plate's sketch at `z0` — the outline, plus a bore rim per peg
/// centre when `bores` is set.
fn plate_profile<S: Scalar>(z0: f64, bores: bool, tol: Tol) -> ValidatedProfile<S> {
    let plane = SketchPlane::new(Affine3::translation(v3(0.0, 0.0, z0)));
    let mut loops = vec![outline::<S>(tol)];
    if bores {
        loops.extend(PEG_X.into_iter().map(|cx| rim::<S>(cx, tol)));
    }
    validated(plane, loops, tol).expect("the plate profile validates")
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
    let plane = SketchPlane::new(Affine3::translation(v3(0.0, 0.0, z0)));
    let profile =
        validated(plane, vec![rim::<S>(cx, tol)], tol).expect("the peg profile validates");
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
/// purpose rule), and it is now the SELECTION half alone: there is no
/// selector on the plain body API for "each peg against its own
/// bore", so the intent is re-derived by walking the arena and
/// filtering, and one contact is still spelled as NINE
/// `FacePairDeclaration`s because no door groups faces by contact.
/// The MATCHING half retired with the detector's curved rungs — this
/// scene used to pair peg walls to bore walls itself against a
/// hand-picked `1e-12`, and the kernel's carrier ladder decides that
/// now. The document layer has selection (`GeoSelect`); the
/// kernel-level `Body` does not — the two-doors gap, #1345.
fn cylinders<S: Scalar>(body: &Body<S>) -> Vec<pncad::topo::FaceKey> {
    query::all_faces(body)
        .into_iter()
        .filter(|&k| {
            query::face_surface_matches(body, k, SurfaceKindSet::just(SurfaceKind::Cylinder))
        })
        .collect()
}

/// Every cylindrical face of `body` whose axis stands on a peg centre:
/// the pegs' walls and the bores', as opposed to the corner fillets.
fn peg_walls<S: Scalar>(body: &Body<S>) -> Vec<pncad::topo::FaceKey> {
    cylinders(body)
        .into_iter()
        .filter(|&k| {
            let surface = body.get_face(k).and_then(|f| body.get_surface(f.surface));
            matches!(
                surface,
                Some(pncad::geom::Surface::Cylinder { origin, .. })
                    if PEG_X.iter().any(|&x| (origin.x.f() - x).abs() < 1e-9)
                        && (origin.y.f() - PEG_Y).abs() < 1e-9
            )
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

/// The mate, in the author's own words: every pair the flush detector
/// finds between the two parts, which on this stack is exactly what the
/// author means — two kinds of statement, and the report already says
/// which is which (`FlushFinding::class`, read off the sense bit).
///
/// - **The CONTACTS the mate is made of** (`Rest`, opposed senses): the
///   mating plane, and each peg against its own bore, three faces a
///   side per fit — no cross-peg pair, because peg 1 and bore 2 are
///   `Distinct` at the door that verifies the declaration rather than
///   at a tolerance this file picks.
/// - **The CONTINUATIONS a stack on one profile has** (aligned senses):
///   P's outer walls and corner fillets carrying on into Q's, and each
///   peg's end flush with Q's top face. These are not contacts, and
///   they are not optional: an undeclared continuation refuses the
///   union, since the op has no licence to make the two faces one, and
///   a declared one merges.
///
/// Nothing is picked out of the report, and the pin rather than a
/// comment keeps that honest: `flush_detector_measurements` asserts the
/// split, so a pair the author did not mean appearing on this part reds
/// the suite instead of being declared unnoticed. Only the mating plane
/// is checked by name ([`plane_face`]), because it is the one contact
/// the scene's story turns on.
fn declarations<S: Scalar>(p: &Body<S>, q: &Body<S>, tol: Tol) -> BooleanDeclarations {
    let found = pncad::topo::flush::find_flush_candidates(p, q, tol)
        .expect("the plates' pairs are authored exactly, so they decide definitely");
    let mating = (plane_face(p, PLATE.2, true), plane_face(q, PLATE.2, false));
    assert!(
        found
            .iter()
            .any(|f| f.pair == mating && f.class == BooleanCoincidence::REST),
        "the mating plane must be a finding: P's top face rests on Q's bottom face"
    );
    pncad::topo::flush::declare_all(&found)
}

/// The cell's boolean work, generic (the K-probe sweep runs the same
/// ops): both parts, the UNDECLARED refusal, the DECLARED mate, and
/// the lifted copy for the apart framing. Returns the undeclared
/// refusal's narration for the f64 captions.
pub(crate) fn build<S: Scalar>(tol: Tol) -> (Body<S>, Body<S>, BooleanBody<S>, Body<S>, String) {
    let p = plate_with_pegs::<S>(tol);
    let q = plate_with_holes::<S>(tol);

    // UNDECLARED, the mate refuses before its crossing layer runs: the
    // two parts' walls carry on across the mating plane, and a
    // continuation nobody declared is a pair the op has no licence to
    // make one, so it is named and refused (`UndeclaredCoincidence`,
    // the same refusal an undeclared resting pair gets), with the
    // recourse to declare it.
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
    let continuations = decls
        .coincident_faces
        .iter()
        .filter(|d| d.class == BooleanCoincidence::Continuation)
        .count();
    println!(
        "   declared: {} face pairs — {} Rest contacts (the mating plane, and each peg \
         against its own bore; cross-peg pairs never arise, since peg 1 and bore 2 sit \
         on distinct carriers) and {continuations} continuations (the walls and their \
         fillets carrying on from P into Q, and the peg ends flush with Q's top)",
        decls.coincident_faces.len(),
        decls.coincident_faces.len() - continuations,
    );
    let mated = expect_seamed(
        "declared two-peg mate (M9-3: the mating plane, each peg fit, and the \
         continuations)",
        check(pncad::topo::union_with(&p, &q, &decls, tol), V_MATED, tol),
        V_MATED,
    );
    // THE ADDITIVITY CLAIM, ASKED OF THREE KERNEL ANSWERS rather than of
    // one answer against a hand-written constant. That is the claim's
    // actual content — the interiors are disjoint, so the glue discards
    // nothing — and asking it of the bodies rather than of 48 is what
    // makes it a statement about the OP instead of about this
    // footprint's arithmetic. It is asserted to 4 ULP of the sum: the
    // fillets' π-terms make no part's volume dyadic, so the sum and the
    // union's own flux total associate the same terms differently.
    let vol = |b: &Body<S>| {
        pncad::topo::mass_properties(b, tol)
            .expect("mass properties")
            .volume
            .f()
    };
    let (vp, vq, v) = (vol(&p), vol(&q), vol(&mated.body));
    let additive = vp + vq;
    let ulps = (v - additive).abs() / (f64::EPSILON * additive.abs());
    assert!(
        ulps <= 4.0,
        "the two-peg mate is additive: vol(mated) must equal vol(P) + vol(Q) = \
         {vp} + {vq} to 4 ULP, and is {v} ({ulps:.2} ULP) — the interiors are \
         disjoint, so the glue discards nothing and the pegs' pi-terms cancel the bores'"
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
         ({}), and each against its own closed form",
        (v - additive).abs(),
        if v == additive {
            "bitwise"
        } else {
            "within 4 ULP"
        }
    );
    // Full engagement: every peg/bore patch is interior, so those walls
    // are REMOVED rather than merged. The cylinders that survive are the
    // corner fillets, P's and Q's still two faces each: the merge glues
    // the declared flat walls and records that it has no curved rung.
    assert!(
        peg_walls(&mated.body).is_empty(),
        "full-engagement patch removal deletes every peg and bore wall"
    );
    println!(
        "   two-peg mate WITH the three contacts and the continuations declared: GLUED \
         — measured volume {v} (vol P + vol Q = {vp} + {vq}), every bore wall interior, \
         {} corner-fillet faces left unmerged as a recorded curved skip",
        cylinders(&mated.body).len()
    );

    // The apart framing: Q lifted by a rigid transform (#84 — every
    // moved edge witness is re-minted, and the moved body revalidates).
    let lift = Affine3::translation(v3(0.0, 0.0, 1.6));
    let q_lifted = pncad::topo::transform_rigid(&q, &lift, tol).expect("lift plate Q");
    (p, q, mated, q_lifted, refusal)
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let (p, _q, mated, q_lifted, refusal) = build::<f64>(tol);
    let note = format!(
        "the mate declared in the author's terms — the mating plane, and each peg \
         against its own bore, which the plain-body door can only spell as ONE planar \
         Rest and EIGHTEEN cylindrical ones, three faces a side per fit (#1345); and \
         where the two parts carry on across the seam — the rounded outline's walls \
         and fillets, the peg ends flush with Q's top — a continuation each. \
         Undeclared the mate refuses ({refusal}); declared, the M9-3 zip GLUES it \
         and the union merges the flat continuations: the closed-form volume {V_MATED} \
         (which the body measures to 1e-9), additive to 4 ULP — in closed form \
         vol(P) + vol(Q) = ({V_P}) + ({V_Q}), the pegs' pi-terms \
         cancelling the bores'. Full engagement removes all four peg and bore \
         patches; the corner fillets stay two faces each, P's and Q's, the merge's \
         recorded curved skip"
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
        story: "two rounded plates located on each other three ways — one planar and \
                two CYLINDRICAL declared Rest contacts — their outer walls declared \
                continuations, and UNIONED into one body through the M9-3 zip; the \
                peg-in-hole join this tour used to say it could not build, now with the \
                fillets that used to stop it. Beside it the same two parts apart, Q \
                lifted clear, so the three contacts are visible before the union makes \
                them interior",
        ops: "PATHS fillet outline; extrude plate + 2 x extrude three-arc peg -> 2 \
              transverse unions (P); extrude one profile whose two inner loops are the \
              bores (Q); find_flush_candidates -> declare_all (Rest contacts and \
              continuations) -> union_with; transform_rigid for the apart framing",
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

/// **The rounded mate builds, and undeclared it names a wall pair.**
/// [`build`] asserts the volumes and the full-engagement removal
/// itself; this row runs it outside the render, so the mate is pinned
/// by the suite, and checks the undeclared refusal names an outer-wall
/// continuation.
#[cfg(test)]
mod rounded_mate {
    use super::*;
    use pncad::geom_core::Tol;

    #[test]
    fn the_rounded_mate_builds_and_undeclared_refuses_on_a_wall_continuation() {
        let tol = Tol::witness();
        let (p, q, mated, _, refusal) = build::<f64>(tol);
        assert!(
            refusal.contains("UndeclaredCoincidence"),
            "the undeclared mate refuses as an undeclared coincidence: {refusal}"
        );
        let err = pncad::topo::union(&p, &q, tol).expect_err("undeclared");
        let pncad::topo::BooleanError::UndeclaredCoincidence {
            pair: [(_, fa), (_, fb)],
            relation: pncad::topo::CarrierRelation::SameOriented,
            ..
        } = err
        else {
            panic!("an undeclared continuation: {err:?}");
        };
        assert!(
            peg_walls(&p).iter().all(|&f| f != fa) && peg_walls(&q).iter().all(|&f| f != fb),
            "the refused pair is an outer wall, not a peg fit: {err:?}"
        );
        assert_eq!(
            cylinders(&mated.body).len(),
            8,
            "the four corner fillets, P's and Q's faces each, survive as the recorded skip"
        );
    }
}

/// **What the flush detector reaches on this mate, measured** — the
/// evidence behind [`declarations`]' note, kept as a test so the
/// sentence is re-derived rather than believed.
///
/// The detector's report on this stack is exactly the author's intent,
/// split by the sense bit: the CONTACTS (the mating plane, and the
/// cylindrical peg/bore pairs, three faces a side per fit, eighteen
/// findings, not one cross-peg pair — the carrier ladder decides
/// sameness) and the CONTINUATIONS (the outline's four walls and four
/// fillets, and the two peg ends flush with Q's top).
#[cfg(test)]
mod flush_detector_measurements {
    use super::*;
    use pncad::geom_core::Tol;

    #[test]
    fn the_report_is_the_mates_contacts_and_the_stacks_continuations() {
        let tol = Tol::witness();
        let p = plate_with_pegs::<f64>(tol);
        let q = plate_with_holes::<f64>(tol);
        let found = pncad::topo::flush::find_flush_candidates(&p, &q, tol)
            .expect("the plates' pairs decide definitely");
        let mating = (
            plane_face(&p, PLATE.2, true),
            plane_face(&q, PLATE.2, false),
        );
        let rest: Vec<_> = found
            .iter()
            .filter(|f| f.class == BooleanCoincidence::REST)
            .collect();
        let continuations: Vec<_> = found
            .iter()
            .filter(|f| f.class == BooleanCoincidence::Continuation)
            .collect();
        assert_eq!(
            rest.len() + continuations.len(),
            found.len(),
            "every finding is a contact or a continuation: {found:?}"
        );
        assert!(
            rest.iter().any(|f| f.pair == mating),
            "the mate's own plane is a Rest finding: {found:?}"
        );
        assert_eq!(
            rest.len(),
            19,
            "the mating plane and the eighteen peg-against-bore pairs: {rest:?}"
        );
        for f in &continuations {
            assert_eq!(
                f.evidence.relation,
                pncad::topo::CarrierRelation::SameOriented,
                "a continuation is an aligned pair: {f:?}"
            );
        }
        assert_eq!(
            continuations.len(),
            10,
            "four walls and four fillets carrying on from P into Q, and two peg ends \
             flush with Q's top: {continuations:?}"
        );
        assert_eq!(
            declarations(&p, &q, tol).coincident_faces.len(),
            found.len(),
            "and the scene declares the report"
        );
    }

    #[test]
    fn the_cylindrical_fits_are_findings_and_no_cross_peg_pair_is() {
        let tol = Tol::witness();
        let p = plate_with_pegs::<f64>(tol);
        let q = plate_with_holes::<f64>(tol);
        let (cp, cq) = (peg_walls(&p), peg_walls(&q));
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
        for &fa in &cp {
            assert_eq!(
                curved.iter().filter(|f| f.pair.0 == fa).count(),
                3,
                "each peg-wall face meets THREE bore faces — its own bore's three arcs \
                 and no others; a cross-peg pair would put a fourth here"
            );
        }
        for f in &curved {
            assert!(cq.contains(&f.pair.1));
            assert_eq!(f.class, BooleanCoincidence::REST);
            assert_eq!(
                f.evidence.relation,
                pncad::topo::CarrierRelation::SameOpposite,
                "a peg's convex wall against its bore's concave wall is Rest: {f:?}"
            );
        }
    }
}
