//! **What the boolean hands a spline pair's reading as known**: the rows
//! and corners two spline faces share, read off their edges.
//!
//! - **A shared row** is an edge of `F` whose image on its wall
//!   ([`crate::pcurves::pcurve_of`]) is an iso row, whose carrier also
//!   runs along a side of `G`'s chart, whole or in part: `G`'s image of it, stated
//!   on the same carrier parameter, certifies on `G`'s wall
//!   ([`crate::pcurves::side_image_of`]). Both images pass the one test a
//!   seam's shared row passes at the result
//!   ([`geom_brep::side_row_certifies`], which
//!   `IntersectionDraft::shared_row` reads each wall by), so they trace
//!   one curve, each on its own wall. The reading admits only a row on a side of each chart
//!   ([`geom_brep::ssi::PairRow`]).
//! - **A shared corner** is a vertex of `F` at a corner of its wall's
//!   rectangle (a joint of its loop's lift whose chart point is exactly
//!   two domain ends: where a row held at one meets a column held at
//!   another, [`crate::pcurves::Lift::joint_point`]) and one of `G` at a
//!   corner of its own, their gap not
//!   decided past the band (`spline_pair_shared_corner`). The reading of a corner holds whether or not the
//!   corner is a point of the section, so the pairing only chooses which
//!   corners are read.
//!
//! A row or corner not handed in is the boundary pass's to read, where
//! the section running along a side or through a corner refuses.

use geom::Surface;
use geom_brep::KnownPair;
use geom_core::k_stats::{Magnitude, decide_magnitude};
use geom_core::{Band, Decide, Margin};

use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey};
use crate::live::BoundaryMember;
use crate::props::AtRestPolicy;

/// **The rows and corners `F` of `a` and `G` of `b` share** (module
/// docs), in the order `[F, G]`. Empty unless both faces lie on spline
/// walls.
pub(crate) fn shared<T: Decide + AtRestPolicy>(
    (a, f, sf): (&Body<T>, FaceKey, &Surface<T>),
    (b, g, sg): (&Body<T>, FaceKey, &Surface<T>),
    band: Band,
) -> KnownPair<T> {
    let mut known = KnownPair::default();
    let (Some(na), Some(nb)) = (sf.spline_chart(), sg.spline_chart()) else {
        return known;
    };
    let rows_f = images(a, f, band);

    for (ha, ia) in &rows_f {
        let Ok((carrier, t0, t1)) = crate::pcurves::half_edge_carrier(a, *ha) else {
            continue;
        };
        // `F`'s image passes the shared row's test on its own wall too.
        if !geom_brep::side_row_certifies(sf, ia, (&carrier, (t0, t1)), band) {
            continue;
        }
        if let Some(ib) = crate::pcurves::side_image_of(&carrier, (t0, t1), sg, band) {
            known.rows.push(geom_brep::KnownRow {
                images: [ia.pcurve().clone(), ib],
                span: (t0, t1),
            });
        }
    }
    let (corners_f, corners_g) = (corners(a, f, na), corners(b, g, nb));
    for &(cf, pf) in &corners_f {
        for &(cg, pg) in &corners_g {
            // One point, as two vertices of one body meet (`vertices_meet`,
            // whose decision this is): their gap decided Zero. A gap in
            // the band is read too, since a corner's reading holds
            // whether or not the corner lies on the section.
            let apart = matches!(
                decide_magnitude("spline_pair_shared_corner", Margin::norm3(pf - pg), band),
                Ok(Magnitude::Positive)
            );
            if !apart {
                let pair = geom_brep::ssi::PairCorner([cf, cg]);
                if !known.corners.contains(&pair) {
                    known.corners.push(pair);
                }
            }
        }
    }
    known
}

/// Each half-edge of `face` whose image on its wall is an iso row, with
/// that image.
fn images<T: Decide + AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Vec<(HalfEdgeKey, geom_brep::ChartImage<T>)> {
    let view = body.proven_face(face);
    view.boundary()
        .filter_map(|member| match member {
            BoundaryMember::Edge { he, .. } => Some(he),
            BoundaryMember::Isolated { .. } => None,
        })
        .filter_map(|he| {
            let image = crate::pcurves::derived_pcurve_of(body, he, band).ok()?;
            matches!(
                image.pcurve(),
                geom_brep::Pcurve::IsoLine { .. } | geom_brep::Pcurve::IsoArc { .. }
            )
            .then_some((he, image))
        })
        .collect()
}

/// The corners of `net`'s rectangle a joint of `face`'s boundary stands
/// at exactly, each with the wall's point there: each end of each iso
/// image of each loop, as the loop's lift reads it
/// ([`crate::pcurves::Lift::joint_point`], [`crate::pcurves::Lift::exit_point`]).
/// Where a row meets a column that point is the meet of their held
/// values, so a corner is found wherever both lines are held at domain
/// ends, however the images' maps read their own ends; an end between
/// two images on one line is the image's value there, a corner only
/// where it is one exactly.
///
/// **A loop with no lift to read hands in no corner**
/// ([`crate::pcurves::Lift::read`] refuses: a member stores no image, or
/// no element on the joint into it). A finished body has none: tier 3
/// refuses a curved face that stores no rows (`PcurveMintError::Unminted`),
/// and the boolean takes finished bodies. Such a loop is a face a caller
/// reads mid-operation or off an unfinished body, and its corners are
/// left out, not derived: a corner handed in only widens what the pair's
/// reading may read, so leaving one out can refuse a section through it
/// (the boundary pass's `ssi_pair_boundary`, as
/// `sweep`'s `a_sheath_grafted_on_its_blade` shows with every corner left
/// out) and can accept nothing. The rows above are read from derived
/// images and do not need the lift.
fn corners<T: Decide + AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    net: &geom::NurbsSurface<T>,
) -> Vec<(geom_brep::ssi::ChartCorner, geom_core::Point3<T>)> {
    use geom_brep::ssi::ChartCorner;
    use geom_core::period::domain_end as end;
    let (du, dv) = (net.knot_values_u().domain(), net.knot_values_v().domain());
    let mut out: Vec<(ChartCorner, geom_core::Point3<T>)> = Vec::new();
    for (r#loop, _) in body.proven_face(face).cycles() {
        // No lift to read: no corner from this loop (the doc above).
        let Ok(lift) = crate::pcurves::Lift::read(body, r#loop) else {
            continue;
        };
        for (i, row) in lift.rows().iter().enumerate() {
            if row.held_line().is_none() {
                continue;
            }
            for p in [lift.joint_point(i), lift.exit_point(i)] {
                let (Some(u), Some(v)) = (end(p.x, du), end(p.y, dv)) else {
                    continue;
                };
                let corner = ChartCorner { u, v };
                if out.iter().all(|(c, _)| *c != corner) {
                    out.push((corner, net.eval(p.x, p.y)));
                }
            }
        }
    }
    out
}
