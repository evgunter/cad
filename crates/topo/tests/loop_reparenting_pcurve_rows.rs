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
/// `work/trim/validate-pcurves-cannot-tell-a-never-minted-face-from-an-emptied-one`.
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
/// `work/topo/two-provenance-free-keys-holding-one-surface-read-as-two-charts`.
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
