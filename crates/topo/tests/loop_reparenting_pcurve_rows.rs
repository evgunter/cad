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

#[test]
fn the_fixture_is_complete_and_tier_three_clean() {
    let s = sheet();
    assert_eq!(rows_of(&s.body, s.low), (4, 0));
    assert_eq!(rows_of(&s.body, s.up), (4, 0));
    assert_eq!(rows_of(&s.body, s.plane), (0, 6));
    assert_eq!(validate_pcurves(&s.body, band()), vec![]);
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

/// Onto a DIFFERENT minting chart the rows are dropped too — and this
/// is the row that shows what the drop gives up. The promoted face is
/// a curved one, so the stale rows used to be measured and refused
/// (one `Certify` each); now there is nothing to refuse, and the face
/// reads as exactly what it is, one the minting pass has not run on.
/// That pass is the caller's step and it restores them.
#[test]
fn mfkrh_onto_a_different_curved_chart_drops_the_rows_rather_than_storing_a_lie() {
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

/// The caller's step after every move that dropped a row. The minting
/// pass derives exactly the rows the DESTINATION chart wants: none on
/// a planar face, which is the pass's own posture, and the full set on
/// a curved one, so the body is whole again either way.
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
