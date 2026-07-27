//! The post-construction description upgrades a revolve performs (the
//! ratified prefer-intrinsic decisions, M2-LOG 2026-07-19): cap–wall
//! meridian rims and cap–cap axis edges to `Intersection`, latitude
//! joins to `Intersection`, full-revolve meridians to `Seam`. All
//! witnesses are the carrier's mid-parameter point (the S2 witness
//! contract; a full-period rim's witness is the start point's
//! antipode); all re-descriptions go through `topo`'s certified
//! `set_edge_curve` door with the carrier and interval kept verbatim.

use geom_brep::{DihedralClass, EdgeCurveSpec, EdgeGeometry, classify_dihedral, edge_extent};
use geom_core::spline::SpanLocate;
use geom_core::{Band, Decide, Point3, Real};
use geom_curves::Curve3;
use topo::{Body, EdgeKey, EulerOpError, FaceKey, SurfaceKey};

use super::RevolveError;

/// Resolves a face's surface key (total: stale keys surface as the
/// operator-layer typed error). Mirror of extrude's helper.
pub(super) fn face_surface_key<T: Real>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<SurfaceKey, RevolveError> {
    Ok(body
        .get_face(face)
        .ok_or(EulerOpError::StaleKey {
            key: topo::EntityId::Face(face),
        })?
        .surface)
}

/// An edge's stored certified carrier: `(carrier, t0, t1, witness,
/// extent, chord endpoints)` — the mid-parameter witness computed with
/// the certification schedule's own association `t0 + (t1 − t0)·½`
/// (chord midpoints fail on arcs), the extent through
/// [`geom_brep::edge_extent`] (carrier diameter for closed rims).
struct EdgeData<T: Real> {
    carrier: Curve3<T>,
    t0: T,
    t1: T,
    witness: Point3<T>,
    extent: T,
}

/// A vertex's point (total: stale keys surface as operator-layer typed
/// errors).
pub(super) fn vertex_point<T: Real>(
    body: &Body<T>,
    vertex: topo::VertexKey,
) -> Result<Point3<T>, RevolveError> {
    let point_key = body
        .get_vertex(vertex)
        .ok_or(EulerOpError::StaleKey {
            key: topo::EntityId::Vertex(vertex),
        })?
        .point;
    Ok(*body
        .get_point(point_key)
        .ok_or(EulerOpError::StaleGeometry {
            key: topo::GeomRef::Point(point_key),
        })?)
}

fn edge_data<T: SpanLocate>(body: &Body<T>, edge: EdgeKey) -> Result<EdgeData<T>, RevolveError> {
    let edge_rec = body.get_edge(edge).ok_or(EulerOpError::StaleKey {
        key: topo::EntityId::Edge(edge),
    })?;
    let curve = body
        .get_curve_geom(edge_rec.curve)
        .ok_or(EulerOpError::StaleGeometry {
            key: topo::GeomRef::Curve(edge_rec.curve),
        })?
        .certified()
        .ok_or(EulerOpError::NullScaffoldCurve {
            curve: edge_rec.curve,
        })?;
    let carrier = curve.carrier().clone();
    let (t0, t1) = curve.params();
    let witness = carrier.eval(t0 + (t1 - t0) * T::from_f64(0.5));
    let he_plus = edge_rec.he_plus;
    let start = body
        .get_half_edge(he_plus)
        .ok_or(EulerOpError::StaleKey {
            key: topo::EntityId::HalfEdge(he_plus),
        })?
        .start;
    let end = body.half_edge_end(he_plus).ok_or(EulerOpError::StaleKey {
        key: topo::EntityId::HalfEdge(he_plus),
    })?;
    let p_start = vertex_point(body, start)?;
    let p_end = vertex_point(body, end)?;
    let extent = edge_extent(&carrier, t0, t1, p_start.distance(p_end));
    Ok(EdgeData {
        carrier,
        t0,
        t1,
        witness,
        extent,
    })
}

/// Upgrades one edge to `Intersection { s1, s2, witness }` when the
/// two surfaces are definitely transverse at the witness: cap–wall
/// meridian rims, cap–cap axis edges (partial), and full-revolve
/// latitude rims all funnel here. Smooth keeps the conventional
/// description (the D2 split; tier 3 permits it); Indeterminate is the
/// typed error built by `sliver`.
pub(super) fn upgrade_intersection<T: Decide>(
    body: &mut Body<T>,
    edge: EdgeKey,
    s1: SurfaceKey,
    s2: SurfaceKey,
    band: Band,
    sliver: impl FnOnce(geom_core::Indeterminate) -> RevolveError,
) -> Result<(), RevolveError> {
    let data = edge_data(body, edge)?;
    let surf1 = body
        .get_surface(s1)
        .cloned()
        .ok_or(EulerOpError::StaleGeometry {
            key: topo::GeomRef::Surface(s1),
        })?;
    let surf2 = body
        .get_surface(s2)
        .cloned()
        .ok_or(EulerOpError::StaleGeometry {
            key: topo::GeomRef::Surface(s2),
        })?;
    match classify_dihedral(&surf1, &surf2, data.witness, data.extent, band) {
        Ok(DihedralClass::Transverse) => {
            let spec = EdgeCurveSpec {
                description: EdgeGeometry::Intersection {
                    s1,
                    s2,
                    witness: data.witness,
                },
                carrier: data.carrier,
                param_start: data.t0,
                param_end: data.t1,
            };
            body.set_edge_curve(edge, spec)?;
            Ok(())
        }
        Ok(DihedralClass::Smooth) => Ok(()),
        Err(source) => Err(sliver(source)),
    }
}

/// Re-describes a full-revolve meridian as `Seam { surface }` when the
/// wall surface is periodic; a plane wall's meridian keeps its
/// conventional `MappedCurve` (module docs — `Seam` is malformed on a
/// non-periodic chart, and the same-surface split is definitely
/// smooth). Carrier and interval kept verbatim.
pub(super) fn upgrade_meridian_seam<T: Decide>(
    body: &mut Body<T>,
    edge: EdgeKey,
    wall: SurfaceKey,
) -> Result<(), RevolveError> {
    let is_plane = matches!(
        body.get_surface(wall).ok_or(EulerOpError::StaleGeometry {
            key: topo::GeomRef::Surface(wall),
        })?,
        geom_surfaces::Surface::Plane { .. }
    );
    if is_plane {
        return Ok(());
    }
    let data = edge_data(body, edge)?;
    let spec = EdgeCurveSpec {
        description: EdgeGeometry::Seam { surface: wall },
        carrier: data.carrier,
        param_start: data.t0,
        param_end: data.t1,
    };
    body.set_edge_curve(edge, spec)?;
    Ok(())
}
