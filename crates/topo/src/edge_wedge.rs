//! **An edge's wedge, read whole**: the one door from an edge to its
//! wedge's faces and walk ([`edge_wedge_walk`]), the whole-edge fold over
//! it levered at its faces' extent ([`edge_wedge`]), and what that
//! verdict demands the edge store ([`edge_wedge_description`]). Tier 3's
//! check 4 reads the same fold over the same walk, so what a constructor
//! stores and what tier 3 accepts are read from one verdict.

use geom_core::{Band, Decide};

use crate::{Body, EdgeKey};

/// **An edge's wedge walk** ([`geom_brep::WedgeWalk::along_edge`]): the
/// face of its `he_plus` as the plus face and of its `he_minus` as the
/// minus face, each with its `Face::sense` bit, and the interior stations
/// of its certified curve, whose increasing parameter is the way
/// `he_plus` walks it (`crate::entity`'s orientation section). The one
/// door from an edge to its wedge's faces and walk, so no reader pairs a
/// face order with a direction by hand (`geom_brep::Convexity`'s
/// convention). `None` where a record on the way does not resolve, which
/// tier 1 rules out.
pub fn edge_wedge_walk<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
) -> Option<geom_brep::WedgeWalk<'_, T>> {
    let e = body.get_edge(edge)?;
    let curve = body
        .curves
        .get(e.curve)
        .and_then(crate::CurveGeom::certified)?;
    let sides = crate::readback::edge_sides(body, edge).ok()?;
    let side = |side: crate::readback::EdgeSide| {
        Some(geom_brep::SideFace {
            surface: body.surfaces.get(side.surface)?,
            sense: body.get_face(side.face)?.sense,
        })
    };
    let (t0, t1) = curve.params();
    Some(geom_brep::WedgeWalk::along_edge(
        side(sides.plus)?,
        side(sides.minus)?,
        curve.carrier(),
        t0,
        t1,
    ))
}

/// **An edge's wedge**: the whole-edge fold ([`geom_brep::edge_wedge_over`])
/// over its walk ([`edge_wedge_walk`]), levered at its faces' extent
/// ([`crate::edge_wedge_reach`]). `None` where the edge carries no
/// certified curve or a record on the way does not resolve.
#[must_use]
pub fn edge_wedge<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    band: Band,
) -> Option<geom_brep::WedgeVerdict> {
    let walk = edge_wedge_walk(body, edge)?;
    let reach = crate::edge_wedge_reach(body, edge)?;
    Some(geom_brep::edge_wedge_over(&walk, reach, band))
}

/// **What an edge's wedge demands it store**
/// ([`geom_brep::WedgeVerdict::description`]): its [`edge_wedge`] over
/// its two faces' surface records, witnessed at its carrier's middle,
/// with the jet certificate's lane read over its carrier
/// ([`geom_brep::tangent_certificate_lane`]). A constructor that wrote
/// the edge restates it with this. `None` as [`edge_wedge`].
#[must_use]
pub fn edge_wedge_description<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    band: Band,
) -> Option<Result<geom_brep::MustCarryDescription<T>, geom_brep::PairRefusal>> {
    demanded(body, edge, edge_wedge(body, edge, band)?)
}

/// What `verdict`, `edge`'s wedge, demands it store
/// ([`edge_wedge_description`]).
pub(crate) fn demanded<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    verdict: geom_brep::WedgeVerdict,
) -> Option<Result<geom_brep::MustCarryDescription<T>, geom_brep::PairRefusal>> {
    let curve = body.stored_curve(edge).ok()?;
    let sides = crate::readback::edge_sides(body, edge).ok()?;
    let lane = geom_brep::tangent_certificate_lane(
        curve.carrier(),
        body.get_surface(sides.plus.surface)?,
        body.get_surface(sides.minus.surface)?,
    );
    let pair = geom_brep::SurfacePair::new(sides.plus.surface, sides.minus.surface);
    Some(verdict.description(lane, pair, curve.mid_point()))
}
