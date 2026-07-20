//! Shared test support: the geometric unit cube, generic over the
//! scalar lane (`f64`, `Dual`, `Interval` — every `Decide` scalar), and
//! the intersection-upgrade pass. Used by `geometric_cube.rs` and the
//! interval lane in `interval_body.rs`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(dead_code)] // each integration test uses the subset it needs

use geom_brep::{EdgeCurveSpec, EdgeGeometry, newell_plane};
use geom_core::{Band, Point3, Real};
use geom_surfaces::Surface;
use topo::{Body, FaceSurface, MefCreated, MefSite, MevCreated, MevSite, MvfsCreated};

/// Key bundle for the geometric unit cube.
#[allow(dead_code)]
pub struct GeoCube<T: Real> {
    pub body: Body<T>,
    pub seed: MvfsCreated,
    pub mevs: [MevCreated; 7],
    pub mefs: [MefCreated; 5],
}

/// The chord-line spec between two points, with the extrude-flavored
/// pushforward description (the sweep's own form for side struts).
pub fn line<T: Real>(p0: Point3<T>, p1: Point3<T>) -> EdgeCurveSpec<T> {
    EdgeCurveSpec::line_between(p0, p1)
}

/// A Newell-certified plane from an outward-CCW-ordered corner list.
pub fn plane<T: geom_core::Decide>(corners: &[Point3<T>]) -> Surface<T> {
    newell_plane(corners, Band::linear().unwrap()).unwrap()
}

/// Builds the geometric unit cube through the public operators: the
/// §9.4.2-minimal sequence with real geometry at every step — every
/// `mef` supplies its face's Newell plane, every edge a certified
/// chord-line carrier; the seed face (which survives as the top cap)
/// gets its plane via `set_face_surface` at the end (the documented
/// seed-face path).
pub fn geometric_cube<T: geom_core::Decide>() -> GeoCube<T> {
    let c = |x: f64, y: f64, z: f64| Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z));
    // Corners: A(0,0,0) B(1,0,0) C(1,1,0) D(0,1,0), primed = z+1.
    let (a, b, cc, d) = (
        c(0.0, 0.0, 0.0),
        c(1.0, 0.0, 0.0),
        c(1.0, 1.0, 0.0),
        c(0.0, 1.0, 0.0),
    );
    let (a1, b1, c1, d1) = (
        c(0.0, 0.0, 1.0),
        c(1.0, 0.0, 1.0),
        c(1.0, 1.0, 1.0),
        c(0.0, 1.0, 1.0),
    );

    let mut body = Body::<T>::new();
    let seed = body.mvfs(a).unwrap();
    let e_ab = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            line(a, b),
        )
        .unwrap();
    let strut = |body: &mut Body<T>, at, from, to| {
        body.mev(MevSite::Fan { he1: at, he2: at }, to, line(from, to))
            .unwrap()
    };
    let e_bc = strut(&mut body, e_ab.he_minus, b, cc);
    let e_cd = strut(&mut body, e_bc.he_minus, cc, d);
    // Bottom face: outward normal −z ⇒ CCW viewed from below is
    // A, D, C, B.
    let he_dc = body
        .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
        .unwrap();
    // The bottom mef's new edge runs start(he_dc) = D → start(he2) = A.
    let f_bottom = body
        .mef(
            MefSite::Chords {
                he1: he_dc,
                he2: e_ab.he_plus,
            },
            line(d, a),
            FaceSurface::New(plane(&[a, d, cc, b])),
        )
        .unwrap();
    let e_aa = strut(&mut body, e_ab.he_plus, a, a1);
    let e_bb = strut(&mut body, e_bc.he_plus, b, b1);
    let e_cc = strut(&mut body, e_cd.he_plus, cc, c1);
    let e_dd = strut(&mut body, f_bottom.he_plus, d, d1);
    // Side faces: outward-CCW corner orders (interior-left rule).
    let f_front = body
        .mef(
            MefSite::Chords {
                he1: e_aa.he_minus,
                he2: e_bb.he_minus,
            },
            line(a1, b1),
            FaceSurface::New(plane(&[a, b, b1, a1])),
        )
        .unwrap();
    let f_right = body
        .mef(
            MefSite::Chords {
                he1: e_bb.he_minus,
                he2: e_cc.he_minus,
            },
            line(b1, c1),
            FaceSurface::New(plane(&[b, cc, c1, b1])),
        )
        .unwrap();
    let f_back = body
        .mef(
            MefSite::Chords {
                he1: e_cc.he_minus,
                he2: e_dd.he_minus,
            },
            line(c1, d1),
            FaceSurface::New(plane(&[cc, d, d1, c1])),
        )
        .unwrap();
    let f_left = body
        .mef(
            MefSite::Chords {
                he1: e_dd.he_minus,
                he2: f_front.he_plus,
            },
            line(d1, a1),
            FaceSurface::New(plane(&[d, a, a1, d1])),
        )
        .unwrap();
    // The seed face survives as the top cap: attach its plane (outward
    // +z ⇒ CCW from above: A′ B′ C′ D′).
    body.set_face_surface(seed.face, FaceSurface::New(plane(&[a1, b1, c1, d1])))
        .unwrap();

    GeoCube {
        body,
        seed,
        mevs: [e_ab, e_bc, e_cd, e_aa, e_bb, e_cc, e_dd],
        mefs: [f_bottom, f_front, f_right, f_back, f_left],
    }
}

/// Re-describes every edge of a (planar-faced) body as the
/// `Intersection` of its two adjacent faces' surfaces, witness at the
/// edge midpoint, carrier the straight chord — the prefer-intrinsic
/// upgrade pass, through the certified `set_edge_curve` path.
pub fn upgrade_edges_to_intersections<T: geom_core::Decide>(body: &mut Body<T>) {
    let edges: Vec<_> = body.edges().map(|(k, e)| (k, e.clone())).collect();
    for (edge_key, edge) in edges {
        let face_surface = |body: &Body<T>, he| {
            let he_data = body.get_half_edge(he).unwrap();
            let loop_data = body.get_loop(he_data.parent_loop).unwrap();
            body.get_face(loop_data.face).unwrap().surface
        };
        let s1 = face_surface(body, edge.he_plus);
        let s2 = face_surface(body, edge.he_minus);
        let start = body.get_half_edge(edge.he_plus).unwrap().start;
        let end = body.half_edge_end(edge.he_plus).unwrap();
        let p0 = *body
            .get_point(body.get_vertex(start).unwrap().point)
            .unwrap();
        let p1 = *body.get_point(body.get_vertex(end).unwrap().point).unwrap();
        let mut spec = EdgeCurveSpec::line_between(p0, p1);
        spec.description = EdgeGeometry::Intersection {
            s1,
            s2,
            witness: p0.lerp(p1, T::from_f64(0.5)),
        };
        body.set_edge_curve(edge_key, spec).unwrap();
    }
}
