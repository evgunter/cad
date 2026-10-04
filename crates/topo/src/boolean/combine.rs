//! **The combine door** — the single sanctioned CrossSolid-adjacent
//! operation (the boundary PR 1 ratified and this PR was required to
//! name): bringing the selected components of TWO bodies into ONE
//! result body. The Euler layer's `kfmrh` refuses cross-SOLID fusion
//! ([`EulerOpError::CrossSolid`](crate::euler::EulerOpError)) because
//! *combining bodies is the boolean pipeline's job*; this module is
//! that job's implementation.
//!
//! # Contract (the door, precisely)
//!
//! `graft_solid(dst, dst_solid, src)` transplants the single solid of
//! `src` into `dst`:
//!
//! - Every geometry and topology entity of `src` is **re-created in
//!   `dst`'s arenas under fresh keys** (keys are body-lineage-scoped;
//!   a cross-body key can never be carried over), walking each arena
//!   in deterministic slot order (D9: byte-identical replay).
//! - `src`'s shells are appended to `dst_solid`'s shell list in their
//!   source order — the movefac semantics of ch. 15's `setopfinish`
//!   across the reduction's annotated bodies: components arrive whole;
//!   no Euler surgery happens here (the seam zip afterwards uses the
//!   now-ordinary same-solid `kfmrh` + `loopglue`).
//! - **Provenance records are transplanted with their payload keys
//!   forwarded.** A graft is not a re-birth — each entity keeps the
//!   record of the operation that created it — but a payload key is
//!   body-lineage-scoped, so a source key carried over would name
//!   whatever holds that key in `dst`. A key the source still holds
//!   forwards to its result key; a key the source no longer holds
//!   (records are historical: a split's parent, a `kemr`'s killed
//!   halves) forwards to a result key that is dead on arrival —
//!   minted and removed in `dst`'s arena, so it resolves nowhere and
//!   slot versioning keeps any later insertion from taking it — one
//!   per source key, so records that named one dead source entity
//!   still name one result key. A split lineage therefore chases
//!   inside `dst` to the root it reached in `src`.
//! - **A refusal writes nothing.** The transplant is staged in a fresh
//!   body and committed into `dst` only once it has succeeded in full
//!   (`graft_staged`).
//! - The returned [`GraftMap`] is the ONLY bridge between source keys
//!   and result keys; downstream consumers (the seam zip's
//!   record-keyed correspondence, contact-record remapping) read it
//!   as data — never key equality across bodies.
//!
//! The result body afterwards holds one solid with shells from both
//! operands: exactly the tier-2-legal multi-shell mid-state the seam
//! zip consumes (or, for fallback results — disjoint unions, voids —
//! the finished multi-shell body itself).

use std::collections::BTreeMap;

use slotmap::{Key, SecondaryMap, SlotMap};

use super::BooleanError;
use crate::body::Body;
use crate::entity::{
    EdgeKey, EntityId, FaceKey, GeomRef, HalfEdgeKey, LoopBoundary, LoopKey, ShellKey, SolidKey,
    VertexKey,
};
use crate::geometry::{CurveKey, PointKey, SurfaceKey};
use crate::live::NAMES_ONLY_LIVE;
use crate::null::CurveGeom;
use geom_core::Tol;

/// The source→result key bridge (module docs). Only the maps the
/// pipeline consumes are exposed; the rest are internal to the graft.
#[derive(Debug, Default)]
pub(crate) struct GraftMap {
    /// Source vertex → result vertex.
    pub vertices: SecondaryMap<VertexKey, VertexKey>,
    /// Source face → result face.
    pub faces: SecondaryMap<FaceKey, FaceKey>,
    /// Source edge → result edge (naming emission, M4 PR 3).
    pub edges: SecondaryMap<EdgeKey, EdgeKey>,
    /// Source edge a transplanted record names but the source no
    /// longer holds → the dead result key standing for it (module
    /// docs). The only way from a forwarded record back to an
    /// ancestor that died before the graft.
    pub dead_edges: BTreeMap<EdgeKey, EdgeKey>,
    /// Source surface → result surface (M4 PR 5: declared-pair
    /// equivalences ride surfaces — fragments inherit surface keys,
    /// so the surface bridge survives fragment-key churn).
    pub surfaces: SecondaryMap<SurfaceKey, SurfaceKey>,
    /// Source shell → result shell (the void-insertion door's
    /// consumers address the transplanted cavity shells by this).
    pub shells: SecondaryMap<ShellKey, ShellKey>,
}

/// One key kind's record forwarding (module docs): a key the source
/// holds goes through the graft's map; any other gets a key of `arena`
/// that is dead on arrival — minted and removed at once, so it
/// resolves nowhere and slot versioning keeps any later insertion from
/// taking it — memoised, one per source key.
struct KindForward<'g, K: Key, V> {
    live: &'g SecondaryMap<K, K>,
    arena: &'g mut SlotMap<K, V>,
    placeholder: fn() -> V,
    dead: BTreeMap<K, K>,
}

impl<'g, K: Key + Ord, V> KindForward<'g, K, V> {
    fn new(
        live: &'g SecondaryMap<K, K>,
        arena: &'g mut SlotMap<K, V>,
        placeholder: fn() -> V,
    ) -> Self {
        Self {
            live,
            arena,
            placeholder,
            dead: BTreeMap::new(),
        }
    }

    fn key(&mut self, k: K) -> K {
        if let Some(&d) = self.live.get(k) {
            return d;
        }
        let (arena, placeholder) = (&mut *self.arena, self.placeholder);
        *self.dead.entry(k).or_insert_with(|| {
            let d = arena.insert(placeholder());
            arena.remove(d);
            d
        })
    }
}

/// `key`'s image under the graft's `map`, for a key the source record
/// `holder`'s field `field` names. The maps are total over the source's
/// live keys, so a miss is a torn source and panics naming the record
/// (D2 row 4).
#[track_caller]
fn image<K: Key, I: core::fmt::Display>(
    map: &SecondaryMap<K, K>,
    key: K,
    id: fn(K) -> I,
    holder: impl core::fmt::Display,
    field: &str,
) -> K {
    map.get(key)
        .copied()
        .unwrap_or_else(|| torn_source(holder, field, id(key)))
}

/// The panic for a source record whose link does not resolve.
#[track_caller]
fn torn_source(holder: impl core::fmt::Display, field: &str, key: impl core::fmt::Display) -> ! {
    unreachable!(
        "graft source: {holder}'s {field} names {key}, which does not resolve: {NAMES_ONLY_LIVE}"
    )
}

/// `key`'s record in the destination arena it was minted into by this
/// transplant.
#[track_caller]
fn fresh<K: Key, V>(arena: &mut SlotMap<K, V>, key: K) -> &mut V {
    arena.get_mut(key).unwrap_or_else(|| {
        unreachable!("graft: {key:?}, minted into the destination by this call, does not resolve")
    })
}

/// One graft's record forwarding, every key kind a payload carries.
struct Forward<'g> {
    half_edges: KindForward<'g, HalfEdgeKey, crate::entity::HalfEdge>,
    edges: KindForward<'g, EdgeKey, crate::entity::Edge>,
    loops: KindForward<'g, LoopKey, crate::entity::Loop>,
    shells: KindForward<'g, ShellKey, crate::entity::Shell>,
    solids: KindForward<'g, SolidKey, crate::entity::Solid>,
}

impl crate::provenance::ForwardKeys for Forward<'_> {
    fn half_edge(&mut self, k: HalfEdgeKey) -> HalfEdgeKey {
        self.half_edges.key(k)
    }
    fn loop_(&mut self, k: LoopKey) -> LoopKey {
        self.loops.key(k)
    }
    fn edge(&mut self, k: EdgeKey) -> EdgeKey {
        self.edges.key(k)
    }
    fn shell(&mut self, k: ShellKey) -> ShellKey {
        self.shells.key(k)
    }
    fn solid(&mut self, k: SolidKey) -> SolidKey {
        self.solids.key(k)
    }
}

/// Transplants `src`'s single solid into `dst_solid` of `dst`
/// (module docs). `src` is only read: its arenas are walked in slot
/// order, and nothing of it is shared with `dst`.
pub(crate) fn graft_solid<T: geom_core::Decide>(
    dst: &mut Body<T>,
    dst_solid: SolidKey,
    src: &Body<T>,
    tol: Tol,
) -> Result<GraftMap, BooleanError> {
    graft_solids_with(dst, &[dst_solid], src, Bridge::Recertify { tol })
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// The bridge each of this thread's grafts ran, in call order: the
    /// witness a test reads to tell which one a graft took, where both
    /// mint the same certificates.
    static BRIDGES: core::cell::RefCell<Vec<Bridge>> = const { core::cell::RefCell::new(Vec::new()) };
}

/// Drains [`BRIDGES`].
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn take_bridges() -> Vec<Bridge> {
    BRIDGES.with(|b| core::mem::take(&mut *b.borrow_mut()))
}

/// How a transplanted edge DESCRIPTION crosses into the destination's
/// key space (the surface-key remap at the end of the graft).
///
/// `Intersection`/`TangentIntersection`/`Seam`/`IsoCurve` name SURFACE
/// KEYS, which are body-lineage-scoped, so the transplanted copies
/// must name the transplanted surfaces. Two ways to write that, and
/// the difference is which claim the graft makes about the result:
///
/// - [`Bridge::Recertify`] re-runs the certification schedule against
///   the destination's surfaces — what the seam-zip lanes want, whose
///   operands have been through surgery.
/// - [`Bridge::RemapKeys`] carries the source's certificate verbatim
///   with only the handles rewritten
///   ([`geom_brep::EdgeCurve::with_remapped_surfaces`]) — what a
///   DISJOINT graft wants, where the transplanted geometry is bitwise
///   the source's and no surgery happened: an import's placed
///   instances, the void door's reversed cavity
///   ([`super::voids::insert_voids`]), the containment fallback's
///   assembly and a sphere re-cut's rotated shells. It is also the only form that
///   can carry a description the certification lanes cannot express at
///   all (a rational NURBS wall certifies nowhere), or one only a
///   lane certifies (a plane × NURBS `Intersection`), at a scalar that
///   holds none.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Bridge {
    /// Re-run the schedule against the destination (the seam-zip
    /// lanes), at the band of `tol` — the one reader of a tolerance in
    /// the graft.
    Recertify {
        /// The run's tolerance.
        tol: Tol,
    },
    /// Rewrite the handles, keep the source's certificate (disjoint).
    RemapKeys,
}

/// [`graft_solid`] for a source holding N solids, with the description
/// bridge chosen explicitly: `dst_solids`
/// names one destination solid per source solid, **positionally in the
/// source's solid order** (slot order, D9), and the arity must match
/// exactly — a source solid with no destination, or a destination with
/// no source solid, is a caller error, never a guess.
///
/// One pass over the source's arenas serves all N (the arenas are
/// whole-body already; only the shell→solid attachment is per-solid),
/// so the result is entity-for-entity what N separate single-solid
/// grafts would have produced, in the same order.
pub(crate) fn graft_solids_with<T: geom_core::Decide>(
    dst: &mut Body<T>,
    dst_solids: &[SolidKey],
    src: &Body<T>,
    bridge: Bridge,
) -> Result<GraftMap, BooleanError> {
    graft_staged(dst, Targets::Existing(dst_solids), src, bridge).map(|(map, _)| map)
}

/// [`graft_solids_with`] onto destination solids minted here, one per
/// source solid in source order, each carrying its source solid's
/// record forwarded like every other record (module docs). Returns the
/// minted keys beside the map.
pub(crate) fn graft_solids_minted<T: geom_core::Decide>(
    dst: &mut Body<T>,
    src: &Body<T>,
    bridge: Bridge,
) -> Result<(GraftMap, Vec<SolidKey>), BooleanError> {
    graft_staged(dst, Targets::Minted, src, bridge)
}

/// **Every graft is staged: a refusal, or a panic on a torn source,
/// leaves `dst` deep-unchanged.** The transplant runs into a fresh body,
/// which is where every refusal and every source-record panic arises,
/// and only a transplant that succeeded in full is committed: grafted
/// from the stage into `dst` with the stage's certificates carried
/// verbatim. Each destination solid is the caller's to have proven live
/// in `dst` (the void door refuses one that is not, as its argument).
/// The stage is a well-formed body this call built, so the commit
/// cannot refuse; it mints in the stage's slot order, which is the
/// source's, so `dst` ends key for key as a transplant straight into
/// it would leave it. The cost is a second transplant of the source,
/// never a copy of `dst`.
fn graft_staged<T: geom_core::Decide>(
    dst: &mut Body<T>,
    targets: Targets<'_>,
    src: &Body<T>,
    bridge: Bridge,
) -> Result<(GraftMap, Vec<SolidKey>), BooleanError> {
    #[cfg(any(test, feature = "test-support"))]
    BRIDGES.with(|b| b.borrow_mut().push(bridge));
    let mut stage = Body::new();
    let (staged, _) = match targets {
        Targets::Minted => graft_solids_impl(&mut stage, Targets::Minted, src, bridge)?,
        Targets::Existing(dst_solids) => {
            if let Some(dead) = dst_solids.iter().find(|&&k| dst.get_solid(k).is_none()) {
                unreachable!(
                    "graft destination solid {dead:?} does not resolve in the destination: every \
                     caller passes a solid it resolved there"
                );
            }
            // One stand-in per destination solid, in the same order, so
            // the commit lands each staged solid's shells positionally.
            let stand_ins: Vec<SolidKey> = dst_solids
                .iter()
                .map(|_| {
                    stage
                        .solids
                        .insert(crate::entity::Solid { shells: Vec::new() })
                })
                .collect();
            graft_solids_impl(&mut stage, Targets::Existing(&stand_ins), src, bridge)?
        }
    };
    let (committed, solids) = match graft_solids_impl(dst, targets, &stage, Bridge::RemapKeys) {
        Ok(done) => done,
        Err(e) => unreachable!(
            "graft commit refused ({e:?}): the stage is a body this call built from a \
             transplant that succeeded, every reference in it minted by that transplant, \
             and every destination solid was checked live (kernel bug)"
        ),
    };
    Ok((staged.then(&committed), solids))
}

/// A graft unstaged, straight into `dst` (`dst_solids` empty mints):
/// the reference a row holds the staged graft against.
#[cfg(test)]
pub(crate) fn graft_unstaged<T: geom_core::Decide>(
    dst: &mut Body<T>,
    dst_solids: &[SolidKey],
    src: &Body<T>,
) -> Result<(GraftMap, Vec<SolidKey>), BooleanError> {
    let targets = if dst_solids.is_empty() {
        Targets::Minted
    } else {
        Targets::Existing(dst_solids)
    };
    graft_solids_impl(dst, targets, src, Bridge::RemapKeys)
}

impl GraftMap {
    /// The bridge of `self` (source → stage) followed by `next`
    /// (stage → destination). Every stage key `self` names was minted
    /// by the transplant that built the stage, so `next` holds it.
    fn then(&self, next: &Self) -> Self {
        fn chain<K: Key>(a: &SecondaryMap<K, K>, b: &SecondaryMap<K, K>) -> SecondaryMap<K, K> {
            a.iter()
                .map(|(k, &mid)| {
                    let Some(&out) = b.get(mid) else {
                        unreachable!(
                            "graft commit: stage key {mid:?} was minted by the staging \
                             transplant and the commit walks every stage key (kernel bug)"
                        )
                    };
                    (k, out)
                })
                .collect()
        }
        Self {
            vertices: chain(&self.vertices, &next.vertices),
            faces: chain(&self.faces, &next.faces),
            edges: chain(&self.edges, &next.edges),
            dead_edges: self
                .dead_edges
                .iter()
                .map(|(&k, mid)| {
                    let Some(&out) = next.dead_edges.get(mid) else {
                        unreachable!(
                            "graft commit: dead stage edge {mid:?} stands for a source key a \
                             staged record names, and the commit forwards that record again \
                             (kernel bug)"
                        )
                    };
                    (k, out)
                })
                .collect(),
            surfaces: chain(&self.surfaces, &next.surfaces),
            shells: chain(&self.shells, &next.shells),
        }
    }
}

/// Which destination solids a graft's source solids land in.
#[derive(Clone, Copy)]
enum Targets<'a> {
    /// Existing solids, positionally; each keeps its own record.
    Existing(&'a [SolidKey]),
    /// Fresh solids minted by the graft, each taking its source
    /// solid's record.
    Minted,
}

fn graft_solids_impl<T: geom_core::Decide>(
    dst: &mut Body<T>,
    targets: Targets<'_>,
    src: &Body<T>,
    bridge: Bridge,
) -> Result<(GraftMap, Vec<SolidKey>), BooleanError> {
    // Arity is this door's precondition, distinct from corruption: the
    // caller states which destination each source solid lands in, so a
    // count mismatch is a caller error, never a thing to guess at.
    let arity = || BooleanError::JoinDesync {
        what: "graft needs exactly one destination solid per source solid",
    };
    // Source solid → its destination, and the source order to attach
    // in. A shell's owner is its own `solid` back-pointer, so this map
    // is the only thing the shell pass needs.
    let mut solid_map: SecondaryMap<SolidKey, SolidKey> = SecondaryMap::new();
    let mut pairs: Vec<(SolidKey, SolidKey)> = Vec::new();
    // Minted targets: the source solids and the records they will carry,
    // read before anything is written.
    let mut minted: Vec<(SolidKey, &crate::provenance::Provenance)> = Vec::new();
    match targets {
        Targets::Existing(dst_solids) => {
            let mut targets = dst_solids.iter();
            for (k, _) in src.solids() {
                let &target = targets.next().ok_or_else(arity)?;
                solid_map.insert(k, target);
                pairs.push((k, target));
            }
            if targets.next().is_some() {
                return Err(arity());
            }
        }
        Targets::Minted => {
            for (k, _) in src.solids() {
                let Some(p) = src.solid_provenance.get(k) else {
                    unreachable!(
                        "graft source: solid {k:?} carries no provenance: every entity of a \
                         tier-1-valid body does, and every public door keeps the body \
                         tier-1-valid"
                    )
                };
                minted.push((k, p));
            }
        }
    }
    if pairs.is_empty() && minted.is_empty() {
        return Err(BooleanError::JoinDesync {
            what: "graft source holds no solid to graft",
        });
    }

    // ---- Geometry arenas (slot order). ----
    let mut points: SecondaryMap<PointKey, PointKey> = SecondaryMap::new();
    for (k, p) in src.points.iter() {
        let dk = dst.points.insert(*p);
        points.insert(k, dk);
        // The description's provenance row rides every graft: a
        // transplanted description came from where it came from, which
        // no graft changes, and N6's recipe identity is that row's
        // `Recipe` arm (`crate::GeomOrigin`, which states the carry
        // obligation and the totality this reads).
        let Some(origin) = src.point_origins.get(k) else {
            unreachable!(
                "grafted point {k:?} is live in the source body and carries no origin row: \
                 the origin map is total over live keys (kernel bug)"
            )
        };
        dst.point_origins.insert(dk, origin.clone());
    }
    let mut surfaces: SecondaryMap<SurfaceKey, SurfaceKey> = SecondaryMap::new();
    for (k, sfc) in src.surfaces.iter() {
        let dk = dst.surfaces.insert(sfc.clone());
        surfaces.insert(k, dk);
        dst.carry_surface_rows(dk, src, k);
    }

    // ---- Topology arenas, pass 1: clone with source-internal keys
    // (patched in pass 2), recording the fresh keys. ----
    let mut vertices: SecondaryMap<VertexKey, VertexKey> = SecondaryMap::new();
    for (k, v) in src.vertices.iter() {
        let dk = dst.vertices.insert(v.clone());
        vertices.insert(k, dk);
    }
    // Curves after vertices (a NullScaffold payload holds vertex keys;
    // a certified description may hold SURFACE keys — `Intersection`/
    // `Seam` re-certify against the grafted surfaces below, once the
    // topology is patched and endpoints resolve).
    let mut curves: SecondaryMap<CurveKey, CurveKey> = SecondaryMap::new();
    for (k, c) in src.curves.iter() {
        let mapped = match c {
            CurveGeom::Certified(_) => c.clone(),
            CurveGeom::NullScaffold(attr) => {
                let mut attr = *attr;
                let holder = GeomRef::Curve(k);
                attr.below_end = image(
                    &vertices,
                    attr.below_end,
                    EntityId::Vertex,
                    holder,
                    "below_end",
                );
                attr.above_end = image(
                    &vertices,
                    attr.above_end,
                    EntityId::Vertex,
                    holder,
                    "above_end",
                );
                CurveGeom::NullScaffold(attr)
            }
        };
        let dk = dst.curves.insert(mapped);
        curves.insert(k, dk);
        let Some(origin) = src.curve_origins.get(k) else {
            unreachable!(
                "grafted curve {k:?} is live in the source body and carries no origin row: \
                 the origin map is total over live keys (kernel bug)"
            )
        };
        dst.curve_origins.insert(dk, origin.clone());
    }
    let mut half_edges: SecondaryMap<HalfEdgeKey, HalfEdgeKey> = SecondaryMap::new();
    for (k, he) in src.half_edges.iter() {
        let dk = dst.half_edges.insert(he.clone());
        half_edges.insert(k, dk);
    }
    let mut edges: SecondaryMap<EdgeKey, EdgeKey> = SecondaryMap::new();
    for (k, e) in src.edges.iter() {
        let dk = dst.edges.insert(e.clone());
        edges.insert(k, dk);
    }
    let mut loops: SecondaryMap<LoopKey, LoopKey> = SecondaryMap::new();
    for (k, l) in src.loops.iter() {
        let dk = dst.loops.insert(l.clone());
        loops.insert(k, dk);
    }
    let mut faces: SecondaryMap<FaceKey, FaceKey> = SecondaryMap::new();
    for (k, f) in src.faces.iter() {
        let dk = dst.faces.insert(f.clone());
        faces.insert(k, dk);
    }
    let mut shells: SecondaryMap<ShellKey, ShellKey> = SecondaryMap::new();
    for (k, s) in src.shells.iter() {
        let dk = dst.shells.insert(s.clone());
        shells.insert(k, dk);
    }

    // ---- Minted destination solids, in source order. ----
    for &(k, _) in &minted {
        let dk = dst
            .solids
            .insert(crate::entity::Solid { shells: Vec::new() });
        solid_map.insert(k, dk);
        pairs.push((k, dk));
    }

    // ---- Provenance: every record forwarded into `dst`'s keys. ----
    //
    // Three kinds of source row can name a key this graft has no image
    // for, and each gets the treatment its meaning dictates. A
    // provenance PAYLOAD is history: it names the entity an operation
    // acted on, which later surgery may have killed, so a dead payload
    // key is the normal case and forwards to a dead-on-arrival key. A
    // pcurve cache row is derived data keyed by the half-edge it
    // describes; surgery leaves stale rows behind, so a dead key marks
    // the row stale and it is skipped. A null-face row describes a live
    // face's current loops and is removed with its face, so a key
    // missing there means the source is corrupt.
    let mut fwd = Forward {
        half_edges: KindForward::new(&half_edges, &mut dst.half_edges, || {
            crate::entity::HalfEdge {
                edge: EdgeKey::null(),
                start: VertexKey::null(),
                parent_loop: LoopKey::null(),
                next: HalfEdgeKey::null(),
                prev: HalfEdgeKey::null(),
            }
        }),
        edges: KindForward::new(&edges, &mut dst.edges, || crate::entity::Edge {
            he_plus: HalfEdgeKey::null(),
            he_minus: HalfEdgeKey::null(),
            curve: CurveKey::null(),
        }),
        loops: KindForward::new(&loops, &mut dst.loops, || crate::entity::Loop {
            boundary: LoopBoundary::Empty {
                vertex: VertexKey::null(),
            },
            face: FaceKey::null(),
        }),
        shells: KindForward::new(&shells, &mut dst.shells, || crate::entity::Shell {
            faces: Vec::new(),
            solid: SolidKey::null(),
        }),
        solids: KindForward::new(&solid_map, &mut dst.solids, || crate::entity::Solid {
            shells: Vec::new(),
        }),
    };
    for (k, &dk) in vertices.iter() {
        if let Some(p) = src.vertex_provenance.get(k) {
            dst.vertex_provenance.insert(dk, p.forwarded(&mut fwd));
        }
    }
    for (k, &dk) in half_edges.iter() {
        if let Some(p) = src.half_edge_provenance.get(k) {
            dst.half_edge_provenance.insert(dk, p.forwarded(&mut fwd));
        }
    }
    for (k, &dk) in edges.iter() {
        if let Some(p) = src.edge_provenance.get(k) {
            dst.edge_provenance.insert(dk, p.forwarded(&mut fwd));
        }
    }
    for (k, &dk) in loops.iter() {
        if let Some(p) = src.loop_provenance.get(k) {
            dst.loop_provenance.insert(dk, p.forwarded(&mut fwd));
        }
    }
    for (k, &dk) in faces.iter() {
        if let Some(p) = src.face_provenance.get(k) {
            dst.face_provenance.insert(dk, p.forwarded(&mut fwd));
        }
    }
    for (k, &dk) in shells.iter() {
        if let Some(p) = src.shell_provenance.get(k) {
            dst.shell_provenance.insert(dk, p.forwarded(&mut fwd));
        }
    }
    for (&(_, p), &(_, dk)) in minted.iter().zip(&pairs) {
        dst.solid_provenance.insert(dk, p.forwarded(&mut fwd));
    }
    let dead_edges = fwd.edges.dead;

    // ---- Pass 2: patch every cross-reference to result keys. ----
    for (k, &dk) in vertices.iter() {
        let holder = EntityId::Vertex(k);
        let v = fresh(&mut dst.vertices, dk);
        v.point = image(&points, v.point, GeomRef::Point, holder, "point");
        if let Some(e) = v.emanating {
            v.emanating = Some(image(
                &half_edges,
                e,
                EntityId::HalfEdge,
                holder,
                "emanating",
            ));
        }
    }
    for (k, &dk) in half_edges.iter() {
        let holder = EntityId::HalfEdge(k);
        let he = fresh(&mut dst.half_edges, dk);
        he.edge = image(&edges, he.edge, EntityId::Edge, holder, "edge");
        he.start = image(&vertices, he.start, EntityId::Vertex, holder, "start");
        he.parent_loop = image(
            &loops,
            he.parent_loop,
            EntityId::Loop,
            holder,
            "parent_loop",
        );
        he.next = image(&half_edges, he.next, EntityId::HalfEdge, holder, "next");
        he.prev = image(&half_edges, he.prev, EntityId::HalfEdge, holder, "prev");
    }
    for (k, &dk) in edges.iter() {
        let holder = EntityId::Edge(k);
        let e = fresh(&mut dst.edges, dk);
        e.he_plus = image(
            &half_edges,
            e.he_plus,
            EntityId::HalfEdge,
            holder,
            "he_plus",
        );
        e.he_minus = image(
            &half_edges,
            e.he_minus,
            EntityId::HalfEdge,
            holder,
            "he_minus",
        );
        e.curve = image(&curves, e.curve, GeomRef::Curve, holder, "curve");
    }
    for (k, &dk) in loops.iter() {
        let holder = EntityId::Loop(k);
        let l = fresh(&mut dst.loops, dk);
        l.boundary = match l.boundary {
            LoopBoundary::Empty { vertex } => LoopBoundary::Empty {
                vertex: image(&vertices, vertex, EntityId::Vertex, holder, "vertex"),
            },
            LoopBoundary::Cycle { first } => LoopBoundary::Cycle {
                first: image(&half_edges, first, EntityId::HalfEdge, holder, "first"),
            },
        };
        l.face = image(&faces, l.face, EntityId::Face, holder, "face");
    }
    for (k, &dk) in faces.iter() {
        let holder = EntityId::Face(k);
        let f = fresh(&mut dst.faces, dk);
        f.surface = image(&surfaces, f.surface, GeomRef::Surface, holder, "surface");
        f.outer = image(&loops, f.outer, EntityId::Loop, holder, "outer");
        for r in &mut f.rings {
            *r = image(&loops, *r, EntityId::Loop, holder, "rings");
        }
        f.shell = image(&shells, f.shell, EntityId::Shell, holder, "shell");
    }
    for (sk, &dk) in shells.iter() {
        // A shell lands under the destination of the solid that owns
        // it in the source — its own back-pointer, so a multi-solid
        // source keeps every shell with its solid.
        let holder = EntityId::Shell(sk);
        let owner = src.shells[sk].solid;
        let target = image(&solid_map, owner, EntityId::Solid, holder, "solid");
        let s = fresh(&mut dst.shells, dk);
        for f in &mut s.faces {
            *f = image(&faces, *f, EntityId::Face, holder, "faces");
        }
        s.solid = target;
    }

    // ---- Null-face records (loop-role attributes travel remapped;
    // fully-finished grafts carry none). ----
    for (k, pair) in src.null_faces.iter() {
        let holder = EntityId::Face(k);
        let dk = faces.get(k).copied().unwrap_or_else(|| {
            unreachable!(
                "graft source: a null-face record names {holder}, which does not resolve: a \
                 null-face record is removed with its face, and {NAMES_ONLY_LIVE}"
            )
        });
        let ml = |l: LoopKey, field| image(&loops, l, EntityId::Loop, holder, field);
        let mapped = match *pair {
            crate::null::NullFacePair::Split {
                above_loop,
                below_loop,
            } => crate::null::NullFacePair::Split {
                above_loop: ml(above_loop, "above_loop"),
                below_loop: ml(below_loop, "below_loop"),
            },
            crate::null::NullFacePair::Boolean { in_copy, out_copy } => {
                crate::null::NullFacePair::Boolean {
                    in_copy: ml(in_copy, "in_copy"),
                    out_copy: ml(out_copy, "out_copy"),
                }
            }
        };
        dst.null_faces.insert(dk, mapped);
    }

    // ---- Pcurve caches (M5 PR 6), re-keyed to the result half-edges.
    // Since M6-3 every curved chart mints, so operands genuinely carry
    // caches here — including STALE rows (the provenance pass above
    // says why they are skipped): surgery kills half-edges without
    // clearing the cache map (the module-docs posture in
    // `topo::pcurves`). The row's key not being in the graft's
    // half-edge walk IS the staleness test (the walk covers every live
    // half-edge of the grafted solid), and the boolean result's own
    // final mint pass clears and re-derives every cache anyway. ----
    for (k, cache) in src.pcurves.iter() {
        let Some(&dk) = half_edges.get(k) else {
            continue;
        };
        dst.pcurves.insert(dk, cache.clone());
    }

    // ---- Description surface-key remap (M3 PR 5, the extrude-operand
    // finding): `Intersection`/`Seam` descriptions reference SURFACE
    // KEYS, which are body-lineage-scoped — the grafted copies must
    // reference the grafted surfaces. The remapped spec re-certifies
    // against the destination lookup (bitwise-identical carrier,
    // parameters, witness, and surface values ⇒ deterministic — D9);
    // a refusal here is loud, never a dangling reference. ----
    for (k, &dk) in curves.iter() {
        let Some(CurveGeom::Certified(curve)) = src.curves.get(k) else {
            continue;
        };
        let Bridge::Recertify { tol } = bridge else {
            // Handles only, certificate verbatim (see `Bridge`).
            let remapped = curve
                .with_remapped_surfaces(|sk| surfaces.get(sk).copied())
                .unwrap_or_else(|| {
                    unreachable!(
                        "graft source: {}'s description names a surface the source does not \
                         hold: {NAMES_ONLY_LIVE}",
                        GeomRef::Curve(k)
                    )
                });
            let Some(slot) = dst.curves.get_mut(dk) else {
                unreachable!(
                    "graft (handle remap): `dk` was minted into `dst.curves` by this \
                     call's curve pass"
                )
            };
            *slot = CurveGeom::Certified(remapped);
            continue;
        };
        let description = match *curve.description() {
            geom_brep::EdgeDescription::Intersection { s1, s2, witness } => {
                geom_brep::EdgeDescriptionSpec::Intersection {
                    s1: image(&surfaces, s1, GeomRef::Surface, GeomRef::Curve(k), "s1"),
                    s2: image(&surfaces, s2, GeomRef::Surface, GeomRef::Curve(k), "s2"),
                    witness,
                }
            }
            geom_brep::EdgeDescription::TangentIntersection { s1, s2, witness } => {
                geom_brep::EdgeDescriptionSpec::TangentIntersection {
                    s1: image(&surfaces, s1, GeomRef::Surface, GeomRef::Curve(k), "s1"),
                    s2: image(&surfaces, s2, GeomRef::Surface, GeomRef::Curve(k), "s2"),
                    witness,
                }
            }
            // The image travels verbatim (chart COORDINATES); only
            // the handle moves. The authority record travels beside
            // it, unchanged — a declaration is not a surface key.
            geom_brep::EdgeDescription::Chart(ref c) => geom_brep::EdgeDescriptionSpec::Chart {
                surface: image(
                    &surfaces,
                    c.surface,
                    GeomRef::Surface,
                    GeomRef::Curve(k),
                    "surface",
                ),
                image: Some(c.pcurve.clone()),
                seam: c.seam,
                declared: match curve.authority() {
                    geom_brep::EdgeAuthority::Declared(mc) => Some(mc),
                    geom_brep::EdgeAuthority::Derived => None,
                },
            },
            geom_brep::EdgeDescription::Scaffold(_) => continue, // no surface keys
        };
        // Endpoints from the (already grafted) owning edge: he_plus
        // runs start → end on the forward carrier.
        let (src_edge_key, _) = src
            .edges
            .iter()
            .find(|(_, e)| e.curve == k)
            .unwrap_or_else(|| {
                unreachable!(
                    "graft source: {} is certified and no edge names it: a tier-1-valid body \
                     holds no orphan geometry, and every public door keeps the body tier-1-valid",
                    GeomRef::Curve(k)
                )
            });
        let dst_edge_key = edges[src_edge_key];
        let e = &dst.edges[dst_edge_key];
        let he_plus = &dst.half_edges[e.he_plus];
        let start_v = he_plus.start;
        let end_v = dst.half_edges[he_plus.next].start;
        let point = |v: VertexKey| dst.points[dst.vertices[v].point];
        let spec = geom_brep::EdgeCurveSpec {
            description,
            carrier: curve.carrier().clone(),
            param_start: curve.params().0,
            param_end: curve.params().1,
        };
        let band = geom_core::Band::linear(tol)?;
        let recert = geom_brep::EdgeCurve::certify(
            spec,
            point(start_v),
            point(end_v),
            |sk| dst.surfaces.get(sk).cloned(),
            band,
        )
        .map_err(BooleanError::GraftRecertify)?;
        let Some(slot) = dst.curves.get_mut(dk) else {
            unreachable!(
                "graft (recertify): `dk` was minted into `dst.curves` by this call's \
                 curve pass"
            )
        };
        *slot = CurveGeom::Certified(recert);
    }

    // ---- Attach the shells to the destination solids (source order,
    // per solid and within each solid). ----
    for &(src_solid, dst_solid) in &pairs {
        let holder = EntityId::Solid(src_solid);
        let shell_list: Vec<ShellKey> = src.solids[src_solid]
            .shells
            .iter()
            .map(|&s| image(&shells, s, EntityId::Shell, holder, "shells"))
            .collect();
        let Some(solid) = dst.get_solid_mut(dst_solid) else {
            unreachable!(
                "graft: destination solid {dst_solid:?} was checked live in, or minted into, \
                 the body this transplant writes"
            )
        };
        solid.shells.extend(shell_list);
    }

    let map = GraftMap {
        vertices,
        faces,
        edges,
        dead_edges,
        surfaces,
        shells,
    };
    Ok((map, pairs.into_iter().map(|(_, dk)| dk).collect()))
}
