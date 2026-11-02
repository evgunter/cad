//! **The two-peg plate** — the join lane's canonical cell (M9-3 PR-B
//! acceptance fixture (i), here as the scene it was always meant to
//! be): two plates that locate on each other three ways at once, and
//! then become ONE body.
//!
//! Plate P is a 6×4×1 plate with two radius-0.5 pegs standing proud of
//! it; plate Q is the same plate with two through-bores on the same
//! centres. Both share one outline, its four corners filleted (the
//! montage-v3 ask). Set Q down on P and the two parts touch on THREE
//! contacts: the mating plane, and each peg's wall against its own
//! bore's wall. One is planar; two are CYLINDRICAL. Where the two
//! parts' surfaces carry on across the seam instead — the outer walls
//! and their corner fillets, each peg's end flush with Q's top — the
//! pair is a CONTINUATION, and the union merges it.
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
//!   their 3×3 pairing is a fact about the loop rather than a
//!   coincidence between two spellings.
//! - **The union decides the mate.** Each face pair in contact (a
//!   `Rest`) and each that carries on across the seam (a
//!   continuation) sits on one carrier by margin (D10), and the union
//!   glues it, recording each as a coincidence row.
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
//!   the finished body carries no peg or bore wall at all. What
//!   survives of each peg is its rim circle, as an inner ring on the
//!   plate's top face.
//!
//! The apart framing is the same two parts, Q lifted by a rigid
//! transform, so a reader can see the three contacts before they
//! become interior.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::authoring::{p2, p3, v3, validated};
use pncad::geom_core::{RigidMotion, Tol, Vec3};
use pncad::prelude::SurfaceKind;
use pncad::prelude::{SurfaceKindSet, query};
use pncad::profile::{ConstructedLoop, Open, SketchPlane, Start, ValidatedProfile, circle_split};
use pncad::sweep::ExtrudeSide;
use pncad::sweep::{Extrusion, extrude};
use pncad::topo::{AtRestBody, Body, BooleanBody};

use crate::booleans::{check, expect_seamed, finished, try_union};
use crate::scalar::Scalar;
use crate::{SceneBody, TourStop, View};

/// Plate footprint and thickness, peg radius, and the two peg centres
/// — the part's whole dimension set, named once so the closed-form
/// oracle below reads as arithmetic on them rather than as a magic
/// number.
pub const PLATE: (f64, f64, f64) = (6.0, 4.0, 1.0);
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
/// only along the mating plane. That is a continuation, and the union
/// merges it.
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
/// wall and bore wall are three faces each and their 3×3 pairing is a
/// fact about the loop rather than a coincidence between two spellings.
fn rim<S: Scalar>(cx: f64, tol: Tol) -> ConstructedLoop<S> {
    circle_split(p2(cx, PEG_Y), S::from_f64(PEG_R), 3, S::from_f64(0.0), tol)
        .expect("the three-arc peg rim authors")
        .into()
}

/// The plate's sketch at `z0` — the outline, plus a bore rim per peg
/// centre when `bores` is set.
fn plate_profile<S: Scalar>(z0: f64, bores: bool, tol: Tol) -> ValidatedProfile<S> {
    let plane = SketchPlane::xy_at(p3(0.0, 0.0, z0));
    let mut loops = vec![outline::<S>(tol)];
    if bores {
        loops.extend(PEG_X.into_iter().map(|cx| rim::<S>(cx, tol)));
    }
    validated(plane, loops, tol).expect("the plate profile validates")
}

/// A plate: the 6×4 footprint, thickness 1, sketched at `z0`.
fn plate<S: Scalar>(z0: f64, tol: Tol) -> AtRestBody<S> {
    let body = extrude(
        &plate_profile::<S>(z0, false, tol),
        Extrusion::Distance {
            depth: S::from_f64(PLATE.2),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .expect("the plate extrudes")
    .body;
    finished("a plate", body, tol)
}

/// A peg: [`rim`] extruded `h` from `z0`.
fn peg<S: Scalar>(cx: f64, z0: f64, h: f64, tol: Tol) -> AtRestBody<S> {
    let plane = SketchPlane::xy_at(p3(0.0, 0.0, z0));
    let profile =
        validated(plane, vec![rim::<S>(cx, tol)], tol).expect("the peg profile validates");
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: S::from_f64(h),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .expect("the peg extrudes")
    .body;
    finished("a peg", body, tol)
}

/// Plate P: the plate, with a peg unioned on at each centre. Each peg
/// is sketched INSIDE the plate and extruded through its top, so both
/// unions are TRANSVERSE curved booleans — the same op `bossplate`
/// shows on its own.
fn plate_with_pegs<S: Scalar>(tol: Tol) -> AtRestBody<S> {
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
fn plate_with_holes<S: Scalar>(tol: Tol) -> AtRestBody<S> {
    let body = extrude(
        &plate_profile::<S>(PLATE.2, true, tol),
        Extrusion::Distance {
            depth: S::from_f64(PLATE.2),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .expect("the holed plate extrudes")
    .body;
    finished("plate Q", body, tol)
}

/// Every cylindrical face of `body` — the kernel's own kind read
/// (`query::face_surface_matches`), for the scene's face counts.
///
/// **A library finding, recorded at the site it was met** (the demos'
/// purpose rule): there is no selector on the plain body API for "each
/// peg against its own bore", so the faces are found by walking the
/// arena and filtering. The document layer has selection
/// (`GeoSelect`); the kernel-level `Body` does not — the two-doors
/// gap, #1345.
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
pub fn peg_walls<S: Scalar>(body: &Body<S>) -> Vec<pncad::topo::FaceKey> {
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

/// The cell's boolean work, generic (the K-probe sweep runs the same
/// ops): both parts, the mate, and the lifted copy for the apart
/// framing.
pub fn build<S: Scalar>(tol: Tol) -> (AtRestBody<S>, AtRestBody<S>, BooleanBody<S>, Body<S>) {
    let p = plate_with_pegs::<S>(tol);
    let q = plate_with_holes::<S>(tol);

    // The contacts and the walls that carry on across the mating plane
    // each sit on one carrier by margin, so the union glues them.
    let mated = expect_seamed(
        "two-peg mate (M9-3: the mating plane, each peg fit, and the continuations)",
        check(try_union(&p, &q, tol), V_MATED, tol),
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
        pncad::topo::mass_properties(&crate::booleans::at_rest(b, tol), tol, crate::printed())
            .expect("mass properties")
            .enclosure
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
    // corner fillets, P's and Q's merged into one face at each corner
    // like the flat walls between them.
    assert!(
        peg_walls(&mated.body).is_empty(),
        "full-engagement patch removal deletes every peg and bore wall"
    );
    assert_eq!(
        cylinders(&mated.body).len(),
        4,
        "each corner fillet continuation merges P's face and Q's into one"
    );
    println!(
        "   two-peg mate, the three contacts and the continuations glued: GLUED \
         — measured volume {v} (vol P + vol Q = {vp} + {vq}), every bore wall interior, \
         the walls and the four corner fillets each merged across the seam"
    );

    // The apart framing: Q lifted by a rigid transform (#84 — every
    // moved edge witness is re-minted, and the moved body revalidates).
    let lift = RigidMotion::translation(v3(0.0, 0.0, 1.6)).expect("a finite lift");
    let q_lifted = pncad::topo::transform_rigid(&q, &lift, tol).expect("lift plate Q");
    (p, q, mated, q_lifted)
}

pub fn stops(tol: Tol) -> Vec<TourStop> {
    let (p, _q, mated, q_lifted) = build::<f64>(tol);
    let note = format!(
        "the mate's contacts — the mating plane, and each peg against its own bore, \
         three faces a side per fit — and where the two parts carry on across the \
         seam — the rounded outline's walls and fillets, the peg ends flush with Q's \
         top — a continuation each, all decided on their zero (D10): the M9-3 zip \
         GLUES the contacts and the union merges the continuations. The closed-form \
         volume {V_MATED} \
         (which the body measures to 1e-9), additive to 4 ULP — in closed form \
         vol(P) + vol(Q) = ({V_P}) + ({V_Q}), the pegs' pi-terms \
         cancelling the bores'. Full engagement removes all four peg and bore \
         patches; the walls and the four corner fillets each merge across the seam \
         into one face"
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
    let aside = RigidMotion::translation(Vec3::new(APART_GAP, 0.0, 0.0)).expect("a finite offset");
    let p_aside = pncad::topo::transform_rigid(&p, &aside, tol).expect("place P aside");
    let q_aside = pncad::topo::transform_rigid(&q_lifted, &aside, tol).expect("place Q aside");
    vec![TourStop {
        name: "twopeg",
        caption: "two-peg plate — mated, and apart".to_string(),
        montage: true,
        story: "two rounded plates located on each other three ways — one planar and \
                two CYLINDRICAL Rest contacts — their outer walls continuations, and \
                UNIONED into one body through the M9-3 zip; the \
                peg-in-hole join this tour used to say it could not build, now with the \
                fillets that used to stop it. Beside it the same two parts apart, Q \
                lifted clear, so the three contacts are visible before the union makes \
                them interior",
        ops: "PATHS fillet outline; extrude plate + 2 x extrude three-arc peg -> 2 \
              transverse unions (P); extrude one profile whose two inner loops are the \
              bores (Q); union (Rest contacts and continuations glued); transform_rigid \
              for the apart framing",
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
                mated.body.into_body(),
                mated.contacts,
            ),
            SceneBody::plain("twopeg_apart_p", [0.62, 0.66, 0.72], p_aside),
            SceneBody::plain("twopeg_apart_q", [0.78, 0.60, 0.42], q_aside),
        ],
    }]
}
