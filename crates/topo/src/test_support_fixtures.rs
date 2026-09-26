//! **The Euler-op fixture family**: the unit cube, the prism builders,
//! the straddle seat, the §9.3 ring and hole surgery, the
//! cylinder-wall sheet, and the construction steps they share. Generic
//! over the scalar lane (`f64`, `Dual`, `Interval` — every `Decide`
//! scalar) wherever the builder is.
//!
//! **Two families, and what separates them is the boundary they can
//! describe.**
//!
//! The POLYHEDRAL family — [`prism_ops`] and everything grown from it,
//! [`geometric_cube`] and [`straddle_seat`] — puts a certified chord
//! line on every edge, and face geometry is the one axis its doors
//! differ on, as a parameter ([`FaceGeometry`]): the certified doors
//! put a `Plane` on every face and build a body whose mass properties
//! compute, while [`declined_cube`] leaves all `n + 2` faces on the one
//! `Surface::nurbs_placeholder` the seed `mvfs` minted, because the
//! suites it serves read that shared key.
//!
//! [`cyl_wall_sheet`] is the other family, and neither sentence is true
//! of it: its rims are `Curve3::Circle` carriers described as the
//! cylinder cut by a plane, and its faces are on the cylinder its
//! [`CylFrame`] names, so there is no `FaceGeometry` to choose. It has
//! an axis of its own instead, [`CylKey`], and it is not about a
//! face's geometry but about WHICH face carries the cylinder key. Only
//! its two meridian struts are chords. A suite wanting a curved chart
//! comes here; a suite wanting a polyhedron does not.
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
//! exist in a doc build.) Its RAW family is built through the raw
//! builder — `NaN` NURBS surfaces, self-loop circle carriers,
//! index-derived collinear points standing in for positions nobody
//! reads — for structural tests that never read a coordinate. Every
//! door here builds through the Euler operators instead, with real
//! points and a certified chord on every edge whichever
//! [`FaceGeometry`] it is handed. **A declined face surface does not
//! narrow that difference**: [`declined_cube`]'s corners are the unit
//! cube's corners and its edges are the chords between them, which is
//! what a raw-family body is not, and `crate::fixtures`' own
//! operator-built family grows from this door rather than from a
//! sequence of its own. A raw-family body is not a substitute for one
//! of these at any call site, and
//! `the_two_prism_families_build_different_bodies` below is the
//! assertion that says so in a form a rebinding would break.

// Test-support code: panicking is a test's failure mechanism (L5), and
// fixture unwraps are on keys the fixture itself just minted.
#![allow(clippy::unwrap_used, clippy::expect_used)]
#![allow(dead_code)] // key bundles expose every minted key; a consumer picks what it needs
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

use crate::{
    Body, FaceKey, FaceSurface, HalfEdgeKey, KemrResult, KfmrhResult, LoopBoundary, MefCreated,
    MefSite, MevCreated, MevSite, MvfsCreated,
};
use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, newell_plane};
use geom_core::Tol;
use geom_core::{Band, Point3, Real, Vec3};

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

/// The unit cube's [`PrismOps`] at `n = 4` — the same keys in the same
/// construction order, flattened into fixed-length arrays — together
/// with the body they were minted into. Both unit-cube doors return it:
/// [`geometric_cube`] and [`declined_cube`] differ in what their faces
/// carry, never in which keys exist or in what order.
pub struct CubeOps<T: Real> {
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

/// Whether [`prism_ops`] certifies its faces or declines face geometry.
///
/// **The declined arm is a subject, not a shortcut.** A prism grown
/// with `Declined` is the §9.4.2 sequence written through `mef`'s
/// [`FaceSurface::Inherit`] arm — exactly what `mef_chord` does — so
/// every face it mints, and the seed face it leaves alone, ends on the
/// ONE `Surface::nurbs_placeholder` the opening `mvfs` minted. The
/// suites built on it are operator-count, atomicity, revert and
/// coplanar-merge rows, and they are not merely geometry-indifferent:
/// several READ that single shared key, and `merge_faces`' coincidence
/// rows are about six faces sitting on it. Handing them real planes
/// would change their subject, which is why this is a parameter rather
/// than a defect to be folded away.
///
/// The edge carriers do NOT vary with it: both arms mint the certified
/// chord line between the two endpoints, which is what `mev_line` and
/// `mef_chord` compute too.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FaceGeometry {
    /// Every `mef` supplies its face's outward-CCW Newell plane, and
    /// the seed face — which survives as the top cap — gets its own
    /// through `set_face_surface` at the end (the documented seed-face
    /// path). `n + 2` surface keys for an `n`-corner profile.
    Certified,
    /// Every `mef` inherits, and the seed face is left as `mvfs` made
    /// it: one placeholder surface key for the whole prism.
    Declined,
}

impl FaceGeometry {
    /// The [`FaceSurface`] this arm hands `mef` for a face whose
    /// outward-CCW corners are `corners`.
    fn of<T: geom_core::Decide>(self, corners: &[Point3<T>], tol: Tol) -> FaceSurface<T> {
        match self {
            // The plane is computed only on this arm: `Declined` is
            // for fixtures that decline geometry, and a profile whose
            // Newell plane does not certify is theirs to build.
            Self::Certified => FaceSurface::New(plane(corners, tol)),
            Self::Declined => FaceSurface::Inherit,
        }
    }
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
/// It is the §9.4.2-minimal sequence: every edge a certified chord-line
/// carrier, and every face's surface whatever `faces` says.
///
/// The three axes the callers differ on are all parameters here, and
/// that is the whole of the difference between them:
///
/// - **`map`** is where the tilted operands live. `Point3::new` at
///   `T::from_f64` gives the untransformed prism; anything else — a
///   scale, an affine, a shear — is the same body pushed through it.
///   The 2x box and the unit cube are one call apart.
/// - **`faces`** is [`FaceGeometry`]: real Newell planes, or face
///   geometry declined. Declining is a SUBJECT, not a shortcut; that
///   enum's doc is where the reason is written.
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
    faces: FaceGeometry,
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
            faces.of(&rev, tol),
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
                faces.of(&[bot[i], bot[j], top[j], top[i]], tol),
                tol,
            )
            .unwrap();
        if i == 0 {
            first_side_he_plus = Some(f.he_plus);
        }
        sides.push(f);
    }
    // The seed face survives as the top cap (outward +z ⇒ profile
    // order viewed from above). Under `Declined` it keeps the `mvfs`
    // placeholder every other face inherited, so the whole prism sits on
    // one surface key.
    if faces == FaceGeometry::Certified {
        body.set_face_surface(seed.face, FaceSurface::New(plane(&top, tol)))
            .unwrap();
    }

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
pub const UNIT_SQUARE: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

/// The geometric unit cube with its key bundle, at any `Decide` scalar:
/// [`prism_ops`] over [`UNIT_SQUARE`] at the identity map into a fresh
/// body, and **no** description step. So every chord is still at the
/// scaffolding door, named by both at-rest rules — the state this
/// fixture's suites measure, and the one thing that distinguishes it
/// from every other box builder in this file.
pub fn geometric_cube<T: geom_core::Decide>(tol: Tol) -> CubeOps<T> {
    unit_cube(FaceGeometry::Certified, tol)
}

/// [`geometric_cube`] with its face geometry **declined**
/// ([`FaceGeometry::Declined`], whose doc is where the reason that arm
/// exists is written): the same operators, the same chord carriers, the
/// same keys in the same order, and all six faces on the one `mvfs`
/// placeholder surface.
///
/// It is a separate door rather than a `bool` at the call site for the
/// same reason the description step is: a reader must be able to see
/// which body a suite took.
pub fn declined_cube<T: geom_core::Decide>(tol: Tol) -> CubeOps<T> {
    unit_cube(FaceGeometry::Declined, tol)
}

/// The shared body of [`geometric_cube`] and [`declined_cube`]: the
/// cube sequence at [`UNIT_SQUARE`], untransformed, bundled.
fn unit_cube<T: geom_core::Decide>(faces: FaceGeometry, tol: Tol) -> CubeOps<T> {
    let mut body = Body::<T>::new();
    let ops = prism_ops(
        &mut body,
        &UNIT_SQUARE,
        (0.0, 1.0),
        |x, y, z| Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z)),
        faces,
        tol,
    );
    // The bundle's arrays are the N-general vectors at N = 4: the rim
    // chain then the struts, the bottom cap then the sides.
    let mevs: Vec<MevCreated> = ops.chain.into_iter().chain(ops.struts).collect();
    let mefs: Vec<MefCreated> = core::iter::once(ops.bottom).chain(ops.sides).collect();
    CubeOps {
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
        FaceGeometry::Certified,
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
/// **Two of the box-or-cube doors here build a different body**, and
/// both stop before [`describe_as_intersections`] so as to keep the
/// conventional chords their rows assert on: [`geometric_cube`], and
/// [`declined_cube`], which declines its face geometry on top of that.
/// Every other one — `brick`, [`prism`], [`prism_z`], [`mapped_cube`],
/// [`cube_into`] — is [`prism_ops`] and then that step, so they agree
/// arena for arena wherever their domains meet, an axis-aligned box,
/// and they differ only in what they vary: an extent on one side, a
/// point map on the other with tilts included. Every one of them is
/// generic in the `Decide` scalar.
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
/// edge's two adjacent surfaces walks: [`Body::face_of_half_edge`] and
/// then that face's surface key.
pub fn face_surface_of_he<T: Real>(
    body: &Body<T>,
    he: crate::entity::HalfEdgeKey,
) -> crate::geometry::SurfaceKey {
    let face = body.face_of_half_edge(he).unwrap();
    body.get_face(face).unwrap().surface
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
    prism_ops(
        body,
        &UNIT_SQUARE,
        (0.0, 1.0),
        map,
        FaceGeometry::Certified,
        tol,
    );
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

// ---------------------------------------------------------------------
// The §9.3 surgery: a ring face planted in a face, and a hole drilled
// through a body
// ---------------------------------------------------------------------

/// The operator keys [`plant_ring_face`] mints, in construction order.
pub struct RingFaceOps {
    /// The strut from `at`'s start vertex to the rim's first corner.
    /// [`RingFaceOps::kill`] kills its edge; its vertex survives as
    /// that corner.
    pub strut: MevCreated,
    /// The `kemr` that strands the rim's first corner as an empty ring
    /// of the host face.
    pub kill: KemrResult,
    /// The rim chain grown inside the ring, one per corner after the
    /// first: corner 0 → 1, … , `n - 2` → `n - 1`.
    pub rim: Vec<MevCreated>,
    /// The `mef` that closes the rim. Its face is the new face covering
    /// the ring, and the host keeps the ring as the rim's other side.
    pub membrane: MefCreated,
}

/// **Mäntylä §9.3 steps (f)–(i)**: plants the polygon `rim` as a ring
/// of the face whose loop holds `at`, and covers it with a new face.
///
/// A strut from `at`'s start vertex to `rim[0]`, killed by `kemr` so
/// that corner is left as an empty ring of the host; the rest of the
/// rim grown inside that ring; one `mef` closing it. The host face
/// then carries the rim as a ring, and the membrane — the `mef`'s new
/// face — covers it with the same corners the other way round.
///
/// **Face geometry is inherited, not chosen**: `mev_line` and
/// `mef_chord` mint certified chords and put the membrane on the host
/// face's surface key, which is the state [`drill_hole`]'s walls and
/// the ring-face suites start from. A caller wanting a plane per face
/// runs [`plane_every_face`] afterwards.
///
/// `rim` has at least three corners, in the membrane's outward-CCW
/// order, and lies in the host face; nothing checks the order or the
/// placement.
pub fn plant_ring_face<T: geom_core::Decide>(
    body: &mut Body<T>,
    at: HalfEdgeKey,
    rim: &[Point3<T>],
    tol: Tol,
) -> RingFaceOps {
    assert!(rim.len() >= 3, "a ring face needs at least three corners");
    let strut = body
        .mev_line(MevSite::Fan { he1: at, he2: at }, rim[0], tol)
        .unwrap();
    let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    let mut chain = vec![
        body.mev_line(MevSite::Lone { r#loop: kill.ring }, rim[1], tol)
            .unwrap(),
    ];
    for &corner in &rim[2..] {
        let prev = chain.last().unwrap().he_minus;
        chain.push(
            body.mev_line(
                MevSite::Fan {
                    he1: prev,
                    he2: prev,
                },
                corner,
                tol,
            )
            .unwrap(),
        );
    }
    let membrane = body
        .mef_chord(
            MefSite::Chords {
                he1: chain[0].he_plus,
                he2: chain.last().unwrap().he_minus,
            },
            tol,
        )
        .unwrap();
    RingFaceOps {
        strut,
        kill,
        rim: chain,
        membrane,
    }
}

/// The operator keys [`drill_hole`] mints, in construction order.
pub struct HoleOps {
    /// Steps (f)–(i): the rim planted in the entry face and its
    /// membrane. The membrane face is dead once [`HoleOps::plug`] has
    /// run; its loop survives as the exit face's ring.
    pub ring: RingFaceOps,
    /// One vertical per rim corner, dropped inside the membrane from
    /// that corner to the matching exit point.
    pub drops: Vec<MevCreated>,
    /// The tube walls, one per rim edge: wall `i` joins drop `i` to
    /// drop `i + 1`, and the last closes onto the first.
    pub walls: Vec<MefCreated>,
    /// The `kfmrh` that kills the membrane into the exit face — the
    /// connected sum that raises the genus by one.
    pub plug: KfmrhResult,
}

/// **Mäntylä §9.3 steps (f)–(l)**: drills a polygonal through-hole
/// from the face whose loop holds `at` to the face `exit`.
///
/// [`plant_ring_face`] over `rim` in the entry face; then one vertical
/// per corner from `rim[i]` to `drops[i]` inside the membrane, the
/// tube walls cut out of the membrane between consecutive verticals,
/// and `kfmrh(exit, membrane)`, which leaves the membrane's loop — by
/// then the polygon `drops` — as a ring of `exit`. The genus rises by
/// one; the shell and solid counts do not change.
///
/// Every face the hole adds inherits the entry face's surface key, as
/// at [`plant_ring_face`]. `drops` has one point per rim corner and
/// lies in `exit`; nothing checks the placement.
pub fn drill_hole<T: geom_core::Decide>(
    body: &mut Body<T>,
    at: HalfEdgeKey,
    exit: FaceKey,
    rim: &[Point3<T>],
    drops: &[Point3<T>],
    tol: Tol,
) -> HoleOps {
    assert_eq!(rim.len(), drops.len(), "one drop per rim corner");
    let ring = plant_ring_face(body, at, rim, tol);
    // Corner `i`'s vertical hangs off the half-edge that starts there
    // inside the membrane: `rim[i].he_plus` moved to the membrane as
    // the `mef`'s `he1` side, and the LAST corner is where the
    // membrane's own `he_minus` starts.
    let mut verticals: Vec<MevCreated> = Vec::with_capacity(drops.len());
    for (i, &drop) in drops.iter().enumerate() {
        let anchor = ring
            .rim
            .get(i)
            .map_or(ring.membrane.he_minus, |m| m.he_plus);
        verticals.push(
            body.mev_line(
                MevSite::Fan {
                    he1: anchor,
                    he2: anchor,
                },
                drop,
                tol,
            )
            .unwrap(),
        );
    }
    let mut walls: Vec<MefCreated> = Vec::with_capacity(drops.len());
    for pair in verticals.windows(2) {
        walls.push(
            body.mef_chord(
                MefSite::Chords {
                    he1: pair[0].he_minus,
                    he2: pair[1].he_minus,
                },
                tol,
            )
            .unwrap(),
        );
    }
    // The last wall closes onto the first wall's far edge, which is
    // still in the membrane's loop.
    let first_far = body
        .find_half_edge(ring.membrane.face, verticals[0].vertex, verticals[1].vertex)
        .unwrap();
    walls.push(
        body.mef_chord(
            MefSite::Chords {
                he1: verticals.last().unwrap().he_minus,
                he2: first_far,
            },
            tol,
        )
        .unwrap(),
    );
    let plug = body.kfmrh(exit, ring.membrane.face).unwrap();
    HoleOps {
        ring,
        drops: verticals,
        walls,
        plug,
    }
}

/// Gives every face of `body` the Newell plane of its outer loop,
/// taken in the loop's stored order — so the plane's normal is the
/// face's OUTWARD normal wherever that loop winds CCW about it, which
/// is what `sense: true` claims. Each face gets a fresh surface key.
///
/// For a body whose faces sit on inherited keys — [`declined_cube`],
/// and whatever [`plant_ring_face`] and [`drill_hole`] add — this is
/// the step that makes every face planar in its own right.
///
/// # Panics
///
/// If a face's outer loop is not a cycle, or its corners do not
/// certify a plane at `tol`.
pub fn plane_every_face<T: geom_core::Decide>(body: &mut Body<T>, tol: Tol) {
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    for face in faces {
        let outer = body.get_face(face).unwrap().outer;
        let first = match body.get_loop(outer).unwrap().boundary {
            LoopBoundary::Cycle { first } => Some(first),
            LoopBoundary::Empty { .. } => None,
        }
        .expect("every outer loop is a cycle: an empty one has no polygon to plane");
        let corners: Vec<Point3<T>> = body
            .loop_cycle(first)
            .unwrap()
            .iter()
            .map(|&he| {
                let v = body.get_half_edge(he).unwrap().start;
                *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
            })
            .collect();
        body.set_face_surface(face, FaceSurface::New(plane(&corners, tol)))
            .unwrap();
    }
}

/// **The `w` × 2 × 2 block with square through-holes**: the box over
/// `[0, w] × [0, 2] × [0, 2]` and, for each `cx` in `hole_centres`, a
/// unit-square hole centred on `(cx, 1)` drilled top → bottom; then
/// every face planed ([`plane_every_face`]). The genus is the number of
/// holes.
///
/// [`prism_ops`] with face geometry declined, then one [`drill_hole`]
/// per centre, each entering the top face (through the first side's
/// top edge, which lies in the top face's loop) and leaving through
/// the bottom cap. **No description step**: a caller whose rows need
/// one runs [`describe_as_intersections`].
///
/// The holes must lie inside the top face and clear of one another;
/// nothing checks either.
pub fn holed_block<T: geom_core::Decide>(w: f64, hole_centres: &[f64], tol: Tol) -> Body<T> {
    let pt = |x: f64, y: f64, z: f64| Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z));
    let mut body = Body::<T>::new();
    let ops = prism_ops(
        &mut body,
        &[(0.0, 0.0), (w, 0.0), (w, 2.0), (0.0, 2.0)],
        (0.0, 2.0),
        pt,
        FaceGeometry::Declined,
        tol,
    );
    let entry = ops.sides[0].he_plus;
    for &cx in hole_centres {
        let (x0, x1, y0, y1) = (cx - 0.5, cx + 0.5, 0.5, 1.5);
        drill_hole(
            &mut body,
            entry,
            ops.bottom.face,
            &[
                pt(x0, y0, 2.0),
                pt(x1, y0, 2.0),
                pt(x1, y1, 2.0),
                pt(x0, y1, 2.0),
            ],
            &[
                pt(x0, y0, 0.0),
                pt(x1, y0, 0.0),
                pt(x1, y1, 0.0),
                pt(x0, y1, 0.0),
            ],
            tol,
        );
    }
    plane_every_face(&mut body, tol);
    body
}

// ---------------------------------------------------------------------
// The cylinder-wall sheet
// ---------------------------------------------------------------------

/// One cylinder description: the frame a [`cyl_wall_sheet`] is
/// authored in.
///
/// **The fields are `f64` at every scalar.** A sheet's corners, rim
/// centres and carrier axes are authored at `f64` and lifted
/// componentwise with `T::from_f64`, the same idiom [`prism_z`]'s point
/// map uses — so the `Interval` lane's points are the `f64` lane's own
/// roundings, enclosed exactly, rather than a second rounding taken in
/// interval arithmetic.
#[derive(Clone, Copy, Debug)]
pub struct CylFrame {
    /// A point on the axis; `v = 0` of the chart.
    pub origin: Point3<f64>,
    /// The axis direction, unit; `v` runs along it.
    pub axis: Vec3<f64>,
    /// The cylinder's radius.
    pub radius: f64,
    /// The seam direction, unit and perpendicular to the axis;
    /// `u = 0` of the chart.
    pub u_ref: Vec3<f64>,
}

impl CylFrame {
    /// The canonical frame: axis +z through the origin, seam at +x.
    pub fn canonical(radius: f64) -> Self {
        Self {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius,
            u_ref: Vec3::unit_x(),
        }
    }

    /// The canonical frame with its axis TILTED by `theta` about +y
    /// through the same origin, the seam co-rotated so it stays a unit
    /// vector perpendicular to the axis.
    ///
    /// A NEARBY locus, not the same one, and how near depends on where
    /// on the chart you look — which is the whole subject of the rows
    /// that use this frame, so they compute it themselves rather than
    /// reading a bound from here.
    pub fn tilted(radius: f64, theta: f64) -> Self {
        Self {
            origin: Point3::origin(),
            axis: Vec3::new(theta.sin(), 0.0, theta.cos()),
            radius,
            u_ref: Vec3::new(theta.cos(), 0.0, -theta.sin()),
        }
    }

    /// The unit cylinder about +z described FROM THE OTHER END: the
    /// origin a quarter up the axis, the axis reversed, and the seam
    /// rotated to azimuth `seam`.
    ///
    /// Not one field in common with `CylFrame::canonical(1.0)` and the
    /// SAME locus — every field a real seat's two instances disagree
    /// on, none of it moving the cylinder. A chart window `[u0, u1]` of
    /// the canonical frame is `[seam - u1, seam - u0]` here, and a
    /// height window `[v0, v1]` is `[0.25 - v1, 0.25 - v0]`; the suites
    /// using this frame write that arithmetic at their call sites,
    /// where the reader can check it against the world region the row
    /// names.
    pub fn opposed(seam: f64) -> Self {
        Self {
            origin: Point3::new(0.0, 0.0, 0.25),
            axis: -Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::new(seam.cos(), seam.sin(), 0.0),
        }
    }

    /// The radial unit vector at azimuth `u`.
    ///
    /// Read from the frame's own fields — a combination of two
    /// orthogonal unit vectors — rather than by projecting a chart
    /// point back onto the plane through the axis, which subtracts the
    /// point's axial component and carries that subtraction's rounding
    /// into a direction the chart already names exactly.
    pub fn radial(&self, u: f64) -> Vec3<f64> {
        let w = self.axis.cross(self.u_ref);
        self.u_ref * u.cos() + w * u.sin()
    }

    /// The chart map `S(u, v) = origin + radial(u)·radius + axis·v`.
    pub fn at<T: Real>(&self, u: f64, v: f64) -> Point3<T> {
        (self.origin + self.radial(u) * self.radius + self.axis * v).map(T::from_f64)
    }

    /// The axis point at height `v` — the centre of the rim there.
    fn centre(&self, v: f64) -> Point3<f64> {
        self.origin + self.axis * v
    }

    /// This frame's cylinder.
    pub fn surface<T: Real>(&self) -> Surface<T> {
        Surface::Cylinder {
            origin: self.origin.map(T::from_f64),
            axis: self.axis.map(T::from_f64),
            radius: T::from_f64(self.radius),
            u_ref: self.u_ref.map(T::from_f64),
        }
    }
}

/// Where a sheet's cylinder surface key lives.
///
/// Not a style choice: the seed face's surface slot is READ, and the
/// two readings want opposite things. A sheet built [`CylKey::OnSeed`]
/// presents TWO faces on the cylinder — the seed and the wall — and a
/// sheet built [`CylKey::Bare`] presents one, leaving the seed face on
/// the NURBS placeholder its `mvfs` minted. `crate::census`'s
/// conformal arm pairs coincident cylinder faces and wants the second;
/// the split rows walk the seed face's own chart and want the first.
#[derive(Clone, Copy, Debug)]
pub enum CylKey {
    /// Mint the cylinder into the SEED face's surface slot, so the
    /// sheet is that face's complement and the two share the key.
    OnSeed,
    /// Mint the cylinder as a bare arena key and leave the seed face
    /// on its `mvfs` placeholder. The wall face is then the body's
    /// only face on the cylinder.
    Bare,
    /// Share a key an earlier sheet minted; the seed face keeps its
    /// placeholder as under [`CylKey::Bare`].
    Shared(crate::geometry::SurfaceKey),
}

/// The cylinder-wall sheet, once, with the axis its callers differ on
/// open: where the cylinder key lives ([`CylKey`]).
///
/// Builds `[u0, u1] x [v0, v1]` of `frame` into `body` and returns the
/// wall face with the cylinder key it is on: two rim arcs on exact
/// circle carriers described as the cylinder cut by the plane at that
/// height, and two meridian struts on certified chord lines.
/// `source`, when given, is the recipe node id recorded on the
/// cylinder key as `GeomSource::minted(source, 0)`; it is recorded
/// whether the key was minted here or shared.
///
/// The descending rim runs on the REVERSED axis so its own parameter
/// still increases, which is how the split lane mints one.
///
/// **Pcurves are NOT minted here.** A caller growing several sheets on
/// one key mints once at the end, and `crate::chart_region`'s rows
/// read the refusal an UNMINTED chart gives before they mint at all.
/// [`cyl_wall_sheet`] is the door that mints.
///
/// Each rim's plane is inserted with `Body::add_surface`, which makes
/// no validity promise on its own: the plane is an orphan surface
/// until the rim edge naming it exists, and the `mev` that mints that
/// edge is the op whose postcondition covers it.
pub fn cyl_wall_sheet_keyed<T: geom_core::Decide + geom_brep::PcurveFittedLane>(
    body: &mut Body<T>,
    frame: CylFrame,
    key: CylKey,
    source: Option<u64>,
    (u0, u1): (f64, f64),
    (v0, v1): (f64, f64),
    tol: Tol,
) -> (FaceKey, crate::geometry::SurfaceKey) {
    let (p00, p10, p11, p01) = (
        frame.at(u0, v0),
        frame.at(u1, v0),
        frame.at(u1, v1),
        frame.at(u0, v1),
    );
    let seed = body.mvfs(p00).unwrap();
    let cyl = match key {
        CylKey::OnSeed => body
            .set_face_surface(seed.face, FaceSurface::New(frame.surface()))
            .unwrap(),
        CylKey::Bare => body.add_surface(frame.surface()),
        CylKey::Shared(cyl) => cyl,
    };
    if let Some(source) = source {
        body.set_surface_source(cyl, crate::GeomSource::minted(source, 0))
            .unwrap();
    }
    let rim = |body: &mut Body<T>, v: f64, ccw: bool| {
        let centre = frame.centre(v).map(T::from_f64);
        let plane = body.add_surface(Surface::Plane {
            origin: centre,
            normal: frame.axis.map(T::from_f64),
            u_ref: frame.u_ref.map(T::from_f64),
        });
        // Ascending: the frame's own axis and seam, over [u0, u1].
        // Descending: the REVERSED axis with the seam moved to u1, so
        // the parameter still runs forward, over [0, u1 - u0]. One
        // circle either way — the arms pick its axis, its seam and its
        // window, and nothing else about it differs.
        let (axis, seam, t0, t1) = if ccw {
            (frame.axis, frame.u_ref, u0, u1)
        } else {
            (-frame.axis, frame.radial(u1), 0.0, u1 - u0)
        };
        let carrier = Curve3::Circle {
            center: centre,
            axis: axis.map(T::from_f64),
            radius: T::from_f64(frame.radius),
            u_ref: seam.map(T::from_f64),
        };
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1: cyl,
                s2: plane,
                witness: frame.at((u0 + u1) * 0.5, v),
            },
            carrier,
            param_start: T::from_f64(t0),
            param_end: T::from_f64(t1),
        }
    };
    let bottom = rim(body, v0, true);
    let e_b = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p10,
            bottom,
            tol,
        )
        .unwrap();
    let e_r = body
        .mev_line(
            MevSite::Fan {
                he1: e_b.he_minus,
                he2: e_b.he_minus,
            },
            p11,
            tol,
        )
        .unwrap();
    let top = rim(body, v1, false);
    let e_t = body
        .mev(
            MevSite::Fan {
                he1: e_r.he_minus,
                he2: e_r.he_minus,
            },
            p01,
            top,
            tol,
        )
        .unwrap();
    let he = body
        .find_half_edge(seed.face, e_t.vertex, e_r.vertex)
        .unwrap();
    let face = body
        .mef(
            MefSite::Chords {
                he1: he,
                he2: e_b.he_plus,
            },
            EdgeCurveSpec::line_between(p01, p00),
            FaceSurface::Shared(cyl),
            tol,
        )
        .unwrap()
        .face;
    (face, cyl)
}

/// The canonical unit cylinder's wall sheet over `[u0, u1] x [z0, z1]`
/// on a bare key — `cyl` mints one when `None` and shares one when
/// `Some` — with the wall face's sense set and no pcurve minted.
///
/// `crate::census` and `crate::chart_region` build their wall-sheet
/// pairs through this spelling — which is what it is for, a window and
/// a sense over the canonical frame. It is not the only in-crate route
/// to a sheet: a fixture wanting another frame calls
/// [`cyl_wall_sheet_keyed`] directly, as `crate::census`'s own
/// divergent-description sheet does.
pub(crate) fn unit_cyl_sheet(
    body: &mut Body<f64>,
    cyl: Option<crate::geometry::SurfaceKey>,
    (u0, u1): (f64, f64),
    (z0, z1): (f64, f64),
    sense: bool,
    tol: Tol,
) -> (FaceKey, crate::geometry::SurfaceKey) {
    let (face, cyl) = cyl_wall_sheet_keyed(
        body,
        CylFrame::canonical(1.0),
        cyl.map_or(CylKey::Bare, CylKey::Shared),
        None,
        (u0, u1),
        (z0, z1),
        tol,
    );
    body.set_face_sense(face, sense).unwrap();
    (face, cyl)
}

/// An open cylinder-wall sheet over `[u0, u1] x [v0, v1]` of `frame`,
/// grown into `body` and returned, with every pcurve minted.
/// `source`, when given, is the recipe node id recorded on the
/// cylinder key as `GeomSource::minted(source, 0)`.
///
/// The sheet is the seed face's complement ([`CylKey::OnSeed`]), so the
/// cylinder key lives in the seed face's surface slot and the returned
/// face shares it. The descending rim runs on the REVERSED axis so its
/// own parameter still increases, which is how the split lane mints
/// one.
///
/// [`cyl_wall_sheet_keyed`] is the construction; this door fixes the
/// key placement and runs the pcurve pass over the result. The wall
/// face's sense is left where `mef` put it — [`unit_cyl_sheet`] is the
/// spelling that writes one.
pub fn cyl_wall_sheet<T: geom_core::Decide + geom_brep::PcurveFittedLane>(
    body: &mut Body<T>,
    frame: CylFrame,
    source: Option<u64>,
    (u0, u1): (f64, f64),
    (v0, v1): (f64, f64),
    tol: Tol,
) -> FaceKey {
    let (face, _) =
        cyl_wall_sheet_keyed(body, frame, CylKey::OnSeed, source, (u0, u1), (v0, v1), tol);
    crate::pcurves::mint_pcurves(body, tol).unwrap();
    face
}

#[cfg(test)]
mod tests {
    use super::*;
    use geom_core::Tol;

    /// **What a [`cyl_wall_sheet`] IS, held true rather than said.**
    /// Its doc states three facts a caller reasons with — one solid,
    /// the cylinder key shared with the seed face it was cut from, and
    /// the reversed descending rim — and each is a claim a rewrite of
    /// the builder could break silently, because a sheet's consumers
    /// read its faces and its carriers, not its counts.
    #[test]
    fn a_sheet_is_one_solid_with_a_shared_key_and_a_reversed_top_rim() {
        let mut body = Body::<f64>::new();
        let face = cyl_wall_sheet(
            &mut body,
            CylFrame::canonical(1.0),
            Some(11),
            (0.2, 1.4),
            (0.0, 1.0),
            Tol::witness(),
        );

        // The rim planes are bare arena keys, so the sheet is one
        // solid: the seed face and the wall cut from it.
        let counts = crate::test_support::arena_counts(&body);
        assert_eq!(
            (counts.solids, counts.faces, counts.vertices),
            (1, 2, 4),
            "the sheet solid alone, no scaffold: {counts:?}"
        );

        // The wall and the seed face it was cut from share one
        // cylinder key, and that key carries the source.
        let cyl = body.get_face(face).unwrap().surface;
        assert!(
            matches!(body.get_surface(cyl), Some(Surface::Cylinder { .. })),
            "the returned face is on the frame's cylinder"
        );
        assert_eq!(
            body.surface_source(cyl),
            Some(&crate::GeomSource::minted(11, 0)),
            "the cylinder key carries the whole source the door mints, \
             `node` and `expr` both — a row reading only `node` leaves \
             the minted index asserted by nothing"
        );

        // No lone-vertex face is left behind: every loop this body
        // holds bounds a face of the sheet itself.
        let empty_loops = body
            .loops()
            .filter(|(_, l)| matches!(l.boundary, crate::entity::LoopBoundary::Empty { .. }))
            .count();
        assert_eq!(empty_loops, 0, "no scaffold face survives the build");

        // The two rim carriers run on OPPOSED axes, which is what
        // keeps both parameters increasing.
        let axes: Vec<[f64; 3]> = body
            .edges()
            .filter_map(
                |(_, e)| match body.get_curve_geom(e.curve)?.certified()?.carrier() {
                    Curve3::Circle { axis, .. } => Some([axis.x, axis.y, axis.z]),
                    _ => None,
                },
            )
            .collect();
        assert_eq!(axes.len(), 2, "two rim arcs, on circle carriers");
        assert_eq!(
            axes[0],
            [-axes[1][0], -axes[1][1], -axes[1][2]],
            "the descending rim reverses the axis"
        );
    }

    /// **[`CylKey`]'s two minting arms, held rather than said.** Its
    /// doc tells a caller how many of the body's faces stand on the
    /// cylinder, which is what `crate::census`'s conformal arm pairs
    /// and what the split rows walk — and the door-level row above
    /// exercises one arm only, so the other would be prose with
    /// nothing under it.
    #[test]
    fn the_key_arms_put_the_cylinder_on_one_face_or_on_two() {
        let on_cylinder = |body: &Body<f64>| -> usize {
            body.faces()
                .filter(|(_, f)| {
                    matches!(body.get_surface(f.surface), Some(Surface::Cylinder { .. }))
                })
                .count()
        };

        let mut seeded = Body::<f64>::new();
        cyl_wall_sheet_keyed(
            &mut seeded,
            CylFrame::canonical(1.0),
            CylKey::OnSeed,
            None,
            (0.2, 1.4),
            (0.0, 1.0),
            Tol::witness(),
        );
        assert_eq!(
            on_cylinder(&seeded),
            2,
            "OnSeed: the seed face and the wall cut from it"
        );

        let mut bare = Body::<f64>::new();
        let (wall, cyl) = cyl_wall_sheet_keyed(
            &mut bare,
            CylFrame::canonical(1.0),
            CylKey::Bare,
            None,
            (0.2, 1.4),
            (0.0, 1.0),
            Tol::witness(),
        );
        assert_eq!(
            on_cylinder(&bare),
            1,
            "Bare: the wall alone, the seed face left on its mvfs placeholder"
        );
        assert_eq!(
            bare.get_face(wall).unwrap().surface,
            cyl,
            "the returned face is on the key the door reports"
        );
    }

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
