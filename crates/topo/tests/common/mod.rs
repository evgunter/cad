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
#![allow(dead_code)] // one instance per binary; no single consumer uses all of it
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

use geom::Surface;
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, newell_plane};
use geom_core::Tol;
use geom_core::{Band, Point3, Real};
use topo::{Body, FaceSurface, MefCreated, MefSite, MevCreated, MevSite, MvfsCreated};

/// **The two independent at-rest rules a conventional chord breaks**,
/// asserted as a pair over exactly this body's edges — and nothing
/// else reported.
///
/// A body assembled from Euler ops and then grafted with planes holds
/// every chord at the SCAFFOLDING door: `mev_line` and `mef_chord`
/// mint an edge before any face surface exists, so they have no chart
/// to name. At rest that breaks two rules at once, and the two are
/// independent, not one report doubled:
///
/// - **Prefer-intrinsic** (D2): a definitely-transverse edge whose
///   locus the modeler DECLARED must instead cite the intersection its
///   two surfaces determine. It fires on a chart image too — a
///   declared image is still a declared locus — so it is not about the
///   scaffolding door.
/// - **The transience fence** (U2 Q2): an edge with two faces has a
///   chart, so a scaffold there is a construction that stopped
///   half-way. It fires regardless of dihedral class — a SMOOTH join,
///   which prefer-intrinsic exempts, is named by it just the same.
///
/// So a conventional chord at rest is named once by each, and the
/// pairing is what this helper pins: one report per rule per edge,
/// over exactly the edge arena, in its order, with no third kind and
/// no cascade. **This is what the pre-P-1b rows' single count meant**;
/// it is asserted as a bijection rather than re-baselined to twice the
/// number, so that a rule firing twice on one edge, or missing one,
/// still fails here.
pub fn assert_every_chord_named_by_both_rules<T: Real>(
    body: &Body<T>,
    errs: &[topo::ValidationError],
) {
    let edges: Vec<topo::EdgeKey> = body.edges().map(|(k, _)| k).collect();
    let named = |pick: fn(&topo::ValidationError) -> Option<topo::EdgeKey>| {
        errs.iter().filter_map(pick).collect::<Vec<_>>()
    };
    let scaffolds = named(|e| match e {
        topo::ValidationError::ScaffoldAtRest { edge } => Some(*edge),
        _ => None,
    });
    let transverse = named(|e| match e {
        topo::ValidationError::TransverseNotIntrinsic { edge } => Some(*edge),
        _ => None,
    });
    assert_eq!(
        scaffolds, edges,
        "the fence names every chord still at the scaffolding door, once: {errs:?}"
    );
    assert_eq!(
        transverse, edges,
        "prefer-intrinsic names every declared transverse chord, once: {errs:?}"
    );
    assert_eq!(
        errs.len(),
        2 * edges.len(),
        "and nothing else is reported: {errs:?}"
    );
}

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

/// **The straddle seat** — issue 973 part (b)'s configuration,
/// verbatim: a rectangular cap `[0.30, 0.60] x [0.20, 0.42]` (z 0 to
/// 0.5) under a shelf `[0, 0.9] x [0, 0.30]` (z 0.5 to 0.54), contact
/// plane `z = 0.5`, the cap straddling the shelf's `y = 0.30`
/// boundary edge so the two cap side edges cross it properly at
/// `(0.30, 0.30, 0.5)` and `(0.60, 0.30, 0.5)`.
///
/// ONE builder, shared: `mate4a_ef_bound_rung` (the re-blessed (b)
/// fence and its bare byte-pin) and `mate9_crossing_rung` (the
/// crossing rung's rows) assert COMPLEMENTARY things about this same
/// seat, and two hand-copies would let a drift silently decouple
/// them.
pub struct StraddleSeat {
    pub body: Body<f64>,
    /// The cap's top face (the resting pair's post side).
    pub post_top: topo::FaceKey,
    /// The cap's `x = 0.30` side face (the perpendicular-pair rows).
    pub post_side_x030: topo::FaceKey,
    /// The shelf's underside (the resting pair's shelf side).
    pub shelf_bottom: topo::FaceKey,
    /// The shelf's `y = 0.30` side face (the perpendicular-pair rows).
    pub shelf_side_y030: topo::FaceKey,
}

/// Builds [`StraddleSeat`] (post grafted first, shelf second — the
/// arena order the fence rows' pinned keys and witnesses assume).
pub fn straddle_seat() -> StraddleSeat {
    let post: Prism<f64> = prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
        0.0,
        0.5,
    );
    let shelf: Prism<f64> = prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
    );
    // side_faces[i] spans profile segment i → i+1: the post's [3] is
    // (0.30, 0.42) → (0.30, 0.20), the plane x = 0.30; the shelf's
    // [2] is (0.9, 0.30) → (0, 0.30), the plane y = 0.30.
    let post_side_x030 = post.side_faces[3];
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, geom_core::Tol::witness())
        .expect("the straddle graft");
    StraddleSeat {
        post_top: post.top_face,
        post_side_x030,
        shelf_bottom: keys.face(shelf.bottom_face).expect("shelf bottom maps"),
        shelf_side_y030: keys.face(shelf.side_faces[2]).expect("shelf side maps"),
        body,
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

/// Test-authoring convenience: the [`BooleanDeclarations`] declaring
/// every flush-plane face pair of `(a, b)` — the test author's
/// stand-in for a recipe `Declare` (the author built the contact
/// deliberately; this writes the intent down).
///
/// The detection is the library's ([`topo::flush`]), so this helper
/// interprets nothing: it detects through the same door the op then
/// verifies with, and hands the findings straight to the declare
/// sugar. Coincidence CERTIFICATION still happens inside the op
/// through the verified declared rung, never here.
///
/// **The in-band arm INVERTED here, deliberately.** The hand declarer
/// this replaced treated an in-band pair as plausible and declared it
/// anyway, leaving the op's declared rung to re-check it. A finding is
/// only ever DEFINITE, so the library refuses instead — and this
/// helper turns that refusal into a panic rather than swallowing it,
/// which makes "every fixture that reaches this helper decides
/// definitely" a fixture assertion instead of an assumption. A future
/// fixture built inside the band fails loudly at its own door; the old
/// helper would have declared it and moved on.
pub fn flush_declarations<T: geom_core::Decide>(
    a: &Body<T>,
    b: &Body<T>,
) -> topo::BooleanDeclarations {
    let found = topo::flush::find_flush_candidates(a, b, Tol::witness())
        .expect("a fixture's flush pairs decide definitely");
    topo::flush::declare_all(&found)
}
