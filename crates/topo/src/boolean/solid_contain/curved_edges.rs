//! **The ray's reading of an edge a curved face bounds, or of a planar
//! face's curved edge** (D10, Booleans): every edge [`super::edge_meets`]
//! does not decide (a line between two crossed planes) is decided here
//! once, for both faces it bounds, where each of them reads its own
//! carrier's crossings on this ray (a planar face whose plane it
//! crosses, a trimmed curved face's roots).
//!
//! - **Where the ray passes the edge.** Each carrier crossing of either
//!   face is read against the edge's carrier first, and one definitely
//!   off it is clear of the edge there. From one that is not, the ray's
//!   point is carried to its closest approach to the carrier by alternate
//!   feet: the point's foot on the carrier ([`geom::Curve3::foot`], the
//!   one foot reading), then that foot's projection back onto the ray.
//!   Near the carrier the foot is well placed; far from it (a point on a
//!   circle's axis) it need not be, and a crossing that far off reads
//!   nothing about the edge. Two faces on one carrier (a seam split)
//!   cross it at one root, so their one point is read against the seam
//!   once.
//! - **Whether it meets it.** That ray point is read against the edge's
//!   span as the census reads it ([`crate::census::edge_span`]): on the
//!   carrier ([`geom::Curve3::foot`]'s `on_carrier`, zero in the band, off
//!   only past it on a lower bound), and where its foot sits against the
//!   span's ends ([`geom_brep::curve_pair::Span::place`]). On the carrier,
//!   or in the band's gap, inside the span, the ray meets the edge. In
//!   band of an end it is marked a graze; past the span its nearest point
//!   is that end, read as a vertex. Off the carrier from every crossing,
//!   it passes clear, and each face reads its own crossing.
//! - **Which crossing it stands for.** A met edge is half a crossing for
//!   each face, at that face's carrier crossing nearest where the ray
//!   passes the edge, decided nearest from both ends of the passage's
//!   span; a nearest in band of another reads nothing. Whether that
//!   crossing is one the edge represents is [`super::ray_crossings`]'
//!   question.
//!
//! An edge both of whose sides are one face (a seam inside a face) is no
//! boundary of it, and is decided as one. One met at two places along
//! the ray, or whose face offers no crossing to stand for it, reads
//! nothing there.
//!
//! **Clear is read at the crossings.** An edge every crossing of whose
//! faces reads clear of it is decided clear, and each face reads its own
//! crossing there. A ray can pass within the band of such an edge: from
//! a face met at another edge it is read again ([`narrow`]), and a met
//! edge stands only for a crossing near where the ray passes it, or one
//! its face holds that the ray reaches from there in band of the face's
//! carrier.

use super::{
    Cross, EdgeMeets, FaceGeo, Gap, Mark, Met, PlaneCross, PointInSolidError, RayHit, RayRows,
    RootCross, face_edges,
};
use crate::body::Body;
use crate::entity::FaceKey;
use crate::ray_walk::RayFault;
use crate::validate::decide;
use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

/// The alternate feet that carry a crossing's point to the ray's closest
/// approach to an edge's carrier. Each foot is never farther than the one
/// before it, so the count is work, not soundness: a point short of the
/// closest approach reads an upper bound on the ray's distance.
const FEET: usize = 8;

/// What the ray reads against an edge from one crossing's point.
enum Passes<T> {
    /// It meets the edge at advance `s`.
    Met(T, Gap),
    /// It passes within the band of an end of the edge at advance `s`.
    End(T),
    /// It passes clear of the edge there.
    Clear,
    /// No reading here decides it.
    Undecided,
}

/// **Decides every edge between two faces that read their own crossings
/// on this ray** and that `meets` has not decided (module docs), into
/// `meets`: its met edges, its marks, its vertices, and its decided
/// edges.
///
/// # Errors
///
/// A broken face, or a vertex that does not resolve.
#[allow(clippy::too_many_arguments)] // the selection, its crossings, the ray and the rows
pub(super) fn read<T: Decide>(
    body: &Body<T>,
    geos: &[(FaceKey, FaceGeo<T>)],
    planes: &[PlaneCross<T>],
    roots: &[RootCross<T>],
    missed: &[FaceKey],
    (q, d, reach): (Point3<T>, Vec3<T>, T),
    band: Band,
    rows: RayRows,
    meets: &mut EdgeMeets<T>,
) -> Result<(), RayFault<PointInSolidError>> {
    let far = q + d * (reach * T::from_f64(2.0));
    // A face's own carrier crossings on this ray, where it reads them:
    // `None` for a face read whole or not at all (a closed sphere or torus
    // group, an untrimmable face, a plane the ray runs beside).
    let crossings = |f: FaceKey| -> Option<Vec<(Cross, T)>> {
        if let Some(c) = planes.iter().find(|c| c.face == f) {
            return Some(vec![(Cross::Plane, c.t)]);
        }
        match geos.iter().find(|g| g.0 == f).map(|g| &g.1) {
            Some(
                FaceGeo::Cylinder { .. }
                | FaceGeo::Cone { .. }
                | FaceGeo::SpherePatch { .. }
                | FaceGeo::TorusPatch { .. },
            ) => Some(
                roots
                    .iter()
                    .enumerate()
                    .filter(|(_, r)| r.face == f)
                    .map(|(k, r)| (Cross::Root(k), r.t))
                    .collect(),
            ),
            _ => None,
        }
    };
    let mut seen = std::collections::BTreeSet::new();
    for &(face, _) in geos {
        for fe in face_edges(body, face).map_err(RayFault::Fatal)? {
            let (e, other) = (fe.edge, fe.across);
            if meets.lines.contains(&e) || !seen.insert(e) {
                continue;
            }
            // A seam inside the face bounds nothing: no reading of it is
            // owed.
            if other == face {
                meets.read.insert(e);
                continue;
            }
            let (Some(ca), Some(cb)) = (crossings(face), crossings(other)) else {
                continue;
            };
            // A carrier the ray definitely misses holds no point of the
            // edge within the band of the ray.
            if missed.contains(&face) || missed.contains(&other) {
                meets.read.insert(e);
                continue;
            }
            if ca.is_empty() || cb.is_empty() {
                continue;
            }
            let Some(span) = crate::census::edge_span(body, e, band) else {
                continue;
            };
            let faces = [face, other];
            for v in fe.ends {
                if !crate::boolean::is_conventional_vertex(body, v) {
                    meets.vertex(body, v, faces, (q, far), d, band, rows)?;
                }
            }
            let mut found: Vec<(T, Gap)> = Vec::new();
            let mut decided = true;
            for &(_, t) in ca.iter().chain(&cb) {
                match passes(&span, q, d, q + d * t, band) {
                    Passes::Met(s, gap) => found.push((s, gap)),
                    Passes::End(s) => meets.marks.push(RayHit::marked(faces, s, Mark::Graze)),
                    Passes::Clear => {}
                    Passes::Undecided => decided = false,
                }
            }
            if decided {
                meets.read.insert(e);
            }
            let Some(&(first, _)) = found.first() else {
                continue;
            };
            let (lo, hi) = found
                .iter()
                .fold((first, first), |(lo, hi), &(s, _)| (lo.min(s), hi.max(s)));
            // Met at two places along the ray, definitely apart: no one
            // half crossing stands for both.
            if geom_core::k_stats::decide_magnitude(rows.order, Margin::of(hi - lo), band)
                == Ok(geom_core::k_stats::Magnitude::Positive)
            {
                for s in [lo, hi] {
                    meets.marks.push(RayHit::marked(faces, s, Mark::Graze));
                }
                continue;
            }
            let at = geom::mid_param(lo, hi);
            // Where the ray comes closest to the edge behind `q`, it is
            // nearest the edge at `q`, which the boundary pre-pass holds
            // clear of both its faces; a passage whose span straddles `q`
            // is read nowhere.
            if decide(rows.advance, Margin::of(hi), band) == Ok(Sign::Negative) {
                continue;
            }
            if decide(rows.advance, Margin::of(lo), band) == Ok(Sign::Negative) {
                meets.marks.push(RayHit::marked(faces, at, Mark::Graze));
                continue;
            }
            let gap = found
                .iter()
                .map(|f| f.1)
                .find(|g| matches!(g, Gap::Zero))
                .unwrap_or(found[0].1);
            let (Some(xa), Some(xb)) = (
                nearest(&ca, (lo, hi), band, rows),
                nearest(&cb, (lo, hi), band, rows),
            ) else {
                meets.marks.push(RayHit::marked(faces, at, Mark::Graze));
                continue;
            };
            meets.meet(Met {
                edge: e,
                faces,
                at,
                span: (lo, hi),
                cross: [xa, xb],
                gap,
            });
        }
    }
    narrow(body, (q, d), band, meets)
}

/// **A face met at one edge, read again at its other edges from where
/// the ray passes that edge** (both ends of the passage's span): an edge
/// the line pass did not read of either face, which the ray passes within
/// the band of from there, is a second edge the face is met at, a wedge,
/// and both are marked (as [`EdgeMeets::meet`] marks one). The edges' own
/// reading starts at their faces' crossings, which a ray shallow to a
/// face can leave definitely off an edge it passes within the band of;
/// from a point where the ray passes within the band of the face, an
/// edge of a face narrower than the band there is near.
fn narrow<T: Decide>(
    body: &Body<T>,
    (q, d): (Point3<T>, Vec3<T>),
    band: Band,
    meets: &mut EdgeMeets<T>,
) -> Result<(), RayFault<PointInSolidError>> {
    let met = meets.met.clone();
    for m in &met {
        for &face in &m.faces {
            for fe in face_edges(body, face).map_err(RayFault::Fatal)? {
                let e = fe.edge;
                // A line edge between two crossed planes is read exactly
                // ([`super::edge_meets`]), and one met is marked where it
                // was met.
                if fe.across == face || meets.lines.contains(&e) || met.iter().any(|w| w.edge == e)
                {
                    continue;
                }
                let Some(span) = crate::census::edge_span(body, e, band) else {
                    continue;
                };
                let faces = [face, fe.across];
                for seed in [m.span.0, m.span.1] {
                    match passes(&span, q, d, q + d * seed, band) {
                        Passes::Met(s, gap) => {
                            let mark = gap.mark(m.gap);
                            meets.marks.push(RayHit::marked(faces, s, mark));
                            meets.marks.push(RayHit::marked(m.faces, m.at, mark));
                        }
                        Passes::End(s) => {
                            meets.marks.push(RayHit::marked(faces, s, Mark::Graze));
                        }
                        Passes::Undecided => {
                            meets.marks.push(RayHit::marked(m.faces, m.at, Mark::Graze));
                        }
                        Passes::Clear => {}
                    }
                }
            }
        }
    }
    Ok(())
}

/// **What the ray `q + s·d` reads against `span` from `seed`**, a point
/// of it (module docs). A point definitely off the edge's carrier (on the
/// carrier's lower bound, where it has a closed form) is clear of it
/// there. One that is not lies near the carrier, where its foot is well
/// placed, and is carried to the ray's closest approach by alternate
/// feet, which is read on the carrier and against the span's ends.
fn passes<T: Decide>(
    span: &crate::census::EdgeSpan<T>,
    q: Point3<T>,
    d: Vec3<T>,
    seed: Point3<T>,
    band: Band,
) -> Passes<T> {
    let carrier = span.carrier();
    let mut near = span.middle();
    // The carrier's closed-form lower bound on the point's distance from
    // it, as its on-carrier reading decides off (a spline's is that
    // reading's own walk): a point far off the carrier asks no foot.
    let clear = match carrier.clearance(seed) {
        Some(lower) => decide("curve_carrier_clear", Margin::of(lower), band) == Ok(Sign::Positive),
        None => matches!(
            carrier.foot(seed, near).on_carrier(band),
            Ok(geom::OnCarrier::Off)
        ),
    };
    if clear {
        return Passes::Clear;
    }
    let mut r = seed;
    for _ in 0..FEET {
        near = carrier.foot(r, near).t();
        r = q + d * (carrier.eval(near) - q).dot(d);
    }
    let s = (r - q).dot(d);
    let gap = match carrier.foot(r, near).on_carrier(band) {
        Ok(geom::OnCarrier::Off) => return Passes::Clear,
        Ok(geom::OnCarrier::On { .. }) => Gap::Zero,
        Err(geom::CarrierRefusal::Escalated(diag)) => Gap::InBand(diag),
        Err(_) => return Passes::Undecided,
    };
    match span.span().place(r, near, band) {
        Ok(geom_brep::curve_pair::Place::Inside { .. }) => Passes::Met(s, gap),
        Ok(geom_brep::curve_pair::Place::Past) => Passes::Clear,
        // In band of an end, or undecided against one: the ray passes
        // within reach of its vertex.
        Ok(geom_brep::curve_pair::Place::End(_)) | Err(_) => Passes::End(s),
    }
}

/// The crossing of `cs` nearest the passage spanning `(lo, hi)` along
/// the ray: the only one, or the one decided nearer than every other
/// (`order`, a magnitude) from both ends of the span; `None` where no
/// one is.
fn nearest<T: Decide>(
    cs: &[(Cross, T)],
    (lo, hi): (T, T),
    band: Band,
    rows: RayRows,
) -> Option<Cross> {
    cs.iter()
        .enumerate()
        .find(|&(i, &(_, t))| {
            cs.iter().enumerate().all(|(j, &(_, u))| {
                j == i
                    || [lo, hi].iter().all(|&at| {
                        decide(
                            rows.order,
                            Margin::of((u - at).abs() - (t - at).abs()),
                            band,
                        ) == Ok(Sign::Positive)
                    })
            })
        })
        .map(|(_, &(c, _))| c)
}
