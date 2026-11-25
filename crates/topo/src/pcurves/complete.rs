//! **Completion**: a body's pcurve rows made complete — the passage
//! from construction state to a finished body or a leaf
//! (`[one-way.places]`, a body's at-rest validity), and the same
//! completion run early where a reader inside the pipeline needs the
//! rows (`crate::finish::complete_in_place`, its one home).
//!
//! It drops the rows on dead half-edges, keeps every stored row, and
//! derives each missing one. It never re-derives a row it can reach, so
//! a stale row a door left behind survives to the verdict and is
//! refused there, never replaced. In construction state a door keeps a
//! stored row current or drops it (the posture table in this module's
//! parent); completion only fills what is absent.
//!
//! Per face:
//!
//! - **A face that stores no row** — no image and no element — is
//!   derived whole ([`derive_face`]), the rows the mint writes there.
//! - **A complete face** keeps its rows.
//! - **A half-minted face** — one storing any image or any element —
//!   keeps every stored image and element, and
//!   on each loop with a gap the walk derives what is missing: the
//!   image of every rowless half ([`derive_image`]) and the element of
//!   every joint that stores none, decided between the two images
//!   ([`walk_cycle`]), the loop's closure checked. A stored image and a
//!   stored element stand as the walk reads them; an image is a function
//!   of its edge and the chart and an element of its two images, so
//!   where the kept rows are current what the walk fills is what the
//!   mint would. Where a kept row is stale the filled joints disagree
//!   with it and the verdict's pass refuses the loop.
//!
//! A face is read whole before it is excused, as the mint reads it: one
//! whose every refusal is not owed ([`not_owed`]) is left as found, and
//! the verdict excuses exactly those. A face on a chart that mints
//! nothing derives nothing.

use geom_brep::PcurveCache;
use geom_core::Tol;
use geom_core::predicate::Band;

use super::{
    DescribedChart, PcurveMintError, SiteImage, WalkFail, WalkItem, Walked, certify_walked,
    derive_face, derive_image, entry_vertex, first_owed, fitted_door, half_edge_carrier, is_plus,
    joint_refusal, not_owed, stored_edge, stored_reach, stored_rows, walk_cycle, walk_runs,
};
use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey};
use crate::joint::JointElement;
use crate::props::AtRestPolicy;

/// What completion writes for one half-edge: its image and the element
/// into it, or the element alone where the image stands.
enum Fill<T: geom_core::Real> {
    Row(PcurveCache<T>, Option<JointElement>),
    Element(JointElement),
}

/// **`body`'s pcurve rows completed** (module docs). Each face's rows
/// are derived before any is written, and a face that refuses is left as
/// found, so the verdict that follows reads it and every other finding
/// besides.
///
/// # Errors
///
/// Every refusal whose rows are owed, a face's first as the mint reports
/// it, in face order; or the band's alone.
pub(crate) fn complete_pcurves<T: AtRestPolicy>(
    body: &mut Body<T>,
    tol: Tol,
) -> Result<(), Vec<PcurveMintError>> {
    let band = Band::linear(tol).map_err(|e| vec![PcurveMintError::Band(e)])?;
    let dead: Vec<HalfEdgeKey> = body
        .pcurves
        .keys()
        .chain(body.joints.keys())
        .filter(|&he| !body.half_edges.contains_key(he))
        .collect();
    body.drop_rows(dead);
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    let mut fills: Vec<(HalfEdgeKey, Fill<T>)> = Vec::new();
    let mut refused = Vec::new();
    for face in faces {
        let mut face_fills = Vec::new();
        match fill_face(body, face, band, &mut face_fills) {
            Ok(()) => fills.extend(face_fills),
            Err(e) if not_owed::<T>(&e) => {}
            Err(e) => refused.push(e),
        }
    }
    for (he, fill) in fills {
        match fill {
            Fill::Row(image, element) => body.write_row(he, image, element),
            Fill::Element(element) => body.write_joint(he, Some(element)),
        }
    }
    if refused.is_empty() {
        Ok(())
    } else {
        Err(refused)
    }
}

/// The rows completion writes on `face` (module docs), appended to
/// `out`.
fn fill_face<T: AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
    out: &mut Vec<(HalfEdgeKey, Fill<T>)>,
) -> Result<(), PcurveMintError> {
    let view = body.proven_face(face);
    let surface = view.surface();
    let Some(chart) = DescribedChart::minting(surface) else {
        return Ok(());
    };
    let rows = stored_rows(body, face);
    if rows.complete() {
        return Ok(());
    }
    // A face keeps whatever it stores: an image, or an element on a
    // joint whose images it does not store (`Construct::detach_pcurve`
    // takes the image and leaves the element). Only a face storing
    // neither is derived whole.
    let keeps = rows.stores
        || rows
            .loops
            .iter()
            .any(|(_, cycle)| cycle.iter().any(|&he| body.joint(he).is_some()));
    if true || !keeps {
        return match derive_face(body, face, band) {
            Ok(derived) => {
                out.extend(
                    derived
                        .into_iter()
                        .map(|(he, image, element)| (he, Fill::Row(image, element))),
                );
                Ok(())
            }
            Err(e) => Err(e),
        };
    }
    let sense = view.sense();
    let wedges = crate::wedge_reach::WedgeReach::new(body);
    let fitted = |w: &Walked<T>| fitted_door(w, surface, band);
    let mut face_fills: Vec<(HalfEdgeKey, Fill<T>)> = Vec::new();
    let mut derived: Vec<Walked<T>> = Vec::new();
    let mut refused: Vec<PcurveMintError> = Vec::new();
    let mut broken: Vec<&[HalfEdgeKey]> = Vec::new();
    for (lp, cycle) in &rows.loops {
        if !rows.gaps.iter().any(|gap| gap.r#loop == *lp) {
            continue;
        }
        let item = |i: usize| -> Result<WalkItem<T, SiteImage<'_, T>>, PcurveMintError> {
            let he = cycle[i];
            let plus = is_plus(body, he);
            let vertex = entry_vertex(body, he);
            let edge = stored_edge(body, he);
            if let Some(cache) = body.pcurve(he) {
                let (carrier, ..) = half_edge_carrier(body, he)?;
                let (t0, t1) = cache.params();
                return Ok(WalkItem {
                    image: SiteImage::Kept(cache.pcurve(), carrier),
                    t0,
                    t1,
                    plus,
                    vertex,
                    edge,
                });
            }
            let row = derive_image(body, he, surface, band)?;
            let (t0, t1) = row.1.window();
            Ok(WalkItem {
                image: SiteImage::Derived(row),
                t0,
                t1,
                plus,
                vertex,
                edge,
            })
        };
        let reach = |i: usize| stored_reach(&wedges, body, cycle[i]);
        let walked = match walk_cycle(
            chart,
            sense,
            cycle.len(),
            item,
            |i| body.joint(cycle[i]),
            &reach,
            band,
            true,
        ) {
            Ok(walked) => walked,
            Err(fail) => {
                refused.push(match fail {
                    WalkFail::Item(e) => e,
                    WalkFail::Miss { index, miss } => joint_refusal(cycle[index], miss),
                    WalkFail::NotClosed => PcurveMintError::LoopNotClosed { face },
                });
                broken.push(cycle);
                continue;
            }
        };
        for (&he, (image, .., element)) in cycle.iter().zip(walked) {
            match image {
                SiteImage::Derived(row) => derived.push(Walked::pinned(he, row, element)),
                SiteImage::Kept(..) => {
                    if body.joint(he).is_none()
                        && let Some(element) = element
                    {
                        face_fills.push((he, Fill::Element(element)));
                    }
                }
            }
        }
    }
    if !refused.is_empty() {
        // As the mint's (`derive_face`): a walk stops at its loop's first
        // refusal, so a face is read whole before it is excused — each
        // broken loop's covered runs walked, and every derived image
        // certified — and it is excused only when every refusal it meets
        // is ([`not_owed`]).
        if refused.iter().all(not_owed::<T>) {
            for cycle in broken {
                walk_runs(body, chart, sense, cycle, band, &mut derived, &mut refused);
            }
            if let Err(certs) = certify_walked(derived, surface, band, Some(&fitted)) {
                refused.extend(
                    certs
                        .into_iter()
                        .map(|(half_edge, error)| PcurveMintError::Certify { half_edge, error }),
                );
            }
        }
        return Err(first_owed::<T>(refused));
    }
    match certify_walked(derived, surface, band, Some(&fitted)) {
        Ok(certified) => face_fills.extend(
            certified
                .into_iter()
                .map(|(he, image, element)| (he, Fill::Row(image, element))),
        ),
        Err(refused) => {
            return Err(first_owed::<T>(
                refused
                    .into_iter()
                    .map(|(half_edge, error)| PcurveMintError::Certify { half_edge, error })
                    .collect(),
            ));
        }
    }
    out.extend(face_fills);
    Ok(())
}
