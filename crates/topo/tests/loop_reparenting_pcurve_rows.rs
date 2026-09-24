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
//! across one chart, dropped across two.
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
use topo::{Body, CurveGeom, FaceKey, FaceSurface, LoopKey, MefSite, MevSite, PcurveMintError};

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

/// One recipe source. Two keys carrying the same one are two spellings
/// of one description (N6), which is how a body records that a second
/// key is the same chart.
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
    let seed = body.mvfs(a).unwrap();
    let cyl = body
        .set_face_surface(seed.face, FaceSurface::New(cylinder()))
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
            FaceSurface::New(plane),
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
            FaceSurface::Shared(cyl),
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
/// contain the two rim ARCS bounding it (the module docs say so), so
/// the structural battery refuses the body — every finding is about
/// that planar face's geometry, and none of them is about a pcurve
/// row. The pcurve pass and the structural battery ask different
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
    let made = s.body.mfkrh_plug(ring).unwrap();
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
    let made = s.body.mfkrh(ring, FaceSurface::New(other)).unwrap();
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

/// **A second key the body records as the same description carries
/// every row** (`kfmrh`). Splitting the sheet's two curved panels onto
/// two keys changes no row's meaning — both keys hold the cylinder —
/// and the rows still certify. So when the loop moves between them the
/// door carries them: what decides is the CHART, not the key, and the
/// body says the two keys are one chart by carrying one
/// [`topo::GeomSource`] on both (the merge door's second hard rung).
#[test]
fn kfmrh_onto_a_second_key_the_body_records_as_one_surface_carries_every_row() {
    let mut s = sheet();
    let cyl = s.body.get_face(s.low).unwrap().surface;
    let second = s
        .body
        .set_face_surface(s.up, FaceSurface::New(cylinder()))
        .unwrap();
    assert_ne!(second, cyl, "`New` mints a fresh key for an equal surface");
    s.body.set_surface_source(cyl, one_recipe()).unwrap();
    s.body.set_surface_source(second, one_recipe()).unwrap();
    // The swap re-charted `up` before either key carried a source, so
    // the setter read it as a chart change and dropped the face's rows;
    // the pass is what puts them back, on the key the face is now on.
    topo::mint_pcurves(&mut s.body, tol()).unwrap();
    assert_eq!(rows_of(&s.body, s.up), (4, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);

    s.body.kfmrh(s.low, s.up).unwrap();
    assert_eq!(rows_of(&s.body, s.low), (8, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// The same through `mfkrh`: a ring promoted onto a second key that
/// the body records as the same description keeps its rows.
#[test]
fn mfkrh_onto_a_second_key_the_body_records_as_one_surface_carries_every_row() {
    let mut s = sheet();
    let cyl = s.body.get_face(s.low).unwrap().surface;
    let second = s
        .body
        .set_face_surface(s.plane, FaceSurface::New(cylinder()))
        .unwrap();
    assert_ne!(second, cyl);
    s.body.set_surface_source(cyl, one_recipe()).unwrap();
    s.body.set_surface_source(second, one_recipe()).unwrap();

    s.body.kfmrh(s.low, s.up).unwrap();
    let ring = ring_of(&s.body, s.low);
    let made = s.body.mfkrh(ring, FaceSurface::Shared(second)).unwrap();
    assert_eq!(rows_of(&s.body, made.face), (4, 0));
    assert_eq!(rows_of(&s.body, s.low), (4, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// **What the door cannot see, measured rather than asserted.** Two
/// keys holding an equal surface with NO record tying them read as two
/// charts, and the moved loop's rows go. The body was complete and
/// correct before the move and is merely unminted after it — a
/// re-mint, never a wrong row — which is the conservative direction of
/// an identity channel that can be absent but never wrong. Deciding
/// those two keys equal means reading the surfaces' scalars
/// structurally, which needs a bound these doors do not carry:
/// `work/origin/two-provenance-free-keys-holding-one-surface-read-as-two-charts`.
#[test]
fn two_keys_holding_one_surface_with_no_provenance_read_as_two_charts() {
    let mut s = sheet();
    let cyl = s.body.get_face(s.low).unwrap().surface;
    let second = s
        .body
        .set_face_surface(s.up, FaceSurface::New(cylinder()))
        .unwrap();
    assert_ne!(second, cyl);
    assert!(s.body.surface_source(cyl).is_none());
    // The setter reads the same two keys the same way and drops `up`'s
    // rows; re-minting is what builds the body this row is about — one
    // carrying rows on two provenance-free keys.
    topo::mint_pcurves(&mut s.body, tol()).unwrap();
    assert_eq!(rows_of(&s.body, s.up), (4, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);

    s.body.kfmrh(s.low, s.up).unwrap();
    assert_eq!(rows_of(&s.body, s.low), (4, 4));
    let findings = validate_pcurves(&s.body, band());
    assert_eq!((missing(&findings), findings.len()), (4, 4));

    topo::mint_pcurves(&mut s.body, tol()).unwrap();
    assert_eq!(rows_of(&s.body, s.low), (8, 0));
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
        .set_face_surface(s.plane, FaceSurface::New(other_cylinder()))
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
        .set_face_surface(s.plane, FaceSurface::New(other_cylinder()))
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
    let made = split_low(&mut s, FaceSurface::New(flat()));
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
    let made = split_low(&mut s, FaceSurface::New(other_cylinder()));
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

    for surface in [FaceSurface::Inherit, FaceSurface::Shared(cyl)] {
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
        assert_eq!(rows_of(&s.body, made.face), (4, 0));
        assert_eq!(rows_of(&s.body, s.low), (4, 0));
        assert_eq!(validate_pcurves(&s.body, band()), vec![]);
    }
}

/// A second key the body records as one description is one chart, so
/// a `Shared` naming it carries the run's rows too — decided by the
/// chart, not the key.
#[test]
fn mef_onto_a_second_key_the_body_records_as_one_surface_carries_the_runs_rows() {
    let mut s = sheet();
    let cyl = s.body.get_face(s.low).unwrap().surface;
    let second = s
        .body
        .set_face_surface(s.plane, FaceSurface::New(cylinder()))
        .unwrap();
    assert_ne!(second, cyl);
    s.body.set_surface_source(cyl, one_recipe()).unwrap();
    s.body.set_surface_source(second, one_recipe()).unwrap();

    let (he1, he2, chord) = ruling_site(&mut s);
    let before = rows_deep(&s.body, s.low);
    let made = s
        .body
        .mef(
            MefSite::Chords { he1, he2 },
            chord,
            FaceSurface::Shared(second),
            tol(),
        )
        .unwrap();
    let moved = found_rows(&s.body, made.face, &made);
    assert_eq!(moved.len(), 3);
    for row in &moved {
        assert!(before.contains(row), "restated across one chart: {row}");
    }
    assert!(s.body.pcurve(made.he_minus).is_some());
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
        .set_face_surface(s.plane, FaceSurface::New(other_cylinder()))
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

/// And onto a second key the body records as the same description:
/// the remnant's rows are carried. **The carry is the `rows_deep`
/// count** — the three rows that arrive are three that left. The
/// `(5, 5)` reading beside it is the surviving face's own five
/// halves, which this fixture re-charted without minting; it pins the
/// fixture's state, not the door's.
#[test]
fn kef_into_a_second_key_the_body_records_as_one_surface_carries_the_remnants_rows() {
    let mut s = sheet();
    let cyl = s.body.get_face(s.low).unwrap().surface;
    let second = s
        .body
        .set_face_surface(s.plane, FaceSurface::New(cylinder()))
        .unwrap();
    s.body.set_surface_source(cyl, one_recipe()).unwrap();
    s.body.set_surface_source(second, one_recipe()).unwrap();
    let before = rows_deep(&s.body, s.low);

    let he = he_at(&s.body, s.low, at(U0, V0));
    s.body.kef(he).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (3, 5));
    let after = rows_deep(&s.body, s.plane);
    let carried = before.iter().filter(|row| after.contains(row)).count();
    assert_eq!(carried, 3, "the remnant's three rows arrive verbatim");
    let findings = validate_pcurves(&s.body, band());
    assert_eq!((missing(&findings), findings.len()), (5, 5));
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
        .set_face_surface(s.up, FaceSurface::New(rotated_cylinder()))
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

/// **The chart is decided where both keys resolve.** The dying face is
/// the ONLY holder of its key — the sheet's back, put on a second key
/// tied to the panel's by one recipe and minted — so this kill reaps
/// that key. The remnant still carries by provenance: a decision read
/// after the orphan sweep would find the dying key gone, read two
/// charts, and drop five rows that were right.
#[test]
fn kef_carries_the_remnant_by_provenance_when_it_reaps_the_dying_key() {
    let mut s = sheet();
    let cyl = s.body.get_face(s.low).unwrap().surface;
    let second = s
        .body
        .set_face_surface(s.plane, FaceSurface::New(cylinder()))
        .unwrap();
    s.body.set_surface_source(cyl, one_recipe()).unwrap();
    s.body.set_surface_source(second, one_recipe()).unwrap();
    topo::mint_pcurves(&mut s.body, tol()).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (6, 0));
    let back_before = rows_deep(&s.body, s.plane);

    // The back's half of the edge `a-b` starts at `b`.
    let he = he_at(&s.body, s.plane, at(U1, V0));
    let killed = s.body.kef(he).unwrap();
    assert_eq!(killed.killed_face, s.plane);
    assert_eq!(
        killed.killed_surface,
        Some(second),
        "the dying key had one holder, and this kill reaps it"
    );
    assert_eq!(rows_of(&s.body, s.low), (8, 0));
    let low_after = rows_deep(&s.body, s.low);
    let carried = back_before
        .iter()
        .filter(|row| low_after.contains(row))
        .count();
    assert_eq!(carried, 5, "every surviving back row arrives verbatim");
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
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
        .set_face_surface(s.low, FaceSurface::New(flat()))
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
        .set_face_surface(s.low, FaceSurface::New(other_cylinder()))
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
        .set_face_surface(s.low, FaceSurface::New(flat()))
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
        .set_face_surface(k.low, FaceSurface::Shared(cyl))
        .unwrap();
    assert_eq!(rows_deep(&k.body, k.low), before);
    assert_eq!(validate_pcurves(&k.body, band()), vec![]);
}

/// **The other control: a second key the body records as one
/// description.** What decides is the CHART, not the key — the same
/// rung the loop doors take — so a face swapped onto a fresh key
/// carrying its old key's `GeomSource` keeps every row it had.
#[test]
fn a_swap_onto_a_second_key_the_body_records_as_one_surface_keeps_every_row() {
    let mut s = sheet();
    let cyl = s.body.get_face(s.low).unwrap().surface;
    let before = rows_deep(&s.body, s.low);
    // A second key for the same cylinder, minted on the rowless planar
    // face so that nothing is dropped establishing it, and tied to the
    // first by one recipe.
    let second = s
        .body
        .set_face_surface(s.plane, FaceSurface::New(cylinder()))
        .unwrap();
    assert_ne!(second, cyl, "`New` mints a fresh key for an equal surface");
    s.body.set_surface_source(cyl, one_recipe()).unwrap();
    s.body.set_surface_source(second, one_recipe()).unwrap();

    s.body
        .set_face_surface(s.low, FaceSurface::Shared(second))
        .unwrap();
    assert_eq!(rows_deep(&s.body, s.low), before);
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
}

/// **What the setter cannot see, measured rather than asserted**, the
/// same bound as the loop doors': two keys holding an equal surface
/// with no record tying them read as two charts and the rows go. A
/// re-mint, never a wrong row
/// (`work/topo/two-provenance-free-keys-holding-one-surface-read-as-two-charts`).
#[test]
fn a_swap_onto_an_equal_surface_with_no_provenance_reads_as_a_chart_change() {
    let mut s = sheet();
    let cyl = s.body.get_face(s.low).unwrap().surface;
    assert!(s.body.surface_source(cyl).is_none());
    s.body
        .set_face_surface(s.low, FaceSurface::New(cylinder()))
        .unwrap();
    assert_eq!(rows_of(&s.body, s.low), (0, 4));

    topo::mint_pcurves(&mut s.body, tol()).unwrap();
    assert_eq!(rows_of(&s.body, s.low), (4, 0));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
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

/// **The chart compare reads two keys, and the setter's own orphan
/// sweep can take one of them away.** A swap onto a second key the
/// body records as one description leaves the first key referenced by
/// nothing — `set_face_surface` removes it — so a setter that asked
/// `Body::same_chart` after that sweep would be asking about a key that
/// resolves to nothing, get `false`, and silently drop the rows of a
/// face whose chart never moved.
///
/// The controls above cannot see that: they keep the old key alive
/// through the sheet's other panel and its edge descriptions. This row
/// hands every rim arc to the second key first, so the last swap really
/// does orphan the first, and then asserts the carry.
#[test]
fn a_same_chart_swap_that_orphans_the_old_key_keeps_every_row() {
    let mut s = sheet();
    let cyl = s.body.get_face(s.low).unwrap().surface;
    // A second key for the same cylinder, minted on the rowless planar
    // face, and tied to the first by one recipe.
    let second = s
        .body
        .set_face_surface(s.plane, FaceSurface::New(cylinder()))
        .unwrap();
    assert_ne!(second, cyl);
    s.body.set_surface_source(cyl, one_recipe()).unwrap();
    s.body.set_surface_source(second, one_recipe()).unwrap();
    s.body
        .set_face_surface(s.up, FaceSurface::Shared(second))
        .unwrap();
    assert_eq!(rows_of(&s.body, s.up), (4, 0), "one chart by `GeomSource`");

    // Re-state every rim arc as an image in `second`, so that after the
    // last swap nothing references `cyl` at all.
    let arcs: Vec<topo::EdgeKey> = s
        .body
        .edges()
        .filter(|(_, e)| {
            matches!(
                s.body.get_curve_geom(e.curve),
                Some(CurveGeom::Certified(c)) if matches!(c.carrier(), Curve3::Circle { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    for edge in arcs {
        let curve = s.body.get_edge(edge).unwrap().curve;
        let Some(CurveGeom::Certified(c)) = s.body.get_curve_geom(curve) else {
            panic!("the fixture's rim arcs are certified")
        };
        let mut spec = c.restated_spec();
        spec.description = EdgeDescriptionSpec::chart(second);
        s.body.set_edge_curve(edge, spec, tol()).unwrap();
    }
    assert!(s.body.get_surface(cyl).is_some(), "still the low panel's");

    let before = rows_deep(&s.body, s.low);
    s.body
        .set_face_surface(s.low, FaceSurface::Shared(second))
        .unwrap();
    assert!(
        s.body.get_surface(cyl).is_none(),
        "the old key was orphaned by this swap and swept"
    );
    assert_eq!(
        rows_deep(&s.body, s.low),
        before,
        "one chart: every row stands, though the key it was compared against is gone"
    );
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
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
/// why `set_edge_curve` stays `Neither` with the blind spot named
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
