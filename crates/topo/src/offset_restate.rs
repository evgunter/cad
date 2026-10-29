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

use geom_brep::{EdgeAuthority, EdgeDescription, EdgeDescriptionSpec, MappedCurve};
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Real, Sign};

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
///   side moves has left the locus it draws, and is re-asked as the
///   moved pair below. Any other image is derived afresh
///   from the moved carrier (`image: None`, the spec's request to
///   derive it) when the edge `slides` within its chart, and kept when
///   it does not — the chart moves rigidly with the edge, or neither
///   moves;
/// - a **pair** the two surfaces under-determined, and such an image,
///   is re-asked the must-carry rule against the moved pair through
///   `re_ask`
///   ([`re_ask_on_pair`]), which the door supplies with the edge's two
///   sides' moved surfaces and its moved carrier, handed the
///   declaration to carry where the rule reads the pair conventional;
/// - an image the door keeps (not a wrap) is asked the rule again
///   through `re_ask_chart` ([`re_ask_chart`]), against the pair as the
///   door leaves it: the rule may demand the intrinsic form where the
///   surfaces moved or the edge slid. No valid body in the suites
///   reaches that demand (an image kept on two distinct surfaces there
///   reads under-determined); a hand-stated image on a corner does;
/// - a **declaration** or a **scaffold** is 3-space sketch data, and
///   rides `carried` — only where the restated edge keeps it, so a
///   declaration the rule's intrinsic reading drops is never asked to
///   travel.
///
/// `sides` is each side's key and whether its chart moves. Nothing
/// stated here is trusted: the attach layer certifies the spec against
/// the new carrier and refuses a mismatch.
#[allow(clippy::too_many_arguments)]
pub(crate) fn restate<T: geom_core::SpanLocate>(
    description: EdgeDescription<T>,
    authority: EdgeAuthority<T>,
    sides: [(SurfaceKey, bool); 2],
    slides: bool,
    (carrier, t0, t1): (&geom::Curve3<T>, T, T),
    carried: impl Fn(MappedCurve<T>) -> Result<MappedCurve<T>, ReplaceFaceError<T>>,
    re_ask: impl FnOnce(
        &dyn Fn() -> Result<Option<MappedCurve<T>>, ReplaceFaceError<T>>,
    ) -> Result<EdgeDescriptionSpec<T>, ReplaceFaceError<T>>,
    re_ask_chart: impl FnOnce() -> Result<Option<EdgeDescriptionSpec<T>>, ReplaceFaceError<T>>,
) -> Result<EdgeDescriptionSpec<T>, ReplaceFaceError<T>> {
    let declared = || match authority {
        EdgeAuthority::Derived => Ok(None),
        EdgeAuthority::Declared(mc) => Ok(Some(carried(mc)?)),
    };
    Ok(match description {
        EdgeDescription::Intersection { pair, selector } => EdgeDescriptionSpec::Intersection {
            pair,
            selector: selector.repinned(carrier, t0, t1),
        },
        EdgeDescription::TangentIntersection { pair, .. } => {
            EdgeDescriptionSpec::TangentIntersection {
                pair,
                witness: carrier.mid_point(t0, t1),
            }
        }
        EdgeDescription::OnPair { .. } => re_ask(&declared)?,
        EdgeDescription::Chart(c) => match beside_moving(c.surface, sides) {
            // The image names no key the moving side wears after the
            // move, and the edge has left the locus it draws: it lies on
            // the moved pair, which the rule is asked again, and it
            // stores what the rule reads, declared or not.
            Some(_) => re_ask(&declared)?,
            None => match (!c.wrap).then(re_ask_chart).transpose()?.flatten() {
                Some(intrinsic) => intrinsic,
                None => EdgeDescriptionSpec::Chart {
                    surface: c.surface,
                    image: (!slides).then_some(c.pcurve),
                    wrap: c.wrap,
                    declared: declared()?,
                },
            },
        },
        EdgeDescription::Scaffold(m) => EdgeDescriptionSpec::Scaffold(carried(m)?),
    })
}

/// **An `OnPair` edge whose pair or extent a door changed**, re-asked
/// the must-carry rule as it now stands ([`geom_brep::reask_pair`]):
/// `surfaces` are the pair's two surfaces after the door, each with the
/// key the edge's description names, and `carrier` over `(t0, t1)` is
/// the edge after it, its wedge levered at `reach`, its faces' extent as
/// the body stands before the door moves them ([`fn@crate::wedge_reach`]).
/// `declared`
/// is the edge's declaration as the
/// door transports it, asked only where the rule reads the pair
/// conventional: an intrinsic reading drops it, so its transport
/// cannot refuse there.
///
/// # Errors
///
/// [`ReplaceFaceError::MixedEdge`] where the moved edge reads a corner
/// at one station and smooth at its neighbour;
/// [`ReplaceFaceError::Escalated`] where a station is in band; and
/// whatever `declared` refuses, where the reading keeps it.
pub(crate) fn re_ask_on_pair<T: Decide>(
    edge: crate::entity::EdgeKey,
    surfaces: [(SurfaceKey, &geom::Surface<T>); 2],
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
    reach: T,
    declared: impl FnOnce() -> Result<Option<MappedCurve<T>>, ReplaceFaceError<T>>,
    band: Band,
) -> Result<EdgeDescriptionSpec<T>, ReplaceFaceError<T>> {
    let read = geom_brep::reask_pair(surfaces, carrier, (t0, t1), reach, None, band)
        .map_err(|refusal| pair_refusal(edge, refusal))?;
    Ok(match read {
        EdgeDescriptionSpec::OnPair { pair, .. } => EdgeDescriptionSpec::OnPair {
            pair,
            declared: declared()?,
        },
        intrinsic => intrinsic,
    })
}

/// **A chart image an offset door keeps, re-asked the must-carry rule**
/// ([`geom_brep::reask_chart`]) between `surfaces`, the edge's two sides
/// after the door, over `carrier` on `(t0, t1)` levered at `reach`
/// ([`fn@crate::wedge_reach`]): `Some` is the intrinsic
/// description the rule now demands, `None` keeps the image.
///
/// # Errors
///
/// As [`re_ask_on_pair`].
pub(crate) fn re_ask_chart<T: Decide>(
    edge: crate::entity::EdgeKey,
    surfaces: [(SurfaceKey, &geom::Surface<T>); 2],
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
    reach: T,
    band: Band,
) -> Result<Option<EdgeDescriptionSpec<T>>, ReplaceFaceError<T>> {
    geom_brep::reask_chart(surfaces, carrier, (t0, t1), reach, band)
        .map_err(|refusal| pair_refusal(edge, refusal))
}

/// An offset door's refusal of the rule's reading of `edge`: a mixed
/// edge, or an in-band station.
pub(crate) fn pair_refusal<T: Real>(
    edge: crate::entity::EdgeKey,
    refusal: geom_brep::PairRefusal,
) -> ReplaceFaceError<T> {
    match refusal {
        geom_brep::PairRefusal::Mixed(stations) => ReplaceFaceError::MixedEdge { edge, stations },
        geom_brep::PairRefusal::InBand(escalation) => ReplaceFaceError::Escalated {
            source: escalation.diag(),
        },
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
