//! **The two-peg plate** — the join lane's canonical cell (M9-3 PR-B
//! acceptance fixture (i), here as the scene it was always meant to
//! be): two plates that locate on each other three ways at once, and
//! then become ONE body.
//!
//! Plate P is a 6×4×1 plate with two radius-0.5 pegs standing proud of
//! it; plate Q is the same plate with two through-bores on the same
//! centres. Set Q down on P and the two parts touch on THREE declared
//! contacts: the mating plane, and each peg's wall against its own
//! bore's wall. One is planar; two are CYLINDRICAL, and until M9-3
//! that second kind did not exist — a glued peg-in-hole was the join
//! this tour said out loud it could not build (the "considered and
//! NOT built" note in `demos/README.md`).
//!
//! What the cell shows, at demo altitude:
//!
//! - **Every part here is itself a boolean result.** P is the plate
//!   unioned with two pegs (transverse curved unions — `bossplate`'s
//!   lane); Q is the plate with two bores subtracted. So the mate is a
//!   boolean of booleans, like the cross-lap.
//! - **The mate is DECLARED, never inferred.** The author knows Q is
//!   located on P — the kernel is told, in the author's own words,
//!   which face pairs are in contact and that each contact is a
//!   `Rest`. Value equality never glues (the coincidence ladder is
//!   law); a declaration is what unlocks the arm, and verification
//!   still happens inside the op.
//! - **The union is exactly additive.** vol(P) + vol(Q) =
//!   (24 + π/2) + (24 − π/2) = **48**, and the glued body measures 48
//!   BITWISE: the interiors are disjoint, so nothing is discarded, and
//!   the pegs' π-terms cancel the bores' exactly. That is the C7-lane
//!   statement — a closed-form oracle, not a tolerance.
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
use pncad::profile::{Profile, SketchPlane, circle_split};
use pncad::sweep::{Extrusion, extrude};
use pncad::topo::{Body, BooleanBody, BooleanDeclarations, ContactClass, FacePairDeclaration};

use crate::booleans::{check, expect_seamed, try_subtract, try_union};
use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};

/// Plate footprint and thickness, peg radius, and the two peg centres
/// — the part's whole dimension set, named once so the closed-form
/// oracle below reads as arithmetic on them rather than as a magic
/// number.
const PLATE: (f64, f64, f64) = (6.0, 4.0, 1.0);
const PEG_R: f64 = 0.5;
const PEG_X: [f64; 2] = [2.0, 4.0];
const PEG_Y: f64 = 2.0;
/// How far each peg stands proud of its plate — and, equally, how deep
/// its bore is. FULL ENGAGEMENT: the peg fills the bore exactly, which
/// is what makes every cylindrical contact patch interior.
const ENGAGE: f64 = 1.0;

/// vol(P) = plate + the two pegs' proud stubs; vol(Q) = plate − the
/// two bores. The π-terms are equal and opposite, so the mated pair is
/// exactly two plates' worth of material.
const V_P: f64 = PLATE.0 * PLATE.1 * PLATE.2 + 2.0 * core::f64::consts::PI * PEG_R * PEG_R * ENGAGE / 2.0;
const V_Q: f64 = PLATE.0 * PLATE.1 * PLATE.2 - 2.0 * core::f64::consts::PI * PEG_R * PEG_R * ENGAGE / 2.0;
const V_MATED: f64 = 2.0 * PLATE.0 * PLATE.1 * PLATE.2;

/// A plate: the 6×4 footprint, thickness 1, sketched at `z0`.
fn plate<S: Scalar>(z0: f64, tol: Tol) -> Body<S> {
    let lp = crate::paths::path_polygon(
        &[
            (0.0, 0.0),
            (PLATE.0, 0.0),
            (PLATE.0, PLATE.1),
            (0.0, PLATE.1),
        ],
        tol,
    );
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(
        S::from_f64(0.0),
        S::from_f64(0.0),
        S::from_f64(z0),
    )));
    let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(S::from_f64(PLATE.2)), tol)
        .unwrap()
        .body
}

/// A peg (or, run through `subtract`, a bore cutter): the radius-0.5
/// circle about `(cx, PEG_Y)`, authored as THREE 120° arcs of one
/// carrier — `circle_split`, as `bossplate` does, because the split
/// count is part of what the seam looks like and must be said out
/// loud.
fn peg<S: Scalar>(cx: f64, z0: f64, h: f64, tol: Tol) -> Body<S> {
    let rim = circle_split(
        Point2::new(S::from_f64(cx), S::from_f64(PEG_Y)),
        S::from_f64(PEG_R),
        3,
        S::from_f64(0.0),
        tol,
    )
    .expect("the three-arc peg rim authors");
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(
        S::from_f64(0.0),
        S::from_f64(0.0),
        S::from_f64(z0),
    )));
    let profile = Profile::new(plane, vec![rim.into()], ).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(S::from_f64(h)), tol)
        .unwrap()
        .body
}

/// Plate P: the plate, with a peg unioned on at each centre. Each peg
/// is sketched INSIDE the plate and extruded through its top, so both
/// unions are TRANSVERSE curved booleans — the same op `bossplate`
/// shows on its own.
fn plate_with_pegs<S: Scalar>(tol: Tol) -> Body<S> {
    let plain = PLATE.0 * PLATE.1 * PLATE.2;
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

/// Plate Q: the same plate a storey up, with a through-bore subtracted
/// on each centre. The cutters overshoot both faces, so no cutter
/// plane coincides with a plate plane.
fn plate_with_bores<S: Scalar>(tol: Tol) -> Body<S> {
    let mut body = plate::<S>(PLATE.2, tol);
    for (i, cx) in PEG_X.into_iter().enumerate() {
        let cutter = peg::<S>(cx, PLATE.2 - 0.2, PLATE.2 + 0.4, tol);
        let vol = PLATE.0 * PLATE.1 * PLATE.2
            - (i as f64 + 1.0) * core::f64::consts::PI * PEG_R * PEG_R * PLATE.2;
        body = expect_seamed(
            "bore subtract",
            check(try_subtract(&body, &cutter, tol), vol, tol),
            vol,
        )
        .body;
    }
    body
}

/// The cylindrical faces of `body` whose carrier axis is the peg
/// centre at `cx`.
///
/// **A library finding, recorded at the site it was met** (the demos'
/// purpose rule): the author knows perfectly well WHICH contact he
/// means — "this peg against its own bore" — but there is no selector
/// on the plain body API to say it with. The intent has to be
/// re-derived from stored surface parameters by walking every face in
/// the arena, and it comes back as THREE faces per side (the
/// three-arc split), so ONE contact the author has in mind is spelled
/// as NINE `FacePairDeclaration`s. The document layer has selection
/// (`GeoSelect`); the kernel-level `Body` does not, and a declared
/// contact is a kernel-level object. `crate::booleans::flush_declarations`
/// is the same gap answered for PLANES only, and deliberately not
/// widened here: it is called by scenes whose contacts must keep
/// refusing (the lily's stem glue), so a curved arm on it would move
/// a wall rather than build a part.
fn peg_walls<S: Scalar>(body: &Body<S>, cx: f64) -> Vec<pncad::topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(pncad::geom::Surface::Cylinder { origin, .. })
                    if (origin.x.f() - cx).abs() < PEG_R
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// The one planar face of `body` at height `z` whose outward normal
/// points up (`up`) or down. Same finding as [`peg_walls`].
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

/// The mate, in the author's own words: THREE contacts, each a
/// `Rest` (the two parts are cosurface there and their material lies
/// on opposite sides).
///
/// Cross-peg pairs are deliberately NOT declared: peg 1 and bore 2 sit
/// on DISTINCT carriers, and declaring them would be a false statement
/// the kernel would contradict — correctly.
fn declarations<S: Scalar>(p: &Body<S>, q: &Body<S>) -> BooleanDeclarations {
    let mut decls = BooleanDeclarations::none();
    // 1. The mating plane: P's top face against Q's bottom face.
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(p, PLATE.2, true),
        plane_face(q, PLATE.2, false),
        ContactClass::Rest,
    ));
    // 2, 3. Each peg's wall against its own bore's wall.
    for cx in PEG_X {
        for &fa in &peg_walls(p, cx) {
            for &fb in &peg_walls(q, cx) {
                decls.coincident_faces.push(FacePairDeclaration::new(
                    fa,
                    fb,
                    ContactClass::Rest,
                ));
            }
        }
    }
    decls
}

/// The cell's boolean work, generic (the K-probe sweep runs the same
/// ops): both parts, the UNDECLARED refusal, the DECLARED mate, and
/// the lifted copy for the apart framing. Returns the undeclared
/// refusal's narration for the f64 captions.
pub(crate) fn build<S: Scalar>(
    tol: Tol,
) -> (Body<S>, Body<S>, BooleanBody<S>, Body<S>, String) {
    let p = plate_with_pegs::<S>(tol);
    let q = plate_with_bores::<S>(tol);

    // UNDECLARED, the mate refuses: value equality never glues. The
    // contrast is the point — a declaration is an author's statement,
    // not a measurement the kernel is allowed to make for him.
    let naive = check(try_union(&p, &q, tol), V_MATED, tol);
    let refusal = crate::booleans::describe(&naive, V_MATED);
    if !matches!(naive, crate::booleans::Verdict::Refused(_)) {
        panic!(
            "the UNDECLARED two-peg mate no longer refuses ({refusal}) — \
             value equality must never glue (coincidence ladder rung (b)); regression"
        );
    }
    println!("   two-peg mate WITHOUT declarations: {refusal}");

    let mated = expect_seamed(
        "declared two-peg mate (M9-3: one planar Rest + two CYLINDRICAL Rests)",
        check(
            pncad::topo::union_with(&p, &q, &declarations(&p, &q), tol),
            V_MATED,
            tol,
        ),
        V_MATED,
    );
    let v = pncad::topo::mass_properties(&mated.body, tol)
        .expect("mass properties")
        .volume
        .f();
    assert_eq!(
        v, V_MATED,
        "the two-peg mate is EXACTLY additive: vol(P) + vol(Q) = \
         ({V_P} ) + ({V_Q}) = {V_MATED}, bitwise"
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
        "three declared contacts — ONE planar Rest (the mating plane) and TWO \
         CYLINDRICAL Rests (each peg against its own bore); undeclared the mate \
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
            ops: "2 x (extrude plate; extrude three-arc peg -> transverse union / subtract); \
                  declare 1 planar + 2 cylindrical Rests -> union_with",
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
