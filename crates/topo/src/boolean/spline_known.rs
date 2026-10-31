//! **What the boolean hands a spline pair's reading as known**: the rows
//! and corners two spline faces share, read off their edges.
//!
//! - **A shared row** is an edge of `F` whose image on its wall
//!   ([`crate::pcurves::pcurve_of`]) is an iso row, whose carrier also
//!   runs along a whole side of `G`'s chart: `G`'s image of it, stated
//!   on the same carrier parameter, certifies on `G`'s wall
//!   ([`crate::pcurves::side_image_of`]). The two images then trace one
//!   curve, each on its own wall, as a seam's shared row does at the
//!   result. The reading admits only a row on a side of each chart
//!   ([`geom_brep::ssi::PairRow`]).
//! - **A shared corner** is a vertex of `F` at a corner of its wall's
//!   rectangle and one of `G` at a corner of its own, their distance not
//!   decided past the band. The reading of a corner holds whether or not the
//!   corner is a point of the section, so the pairing only chooses which
//!   corners are read.
//!
//! A row or corner not handed in is the boundary pass's to read, where
//! the section running along a side or through a corner refuses.

use geom::Surface;
use geom_core::{Band, Decide, Point2, Sign};

use super::section_cert::Known;
use crate::body::Body;
use crate::entity::{EntityId, FaceKey, HalfEdgeKey};
use crate::live::{BoundaryMember, proven};
use crate::props::AtRestPolicy;

/// **The rows and corners `F` of `a` and `G` of `b` share** (module
/// docs), in the order `[F, G]`. Empty unless both faces lie on spline
/// walls.
pub(crate) fn shared<T: Decide + AtRestPolicy>(
    (a, f, sf): (&Body<T>, FaceKey, &Surface<T>),
    (b, g, sg): (&Body<T>, FaceKey, &Surface<T>),
    band: Band,
) -> Known<T> {
    let mut known = Known::default();
    let (Some(na), Some(nb)) = (sf.spline_chart(), sg.spline_chart()) else {
        return known;
    };
    let (rows_f, rows_g) = (images(a, f, band), images(b, g, band));

    for (ha, ia) in &rows_f {
        let Ok((carrier, t0, t1)) = crate::pcurves::half_edge_carrier(a, *ha) else {
            continue;
        };
        // `G` holds an edge along a side of its own chart, and `F`'s
        // carrier runs along a side of `G`'s chart too.
        if rows_g.is_empty() {
            continue;
        }
        if let Some(ib) = crate::pcurves::side_image_of(&carrier, (t0, t1), sg, band) {
            known.rows.push(geom_brep::KnownRow {
                images: [ia.clone(), ib],
                span: (t0, t1),
            });
        }
    }
    let (corners_f, corners_g) = (corners(&rows_f, na, a), corners(&rows_g, nb, b));
    for &(cf, pf) in &corners_f {
        for &(cg, pg) in &corners_g {
            // Within the band of each other: the gap not decided past it.
            let apart = matches!(
                (pf - pg).norm().sign_within(band),
                Ok(d) if d.sign == Sign::Positive
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
) -> Vec<(HalfEdgeKey, geom_brep::Pcurve<T>)> {
    let fd = proven(&body.faces, face, EntityId::Face);
    body.face_boundary_linked(face, fd)
        .filter_map(|member| match member {
            BoundaryMember::Edge { he, .. } => Some(he),
            BoundaryMember::Isolated { .. } => None,
        })
        .filter_map(|he| {
            let image = crate::pcurves::pcurve_of(body, he, band).ok()?;
            matches!(
                image,
                geom_brep::Pcurve::IsoLine { .. } | geom_brep::Pcurve::IsoArc { .. }
            )
            .then_some((he, image))
        })
        .collect()
}

/// The corners of `net`'s rectangle an end of one of `rows` stands at
/// exactly, each with the wall's point there.
fn corners<T: Decide + AtRestPolicy>(
    rows: &[(HalfEdgeKey, geom_brep::Pcurve<T>)],
    net: &geom::NurbsSurface<T>,
    body: &Body<T>,
) -> Vec<(geom_brep::ssi::ChartCorner, geom_core::Point3<T>)> {
    use geom_brep::ssi::{ChartCorner, ChartEnd};
    let (du, dv) = (net.knot_values_u().domain(), net.knot_values_v().domain());
    let end = |x: T, (lo, hi): (T, T)| {
        if geom_core::is_exact_zero(x - lo) {
            Some(ChartEnd::Low)
        } else if geom_core::is_exact_zero(x - hi) {
            Some(ChartEnd::High)
        } else {
            None
        }
    };
    let mut out: Vec<(ChartCorner, geom_core::Point3<T>)> = Vec::new();
    for (he, image) in rows {
        let Ok((_, t0, t1)) = crate::pcurves::half_edge_carrier(body, *he) else {
            continue;
        };
        for t in [t0, t1] {
            let p: Point2<T> = image.eval(t);
            let (Some(u), Some(v)) = (end(p.x, du), end(p.y, dv)) else {
                continue;
            };
            let corner = ChartCorner { u, v };
            if out.iter().all(|(c, _)| *c != corner) {
                out.push((corner, net.eval(p.x, p.y)));
            }
        }
    }
    out
}
