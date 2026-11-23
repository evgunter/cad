//! The census's confirmation direction, run alone, and what each of
//! its answers on a recorded pair means to a gate.

use geom_core::Band;

use crate::body::Body;
use crate::boolean::ContactRecords;
use crate::validate::{CensusUnsupportedCause, ValidationError};

/// **What the census answered on the records a gate was handed**
/// ([`confirm_only`]). A record with no finding here is confirmed.
#[derive(Debug, Default)]
pub(crate) struct Confirmation {
    /// The census read the pair and the record is not what it found, or
    /// it read it and could not decide: the gate refuses on these.
    pub(crate) refused: Vec<ValidationError>,
    /// The census has no lane that reads the pair: the record is held
    /// on its citation, and the body's bar is tier 3
    /// ([`crate::Bar::Tier3`]).
    pub(crate) no_lane: Vec<ValidationError>,
}

/// One finding of the confirmation direction, as a gate takes it.
enum Reading {
    /// No lane reads the pair.
    NoLane,
    /// Read, and not confirmed.
    NotConfirmed,
}

/// **The confirmation direction alone**: each of `contacts` read where
/// it lies ([`super::confirm_records`]), every finding one record's that
/// did not confirm, sorted by what the census did ([`Confirmation`]). A
/// deciding door's gate runs this over the records it is handed
/// (`[topology.tiers]`); it searches for no meeting, so it pays nothing
/// for the body's other cells.
pub(crate) fn confirm_only<T: crate::props::AtRestPolicy>(
    body: &Body<T>,
    contacts: &ContactRecords,
    band: Band,
    region: Option<T::RegionLane>,
) -> Confirmation {
    let mut errors = Vec::new();
    if !contacts.is_empty() {
        super::confirm_records(
            body,
            &super::snapshot(body),
            contacts,
            band,
            region,
            &mut errors,
        );
    }
    let mut out = Confirmation::default();
    for finding in errors {
        match reading(&finding) {
            Reading::NoLane => out.no_lane.push(finding),
            Reading::NotConfirmed => out.refused.push(finding),
        }
    }
    out
}

/// **How a gate takes one finding of the confirmation direction.** The
/// lane causes are matched whole, with no arm for "any other": a cause
/// added to one of them is placed here by whoever adds it. A finding
/// of any other kind is not a lane's answer about a pair, and refuses.
fn reading(finding: &ValidationError) -> Reading {
    use crate::chart_region::ChartRegionError as Region;
    use crate::contact::ContactRefusal;
    match finding {
        ValidationError::CensusUnsupported { cause, .. } => match cause {
            // The tangent certifier's set does not hold the pair (two
            // carriers with no closed-form tangent locus over this
            // extent): nothing was read. Its lane is owed by
            // `work/heldrecords/a-recorded-tangency-past-the-certifiers-set-is-held-on-its-citation.md`.
            CensusUnsupportedCause::ContactLane(ContactRefusal::NotCertifiable { .. }) => {
                Reading::NoLane
            }
            // In the band: the certifier looked and could not decide.
            CensusUnsupportedCause::ContactLane(ContactRefusal::Escalated { .. }) => {
                Reading::NotConfirmed
            }
            CensusUnsupportedCause::ChartRegion(region) => match region {
                // The boundaries touch, so the overlap is undecidable
                // at this ε either way: looked, and could not decide.
                Region::TouchingBoundary
                // In the band, or every ray grazed: looked, undecided.
                | Region::Escalated(_)
                | Region::RayExhausted
                // The carriers disagree by more than the pair absorbs:
                // read, and not one contact.
                | Region::CarrierTilt
                // The body's own state is wrong: a kernel defect.
                | Region::Corrupt
                // The region door's limits on a shape or a size. No
                // recorded pair reaches one in any suite, so none is
                // classed as a missing lane: each refuses until a body
                // a door builds shows it holding a true record.
                | Region::ChartDivergence { .. }
                | Region::NonPlanarTrim { .. }
                | Region::TrimOffFace { .. }
                | Region::MissingCache { .. }
                | Region::ArmUnbounded { .. }
                | Region::SeamBranch
                | Region::PeriodFold
                | Region::DegenerateLoop { .. }
                | Region::ShortLoop { .. }
                | Region::WitnessSegmentCapExceeded { .. }
                | Region::WitnessCellCapExceeded { .. }
                | Region::EdgeScale
                | Region::FaceUncovered(_) => Reading::NotConfirmed,
            },
            // A curved edge, a meeting, a face's bound or a containment
            // the census could not read. As the region door's limits
            // above: unreached by a recorded pair, refused until shown.
            CensusUnsupportedCause::CurvedEdge(_)
            | CensusUnsupportedCause::Meeting(_)
            | CensusUnsupportedCause::FaceUnboundable
            | CensusUnsupportedCause::Containment(_) => Reading::NotConfirmed,
        },
        // Stale, contradicted, in the band, undecidable, and whatever
        // else the confirmation raises: the census read the pair, or
        // the body, and did not confirm the record.
        _ => Reading::NotConfirmed,
    }
}
