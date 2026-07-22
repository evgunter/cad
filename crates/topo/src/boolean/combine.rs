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
//! - **Provenance records are transplanted verbatim.** They are
//!   historical data (their payload keys reference the *source*
//!   operand's lineage, which the recipe's boolean node names), per
//!   the provenance module's records-are-historical contract — a
//!   graft is not a re-birth.
//! - The returned [`GraftMap`] is the ONLY bridge between source keys
//!   and result keys; downstream consumers (the seam zip's
//!   record-keyed correspondence, contact-record remapping) read it
//!   as data — never key equality across bodies.
//!
//! The result body afterwards holds one solid with shells from both
//! operands: exactly the tier-2-legal multi-shell mid-state the seam
//! zip consumes (or, for fallback results — disjoint unions, voids —
//! the finished multi-shell body itself).


use slotmap::SecondaryMap;

use super::BooleanError;
use crate::body::Body;
use crate::entity::{
    EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, ShellKey, SolidKey, VertexKey,
};
use crate::geometry::{CurveKey, PointKey, SurfaceKey};
use crate::null::CurveGeom;

/// The source→result key bridge (module docs). Only the maps the
/// pipeline consumes are exposed; the rest are internal to the graft.
#[derive(Debug, Default)]
pub(super) struct GraftMap {
    /// Source vertex → result vertex.
    pub vertices: SecondaryMap<VertexKey, VertexKey>,
    /// Source face → result face.
    pub faces: SecondaryMap<FaceKey, FaceKey>,
}

/// Transplants `src`'s single solid into `dst_solid` of `dst`
/// (module docs). `src` is consumed by value — its arenas are read in
/// slot order; nothing of it survives as shared state.
pub(super) fn graft_solid<T: geom_core::Decide>(
    dst: &mut Body<T>,
    dst_solid: SolidKey,
    src: &Body<T>,
) -> Result<GraftMap, BooleanError> {
    let corrupt = || BooleanError::JoinDesync {
        what: "graft source is not a well-formed single-solid body",
    };
    let mut src_solids = src.solids();
    let (_, src_solid) = src_solids.next().ok_or_else(corrupt)?;
    if src_solids.next().is_some() {
        return Err(corrupt());
    }

    // ---- Geometry arenas (slot order). ----
    let mut points: SecondaryMap<PointKey, PointKey> = SecondaryMap::new();
    for (k, p) in src.points.iter() {
        points.insert(k, dst.points.insert(*p));
    }
    let mut surfaces: SecondaryMap<SurfaceKey, SurfaceKey> = SecondaryMap::new();
    for (k, s) in src.surfaces.iter() {
        surfaces.insert(k, dst.surfaces.insert(s.clone()));
    }

    // ---- Topology arenas, pass 1: clone with source-internal keys
    // (patched in pass 2), recording the fresh keys. ----
    let mut vertices: SecondaryMap<VertexKey, VertexKey> = SecondaryMap::new();
    for (k, v) in src.vertices.iter() {
        let dk = dst.vertices.insert(v.clone());
        vertices.insert(k, dk);
        if let Some(p) = src.vertex_provenance.get(k) {
            dst.vertex_provenance.insert(dk, p.clone());
        }
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
        curves.insert(k, dst.curves.insert(mapped));
    }
    let mut half_edges: SecondaryMap<HalfEdgeKey, HalfEdgeKey> = SecondaryMap::new();
    for (k, he) in src.half_edges.iter() {
        let dk = dst.half_edges.insert(he.clone());
        half_edges.insert(k, dk);
        if let Some(p) = src.half_edge_provenance.get(k) {
            dst.half_edge_provenance.insert(dk, p.clone());
        }
    }
    let mut edges: SecondaryMap<EdgeKey, EdgeKey> = SecondaryMap::new();
    for (k, e) in src.edges.iter() {
        let dk = dst.edges.insert(e.clone());
        edges.insert(k, dk);
        if let Some(p) = src.edge_provenance.get(k) {
            dst.edge_provenance.insert(dk, p.clone());
        }
    }
    let mut loops: SecondaryMap<LoopKey, LoopKey> = SecondaryMap::new();
    for (k, l) in src.loops.iter() {
        let dk = dst.loops.insert(l.clone());
        loops.insert(k, dk);
        if let Some(p) = src.loop_provenance.get(k) {
            dst.loop_provenance.insert(dk, p.clone());
        }
    }
    let mut faces: SecondaryMap<FaceKey, FaceKey> = SecondaryMap::new();
    for (k, f) in src.faces.iter() {
        let dk = dst.faces.insert(f.clone());
        faces.insert(k, dk);
        if let Some(p) = src.face_provenance.get(k) {
            dst.face_provenance.insert(dk, p.clone());
        }
    }
    let mut shells: SecondaryMap<ShellKey, ShellKey> = SecondaryMap::new();
    for (k, s) in src.shells.iter() {
        let dk = dst.shells.insert(s.clone());
        shells.insert(k, dk);
        if let Some(p) = src.shell_provenance.get(k) {
            dst.shell_provenance.insert(dk, p.clone());
        }
    }

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
    for (_, &dk) in shells.iter() {
        let s = dst.shells.get_mut(dk).ok_or_else(corrupt)?;
        for f in &mut s.faces {
            *f = *faces.get(*f).ok_or_else(corrupt)?;
        }
        s.solid = dst_solid;
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
        let description = match *curve.description() {
            geom_brep::EdgeGeometry::Intersection { s1, s2, witness } => {
                geom_brep::EdgeGeometry::Intersection {
                    s1: *surfaces.get(s1).ok_or_else(corrupt)?,
                    s2: *surfaces.get(s2).ok_or_else(corrupt)?,
                    witness,
                }
            }
            geom_brep::EdgeGeometry::Seam { surface } => geom_brep::EdgeGeometry::Seam {
                surface: *surfaces.get(surface).ok_or_else(corrupt)?,
            },
            geom_brep::EdgeGeometry::MappedCurve(_) => continue, // no surface keys
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
        let band = geom_core::Band::linear().map_err(|_| corrupt())?;
        let recert = geom_brep::EdgeCurve::certify(
            spec,
            point(start_v)?,
            point(end_v)?,
            |sk| dst.surfaces.get(sk).cloned(),
            band,
        )
        .map_err(BooleanError::GraftRecertify)?;
        if let Some(slot) = dst.curves.get_mut(dk) {
            *slot = CurveGeom::Certified(recert);
        }
    }

    // ---- Attach the shells to the destination solid (source order). ----
    let shell_list: Vec<ShellKey> = src_solid
        .shells
        .iter()
        .map(|&s| shells.get(s).copied().ok_or_else(corrupt))
        .collect::<Result<_, _>>()?;
    let solid = dst.get_solid_mut(dst_solid).ok_or_else(corrupt)?;
    solid.shells.extend(shell_list);

    Ok(GraftMap {
        vertices,
        faces,
    })
}
