//! The post-construction description upgrades a revolve performs (the
//! ratified prefer-intrinsic decisions, M2-LOG 2026-07-19): cap–wall
//! meridian rims and cap–cap axis edges to `Intersection`, latitude
//! joins to `Intersection`, full-revolve meridians to `Seam`. All
//! witnesses are the carrier's mid-parameter point (the S2 witness
//! contract; a full-period rim's witness is the start point's
//! antipode); every re-description restates the stored carrier and
//! interval verbatim (`topo::Body::restate_stored_edge`, or
//! `describe_at_rest`).

use geom::Curve3;
use geom_brep::{
    DihedralClass, EdgeDescriptionSpec, MustCarryDescription, SmoothJoin, SurfacePair,
    classify_dihedral, must_carry_description,
};
use geom_core::spline::SpanLocate;
use geom_core::{Band, Decide, Point3, Real};
use topo::construction::Construct;
use topo::{Body, DihedralReading, EdgeKey, EulerOpError, SurfaceKey};

use super::RevolveError;
use geom_core::Tol;

/// An edge's stored certified carrier: `(carrier, t0, t1, witness,
/// reach)` — the mid-parameter witness computed with the certification
/// schedule's own association `t0 + (t1 − t0)·½` (chord midpoints fail
/// on arcs), and the faces' extent its wedge is levered at
/// ([`topo::wedge_reach`]).
struct EdgeData<'b, T: Real> {
    carrier: &'b Curve3<T>,
    t0: T,
    t1: T,
    witness: Point3<T>,
    reach: T,
}

fn edge_data<T: SpanLocate + Decide>(
    body: &Body<T>,
    edge: EdgeKey,
) -> Result<EdgeData<'_, T>, RevolveError> {
    let curve = body.stored_curve(edge).map_err(|e| match e {
        EulerOpError::Argument(_) => unreachable!(
            "edge {edge:?} was minted by this revolve and is upgraded before any kill of it"
        ),
        e => e,
    })?;
    let carrier = curve.carrier();
    let (t0, t1) = curve.params();
    let witness = curve.mid_point();
    let reach = topo::wedge_reach(body, topo::edge_faces(body, edge), carrier, (t0, t1));
    Ok(EdgeData {
        carrier,
        t0,
        t1,
        witness,
        reach,
    })
}

/// Upgrades one edge to `Intersection { pair, witness }` when the
/// two surfaces are definitely transverse at the witness: cap–wall
/// meridian rims, cap–cap axis edges (partial), and full-revolve
/// latitude rims all funnel here. Smooth descends one order through
/// the must-carry rule over the edge
/// ([`geom_brep::must_carry_over_edge`] — the lane gate and the
/// certification schedule's interior stations, in its one home), and a
/// station the rule reads transverse refuses
/// [`RevolveError::SmoothJoinRefuted`]; Indeterminate is the typed
/// error built by `sliver` from the reading that escalated, at the
/// first-order classification and at the rule alike.
pub(super) fn upgrade_intersection<T: Decide + topo::AtRestPolicy>(
    body: &mut Body<T>,
    edge: EdgeKey,
    s1: SurfaceKey,
    s2: SurfaceKey,
    band: Band,
    sliver: impl Fn(DihedralReading, geom_core::Indeterminate) -> RevolveError,
    tol: Tol,
) -> Result<(), RevolveError> {
    let data = edge_data(body, edge)?;
    let surf1 = body
        .get_surface(s1)
        .cloned()
        .unwrap_or_else(|| unreachable!("surface {s1:?} was read off a live face of this revolve"));
    let surf2 = body
        .get_surface(s2)
        .cloned()
        .unwrap_or_else(|| unreachable!("surface {s2:?} was read off a live face of this revolve"));
    match classify_dihedral(&surf1, &surf2, data.witness, data.reach, band) {
        Ok(DihedralClass::Transverse) => {
            body.restate_stored_edge(
                edge,
                EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(s1, s2),
                    selector: geom_brep::CurveSelector::Witness(data.witness),
                },
                tol,
            )?;
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
            // In-band: near-osculating geometry, certifiable as
            // neither intrinsic nor conventional. A conventional
            // description is not an escape hatch from ill-conditioned
            // geometry (D2), so the escalation is typed through the
            // caller's own `sliver` (D4 ¶3) — the same answer the
            // transverse arm's `Err` below gives, and the same one the
            // extrude strut gives. A station that reads the join a
            // corner where the witness read it smooth refutes this
            // arm's premise, and the edge refuses rather than store a
            // description neither reading chose.
            match must_carry_description(
                SmoothJoin {
                    surfaces: [(s1, &surf1), (s2, &surf2)],
                    carrier: data.carrier,
                    interval: (data.t0, data.t1),
                    reach: data.reach,
                    witness: data.witness,
                },
                band,
                &sliver,
                &sliver,
                |mixed| RevolveError::SmoothJoinRefuted { edge, mixed },
            )? {
                MustCarryDescription::Intrinsic(description) => {
                    body.restate_stored_edge(edge, description, tol)?;
                }
                MustCarryDescription::Conventional => {
                    // No intrinsic tangency is demanded (a zero-side
                    // station, or an all-positive pair outside the
                    // certificate's lane), so the description stays
                    // CONVENTIONAL — but the edge is at rest between
                    // two faces now, so it says where it rests: the
                    // pair it lies on (D3's transience fence). The
                    // pushforward it was scaffolded from stays beside
                    // it as the authority record.
                    body.describe_at_rest(edge, tol)?;
                }
            }
            Ok(())
        }
        Err(escalation) => {
            let (reading, source) = DihedralReading::of_lever(escalation);
            Err(sliver(reading, source))
        }
    }
}

/// Re-describes a full-revolve meridian as its wall's wrap edge (D1)
/// when both its halves bound one face on a periodic wall surface — a
/// lamina wall, closed on itself across it. Otherwise it rests where
/// [`topo::Body::describe_at_rest`] reads it resting (`topo::rest_locus`):
/// a plane wall's meridian (module docs — a plane's chart closes in no
/// direction, and one surface on both sides determines no locus, so D2's
/// conventional split applies), and a wire wall's angle-0 meridian,
/// which parts its two π-bands rather than closing either. Carrier and
/// interval kept verbatim either way.
pub(super) fn upgrade_meridian_wrap<T: Decide + topo::AtRestPolicy>(
    body: &mut Body<T>,
    edge: EdgeKey,
    wall: SurfaceKey,
    tol: Tol,
) -> Result<(), RevolveError> {
    let is_plane = matches!(
        body.get_surface(wall).unwrap_or_else(|| {
            unreachable!("wall surface {wall:?} was read off a live wall face of this revolve")
        }),
        geom::Surface::Plane { .. }
    );
    let (f_plus, f_minus) = topo::readback::edge_sides(body, edge)
        .unwrap_or_else(|_| {
            unreachable!("meridian {edge:?} was minted by this revolve and is live")
        })
        .faces();
    if is_plane || f_plus != f_minus {
        // The meridian is at rest, and the scaffolding door it was
        // minted through is for edges whose surfaces do not exist yet
        // (D3's transience fence). So it is described where it rests
        // (`Body::describe_at_rest`, which asks the rule over a pair).
        body.describe_at_rest(edge, tol)?;
        return Ok(());
    }
    crate::swept::describe_wrap_edge(body, edge, wall, tol)?;
    Ok(())
}
