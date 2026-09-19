//! **The Euler-op fixture family**: the geometric unit cube, the prism
//! builders, the straddle seat, and the two construction steps they
//! share. Generic over the scalar lane (`f64`, `Dual`, `Interval` —
//! every `Decide` scalar) wherever the builder is, with real certified
//! geometry at every step: a `Plane` on every face, a certified chord
//! line on every edge, and a body whose mass properties compute.
//!
//! # Which home this is, and the one it is not
//!
//! [`crate::test_support_impl`]'s docs state this crate's homes for
//! test vocabulary and the rule that routes an item — and a family —
//! between them. This module is the home for what a `tests/` binary
//! and an in-crate probe must BOTH be able to name, which is the one
//! thing neither `src/fixtures.rs` nor a module under `tests/` can
//! serve. Its consumers are this crate's `tests/` binaries, which
//! reach it as `crate::test_support`, and this crate's own in-crate
//! probes, which reach it by path. It is gated on the test arms alone
//! — nothing the library itself needs lives here, which is why it does
//! not carry `test_support_impl`'s `debug_assertions` arm.
//!
//! The family is here **whole**, which is that rule's family clause and
//! not its narrowest-home clause: `geometric_cube`,
//! `describe_as_intersections` and `face_surface_of_he` are what
//! `crate::cert_m3r1_probes` names from `src/`, and the builders,
//! bundles and assertions they share a vocabulary with travel with
//! them rather than being split across two homes.
//!
//! **It is not `crate::fixtures`, and the two are not two spellings of
//! one thing.** (Not linked: that module is `#[cfg(test)]` and does not
//! exist in a doc build.) That module builds bodies through the raw builder
//! with placeholder geometry — `NaN` NURBS surfaces, self-loop circle
//! carriers, index-derived collinear points, no mass properties at all
//! — for structural tests that never read a coordinate. This one
//! builds them through the Euler operators with the real geometry
//! above, for suites that do. A body from one is not a substitute for
//! a body from the other at any call site, and
//! `the_two_prism_families_build_different_bodies` below is the
//! assertion that says so in a form a rebinding would break.

// Test-support code: panicking is a test's failure mechanism (L5), and
// fixture unwraps are on keys the fixture itself just minted.
#![allow(clippy::unwrap_used, clippy::expect_used)]
#![allow(dead_code)] // key bundles expose every minted key; a consumer picks what it needs
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

use crate::{Body, FaceSurface, MefCreated, MefSite, MevCreated, MevSite, MvfsCreated};
use geom::Surface;
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, newell_plane};
use geom_core::Tol;
use geom_core::{Band, Point3, Real};

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
    errs: &[crate::ValidationError],
) {
    let edges: Vec<crate::EdgeKey> = body.edges().map(|(k, _)| k).collect();
    let named = |pick: fn(&crate::ValidationError) -> Option<crate::EdgeKey>| {
        errs.iter().filter_map(pick).collect::<Vec<_>>()
    };
    let scaffolds = named(|e| match e {
        crate::ValidationError::ScaffoldAtRest { edge } => Some(*edge),
        _ => None,
    });
    let transverse = named(|e| match e {
        crate::ValidationError::TransverseNotIntrinsic { edge } => Some(*edge),
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
pub struct GeoCube<T: Real> {
    pub body: Body<T>,
    pub seed: MvfsCreated,
    /// [`PrismOps::chain`] then [`PrismOps::struts`], in that order:
    /// `[0..3]` are the bottom rim edges v0→v1, v1→v2, v2→v3 and
    /// `[3..7]` the four struts up from v0, v1, v2, v3.
    pub mevs: [MevCreated; 7],
    /// **The order is the interface.** [`PrismOps::bottom`] then
    /// [`PrismOps::sides`]: `[0]` is the `z = 0` bottom cap and
    /// `[1..5]` are the walls over [`UNIT_SQUARE`]'s segments 0 to 3,
    /// so `[1]` is `y = 0`, `[2]` is `x = 1`, `[3]` is `y = 1` and
    /// `[4]` is `x = 0`. The top cap is the seed face and is in
    /// neither array. Consumers index this — `cert_m3r1_probes` takes
    /// `[1]` as the front wall, `null` takes `[0]` and `[1]` as a face
    /// pair — so a reordering that preserved the lengths would move
    /// them onto other faces silently.
    pub mefs: [MefCreated; 5],
}

/// The chord-line spec between two points, with the extrude-flavored
/// pushforward description (the sweep's own form for side struts).
pub fn line<T: Real>(p0: Point3<T>, p1: Point3<T>) -> EdgeCurveSpec<T> {
    EdgeCurveSpec::line_between(p0, p1)
}

/// A Newell-certified plane from an outward-CCW-ordered corner list.
pub fn plane<T: geom_core::Decide>(corners: &[Point3<T>], tol: Tol) -> Surface<T> {
    newell_plane(corners, Band::linear(tol).unwrap()).unwrap()
}

/// The operator keys [`prism_ops`] mints, in construction order.
pub struct PrismOps {
    /// The `mvfs` that seeds the solid at the profile's first corner.
    pub seed: MvfsCreated,
    /// The bottom rim's chain, `n - 1` of them: corner 0 → 1, … ,
    /// `n - 2` → `n - 1`. The rim's closing chord belongs to
    /// [`PrismOps::bottom`], which mints it.
    pub chain: Vec<MevCreated>,
    /// One strut per profile corner, bottom rim → top rim, in profile
    /// order.
    pub struts: Vec<MevCreated>,
    /// The bottom cap.
    pub bottom: MefCreated,
    /// One side face per profile segment `i → i+1` (cyclic).
    pub sides: Vec<MefCreated>,
}

/// **The one Euler sequence every box and prism in this file is built
/// by**: a right prism over the simple polygon `profile` (x, y corners,
/// no repeats, reflex corners welcome) spanning `z`, into `body`, with
/// every corner placed through `map`.
///
/// **The winding rule is about `profile` and `map` together.** A
/// counterclockwise-from-+z profile under an orientation-PRESERVING map
/// and a clockwise one under an orientation-REVERSING map both build an
/// outward-facing prism; the two mixed combinations build an inside-out
/// one. Both supported combinations are in use — `review_m3_pr55`'s
/// reflected placements pass reversed profiles on purpose — so "the
/// profile must be counterclockwise", which this doc said until the
/// builders were unified, was never the rule the callers obeyed.
///
/// It is the §9.4.2-minimal sequence with real geometry at every step —
/// every `mef` supplies its face's Newell plane, every edge a certified
/// chord-line carrier; the seed face (which survives as the top cap)
/// gets its plane via `set_face_surface` at the end (the documented
/// seed-face path).
///
/// The two axes the callers differ on are both parameters here, and
/// that is the whole of the difference between them:
///
/// - **`map`** is where the tilted operands live. `Point3::new` at
///   `T::from_f64` gives the untransformed prism; anything else — a
///   scale, an affine, a shear — is the same body pushed through it.
/// - **The description step is the CALLER's.** This stops before
///   [`describe_as_intersections`]: run it and every transverse edge
///   trades its conventional chord for the `Intersection` its two faces
///   determine; skip it and all `3n` stay
///   `Scaffold(ExtrudedPoint …)`/`Declared`, which is the state
///   [`assert_every_chord_named_by_both_rules`] is about. So the choice
///   belongs where a reader can see it, not inside a shared body — and
///   it is a line at each call site rather than a `bool` argument,
///   because the risk here is a reader not noticing that step.
///
/// A second call on the same `body` seeds a second solid, so the
/// caller also chooses whether the body is fresh.
///
/// **Only the corner count is checked, and the rest are not
/// preconditions at all** — which is worth stating, because the list
/// above reads like four and is one.
///
/// - The **winding rule is not enforced and must not be**: building an
///   inside-out prism on purpose is a fixture this tree needs.
///   `review_m2_pr7` mirrors a cube through [`mapped_cube`] precisely to
///   assert that tiers 1 and 2 CANNOT see the orientation and that
///   `mass_properties` can. A refusal here would delete that suite's
///   subject. (Measured: a fail-loud winding assertion in this function
///   reds four of its rows and one of `review_m3_pr55`'s.)
/// - **Simplicity and no-repeats are not checked either**: a
///   self-intersecting or repeating profile is undefined behaviour of
///   this builder, and a caller that wants either refused owes the
///   check itself.
///
/// Where the winding rule IS enforced is on the fixtures that claim to
/// be outward-facing, in `tests/cube_doors_agree.rs` — which asserts
/// each face's outward normal against the corners and so reads the
/// composite rule rather than the profile alone.
pub fn prism_ops<T: geom_core::Decide>(
    body: &mut Body<T>,
    profile: &[(f64, f64)],
    z: (f64, f64),
    map: impl Fn(f64, f64, f64) -> Point3<T>,
    tol: Tol,
) -> PrismOps {
    assert!(profile.len() >= 3, "a prism needs at least three corners");
    let n = profile.len();
    let bot: Vec<Point3<T>> = profile.iter().map(|&(x, y)| map(x, y, z.0)).collect();
    let top: Vec<Point3<T>> = profile.iter().map(|&(x, y)| map(x, y, z.1)).collect();

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
            tol,
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
                tol,
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
    let bottom = body
        .mef(
            MefSite::Chords {
                he1: he_last,
                he2: chain[0].he_plus,
            },
            line(bot[n - 1], bot[0]),
            FaceSurface::New(plane(&rev, tol)),
            tol,
        )
        .unwrap();
    // Struts up from each bottom vertex. The chain edge from v_i has
    // he_plus starting at v_i for every i < n−1, `chain[0]` included;
    // the last vertex is reached instead by the closing edge the bottom
    // `mef` minted, whose he_plus starts at v_{n-1}.
    let mut struts = Vec::new();
    for i in 0..n {
        let at = if i < n - 1 {
            chain[i].he_plus
        } else {
            bottom.he_plus
        };
        struts.push(
            body.mev(
                MevSite::Fan { he1: at, he2: at },
                top[i],
                line(bot[i], top[i]),
                tol,
            )
            .unwrap(),
        );
    }
    // Side faces for segments 0..n−1; the last (n−1 → 0) closes against
    // the first side face's top edge. Outward-CCW corner orders
    // (interior-left rule).
    let mut sides = Vec::new();
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
                FaceSurface::New(plane(&[bot[i], bot[j], top[j], top[i]], tol)),
                tol,
            )
            .unwrap();
        if i == 0 {
            first_side_he_plus = Some(f.he_plus);
        }
        sides.push(f);
    }
    // The seed face survives as the top cap (outward +z ⇒ profile
    // order viewed from above).
    body.set_face_surface(seed.face, FaceSurface::New(plane(&top, tol)))
        .unwrap();

    PrismOps {
        seed,
        chain,
        struts,
        bottom,
        sides,
    }
}

/// The unit square, counterclockwise viewed from +z — the profile the
/// cube doors spell, and the one at which [`prism_ops`] is the cube
/// sequence.
const UNIT_SQUARE: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

/// The geometric unit cube with its key bundle, at any `Decide` scalar:
/// [`prism_ops`] over [`UNIT_SQUARE`] at the identity map into a fresh
/// body, and **no** description step. So every chord is still at the
/// scaffolding door, named by both at-rest rules — the state this
/// fixture's suites measure, and the one thing that distinguishes it
/// from every other box builder in this file.
pub fn geometric_cube<T: geom_core::Decide>(tol: Tol) -> GeoCube<T> {
    let mut body = Body::<T>::new();
    let ops = prism_ops(
        &mut body,
        &UNIT_SQUARE,
        (0.0, 1.0),
        |x, y, z| Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z)),
        tol,
    );
    // The bundle's arrays are the N-general vectors at N = 4: the rim
    // chain then the struts, the bottom cap then the sides.
    let mevs: Vec<MevCreated> = ops.chain.into_iter().chain(ops.struts).collect();
    let mefs: Vec<MefCreated> = core::iter::once(ops.bottom).chain(ops.sides).collect();
    GeoCube {
        body,
        seed: ops.seed,
        // Infallible: `UNIT_SQUARE` fixes n = 4, so `prism_ops` returns
        // n − 1 = 3 chain plus n = 4 struts and 1 bottom plus n = 4
        // sides. The lengths are decided by a const above, not by any
        // runtime value, which is why these read as conversions rather
        // than as checks.
        mevs: mevs.try_into().expect("7 mevs at a four-corner profile"),
        mefs: mefs.try_into().expect("5 mefs at a four-corner profile"),
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
    pub post_top: crate::FaceKey,
    /// The cap's `x = 0.30` side face (the perpendicular-pair rows).
    pub post_side_x030: crate::FaceKey,
    /// The shelf's underside (the resting pair's shelf side).
    pub shelf_bottom: crate::FaceKey,
    /// The shelf's `y = 0.30` side face (the perpendicular-pair rows).
    pub shelf_side_y030: crate::FaceKey,
}

/// Builds [`StraddleSeat`] (post grafted first, shelf second — the
/// arena order the fence rows' pinned keys and witnesses assume).
pub fn straddle_seat(tol: Tol) -> StraddleSeat {
    let post: Prism<f64> = prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
        0.0,
        0.5,
        tol,
    );
    let shelf: Prism<f64> = prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
        tol,
    );
    // side_faces[i] spans profile segment i → i+1: the post's [3] is
    // (0.30, 0.42) → (0.30, 0.20), the plane x = 0.30; the shelf's
    // [2] is (0.9, 0.30) → (0, 0.30), the plane y = 0.30.
    let post_side_x030 = post.side_faces[3];
    let mut body = post.body;
    let keys =
        crate::graft_disjoint_all_keyed(&mut body, &shelf.body, tol).expect("the straddle graft");
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
    pub bottom: Vec<crate::VertexKey>,
    /// Top-rim vertices, one per profile corner (same order).
    pub top: Vec<crate::VertexKey>,
    pub bottom_face: crate::FaceKey,
    /// One side face per profile segment `i → i+1` (cyclic).
    pub side_faces: Vec<crate::FaceKey>,
    pub top_face: crate::FaceKey,
}

/// Builds a right prism over a simple polygon `profile` (x, y corners,
/// **counterclockwise viewed from +z**, no repeats, reflex corners
/// welcome), extruded from z = 0 to z = `height`: [`prism_ops`]
/// untransformed, then described. Every face gets its outward-CCW
/// Newell plane, every edge a certified chord line.
pub fn prism<T: geom_core::Decide>(profile: &[(f64, f64)], height: f64, tol: Tol) -> Prism<T> {
    prism_z(profile, 0.0, height, tol)
}

/// [`prism`] with an explicit z-range `[z0, z1]` (M3 PR 4: bricks at
/// arbitrary heights for the boolean fixtures).
pub fn prism_z<T: geom_core::Decide>(
    profile: &[(f64, f64)],
    z0: f64,
    z1: f64,
    tol: Tol,
) -> Prism<T> {
    let mut body = Body::<T>::new();
    let ops = prism_ops(
        &mut body,
        profile,
        (z0, z1),
        |x, y, z| Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z)),
        tol,
    );
    // Construction-final description step (D6): prisms are the M3
    // boolean/split operand factories — tier-3-grade by construction.
    describe_as_intersections(&mut body, tol);

    Prism {
        bottom: core::iter::once(ops.seed.vertex)
            .chain(ops.chain.iter().map(|m| m.vertex))
            .collect(),
        top: ops.struts.iter().map(|m| m.vertex).collect(),
        bottom_face: ops.bottom.face,
        side_faces: ops.sides.iter().map(|f| f.face).collect(),
        top_face: ops.seed.face,
        body,
    }
}

/// The axis-aligned box `[x.0, x.1] x [y.0, y.1] x [z.0, z.1]` — the
/// rectangular case of [`prism_z`], body only; a caller that needs the
/// keys calls `prism_z` and keeps its [`Prism`].
///
/// [`geometric_cube`] is the one box-or-cube door in this file that
/// builds a different body: it stops before
/// [`describe_as_intersections`] and so keeps the conventional chords
/// its rows assert on. Every other one — `brick`, [`prism`],
/// [`prism_z`], [`mapped_cube`], [`cube_into`] — is [`prism_ops`] and
/// then that step, so they agree arena for arena wherever their domains
/// meet, an axis-aligned box, and they differ only in what they vary:
/// an extent on one side, a point map on the other with tilts included.
/// Every one of them is generic in the `Decide` scalar.
///
/// **Both halves of that are pinned by `tests/cube_doors_agree.rs`**,
/// which is where to look before trusting either. A shared core is what
/// makes the agreement half cheap and the OTHER half load-bearing:
/// doors that are one function agree by construction, so what is worth
/// pinning is that the one function still builds the prism its inputs
/// name, and that one door still stops short of the description step.
/// That file's independent row is the first claim and its negative row
/// the second.
pub fn brick<T: geom_core::Decide>(
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
    tol: Tol,
) -> Body<T> {
    prism_z::<T>(
        &[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)],
        z.0,
        z.1,
        tol,
    )
    .body
}

/// The surface carried by the face `he` bounds — the one step both
/// [`describe_as_intersections`] and every caller that has to name an
/// edge's two adjacent surfaces walks: half-edge to its loop, loop to
/// its face, face to its surface key.
pub fn face_surface_of_he<T: Real>(
    body: &Body<T>,
    he: crate::entity::HalfEdgeKey,
) -> crate::geometry::SurfaceKey {
    let he_data = body.get_half_edge(he).unwrap();
    let loop_data = body.get_loop(he_data.parent_loop).unwrap();
    body.get_face(loop_data.face).unwrap().surface
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
pub fn describe_as_intersections<T: geom_core::Decide>(body: &mut Body<T>, tol: Tol) {
    let band = Band::linear(tol).unwrap();
    let edges: Vec<_> = body.edges().map(|(k, e)| (k, e.clone())).collect();
    for (edge_key, edge) in edges {
        let s1 = face_surface_of_he(body, edge.he_plus);
        let s2 = face_surface_of_he(body, edge.he_minus);
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
        body.set_edge_curve(edge_key, spec, tol).unwrap();
    }
}

/// A cube built like [`geometric_cube`] but through an arbitrary point
/// transform — and **with** the description step, so its edges carry
/// `Intersection`/`Derived` where `geometric_cube`'s carry
/// `Scaffold(ExtrudedPoint …)`/`Declared`.
pub fn mapped_cube<T: geom_core::Decide>(
    map: impl Fn(f64, f64, f64) -> Point3<T>,
    tol: Tol,
) -> Body<T> {
    let mut body = Body::<T>::new();
    cube_into(&mut body, map, tol);
    body
}

/// [`mapped_cube`] into an EXISTING body (a second `mvfs` seeds a
/// second solid — the hand-built self-intersection control's door).
pub fn cube_into<T: geom_core::Decide>(
    body: &mut Body<T>,
    map: impl Fn(f64, f64, f64) -> Point3<T>,
    tol: Tol,
) {
    prism_ops(body, &UNIT_SQUARE, (0.0, 1.0), map, tol);
    // Construction-final description step (D6) — the whole of what
    // this door does that [`geometric_cube`] does not.
    describe_as_intersections(body, tol);
}

/// Test-authoring convenience: the [`crate::BooleanDeclarations`] declaring
/// every flush face pair of `(a, b)`, on any carrier the `Rest`
/// ladder verifies — the test author's
/// stand-in for a recipe `Declare` (the author built the contact
/// deliberately; this writes the intent down).
///
/// The detection is the library's ([`crate::flush`]), so this helper
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
    tol: Tol,
) -> crate::BooleanDeclarations {
    let found = crate::flush::find_flush_candidates(a, b, tol)
        .expect("a fixture's flush pairs decide definitely");
    crate::flush::declare_all(&found)
}

#[cfg(test)]
mod tests {
    use super::*;
    use geom_core::Tol;

    /// **The two `prism`s in this crate build different artifacts, and
    /// the difference is geometric.** `crate::fixtures::raw_prism`
    /// and this module's [`prism`] take near-identical arguments and
    /// agree on every arena LENGTH, so a call site rebound from one to
    /// the other still compiles and still passes any count assertion.
    /// What separates them is what the arenas hold: a certified solid
    /// against a structural skeleton with placeholder geometry.
    ///
    /// Asserted here rather than left to the names, because the names
    /// are what nearly collided: three readings of the same surface —
    /// a name, a signature and a neighbourhood — do not distinguish
    /// two builders, and a body's surfaces, carriers and volume do.
    #[test]
    fn the_two_prism_families_build_different_bodies() {
        let tol = Tol::witness();
        let euler = prism::<f64>(&UNIT_SQUARE, 1.0, tol);
        let raw = crate::fixtures::raw_prism(4, tol);

        // The lengths agree, which is exactly why the rest of this test
        // exists: a counts-only check cannot tell the two apart.
        assert_eq!(
            crate::test_support::arena_counts(&euler.body),
            crate::test_support::arena_counts(&raw.body),
            "the two families agree on every arena length"
        );

        // Surfaces: six certified Newell planes against six NURBS
        // placeholders.
        assert_eq!(euler.body.surfaces().count(), 6);
        assert!(
            euler
                .body
                .surfaces()
                .all(|(_, s)| matches!(s, geom::Surface::Plane { .. })),
            "the Euler-op family carries a real plane on every face"
        );
        // The count is asserted on this side too: `ArenaCounts` covers
        // the seven TOPOLOGY arenas, so nothing above pins the surface
        // arena, and `all` over an empty one is vacuously true.
        assert_eq!(raw.body.surfaces().count(), 6);
        assert!(
            raw.body
                .surfaces()
                .all(|(_, s)| !matches!(s, geom::Surface::Plane { .. })),
            "the raw family carries no plane at all"
        );

        // Carriers: every edge described as the intersection its two
        // faces determine, against every edge still at the scaffolding
        // door.
        let described = |body: &Body<f64>| {
            body.edges()
                .filter(|(_, e)| {
                    body.get_curve_geom(e.curve)
                        .and_then(crate::CurveGeom::certified)
                        .is_some_and(|c| {
                            matches!(
                                c.description(),
                                geom_brep::EdgeDescription::Intersection { .. }
                            )
                        })
                })
                .count()
        };
        assert_eq!(described(&euler.body), 12, "twelve intrinsic carriers");
        assert_eq!(described(&raw.body), 0, "no intrinsic carrier at all");

        // Mass properties: the unit box's volume against a refusal.
        assert_eq!(
            crate::mass_properties(&euler.body, tol)
                .expect("the Euler-op prism has mass properties")
                .volume,
            1.0
        );
        assert!(
            crate::mass_properties(&raw.body, tol).is_err(),
            "the raw prism's placeholder geometry has no mass properties"
        );
    }
}
