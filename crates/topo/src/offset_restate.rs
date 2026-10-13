//! An edge's description re-stated for an offset — the rules the three
//! offset doors ([`crate::replace_faces_offset`],
//! [`crate::offset_charts_together`],
//! [`crate::offset_planes_together`]) share, in one
//! home none of them owns.
//!
//! A door here moves some charts and holds others; each side of an
//! edge is the key it wears and whether its chart moves
//! ([`chart_moves`]). What the doors differ in is how a declaration
//! travels with the edge, so each passes that as `carried`.

use geom_brep::{EdgeAuthority, EdgeDescription, EdgeDescriptionSpec, MappedCurve, SurfacePair};
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Point3, Real, Sign};

use crate::geometry::SurfaceKey;
use crate::replace_face::ReplaceFaceError;

/// Whether a chart asked to offset by `distance` moves at all: a chart
/// asked to move nothing holds, and keeps its key.
pub(crate) fn chart_moves<T: Decide>(distance: T, band: Band) -> Result<bool, ReplaceFaceError<T>> {
    match decide("offset_chart_motion", Margin::of(distance), band) {
        Ok(Sign::Zero) => Ok(false),
        Ok(_) => Ok(true),
        Err(source) => Err(ReplaceFaceError::Escalated { source }),
    }
}

/// `description` re-stated for the moved edge, whose new carrier meets
/// its new mid-parameter at `mid`.
///
/// - an **intrinsic** one keeps its (about to be remapped) surfaces
///   with the witness at `mid`;
/// - a **chart image** in a chart that holds while the edge's other
///   side moves is restated against the moving side
///   ([`held_neighbour_image`]). Any other image is derived afresh
///   from the moved carrier (`image: None`, the spec's request to
///   derive it) when the edge `slides` within its chart, and kept when
///   it does not — the chart moves rigidly with the edge, or neither
///   moves;
/// - a **pair** the two surfaces under-determined is re-asked the
///   must-carry rule against the moved pair through `re_ask`
///   ([`re_ask_on_pair`]), which the door supplies with the edge's two
///   sides' moved surfaces and its moved carrier, handed the
///   declaration;
/// - a **declaration** or a **scaffold** is 3-space sketch data, and
///   rides `carried`.
///
/// `sides` is each side's key and whether its chart moves. Nothing
/// stated here is trusted: the attach layer certifies the spec against
/// the new carrier and refuses a mismatch.
pub(crate) fn restate<T: Real>(
    description: EdgeDescription<T>,
    authority: EdgeAuthority<T>,
    sides: [(SurfaceKey, bool); 2],
    slides: bool,
    mid: Point3<T>,
    carried: impl Fn(MappedCurve<T>) -> Result<MappedCurve<T>, ReplaceFaceError<T>>,
    re_ask: impl FnOnce(Option<MappedCurve<T>>) -> Result<EdgeDescriptionSpec<T>, ReplaceFaceError<T>>,
) -> Result<EdgeDescriptionSpec<T>, ReplaceFaceError<T>> {
    let declared = match authority {
        EdgeAuthority::Derived => None,
        EdgeAuthority::Declared(mc) => Some(carried(mc)?),
    };
    Ok(match description {
        EdgeDescription::Intersection { pair, .. } => {
            EdgeDescriptionSpec::Intersection { pair, witness: mid }
        }
        EdgeDescription::TangentIntersection { pair, .. } => {
            EdgeDescriptionSpec::TangentIntersection { pair, witness: mid }
        }
        EdgeDescription::OnPair { .. } => re_ask(declared)?,
        EdgeDescription::Chart(c) => match beside_moving(c.surface, sides) {
            Some(moving) => held_neighbour_image(moving, c.surface, declared, mid),
            None => EdgeDescriptionSpec::Chart {
                surface: c.surface,
                image: (!slides).then_some(c.pcurve),
                wrap: c.wrap,
                declared,
            },
        },
        EdgeDescription::Scaffold(m) => EdgeDescriptionSpec::Scaffold(carried(m)?),
    })
}

/// **An `OnPair` edge moved onto `carrier`**, re-asked the must-carry
/// rule against the pair as it now stands (D2): `surfaces` are the
/// pair's two surfaces after the move, each with the key the edge's
/// description names. The pair may now determine the locus — a corner
/// (`Intersection`) or a jet-determinate tangency
/// (`TangentIntersection`), each witnessed at the carrier's middle and
/// derived, so a declaration does not ride it — or still
/// under-determine it, when it stays `OnPair` with its declaration.
///
/// # Errors
///
/// [`ReplaceFaceError::MixedEdge`] where the moved edge reads a corner
/// at one station and smooth at its neighbour;
/// [`ReplaceFaceError::Escalated`] where a station is in band.
pub(crate) fn re_ask_on_pair<T: Decide>(
    edge: crate::entity::EdgeKey,
    surfaces: [(SurfaceKey, &geom::Surface<T>); 2],
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
    declared: Option<MappedCurve<T>>,
    band: Band,
) -> Result<EdgeDescriptionSpec<T>, ReplaceFaceError<T>> {
    use geom_brep::MustCarryVerdict as V;
    let [(ka, a), (kb, b)] = surfaces;
    let (pair, [sa, sb]) = SurfacePair::sorted(ka, a, kb, b);
    let witness = carrier.mid_point(t0, t1);
    let extent =
        geom_brep::edge_extent(carrier, t0, t1, carrier.eval(t0).distance(carrier.eval(t1)));
    Ok(
        match geom_brep::must_carry_over_edge(sa, sb, carrier, t0, t1, extent, band) {
            V::Transverse => EdgeDescriptionSpec::Intersection { pair, witness },
            V::JetDeterminate => EdgeDescriptionSpec::TangentIntersection { pair, witness },
            V::UnderDetermined => EdgeDescriptionSpec::OnPair { pair, declared },
            V::Mixed(stations) => return Err(ReplaceFaceError::MixedEdge { edge, stations }),
            V::InBand(escalation) => {
                return Err(ReplaceFaceError::Escalated {
                    source: escalation.diag(),
                });
            }
        },
    )
}

/// An edge whose description is an image in `held`'s chart, `held`
/// holding still while the edge's other side `moving` moves, restated
/// for the move.
///
/// The image names no key the moving side wears after the move, and
/// the edge has left the locus it draws, so the edge is stated as what
/// it now is: the section of the two charts. A declaration rides only a
/// chart image (an intersection has no slot for one), so a declared
/// edge moves into the moving side's own chart instead, its image
/// derived from the moved carrier. `moving` is the key the side wears
/// now; the door's remap re-points it at the chart it mints.
pub(crate) fn held_neighbour_image<T: Real>(
    moving: SurfaceKey,
    held: SurfaceKey,
    declared: Option<MappedCurve<T>>,
    witness: Point3<T>,
) -> EdgeDescriptionSpec<T> {
    match declared {
        None => EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(moving, held),
            witness,
        },
        Some(mc) => EdgeDescriptionSpec::chart(moving).declared_by(mc),
    }
}

/// The moving side's key, where the chart `named` is the edge's other
/// side and holds still; each side is its key and whether it moves.
pub(crate) fn beside_moving(
    named: SurfaceKey,
    sides: [(SurfaceKey, bool); 2],
) -> Option<SurfaceKey> {
    match sides {
        [(held, false), (moving, true)] | [(moving, true), (held, false)] if held == named => {
            Some(moving)
        }
        _ => None,
    }
}
