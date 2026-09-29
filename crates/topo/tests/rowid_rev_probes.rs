//! Reviewer probes (origin-rowid-rev, PR 3414): the row doors against forged stamps, the Arc rung, and the stamp-door assertion.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Point3, Tol, Vec3};
use topo::{Body, FaceKey, FaceSurface, LoopKey, MefSite, MevSite};

fn tol() -> Tol {
    Tol::witness()
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

/// The two curved panels carry a complete row set and the pcurve pass
/// accepts the body. That is the whole of what this fixture is for and
/// the whole of what this row asserts — the row below measures what it
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

/// Splits the lower panel with a chord from `c` to `a`: the run
/// `[a→b, b→c]` — two rim-and-meridian rows — moves into the new
/// face's loop with the minted `c→a`, and the old face keeps `c→f`,
/// `f→a` and the minted `a→c`.
fn split_low(s: &mut Sheet, surface: FaceSurface<f64>) -> topo::MefCreated {
    let (a, c) = (at(U0, V0), at(U1, VM));
    let he1 = he_at(&s.body, s.low, a);
    let he2 = he_at(&s.body, s.low, c);
    s.body
        .mef(
            MefSite::Chords { he1, he2 },
            EdgeCurveSpec::line_between(a, c),
            surface,
            tol(),
        )
        .unwrap()
}

// ---------------------------------------------------------------
// Claim 2: a forged stamp (cylinder + plane under one recipe, brought
// in by the graft) moves no row at any of the six doors.
// ---------------------------------------------------------------

/// Stamps the sheet's cylinder with `one_recipe()` and grafts in a
/// plane carrying the same stamp; returns the forged plane key.
fn forge(s: &mut Sheet) -> topo::SurfaceKey {
    let cyl = s.body.get_face(s.low).unwrap().surface;
    s.body.set_surface_source(cyl, one_recipe()).unwrap();
    let mut other = Body::<f64>::new();
    let seed = other.mvfs(Point3::origin()).unwrap();
    let k = other
        .set_face_surface(seed.face, FaceSurface::New(flat()))
        .unwrap();
    other.set_surface_source(k, one_recipe()).unwrap();
    topo::graft_disjoint(&mut s.body, &other, tol()).unwrap();
    s.body
        .surfaces()
        .map(|(k, _)| k)
        .find(|&k| k != cyl && s.body.surface_source(k) == Some(&one_recipe()))
        .expect("the graft carried the plane's stamp")
}

#[test]
fn forged_set_face_surface_carries_no_row() {
    let mut s = sheet();
    let forged = forge(&mut s);
    s.body
        .set_face_surface(s.low, FaceSurface::Shared(forged))
        .unwrap();
    assert_eq!(rows_of(&s.body, s.low), (0, 4));
}

#[test]
fn forged_kfmrh_carries_no_row() {
    let mut s = sheet();
    let forged = forge(&mut s);
    s.body
        .set_face_surface(s.plane, FaceSurface::Shared(forged))
        .unwrap();
    s.body.kfmrh(s.plane, s.low).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (0, 10));
}

#[test]
fn forged_ring_move_carries_no_row() {
    let mut s = sheet();
    let forged = forge(&mut s);
    s.body.kfmrh(s.low, s.up).unwrap();
    let ring = ring_of(&s.body, s.low);
    s.body
        .set_face_surface(s.plane, FaceSurface::Shared(forged))
        .unwrap();
    s.body.ring_move(ring, s.plane).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (0, 10));
}

#[test]
fn forged_mfkrh_carries_no_row() {
    let mut s = sheet();
    let forged = forge(&mut s);
    s.body.kfmrh(s.low, s.up).unwrap();
    let ring = ring_of(&s.body, s.low);
    let made = s.body.mfkrh(ring, FaceSurface::Shared(forged)).unwrap();
    assert_eq!(rows_of(&s.body, made.face), (0, 4));
}

#[test]
fn forged_kef_carries_no_row() {
    let mut s = sheet();
    let forged = forge(&mut s);
    s.body
        .set_face_surface(s.plane, FaceSurface::Shared(forged))
        .unwrap();
    let he = he_at(&s.body, s.low, at(U0, V0));
    s.body.kef(he).unwrap();
    assert_eq!(rows_of(&s.body, s.plane), (0, 8));
}

#[test]
fn forged_mef_carries_no_row() {
    let mut s = sheet();
    let forged = forge(&mut s);
    let made = split_low(&mut s, FaceSurface::Shared(forged));
    assert_eq!(rows_of(&s.body, made.face), (0, 3));
}

// ---------------------------------------------------------------
// Claim 5: the Arc rung — the only tie left — witnessed at every
// door, including the two orderings the PR's removals left bare.
// Rows are re-attached verbatim after the swaps: the probe asks what
// the DOOR decides, not whether the row is coherent with the patch.
// ---------------------------------------------------------------

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
    /// keys, or (when `tied == false`) three deep copies.
    keys: [topo::SurfaceKey; 3],
}

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
                .set_face_surface(face, FaceSurface::New(Surface::Nurbs(payload)))
                .unwrap(),
        );
    }
    let arc = |k| match s.body.get_surface(k) {
        Some(Surface::Nurbs(x)) => x.clone(),
        _ => panic!("nurbs"),
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

#[test]
fn arc_set_face_surface_orphaning_the_old_key_carries_every_row() {
    for tied in [true, false] {
        let ArcSheet { mut s, keys } = arc_sheet(tied);
        s.body
            .set_face_surface(s.low, FaceSurface::Shared(keys[1]))
            .unwrap();
        assert!(
            s.body.get_surface(keys[0]).is_none(),
            "the old key was reaped"
        );
        let want = if tied { (4, 0) } else { (0, 4) };
        assert_eq!(rows_of(&s.body, s.low), want, "tied: {tied}");
    }
}

#[test]
fn arc_kef_reaping_the_dying_key_carries_the_remnant() {
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
fn arc_kfmrh_carries_every_row() {
    for tied in [true, false] {
        let ArcSheet { mut s, .. } = arc_sheet(tied);
        s.body.kfmrh(s.low, s.up).unwrap();
        let want = if tied { (8, 0) } else { (4, 4) };
        assert_eq!(rows_of(&s.body, s.low), want, "tied: {tied}");
    }
}

#[test]
fn arc_ring_move_and_mfkrh_carry_every_row() {
    for tied in [true, false] {
        // Demote `up` into `low` by one key (same face key is not the
        // question; kfmrh between two keys is the row above), then
        // move the ring onto the plane face's key.
        let ArcSheet { mut s, keys } = arc_sheet(tied);
        s.body
            .set_face_surface(s.up, FaceSurface::Shared(keys[0]))
            .unwrap();
        let before = rows_of(&s.body, s.up);
        s.body.kfmrh(s.low, s.up).unwrap();
        let ring = ring_of(&s.body, s.low);
        let ring_rows = before.0;
        s.body.ring_move(ring, s.plane).unwrap();
        let want = if tied {
            (ring_rows, 6 + 4 - ring_rows)
        } else {
            (0, 10)
        };
        assert_eq!(rows_of(&s.body, s.plane), want, "ring_move tied: {tied}");

        let ArcSheet { mut s, keys } = arc_sheet(tied);
        s.body
            .set_face_surface(s.up, FaceSurface::Shared(keys[0]))
            .unwrap();
        let before = rows_of(&s.body, s.up);
        s.body.kfmrh(s.low, s.up).unwrap();
        let ring = ring_of(&s.body, s.low);
        let made = s.body.mfkrh(ring, FaceSurface::Shared(keys[2])).unwrap();
        let want = if tied {
            (before.0, 4 - before.0)
        } else {
            (0, 4)
        };
        assert_eq!(rows_of(&s.body, made.face), want, "mfkrh tied: {tied}");
    }
}

#[test]
fn arc_mef_carries_the_runs_rows() {
    for tied in [true, false] {
        let ArcSheet { mut s, keys } = arc_sheet(tied);
        let made = split_low(&mut s, FaceSurface::Shared(keys[2]));
        let want = if tied { (2, 1) } else { (0, 3) };
        assert_eq!(rows_of(&s.body, made.face), want, "tied: {tied}");
    }
}

// ---------------------------------------------------------------
// Claim 4: the stamp-door assertion, per kind, and at Dual.
// ---------------------------------------------------------------

fn two_stamped<T: geom_core::Decide>(a: Surface<T>, b: Surface<T>, origin: Point3<T>) {
    let mut body = Body::<T>::new();
    let fa = body.mvfs(origin).unwrap().face;
    let fb = body.mvfs(origin).unwrap().face;
    let ka = body.set_face_surface(fa, FaceSurface::New(a)).unwrap();
    let kb = body.set_face_surface(fb, FaceSurface::New(b)).unwrap();
    assert_ne!(ka, kb);
    body.set_surface_source(ka, one_recipe()).unwrap();
    body.set_surface_source(kb, one_recipe()).unwrap();
}

fn f(a: Surface<f64>, b: Surface<f64>) {
    two_stamped(a, b, Point3::origin())
}

fn cone(h: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::origin(),
        axis: axis(),
        half_angle: h,
        u_ref: u_ref(),
    }
}
fn sphere(r: f64) -> Surface<f64> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: r,
        axis: axis(),
        u_ref: u_ref(),
    }
}
fn torus(r: f64) -> Surface<f64> {
    Surface::Torus {
        center: Point3::origin(),
        axis: axis(),
        major_radius: 3.0,
        minor_radius: r,
        u_ref: u_ref(),
    }
}
fn nurbs_moved(dz: f64, w: f64) -> Surface<f64> {
    let p = patch();
    let control: Vec<_> = p
        .control()
        .iter()
        .map(|q| Point3::new(q.x, q.y, q.z + dz))
        .collect();
    let k = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    Surface::Nurbs(std::sync::Arc::new(
        geom::NurbsSurface::new(k.clone(), k, control, vec![1.0, w, 1.0, 1.0]).unwrap(),
    ))
}

#[test]
fn assert_accepts_equal_bits_on_every_kind() {
    f(flat(), flat());
    f(cylinder(), cylinder());
    f(cone(0.3), cone(0.3));
    f(sphere(1.0), sphere(1.0));
    f(torus(1.0), torus(1.0));
    f(nurbs_moved(0.0, 1.0), nurbs_moved(0.0, 1.0));
    let p = Surface::Nurbs(patch());
    f(p.clone(), p);
}

#[test]
#[should_panic(expected = "one recipe evaluates")]
fn assert_fires_plane() {
    let mut g = flat();
    if let Surface::Plane { origin, .. } = &mut g {
        origin.x += 1e-12;
    }
    f(flat(), g);
}
#[test]
#[should_panic(expected = "one recipe evaluates")]
fn assert_fires_cylinder() {
    f(cylinder(), other_cylinder());
}
#[test]
#[should_panic(expected = "one recipe evaluates")]
fn assert_fires_cone() {
    f(cone(0.3), cone(0.31));
}
#[test]
#[should_panic(expected = "one recipe evaluates")]
fn assert_fires_sphere() {
    f(sphere(1.0), sphere(1.5));
}
#[test]
#[should_panic(expected = "one recipe evaluates")]
fn assert_fires_torus() {
    f(torus(1.0), torus(1.5));
}
#[test]
#[should_panic(expected = "one recipe evaluates")]
fn assert_fires_nurbs_net() {
    f(nurbs_moved(0.0, 1.0), nurbs_moved(0.5, 1.0));
}
#[test]
#[should_panic(expected = "one recipe evaluates")]
fn assert_fires_nurbs_weight() {
    f(nurbs_moved(0.0, 1.0), nurbs_moved(0.0, 2.0));
}
#[test]
#[should_panic(expected = "one recipe evaluates")]
fn assert_fires_kind_mismatch() {
    f(flat(), cylinder());
}
#[test]
#[should_panic(expected = "one recipe evaluates")]
fn assert_fires_signed_zero() {
    let with_x = |x: f64| {
        let mut g = flat();
        if let Surface::Plane { origin, .. } = &mut g {
            origin.x = x;
        }
        g
    };
    f(with_x(0.0), with_x(-0.0));
}

#[test]
fn assert_is_silent_at_dual() {
    use geom_core::Dual64 as D;
    let c = |x: f64| D::constant(x);
    let p = |x: f64, y: f64, z: f64| Point3::new(c(x), c(y), c(z));
    let v = |x: f64, y: f64, z: f64| Vec3::new(c(x), c(y), c(z));
    let plane = Surface::Plane {
        origin: p(0.0, 0.0, 0.0),
        normal: v(0.0, 0.0, 1.0),
        u_ref: v(1.0, 0.0, 0.0),
    };
    let cyl = Surface::Cylinder {
        origin: p(0.0, 0.0, 0.0),
        axis: v(0.0, 0.0, 1.0),
        radius: c(1.0),
        u_ref: v(1.0, 0.0, 0.0),
    };
    let cyl2 = Surface::Cylinder {
        origin: p(0.0, 0.0, 0.0),
        axis: v(0.0, 0.0, 1.0),
        radius: c(2.0),
        u_ref: v(1.0, 0.0, 0.0),
    };
    two_stamped(plane, cyl.clone(), p(0.0, 0.0, 0.0));
    two_stamped(cyl, cyl2, p(0.0, 0.0, 0.0));
}

/// Cost of the stamp-door scan: stamping N keys with N distinct
/// sources, the shape of editor-core's `stamp_minted_from`.
#[test]
#[ignore]
fn stamp_door_scan_cost() {
    for n in [250usize, 500, 1000] {
        let mut body = Body::<f64>::new();
        let mut keys = Vec::new();
        for i in 0..n {
            let fa = body.mvfs(Point3::origin()).unwrap().face;
            let mut g = flat();
            if let Surface::Plane { origin, .. } = &mut g {
                origin.x = i as f64;
            }
            keys.push(body.set_face_surface(fa, FaceSurface::New(g)).unwrap());
        }
        let t = std::time::Instant::now();
        for (i, k) in keys.iter().enumerate() {
            body.set_surface_source(*k, topo::GeomSource::minted(1, i as u32))
                .unwrap();
        }
        eprintln!("PERF n={n} stamp={:?}", t.elapsed());
    }
}
