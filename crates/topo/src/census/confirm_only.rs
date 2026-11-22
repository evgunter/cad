//! The census's confirmation direction, run alone.

use geom_core::Band;

use crate::body::Body;
use crate::boolean::ContactRecords;
use crate::validate::ValidationError;

/// **The confirmation direction alone**: each of `contacts` read where
/// it lies ([`super::confirm_records`]), and every finding is one
/// record's that did not confirm: stale, contradicted, in the band,
/// outside a lane. A deciding door's gate runs this over the records it
/// is handed (`[topology.tiers]`); it searches for no meeting, so it
/// pays nothing for the body's other cells.
pub(crate) fn confirm_only<T: crate::props::AtRestPolicy>(
    body: &Body<T>,
    contacts: &ContactRecords,
    band: Band,
    region: Option<T::RegionLane>,
) -> Vec<ValidationError> {
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
    errors
}
