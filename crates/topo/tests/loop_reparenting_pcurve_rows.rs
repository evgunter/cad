//! **The loop-re-parenting doors leave no row stated in a chart its
//! face is not on.**
//!
//! A pcurve row is a curve stated in a FACE's chart, keyed on a
//! half-edge. `kfmrh`, `mfkrh` and `ring_move` move a whole LOOP from
//! one face to another, which changes which chart every row on that
//! loop is about while changing no key. Each row below is one door in
//! one direction, and the claim under all of them is one sentence: a
//! moved loop keeps its rows exactly where the two faces are on one
//! surface key, and keeps none where they are not.
//!
//! **The subject is the class, not the three loop doors alone.** After
//! them come the two doors that move a RUN of half-edges between two
//! faces' loops (`mef`'s chord surgery, `kef`'s unsplice), and under
//! the last heading `Body::set_face_surface`, which re-charts a face in
//! place: no loop moves and no key changes, and every row the face
//! stores is a curve stated in the chart the face LEFT. Each is rowed
//! on this same fixture, and each takes the same answer — carried
//! across one chart, dropped across two. The last three headings row
//! the one tie between two keys the doors read (a shared payload
//! `Arc`), the `sense` a minted or re-charted face takes on the same
//! chart answer (D1), and the stamp door whose assertion keeps a
//! recipe stamp to one description.
//!
//! The fixture is a minted cylinder-wall sheet split at mid-height into
//! two curved faces, with the sheet's other side put on a PLANE: three
//! faces in one shell, two of them minting and one of them not. The
//! plane carries all six of the sheet's vertices (they lie on two
//! meridian lines) but not its two rim ARCS, which bow off it — every
//! reader here is the pcurve map, and the plane's whole role is to be a
//! chart that mints nothing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::pcurves::validate_pcurves;
use topo::{Body, FaceKey, FaceSurface, LoopKey, MefSite, MevSite, PcurveMintError};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

const U0: f64 = 0.2;
const U1: f64 = 1.4;
const V0: f64 = -0.5;
const VM: f64 = 0.0;
const V1: f64 = 0.7;
/// The ruling `mef`'s rows split the lower panel along.
const UM: f64 = 0.8;

fn axis() -> Vec3<f64> {
    Vec3::unit_z()
}

fn u_ref() -> Vec3<f64> {
    Vec3::unit_x()
}

fn cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::origin(),
        axis: axis(),
        radius: 1.0,
        u_ref: u_ref(),
    }
}

/// A coaxial cylinder of another radius — a chart nothing on this
/// sheet is stated in.
fn other_cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::origin(),
        axis: axis(),
        radius: 2.0,
        u_ref: u_ref(),
    }
}

/// One recipe source. Stamped on two keys it declares them one
/// surface — the merge door's question — and ties no row to either.
fn one_recipe() -> topo::GeomSource {
    topo::GeomSource::minted(7, 0)
}

/// The sheet's point at chart coordinates `(u, v)`.
fn at(u: f64, v: f64) -> Point3<f64> {
    let w = axis().cross(u_ref());
    Point3::origin() + (u_ref() * u.cos() + w * u.sin()) * 1.0 + axis() * v
}

/// A rim arc at height `v` from `U1` to `U0` — the reversed circle, so
/// the carrier runs forward over `[0, U1 - U0]`.
fn rim_back(v: f64, cyl: topo::SurfaceKey) -> EdgeCurveSpec<f64> {
    let w = axis().cross(u_ref());
    let start = u_ref() * U1.cos() + w * U1.sin();
    EdgeCurveSpec {
        description: EdgeDescriptionSpec::chart(cyl),
        carrier: Curve3::Circle {
            center: Point3::origin() + axis() * v,
            axis: -axis(),
            radius: 1.0,
            u_ref: start,
        },
        param_start: 0.0,
        param_end: U1 - U0,
    }
}

/// A rim arc at height `v` from `U0` to `U1`.
fn rim_fwd(v: f64, cyl: topo::SurfaceKey) -> EdgeCurveSpec<f64> {
    EdgeCurveSpec {
        description: EdgeDescriptionSpec::chart(cyl),
        carrier: Curve3::Circle {
            center: Point3::origin() + axis() * v,
            axis: axis(),
            radius: 1.0,
            u_ref: u_ref(),
        },
        param_start: U0,
        param_end: U1,
    }
}

/// The three faces of the fixture, in one shell.
struct Sheet {
    body: Body<f64>,
    /// The lower curved panel, `[V0, VM]` — four rows.
    low: FaceKey,
    /// The upper curved panel, `[VM, V1]` — four rows.
    up: FaceKey,
    /// The sheet's other side, on a PLANE — no rows.
    plane: FaceKey,
}

/// A minted cylinder-wall sheet whose front is split at `VM` into two
/// curved faces and whose back is a planar face, all three in one
/// shell.
fn sheet() -> Sheet {
    let (a, b, c, d, e, f) = (
        at(U0, V0),
        at(U1, V0),
        at(U1, VM),
        at(U1, V1),
        at(U0, V1),
        at(U0, VM),
    );
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a, true).unwrap();
    let cyl = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: cylinder(),
                sense: true,
            },
        )
        .unwrap();
    let e_ab = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            rim_fwd(V0, cyl),
            tol(),
        )
        .unwrap();
    let e_bc = body
        .mev_line(
            MevSite::Fan {
                he1: e_ab.he_minus,
                he2: e_ab.he_minus,
            },
            c,
            tol(),
        )
        .unwrap();
    let e_cd = body
        .mev_line(
            MevSite::Fan {
                he1: e_bc.he_minus,
                he2: e_bc.he_minus,
            },
            d,
            tol(),
        )
        .unwrap();
    let e_de = body
        .mev(
            MevSite::Fan {
                he1: e_cd.he_minus,
                he2: e_cd.he_minus,
            },
            e,
            rim_back(V1, cyl),
            tol(),
        )
        .unwrap();
    let e_ef = body
        .mev_line(
            MevSite::Fan {
                he1: e_de.he_minus,
                he2: e_de.he_minus,
            },
            f,
            tol(),
        )
        .unwrap();
    // The back side: every vertex of the sheet lies on the plane
    // through the two meridian lines (module docs).
    let plane = Surface::Plane {
        origin: a,
        normal: (b - a).cross(axis()).normalize(),
        u_ref: (b - a).normalize(),
    };
    let closing = body
        .mef(
            MefSite::Chords {
                he1: e_ef.he_minus,
                he2: e_ab.he_plus,
            },
            EdgeCurveSpec::line_between(f, a),
            FaceSurface::New {
                surface: plane,
                sense: true,
            },
            tol(),
        )
        .unwrap();
    // The front side, split at `VM` by a rim arc from `c` to `f`:
    // `he1` is the half-edge leaving `c` (`c -> d`), `he2` the one
    // leaving `f` (`f -> a`, the closing edge's plus half).
    let up = body
        .mef(
            MefSite::Chords {
                he1: e_cd.he_plus,
                he2: closing.he_plus,
            },
            rim_back(VM, cyl),
            FaceSurface::Shared {
                key: cyl,
                sense: true,
            },
            tol(),
        )
        .unwrap()
        .face;
    topo::mint_pcurves(&mut body, tol()).unwrap();
    Sheet {
        body,
        low: seed.face,
        up,
        plane: closing.face,
    }
}

/// `(rows stored, half-edges with no row)` over every loop of `face`.
fn rows_of(body: &Body<f64>, face: FaceKey) -> (usize, usize) {
    let f = body.get_face(face).unwrap();
    let mut stored = 0;
    let mut rowless = 0;
    for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let topo::LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            if body.pcurve(he).is_some() {
                stored += 1;
            } else {
                rowless += 1;
            }
        }
    }
    (stored, rowless)
}

/// The first half-edge of `face`'s outer loop.
fn first_he(body: &Body<f64>, face: FaceKey) -> topo::HalfEdgeKey {
    let outer = body.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the fixture's faces are bounded by cycles")
    };
    first
}

/// Every stored row of the whole body.
fn rows_total(body: &Body<f64>) -> usize {
    body.faces().map(|(fk, _)| rows_of(body, fk).0).sum()
}

/// The one ring of `face`.
fn ring_of(body: &Body<f64>, face: FaceKey) -> LoopKey {
    let rings = &body.get_face(face).unwrap().rings;
    assert_eq!(rings.len(), 1, "expected exactly one ring");
    rings[0]
}

/// How many findings report a half-edge carrying no row.
fn missing(findings: &[PcurveMintError]) -> usize {
    findings
        .iter()
        .filter(|f| matches!(f, PcurveMintError::MissingCache { .. }))
        .count()
}

/// The two curved panels carry a complete row set and the pcurve pass
/// accepts the body. That is the whole of what this fixture is for and
/// the whole of what this row asserts — the row below measures what it
/// does NOT claim.
#[test]
fn the_fixture_is_pcurve_complete_and_the_pcurve_pass_accepts_it() {
    let s = sheet();
    assert_eq!(rows_of(&s.body, s.low), (4, 0));
    assert_eq!(rows_of(&s.body, s.up), (4, 0));
    assert_eq!(rows_of(&s.body, s.plane), (0, 6));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// **The fixture is a pcurve fixture, not a valid solid**, and this
/// measures the difference so no row above is read as more than it is.
/// The sheet's back is a plane through the six vertices that does not
/// contain the two rim ARCS bounding it (the module docs say so), and
/// whose normal is set by hand without regard to its loop's traversal,
/// so the structural battery refuses the body — every finding is about
/// that planar face's geometry, and none of them is about a pcurve
/// row. `LoopRoleInverted` is the hand-set normal: the rim arcs' axis
/// lies IN the plane, so the loop's winding about the normal is its
/// chord hexagon's, and that winds clockwise. The pcurve pass and the structural battery ask different
/// questions of the same body and this suite asks only the first.
#[test]
fn the_fixture_is_not_a_structurally_valid_body() {
    let s = sheet();
    let errors = topo::validate::validate_geometric_structural(&s.body, tol())
        .expect_err("a plane that does not contain its own rim arcs is not a valid face");
    let mut kinds: Vec<String> = errors
        .iter()
        .map(|e| {
            format!("{e:?}")
                .split(|c: char| !c.is_alphanumeric())
                .next()
                .unwrap_or("?")
                .to_string()
        })
        .collect();
    kinds.sort();
    kinds.dedup();
    assert_eq!(
        kinds,
        vec![
            "LoopRoleInverted".to_string(),
            "PlanarBoundaryResidual".to_string(),
            "ScaffoldAtRest".to_string(),
            "TransverseNotIntrinsic".to_string(),
        ]
    );
}

/// Onto a chart that mints nothing the rows can neither be re-stated
/// nor measured — the pass skips a planar face — so the door drops
/// them rather than leaving four cylinder-chart curves on a plane for
/// `props`, the tessellator and `chart_boundary` to read.
#[test]
fn kfmrh_onto_a_chart_that_mints_nothing_drops_the_demoted_loops_rows() {
    let mut s = sheet();
    s.body.kfmrh(s.plane, s.low).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (0, 10));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// The other direction is loud and stays loud: the demoted loop brought
/// no row, so nothing is dropped, and the curved face it joined is
/// half-minted until a caller re-mints.
#[test]
fn kfmrh_onto_a_curved_face_leaves_it_incomplete_and_tier_three_says_so() {
    let mut s = sheet();
    s.body.kfmrh(s.low, s.plane).unwrap();
    assert_eq!(rows_of(&s.body, s.low), (4, 6));
    let findings = validate_pcurves(&s.body, band());
    assert_eq!((missing(&findings), findings.len()), (6, 6));
}

/// One surface key on both faces is the case where nothing a row says
/// changed, and every row travels.
#[test]
fn kfmrh_between_faces_on_one_surface_carries_every_row() {
    let mut s = sheet();
    s.body.kfmrh(s.low, s.up).unwrap();
    assert_eq!(rows_of(&s.body, s.low), (8, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// The same two dispositions through the other door: a ring that leaves
/// a cylinder face for the plane leaves its rows behind, and the face
/// it left keeps its own.
#[test]
fn ring_move_onto_a_chart_that_mints_nothing_drops_the_rings_rows() {
    let mut s = sheet();
    s.body.kfmrh(s.low, s.up).unwrap();
    let ring = ring_of(&s.body, s.low);
    s.body.ring_move(ring, s.plane).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (0, 10));
    assert_eq!(rows_of(&s.body, s.low), (4, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// A rowless ring moved onto a minted curved face has nothing to drop
/// and leaves that face incomplete, which tier 3 reports per half-edge.
#[test]
fn ring_move_onto_a_curved_face_leaves_it_incomplete() {
    let mut s = sheet();
    s.body.kfmrh(s.low, s.plane).unwrap();
    let ring = ring_of(&s.body, s.low);
    s.body.ring_move(ring, s.up).unwrap();
    assert_eq!(rows_of(&s.body, s.up), (4, 6));
    assert_eq!(rows_of(&s.body, s.low), (4, 0));
    let findings = validate_pcurves(&s.body, band());
    assert_eq!((missing(&findings), findings.len()), (6, 6));
}

/// A ring moved back onto its own face moves nothing and keeps every
/// row — the door's documented no-op reaches the map too.
#[test]
fn ring_move_to_its_own_face_keeps_every_row() {
    let mut s = sheet();
    s.body.kfmrh(s.up, s.low).unwrap();
    let ring = ring_of(&s.body, s.up);
    s.body.ring_move(ring, s.up).unwrap();
    assert_eq!(rows_of(&s.body, s.up), (8, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// A ring moved between two faces of ONE surface carries every row:
/// nothing a row says changed, so nothing is dropped. The second
/// cylinder face is promoted out of the sheet's planar side onto the
/// cylinder's own key, which is how this fixture holds two live curved
/// faces beside a ring. The face that receives the ring is left
/// incomplete, but by its OWN promoted loop, which the minting pass
/// never ran on — the carried rows are all four of the rows tier 3
/// finds present.
#[test]
fn ring_move_between_faces_on_one_surface_carries_every_row() {
    let mut s = sheet();
    s.body.kfmrh(s.up, s.low).unwrap();
    s.body.kfmrh(s.up, s.plane).unwrap();
    let rings = s.body.get_face(s.up).unwrap().rings.clone();
    assert_eq!(rings.len(), 2, "the rows ring, then the rowless one");
    let sibling = s.body.mfkrh(rings[1], FaceSurface::Inherit).unwrap().face;
    assert_eq!(rows_of(&s.body, s.up), (8, 0));
    assert_eq!(rows_of(&s.body, sibling), (0, 6));
    s.body.ring_move(rings[0], sibling).unwrap();
    assert_eq!(rows_of(&s.body, s.up), (4, 0));
    assert_eq!(rows_of(&s.body, sibling), (4, 6));
    let findings = validate_pcurves(&s.body, band());
    assert_eq!((missing(&findings), findings.len()), (6, 6));
}

/// `mfkrh` is the third loop re-parenting, and its default sugar always
/// changes chart: the placeholder surface is a fresh key, so the
/// promoted ring arrives rowless rather than carrying four cylinder
/// rows onto a face that is not a described surface at all.
#[test]
fn mfkrh_plug_drops_the_promoted_rings_rows() {
    let mut s = sheet();
    s.body.kfmrh(s.low, s.up).unwrap();
    let ring = ring_of(&s.body, s.low);
    let made = s.body.mfkrh_plug(ring, true).unwrap();
    assert_eq!(rows_of(&s.body, made.face), (0, 4));
    assert_eq!(rows_of(&s.body, s.low), (4, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// `FaceSurface::Inherit` is the same chart by construction, so the
/// promoted ring's rows are still its rows.
#[test]
fn mfkrh_inheriting_the_chart_carries_every_row() {
    let mut s = sheet();
    s.body.kfmrh(s.low, s.up).unwrap();
    let ring = ring_of(&s.body, s.low);
    let made = s.body.mfkrh(ring, FaceSurface::Inherit).unwrap();
    assert_eq!(rows_of(&s.body, made.face), (4, 0));
    assert_eq!(rows_of(&s.body, s.low), (4, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// **The loud-to-silent trade, door one of three.** A target face that
/// is CURVED and carries no rows of its own reads, after the drop, as
/// a face the minting pass has not run on — and that pass says nothing
/// about such a face, so the findings go from one `Certify` per moved
/// row to nothing at all. That is not one direction of one door: it is
/// every rowless curved target through every door here, and the two
/// rows below are the other two doors. What the trade buys is that the
/// body no longer HOLDS a row about another surface for `props`, the
/// tessellator and `chart_boundary` to read; the caller's re-mint is
/// what restores the face. That `validate_pcurves` cannot tell a
/// never-minted face from one a door emptied is filed as
/// `work/pcert/validate-pcurves-cannot-tell-a-never-minted-face-from-an-emptied-one`.
#[test]
fn mfkrh_onto_a_rowless_curved_face_drops_the_rows_and_the_pass_goes_quiet() {
    let mut s = sheet();
    s.body.kfmrh(s.low, s.up).unwrap();
    let ring = ring_of(&s.body, s.low);
    let other = Surface::Cylinder {
        origin: Point3::origin(),
        axis: axis(),
        radius: 2.0,
        u_ref: u_ref(),
    };
    let made = s
        .body
        .mfkrh(
            ring,
            FaceSurface::New {
                surface: other,
                sense: true,
            },
        )
        .unwrap();
    assert_eq!(rows_of(&s.body, made.face), (0, 4));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// Every stored row of `face`, with its interval, its image and its
/// certificate — what "byte for byte" means for a carry.
fn rows_deep(body: &Body<f64>, face: FaceKey) -> Vec<String> {
    let f = body.get_face(face).unwrap();
    let mut out = Vec::new();
    for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let topo::LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            if let Some(c) = body.pcurve(he) {
                out.push(format!(
                    "{he:?} {:?} {:?} {:?}",
                    c.params(),
                    c.pcurve(),
                    c.certificate()
                ));
            }
        }
    }
    out
}

/// A carry across one surface key moves nothing: the rows on the far
/// side are the rows that were on the near side, interval, image and
/// certificate alike.
#[test]
fn a_same_surface_move_keeps_every_row_byte_for_byte() {
    let s = sheet();
    let before = rows_deep(&s.body, s.low);
    assert_eq!(before.len(), 4);

    let mut k = sheet();
    k.body.kfmrh(k.up, k.low).unwrap();
    let after = rows_deep(&k.body, k.up);
    assert_eq!(after.len(), 8);
    for row in &before {
        assert!(
            after.contains(row),
            "kfmrh lost or restated a row across one surface key: {row}"
        );
    }

    let mut r = sheet();
    r.body.kfmrh(r.up, r.low).unwrap();
    let ring = ring_of(&r.body, r.up);
    r.body.ring_move(ring, r.up).unwrap();
    assert_eq!(rows_deep(&r.body, r.up), after);
}

/// **A carried row is the row the minting pass derives.** A carry is
/// not "some rows survived": the rows that arrive on the target are
/// the ones the pass would put there, which is what a door that
/// carried a row onto a chart it is not stated in would break.
/// Measured by re-minting the carried body and comparing interval,
/// image and certificate.
#[test]
fn a_carried_row_is_the_row_the_minting_pass_derives() {
    let mut s = sheet();
    s.body.kfmrh(s.up, s.low).unwrap();
    let carried = rows_deep(&s.body, s.up);
    assert_eq!(carried.len(), 8);
    topo::mint_pcurves(&mut s.body, tol()).unwrap();
    assert_eq!(
        rows_deep(&s.body, s.up),
        carried,
        "a carried row is not the row the pass derives for that face"
    );
}

/// And the other side of the trade: what a drop gives up is a re-mint
/// and nothing else. Each move below dropped every row it moved; the
/// pass makes the body whole, on the destination's own chart — none
/// on a planar face, which is that pass's posture, and the full set on
/// a curved one.
#[test]
fn the_minting_pass_restores_what_each_move_left_the_caller() {
    // `kfmrh` onto the plane: there is nothing to restore, and the
    // face the rows left is gone with the op.
    let mut s = sheet();
    s.body.kfmrh(s.plane, s.low).unwrap();
    topo::mint_pcurves(&mut s.body, tol()).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (0, 10));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);

    // `ring_move` onto the plane: the curved face the ring left keeps
    // its own rows across the pass, and the plane still stores none.
    let mut s = sheet();
    s.body.kfmrh(s.low, s.up).unwrap();
    let ring = ring_of(&s.body, s.low);
    s.body.ring_move(ring, s.plane).unwrap();
    topo::mint_pcurves(&mut s.body, tol()).unwrap();
    assert_eq!(rows_of(&s.body, s.low), (4, 0));
    assert_eq!(rows_of(&s.body, s.plane), (0, 10));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);

    // `ring_move` onto a curved face left it incomplete; the pass
    // mints the ring's rows on the chart it is now on.
    let mut s = sheet();
    s.body.kfmrh(s.low, s.plane).unwrap();
    let ring = ring_of(&s.body, s.low);
    s.body.ring_move(ring, s.up).unwrap();
    topo::mint_pcurves(&mut s.body, tol()).unwrap();
    assert_eq!(rows_of(&s.body, s.up), (10, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

// ---------------------------------------------------------------
// Two keys, one surface: which of them is a chart CHANGE.
// ---------------------------------------------------------------

/// **Two keys holding one surface, with or without one recipe stamped
/// on both, read as two charts** (`kfmrh`). The door asks whether the
/// two keys hold one DESCRIPTION, and answers from identity alone — one
/// key, or one shared payload `Arc`; a `GeomSource` stamp declares what
/// the recipe intended and proves nothing about the values. So the
/// moved loop's rows go, and what that costs is a re-mint, never a
/// wrong row.
#[test]
fn two_keys_holding_one_surface_read_as_two_charts_whatever_recipe_they_carry() {
    for stamped in [false, true] {
        let mut s = sheet();
        let cyl = s.body.get_face(s.low).unwrap().surface;
        let second = s
            .body
            .set_face_surface(
                s.up,
                FaceSurface::New {
                    surface: cylinder(),
                    sense: true,
                },
            )
            .unwrap();
        assert_ne!(second, cyl, "`New` mints a fresh key for an equal surface");
        if stamped {
            s.body.set_surface_source(cyl, one_recipe()).unwrap();
            s.body.set_surface_source(second, one_recipe()).unwrap();
        }
        // The setter reads the same two keys the same way and drops
        // `up`'s rows; re-minting builds the body this row is about —
        // one carrying rows on two keys that hold one surface.
        topo::mint_pcurves(&mut s.body, tol()).unwrap();
        assert_eq!(rows_of(&s.body, s.up), (4, 0), "stamped: {stamped}");
        assert_eq!(validate_pcurves(&s.body, band()), vec![]);

        s.body.kfmrh(s.low, s.up).unwrap();
        assert_eq!(rows_of(&s.body, s.low), (4, 4), "stamped: {stamped}");
        let findings = validate_pcurves(&s.body, band());
        assert_eq!(
            (missing(&findings), findings.len()),
            (4, 4),
            "stamped: {stamped}"
        );

        topo::mint_pcurves(&mut s.body, tol()).unwrap();
        assert_eq!(rows_of(&s.body, s.low), (8, 0), "stamped: {stamped}");
        assert_eq!(validate_pcurves(&s.body, band()), vec![]);
    }
}

/// The same through `mfkrh`: a ring promoted onto a second key that
/// carries the old key's recipe loses its rows, and the pass restores
/// them on the key the new face is on.
#[test]
fn mfkrh_onto_a_second_key_sharing_a_recipe_drops_the_rings_rows_until_the_pass() {
    let mut s = sheet();
    let cyl = s.body.get_face(s.low).unwrap().surface;
    let second = s
        .body
        .set_face_surface(
            s.plane,
            FaceSurface::New {
                surface: cylinder(),
                sense: true,
            },
        )
        .unwrap();
    assert_ne!(second, cyl);
    s.body.set_surface_source(cyl, one_recipe()).unwrap();
    s.body.set_surface_source(second, one_recipe()).unwrap();

    s.body.kfmrh(s.low, s.up).unwrap();
    let ring = ring_of(&s.body, s.low);
    let made = s
        .body
        .mfkrh(
            ring,
            FaceSurface::Shared {
                key: second,
                sense: true,
            },
        )
        .unwrap();
    assert_eq!(rows_of(&s.body, made.face), (0, 4));
    assert_eq!(rows_of(&s.body, s.low), (4, 0));

    topo::mint_pcurves(&mut s.body, tol()).unwrap();
    assert_eq!(rows_of(&s.body, made.face), (4, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

// ---------------------------------------------------------------
// The loud-to-silent trade, doors two and three: every rowless
// CURVED target, not one direction of one door.
// ---------------------------------------------------------------

/// **Door two** (`kfmrh`). The sheet's back is put on a chart of its
/// own and left rowless; the curved panel's four rows demote into it
/// and are dropped, and the pass — which says nothing about a face it
/// has not minted — reports nothing. Without the drop those four rows
/// would be measured against the target's chart and refused.
#[test]
fn kfmrh_onto_a_rowless_curved_face_drops_the_rows_and_the_pass_goes_quiet() {
    let mut s = sheet();
    s.body
        .set_face_surface(
            s.plane,
            FaceSurface::New {
                surface: other_cylinder(),
                sense: true,
            },
        )
        .unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (0, 6));

    s.body.kfmrh(s.plane, s.low).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (0, 10));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// **Door three** (`ring_move`), the same class again: a ring of four
/// rows onto a rowless curved face leaves it rowless, and quiet.
#[test]
fn ring_move_onto_a_rowless_curved_face_drops_the_rows_and_the_pass_goes_quiet() {
    let mut s = sheet();
    s.body
        .set_face_surface(
            s.plane,
            FaceSurface::New {
                surface: other_cylinder(),
                sense: true,
            },
        )
        .unwrap();
    s.body.kfmrh(s.low, s.up).unwrap();
    let ring = ring_of(&s.body, s.low);
    s.body.ring_move(ring, s.plane).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (0, 10));
    assert_eq!(rows_of(&s.body, s.low), (4, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

// ---------------------------------------------------------------
// The loop with no cycle.
// ---------------------------------------------------------------

/// A ring whose boundary is not a cycle holds no half-edge, so it
/// holds no row, and moving it between charts moves nothing. This is
/// the arm the discard register files `audited` at
/// `pcurves::loop_rows`, measured rather than argued.
#[test]
fn an_empty_boundary_ring_moves_with_the_map_untouched() {
    let mut s = sheet();
    let he = first_he(&s.body, s.low);
    let v = s.body.get_half_edge(he).unwrap().start;
    let p = *s
        .body
        .get_point(s.body.get_vertex(v).unwrap().point)
        .unwrap();
    let spur = s
        .body
        .mev_line(MevSite::Fan { he1: he, he2: he }, p + axis() * 0.05, tol())
        .unwrap();
    let ring = s.body.kemr(spur.he_plus, spur.he_minus).unwrap().ring;
    assert!(matches!(
        s.body.get_loop(ring).unwrap().boundary,
        topo::LoopBoundary::Empty { .. }
    ));

    let before_low = rows_deep(&s.body, s.low);
    let before_total = rows_total(&s.body);
    s.body.ring_move(ring, s.plane).unwrap();
    assert_eq!(rows_deep(&s.body, s.low), before_low);
    assert_eq!(rows_total(&s.body), before_total);
}

// ---------------------------------------------------------------
// One level down: the doors that move a RUN of half-edges between
// loops of two faces — `mef`'s chord surgery and `kef`'s unsplice.
// ---------------------------------------------------------------
//
// The loop doors above move a whole loop. `mef` moves the run
// `[he1 .. he2)` into the loop of a face it mints, and `kef` moves the
// dying loop's remnant into the mate's loop on the face that survives;
// the rows on that run change chart exactly as a re-parented loop's
// do, and take the same answer: carried where the two faces are on one
// chart, dropped where they are not, with the destination read by
// `Body::same_chart` and nothing derived.

/// A plane nothing on the sheet is stated in.
fn flat() -> Surface<f64> {
    Surface::Plane {
        origin: at(U0, V0),
        normal: axis(),
        u_ref: u_ref(),
    }
}

/// The outer loop of `face` in cycle order.
fn cycle_of(body: &Body<f64>, face: FaceKey) -> Vec<topo::HalfEdgeKey> {
    body.loop_cycle(first_he(body, face)).unwrap()
}

/// The half-edge of `face`'s outer loop that starts at the vertex
/// sitting at `p` — one per vertex per loop, so the point names it.
fn he_at(body: &Body<f64>, face: FaceKey, p: Point3<f64>) -> topo::HalfEdgeKey {
    let hit: Vec<_> = cycle_of(body, face)
        .into_iter()
        .filter(|&he| {
            let v = body.get_half_edge(he).unwrap().start;
            let q = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            (q - p).norm() == 0.0
        })
        .collect();
    assert_eq!(hit.len(), 1, "one half-edge of the loop starts at {p:?}");
    hit[0]
}

/// Splits both rims of the lower panel at the ruling `UM` —
/// `split_edge` carries their rows — and returns the `mef` site that
/// closes that ruling: `he1` leaves the bottom split vertex `m1`
/// (`m1→b`), `he2` the top one `m2` (`m2→f`), and the chord `m1→m2`
/// is the ruling itself, a line ON the cylinder.
fn ruling_site(s: &mut Sheet) -> (topo::HalfEdgeKey, topo::HalfEdgeKey, EdgeCurveSpec<f64>) {
    let (a, c) = (at(U0, V0), at(U1, VM));
    let bottom = s.body.get_half_edge(he_at(&s.body, s.low, a)).unwrap().edge;
    let mid = s.body.get_half_edge(he_at(&s.body, s.low, c)).unwrap().edge;
    // `a→b` runs forward in `u`; the mid rim `c→f` is `rim_back`, whose
    // parameter runs from `U1` down.
    let m1 = s.body.split_edge(bottom, UM, tol()).unwrap();
    let m2 = s.body.split_edge(mid, U1 - UM, tol()).unwrap();
    let point = |v: topo::VertexKey| {
        *s.body
            .get_point(s.body.get_vertex(v).unwrap().point)
            .unwrap()
    };
    let chord = EdgeCurveSpec::line_between(point(m1.vertex), point(m2.vertex));
    (m1.he_plus, m2.he_plus, chord)
}

/// Splits the lower panel along the ruling at `UM` ([`ruling_site`]):
/// the run `[m1→b, b→c, c→m2]` — three rim-and-meridian rows — moves
/// into the new face's loop with the minted `m2→m1`, and the old face
/// keeps `m2→f`, `f→a`, `a→m1` and the minted `m1→m2`.
fn split_low(s: &mut Sheet, surface: FaceSurface<f64>) -> topo::MefCreated {
    let (he1, he2, chord) = ruling_site(s);
    s.body
        .mef(MefSite::Chords { he1, he2 }, chord, surface, tol())
        .unwrap()
}

/// The rows of `face` other than those of `made`'s two new halves —
/// the rows a `mef` found rather than minted.
fn found_rows(body: &Body<f64>, face: FaceKey, made: &topo::MefCreated) -> Vec<String> {
    let minted = [
        format!("{:?} ", made.he_plus),
        format!("{:?} ", made.he_minus),
    ];
    rows_deep(body, face)
        .into_iter()
        .filter(|row| !minted.iter().any(|m| row.starts_with(m.as_str())))
        .collect()
}

/// **`mef` onto a chart that mints nothing drops the moved run's
/// rows.** Before the door disposed of them, the three cylinder rows
/// on the run rode onto the new PLANAR face, and the pass, which skips
/// a planar face, said nothing about them. The planar face holds no
/// row about a chart it is not on; the old face, still on the
/// cylinder, gets the row of the half this door mints into it, so the
/// pass has nothing to say about either.
#[test]
fn mef_onto_a_chart_that_mints_nothing_drops_the_moved_runs_rows() {
    let mut s = sheet();
    let made = split_low(
        &mut s,
        FaceSurface::New {
            surface: flat(),
            sense: true,
        },
    );
    assert_eq!(rows_of(&s.body, made.face), (0, 4));
    assert_eq!(rows_of(&s.body, s.low), (4, 0));
    assert!(s.body.pcurve(made.he_plus).is_some());
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
    // The sheet's other curved panel lost nothing: its mid rim was
    // split beside the lower panel's, and carried its rows.
    assert_eq!(rows_of(&s.body, s.up), (5, 0));
}

/// **The loud-to-silent trade, through this door.** Onto a rowless
/// CURVED chart the moved rows used to leave the new face half-minted —
/// its own minted half rowless beside them — and the pass reported
/// that half; dropped, the new face stores no row at all and reads as
/// one the pass has not minted, about which it says nothing
/// (`work/pcert/validate-pcurves-cannot-tell-a-never-minted-face-from-an-emptied-one`).
/// Its minted half stays rowless with them: an unminted face is the
/// minting pass's. What the trade buys is the same as at the loop
/// doors: the body no longer holds curves stated in a chart the face
/// is not on.
#[test]
fn mef_onto_a_rowless_curved_chart_drops_the_runs_rows_and_the_pass_goes_quiet() {
    let mut s = sheet();
    let made = split_low(
        &mut s,
        FaceSurface::New {
            surface: other_cylinder(),
            sense: true,
        },
    );
    assert_eq!(rows_of(&s.body, made.face), (0, 4));
    assert!(s.body.pcurve(made.he_minus).is_none());
    assert_eq!(rows_of(&s.body, s.low), (4, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// **The controls: the same chart carries the run's rows byte for
/// byte.** `Inherit` and a `Shared` naming the old key are the same
/// chart by construction, so the three rows that move are the three
/// rows that were there — interval, image and certificate — and the
/// three that stay are the other three. **The carry is the
/// `found_rows` equality.** Beside it, each face carries one row more:
/// the minted half's, `he_plus` on the old face and `he_minus` on the
/// new, which the operator mints at the site — so the pass reads both
/// faces complete, where it used to report those two halves missing.
#[test]
fn mef_inheriting_or_sharing_the_chart_carries_the_runs_rows_byte_for_byte() {
    let base = sheet();
    let cyl = base.body.get_face(base.low).unwrap().surface;

    for (label, surface) in [
        ("Inherit", FaceSurface::Inherit),
        (
            "Shared(own key)",
            FaceSurface::Shared {
                key: cyl,
                sense: true,
            },
        ),
    ] {
        let mut s = sheet();
        let (he1, he2, chord) = ruling_site(&mut s);
        let before = rows_deep(&s.body, s.low);
        assert_eq!(before.len(), 6);
        let made = s
            .body
            .mef(MefSite::Chords { he1, he2 }, chord, surface, tol())
            .unwrap();
        let moved = found_rows(&s.body, made.face, &made);
        let kept = found_rows(&s.body, s.low, &made);
        assert_eq!((moved.len(), kept.len()), (3, 3));
        let mut after: Vec<String> = moved.into_iter().chain(kept).collect();
        after.sort();
        let mut expected = before.clone();
        expected.sort();
        assert_eq!(after, expected, "a same-chart mef lost or restated a row");
        assert_eq!(
            validate_pcurves(&s.body, band()),
            vec![],
            "the minted halves' rows, onto {label}"
        );
        assert_eq!(rows_of(&s.body, made.face), (4, 0));
        assert_eq!(rows_of(&s.body, s.low), (4, 0));
    }
}

/// **A second key holding the same cylinder, with or without the old
/// key's recipe stamped on both, is not the same chart to this door**:
/// a `Shared` naming it drops the run's rows, exactly as a rowless
/// curved chart of its own does. The operator mints nothing on the new
/// face either, although the closed-form lane could: a face whose rows
/// did not stand is the minting pass's, and a row minted onto it would
/// claim a chart identity the body does not hold. The old face, still
/// on its own key, is complete with the minted half's row.
#[test]
fn mef_onto_a_second_key_holding_one_surface_drops_the_runs_rows_and_mints_none_there() {
    for stamped in [false, true] {
        let mut s = sheet();
        let cyl = s.body.get_face(s.low).unwrap().surface;
        let second = s
            .body
            .set_face_surface(
                s.plane,
                FaceSurface::New {
                    surface: cylinder(),
                    sense: true,
                },
            )
            .unwrap();
        assert_ne!(second, cyl);
        if stamped {
            s.body.set_surface_source(cyl, one_recipe()).unwrap();
            s.body.set_surface_source(second, one_recipe()).unwrap();
        }

        let made = split_low(
            &mut s,
            FaceSurface::Shared {
                key: second,
                sense: true,
            },
        );
        assert_eq!(rows_of(&s.body, made.face), (0, 4), "stamped: {stamped}");
        assert_eq!(rows_of(&s.body, s.low), (4, 0), "stamped: {stamped}");
        assert_eq!(
            validate_pcurves(&s.body, band()),
            vec![],
            "stamped: {stamped}"
        );
    }
}

/// **A chord off the chart leaves both pieces unminted, and the pass
/// refuses the result.** A straight chord from `a` to `c` cuts through
/// the cylinder rather than lying on it, so neither piece of the lower
/// panel has a closed-form row set that certifies: the chord's image
/// meets its loop on no branch of the chart. The operator does not
/// refuse — it is called mid-surgery on states a later door finishes
/// describing — and it does not return a panel half-minted either: both
/// pieces store nothing. The loud reading is the pass's, run over the
/// result.
#[test]
fn mef_with_a_chord_off_a_minted_chart_leaves_both_pieces_unminted() {
    let mut s = sheet();
    let (a, c) = (at(U0, V0), at(U1, VM));
    let he1 = he_at(&s.body, s.low, a);
    let he2 = he_at(&s.body, s.low, c);
    let made = s
        .body
        .mef(
            MefSite::Chords { he1, he2 },
            EdgeCurveSpec::line_between(a, c),
            FaceSurface::Inherit,
            tol(),
        )
        .unwrap();
    assert_eq!(rows_of(&s.body, s.low), (0, 3));
    assert_eq!(rows_of(&s.body, made.face), (0, 3));
    assert_eq!(rows_of(&s.body, s.up), (4, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
    assert!(
        topo::mint_pcurves(&mut s.body, tol()).is_err(),
        "the pass refuses a panel whose chord leaves its chart"
    );
}

/// Stamps the sheet's cylinder with `one_recipe()` and grafts in a
/// PLANE carrying the same stamp; returns the plane's key. The stamp
/// door's assertion refuses that pair, so it arrives the way the graft
/// brings it — each stamp true in its own body.
fn forge(s: &mut Sheet) -> topo::SurfaceKey {
    let cyl = s.body.get_face(s.low).unwrap().surface;
    s.body.set_surface_source(cyl, one_recipe()).unwrap();
    let mut other = Body::<f64>::new();
    let seed = other.mvfs(Point3::origin(), true).unwrap();
    let flat_key = other
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: flat(),
                sense: true,
            },
        )
        .unwrap();
    other.set_surface_source(flat_key, one_recipe()).unwrap();
    topo::graft_disjoint(&mut s.body, &other, tol()).unwrap();
    let forged = s
        .body
        .surfaces()
        .map(|(k, _)| k)
        .find(|&k| k != cyl && s.body.surface_source(k) == Some(&one_recipe()))
        .expect("the graft carried the plane's stamp");
    assert!(matches!(
        s.body.get_surface(forged),
        Some(Surface::Plane { .. })
    ));
    forged
}

/// **A forged recipe stamp moves no row.** The sheet's cylinder and a
/// PLANE carry one `GeomSource` ([`forge`]), and `mef` with
/// `Shared(plane)` sends the run onto the planar face. Its three cylinder
/// rows do not come with it: a row carried there would be one no reader
/// is ever warned about, since the pass skips a planar face.
#[test]
fn a_recipe_stamp_joining_a_cylinder_to_a_plane_carries_no_row_onto_the_plane() {
    let mut s = sheet();
    let forged = forge(&mut s);
    let made = split_low(
        &mut s,
        FaceSurface::Shared {
            key: forged,
            sense: true,
        },
    );
    assert_eq!(
        rows_of(&s.body, made.face),
        (0, 4),
        "a cylinder row landed on the planar face"
    );
}

/// The same forged pair at the other five doors: each moves the
/// cylinder panel's rows onto the planar key and carries none of them.
#[test]
fn a_recipe_stamp_joining_a_cylinder_to_a_plane_carries_no_row_through_any_door() {
    let mut s = sheet();
    let forged = forge(&mut s);
    s.body
        .set_face_surface(
            s.low,
            FaceSurface::Shared {
                key: forged,
                sense: true,
            },
        )
        .unwrap();
    assert_eq!(rows_of(&s.body, s.low), (0, 4), "set_face_surface");

    let mut s = sheet();
    let forged = forge(&mut s);
    s.body
        .set_face_surface(
            s.plane,
            FaceSurface::Shared {
                key: forged,
                sense: true,
            },
        )
        .unwrap();
    s.body.kfmrh(s.plane, s.low).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (0, 10), "kfmrh");

    let mut s = sheet();
    let forged = forge(&mut s);
    s.body.kfmrh(s.low, s.up).unwrap();
    let ring = ring_of(&s.body, s.low);
    s.body
        .set_face_surface(
            s.plane,
            FaceSurface::Shared {
                key: forged,
                sense: true,
            },
        )
        .unwrap();
    s.body.ring_move(ring, s.plane).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (0, 10), "ring_move");

    let mut s = sheet();
    let forged = forge(&mut s);
    s.body.kfmrh(s.low, s.up).unwrap();
    let ring = ring_of(&s.body, s.low);
    let made = s
        .body
        .mfkrh(
            ring,
            FaceSurface::Shared {
                key: forged,
                sense: true,
            },
        )
        .unwrap();
    assert_eq!(rows_of(&s.body, made.face), (0, 4), "mfkrh");

    let mut s = sheet();
    let forged = forge(&mut s);
    s.body
        .set_face_surface(
            s.plane,
            FaceSurface::Shared {
                key: forged,
                sense: true,
            },
        )
        .unwrap();
    let he = he_at(&s.body, s.low, at(U0, V0));
    s.body.kef(he).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (0, 8), "kef");
}

/// **`kef` into a face on a chart that mints nothing drops the
/// remnant's rows.** The red-first row: killing the lower panel's
/// edge `a→b` from the panel's side sends the dying loop's remnant —
/// three cylinder rows — into the PLANAR face's loop, where the pass
/// never looks. Before the door disposed of them the planar face read
/// `(3, 5)` and the pass was silent; now it is rowless and the pass
/// is silent for the right reason. A re-mint leaves the plane
/// rowless and the sheet's other panel whole.
#[test]
fn kef_into_a_face_on_a_chart_that_mints_nothing_drops_the_remnants_rows() {
    let mut s = sheet();
    let he = he_at(&s.body, s.low, at(U0, V0));
    let killed = s.body.kef(he).unwrap();
    assert_eq!(killed.killed_face, s.low);
    assert_eq!(rows_of(&s.body, s.plane), (0, 8));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);

    topo::mint_pcurves(&mut s.body, tol()).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (0, 8));
    assert_eq!(rows_of(&s.body, s.up), (4, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// The trade through the other run door: into a rowless CURVED face
/// the three rows used to arrive beside five rowless halves and the
/// pass reported those five; dropped, the face reads as never minted
/// and the pass goes quiet.
#[test]
fn kef_into_a_rowless_curved_face_drops_the_remnants_rows_and_the_pass_goes_quiet() {
    let mut s = sheet();
    s.body
        .set_face_surface(
            s.plane,
            FaceSurface::New {
                surface: other_cylinder(),
                sense: true,
            },
        )
        .unwrap();
    let he = he_at(&s.body, s.low, at(U0, V0));
    s.body.kef(he).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (0, 8));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// **The control: two faces on one chart carry the remnant's rows
/// byte for byte.** Killing the rim edge `c→f` between the two curved
/// panels from the lower one's side sends its remnant — `f→a`, `a→b`,
/// `b→c` — into the upper panel's loop, and the rows that arrive are
/// the rows that left; the upper panel is complete and the pass
/// accepts it. (The killed edge's own two rows die with their keys.)
#[test]
fn kef_between_faces_on_one_chart_carries_the_remnants_rows_byte_for_byte() {
    let mut s = sheet();
    let he = he_at(&s.body, s.low, at(U1, VM));
    let remnant_rows: Vec<String> = rows_deep(&s.body, s.low)
        .into_iter()
        .filter(|row| !row.starts_with(&format!("{he:?} ")))
        .collect();
    assert_eq!(remnant_rows.len(), 3);
    let killed = s.body.kef(he).unwrap();
    assert_eq!(killed.killed_face, s.low);
    assert_eq!(rows_of(&s.body, s.up), (6, 0));
    let after = rows_deep(&s.body, s.up);
    for row in &remnant_rows {
        assert!(
            after.contains(row),
            "kef lost or restated a row across one chart: {row}"
        );
    }
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// And into a face on a second key carrying the dying face's recipe:
/// the remnant's rows drop, and the pass mints the whole face.
#[test]
fn kef_into_a_second_key_sharing_a_recipe_drops_the_remnants_rows_until_the_pass() {
    let mut s = sheet();
    let cyl = s.body.get_face(s.low).unwrap().surface;
    let second = s
        .body
        .set_face_surface(
            s.plane,
            FaceSurface::New {
                surface: cylinder(),
                sense: true,
            },
        )
        .unwrap();
    s.body.set_surface_source(cyl, one_recipe()).unwrap();
    s.body.set_surface_source(second, one_recipe()).unwrap();

    let he = he_at(&s.body, s.low, at(U0, V0));
    s.body.kef(he).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (0, 8));

    topo::mint_pcurves(&mut s.body, tol()).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (8, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// The sheet's cylinder charted with a rotated `u_ref`: the same point
/// set, a different chart. A face put on it MINTS, and its rows are
/// not the rows of the sheet's chart — which is what a destination
/// that is both minted and on another chart needs.
fn rotated_cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::origin(),
        axis: axis(),
        radius: 1.0,
        u_ref: Vec3::new((-0.3f64).cos(), (-0.3f64).sin(), 0.0),
    }
}

/// **`kef` drops the remnant's rows and no others.** The surviving
/// face is MINTED on a different chart (the sheet's cylinder,
/// re-charted), so the remnant's three rows go — and the surviving
/// loop's own three, stated in its own chart already, stay byte for
/// byte, with the pass naming exactly the three rowless arrivals. A
/// `kef` that disposed of the destination LOOP's rows (the loop doors'
/// walk) would read `(0, 6)` and be silent; no other row here reaches
/// a minted survivor on another chart.
#[test]
fn kef_into_a_minted_face_on_another_chart_keeps_the_survivors_own_rows() {
    let mut s = sheet();
    s.body
        .set_face_surface(
            s.up,
            FaceSurface::New {
                surface: rotated_cylinder(),
                sense: true,
            },
        )
        .unwrap();
    topo::mint_pcurves(&mut s.body, tol()).unwrap();
    assert_eq!(rows_of(&s.body, s.up), (4, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
    let up_before = rows_deep(&s.body, s.up);

    let he = he_at(&s.body, s.low, at(U1, VM));
    let killed = s.body.kef(he).unwrap();
    assert_eq!(killed.killed_face, s.low);
    assert_eq!(rows_of(&s.body, s.up), (3, 3));
    for row in rows_deep(&s.body, s.up) {
        assert!(
            up_before.contains(&row),
            "kef restated a surviving row: {row}"
        );
    }
    let findings = validate_pcurves(&s.body, band());
    assert_eq!((missing(&findings), findings.len()), (3, 3));
}

// ---------------------------------------------------------------
// The chart moving under the rows: `set_face_surface`.
// ---------------------------------------------------------------
//
// The doors above move a loop, or a run of half-edges, to another
// chart. The surface setter moves the CHART, under every row the face stores at once, and
// the rows are wrong in exactly the same way. Before this suite's
// setter rows it changed nothing: a swap onto another cylinder left
// four rows stated in the chart the face had left, which tier 3
// measured and refused four times over, and a swap onto a PLANE left
// the same four rows where `validate_pcurves` never looks — `(4, 0)`
// rows and `[]` findings, silent.

/// **The silent direction, which is the row.** A minted cylinder face
/// swapped onto a chart that mints nothing keeps none of its rows: the
/// pass skips a planar face, so a row left here is one no reader could
/// ever be warned about, and `props`, the tessellator and
/// `chart_boundary` would read four cylinder curves off a plane.
///
/// **What discriminates here is the row count, and only that.** The
/// pass skips a planar face whatever it holds (`chart_mints`), so
/// `validate_pcurves` answers `[]` on this body before the drop and
/// after it alike — the finding list is the silence this row is ABOUT,
/// never evidence the drop happened. `rows_of == (0, 4)` is the whole
/// signal, and it is what reads `(4, 0)` on a tree that carries.
#[test]
fn a_swap_onto_a_chart_that_mints_nothing_drops_the_faces_rows() {
    let mut s = sheet();
    s.body
        .set_face_surface(
            s.low,
            FaceSurface::New {
                surface: flat(),
                sense: true,
            },
        )
        .unwrap();
    assert_eq!(rows_of(&s.body, s.low), (0, 4));
    // Unchanged by the drop, and stated here as the silence it is.
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
    // Only this face: the sheet's other curved panel is untouched, and
    // it is still complete.
    assert_eq!(rows_of(&s.body, s.up), (4, 0));
    // The pass is what puts a face's rows on the chart it is now on,
    // and a plane is a chart it mints none for.
    topo::mint_pcurves(&mut s.body, tol()).unwrap();
    assert_eq!(rows_of(&s.body, s.low), (0, 4));
    assert_eq!(rows_of(&s.body, s.up), (4, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// The loud direction loses its noise and keeps its meaning: onto
/// ANOTHER minting chart the four rows used to survive and be refused
/// one by one, and now they are gone and there is nothing to refuse.
/// What the trade buys is the same thing it buys at the loop doors —
/// the body no longer HOLDS a row about a surface the face is not on.
///
/// **And the loudness is not lost, it moves to where it belongs.** A
/// face swapped onto a cylinder its own boundary does not lie on is
/// geometrically wrong, not merely unminted, and the caller's re-mint
/// is what says so: the pass refuses the face by name rather than
/// deriving a row for it. The stored rows were never the thing that
/// reported this.
#[test]
fn a_swap_onto_another_minting_chart_drops_the_rows_and_the_refusals_with_them() {
    let mut s = sheet();
    s.body
        .set_face_surface(
            s.low,
            FaceSurface::New {
                surface: other_cylinder(),
                sense: true,
            },
        )
        .unwrap();
    assert_eq!(rows_of(&s.body, s.low), (0, 4));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);

    let refused = topo::mint_pcurves(&mut s.body, tol())
        .expect_err("a rim arc of radius 1 is not a curve of the radius-2 chart");
    // WHICH half-edge refuses, not merely that one does: the refusal
    // must be about the re-charted face, and the pass walks the face
    // arena in order, so it is this face's first rim arc.
    let PcurveMintError::Certify { half_edge, .. } = refused else {
        panic!("the re-mint refuses the rim it cannot state: {refused:?}")
    };
    let cycle = outer_cycle(&s.body, s.low);
    assert!(
        cycle.contains(&half_edge),
        "the refusal names a half-edge of the re-charted face"
    );
    assert_eq!(half_edge, cycle[0]);
}

/// **A face's rows are its loops' rows, rings included.** A setter that
/// walked only the outer loop would leave a demoted ring's four rows
/// behind on the new chart, which is the same defect one level in.
#[test]
fn a_swap_drops_the_rows_of_every_loop_of_the_face() {
    let mut s = sheet();
    s.body.kfmrh(s.low, s.up).unwrap();
    assert_eq!(rows_of(&s.body, s.low), (8, 0));
    assert_eq!(s.body.get_face(s.low).unwrap().rings.len(), 1);

    s.body
        .set_face_surface(
            s.low,
            FaceSurface::New {
                surface: flat(),
                sense: true,
            },
        )
        .unwrap();
    assert_eq!(rows_of(&s.body, s.low), (0, 8));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// **The control: the same key is the same chart**, and `Inherit` and
/// `Shared` naming it move nothing at all — interval, image and
/// certificate alike.
///
/// **What this pins is the setter's `new == old` guard, not
/// `Body::same_chart`.** Both specs resolve to the key the face is
/// already on, so the door that drops rows is never entered and no
/// rung of the predicate is exercised; a setter that dropped on every
/// swap it DID enter would leave this row green. The rung that says
/// two keys are one chart is the row below.
#[test]
fn a_swap_onto_the_faces_own_key_keeps_every_row_byte_for_byte() {
    let s = sheet();
    let before = rows_deep(&s.body, s.low);
    assert_eq!(before.len(), 4);

    let mut i = sheet();
    i.body
        .set_face_surface(i.low, FaceSurface::Inherit)
        .unwrap();
    assert_eq!(rows_deep(&i.body, i.low), before);

    let mut k = sheet();
    let cyl = k.body.get_face(k.low).unwrap().surface;
    k.body
        .set_face_surface(
            k.low,
            FaceSurface::Shared {
                key: cyl,
                sense: true,
            },
        )
        .unwrap();
    assert_eq!(rows_deep(&k.body, k.low), before);
    assert_eq!(validate_pcurves(&k.body, band()), vec![]);
}

/// **What the setter cannot see, measured rather than asserted**, the
/// same bound as the loop doors': a swap onto a second key holding an
/// equal surface reads as a chart change whether or not one recipe is
/// stamped on both, and the rows go. A re-mint, never a wrong row
/// (`work/origin/two-provenance-free-keys-holding-one-surface-read-as-two-charts`).
#[test]
fn a_swap_onto_an_equal_surface_on_another_key_reads_as_a_chart_change() {
    for stamped in [false, true] {
        let mut s = sheet();
        let cyl = s.body.get_face(s.low).unwrap().surface;
        // A second key for the same cylinder, minted on the rowless
        // planar face so that nothing is dropped establishing it.
        let second = s
            .body
            .set_face_surface(
                s.plane,
                FaceSurface::New {
                    surface: cylinder(),
                    sense: true,
                },
            )
            .unwrap();
        assert_ne!(second, cyl, "`New` mints a fresh key for an equal surface");
        if stamped {
            s.body.set_surface_source(cyl, one_recipe()).unwrap();
            s.body.set_surface_source(second, one_recipe()).unwrap();
        }

        s.body
            .set_face_surface(
                s.low,
                FaceSurface::Shared {
                    key: second,
                    sense: true,
                },
            )
            .unwrap();
        assert_eq!(rows_of(&s.body, s.low), (0, 4), "stamped: {stamped}");

        topo::mint_pcurves(&mut s.body, tol()).unwrap();
        assert_eq!(rows_of(&s.body, s.low), (4, 0), "stamped: {stamped}");
        assert_eq!(validate_pcurves(&s.body, band()), vec![]);
    }
}

/// **The sibling setter is not the same case.** `set_edge_curve` moves
/// neither a row's key nor its chart — it changes what the row must
/// agree WITH — and the pcurve pass re-derives that agreement from the
/// edge's CURRENT carrier on every run. So a carrier swap that leaves a
/// row saying the old image is refused per half-edge, loud, on the same
/// body where the surface setter was silent: this row swaps a rim ARC
/// for the straight line between its own endpoints and reads the two
/// refusals, one per side of the edge.
#[test]
fn an_edge_carrier_swap_leaves_rows_the_pcurve_pass_refuses_loud() {
    let mut s = sheet();
    let he = first_he(&s.body, s.low);
    let edge = s.body.get_half_edge(he).unwrap().edge;
    let start = s.body.get_half_edge(he).unwrap().start;
    let end = s.body.half_edge_end(he).unwrap();
    let p0 = *s
        .body
        .get_point(s.body.get_vertex(start).unwrap().point)
        .unwrap();
    let p1 = *s
        .body
        .get_point(s.body.get_vertex(end).unwrap().point)
        .unwrap();

    s.body
        .set_edge_curve(edge, EdgeCurveSpec::line_between(p0, p1), tol())
        .unwrap();
    assert_eq!(rows_of(&s.body, s.low), (4, 0));
    let findings = validate_pcurves(&s.body, band());
    let certify = findings
        .iter()
        .filter(|f| matches!(f, PcurveMintError::Certify { .. }))
        .count();
    assert_eq!((certify, findings.len()), (2, 2));
}

/// The half-edges of `face`'s outer loop, in cycle order.
fn outer_cycle(body: &Body<f64>, face: FaceKey) -> Vec<topo::HalfEdgeKey> {
    let outer = body.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the fixture's faces are bounded by cycles")
    };
    body.loop_cycle(first).unwrap()
}

/// **The sibling setter's loudness is the pcurve pass's, and the pass
/// is silent on a half-minted face.** `validate_pcurves` runs its
/// re-certification only where a face's row set is COMPLETE
/// (`work/trim/validate-pcurves-never-recertifies-a-face-it-finds-incomplete`):
/// one rowless half-edge and the whole face is reported `MissingCache`
/// and measured no further. So the same carrier swap the row above
/// reads two refusals for is refused ONCE here — by the mate face,
/// which is still complete — and the staled row on the half-minted face
/// is accepted unmeasured.
///
/// That is a property of the pass rather than of the door, and it is
/// why `set_edge_curve` keeps the rows it finds with the blind spot
/// named — re-minting only where a null edge gets its first carrier —
/// rather than dropping rows to convert it into a `MissingCache`:
/// every content staleness in the tree meets the same silence, and the
/// row that closes it closes them all.
#[test]
fn a_carrier_swap_on_a_half_minted_face_is_refused_only_by_the_complete_side() {
    let mut s = sheet();
    let he = first_he(&s.body, s.low);
    let edge = s.body.get_half_edge(he).unwrap().edge;
    let mate = s.body.get_edge(edge).unwrap().he_minus;
    let mate = if mate == he {
        s.body.get_edge(edge).unwrap().he_plus
    } else {
        mate
    };
    let start = s.body.get_half_edge(he).unwrap().start;
    let end = s.body.half_edge_end(he).unwrap();
    let p0 = *s
        .body
        .get_point(s.body.get_vertex(start).unwrap().point)
        .unwrap();
    let p1 = *s
        .body
        .get_point(s.body.get_vertex(end).unwrap().point)
        .unwrap();

    // Half-mint `low`: drop ONE row that is not the swapped edge's.
    let victim = *outer_cycle(&s.body, s.low)
        .iter()
        .find(|&&h| h != he)
        .unwrap();
    assert!(s.body.detach_pcurve(victim).is_some());

    s.body
        .set_edge_curve(edge, EdgeCurveSpec::line_between(p0, p1), tol())
        .unwrap();
    let findings = validate_pcurves(&s.body, band());
    let refused: Vec<topo::HalfEdgeKey> = findings
        .iter()
        .filter_map(|f| match f {
            PcurveMintError::Certify { half_edge, .. } => Some(*half_edge),
            _ => None,
        })
        .collect();
    let absent: Vec<topo::HalfEdgeKey> = findings
        .iter()
        .filter_map(|f| match f {
            PcurveMintError::MissingCache { half_edge } => Some(*half_edge),
            _ => None,
        })
        .collect();
    assert_eq!(absent, vec![victim]);
    assert_eq!(
        refused,
        vec![mate],
        "only the mate face re-certifies; `low`'s staled row is measured by nothing"
    );
    assert!(
        s.body.pcurve(he).is_some(),
        "the staled row is still there — unmeasured, not removed"
    );
}

// ---------------------------------------------------------------
// The one tie across two keys: a shared payload `Arc`.
// ---------------------------------------------------------------
//
// Every face is put on a NURBS patch, which drops every row, and the
// minted rows are then re-attached verbatim through
// `Body::attach_pcurve`. Those rows are not coherent with the patch,
// so these rows assert what each door DECIDED — carried or dropped —
// and never ask the pcurve pass. Each runs twice: with one payload
// `Arc` on every key, and with a deep copy per key, which holds an
// equal patch with no identity tie and must drop.

/// A flat bilinear patch.
fn patch() -> std::sync::Arc<geom::NurbsSurface<f64>> {
    let k = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
    ];
    std::sync::Arc::new(geom::NurbsSurface::new(k.clone(), k, control, vec![1.0; 4]).unwrap())
}

struct ArcSheet {
    s: Sheet,
    /// `low`'s, `up`'s and `plane`'s keys: one payload `Arc` on three
    /// keys (`tied`), or three deep copies.
    keys: [topo::SurfaceKey; 3],
}

/// The sheet with all three faces on [`patch`] — tied or deep-copied —
/// and the two curved panels' eight minted rows attached back.
fn arc_sheet(tied: bool) -> ArcSheet {
    let mut s = sheet();
    let saved: Vec<_> = s.body.pcurves().map(|(h, c)| (h, c.clone())).collect();
    assert_eq!(saved.len(), 8);
    let p = patch();
    let mut keys = Vec::new();
    for face in [s.low, s.up, s.plane] {
        let payload = if tied {
            p.clone()
        } else {
            std::sync::Arc::new((*p).clone())
        };
        keys.push(
            s.body
                .set_face_surface(
                    face,
                    FaceSurface::New {
                        surface: Surface::Nurbs(payload),
                        sense: true,
                    },
                )
                .unwrap(),
        );
    }
    let arc = |k| match s.body.get_surface(k) {
        Some(Surface::Nurbs(x)) => x.clone(),
        _ => panic!("every face was put on the patch"),
    };
    assert_eq!(
        std::sync::Arc::ptr_eq(&arc(keys[0]), &arc(keys[1])),
        tied,
        "the door keeps the caller's payload Arc"
    );
    assert_eq!(rows_total(&s.body), 0, "the swaps dropped every row");
    for (h, c) in saved {
        s.body.attach_pcurve(h, c);
    }
    assert_eq!(rows_of(&s.body, s.low), (4, 0));
    assert_eq!(rows_of(&s.body, s.up), (4, 0));
    ArcSheet {
        s,
        keys: [keys[0], keys[1], keys[2]],
    }
}

/// **The swap decides before it reaps.** Moving `low` onto `up`'s key
/// orphans `low`'s own key, which the swap removes; a decision taken
/// after that removal reads a dead key and drops the rows whatever
/// the two keys held.
#[test]
fn a_swap_orphaning_the_old_key_carries_every_row_across_one_payload() {
    for tied in [true, false] {
        let ArcSheet { mut s, keys } = arc_sheet(tied);
        s.body
            .set_face_surface(
                s.low,
                FaceSurface::Shared {
                    key: keys[1],
                    sense: true,
                },
            )
            .unwrap();
        assert!(
            s.body.get_surface(keys[0]).is_none(),
            "the old key was reaped"
        );
        let want = if tied { (4, 0) } else { (0, 4) };
        assert_eq!(rows_of(&s.body, s.low), want, "tied: {tied}");
    }
}

/// **`kef` decides before it reaps.** Killing the rim between the two
/// panels from `low`'s side kills `low` and orphans its key; the
/// remnant's three rows are carried onto `up` only if the decision
/// was taken while that key still resolved.
#[test]
fn kef_reaping_the_dying_key_carries_the_remnant_across_one_payload() {
    for tied in [true, false] {
        let ArcSheet { mut s, keys } = arc_sheet(tied);
        let he = he_at(&s.body, s.low, at(U1, VM));
        let killed = s.body.kef(he).unwrap();
        assert_eq!(killed.killed_face, s.low);
        assert!(
            s.body.get_surface(keys[0]).is_none(),
            "the dying key was reaped"
        );
        let want = if tied { (6, 0) } else { (3, 3) };
        assert_eq!(rows_of(&s.body, s.up), want, "tied: {tied}");
    }
}

#[test]
fn kfmrh_carries_every_row_across_one_payload() {
    for tied in [true, false] {
        let ArcSheet { mut s, .. } = arc_sheet(tied);
        s.body.kfmrh(s.low, s.up).unwrap();
        let want = if tied { (8, 0) } else { (4, 4) };
        assert_eq!(rows_of(&s.body, s.low), want, "tied: {tied}");
    }
}

/// `up` is first moved onto `low`'s key, so the demotion into `low`
/// is one key and carries whatever `up` kept; the ring then moves (or
/// is promoted) onto the plane face's key, which is the question.
///
/// The ring's row count is pinned, not read off the body: on the tied
/// fixture `up` keeps its four rows across the move onto `low`'s key,
/// and a count read after that move would take whatever the setter
/// left and pass either door's half whatever it did.
#[test]
fn ring_move_and_mfkrh_carry_every_row_across_one_payload() {
    const RING_ROWS: usize = 4;
    let mut failures = Vec::new();
    for tied in [true, false] {
        let ArcSheet { mut s, keys } = arc_sheet(tied);
        s.body
            .set_face_surface(
                s.up,
                FaceSurface::Shared {
                    key: keys[0],
                    sense: true,
                },
            )
            .unwrap();
        s.body.kfmrh(s.low, s.up).unwrap();
        let ring = ring_of(&s.body, s.low);
        s.body.ring_move(ring, s.plane).unwrap();
        let want = if tied {
            (RING_ROWS, 6 + 4 - RING_ROWS)
        } else {
            (0, 10)
        };
        let got = rows_of(&s.body, s.plane);
        if got != want {
            failures.push(format!("ring_move tied {tied}: want {want:?}, got {got:?}"));
        }

        let ArcSheet { mut s, keys } = arc_sheet(tied);
        s.body
            .set_face_surface(
                s.up,
                FaceSurface::Shared {
                    key: keys[0],
                    sense: true,
                },
            )
            .unwrap();
        s.body.kfmrh(s.low, s.up).unwrap();
        let ring = ring_of(&s.body, s.low);
        let made = s
            .body
            .mfkrh(
                ring,
                FaceSurface::Shared {
                    key: keys[2],
                    sense: false,
                },
            )
            .unwrap();
        let want = if tied {
            (RING_ROWS, 4 - RING_ROWS)
        } else {
            (0, 4)
        };
        let got = rows_of(&s.body, made.face);
        if got != want {
            failures.push(format!("mfkrh tied {tied}: want {want:?}, got {got:?}"));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// `mef` on the patch sheet. Its lower panel's rows are complete, and a
/// face with complete rows on a SPLINE chart is the Euler operators'
/// fitted frontier: the op refuses typed and moves nothing. So the
/// carry is read on the panel half-minted — `f→a`'s row detached, a
/// face the operator leaves as found — where the run `[a→b, b→c]`
/// moves by the payload alone.
#[test]
fn mef_carries_the_runs_rows_across_one_payload() {
    for tied in [true, false] {
        let ArcSheet { mut s, keys } = arc_sheet(tied);
        let (a, c) = (at(U0, V0), at(U1, VM));
        let (he1, he2) = (he_at(&s.body, s.low, a), he_at(&s.body, s.low, c));
        let chord = || EdgeCurveSpec::line_between(a, c);
        let before = rows_deep(&s.body, s.low);
        let refused = s
            .body
            .mef(
                MefSite::Chords { he1, he2 },
                chord(),
                FaceSurface::Shared {
                    key: keys[2],
                    sense: true,
                },
                tol(),
            )
            .unwrap_err();
        assert_eq!(
            refused,
            topo::EulerOpError::PcurveMint {
                face: s.low,
                refusal: topo::SiteRowRefusal::SplineChart,
            },
            "tied: {tied}"
        );
        assert_eq!(rows_deep(&s.body, s.low), before, "tied: {tied}");

        let fa = he_at(&s.body, s.low, at(U0, VM));
        assert!(s.body.detach_pcurve(fa).is_some());
        let made = s
            .body
            .mef(
                MefSite::Chords { he1, he2 },
                chord(),
                FaceSurface::Shared {
                    key: keys[2],
                    sense: true,
                },
                tol(),
            )
            .unwrap();
        let want = if tied { (2, 1) } else { (0, 3) };
        assert_eq!(rows_of(&s.body, made.face), want, "tied: {tied}");
    }
}

// ---------------------------------------------------------------
// A minted or re-charted face's `sense` (D1): derived on the parent's
// chart, stated on any other, and a contradicting stated bit refused.
// ---------------------------------------------------------------

fn sense_of(body: &Body<f64>, face: FaceKey) -> bool {
    body.get_face(face).unwrap().sense
}

/// The payload `Arc` a patch-charted key holds.
fn payload_of(body: &Body<f64>, key: topo::SurfaceKey) -> std::sync::Arc<geom::NurbsSurface<f64>> {
    match body.get_surface(key) {
        Some(Surface::Nurbs(x)) => x.clone(),
        _ => panic!("the key is on a patch"),
    }
}

/// A spec a `sense` row hands a door, relative to the parent's own key
/// and a second key that holds the parent's payload `Arc` on a tied
/// fixture and a deep copy of it on an untied one.
#[derive(Clone, Copy, Debug)]
enum Spec {
    Inherit,
    /// `Shared` on the parent's own key, stating the bit.
    OwnKey(bool),
    /// `Shared` on the second key, stating the bit.
    SecondKey(bool),
    /// `New` holding the parent's own payload `Arc`, stating the bit.
    OwnPayload(bool),
    /// `New` on a patch of its own, stating the bit.
    AnotherPatch(bool),
}

impl Spec {
    fn build(
        self,
        body: &Body<f64>,
        own: topo::SurfaceKey,
        second: topo::SurfaceKey,
    ) -> FaceSurface<f64> {
        match self {
            Spec::Inherit => FaceSurface::Inherit,
            Spec::OwnKey(sense) => FaceSurface::Shared { key: own, sense },
            Spec::SecondKey(sense) => FaceSurface::Shared { key: second, sense },
            Spec::OwnPayload(sense) => FaceSurface::New {
                surface: Surface::Nurbs(payload_of(body, own)),
                sense,
            },
            Spec::AnotherPatch(sense) => FaceSurface::New {
                surface: Surface::Nurbs(patch()),
                sense,
            },
        }
    }
}

/// What a door did with a spec: the face's bit and whether the spec
/// landed on the parent's chart (read off the rows it carried), or the
/// refusal.
type Want = Result<(bool, bool), ()>;

/// The rows for a door that keeps the parent's side — `mef` and
/// `set_face_surface` — on a parent whose bit is `false`: whether the
/// fixture is tied, the spec, and what the door must do. On the
/// parent's chart the bit is derived (`false`) and the other one is
/// refused; anywhere else the stated bit is written.
fn with_parent_rows() -> [(bool, Spec, Want); 13] {
    use Spec::*;
    [
        (true, Inherit, Ok((false, true))),
        (true, OwnKey(false), Ok((false, true))),
        (true, OwnKey(true), Err(())),
        (true, SecondKey(false), Ok((false, true))),
        (true, SecondKey(true), Err(())),
        (true, OwnPayload(false), Ok((false, true))),
        (true, OwnPayload(true), Err(())),
        (false, SecondKey(false), Ok((false, false))),
        (false, SecondKey(true), Ok((true, false))),
        (false, OwnKey(false), Ok((false, true))),
        (false, OwnKey(true), Err(())),
        (true, AnotherPatch(false), Ok((false, false))),
        (true, AnotherPatch(true), Ok((true, false))),
    ]
}

/// The refusal a row wants: typed, naming the parent and both bits,
/// with the body deep-untouched.
fn refused_untouched(
    name: &str,
    got: Result<(), topo::EulerOpError>,
    parent: FaceKey,
    stated: bool,
    before: &str,
    body: &Body<f64>,
    failures: &mut Vec<String>,
) {
    let want = topo::EulerOpError::SenseContradictsChart {
        face: parent,
        stated,
        derived: !stated,
    };
    if got != Err(want.clone()) {
        failures.push(format!("{name}: want {want:?}, got {got:?}"));
    }
    if format!("{body:?}") != before {
        failures.push(format!("{name}: the refusal touched the body"));
    }
}

/// The bit a spec states, if it states one.
fn stated(spec: Spec) -> bool {
    match spec {
        Spec::Inherit => unreachable!("a refusal row states a bit"),
        Spec::OwnKey(b) | Spec::SecondKey(b) | Spec::OwnPayload(b) | Spec::AnotherPatch(b) => b,
    }
}

/// **`mef` (`Chords`).** The lower panel is reversed and split,
/// half-minted as above, with each spec. The run's rows are carried
/// exactly where the spec is on the parent's chart, which is where the
/// bit is the parent's.
#[test]
fn mef_derives_the_parents_bit_on_its_chart_and_writes_the_stated_one_elsewhere() {
    let mut failures = Vec::new();
    for (tied, spec, want) in with_parent_rows() {
        let name = format!("mef {spec:?} tied {tied}");
        let ArcSheet { mut s, keys } = arc_sheet(tied);
        s.body.set_face_sense(s.low, false).unwrap();
        let (a, c) = (at(U0, V0), at(U1, VM));
        let (he1, he2) = (he_at(&s.body, s.low, a), he_at(&s.body, s.low, c));
        assert!(
            s.body
                .detach_pcurve(he_at(&s.body, s.low, at(U0, VM)))
                .is_some()
        );
        let before = format!("{:?}", s.body);
        let got = s.body.mef(
            MefSite::Chords { he1, he2 },
            EdgeCurveSpec::line_between(a, c),
            spec.build(&s.body, keys[0], keys[2]),
            tol(),
        );
        match (got, want) {
            (Ok(made), Ok((sense, carried))) => {
                let got = (
                    sense_of(&s.body, made.face),
                    rows_of(&s.body, made.face) == (2, 1),
                );
                if got != (sense, carried) {
                    failures.push(format!("{name}: want {:?}, got {got:?}", (sense, carried)));
                }
                if sense_of(&s.body, s.low) {
                    failures.push(format!("{name}: the parent lost its bit"));
                }
            }
            (got, Err(())) => refused_untouched(
                &name,
                got.map(|_| ()),
                s.low,
                stated(spec),
                &before,
                &s.body,
                &mut failures,
            ),
            (Err(e), Ok(_)) => failures.push(format!("{name}: refused {e:?}")),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// **`set_face_surface`.** The reversed lower panel re-charted in place
/// with each spec: on its own chart it keeps its bit (and its rows),
/// and anywhere else the stated bit is written (and the rows dropped).
#[test]
fn set_face_surface_keeps_the_bit_on_the_faces_chart_and_writes_the_stated_one_elsewhere() {
    let mut failures = Vec::new();
    for (tied, spec, want) in with_parent_rows() {
        let name = format!("set_face_surface {spec:?} tied {tied}");
        let ArcSheet { mut s, keys } = arc_sheet(tied);
        s.body.set_face_sense(s.low, false).unwrap();
        let before = format!("{:?}", s.body);
        let got = s
            .body
            .set_face_surface(s.low, spec.build(&s.body, keys[0], keys[2]));
        match (got, want) {
            (Ok(_), Ok((sense, carried))) => {
                let got = (sense_of(&s.body, s.low), rows_of(&s.body, s.low) == (4, 0));
                if got != (sense, carried) {
                    failures.push(format!("{name}: want {:?}, got {got:?}", (sense, carried)));
                }
            }
            (got, Err(())) => refused_untouched(
                &name,
                got.map(|_| ()),
                s.low,
                stated(spec),
                &before,
                &s.body,
                &mut failures,
            ),
            (Err(e), Ok(_)) => failures.push(format!("{name}: refused {e:?}")),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// **`mef` (`Lone`).** Two `mvfs` seeds on the patch — the first
/// reversed, the second holding its payload `Arc` or a deep copy — and
/// the first seed's lone vertex split into a self-loop face with each
/// spec. A seed has no rows to carry, so the carried column is read by
/// the two rows above and not here.
#[test]
fn mef_lone_derives_the_parents_bit_on_its_chart_and_writes_the_stated_one_elsewhere() {
    let mut failures = Vec::new();
    for (tied, spec, want) in with_parent_rows() {
        let name = format!("mef Lone {spec:?} tied {tied}");
        let p = patch();
        let mut body = Body::<f64>::new();
        let origin = Point3::new(0.2, 0.3, 0.0);
        let seed = body.mvfs(origin, true).unwrap();
        let other = body.mvfs(Point3::new(0.7, 0.6, 0.0), true).unwrap();
        let own = body
            .set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: Surface::Nurbs(p.clone()),
                    sense: false,
                },
            )
            .unwrap();
        let second_payload = if tied {
            p
        } else {
            std::sync::Arc::new((*p).clone())
        };
        let second = body
            .set_face_surface(
                other.face,
                FaceSurface::New {
                    surface: Surface::Nurbs(second_payload),
                    sense: true,
                },
            )
            .unwrap();
        let before = format!("{body:?}");
        let got = body.mef(
            MefSite::Lone {
                r#loop: seed.r#loop,
            },
            EdgeCurveSpec::self_loop_circle_at(origin),
            spec.build(&body, own, second),
            tol(),
        );
        match (got, want) {
            (Ok(made), Ok((sense, _))) => {
                if sense_of(&body, made.face) != sense {
                    failures.push(format!("{name}: want {sense}"));
                }
                if sense_of(&body, seed.face) {
                    failures.push(format!("{name}: the parent lost its bit"));
                }
            }
            (got, Err(())) => refused_untouched(
                &name,
                got.map(|_| ()),
                seed.face,
                stated(spec),
                &before,
                &body,
                &mut failures,
            ),
            (Err(e), Ok(_)) => failures.push(format!("{name}: refused {e:?}")),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// A genuine hole: the bored block, described, valid at tier 3.
fn bored_block(centres: &[f64]) -> Body<f64> {
    let mut body = topo::test_support::holed_block::<f64>(3.0, centres, Tol::witness());
    topo::test_support::describe_as_intersections(&mut body, Tol::witness());
    assert_eq!(topo::validate_geometric(&body, Tol::witness()), Ok(()));
    body
}

/// Every ring of `body`, with its face and that face's bit.
fn rings(body: &Body<f64>) -> Vec<(FaceKey, LoopKey, bool)> {
    body.faces()
        .flat_map(|(k, f)| f.rings.iter().map(move |&r| (k, r, f.sense)))
        .collect()
}

/// The bored block with each cap re-charted onto its own plane with
/// the normal negated, stating `false`: the same solid, with every
/// cap's bit `false`. The caps' rim edges are re-described against the
/// new keys, so the body stays tier-3 valid. (Adopted from the second
/// review of PR 3467.)
fn caps_on_flipped_planes(mut body: Body<f64>) -> Body<f64> {
    let caps: Vec<FaceKey> = rings(&body).into_iter().map(|(f, _, _)| f).collect();
    for cap in caps {
        let key = body.get_face(cap).unwrap().surface;
        let Some(Surface::Plane {
            origin,
            normal,
            u_ref,
        }) = body.get_surface(key).cloned()
        else {
            panic!("a planar cap");
        };
        body.set_face_surface(
            cap,
            FaceSurface::New {
                surface: Surface::Plane {
                    origin,
                    normal: -normal,
                    u_ref,
                },
                sense: false,
            },
        )
        .unwrap();
    }
    topo::test_support::describe_as_intersections(&mut body, Tol::witness());
    assert_eq!(
        topo::validate_geometric(&body, Tol::witness()),
        Ok(()),
        "the re-charted caps bound the same solid"
    );
    body
}

/// **`mfkrh` promotes a hole to a face facing against its parent.**
/// Each cap's bore ring on the bored block was wound clockwise about
/// the cap's outward normal, so on the cap's chart the promoted face
/// takes the cap's bit negated, and the body stays tier-3 valid. The
/// cap's own bit stated there is refused; on another chart the stated
/// bit is written. Run on the block as built (caps `true`) and with
/// its caps on their flipped planes (caps `false`), so the negation is
/// read from both bits.
#[test]
fn mfkrh_negates_the_parents_bit_on_its_chart_and_writes_the_stated_one_elsewhere() {
    let mut failures = Vec::new();
    for (caps, base) in [
        (true, bored_block(&[1.5])),
        (false, caps_on_flipped_planes(bored_block(&[1.5]))),
    ] {
        let found = rings(&base);
        assert_eq!(found.len(), 2, "one bore ring in each cap");
        assert!(
            found.iter().all(|&(_, _, p)| p == caps),
            "every cap carries {caps}"
        );
        mfkrh_rows(&base, found, &mut failures);
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// The `mfkrh` rows on one bored block: each ring in `found` promoted
/// from a clone of `base` with each spec.
fn mfkrh_rows(base: &Body<f64>, found: Vec<(FaceKey, LoopKey, bool)>, failures: &mut Vec<String>) {
    for (parent, ring, p) in found {
        let own = base.get_face(parent).unwrap().surface;
        let foreign = |sense| FaceSurface::New {
            surface: base.get_surface(own).unwrap().clone(),
            sense,
        };
        let rows: [(&str, FaceSurface<f64>, Result<bool, bool>); 5] = [
            ("Inherit", FaceSurface::Inherit, Ok(!p)),
            (
                "Shared(own key) stating the derived bit",
                FaceSurface::Shared {
                    key: own,
                    sense: !p,
                },
                Ok(!p),
            ),
            (
                "Shared(own key) stating the parent's bit",
                FaceSurface::Shared { key: own, sense: p },
                Err(p),
            ),
            (
                "New(an equal plane) stating false",
                foreign(false),
                Ok(false),
            ),
            ("New(an equal plane) stating true", foreign(true), Ok(true)),
        ];
        for (name, spec, want) in rows {
            let name = format!("mfkrh {name} on {parent:?} (sense {p})");
            let on_chart = !matches!(spec, FaceSurface::New { .. });
            let mut b = base.clone();
            let before = format!("{b:?}");
            match (b.mfkrh(ring, spec), want) {
                (Ok(made), Ok(sense)) => {
                    if sense_of(&b, made.face) != sense {
                        failures.push(format!("{name}: want {sense}"));
                    }
                    if on_chart && topo::validate_geometric(&b, Tol::witness()).is_err() {
                        failures.push(format!(
                            "{name}: {:?}",
                            topo::validate_geometric(&b, Tol::witness())
                        ));
                    }
                }
                (got, Err(stated)) => refused_untouched(
                    &name,
                    got.map(|_| ()),
                    parent,
                    stated,
                    &before,
                    &b,
                    failures,
                ),
                (Err(e), Ok(_)) => failures.push(format!("{name}: refused {e:?}")),
            }
        }
    }
}

/// **`mfkrh` reads a shared payload as the parent's chart.** The bored
/// block with two holes, its top cap re-charted onto a flat patch that
/// faces the cap's way. The first hole is promoted with `New` holding
/// the cap's payload, which mints a second key on it; the second hole
/// is then promoted with `Shared` naming that second key. Both are on
/// the cap's chart, so both take the cap's bit negated, and the cap's
/// own bit stated on the second key is refused.
///
/// Check 6, which reads `sense` against a loop's winding, reads planar
/// faces only, and the cap's rim edges cannot be re-described against
/// a patch (a plane–patch `Intersection` does not certify). So the
/// derived bits are read by check 6 on the cap's plane instead, where
/// they mean the same thing.
#[test]
fn mfkrh_derives_on_a_second_key_holding_the_parents_payload() {
    let mut body = bored_block(&[0.75, 2.25]);
    let caps: Vec<FaceKey> = rings(&body)
        .into_iter()
        .map(|(f, _, _)| f)
        .filter(|&f| {
            let key = body.get_face(f).unwrap().surface;
            matches!(body.get_surface(key), Some(Surface::Plane { normal, .. }) if normal.z > 0.5)
        })
        .collect();
    let top = caps[0];
    assert!(caps.iter().all(|&f| f == top), "one top cap");
    assert!(sense_of(&body, top), "the top cap faces its plane's way");
    let k = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 2.0),
        Point3::new(0.0, 2.0, 2.0),
        Point3::new(3.0, 0.0, 2.0),
        Point3::new(3.0, 2.0, 2.0),
    ];
    let cap_patch =
        std::sync::Arc::new(geom::NurbsSurface::new(k.clone(), k, control, vec![1.0; 4]).unwrap());
    let jet = cap_patch.ders(0.5, 0.5);
    assert!(
        jet.du.cross(jet.dv).z > 0.0,
        "the patch faces the plane's way"
    );
    let plane_key = body.get_face(top).unwrap().surface;
    let own = body
        .set_face_surface(
            top,
            FaceSurface::New {
                surface: Surface::Nurbs(cap_patch.clone()),
                sense: true,
            },
        )
        .unwrap();
    let holes = body.get_face(top).unwrap().rings.clone();
    assert_eq!(holes.len(), 2);

    let first = body
        .mfkrh(
            holes[0],
            FaceSurface::New {
                surface: Surface::Nurbs(cap_patch),
                sense: false,
            },
        )
        .unwrap();
    assert_ne!(first.surface, own, "a second key");
    assert!(
        !sense_of(&body, first.face),
        "New(the cap's payload): the cap's bit negated"
    );

    let mut failures = Vec::new();
    let before = format!("{body:?}");
    let refused = body.mfkrh(
        holes[1],
        FaceSurface::Shared {
            key: first.surface,
            sense: true,
        },
    );
    refused_untouched(
        "mfkrh Shared(second key, one payload) stating the cap's bit",
        refused.map(|_| ()),
        top,
        true,
        &before,
        &body,
        &mut failures,
    );
    let second = body
        .mfkrh(
            holes[1],
            FaceSurface::Shared {
                key: first.surface,
                sense: false,
            },
        )
        .unwrap();
    if sense_of(&body, second.face) {
        failures.push("mfkrh Shared(second key, one payload): want false".to_owned());
    }

    // Check 6 on the derived bits: each promoted face is put on the
    // cap's plane key (off its chart, so the bit is written as stated)
    // stating the bit derived on the patch, which means the same there
    // since the patch faces the plane's way. Check 6 must find nothing
    // on the face; the other bit is the control, and must draw
    // `LoopRoleInverted`. (Adopted from the first review of PR 3467.)
    assert!(
        body.get_surface(plane_key).is_some(),
        "the rims keep the cap's plane key live"
    );
    for f in [first.face, second.face] {
        let bit = sense_of(&body, f);
        for (stated, want_inverted) in [(bit, false), (!bit, true)] {
            let mut b = body.clone();
            b.set_face_surface(
                f,
                FaceSurface::Shared {
                    key: plane_key,
                    sense: stated,
                },
            )
            .unwrap();
            let on_f: Vec<String> = match topo::validate_geometric(&b, Tol::witness()) {
                Ok(()) => Vec::new(),
                Err(errors) => errors
                    .iter()
                    .map(|e| format!("{e:?}"))
                    .filter(|e| e.contains(&format!("{f:?}")))
                    .collect(),
            };
            let inverted = on_f.iter().any(|e| e.starts_with("LoopRoleInverted"));
            let ok = if want_inverted {
                inverted
            } else {
                on_f.is_empty()
            };
            if !ok {
                failures.push(format!(
                    "{f:?} on the plane stating {stated} (derived {bit}): {on_f:?}"
                ));
            }
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

// ---------------------------------------------------------------
// The stamp door's assertion: one recipe, one description (N6).
// ---------------------------------------------------------------

/// Stamps `one_recipe()` on two fresh keys holding `a` and `b`.
fn stamp_both<T: geom_core::Decide>(a: Surface<T>, b: Surface<T>, seed: Point3<T>) {
    let mut body = Body::<T>::new();
    let fa = body.mvfs(seed, true).unwrap().face;
    let fb = body.mvfs(seed, true).unwrap().face;
    let ka = body
        .set_face_surface(
            fa,
            FaceSurface::New {
                surface: a,
                sense: true,
            },
        )
        .unwrap();
    let kb = body
        .set_face_surface(
            fb,
            FaceSurface::New {
                surface: b,
                sense: true,
            },
        )
        .unwrap();
    assert_ne!(ka, kb);
    body.set_surface_source(ka, one_recipe()).unwrap();
    body.set_surface_source(kb, one_recipe()).unwrap();
}

/// Whether stamping one recipe on `a` and `b` trips the stamp door's
/// assertion; any other panic propagates.
fn stamp_panics<T: geom_core::Decide>(a: Surface<T>, b: Surface<T>, seed: Point3<T>) -> bool {
    refused_by_the_stamp_door(|| stamp_both(a, b, seed))
}

/// Runs `stamp`, answering whether the N6 assertion refused it; any
/// other panic is re-raised, so a fixture failure cannot read as a
/// refusal.
fn refused_by_the_stamp_door(stamp: impl FnOnce()) -> bool {
    let Err(payload) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(stamp)) else {
        return false;
    };
    let message = payload
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| payload.downcast_ref::<&str>().copied())
        .unwrap_or_default();
    if !message.contains("one recipe evaluates to one description") {
        std::panic::resume_unwind(payload);
    }
    true
}

/// [`patch`] generic over the scalar, lifted by `dz`, with its second
/// weight `w`.
fn patch_at<T: geom_core::Real>(lift: impl Fn(f64) -> T, dz: f64, w: f64) -> Surface<T> {
    let k = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
        .into_iter()
        .map(|(x, y)| Point3::new(lift(x), lift(y), lift(dz)))
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        geom::NurbsSurface::new(k.clone(), k, control, vec![1.0, w, 1.0, 1.0]).unwrap(),
    ))
}

fn cone(half_angle: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::origin(),
        axis: axis(),
        half_angle,
        u_ref: u_ref(),
    }
}

fn sphere(radius: f64) -> Surface<f64> {
    Surface::Sphere {
        center: Point3::origin(),
        radius,
        axis: axis(),
        u_ref: u_ref(),
    }
}

fn torus(minor_radius: f64) -> Surface<f64> {
    Surface::Torus {
        center: Point3::origin(),
        axis: axis(),
        major_radius: 3.0,
        minor_radius,
        u_ref: u_ref(),
    }
}

fn plane_at_x(x: f64) -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(x, 0.0, 0.0),
        normal: axis(),
        u_ref: u_ref(),
    }
}

/// **Every kind, both ways**: one description under one recipe on two
/// keys is accepted, and two that differ in any one part — a scalar of
/// each kind, a NURBS net or weight, the kind itself, a zero's sign —
/// are refused.
#[cfg(debug_assertions)]
#[test]
fn the_stamp_door_refuses_one_recipe_over_two_descriptions_on_every_kind() {
    let id = |x: f64| x;
    let o = Point3::origin();
    let shared = Surface::Nurbs(patch());
    let equal = [
        ("plane", flat(), flat()),
        ("cylinder", cylinder(), cylinder()),
        ("cone", cone(0.3), cone(0.3)),
        ("sphere", sphere(1.0), sphere(1.0)),
        ("torus", torus(1.0), torus(1.0)),
        ("nurbs", patch_at(id, 0.0, 1.0), patch_at(id, 0.0, 1.0)),
        ("one payload", shared.clone(), shared),
    ];
    for (what, a, b) in equal {
        assert!(!stamp_panics(a, b, o), "{what}: an equal pair was refused");
    }
    let differing = [
        ("plane", plane_at_x(0.0), plane_at_x(1e-12)),
        ("cylinder", cylinder(), other_cylinder()),
        ("cone", cone(0.3), cone(0.31)),
        ("sphere", sphere(1.0), sphere(1.5)),
        ("torus", torus(1.0), torus(1.5)),
        ("nurbs net", patch_at(id, 0.0, 1.0), patch_at(id, 0.5, 1.0)),
        (
            "nurbs weight",
            patch_at(id, 0.0, 1.0),
            patch_at(id, 0.0, 2.0),
        ),
        ("kind", flat(), cylinder()),
        ("signed zero", plane_at_x(0.0), plane_at_x(-0.0)),
    ];
    for (what, a, b) in differing {
        assert!(
            stamp_panics(a, b, o),
            "{what}: a differing pair was accepted"
        );
    }
}

/// **At `Dual`, what needs no bit channel is still decided.** A dual
/// scalar has no bit channel, so two same-kind analytic surfaces that
/// differ only in a dual scalar offer no evidence and pass; but a kind
/// mismatch, and a NURBS pair whose `f64` weights differ, are refused.
#[cfg(debug_assertions)]
#[test]
fn the_stamp_door_decides_at_dual_what_needs_no_bit_channel() {
    use geom_core::Dual64 as D;
    let c = D::constant;
    let o = Point3::new(c(0.0), c(0.0), c(0.0));
    let v = |x: f64, y: f64, z: f64| Vec3::new(c(x), c(y), c(z));
    let plane = Surface::Plane {
        origin: o,
        normal: v(0.0, 0.0, 1.0),
        u_ref: v(1.0, 0.0, 0.0),
    };
    let cyl = |r: f64| Surface::Cylinder {
        origin: o,
        axis: v(0.0, 0.0, 1.0),
        radius: c(r),
        u_ref: v(1.0, 0.0, 0.0),
    };
    assert!(
        !stamp_panics(cyl(1.0), cyl(2.0), o),
        "a dual radius is no evidence"
    );
    assert!(
        !stamp_panics(patch_at(c, 0.0, 1.0), patch_at(c, 0.5, 1.0), o),
        "a dual control net is no evidence"
    );
    assert!(stamp_panics(plane, cyl(1.0), o), "kind mismatch at Dual");
    assert!(
        stamp_panics(patch_at(c, 0.0, 1.0), patch_at(c, 0.0, 2.0), o),
        "f64 weights at Dual"
    );
}

/// **Every holder is read.** A graft brings in a key carrying the
/// recipe over a PLANE beside the sheet's stamped cylinder; stamping a
/// third key holding the cylinder again agrees with the first holder
/// in arena order and disagrees with the second, and is refused.
#[cfg(debug_assertions)]
#[test]
fn the_stamp_door_reads_every_holder_not_the_first() {
    let mut s = sheet();
    let forged = forge(&mut s);
    let cyl = s.body.get_face(s.low).unwrap().surface;
    let third = s
        .body
        .set_face_surface(
            s.up,
            FaceSurface::New {
                surface: cylinder(),
                sense: true,
            },
        )
        .unwrap();
    assert!(third != cyl && third != forged);
    let refused = refused_by_the_stamp_door(|| {
        s.body.set_surface_source(third, one_recipe()).unwrap();
    });
    assert!(refused, "the forged holder went unread");
}
