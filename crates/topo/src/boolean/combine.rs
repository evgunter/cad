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
    EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, ShellKey, SolidKey, VertexKey,
};
use crate::geometry::{CurveKey, PointKey, SurfaceKey};
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

/// The dead result keys minted for source keys a record names but the
/// source no longer holds, one per source key.
#[derive(Default)]
struct Dead {
    half_edges: BTreeMap<HalfEdgeKey, HalfEdgeKey>,
    edges: BTreeMap<EdgeKey, EdgeKey>,
    loops: BTreeMap<LoopKey, LoopKey>,
    shells: BTreeMap<ShellKey, ShellKey>,
    solids: BTreeMap<SolidKey, SolidKey>,
}

/// A key of `arena` that is dead on arrival: minted and removed at
/// once, so it resolves nowhere and slot versioning keeps any later
/// insertion from taking it.
fn tombstone<K: Key, V>(arena: &mut SlotMap<K, V>, placeholder: V) -> K {
    let k = arena.insert(placeholder);
    arena.remove(k);
    k
}

/// One graft's record forwarding (module docs): live source keys
/// through the graft's maps, dead ones to their tombstones.
struct Forward<'g, T: geom_core::Real> {
    half_edges: &'g SecondaryMap<HalfEdgeKey, HalfEdgeKey>,
    edges: &'g SecondaryMap<EdgeKey, EdgeKey>,
    loops: &'g SecondaryMap<LoopKey, LoopKey>,
    shells: &'g SecondaryMap<ShellKey, ShellKey>,
    solids: &'g SecondaryMap<SolidKey, SolidKey>,
    dst: &'g mut Body<T>,
    dead: Dead,
}

impl<T: geom_core::Real> crate::provenance::ForwardKeys for Forward<'_, T> {
    fn half_edge(&mut self, k: HalfEdgeKey) -> HalfEdgeKey {
        if let Some(&d) = self.half_edges.get(k) {
            return d;
        }
        let arena = &mut self.dst.half_edges;
        *self.dead.half_edges.entry(k).or_insert_with(|| {
            let placeholder = crate::entity::HalfEdge {
                edge: EdgeKey::null(),
                start: VertexKey::null(),
                parent_loop: LoopKey::null(),
                next: HalfEdgeKey::null(),
                prev: HalfEdgeKey::null(),
            };
            tombstone(arena, placeholder)
        })
    }
    fn loop_(&mut self, k: LoopKey) -> LoopKey {
        if let Some(&d) = self.loops.get(k) {
            return d;
        }
        let arena = &mut self.dst.loops;
        *self.dead.loops.entry(k).or_insert_with(|| {
            let placeholder = crate::entity::Loop {
                boundary: LoopBoundary::Empty {
                    vertex: VertexKey::null(),
                },
                face: FaceKey::null(),
            };
            tombstone(arena, placeholder)
        })
    }
    fn edge(&mut self, k: EdgeKey) -> EdgeKey {
        if let Some(&d) = self.edges.get(k) {
            return d;
        }
        let arena = &mut self.dst.edges;
        *self.dead.edges.entry(k).or_insert_with(|| {
            let placeholder = crate::entity::Edge {
                he_plus: HalfEdgeKey::null(),
                he_minus: HalfEdgeKey::null(),
                curve: CurveKey::null(),
            };
            tombstone(arena, placeholder)
        })
    }
    fn shell(&mut self, k: ShellKey) -> ShellKey {
        if let Some(&d) = self.shells.get(k) {
            return d;
        }
        let arena = &mut self.dst.shells;
        *self.dead.shells.entry(k).or_insert_with(|| {
            let placeholder = crate::entity::Shell {
                faces: Vec::new(),
                solid: SolidKey::null(),
            };
            tombstone(arena, placeholder)
        })
    }
    fn solid(&mut self, k: SolidKey) -> SolidKey {
        if let Some(&d) = self.solids.get(k) {
            return d;
        }
        let arena = &mut self.dst.solids;
        *self
            .dead
            .solids
            .entry(k)
            .or_insert_with(|| tombstone(arena, crate::entity::Solid { shells: Vec::new() }))
    }
}

/// Transplants `src`'s single solid into `dst_solid` of `dst`
/// (module docs). `src` is consumed by value — its arenas are read in
/// slot order; nothing of it survives as shared state.
pub(crate) fn graft_solid<T: geom_core::Decide>(
    dst: &mut Body<T>,
    dst_solid: SolidKey,
    src: &Body<T>,
    tol: Tol,
) -> Result<GraftMap, BooleanError> {
    graft_solid_with(dst, dst_solid, src, Bridge::Recertify, tol)
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
///   the destination's surfaces — what the boolean pipeline wants,
///   whose operands have been through surgery.
/// - [`Bridge::RemapKeys`] carries the source's certificate verbatim
///   with only the handles rewritten
///   ([`geom_brep::EdgeCurve::with_remapped_surfaces`]) — what a
///   DISJOINT graft wants, where the transplanted geometry is bitwise
///   the source's and no surgery happened. It is also the only form
///   that can carry a description the certification lanes cannot
///   express at all (a rational NURBS wall certifies nowhere), which
///   is why an import's placed instances take it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Bridge {
    /// Re-run the schedule against the destination (booleans).
    Recertify,
    /// Rewrite the handles, keep the source's certificate (disjoint).
    RemapKeys,
}

/// [`graft_solid`] with the description bridge chosen explicitly.
pub(crate) fn graft_solid_with<T: geom_core::Decide>(
    dst: &mut Body<T>,
    dst_solid: SolidKey,
    src: &Body<T>,
    bridge: Bridge,
    tol: Tol,
) -> Result<GraftMap, BooleanError> {
    graft_solids_with(dst, &[dst_solid], src, bridge, tol)
}

/// [`graft_solid_with`] for a source holding N solids: `dst_solids`
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
    tol: Tol,
) -> Result<GraftMap, BooleanError> {
    graft_solids_impl(dst, dst_solids, src, bridge, SolidRecords::Kept, tol)
}

/// [`graft_solids_with`] onto destination solids the caller minted FOR
/// the source's solids: each takes its source solid's record,
/// forwarded like every other record (module docs), in place of the
/// one it was minted with.
pub(crate) fn graft_solids_minted<T: geom_core::Decide>(
    dst: &mut Body<T>,
    dst_solids: &[SolidKey],
    src: &Body<T>,
    bridge: Bridge,
    tol: Tol,
) -> Result<GraftMap, BooleanError> {
    graft_solids_impl(dst, dst_solids, src, bridge, SolidRecords::FromSource, tol)
}

/// Whose provenance record a destination solid carries after a graft.
#[derive(Clone, Copy, PartialEq, Eq)]
enum SolidRecords {
    /// Its own: the solid was there before, and the source's shells
    /// joined it.
    Kept,
    /// Its source solid's: the solid was minted to receive that one.
    FromSource,
}

fn graft_solids_impl<T: geom_core::Decide>(
    dst: &mut Body<T>,
    dst_solids: &[SolidKey],
    src: &Body<T>,
    bridge: Bridge,
    solid_records: SolidRecords,
    tol: Tol,
) -> Result<GraftMap, BooleanError> {
    let corrupt = || BooleanError::JoinDesync {
        what: "graft source is not a well-formed body",
    };
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
    {
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
    if pairs.is_empty() {
        return Err(corrupt());
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
        let Some(origin) = src.surface_origins.get(k) else {
            unreachable!(
                "grafted surface {k:?} is live in the source body and carries no origin row: \
                 the origin map is total over live keys (kernel bug)"
            )
        };
        dst.surface_origins.insert(dk, origin.clone());
        // The per-FIELD ParamSource rows ride the graft for the same
        // reason and by the same rule: a description's parameter
        // identity is carried with the description, never re-derived
        // from the transplanted values.
        if let Some(fields) = src.surface_field_sources.get(k) {
            dst.surface_field_sources.insert(dk, fields.clone());
        }
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
                attr.below_end = *vertices.get(attr.below_end).ok_or_else(corrupt)?;
                attr.above_end = *vertices.get(attr.above_end).ok_or_else(corrupt)?;
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

    // ---- Provenance: every record forwarded into `dst`'s keys. ----
    let mut fwd = Forward {
        half_edges: &half_edges,
        edges: &edges,
        loops: &loops,
        shells: &shells,
        solids: &solid_map,
        dst: &mut *dst,
        dead: Dead::default(),
    };
    for (k, &dk) in vertices.iter() {
        if let Some(p) = src.vertex_provenance.get(k) {
            let p = p.forwarded(&mut fwd);
            fwd.dst.vertex_provenance.insert(dk, p);
        }
    }
    for (k, &dk) in half_edges.iter() {
        if let Some(p) = src.half_edge_provenance.get(k) {
            let p = p.forwarded(&mut fwd);
            fwd.dst.half_edge_provenance.insert(dk, p);
        }
    }
    for (k, &dk) in edges.iter() {
        if let Some(p) = src.edge_provenance.get(k) {
            let p = p.forwarded(&mut fwd);
            fwd.dst.edge_provenance.insert(dk, p);
        }
    }
    for (k, &dk) in loops.iter() {
        if let Some(p) = src.loop_provenance.get(k) {
            let p = p.forwarded(&mut fwd);
            fwd.dst.loop_provenance.insert(dk, p);
        }
    }
    for (k, &dk) in faces.iter() {
        if let Some(p) = src.face_provenance.get(k) {
            let p = p.forwarded(&mut fwd);
            fwd.dst.face_provenance.insert(dk, p);
        }
    }
    for (k, &dk) in shells.iter() {
        if let Some(p) = src.shell_provenance.get(k) {
            let p = p.forwarded(&mut fwd);
            fwd.dst.shell_provenance.insert(dk, p);
        }
    }
    if solid_records == SolidRecords::FromSource {
        for &(k, dk) in &pairs {
            let p = src.solid_provenance.get(k).ok_or_else(corrupt)?;
            let p = p.forwarded(&mut fwd);
            fwd.dst.solid_provenance.insert(dk, p);
        }
    }
    let dead_edges = fwd.dead.edges;

    // ---- Pass 2: patch every cross-reference to result keys. ----
    let map = |m: &SecondaryMap<VertexKey, VertexKey>, k: VertexKey| m.get(k).copied();
    for (_, &dk) in vertices.iter() {
        let v = dst.vertices.get_mut(dk).ok_or_else(corrupt)?;
        v.point = *points.get(v.point).ok_or_else(corrupt)?;
        if let Some(e) = v.emanating {
            v.emanating = Some(*half_edges.get(e).ok_or_else(corrupt)?);
        }
    }
    for (_, &dk) in half_edges.iter() {
        let he = dst.half_edges.get_mut(dk).ok_or_else(corrupt)?;
        he.edge = *edges.get(he.edge).ok_or_else(corrupt)?;
        he.start = map(&vertices, he.start).ok_or_else(corrupt)?;
        he.parent_loop = *loops.get(he.parent_loop).ok_or_else(corrupt)?;
        he.next = *half_edges.get(he.next).ok_or_else(corrupt)?;
        he.prev = *half_edges.get(he.prev).ok_or_else(corrupt)?;
    }
    for (_, &dk) in edges.iter() {
        let e = dst.edges.get_mut(dk).ok_or_else(corrupt)?;
        e.he_plus = *half_edges.get(e.he_plus).ok_or_else(corrupt)?;
        e.he_minus = *half_edges.get(e.he_minus).ok_or_else(corrupt)?;
        e.curve = *curves.get(e.curve).ok_or_else(corrupt)?;
    }
    for (_, &dk) in loops.iter() {
        let l = dst.loops.get_mut(dk).ok_or_else(corrupt)?;
        l.boundary = match l.boundary {
            LoopBoundary::Empty { vertex } => LoopBoundary::Empty {
                vertex: map(&vertices, vertex).ok_or_else(corrupt)?,
            },
            LoopBoundary::Cycle { first } => LoopBoundary::Cycle {
                first: *half_edges.get(first).ok_or_else(corrupt)?,
            },
        };
        l.face = *faces.get(l.face).ok_or_else(corrupt)?;
    }
    for (_, &dk) in faces.iter() {
        let f = dst.faces.get_mut(dk).ok_or_else(corrupt)?;
        f.surface = *surfaces.get(f.surface).ok_or_else(corrupt)?;
        f.outer = *loops.get(f.outer).ok_or_else(corrupt)?;
        for r in &mut f.rings {
            *r = *loops.get(*r).ok_or_else(corrupt)?;
        }
        f.shell = *shells.get(f.shell).ok_or_else(corrupt)?;
    }
    for (sk, &dk) in shells.iter() {
        // A shell lands under the destination of the solid that owns
        // it in the source — its own back-pointer, so a multi-solid
        // source keeps every shell with its solid.
        let owner = src.shells.get(sk).ok_or_else(corrupt)?.solid;
        let target = *solid_map.get(owner).ok_or_else(corrupt)?;
        let s = dst.shells.get_mut(dk).ok_or_else(corrupt)?;
        for f in &mut s.faces {
            *f = *faces.get(*f).ok_or_else(corrupt)?;
        }
        s.solid = target;
    }

    // ---- Null-face records (loop-role attributes travel remapped;
    // fully-finished grafts carry none). ----
    for (k, pair) in src.null_faces.iter() {
        let dk = *faces.get(k).ok_or_else(corrupt)?;
        let ml = |l: LoopKey| loops.get(l).copied().ok_or_else(corrupt);
        let mapped = match *pair {
            crate::null::NullFacePair::Split {
                above_loop,
                below_loop,
            } => crate::null::NullFacePair::Split {
                above_loop: ml(above_loop)?,
                below_loop: ml(below_loop)?,
            },
            crate::null::NullFacePair::Boolean { in_copy, out_copy } => {
                crate::null::NullFacePair::Boolean {
                    in_copy: ml(in_copy)?,
                    out_copy: ml(out_copy)?,
                }
            }
        };
        dst.null_faces.insert(dk, mapped);
    }

    // ---- Pcurve caches (M5 PR 6): remapped like provenance. Since
    // M6-3 every curved chart mints, so operands genuinely carry
    // caches here — including STALE rows: surgery kills half-edges
    // without clearing the cache map (the module-docs posture in
    // `topo::pcurves`), so a reduced operand may hold rows keyed by
    // half-edges that no longer exist. Those rows are skipped, not
    // treated as corruption: the row's key not being in the graft's
    // half-edge walk IS the staleness test (the walk covers every
    // live half-edge of the grafted solid), and the boolean result's
    // own final mint pass clears and re-derives every cache anyway. ----
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
        if bridge == Bridge::RemapKeys {
            // Handles only, certificate verbatim (see `Bridge`).
            let remapped = curve
                .with_remapped_surfaces(|sk| surfaces.get(sk).copied())
                .ok_or_else(corrupt)?;
            let Some(slot) = dst.curves.get_mut(dk) else {
                unreachable!(
                    "graft (handle remap): `dk` was minted into `dst.curves` by this \
                     call's curve pass"
                )
            };
            *slot = CurveGeom::Certified(remapped);
            continue;
        }
        let description = match *curve.description() {
            geom_brep::EdgeDescription::Intersection { s1, s2, witness } => {
                geom_brep::EdgeDescriptionSpec::Intersection {
                    s1: *surfaces.get(s1).ok_or_else(corrupt)?,
                    s2: *surfaces.get(s2).ok_or_else(corrupt)?,
                    witness,
                }
            }
            geom_brep::EdgeDescription::TangentIntersection { s1, s2, witness } => {
                geom_brep::EdgeDescriptionSpec::TangentIntersection {
                    s1: *surfaces.get(s1).ok_or_else(corrupt)?,
                    s2: *surfaces.get(s2).ok_or_else(corrupt)?,
                    witness,
                }
            }
            // The image travels verbatim (chart COORDINATES); only
            // the handle moves. The authority record travels beside
            // it, unchanged — a declaration is not a surface key.
            geom_brep::EdgeDescription::Chart(ref c) => geom_brep::EdgeDescriptionSpec::Chart {
                surface: *surfaces.get(c.surface).ok_or_else(corrupt)?,
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
            .ok_or_else(corrupt)?;
        let dst_edge_key = *edges.get(src_edge_key).ok_or_else(corrupt)?;
        let e = dst.edges.get(dst_edge_key).ok_or_else(corrupt)?;
        let start_v = dst.half_edges.get(e.he_plus).ok_or_else(corrupt)?.start;
        let end_v = dst
            .half_edges
            .get(dst.half_edges.get(e.he_plus).ok_or_else(corrupt)?.next)
            .ok_or_else(corrupt)?
            .start;
        let point = |v: VertexKey| -> Result<geom_core::Point3<T>, BooleanError> {
            let vd = dst.vertices.get(v).ok_or_else(corrupt)?;
            dst.points.get(vd.point).copied().ok_or_else(corrupt)
        };
        let spec = geom_brep::EdgeCurveSpec {
            description,
            carrier: curve.carrier().clone(),
            param_start: curve.params().0,
            param_end: curve.params().1,
        };
        let band = geom_core::Band::linear(tol).map_err(|_| corrupt())?;
        let recert = geom_brep::EdgeCurve::certify(
            spec,
            point(start_v)?,
            point(end_v)?,
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
    for (src_solid, dst_solid) in pairs {
        let shell_list: Vec<ShellKey> = src
            .shells_of_solid(src_solid)
            .ok_or_else(corrupt)?
            .iter()
            .map(|&s| shells.get(s).copied().ok_or_else(corrupt))
            .collect::<Result<_, _>>()?;
        let solid = dst.get_solid_mut(dst_solid).ok_or_else(corrupt)?;
        solid.shells.extend(shell_list);
    }

    Ok(GraftMap {
        vertices,
        faces,
        edges,
        dead_edges,
        surfaces,
        shells,
    })
}
