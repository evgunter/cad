//! Shared test support: the geometric unit cube and the prism builders,
//! generic over the scalar lane (`f64`, `Dual`, `Interval` — every
//! `Decide` scalar), the intersection-upgrade pass, and the declaration
//! flush. Most of this crate's suites declare `mod common;` — the
//! consumers are deliberately not listed here, because the compiler
//! knows that set and prose does not.
//!
//! Compiled into the test binary, not the library: the cheapest of this
//! crate's three homes for test vocabulary, and the right one whenever
//! the library itself never names the item. `topo`'s
//! `src/test_support_impl.rs` docs give the rule for all three.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(dead_code)] // loaded once per consumer; each uses a subset
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

use geom::Surface;
use geom_brep::{EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec, newell_plane};
use geom_core::Tol;
use geom_core::{Band, Point3, Real};
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
    newell_plane(corners, Band::linear(Tol::witness()).unwrap()).unwrap()
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
            Tol::witness(),
        )
        .unwrap();
    let strut = |body: &mut Body<T>, at, from, to| {
        body.mev(
            MevSite::Fan { he1: at, he2: at },
            to,
            line(from, to),
            Tol::witness(),
        )
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
            Tol::witness(),
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
            Tol::witness(),
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
            Tol::witness(),
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
            Tol::witness(),
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
            Tol::witness(),
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

/// Key bundle for a [`prism`] fixture.
pub struct Prism<T: Real> {
    pub body: Body<T>,
    /// Bottom-rim vertices, one per profile corner (same order).
    pub bottom: Vec<topo::VertexKey>,
    /// Top-rim vertices, one per profile corner (same order).
    pub top: Vec<topo::VertexKey>,
    pub bottom_face: topo::FaceKey,
    /// One side face per profile segment `i → i+1` (cyclic).
    pub side_faces: Vec<topo::FaceKey>,
    pub top_face: topo::FaceKey,
}

/// Builds a right prism over a simple polygon `profile` (x, y corners,
/// **counterclockwise viewed from +z**, no repeats), extruded from
/// z = 0 to z = `height` — the geometric_cube construction generalized
/// to N corners (reflex corners welcome). Every face gets its
/// outward-CCW Newell plane, every edge a certified chord line.
pub fn prism<T: geom_core::Decide>(profile: &[(f64, f64)], height: f64) -> Prism<T> {
    prism_z(profile, 0.0, height)
}

/// [`prism`] with an explicit z-range `[z0, z1]` (M3 PR 4: bricks at
/// arbitrary heights for the boolean fixtures).
pub fn prism_z<T: geom_core::Decide>(profile: &[(f64, f64)], z0: f64, z1: f64) -> Prism<T> {
    assert!(profile.len() >= 3);
    let n = profile.len();
    let c =
        |&(x, y): &(f64, f64), z: f64| Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z));
    let bot: Vec<Point3<T>> = profile.iter().map(|p| c(p, z0)).collect();
    let top: Vec<Point3<T>> = profile.iter().map(|p| c(p, z1)).collect();

    let mut body = Body::<T>::new();
    let seed = body.mvfs(bot[0]).unwrap();
    // Bottom rim chain v0 → v1 → … → v_{n-1}.
    let mut chain = Vec::new();
    chain.push(
        body.mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            bot[1],
            line(bot[0], bot[1]),
            Tol::witness(),
        )
        .unwrap(),
    );
    for i in 2..n {
        let at = chain[i - 2].he_minus;
        chain.push(
            body.mev(
                MevSite::Fan { he1: at, he2: at },
                bot[i],
                line(bot[i - 1], bot[i]),
                Tol::witness(),
            )
            .unwrap(),
        );
    }
    let bottom_vertices: Vec<_> = core::iter::once(seed.vertex)
        .chain(chain.iter().map(|m| m.vertex))
        .collect();
    // Close the bottom face: outward −z ⇒ CCW from below = reversed
    // profile order.
    let he_last = body
        .find_half_edge(seed.face, bottom_vertices[n - 1], bottom_vertices[n - 2])
        .unwrap();
    let rev: Vec<Point3<T>> = core::iter::once(bot[0])
        .chain(bot[1..].iter().rev().copied())
        .collect();
    let f_bottom = body
        .mef(
            MefSite::Chords {
                he1: he_last,
                he2: chain[0].he_plus,
            },
            line(bot[n - 1], bot[0]),
            FaceSurface::New(plane(&rev)),
            Tol::witness(),
        )
        .unwrap();
    // Struts up from each bottom vertex. The chain edge from v_i has
    // he_plus starting at v_i (i < n−1); the closing edge's he_plus
    // starts at v_{n-1}.
    let mut struts = Vec::new();
    for i in 0..n {
        let at = if i == 0 {
            chain[0].he_plus
        } else if i < n - 1 {
            chain[i].he_plus
        } else {
            f_bottom.he_plus
        };
        struts.push(
            body.mev(
                MevSite::Fan { he1: at, he2: at },
                top[i],
                line(bot[i], top[i]),
                Tol::witness(),
            )
            .unwrap(),
        );
    }
    // Side faces for segments 0..n−1; the last (n−1 → 0) closes against
    // the first side face's top edge.
    let mut side_faces = Vec::new();
    let mut first_side_he_plus = None;
    for i in 0..n {
        let j = (i + 1) % n;
        let he2 = if i < n - 1 {
            struts[j].he_minus
        } else {
            first_side_he_plus.unwrap()
        };
        let f = body
            .mef(
                MefSite::Chords {
                    he1: struts[i].he_minus,
                    he2,
                },
                line(top[i], top[j]),
                FaceSurface::New(plane(&[bot[i], bot[j], top[j], top[i]])),
                Tol::witness(),
            )
            .unwrap();
        if i == 0 {
            first_side_he_plus = Some(f.he_plus);
        }
        side_faces.push(f.face);
    }
    // The seed face survives as the top cap (outward +z ⇒ profile
    // order viewed from above).
    body.set_face_surface(seed.face, FaceSurface::New(plane(&top)))
        .unwrap();
    // Construction-final description step (D6): prisms are the M3
    // boolean/split operand factories — tier-3-grade by construction.
    describe_as_intersections(&mut body);

    Prism {
        body,
        bottom: bottom_vertices,
        top: struts.iter().map(|m| m.vertex).collect(),
        bottom_face: f_bottom.face,
        side_faces,
        top_face: seed.face,
    }
}

/// **Construction step** for hand-built planar fixtures (M3 PR 6a,
/// D6): describes every definitely-transverse edge as the
/// `Intersection` of its two adjacent faces' surfaces, witness at the
/// edge midpoint, carrier the straight chord — through the certified
/// `set_edge_curve` path. Called as the LAST construction step of a
/// fixture builder (both surfaces are known — certified-by-
/// construction), never applied to an op result: split and boolean
/// results carry honest descriptions natively (the retired
/// `upgrade_edges_to_intersections` review posture). Smooth edges
/// (coplanar neighbors — collinear profile runs) keep their
/// conventional chord, mirroring the pipeline's D2 split.
pub fn describe_as_intersections<T: geom_core::Decide>(body: &mut Body<T>) {
    let band = Band::linear(Tol::witness()).unwrap();
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
        let witness = p0.lerp(p1, T::from_f64(0.5));
        let (surf1, surf2) = (
            body.get_surface(s1).unwrap().clone(),
            body.get_surface(s2).unwrap().clone(),
        );
        match geom_brep::classify_dihedral(&surf1, &surf2, witness, p0.distance(p1), band).unwrap()
        {
            geom_brep::DihedralClass::Smooth => continue,
            geom_brep::DihedralClass::Transverse => {}
        }
        let mut spec = EdgeCurveSpec::line_between(p0, p1);
        spec.description = EdgeDescriptionSpec::Intersection { s1, s2, witness };
        body.set_edge_curve(edge_key, spec, Tol::witness()).unwrap();
    }
}

/// A cube built like `geometric_cube` but through an arbitrary
/// point transform (tilted operands are outside the prism builder).
pub fn mapped_cube(map: impl Fn(f64, f64, f64) -> Point3<f64>) -> Body<f64> {
    let mut body = Body::<f64>::new();
    cube_into(&mut body, map);
    body
}

/// [`mapped_cube`] into an EXISTING body (a second `mvfs` seeds a
/// second solid — the hand-built self-intersection control's door).
pub fn cube_into(body: &mut Body<f64>, map: impl Fn(f64, f64, f64) -> Point3<f64>) {
    let (a, b, cc, d) = (
        map(0.0, 0.0, 0.0),
        map(1.0, 0.0, 0.0),
        map(1.0, 1.0, 0.0),
        map(0.0, 1.0, 0.0),
    );
    let (a1, b1, c1, d1) = (
        map(0.0, 0.0, 1.0),
        map(1.0, 0.0, 1.0),
        map(1.0, 1.0, 1.0),
        map(0.0, 1.0, 1.0),
    );
    let seed = body.mvfs(a).unwrap();
    let e_ab = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            line(a, b),
            Tol::witness(),
        )
        .unwrap();
    let strut = |body: &mut Body<f64>, at, from, to| {
        body.mev(
            MevSite::Fan { he1: at, he2: at },
            to,
            line(from, to),
            Tol::witness(),
        )
        .unwrap()
    };
    let e_bc = strut(body, e_ab.he_minus, b, cc);
    let e_cd = strut(body, e_bc.he_minus, cc, d);
    let he_dc = body
        .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
        .unwrap();
    let f_bottom = body
        .mef(
            MefSite::Chords {
                he1: he_dc,
                he2: e_ab.he_plus,
            },
            line(d, a),
            FaceSurface::New(plane(&[a, d, cc, b])),
            Tol::witness(),
        )
        .unwrap();
    let e_aa = strut(body, e_ab.he_plus, a, a1);
    let e_bb = strut(body, e_bc.he_plus, b, b1);
    let e_cc = strut(body, e_cd.he_plus, cc, c1);
    let e_dd = strut(body, f_bottom.he_plus, d, d1);
    let f_front = body
        .mef(
            MefSite::Chords {
                he1: e_aa.he_minus,
                he2: e_bb.he_minus,
            },
            line(a1, b1),
            FaceSurface::New(plane(&[a, b, b1, a1])),
            Tol::witness(),
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e_bb.he_minus,
            he2: e_cc.he_minus,
        },
        line(b1, c1),
        FaceSurface::New(plane(&[b, cc, c1, b1])),
        Tol::witness(),
    )
    .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e_cc.he_minus,
            he2: e_dd.he_minus,
        },
        line(c1, d1),
        FaceSurface::New(plane(&[cc, d, d1, c1])),
        Tol::witness(),
    )
    .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e_dd.he_minus,
            he2: f_front.he_plus,
        },
        line(d1, a1),
        FaceSurface::New(plane(&[d, a, a1, d1])),
        Tol::witness(),
    )
    .unwrap();
    body.set_face_surface(seed.face, FaceSurface::New(plane(&[a1, b1, c1, d1])))
        .unwrap();
    describe_as_intersections(body);
}

/// Test-authoring convenience (M4 PR 5): the [`BooleanDeclarations`]
/// declaring EVERY geometrically-plausible cross-operand flush-plane
/// face pair of `(a, b)` — the test author's stand-in for a recipe
/// `Declare` (the author built the contact deliberately; this writes
/// the intent down). Selection is banded (in-band pairs are declared
/// too — the declared rung's verification re-checks each pair at its
/// meeting edges — exact fixtures decide definitely); coincidence
/// CERTIFICATION still happens inside the
/// op through the verified declared rung, never here.
pub fn flush_declarations<T: geom_core::Decide>(
    a: &Body<T>,
    b: &Body<T>,
) -> topo::BooleanDeclarations {
    use geom_core::k_stats::{decide, decide_flagged};
    use geom_core::{Margin, Sign};
    let band = Band::linear(Tol::witness()).unwrap();
    let planes = |body: &Body<T>| -> Vec<(topo::FaceKey, Point3<T>, geom_core::Vec3<T>)> {
        body.faces()
            .filter_map(|(k, f)| match body.get_surface(f.surface) {
                Some(&Surface::Plane { origin, normal, .. }) => Some((k, origin, normal)),
                _ => None,
            })
            .collect()
    };
    let mut decls = topo::BooleanDeclarations::none();
    for &(fa, oa, na) in &planes(a) {
        for &(fb, ob, nb) in &planes(b) {
            // Parallel? (in-band counts as plausible.)
            let par = na.cross(nb).norm();
            if matches!(
                decide_flagged(
                    "test_flush_parallel",
                    par,
                    band,
                    "test fixture: bare sine gate"
                ),
                Ok(Sign::Positive)
            ) {
                continue;
            }
            // Relative orientation, then the offset in that frame.
            let sigma = match decide_flagged(
                "test_flush_orient",
                na.dot(nb),
                band,
                "test fixture: bare cosine gate",
            ) {
                Ok(Sign::Positive) => T::one(),
                Ok(Sign::Negative) => -T::one(),
                _ => continue,
            };
            let da = na.dot(oa - Point3::origin());
            let db = nb.dot(ob - Point3::origin());
            if matches!(
                decide("test_flush_offset", Margin::of(da - sigma * db), band),
                Ok(Sign::Zero)
            ) {
                decls
                    .coincident_faces
                    .push(topo::FacePairDeclaration::rest(fa, fb));
            }
        }
    }
    decls
}
