//! **An edge's wedge, read whole**: the one door from an edge to its
//! wedge's faces and walk ([`edge_wedge_walk`]), and the one composition
//! of its verdict ([`edge_wedge`]): the seam read by identity where the
//! two sides are one surface record ([`one_surface_seam`]), else the
//! whole-edge fold over the walk levered at its faces' extent. What the
//! verdict demands the edge store is read off the same walk
//! ([`edge_wedge_description`]). Tier 3's check 4 reads its edges
//! through this composition, so what a constructor stores and what tier
//! 3 accepts are read from one verdict.

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
    walk_of(body, edge).map(|read| read.walk)
}

/// An edge's walk with the sides it was built from.
struct EdgeRead<'a, T: Decide> {
    walk: geom_brep::WedgeWalk<'a, T>,
    sides: crate::readback::EdgeSides,
}

/// [`edge_wedge_walk`], keeping the sides.
fn walk_of<T: Decide>(body: &Body<T>, edge: EdgeKey) -> Option<EdgeRead<'_, T>> {
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
    Some(EdgeRead {
        walk: geom_brep::WedgeWalk::along_edge(
            side(sides.plus)?,
            side(sides.minus)?,
            curve.carrier(),
            t0,
            t1,
        ),
        sides,
    })
}

/// **A seam read by identity**: an edge whose two sides lie on ONE
/// surface record with one sense is interior to that surface, which is
/// smooth and shares its own curvature there, so its wedge is the seam
/// with an under-determined bend — what the whole-edge fold reads at
/// `f64`, where the sagitta of a surface against itself is an exact zero
/// — and no margin is read for it. Keyed on the record, not on a value,
/// so it answers alike at every scalar: over a box the two evaluations
/// of one surface do not cancel, and the fold would read the zero as an
/// enclosure `[0, w]` in band (ONEHOME fork 259: a cylinder leaf taken
/// alone at `Sym`). `None` where the sides are two records or two
/// senses: the fold reads them.
fn one_surface_seam<T: geom_core::Real>(
    body: &Body<T>,
    sides: crate::readback::EdgeSides,
) -> Option<geom_brep::WedgeVerdict> {
    let sense = |face| body.get_face(face).map(|f| f.sense);
    (sides.plus.surface == sides.minus.surface
        && sense(sides.plus.face)? == sense(sides.minus.face)?)
    .then_some(geom_brep::WedgeVerdict::Seam {
        bend: geom_brep::Bend::UnderDetermined,
        margin: None,
    })
}

/// The one composition of an edge's verdict over its walk, levered at
/// `reach`.
fn verdict_of<T: Decide>(
    body: &Body<T>,
    read: &EdgeRead<'_, T>,
    reach: geom_core::Lever<T>,
    band: Band,
    names: geom_brep::FoldNames,
) -> geom_brep::WedgeVerdict {
    one_surface_seam(body, read.sides)
        .unwrap_or_else(|| geom_brep::edge_wedge_over_as(&read.walk, reach, band, names))
}

/// **An edge's wedge** (module docs), levered at its faces' extent
/// ([`crate::edge_wedge_reach`]). `None` where the edge carries no
/// certified curve or a record on the way does not resolve.
#[must_use]
pub fn edge_wedge<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    band: Band,
) -> Option<geom_brep::WedgeVerdict> {
    edge_wedge_as(body, edge, band, geom_brep::FoldNames::SHAPE)
}

/// [`edge_wedge`], each corner's sign and each tangency's end decided
/// under `names`: the door for a reader whose side decides which body it
/// builds, which records those decisions under names of its own
/// (`geom_brep::DIHEDRAL_SIGN`'s docs).
#[must_use]
pub fn edge_wedge_as<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    band: Band,
    names: geom_brep::FoldNames,
) -> Option<geom_brep::WedgeVerdict> {
    let read = walk_of(body, edge)?;
    let reach = crate::edge_wedge_reach(body, edge)?;
    Some(verdict_of(body, &read, reach, band, names))
}

/// [`edge_wedge`] levered at a `reach` the caller has read for the edge
/// already (check 4 reads every edge's once).
pub(crate) fn edge_wedge_at<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    reach: geom_core::Lever<T>,
    band: Band,
) -> Option<geom_brep::WedgeVerdict> {
    let read = walk_of(body, edge)?;
    Some(verdict_of(
        body,
        &read,
        reach,
        band,
        geom_brep::FoldNames::SHAPE,
    ))
}

/// **What an edge's wedge demands it store**
/// ([`geom_brep::WedgeWalk::description`]): its [`edge_wedge`] over its
/// two faces' surface records, read off the walk the verdict was read
/// over. A constructor that wrote the edge restates it with this. `None`
/// as [`edge_wedge`].
#[must_use]
pub fn edge_wedge_description<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    band: Band,
) -> Option<Result<geom_brep::MustCarryDescription<T>, geom_brep::WedgeRefusal>> {
    edge_wedge_read(body, edge, band).map(|(_, description)| description)
}

/// [`edge_wedge`] with what it demands the edge store
/// ([`edge_wedge_description`]), both off one walk.
pub(crate) fn edge_wedge_read<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    band: Band,
) -> Option<(
    geom_brep::WedgeVerdict,
    Result<geom_brep::MustCarryDescription<T>, geom_brep::WedgeRefusal>,
)> {
    let read = walk_of(body, edge)?;
    let verdict = verdict_of(
        body,
        &read,
        crate::edge_wedge_reach(body, edge)?,
        band,
        geom_brep::FoldNames::SHAPE,
    );
    let pair = geom_brep::SurfacePair::new(read.sides.plus.surface, read.sides.minus.surface);
    Some((verdict, read.walk.description(verdict, pair)))
}
