//! Shared test fixtures: canonical well-formed half-edge bodies, built
//! through the raw builder (insert with provisional null keys, then
//! patch — see the builder notes in [`crate::body`]).
//!
//! Test-only (`#[cfg(test)]` at the declaration site), so a `tests/`
//! binary cannot name any of it. This is one of three homes for test
//! vocabulary in this crate; `src/test_support_impl.rs`'s docs give the
//! rule for which one a new item belongs in. Three families:
//!
//! - [`ngon_pillow`] — the minimal *closed* body family: two n-gon faces
//!   glued along an n-cycle of edges ("pillow"). `n = 2` is the digon
//!   pillow (v2 e2 f2 — Euler–Poincaré 2−2+2 = 2, genus 0), the smallest
//!   closed manifold body and the successor of M0's single-face `tiny()`;
//!   `n = 1` is the legal self-loop digon (one vertex, one edge, both
//!   halves in different faces' one-half-edge loops).
//! - [`raw_prism`] — 2 n-gon caps + n quads (v = 2n, e = 3n, f = n + 2);
//!   every vertex has valence 3, exercising nontrivial vertex orbits.
//! - [`mvfs_state`] — the skeletal body `mvfs` creates: solid + shell +
//!   one face whose outer loop is `Empty`, holding a lone vertex.
//!   Tier-1-legal by design.
//!
//! Plus [`refile_shells`], the raw arena write that files several
//! shells under one solid, which no operator does.
//!
//! Plus the whole-body observations the suites compare by —
//! [`arena_snapshot`] (every arena's length), [`deep_snapshot`]
//! (key-for-key, field-for-field, provenance-for-provenance, and each
//! arena's next key) and [`deep_rows`] (the same less the next keys).
//!
//! Plus the **operator-built** family — [`ops_holed_box`] and
//! [`ops_genus2`], the acceptance-test bodies rebuilt in-crate for
//! the kill-direction, oracle, and teardown tests over
//! [`crate::test_support_fixtures::declined_cube`], and
//! [`ops_ring_bridge`] and [`ops_strut_cube`], the two shapes here
//! whose edge has both halves in one loop — the holed box with its hole
//! rim bridged back into the top face's outer loop, and the cube with a
//! pendant strut planted on that loop.
//!
//! All geometry is placeholder (structural validation never reads scalar
//! values). Coordinates are index-derived placeholders, **not** faithful
//! positions (the prism's points are collinear, not an n-gon); the
//! documented geometric pictures live in the doc comments here and in
//! the topology itself, and that is what orientation reasoning in tests
//! points at.
//!

// Test-support code: panicking is a test's failure mechanism (L5), and
// fixture unwraps are on keys the fixture itself just minted.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;

use crate::body::Body;
use crate::entity::{
    Edge, EdgeKey, EntityId, Face, FaceKey, HalfEdge, HalfEdgeKey, Loop, LoopBoundary, LoopKey,
    Shell, ShellKey, Solid, SolidKey, Vertex, VertexKey,
};
use crate::euler::{MefCreated, MefSite, MevCreated, MevSite, MvfsCreated};
use crate::euler_ring::{KemrResult, KfmrhResult, MekrResult, MekrSite};
use crate::geometry::{CurveKey, PointKey, SurfaceKey};
use crate::provenance::Provenance;
use crate::readback::euler_counts;
use crate::test_support_fixtures::{CubeOps, declined_cube, drill_hole};
use crate::test_support_impl::ArenaCounts;
use geom_core::Tol;

/// The fixture provenance (all fixture entities share it).
pub(crate) fn prov() -> Provenance {
    Provenance::Primordial { op: "fixture" }
}

/// A certified scaffolding curve for raw-insertion fixtures (M2 PR 3:
/// the curve arena holds certified `EdgeCurve`s and nothing else): the
/// canonical self-loop circle at `anchor` — deterministic, honest data,
/// no geometric claims about the fixture's topology (raw fixtures make
/// no validity promises anyway; the anchor keeps snapshots
/// per-call-site distinct like the M0 placeholder anchors did).
pub(crate) fn test_curve(anchor: Point3<f64>, tol: Tol) -> geom_brep::EdgeCurve<f64> {
    let spec = geom_brep::EdgeCurveSpec::self_loop_circle_at(anchor);
    geom_brep::EdgeCurve::certify(
        spec,
        anchor,
        anchor,
        |_| None,
        geom_core::Band::linear(tol).unwrap(),
    )
    .unwrap()
}

/// A raw-fixture surface: the `Nurbs` "no description yet" state (the
/// anchor argument is accepted for call-site symmetry with
/// [`test_curve`] and ignored — surfaces carry no certification).
pub(crate) fn test_surface(_anchor: Point3<f64>) -> geom::Surface<f64> {
    geom::Surface::nurbs_placeholder()
}

/// A raw plane SURFACE fixture (an operation input, not a face datum —
/// it belongs to no body, so there is no sense bit to fold and no
/// outward normal to mint; suites that need a material side for it
/// state one explicitly at the call site).
pub(crate) fn plane_surface(
    origin: Point3<f64>,
    normal: geom_core::Vec3<f64>,
    u_ref: geom_core::Vec3<f64>,
) -> geom::Surface<f64> {
    geom::Surface::Plane {
        origin,
        normal,
        u_ref,
    }
}

/// All ten arena lengths of a body: the seven topology arenas, held as
/// the crate's one [`ArenaCounts`], plus the three geometry arenas.
/// The delta base of the operator count checks. Counts are not a
/// "body unchanged" observation; [`deep_snapshot`] is.
///
/// The topology half is *not* restated here: an `ArenaSnapshot` is an
/// [`ArenaCounts`] extended by geometry, and the seven have exactly one
/// producer ([`Body::arena_counts`]), shared with the debug
/// postcondition that checks them against each operator's declared
/// delta.
///
/// A different quantity from [`crate::euler::ArenaDelta`]: these are
/// counts, not shifts, and they include geometry.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct ArenaSnapshot {
    pub counts: ArenaCounts,
    pub points: usize,
    pub curves: usize,
    pub surfaces: usize,
}

/// Captures every arena length of `body`.
pub(crate) fn arena_snapshot(body: &Body<f64>) -> ArenaSnapshot {
    ArenaSnapshot {
        counts: body.arena_counts(),
        points: body.points().count(),
        curves: body.curves().count(),
        surfaces: body.surfaces().count(),
    }
}

/// A deep, order-sensitive snapshot of a body, and the crate's one
/// "body unchanged" observation: one line per row of every table on
/// [`Body`] (the ten arenas, the seven D5 provenance maps, the pcurve
/// caches, the null-face records, the three geometry origin maps, and
/// the field and axis source channels), each in slot-index order,
/// carrying the row's key and its full payload through `Debug` (which
/// prints every field). Two snapshots compare equal iff the bodies
/// are row-for-row and field-for-field identical and every arena would
/// mint the same next key.
///
/// Every table is walked as a table rather than through live keys, so
/// a row left behind for a dead key shows like any other. Each arena
/// also gives one "next key" line: the key its next insert would mint,
/// which is its free-list head. A key slot minted and freed again
/// leaves every row as it was, but the last slot freed goes to the
/// head with its version bumped past any key it held before, so the
/// line moves (D1: a refused op consumes no key slots).
///
/// `Body` is destructured without `..`, so a field this walk does not
/// read fails to compile here. The one field it skips is the debug
/// build's surgery depth, which counts the door scopes open on the
/// body and is not body state (a clone resets it).
pub(crate) fn deep_snapshot(body: &Body<f64>) -> Vec<String> {
    let (mut lines, next_keys) = snapshot_rows_and_next_keys(body);
    lines.extend(next_keys);
    lines
}

/// [`deep_snapshot`] without its next-key lines: the observation for a
/// make-then-kill round trip, which restores every row and consumes the
/// key slots it minted.
pub(crate) fn deep_rows(body: &Body<f64>) -> Vec<String> {
    snapshot_rows_and_next_keys(body).0
}

fn snapshot_rows_and_next_keys(body: &Body<f64>) -> (Vec<String>, Vec<String>) {
    fn walk<K: std::fmt::Debug, V: std::fmt::Debug>(
        lines: &mut Vec<String>,
        table: &str,
        rows: impl Iterator<Item = (K, V)>,
    ) {
        lines.extend(rows.map(|(k, v)| format!("{table} {k:?}: {v:?}")));
    }
    fn walk_arena<K: slotmap::Key, V: Clone + std::fmt::Debug>(
        lines: &mut Vec<String>,
        next_keys: &mut Vec<String>,
        table: &str,
        arena: &slotmap::SlotMap<K, V>,
    ) {
        walk(lines, table, arena.iter());
        // An insert whose value closure fails reports the key it would
        // have minted and leaves the arena as it was.
        let next = arena
            .clone()
            .try_insert_with_key(Err::<V, K>)
            .expect_err("the value closure refuses");
        next_keys.push(format!("{table} next key: {next:?}"));
    }
    let Body {
        solids,
        shells,
        faces,
        loops,
        half_edges,
        edges,
        vertices,
        points,
        curves,
        surfaces,
        pcurves,
        null_faces,
        solid_provenance,
        shell_provenance,
        face_provenance,
        loop_provenance,
        half_edge_provenance,
        edge_provenance,
        vertex_provenance,
        point_origins,
        curve_origins,
        surface_origins,
        surface_field_sources,
        surface_axis_sources,
        #[cfg(debug_assertions)]
            surgery: _,
    } = body;
    let mut lines = Vec::new();
    let mut next_keys = Vec::new();
    let keys = &mut next_keys;
    walk_arena(&mut lines, keys, "solid", solids);
    walk_arena(&mut lines, keys, "shell", shells);
    walk_arena(&mut lines, keys, "face", faces);
    walk_arena(&mut lines, keys, "loop", loops);
    walk_arena(&mut lines, keys, "half-edge", half_edges);
    walk_arena(&mut lines, keys, "edge", edges);
    walk_arena(&mut lines, keys, "vertex", vertices);
    walk_arena(&mut lines, keys, "point", points);
    walk_arena(&mut lines, keys, "curve", curves);
    walk_arena(&mut lines, keys, "surface", surfaces);
    walk(&mut lines, "pcurve", pcurves.iter());
    walk(&mut lines, "null-face", null_faces.iter());
    walk(&mut lines, "solid-provenance", solid_provenance.iter());
    walk(&mut lines, "shell-provenance", shell_provenance.iter());
    walk(&mut lines, "face-provenance", face_provenance.iter());
    walk(&mut lines, "loop-provenance", loop_provenance.iter());
    walk(
        &mut lines,
        "half-edge-provenance",
        half_edge_provenance.iter(),
    );
    walk(&mut lines, "edge-provenance", edge_provenance.iter());
    walk(&mut lines, "vertex-provenance", vertex_provenance.iter());
    walk(&mut lines, "point-origin", point_origins.iter());
    walk(&mut lines, "curve-origin", curve_origins.iter());
    walk(&mut lines, "surface-origin", surface_origins.iter());
    walk(
        &mut lines,
        "surface-field-sources",
        surface_field_sources.iter(),
    );
    walk(
        &mut lines,
        "surface-axis-source",
        surface_axis_sources.iter(),
    );
    (lines, next_keys)
}

/// Runs `op` on `body`, asserts it fails with exactly `expected`, and
/// asserts the body is untouched to the [`deep_snapshot`].
pub(crate) fn assert_err_deep_unchanged(
    body: &mut Body<f64>,
    expected: &crate::euler::EulerOpError,
    op: impl FnOnce(&mut Body<f64>) -> crate::euler::EulerOpError,
) {
    let before = deep_snapshot(body);
    let err = op(body);
    assert_eq!(&err, expected);
    assert_eq!(deep_snapshot(body), before, "body changed on Err");
}

/// An anchor fault a kill can write ([`kill_anchor_faults`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum KillAnchorFault {
    /// A vertex whose `emanating` does not start at it.
    AnchorOff(VertexKey),
    /// A vertex at `None` that a half-edge starts at.
    NoneWithEdges(VertexKey),
    /// A loop whose `first` is dead or lies in another loop, or whose
    /// `Empty` vertex is dead, has a half-edge starting at it, or shares
    /// the loop with a member.
    LoopOff(LoopKey),
    /// A vertex two `Empty` loops hold.
    HeldTwice(VertexKey),
    /// A vertex no half-edge starts at and no `Empty` loop holds.
    Orphan(VertexKey),
    /// A half-edge whose `parent_loop` does not resolve, or a face
    /// that lists a loop that does not.
    DeadLoop(EntityId),
    /// A half-edge whose start does not resolve, or an `Empty` loop
    /// whose vertex does not.
    DeadStart(EntityId),
    /// A loop whose `face` does not resolve, or a shell that lists a
    /// face that does not.
    DeadFace(EntityId),
    /// A face whose `shell` does not resolve, or a solid that lists a
    /// shell that does not.
    DeadShell(EntityId),
    /// A shell whose `solid` does not resolve.
    DeadSolid(ShellKey),
    /// A half-edge whose `edge` does not resolve.
    DeadEdge(HalfEdgeKey),
    /// A half-edge whose `next` or `prev` does not resolve, a loop whose
    /// `first` does not, a vertex whose `emanating` does not, or an edge
    /// a slot of which does not.
    DeadHalfEdge(EntityId),
    /// A face whose null-face record names a loop that does not
    /// resolve.
    DeadNullFaceLoop(FaceKey),
}

/// Every [`KillAnchorFault`] on `body`.
pub(crate) fn kill_anchor_faults(body: &Body<f64>) -> Vec<KillAnchorFault> {
    let mut faults = Vec::new();
    for (v, vertex) in body.vertices() {
        let incident = body.half_edges().any(|(_, h)| h.start == v);
        let holders = body
            .loops()
            .filter(|(_, l)| l.boundary == LoopBoundary::Empty { vertex: v })
            .count();
        match vertex.emanating {
            Some(he) if body.get_half_edge(he).map(|h| h.start) != Some(v) => {
                faults.push(KillAnchorFault::AnchorOff(v));
            }
            None if incident => faults.push(KillAnchorFault::NoneWithEdges(v)),
            _ => {}
        }
        if holders >= 2 {
            faults.push(KillAnchorFault::HeldTwice(v));
        }
        if holders == 0 && !incident {
            faults.push(KillAnchorFault::Orphan(v));
        }
    }
    for (l, data) in body.loops() {
        let off = match data.boundary {
            LoopBoundary::Cycle { first } => {
                body.get_half_edge(first).map(|h| h.parent_loop) != Some(l)
            }
            LoopBoundary::Empty { vertex } => {
                body.get_vertex(vertex).is_none()
                    || body
                        .half_edges()
                        .any(|(_, h)| h.start == vertex || h.parent_loop == l)
            }
        };
        if off {
            faults.push(KillAnchorFault::LoopOff(l));
        }
    }
    for (he, data) in body.half_edges() {
        if body.get_loop(data.parent_loop).is_none() {
            faults.push(KillAnchorFault::DeadLoop(EntityId::HalfEdge(he)));
        }
        if body.get_vertex(data.start).is_none() {
            faults.push(KillAnchorFault::DeadStart(EntityId::HalfEdge(he)));
        }
        if body.get_edge(data.edge).is_none() {
            faults.push(KillAnchorFault::DeadEdge(he));
        }
        if [data.next, data.prev]
            .iter()
            .any(|&link| body.get_half_edge(link).is_none())
        {
            faults.push(KillAnchorFault::DeadHalfEdge(EntityId::HalfEdge(he)));
        }
    }
    for (l, data) in body.loops() {
        if let LoopBoundary::Cycle { first } = data.boundary
            && body.get_half_edge(first).is_none()
        {
            faults.push(KillAnchorFault::DeadHalfEdge(EntityId::Loop(l)));
        }
    }
    for (v, data) in body.vertices() {
        if data
            .emanating
            .is_some_and(|he| body.get_half_edge(he).is_none())
        {
            faults.push(KillAnchorFault::DeadHalfEdge(EntityId::Vertex(v)));
        }
    }
    for (e, data) in body.edges() {
        if [data.he_plus, data.he_minus]
            .iter()
            .any(|&slot| body.get_half_edge(slot).is_none())
        {
            faults.push(KillAnchorFault::DeadHalfEdge(EntityId::Edge(e)));
        }
    }
    for (l, data) in body.loops() {
        if let LoopBoundary::Empty { vertex } = data.boundary
            && body.get_vertex(vertex).is_none()
        {
            faults.push(KillAnchorFault::DeadStart(EntityId::Loop(l)));
        }
        if body.get_face(data.face).is_none() {
            faults.push(KillAnchorFault::DeadFace(EntityId::Loop(l)));
        }
    }
    for (f, data) in body.faces() {
        if core::iter::once(&data.outer)
            .chain(&data.rings)
            .any(|&l| body.get_loop(l).is_none())
        {
            faults.push(KillAnchorFault::DeadLoop(EntityId::Face(f)));
        }
        if body.get_shell(data.shell).is_none() {
            faults.push(KillAnchorFault::DeadShell(EntityId::Face(f)));
        }
    }
    for (s, data) in body.shells() {
        if data.faces.iter().any(|&f| body.get_face(f).is_none()) {
            faults.push(KillAnchorFault::DeadFace(EntityId::Shell(s)));
        }
        if body.get_solid(data.solid).is_none() {
            faults.push(KillAnchorFault::DeadSolid(s));
        }
    }
    for (solid, data) in body.solids() {
        if data.shells.iter().any(|&s| body.get_shell(s).is_none()) {
            faults.push(KillAnchorFault::DeadShell(EntityId::Solid(solid)));
        }
    }
    for (face, record) in body.null_faces() {
        if record.loops().iter().any(|&l| body.get_loop(l).is_none()) {
            faults.push(KillAnchorFault::DeadNullFaceLoop(face));
        }
    }
    faults
}

/// Asserts that `kill` refuses exactly `expected` and leaves `body`
/// deep-unchanged. The kill runs inside a surgery scope: a debug build's
/// tier-1 postcondition would otherwise answer an `Ok` on a torn body
/// first, whatever the kill wrote. An `Ok` fails naming the anchor
/// faults the kill wrote, those [`kill_anchor_faults`] reads after it and
/// not before.
pub(crate) fn assert_kill_refuses<R>(
    body: &mut Body<f64>,
    expected: &crate::euler::EulerOpError,
    kill: impl FnOnce(&mut Body<f64>) -> Result<R, crate::euler::EulerOpError>,
) {
    assert_torn_op_refuses(body, expected, "kill", kill);
}

/// [`assert_kill_refuses`] for a make operator, which a torn input can
/// carry to the same anchor faults.
pub(crate) fn assert_make_refuses<R>(
    body: &mut Body<f64>,
    expected: &crate::euler::EulerOpError,
    make: impl FnOnce(&mut Body<f64>) -> Result<R, crate::euler::EulerOpError>,
) {
    assert_torn_op_refuses(body, expected, "make", make);
}

fn assert_torn_op_refuses<R>(
    body: &mut Body<f64>,
    expected: &crate::euler::EulerOpError,
    what: &str,
    op: impl FnOnce(&mut Body<f64>) -> Result<R, crate::euler::EulerOpError>,
) {
    let before = deep_snapshot(body);
    let faults_before = kill_anchor_faults(body);
    let mut scope = body.begin_surgery();
    let got = op(&mut scope).map(|_| ());
    drop(scope);
    match got {
        Ok(()) => {
            let written: Vec<_> = kill_anchor_faults(body)
                .into_iter()
                .filter(|fault| !faults_before.contains(fault))
                .collect();
            panic!("expected {expected:?}; the {what} returned Ok, writing {written:?}");
        }
        Err(err) => {
            assert_eq!(&err, expected);
            assert_eq!(deep_snapshot(body), before, "body changed on Err");
        }
    }
}

/// A distinct-per-index placeholder coordinate (`u32` round trip keeps
/// the cast lossless; fixture sizes are tiny).
fn index_coord(i: usize) -> f64 {
    f64::from(u32::try_from(i).unwrap_or(u32::MAX))
}

/// Key bundle for [`ngon_pillow`] (and [`pillow`], its n = 2 instance).
///
/// Naming: face A is the "front" n-gon, face B the "back". Edge `e[i]`
/// runs `v[i] → v[(i+1) % n]` in its intrinsic (plus) direction.
/// `hes_a[i]` is `e[i]`'s half in loop A (start `v[i]`, the plus half);
/// `hes_b[i]` is its half in loop B (start `v[(i+1) % n]`, the minus
/// half). Loop A walks `v0 → v1 → …`; loop B walks the same rim the
/// other way (`next(hes_b[i]) = hes_b[i-1]`), per antiparallelism.
#[allow(dead_code)] // key bundles expose every minted key; tests pick what they need
pub(crate) struct NgonPillow {
    pub body: Body<f64>,
    pub points: Vec<PointKey>,
    pub curves: Vec<CurveKey>,
    pub surface_a: SurfaceKey,
    pub surface_b: SurfaceKey,
    pub vertices: Vec<VertexKey>,
    pub edges: Vec<EdgeKey>,
    pub hes_a: Vec<HalfEdgeKey>,
    pub hes_b: Vec<HalfEdgeKey>,
    pub loop_a: LoopKey,
    pub loop_b: LoopKey,
    pub face_a: FaceKey,
    pub face_b: FaceKey,
    pub shell: ShellKey,
    pub solid: SolidKey,
}

/// Builds the n-gon pillow (n ≥ 1): two faces glued along an n-cycle.
///
/// Counts: v = n, e = n, f = 2, so v − e + f = 2 (sphere, genus 0) for
/// every n. Vertex `v[i]`'s emanating half-edge is `hes_a[i]`; its orbit
/// is `[hes_a[i], hes_b[(i-1+n) % n]]` (both halves starting at `v[i]`).
///
/// # Panics
///
/// If `n == 0` (fixture misuse, not kernel behavior).
pub(crate) fn ngon_pillow(n: usize, tol: Tol) -> NgonPillow {
    assert!(n >= 1, "an n-gon pillow needs at least one edge");
    let mut body = Body::<f64>::new();
    let null_he = HalfEdgeKey::default();

    // Spine, top down, patching the upward lists as children arrive.
    let solid = body.add_solid(Solid { shells: vec![] }, prov());
    let shell = body.add_shell(
        Shell {
            faces: vec![],
            solid,
        },
        prov(),
    );
    body.get_solid_mut(solid).unwrap().shells.push(shell);

    // Geometry: n rim points (values irrelevant to structural checks),
    // one placeholder curve per edge, one placeholder surface per face.
    let points: Vec<PointKey> = (0..n)
        .map(|i| body.add_point(Point3::new(index_coord(i), 0.0, 0.0)))
        .collect();
    let curves: Vec<CurveKey> = (0..n)
        .map(|_| body.add_curve(test_curve(Point3::origin(), tol)))
        .collect();
    let surface_a = body.add_surface(test_surface(Point3::origin()));
    let surface_b = body.add_surface(test_surface(Point3::origin()));

    // Vertices (emanating patched once half-edges exist).
    let vertices: Vec<VertexKey> = points
        .iter()
        .map(|&point| {
            body.add_vertex(
                Vertex {
                    point,
                    emanating: None,
                },
                prov(),
            )
        })
        .collect();

    // Edges with provisional half-edge slots.
    let edges: Vec<EdgeKey> = curves
        .iter()
        .map(|&curve| {
            body.add_edge(
                Edge {
                    he_plus: null_he,
                    he_minus: null_he,
                    curve,
                },
                prov(),
            )
        })
        .collect();

    // Half-edges with provisional links.
    let hes_a: Vec<HalfEdgeKey> = (0..n)
        .map(|i| {
            body.add_half_edge(
                HalfEdge {
                    edge: edges[i],
                    start: vertices[i],
                    parent_loop: LoopKey::default(),
                    next: null_he,
                    prev: null_he,
                },
                prov(),
            )
        })
        .collect();
    let hes_b: Vec<HalfEdgeKey> = (0..n)
        .map(|i| {
            body.add_half_edge(
                HalfEdge {
                    edge: edges[i],
                    start: vertices[(i + 1) % n],
                    parent_loop: LoopKey::default(),
                    next: null_he,
                    prev: null_he,
                },
                prov(),
            )
        })
        .collect();

    // Loops and faces, then patch the loop→face back-pointers.
    let loop_a = body.add_loop(
        Loop {
            boundary: LoopBoundary::Cycle { first: hes_a[0] },
            face: FaceKey::default(),
        },
        prov(),
    );
    let loop_b = body.add_loop(
        Loop {
            boundary: LoopBoundary::Cycle { first: hes_b[0] },
            face: FaceKey::default(),
        },
        prov(),
    );
    let face_a = body.add_face(
        Face {
            sense: true,
            surface: surface_a,
            outer: loop_a,
            rings: vec![],
            shell,
        },
        prov(),
    );
    let face_b = body.add_face(
        Face {
            sense: true,
            surface: surface_b,
            outer: loop_b,
            rings: vec![],
            shell,
        },
        prov(),
    );
    body.get_loop_mut(loop_a).unwrap().face = face_a;
    body.get_loop_mut(loop_b).unwrap().face = face_b;
    body.get_shell_mut(shell).unwrap().faces = vec![face_a, face_b];

    // Close the cycles: loop A walks the rim forward, loop B backward.
    for i in 0..n {
        let a = body.get_half_edge_mut(hes_a[i]).unwrap();
        a.parent_loop = loop_a;
        a.next = hes_a[(i + 1) % n];
        a.prev = hes_a[(i + n - 1) % n];
        let b = body.get_half_edge_mut(hes_b[i]).unwrap();
        b.parent_loop = loop_b;
        b.next = hes_b[(i + n - 1) % n];
        b.prev = hes_b[(i + 1) % n];
    }
    // Claim the halves: the A half is the plus (intrinsic) direction.
    for i in 0..n {
        let e = body.get_edge_mut(edges[i]).unwrap();
        e.he_plus = hes_a[i];
        e.he_minus = hes_b[i];
    }
    // Anchor the vertices.
    for i in 0..n {
        body.get_vertex_mut(vertices[i]).unwrap().emanating = Some(hes_a[i]);
    }

    NgonPillow {
        body,
        points,
        curves,
        surface_a,
        surface_b,
        vertices,
        edges,
        hes_a,
        hes_b,
        loop_a,
        loop_b,
        face_a,
        face_b,
        shell,
        solid,
    }
}

/// The digon pillow — the minimal closed fixture (see [`ngon_pillow`]).
pub(crate) fn pillow(tol: Tol) -> NgonPillow {
    ngon_pillow(2, tol)
}

/// The PR 4 detached-digon transient with `n` digons: a pillow, and
/// `n` digons each grown on its own ring of the pillow's seed face and
/// promoted (`mfkrh`) — one shell entity of `n + 1` closed components.
/// Returns (body, shell, seed face, the promoted faces in order). The
/// seed face is the shell's first face.
pub(crate) fn detached_digons(n: usize) -> (Body<f64>, ShellKey, FaceKey, Vec<FaceKey>) {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(1.0, 0.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    body.mef_chord(
        MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        },
        Tol::witness(),
    )
    .unwrap();
    let mut promoted = Vec::new();
    for i in 0..n {
        let x = 2.0 * (i as f64) + 2.0;
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                Point3::new(x, 0.0, 0.0),
                Tol::witness(),
            )
            .unwrap();
        let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
        let grow = body
            .mev_line(
                MevSite::Lone { r#loop: kill.ring },
                Point3::new(x + 1.0, 0.0, 0.0),
                Tol::witness(),
            )
            .unwrap();
        body.mef_chord(
            MefSite::Chords {
                he1: grow.he_plus,
                he2: grow.he_minus,
            },
            Tol::witness(),
        )
        .unwrap();
        promoted.push(body.mfkrh_plug(kill.ring, true).unwrap().face);
    }
    assert_eq!(body.get_shell(seed.shell).unwrap().faces[0], seed.face);
    (body, seed.shell, seed.face, promoted)
}

/// Every shell of `donor` refiled under `keeper` — appended to
/// `keeper`'s shell list in `donor`'s order, each back-pointer moved —
/// and `donor` removed: the raw-arena spelling of "these shells are
/// one solid's".
///
/// **A solid with several shells is not constructible through the
/// public operators** (`mvfs` mints one solid per shell), so every row
/// that needs one writes the arenas. The emptied donor is REMOVED,
/// not left standing — a shell-less solid is `SolidWithoutShells` —
/// and its arena removal is PAIRED with its provenance removal the
/// way `kvfs` pairs them, because a removal that leaves the record
/// behind is `LeakedProvenance`. Which of `keeper`'s shells comes
/// first is the caller's choice of which solid is the keeper.
pub(crate) fn refile_shells(body: &mut Body<f64>, donor: SolidKey, keeper: SolidKey) {
    let moved = body.shells_of_solid(donor).expect("a live donor").to_vec();
    for shell in &moved {
        body.get_shell_mut(*shell).expect("a live shell").solid = keeper;
    }
    body.get_solid_mut(keeper)
        .expect("a live keeper")
        .shells
        .extend(moved);
    body.solids.remove(donor);
    body.solid_provenance.remove(donor);
}

/// Key bundle for [`prism`].
///
/// Geometric picture (documented so orientation tests can point at it):
/// rim indices increase **counterclockwise viewed from above** (+z);
/// bottom cap at z = 0 (outward normal −z), top cap at z = 1 (outward
/// normal +z), side quads with outward normals radially out. Loops obey
/// the interior-left rule ([`crate::entity`] module docs):
///
/// - top cap: `t0 → t1 → … → t(n−1)` (counterclockwise seen from above
///   = from outside);
/// - bottom cap: `… → u1 → u0 → u(n−1) → …` (decreasing index — that is
///   counterclockwise seen from *below*, i.e. from outside);
/// - side quad `i`: `u[i] → u[i+1] → t[i+1] → t[i]`.
///
/// Edges (intrinsic/plus directions): `et[i]: t[i] → t[i+1]` (plus half
/// `ht[i]` in the top cap), `eb[i]: u[i] → u[i+1]` (plus half `s0[i]` in
/// side `i`), `ev[i]: u[i] → t[i]` (plus half `s1[(i−1+n) % n]` in side
/// `i−1`). Side quad `i`'s cycle is `s0[i] → s1[i] → s2[i] → s3[i]`
/// with starts `u[i], u[i+1], t[i+1], t[i]`.
#[allow(dead_code)] // key bundles expose every minted key; tests pick what they need
pub(crate) struct RawPrism {
    pub body: Body<f64>,
    pub t: Vec<VertexKey>,
    pub u: Vec<VertexKey>,
    pub ht: Vec<HalfEdgeKey>,
    pub hb: Vec<HalfEdgeKey>,
    pub s0: Vec<HalfEdgeKey>,
    pub s1: Vec<HalfEdgeKey>,
    pub s2: Vec<HalfEdgeKey>,
    pub s3: Vec<HalfEdgeKey>,
    pub et: Vec<EdgeKey>,
    pub eb: Vec<EdgeKey>,
    pub ev: Vec<EdgeKey>,
    pub loop_top: LoopKey,
    pub loop_bottom: LoopKey,
    pub loop_side: Vec<LoopKey>,
    pub face_top: FaceKey,
    pub face_bottom: FaceKey,
    pub face_side: Vec<FaceKey>,
    pub shell: ShellKey,
    pub solid: SolidKey,
}

/// Builds the n-prism (n ≥ 2): two n-gon caps plus n side quads.
///
/// Counts: v = 2n, e = 3n, f = n + 2, so v − e + f = 2 (genus 0); every
/// vertex has valence 3. See [`RawPrism`] for the orientation picture.
///
/// # Panics
///
/// If `n < 2` (fixture misuse, not kernel behavior).
pub(crate) fn raw_prism(n: usize, tol: Tol) -> RawPrism {
    assert!(n >= 2, "a prism needs at least a digon cap");
    let mut body = Body::<f64>::new();
    let null_he = HalfEdgeKey::default();

    let solid = body.add_solid(Solid { shells: vec![] }, prov());
    let shell = body.add_shell(
        Shell {
            faces: vec![],
            solid,
        },
        prov(),
    );
    body.get_solid_mut(solid).unwrap().shells.push(shell);

    // Geometry. Coordinates are indexed placeholders standing in for the
    // documented picture (indices counterclockwise from above; bottom
    // z = 0, top z = 1); structural validation never reads them.
    let top_points: Vec<PointKey> = (0..n)
        .map(|i| body.add_point(Point3::new(index_coord(i), 0.0, 1.0)))
        .collect();
    let bottom_points: Vec<PointKey> = (0..n)
        .map(|i| body.add_point(Point3::new(index_coord(i), 0.0, 0.0)))
        .collect();
    let mut curve = || body.add_curve(test_curve(Point3::origin(), tol));
    let curves_t: Vec<CurveKey> = (0..n).map(|_| curve()).collect();
    let curves_b: Vec<CurveKey> = (0..n).map(|_| curve()).collect();
    let curves_v: Vec<CurveKey> = (0..n).map(|_| curve()).collect();
    let mut surface = || body.add_surface(test_surface(Point3::origin()));
    let surface_top = surface();
    let surface_bottom = surface();
    let surface_side: Vec<SurfaceKey> = (0..n).map(|_| surface()).collect();

    let t: Vec<VertexKey> = top_points
        .iter()
        .map(|&point| {
            body.add_vertex(
                Vertex {
                    point,
                    emanating: None,
                },
                prov(),
            )
        })
        .collect();
    let u: Vec<VertexKey> = bottom_points
        .iter()
        .map(|&point| {
            body.add_vertex(
                Vertex {
                    point,
                    emanating: None,
                },
                prov(),
            )
        })
        .collect();

    let mut edge = |curve: CurveKey| {
        body.add_edge(
            Edge {
                he_plus: null_he,
                he_minus: null_he,
                curve,
            },
            prov(),
        )
    };
    let et: Vec<EdgeKey> = curves_t.iter().map(|&c| edge(c)).collect();
    let eb: Vec<EdgeKey> = curves_b.iter().map(|&c| edge(c)).collect();
    let ev: Vec<EdgeKey> = curves_v.iter().map(|&c| edge(c)).collect();

    let mut half_edge = |edge: EdgeKey, start: VertexKey| {
        body.add_half_edge(
            HalfEdge {
                edge,
                start,
                parent_loop: LoopKey::default(),
                next: null_he,
                prev: null_he,
            },
            prov(),
        )
    };
    // Top cap: ht[i] runs t[i] → t[i+1].
    let ht: Vec<HalfEdgeKey> = (0..n).map(|i| half_edge(et[i], t[i])).collect();
    // Bottom cap: hb[i] runs u[i+1] → u[i] (edge eb[i] backward).
    let hb: Vec<HalfEdgeKey> = (0..n).map(|i| half_edge(eb[i], u[(i + 1) % n])).collect();
    // Side quad i: u[i] → u[i+1] → t[i+1] → t[i].
    let s0: Vec<HalfEdgeKey> = (0..n).map(|i| half_edge(eb[i], u[i])).collect();
    let s1: Vec<HalfEdgeKey> = (0..n)
        .map(|i| half_edge(ev[(i + 1) % n], u[(i + 1) % n]))
        .collect();
    let s2: Vec<HalfEdgeKey> = (0..n).map(|i| half_edge(et[i], t[(i + 1) % n])).collect();
    let s3: Vec<HalfEdgeKey> = (0..n).map(|i| half_edge(ev[i], t[i])).collect();

    // Loops and faces.
    let mut cycle_loop = |first: HalfEdgeKey| {
        body.add_loop(
            Loop {
                boundary: LoopBoundary::Cycle { first },
                face: FaceKey::default(),
            },
            prov(),
        )
    };
    let loop_top = cycle_loop(ht[0]);
    let loop_bottom = cycle_loop(hb[0]);
    let loop_side: Vec<LoopKey> = (0..n).map(|i| cycle_loop(s0[i])).collect();

    let mut face = |surface: SurfaceKey, outer: LoopKey| {
        body.add_face(
            Face {
                sense: true,
                surface,
                outer,
                rings: vec![],
                shell,
            },
            prov(),
        )
    };
    let face_top = face(surface_top, loop_top);
    let face_bottom = face(surface_bottom, loop_bottom);
    let face_side: Vec<FaceKey> = (0..n)
        .map(|i| face(surface_side[i], loop_side[i]))
        .collect();

    body.get_loop_mut(loop_top).unwrap().face = face_top;
    body.get_loop_mut(loop_bottom).unwrap().face = face_bottom;
    for i in 0..n {
        body.get_loop_mut(loop_side[i]).unwrap().face = face_side[i];
    }
    {
        let faces = &mut body.get_shell_mut(shell).unwrap().faces;
        faces.push(face_top);
        faces.push(face_bottom);
        faces.extend(&face_side);
    }

    // Close the cycles.
    let link = |body: &mut Body<f64>, cycle: &[HalfEdgeKey], parent: LoopKey| {
        let len = cycle.len();
        for (i, &he) in cycle.iter().enumerate() {
            let h = body.get_half_edge_mut(he).unwrap();
            h.parent_loop = parent;
            h.next = cycle[(i + 1) % len];
            h.prev = cycle[(i + len - 1) % len];
        }
    };
    link(&mut body, &ht, loop_top);
    // Bottom cap walks hb in DECREASING index order (see the struct
    // docs), so the "cycle" slice is reversed.
    let hb_walk: Vec<HalfEdgeKey> = hb.iter().rev().copied().collect();
    link(&mut body, &hb_walk, loop_bottom);
    for i in 0..n {
        link(&mut body, &[s0[i], s1[i], s2[i], s3[i]], loop_side[i]);
    }

    // Claim the halves (plus = intrinsic direction, per the struct docs).
    for i in 0..n {
        let e = body.get_edge_mut(et[i]).unwrap();
        e.he_plus = ht[i];
        e.he_minus = s2[i];
        let e = body.get_edge_mut(eb[i]).unwrap();
        e.he_plus = s0[i];
        e.he_minus = hb[i];
        let e = body.get_edge_mut(ev[i]).unwrap();
        e.he_plus = s1[(i + n - 1) % n];
        e.he_minus = s3[i];
    }

    // Anchor the vertices.
    for i in 0..n {
        body.get_vertex_mut(t[i]).unwrap().emanating = Some(ht[i]);
        body.get_vertex_mut(u[i]).unwrap().emanating = Some(s0[i]);
    }

    RawPrism {
        body,
        t,
        u,
        ht,
        hb,
        s0,
        s1,
        s2,
        s3,
        et,
        eb,
        ev,
        loop_top,
        loop_bottom,
        loop_side,
        face_top,
        face_bottom,
        face_side,
        shell,
        solid,
    }
}

/// Key bundle for [`mvfs_state`].
#[allow(dead_code)] // key bundles expose every minted key; tests pick what they need
pub(crate) struct MvfsState {
    pub body: Body<f64>,
    pub point: PointKey,
    pub surface: SurfaceKey,
    pub vertex: VertexKey,
    pub lone_loop: LoopKey,
    pub face: FaceKey,
    pub shell: ShellKey,
    pub solid: SolidKey,
}

/// Builds the skeletal `mvfs` state: solid + shell + one face whose
/// outer loop is [`LoopBoundary::Empty`], holding a lone vertex
/// (`emanating: None`). No edges, no half-edges. Tier-1-legal by design
/// — this is the state every Euler construction starts from.
pub(crate) fn mvfs_state() -> MvfsState {
    let mut body = Body::<f64>::new();
    let point = body.add_point(Point3::origin());
    let vertex = body.add_vertex(
        Vertex {
            point,
            emanating: None,
        },
        prov(),
    );
    let solid = body.add_solid(Solid { shells: vec![] }, prov());
    let shell = body.add_shell(
        Shell {
            faces: vec![],
            solid,
        },
        prov(),
    );
    body.get_solid_mut(solid).unwrap().shells.push(shell);
    let surface = body.add_surface(test_surface(Point3::origin()));
    let lone_loop = body.add_loop(
        Loop {
            boundary: LoopBoundary::Empty { vertex },
            face: FaceKey::default(),
        },
        prov(),
    );
    let face = body.add_face(
        Face {
            sense: true,
            surface,
            outer: lone_loop,
            rings: vec![],
            shell,
        },
        prov(),
    );
    body.get_loop_mut(lone_loop).unwrap().face = face;
    body.get_shell_mut(shell).unwrap().faces.push(face);

    MvfsState {
        body,
        point,
        surface,
        vertex,
        lone_loop,
        face,
        shell,
        solid,
    }
}

// ---------------------------------------------------------------------
// Operator-built fixtures (M1 PR 4): the acceptance-test bodies rebuilt
// through the public Euler operators, in-crate, for the kill-direction
// unit tests, the isomorphism-oracle tests, and the teardown property
// test (which needs crate access to the provenance maps). The cube each
// one grows from is `test_support_fixtures::declined_cube` and each hole
// is `test_support_fixtures::drill_hole`, neither a sequence written
// here; what is written here is where each hole goes.
// ---------------------------------------------------------------------

/// Key bundle for [`ops_holed_box`].
#[allow(dead_code)] // key bundles expose every minted key; tests pick what they need
pub(crate) struct OpsHoledBox {
    pub body: Body<f64>,
    pub seed: MvfsCreated,
    pub box_mevs: [MevCreated; 7],
    pub box_mefs: [MefCreated; 5],
    pub strut: MevCreated,
    pub kill: KemrResult,
    pub rim_mevs: [MevCreated; 3],
    pub mef_top: MefCreated,
    pub tube_mevs: [MevCreated; 4],
    pub tube_mefs: [MefCreated; 4],
    pub plug: KfmrhResult,
}

/// Builds the box with a square through-hole (genus 1) through the
/// operators — the §9.3-minimal 1 mvfs + 15 mev + 10 mef + 1 kemr +
/// 1 kfmrh, the construction of the PR 3 acceptance test on the unit
/// cube instead of the 2×2×2 box: [`declined_cube`], then
/// [`crate::test_support_fixtures::drill_hole`] from the top face to
/// the bottom.
pub(crate) fn ops_holed_box(tol: Tol) -> OpsHoledBox {
    let CubeOps {
        mut body,
        seed,
        mevs,
        mefs,
    } = declined_cube::<f64>(tol);
    // The anchor is the front face's plus half, which lies in the top
    // face's loop; the hole leaves through the bottom cap.
    let hole = drill_hole(
        &mut body,
        mefs[1].he_plus,
        mefs[0].face,
        &[
            Point3::new(0.25, 0.25, 1.0),
            Point3::new(0.75, 0.25, 1.0),
            Point3::new(0.75, 0.75, 1.0),
            Point3::new(0.25, 0.75, 1.0),
        ],
        &[
            Point3::new(0.25, 0.25, 0.0),
            Point3::new(0.75, 0.25, 0.0),
            Point3::new(0.75, 0.75, 0.0),
            Point3::new(0.25, 0.75, 0.0),
        ],
        tol,
    );
    assert_eq!(crate::validate::validate(&body), Ok(()));
    // Infallible, all three: the rim above is a literal of four
    // corners, so `drill_hole` returns n − 1 = 3 rim edges, n = 4 drops
    // and n = 4 walls.
    OpsHoledBox {
        body,
        seed,
        box_mevs: mevs,
        box_mefs: mefs,
        strut: hole.ring.strut,
        kill: hole.ring.kill,
        rim_mevs: hole.ring.rim.try_into().expect("n = 4"),
        mef_top: hole.ring.membrane,
        tube_mevs: hole.drops.try_into().expect("n = 4"),
        tube_mefs: hole.walls.try_into().expect("n = 4"),
        plug: hole.plug,
    }
}

/// Builds the genus-2 double-hole body: [`ops_holed_box`] plus a
/// triangular through-hole drilled front → back
/// ([`crate::test_support_fixtures::drill_hole`] again, entering at the
/// front face's first half-edge). Euler ledger check inside:
/// v − e + f − r = 22 − 33 + 13 − 4 = −2 = 2(1 − 2).
pub(crate) fn ops_genus2(tol: Tol) -> Body<f64> {
    let t = ops_holed_box(tol);
    let mut body = t.body;
    let f_front = t.box_mefs[1].face;
    let f_back = t.box_mefs[3].face;
    let front_outer = body.get_face(f_front).unwrap().outer;
    let LoopBoundary::Cycle { first: at } = body.get_loop(front_outer).unwrap().boundary else {
        panic!("front outer is a cycle");
    };
    drill_hole(
        &mut body,
        at,
        f_back,
        &[
            Point3::new(0.3, 0.0, 0.3),
            Point3::new(0.7, 0.0, 0.3),
            Point3::new(0.5, 0.0, 0.7),
        ],
        &[
            Point3::new(0.3, 1.0, 0.3),
            Point3::new(0.7, 1.0, 0.3),
            Point3::new(0.5, 1.0, 0.7),
        ],
        tol,
    );
    // Genus-2 checkpoint.
    let counts = euler_counts(&body);
    assert_eq!(
        (counts.v, counts.e, counts.f, counts.r, counts.s),
        (22, 33, 13, 4, 1)
    );
    assert_eq!(counts.genus(), Ok(2), "genus 2");
    assert_eq!(crate::validate::validate(&body), Ok(()));
    body
}

/// Key bundle for [`ops_ring_bridge`].
#[allow(dead_code)] // key bundles expose every minted key; tests pick what they need
pub(crate) struct OpsRingBridge {
    pub body: Body<f64>,
    /// The face whose outer loop carries the bridge — the holed box's
    /// top face, the one [`ops_holed_box`] leaves holding the hole rim
    /// as a ring.
    pub face: FaceKey,
    /// That face's outer loop: after the bridge, the merged cycle
    /// holding the former rim, the two bridge halves and the former
    /// outer.
    pub outer: LoopKey,
    /// The bridge edge. Both of its halves lie in
    /// [`OpsRingBridge::outer`], which is the shape [`Body::kemr`]
    /// requires of its two arguments.
    pub bridge: MekrResult,
}

/// Builds the holed box with its top face's hole rim **joined back into
/// that face's outer loop** by one `mekr` — [`ops_holed_box`] plus one
/// operator, so genus and shell count are unchanged and the body still
/// validates.
///
/// **The shape [`Body::kemr`] needs, which no other fixture here
/// presents.** `kemr` takes two halves of ONE edge lying in ONE loop;
/// every edge of [`crate::test_support_fixtures::declined_cube`], [`ops_holed_box`] and [`ops_genus2`]
/// borders two distinct faces, so its halves sit in two loops and
/// `kemr`'s plan phase refuses at `NotSameLoop` on every pair those
/// bodies present. A bridge edge is the M1 shape whose two halves share
/// a loop, and `mekr` is the door that makes one.
///
/// Both components of the split are **non-empty** — the rim halves on
/// one side of the bridge, the former outer's on the other — so `kemr`
/// here runs both of its `link_half_edges` splices rather than the one
/// a strut kill (whose ring side is empty) reaches.
pub(crate) fn ops_ring_bridge(tol: Tol) -> OpsRingBridge {
    let t = ops_holed_box(tol);
    let mut body = t.body;
    let face = t.seed.face;
    let (outer, rings) = {
        let data = body.get_face(face).unwrap();
        (data.outer, data.rings.clone())
    };
    assert_eq!(
        rings.len(),
        1,
        "the holed box's top face carries exactly the hole rim as a ring"
    );
    let LoopBoundary::Cycle { first: target } = body.get_loop(outer).unwrap().boundary else {
        panic!("the top face's outer loop is a cycle");
    };
    let LoopBoundary::Cycle { first: rim } = body.get_loop(rings[0]).unwrap().boundary else {
        panic!("the hole rim is a cycle");
    };
    let bridge = body
        .mekr_chord(MekrSite::Cycles { target, ring: rim }, tol)
        .unwrap();
    // The property the fixture exists for, asserted here so a change to
    // `mekr`'s splice cannot leave a consumer silently back at
    // `NotSameLoop`: the bridge's two halves share one loop, and it is
    // the face's outer.
    for half in [bridge.he_plus, bridge.he_minus] {
        assert_eq!(
            body.get_half_edge(half).unwrap().parent_loop,
            outer,
            "the bridge's halves must both lie in the merged outer loop"
        );
    }
    assert!(
        body.get_face(face).unwrap().rings.is_empty(),
        "the bridge consumed the top face's only ring"
    );
    assert_eq!(crate::validate::validate(&body), Ok(()));
    OpsRingBridge {
        body,
        face,
        outer,
        bridge,
    }
}

/// Key bundle for [`ops_strut_cube`].
#[allow(dead_code)] // key bundles expose every minted key; tests pick what they need
pub(crate) struct OpsStrutCube {
    pub body: Body<f64>,
    /// The loop the strut hangs in — the seed (top) face's outer loop,
    /// the same loop [`ops_holed_box`] plants its hole anchor in.
    pub outer: LoopKey,
    /// The pendant edge. Its two halves lie in
    /// [`OpsStrutCube::outer`] and are **adjacent** there, which is the
    /// shape whose [`Body::kemr`] leaves an EMPTY ring side and
    /// therefore runs one splice rather than two.
    pub strut: MevCreated,
}

/// Builds the cube with one pendant strut planted on the top face's
/// outer loop — [`crate::test_support_fixtures::declined_cube`] plus one `mev_line` at a `Fan` site, which
/// is the state [`ops_holed_box`] passes through at its hole anchor and
/// kills with `kemr` in the next line.
///
/// **The shape whose `kemr` empties the ring side.** `kemr` splits its
/// loop's cycle at the two halves it is handed; when they are ADJACENT
/// the side strictly between them is empty, the ring loop is minted
/// `Empty` and only the old loop's splice runs. [`ops_ring_bridge`]'s
/// bridge edge is the other arm — both sides non-empty, both splices —
/// so the two fixtures together present both shapes of `kemr`'s
/// mutation phase.
pub(crate) fn ops_strut_cube(tol: Tol) -> OpsStrutCube {
    let t = declined_cube::<f64>(tol);
    let mut body = t.body;
    // The same site `ops_holed_box` plants its hole anchor at: a `Fan`
    // on the front face's plus half, which lies in the top face's loop.
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: t.mefs[1].he_plus,
                he2: t.mefs[1].he_plus,
            },
            Point3::new(0.25, 0.25, 1.0),
            tol,
        )
        .unwrap();
    let outer = body.get_half_edge(strut.he_plus).unwrap().parent_loop;
    // The property the fixture exists for, asserted here rather than
    // described: the halves share a loop AND follow one another in it,
    // so `kemr`'s ring side is the empty one.
    assert_eq!(
        body.get_half_edge(strut.he_minus).unwrap().parent_loop,
        outer,
        "the strut's halves must both lie in the loop it was planted in"
    );
    assert_eq!(
        body.get_half_edge(strut.he_plus).unwrap().next,
        strut.he_minus,
        "the strut's halves must be adjacent, or `kemr` splits off a cycle here"
    );
    assert_eq!(crate::validate::validate(&body), Ok(()));
    OpsStrutCube { body, outer, strut }
}

/// The segment body: `mvfs` at the origin, then `mev_line` at its
/// `Lone` site to `(1, 0, 0)` — one loop `[he_plus, he_minus]`.
pub(crate) fn ops_segment(tol: Tol) -> (Body<f64>, MvfsCreated, MevCreated) {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
    let site = MevSite::Lone {
        r#loop: seed.r#loop,
    };
    let seg = body
        .mev_line(site, Point3::new(1.0, 0.0, 0.0), tol)
        .unwrap();
    (body, seed, seg)
}

/// [`ops_segment`] with one strut at its far vertex, to `(2, 0, 0)`:
/// cycle `[seg+, strut+, strut−, seg−]`. Returns the body, the seed, the
/// segment and the strut.
pub(crate) fn ops_strutted(tol: Tol) -> (Body<f64>, MvfsCreated, MevCreated, MevCreated) {
    let (mut body, seed, seg) = ops_segment(tol);
    let site = MevSite::Fan {
        he1: seg.he_minus,
        he2: seg.he_minus,
    };
    let strut = body
        .mev_line(site, Point3::new(2.0, 0.0, 0.0), tol)
        .unwrap();
    (body, seed, seg, strut)
}

// ---------------------------------------------------------------------
// The offset-fit door's subject
// ---------------------------------------------------------------------

/// **Two offset certificates agree, limb for limb, by bits.**
///
/// The five limbs are the whole certificate's numeric content, and a
/// row that compares four of them is a row with a hole in it — which
/// is why this is one function rather than a loop each caller writes.
/// `what` names the pair so a failure says which comparison broke.
pub(crate) fn assert_certificates_agree(
    what: &str,
    got: &geom::OffsetCertificate,
    expected: &geom::OffsetCertificate,
) {
    for (limb, a, b) in [
        ("distance", got.distance, expected.distance),
        ("on_locus_max", got.on_locus_max, expected.on_locus_max),
        ("hull_sup", got.hull_sup, expected.hull_sup),
        ("normal_floor", got.normal_floor, expected.normal_floor),
        (
            "curvature_reach",
            got.curvature_reach,
            expected.curvature_reach,
        ),
    ] {
        assert_eq!(
            a.to_bits(),
            b.to_bits(),
            "{what}: {limb} moved behind the door ({a:e} vs {b:e})"
        );
    }
    assert_eq!(
        (got.cells, got.samples),
        (expected.cells, expected.samples),
        "{what}: the schedule moved"
    );
}

/// A gently bowed polynomial patch over `[0,1]²` — a base whose offset
/// is genuinely not a NURBS, so the fit has real work to do.
///
/// **The bow is the largest one that certifies at every eps row, and
/// that is what picks it.** This patch goes through the `Tol` door, so
/// its fit target is the RUN's ε, and the gate commits three rows —
/// `1e-6`, the default `1e-9` and `1e-12`
/// (`.github/workflows/ci.yml`, `EPS_ROWS`). Measured through the mint
/// door at `d = 0.05`:
///
/// | bow | 1e-6 | 1e-9 | 1e-12 |
/// |---|---|---|---|
/// | `0.15` | `rounds 0`, `hull_sup 4.4e-7` | `rounds 3`, `9.3e-10` | budget exhausted at `3.3e-10` |
/// | `1.5e-2` | `rounds 0`, `4.2e-11` | `rounds 0`, `4.2e-11` | `rounds 3`, `8.9e-13` |
/// | `1.5e-5` | `rounds 0`, `2.9e-15` | `rounds 0`, `2.9e-15` | `rounds 0`, `2.9e-15` |
///
/// **No bow refines at every row.** The refinement loop runs only when
/// the unrefined residual is above the target, so running it at `1e-6`
/// wants a residual above `1e-6`, while certifying at `1e-12` wants
/// the converged residual below `1e-12` — and the loop saturates near
/// `3e-10` on this geometry (six rounds, a 27×27 grid), so the two
/// cannot both hold. What IS available is a certificate whose limbs
/// are measurements rather than f64 rounding noise at every row, and
/// a loop that runs at the tightest one; `1.5e-2` is the largest bow
/// with both, and `curvature_reach` at it is `1.0e2` rather than the
/// `1.0e5` a hair-thin bow reports.
pub(crate) fn bowed_patch() -> geom::NurbsSurface<f64> {
    /// The bow's amplitude in `u`; the `v` bow is two thirds of it.
    const BOW: f64 = 1.5e-2;
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut control = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            let (u, v) = (f64::from(i) * 0.5, f64::from(j) * 0.5);
            control.push(Point3::new(
                u,
                v,
                BOW * u * (1.0 - u) + (BOW * 2.0 / 3.0) * v * v,
            ));
        }
    }
    geom::NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 9]).unwrap()
}

/// The certified `Approx` surface of [`bowed_patch`]'s `+0.05` offset,
/// at the RUN's tolerance — the smallest subject that reaches the
/// offset-fit door, lifted to `T` verbatim
/// (`geom::ApproxSurface::map_scalar`, which carries the stored
/// certificate rather than re-deriving it, so the lift is available at
/// scalars that have no fit).
pub(crate) fn bowed_offset_approx<T: geom_core::Real>() -> geom::ApproxSurface<T> {
    let tol = Tol::witness();
    let approx = geom_brep::approx_offset_surface(std::sync::Arc::new(bowed_patch()), 0.05, tol)
        .expect("the bowed patch's offset fits at every eps row the gate commits");
    approx.map_scalar(T::from_f64)
}

/// The `mvfs` seed body with [`bowed_offset_approx`] on its one face —
/// the smallest body whose check-1 walk reaches the offset-fit door.
pub(crate) fn approx_faced_body<T: geom_core::Decide>() -> (Body<T>, FaceKey) {
    let mut body = Body::<T>::new();
    let created = body
        .mvfs(Point3::new(T::zero(), T::zero(), T::zero()), true)
        .expect("mvfs has no preconditions");
    body.set_face_surface(
        created.face,
        crate::euler::FaceSurface::New {
            surface: geom::Surface::Approx(std::sync::Arc::new(bowed_offset_approx::<T>())),
            sense: true,
        },
    )
    .expect("the seed face takes a fresh surface");
    (body, created.face)
}

mod tests {
    use super::*;

    /// Removes every arena entry `original` does not hold, bumping the
    /// version of each slot it frees.
    fn drop_entries_not_in(body: &mut Body<f64>, original: &Body<f64>) {
        fn keep<K: slotmap::Key, V, W>(
            arena: &mut slotmap::SlotMap<K, V>,
            original: &slotmap::SlotMap<K, W>,
        ) {
            arena.retain(|k, _| original.contains_key(k));
        }
        keep(&mut body.solids, &original.solids);
        keep(&mut body.shells, &original.shells);
        keep(&mut body.faces, &original.faces);
        keep(&mut body.loops, &original.loops);
        keep(&mut body.half_edges, &original.half_edges);
        keep(&mut body.edges, &original.edges);
        keep(&mut body.vertices, &original.vertices);
        keep(&mut body.points, &original.points);
        keep(&mut body.curves, &original.curves);
        keep(&mut body.surfaces, &original.surfaces);
    }

    /// Every record the snapshot claims to walk moves it: a new entry in
    /// each of the ten arenas, then a provenance record for each new
    /// topology entry, and, on its own, the key slot that entry consumes
    /// once it is removed again. A walk that drops an arena, a
    /// provenance lookup or an arena's next key leaves that row's
    /// snapshot unmoved.
    #[test]
    fn deep_snapshot_sees_every_arena_and_provenance_record() {
        let s = mvfs_state();
        let before = deep_snapshot(&s.body);
        type Insert<'a> = Box<dyn Fn(&mut Body<f64>) -> Option<EntityId> + 'a>;
        let rows: [(&str, Insert); 10] = [
            (
                "solid",
                Box::new(|b| Some(EntityId::Solid(b.solids.insert(Solid { shells: vec![] })))),
            ),
            (
                "shell",
                Box::new(|b| {
                    let shell = Shell {
                        faces: vec![],
                        solid: s.solid,
                    };
                    Some(EntityId::Shell(b.shells.insert(shell)))
                }),
            ),
            (
                "face",
                Box::new(|b| {
                    let face = Face {
                        sense: true,
                        surface: s.surface,
                        outer: s.lone_loop,
                        rings: vec![],
                        shell: s.shell,
                    };
                    Some(EntityId::Face(b.faces.insert(face)))
                }),
            ),
            (
                "loop",
                Box::new(|b| {
                    let loop_ = Loop {
                        boundary: LoopBoundary::Empty { vertex: s.vertex },
                        face: s.face,
                    };
                    Some(EntityId::Loop(b.loops.insert(loop_)))
                }),
            ),
            (
                "half-edge",
                Box::new(|b| {
                    let he = HalfEdge {
                        edge: EdgeKey::default(),
                        start: s.vertex,
                        parent_loop: s.lone_loop,
                        next: HalfEdgeKey::default(),
                        prev: HalfEdgeKey::default(),
                    };
                    Some(EntityId::HalfEdge(b.half_edges.insert(he)))
                }),
            ),
            (
                "edge",
                Box::new(|b| {
                    let edge = Edge {
                        he_plus: HalfEdgeKey::default(),
                        he_minus: HalfEdgeKey::default(),
                        curve: CurveKey::default(),
                    };
                    Some(EntityId::Edge(b.edges.insert(edge)))
                }),
            ),
            (
                "vertex",
                Box::new(|b| {
                    let vertex = Vertex {
                        point: s.point,
                        emanating: None,
                    };
                    Some(EntityId::Vertex(b.vertices.insert(vertex)))
                }),
            ),
            (
                "point",
                Box::new(|b| {
                    b.points.insert(Point3::new(1.0, 2.0, 3.0));
                    None
                }),
            ),
            (
                "curve",
                Box::new(|b| {
                    let curve = test_curve(Point3::origin(), Tol::witness());
                    b.curves.insert(crate::null::CurveGeom::Certified(curve));
                    None
                }),
            ),
            (
                "surface",
                Box::new(|b| {
                    b.surfaces.insert(test_surface(Point3::origin()));
                    None
                }),
            ),
        ];

        for (arena, insert) in &rows {
            let mut churned = s.body.clone();
            insert(&mut churned);
            drop_entries_not_in(&mut churned, &s.body);
            assert_eq!(
                arena_snapshot(&churned),
                arena_snapshot(&s.body),
                "{arena}: the churn leaves every arena its length"
            );
            assert_ne!(
                deep_snapshot(&churned),
                before,
                "{arena}: an entry minted and removed leaves the snapshot unmoved"
            );

            let mut body = s.body.clone();
            let entity = insert(&mut body);
            let with_entry = deep_snapshot(&body);
            assert_ne!(
                with_entry, before,
                "{arena}: a new entry leaves the snapshot unmoved"
            );
            let Some(entity) = entity else { continue };
            let record = Provenance::Primordial { op: "snapshot row" };
            match entity {
                EntityId::Solid(k) => body.solid_provenance.insert(k, record),
                EntityId::Shell(k) => body.shell_provenance.insert(k, record),
                EntityId::Face(k) => body.face_provenance.insert(k, record),
                EntityId::Loop(k) => body.loop_provenance.insert(k, record),
                EntityId::HalfEdge(k) => body.half_edge_provenance.insert(k, record),
                EntityId::Edge(k) => body.edge_provenance.insert(k, record),
                EntityId::Vertex(k) => body.vertex_provenance.insert(k, record),
            };
            assert_ne!(
                deep_snapshot(&body),
                with_entry,
                "{arena}: a provenance record leaves the snapshot unmoved"
            );
        }
    }

    /// Every side table the snapshot claims to walk moves it: one new
    /// row in each, on a key that table holds no row for. A walk that
    /// drops a table leaves that row's snapshot unmoved. (The seven
    /// provenance maps are the arena row's second half.)
    #[test]
    fn deep_snapshot_sees_every_side_table_row() {
        fn fresh<K: slotmap::Key>(taken: impl Fn(K) -> bool) -> K {
            let mut keys = slotmap::SlotMap::<K, ()>::with_key();
            std::iter::repeat_with(|| keys.insert(()))
                .find(|&k| !taken(k))
                .expect("an unbounded key supply")
        }
        let s = mvfs_state();
        let cache = {
            let mut sheet = Body::<f64>::new();
            crate::test_support_fixtures::cyl_wall_sheet(
                &mut sheet,
                crate::test_support_fixtures::CylFrame::canonical(1.0),
                None,
                (0.0, 1.0),
                (0.0, 1.0),
                Tol::witness(),
            );
            let (_, cache) = sheet
                .pcurves
                .iter()
                .next()
                .expect("the wall sheet mints pcurves");
            cache.clone()
        };
        let before = deep_snapshot(&s.body);
        type Insert<'a> = Box<dyn Fn(&mut Body<f64>) + 'a>;
        let rows: [(&str, Insert); 7] = [
            (
                "pcurves",
                Box::new(|b| {
                    let k = fresh(|k| b.pcurves.contains_key(k));
                    b.pcurves.insert(k, cache.clone());
                }),
            ),
            (
                "null_faces",
                Box::new(|b| {
                    let k = fresh(|k| b.null_faces.contains_key(k));
                    let pair = crate::null::NullFacePair::Split {
                        above_loop: s.lone_loop,
                        below_loop: s.lone_loop,
                    };
                    b.null_faces.insert(k, pair);
                }),
            ),
            (
                "point_origins",
                Box::new(|b| {
                    let k = fresh(|k| b.point_origins.contains_key(k));
                    b.point_origins.insert(k, crate::GeomOrigin::Imported);
                }),
            ),
            (
                "curve_origins",
                Box::new(|b| {
                    let k = fresh(|k| b.curve_origins.contains_key(k));
                    b.curve_origins.insert(k, crate::GeomOrigin::Imported);
                }),
            ),
            (
                "surface_origins",
                Box::new(|b| {
                    let k = fresh(|k| b.surface_origins.contains_key(k));
                    b.surface_origins.insert(k, crate::GeomOrigin::Imported);
                }),
            ),
            (
                "surface_field_sources",
                Box::new(|b| {
                    let k = fresh(|k| b.surface_field_sources.contains_key(k));
                    b.surface_field_sources
                        .insert(k, crate::param_source::FieldSources::default());
                }),
            ),
            (
                "surface_axis_sources",
                Box::new(|b| {
                    let k = fresh(|k| b.surface_axis_sources.contains_key(k));
                    b.surface_axis_sources.insert(k, crate::AxisRecord::Cleared);
                }),
            ),
        ];
        for (table, insert) in &rows {
            let mut body = s.body.clone();
            insert(&mut body);
            assert_ne!(
                deep_snapshot(&body),
                before,
                "{table}: a new row leaves the snapshot unmoved"
            );
        }
    }
}
