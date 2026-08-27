//! The post-construction description upgrades a revolve performs (the
//! ratified prefer-intrinsic decisions, M2-LOG 2026-07-19): cap–wall
//! meridian rims and cap–cap axis edges to `Intersection`, latitude
//! joins to `Intersection`, full-revolve meridians to `Seam`. All
//! witnesses are the carrier's mid-parameter point (the S2 witness
//! contract; a full-period rim's witness is the start point's
//! antipode); all re-descriptions go through `topo`'s certified
//! `set_edge_curve` door with the carrier and interval kept verbatim.

use geom::Curve3;
use geom_brep::{
    DihedralClass, EdgeCurveSpec, EdgeDescriptionSpec, classify_dihedral, curvature_lever_arm,
    edge_extent, tangent_certificate_lane, tangent_jet,
};
use geom_core::spline::SpanLocate;
use geom_core::{Band, Decide, Margin, Point3, Real};
use topo::{Body, EdgeKey, EulerOpError, SurfaceKey};

use super::RevolveError;
use geom_core::Tol;

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

/// A vertex's point, with the kernel read-back door's unresolved
/// reference renamed into the operator layer's stale-key vocabulary
/// (total: stale keys surface as operator-layer typed errors).
pub(super) fn vertex_point<T: Real>(
    body: &Body<T>,
    vertex: topo::VertexKey,
) -> Result<Point3<T>, RevolveError> {
    topo::readback::vertex_point_ref(body, vertex).map_err(|what| EulerOpError::from(what).into())
}

/// This edge restated as a spec — description, carrier and interval
/// verbatim — for a re-description that moves nothing geometric.
fn restated<T: SpanLocate>(
    body: &Body<T>,
    edge: EdgeKey,
) -> Result<geom_brep::EdgeCurveSpec<T>, RevolveError> {
    let edge_rec = body.get_edge(edge).ok_or(EulerOpError::StaleKey {
        key: topo::EntityId::Edge(edge),
    })?;
    Ok(body
        .get_curve_geom(edge_rec.curve)
        .ok_or(EulerOpError::StaleGeometry {
            key: topo::GeomRef::Curve(edge_rec.curve),
        })?
        .certified()
        .ok_or(EulerOpError::NullScaffoldCurve {
            curve: edge_rec.curve,
        })?
        .restated_spec())
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
    tol: Tol,
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
                description: EdgeDescriptionSpec::Intersection {
                    s1,
                    s2,
                    witness: data.witness,
                },
                carrier: data.carrier,
                param_start: data.t0,
                param_end: data.t1,
            };
            body.set_edge_curve(edge, spec, tol)?;
            Ok(())
        }
        // **The lane's next retirement, taken (M5 PR 12).** A revolve
        // join's carrier is a latitude CIRCLE, which the jet
        // certificate's circle arm now covers on every surface of
        // revolution — so a jet-DETERMINATE smooth join is a genuine
        // `TangentIntersection` and prefer-intrinsic (D2/OQ7) demands
        // it. A jet-UNDER-determined one (a G2 conventional join, a
        // same-surface split: `κ_rel` at zero) keeps the conventional
        // description, exactly as tier 3's must-carry exempts it —
        // both sides read the same `tangent_second_order` predicate,
        // so the demanded set and the stored set stay one set.
        Ok(DihedralClass::Smooth) => {
            if jet_determinate(&surf1, &surf2, &data, band) {
                let spec = EdgeCurveSpec {
                    description: EdgeDescriptionSpec::TangentIntersection {
                        s1,
                        s2,
                        witness: data.witness,
                    },
                    carrier: data.carrier,
                    param_start: data.t0,
                    param_end: data.t1,
                };
                body.set_edge_curve(edge, spec, tol)?;
            } else {
                // The surfaces UNDER-determine the locus, so the
                // description stays CONVENTIONAL — but the edge is at
                // rest between two faces now, so it says where it
                // rests: an image in `s1`'s chart (D3's transience
                // fence). The pushforward it was scaffolded from stays
                // beside it as the authority record, which is what
                // keeps tier 3's prefer-intrinsic reading unchanged.
                let spec = restated(body, edge)?.at_rest_in_chart(s1, false);
                body.set_edge_curve(edge, spec, tol)?;
            }
            Ok(())
        }
        Err(source) => Err(sliver(source)),
    }
}

/// Is this smooth join **jet-determinate** — inside the certificate's
/// span-bound lane, and with the second-order separation definitely
/// positive at every sample of the certification schedule?
///
/// The margin is tier 3's own: `|κ_rel|·arm²/2` in meters, with `arm`
/// the folded lever arm `min(curvature arms, extent)` — the SAME
/// quantity, the same predicate name, so the constructor stores
/// exactly what the validator will demand and nothing more. An
/// under-determined join (`Zero`) or an in-band one (an escalation,
/// which tier 3 reports as an F6 sliver and exempts here) keeps its
/// conventional description: the intrinsic upgrade is an enrichment,
/// never a new refusal.
fn jet_determinate<T: Decide>(
    s1: &geom::Surface<T>,
    s2: &geom::Surface<T>,
    data: &EdgeData<T>,
    band: Band,
) -> bool {
    if !tangent_certificate_lane(&data.carrier, s1, s2) {
        return false;
    }
    let samples = 9u32;
    for i in 1..samples - 1 {
        let f = T::from_f64(f64::from(i) / f64::from(samples - 1));
        let t = data.t0 + (data.t1 - data.t0) * f;
        let p = data.carrier.eval(t);
        let jet = tangent_jet(s1, s2, p, data.carrier.deriv(t));
        let arm = curvature_lever_arm(s1, p)
            .min(curvature_lever_arm(s2, p))
            .min(data.extent);
        let margin = Margin::sagitta(jet.kappa_rel.abs(), arm);
        if !matches!(
            crate::swept::decide("tangent_second_order", margin, band),
            Ok(geom_core::Sign::Positive)
        ) {
            return false;
        }
    }
    true
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
    tol: Tol,
) -> Result<(), RevolveError> {
    let is_plane = matches!(
        body.get_surface(wall).ok_or(EulerOpError::StaleGeometry {
            key: topo::GeomRef::Surface(wall),
        })?,
        geom::Surface::Plane { .. }
    );
    if is_plane {
        // A plane wall has no seam to be — but the meridian is still
        // at rest in that wall's chart, and the scaffolding door it
        // was minted through is for edges whose surfaces do not exist
        // yet (D3's transience fence). So it is described where it
        // rests, as an ordinary chart image owing the one meter.
        let spec = restated(body, edge)?.at_rest_in_chart(wall, false);
        body.set_edge_curve(edge, spec, tol)?;
        return Ok(());
    }
    let data = edge_data(body, edge)?;
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::seam(wall),
        carrier: data.carrier,
        param_start: data.t0,
        param_end: data.t1,
    };
    body.set_edge_curve(edge, spec, tol)?;
    Ok(())
}
