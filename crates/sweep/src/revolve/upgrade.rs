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
    DihedralClass, EdgeCurveSpec, EdgeDescriptionSpec, MustCarryVerdict, classify_dihedral,
    edge_extent, must_carry_over_edge,
};
use geom_core::spline::SpanLocate;
use geom_core::{Band, Decide, Point3, Real};
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
/// latitude rims all funnel here. Smooth descends one order through
/// the must-carry rule over the edge
/// ([`geom_brep::must_carry_over_edge`] — the lane gate and the
/// certification schedule's interior stations, in its one home), and a
/// station the rule reads transverse refuses
/// [`RevolveError::SmoothJoinRefuted`]; Indeterminate is the typed
/// error built by `sliver`, at the first-order classification and at
/// the rule alike.
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
        // A revolve join's carrier is a latitude CIRCLE, which the
        // jet certificate's circle arm covers on every surface of
        // revolution, so the rule can reach its determinate answer
        // here: a jet-DETERMINATE smooth join is a genuine
        // `TangentIntersection` and prefer-intrinsic (D2/OQ7) demands
        // it; an under-determined one (a G2 conventional join, a
        // same-surface split: `κ_rel` at zero) keeps the conventional
        // description, exactly as tier 3's must-carry exempts it; an
        // in-band one escalates typed. Both sides read the same
        // predicate at the same stations, so the demanded set and the
        // stored set stay one set.
        Ok(DihedralClass::Smooth) => {
            match must_carry_over_edge(
                &surf1,
                &surf2,
                &data.carrier,
                data.t0,
                data.t1,
                data.extent,
                band,
            ) {
                MustCarryVerdict::JetDeterminate => {
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
                }
                MustCarryVerdict::UnderDetermined => {
                    // The surfaces UNDER-determine the locus, so the
                    // description stays CONVENTIONAL — but the edge is
                    // at rest between two faces now, so it says where
                    // it rests: an image in `s1`'s chart (D3's
                    // transience fence). The pushforward it was
                    // scaffolded from stays beside it as the authority
                    // record, which is what keeps tier 3's
                    // prefer-intrinsic reading unchanged.
                    body.describe_at_rest(edge, s1, tol)?;
                }
                // In-band: near-osculating geometry, certifiable as
                // neither intrinsic nor conventional. A conventional
                // description is not an escape hatch from
                // ill-conditioned geometry (D2), so the escalation is
                // typed through the caller's own `sliver` (D4 ¶3) —
                // the same answer the transverse arm's `Err` below
                // gives, and the same one the extrude strut gives.
                MustCarryVerdict::InBand(source) => return Err(sliver(source)),
                // A station reads the join a corner where the witness
                // read it smooth: this arm's premise is refuted, and
                // the edge refuses rather than store a description
                // neither reading chose.
                MustCarryVerdict::Transverse => {
                    return Err(RevolveError::SmoothJoinRefuted { edge });
                }
            }
            Ok(())
        }
        Err(source) => Err(sliver(source)),
    }
}

/// Re-describes a full-revolve meridian as `Seam { surface }` when the
/// wall surface is periodic; a plane wall's meridian becomes an image
/// at rest in that wall's chart (module docs — `Seam` is malformed on
/// a non-periodic chart, and one surface on both sides determines no
/// locus, so D2's conventional split applies). Carrier and interval
/// kept verbatim either way.
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
        body.describe_at_rest(edge, wall, tol)?;
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
