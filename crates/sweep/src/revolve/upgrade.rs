//! The post-construction description upgrades a revolve performs (the
//! ratified prefer-intrinsic decisions, M2-LOG 2026-07-19): cap–wall
//! meridian rims and cap–cap axis edges to `Intersection`, latitude
//! joins to `Intersection`, full-revolve meridians to `Seam`. All
//! witnesses are the carrier's mid-parameter point (the S2 witness
//! contract; a full-period rim's witness is the start point's
//! antipode); every re-description restates the stored carrier and
//! interval verbatim (`topo::Body::restate_stored_edge`, or
//! `describe_at_rest`).

use geom_brep::{MustCarryDescription, WedgeRefusal};
use geom_core::Decide;
use topo::construction::Construct;
use topo::{Body, EdgeKey, SurfaceKey};

use super::RevolveError;
use geom_core::Tol;

/// Upgrades one edge between the faces wearing `s1` and `s2` by its
/// whole wedge (the ratified prefer-intrinsic decisions), the fold tier 3
/// reads at every station, read across the edge by
/// [`topo::Body::dihedral_across`]: cap–wall meridian rims, cap–cap axis
/// edges (partial), latitude joins and full-revolve latitude rims all
/// funnel here. A crease is the pair's `Intersection`; a smooth join
/// stores what the must-carry rule demands. A revolve join's carrier is a
/// latitude CIRCLE, which the jet certificate's circle arm covers on
/// every surface of revolution, so a jet-DETERMINATE smooth join is a
/// genuine `TangentIntersection` (D2/OQ7), and an under-determined one (a
/// G2 conventional join, a same-surface split: `κ_rel` at zero) rests as
/// the pair it lies on (D3's transience fence), its scaffolding
/// pushforward kept beside it as the authority record. In band is the
/// typed error `sliver` builds from the escalation, and stations
/// that disagree refuse [`RevolveError::SmoothJoinRefuted`]. Two planes
/// at the π seam return the verdict that the two faces are one carrier.
pub(super) fn upgrade_intersection<T: Decide + topo::AtRestPolicy>(
    body: &mut Body<T>,
    edge: EdgeKey,
    s1: SurfaceKey,
    s2: SurfaceKey,
    band: geom_core::Band,
    sliver: impl Fn(geom_brep::WedgeEscalation) -> RevolveError,
    tol: Tol,
) -> Result<Option<topo::SmoothAcross>, RevolveError> {
    let across = match body.dihedral_across(edge, [s1, s2], band)? {
        Ok(across) => across,
        Err(WedgeRefusal::InBand(escalation)) => {
            return Err(sliver(escalation));
        }
        Err(WedgeRefusal::Mixed(mixed)) => {
            return Err(RevolveError::SmoothJoinRefuted { edge, mixed });
        }
    };
    match across.description {
        MustCarryDescription::Intrinsic(description) => {
            body.restate_stored_edge(edge, description, tol)?;
        }
        MustCarryDescription::Conventional => body.describe_at_rest(edge, tol)?,
    }
    Ok(across.one_carrier)
}

/// Re-describes a full-revolve meridian as its wall's wrap edge (D1):
/// both its halves bound its one curved wall, which closes on itself
/// across it. A plane wall's meridian is a slit the revolve kills before
/// this, so none reaches here; the restatement refuses an edge whose
/// halves bound two faces. Carrier and interval kept verbatim, and the
/// stored certificate the one a fresh run mints
/// ([`crate::swept::describe_wrap_edge_fresh`]).
pub(super) fn upgrade_meridian_wrap<T: Decide + topo::AtRestPolicy>(
    body: &mut Body<T>,
    edge: EdgeKey,
    wall: SurfaceKey,
    tol: Tol,
) -> Result<(), RevolveError> {
    crate::swept::describe_wrap_edge_fresh(body, edge, wall, tol)?;
    Ok(())
}
