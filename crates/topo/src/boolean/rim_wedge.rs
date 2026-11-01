//! **The shared-rim routing** — the ruling's material-wedge table read
//! at a cross-operand contact, before a body exists to validate.
//!
//! Two faces of two operands meeting along one circle are two
//! physically different situations wearing one description, and which
//! one a rim is decides what the boolean owes it:
//!
//! - **wedge π** — the outward normals agree across the rim and the
//!   two faces leave it on opposite sides ([`departures`]): the
//!   composed material is smooth through it, and the rim is a SEAM of
//!   one composite wall. Between two operands the boolean glues the
//!   faces' tangency (C4) and the seam is read on the result; inside
//!   one body it is structural.
//! - **normals agree, same departure** — the two faces leave the rim on
//!   ONE side (a bowl hanging in a tube's mouth): a nested touch, a
//!   cusp the normals alone cannot tell from the seam. The material
//!   table answers [`RimRouting::Seam`] for it, since it reads only the
//!   two sheets' normals and jets; [`departures`] is what tells the two
//!   apart, and the glue door (`route_rim`) asks it after the table.
//! - **wedge 0 or 2π** — the normals oppose across the rim. A pair
//!   reaching this routing has no tangent-locus arm for its two
//!   surface kinds, which is what its refusal names.
//! - **anything the samples cannot settle** — escalates, naming the
//!   predicate that failed to decide. Never a silent verdict.
//!
//! **This is the tier-3 verdict table's own machinery, consumed one
//! stage earlier.** `validate`'s check-4 material arm asks the same
//! question of an EDGE of a finished body; the answer there is a fold
//! over per-sample `classify_material_pairing` and `tangent_jet`
//! readings, and both are functions of two SURFACES, two face senses,
//! a point and a direction — no topology at all. So the classification
//! ports to a cross-operand rim by supplying the rim's stations in place
//! of the edge's, and the per-station reads, the walk and the fold are
//! all imported rather than restated: one table, two callers, and a new
//! wedge row cannot reach one and miss the other.
//!
//! What does NOT port is the rim's own identification. A body's edge
//! IS its two faces' shared locus; two operands share nothing, so the
//! rim is found geometrically — a boundary edge of each face riding
//! the same circle carrier, decided on the carrier's own data.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Vec3};

use crate::validate::{MaterialArmOutcome, MaterialStations};
use crate::{Body, FaceKey};
use geom_brep::MaterialWedge;

/// A rim circle, in the data the curve carries: centre, unit axis,
/// radius and the seam reference the sampling phase starts from.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Rim<T: Real> {
    /// The circle's centre.
    pub center: Point3<T>,
    /// The unit axis.
    pub axis: Vec3<T>,
    /// The radius.
    pub radius: T,
    /// The unit reference direction where the parameter is zero.
    pub u_ref: Vec3<T>,
}

/// What the ruling's table routes a shared rim to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RimRouting {
    /// Outward normals aligned along the rim: wedge π if the faces
    /// leave it on opposite sides (the smooth seam, the built arm), a
    /// nested touch if on one side. [`departures`] decides which.
    Seam,
    /// Wedge 0 or 2π: outward normals opposed along the rim
    /// ([`super::BooleanError::RimCuspArmUnbuilt`]).
    Cusp(MaterialWedge),
    /// Opposed sides whose jets osculate — conformal contact along the
    /// locus, which is a `Rest` claim wearing a rim's clothes.
    Lamina,
    /// Tangent planes definitely DISTINCT at every station: a genuine
    /// corner, which is the ordinary crossing lanes' business and not a
    /// rim contact at all. A first-order verdict, decided by the screen
    /// and never by the material arm.
    Transverse,
}

/// The circle two faces' boundaries share — the rim the routing is
/// taken along, carried with the seam reference the curve itself
/// stores so no perpendicular has to be invented for it.
///
/// Decided on the carrier's own data at the run's band, which is the
/// same three data `carrier_eq` compares for a cylinder and for the
/// same reason: a circle is its centre, its axis LINE and its radius,
/// and the axis's direction sign carries no information about the
/// point set. Every candidate pair is examined and the FIRST match
/// wins; a face pair riding two common circles at once is not a rim
/// contact and is outside what this door claims to see.
///
/// **Why the comparison is written here rather than consumed from
/// `carrier_eq`, stated so the next reader does not have to re-derive
/// it.** The resemblance is real and it is now exact in POSTURE as well
/// as in data: both compare a centre, an axis line and a radius; both
/// meter the angular datum at a consumption extent rather than at unit
/// arm; both treat an in-band datum as an escalation rather than as a
/// separation. What differs is the subject. `carrier_eq` is a ladder
/// over SURFACE carriers — its inventory is plane, sphere, cylinder,
/// torus, its verdict is a material-side relation (`SameOriented` /
/// `SameOpposite` / `Distinct`), and its rungs consult declared
/// intent. A rim is a CURVE, it has no material side, and
/// no declaration is being verified here: the question is only "are
/// these two boundary edges the same circle". Consuming the ladder
/// would mean giving it a curve-carrier variant with no orientation and
/// no identity rungs — a fifth `CarrierDesc` arm that answers a
/// different kind of question — which is a design move on the C4 table,
/// not a de-duplication. Until someone wants that variant for its own
/// sake, one small comparison stated plainly beats a ladder bent to
/// cover two subjects.
///
/// **The angular margin is metered at the candidate's own extent**, not
/// at unit arm. A dimensionless sine says nothing until it is priced as
/// the displacement it induces over the reach the verdict is consumed
/// across, and that reach is the rim's diameter. That diameter is
/// derived here, not threaded in, because it cannot be: it is a
/// property of the rim, and the caller has no rim until this function
/// returns one. (The wedge screen and the material arm after it read a
/// different quantity, the faces' wedge, which the caller levers at the
/// faces' reach, `crate::wedge_reach`, as every reader of it does.) `ra.radius +
/// rb.radius` is that diameter for any pair whose radii agree, and a
/// pair whose radii disagree is about to be rejected on the radius
/// margin regardless.
///
/// **An in-band comparison ESCALATES where no pair of circles is
/// decided one; it never reads as "not this pair".** Two circles a hair
/// apart are the sliver case, not a separation: answering `None` there
/// would make an undecidable rim indistinguishable from a definitely
/// absent one and hand the caller the bare class refusal as though the
/// geometry had been examined and cleared. That is the silence this module's own docs forbid.
///
/// # Errors
///
/// [`Indeterminate`] naming the rim-identity predicate that landed in
/// the band.
pub(crate) fn shared_rim<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<Option<(Rim<T>, geom_core::MarginDiag)>, Indeterminate> {
    // A pair decided one answers whatever another pair read; an
    // undecided pair refuses only where none is.
    let mut undecided = None;
    let mut undecided_logs = Vec::new();
    for ra in face_boundary_circles(a, fa) {
        for rb in face_boundary_circles(b, fb) {
            let (read, log) = geom_core::k_stats::detached(|| same_circle(ra, rb, band));
            match read {
                Ok(Some(margin)) => {
                    undecided_logs
                        .into_iter()
                        .for_each(geom_core::k_stats::splice_superseded);
                    geom_core::k_stats::splice(log);
                    return Ok(Some((ra, margin)));
                }
                Ok(None) => geom_core::k_stats::splice(log),
                Err(diag) => {
                    undecided_logs.push(log);
                    undecided.get_or_insert(diag);
                }
            }
        }
    }
    undecided_logs
        .into_iter()
        .for_each(geom_core::k_stats::splice);
    undecided.map_or(Ok(None), Err)
}

/// Whether two circles are one, on their own data at `band`, by the
/// identity ladder's rule ([`super::carrier_eq::identity_data`]): any
/// datum decided apart answers no, so an in-band radius never hides a
/// centre decided elsewhere. Radius and centre first, angle last — the
/// two LENGTH data separate almost every non-rim pair definitely and
/// cheaply. [`shared_rim`]'s docs carry the rest. The circles' one
/// margin, where every datum decides Zero: the radius row's, standing
/// jointly for the others.
fn same_circle<T: Decide>(
    ra: Rim<T>,
    rb: Rim<T>,
    band: Band,
) -> Result<Option<geom_core::MarginDiag>, Indeterminate> {
    use super::carrier_eq::IdentityData;
    match super::carrier_eq::identity_data(
        [
            ("rim_circle_radius", Margin::of(ra.radius - rb.radius)),
            ("rim_circle_center", Margin::norm3(ra.center - rb.center)),
            (
                "rim_circle_axis_parallel",
                Margin::levered(ra.axis.cross(rb.axis).norm(), ra.radius + rb.radius),
            ),
        ],
        band,
    ) {
        IdentityData::Distinct => Ok(None),
        IdentityData::One(margin) => Ok(margin),
        IdentityData::Undecided(measure) => Err(measure.reported()),
    }
}

/// The curve two declared faces are tangent along: a shared rim circle,
/// or the DEV-1 closed-form line (`origin + t·dir`, `dir` unit).
#[derive(Clone, Copy, Debug)]
pub(crate) enum Locus<T: Real> {
    /// A rim circle both faces' boundaries ride.
    Rim(Rim<T>),
    /// A tangent line.
    Line {
        /// A point on the line.
        origin: Point3<T>,
        /// The unit direction.
        dir: Vec3<T>,
    },
}

/// Which way two faces leave a locus they are tangent along, over the
/// stretch where they touch ([`departures`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Departure {
    /// Wherever they touch, both faces end at the locus and leave it on
    /// opposite sides: the π wedge of a seam.
    Opposite,
    /// Wherever they touch, they leave it on the same side: a cusp.
    Same,
    /// Opposite sides along part of the touch and the same side along
    /// another, or a face whose boundary runs one stretch of the locus
    /// BOTH ways (the two half-edges of one edge, a chart seam or a
    /// bridge): no one claim fits the pair.
    Mixed,
    /// Somewhere the two faces touch along the locus, one of them has no
    /// boundary edge there: the locus runs through its interior, so the
    /// face does not END at the locus and leaves it on no one side.
    NoEdge,
    /// The two faces share no stretch of the locus, and no point of it
    /// where both END (an abutting pair is read there): the declaration
    /// meets no curve of contact.
    Untouched,
    /// A boundary edge of one face is a curve the read does not place
    /// against the locus (an ellipse, a spline), or a touch piece its
    /// boundary pass does not place (its midpoint on the face's boundary,
    /// or at an arc's end its corners disagree with): the touch is not
    /// read, and the claim is refused as outside the envelope rather
    /// than answered.
    EdgeKindUnread,
}

/// **Which way two faces leave a locus they are tangent along, wherever
/// they TOUCH**, read from the boundary half-edges that run along it:
/// the half of the wedge a pair of operands can get wrong and a body's
/// edge cannot.
///
/// A face's interior lies to the LEFT of each of its half-edges about
/// its outward normal (outer loops counterclockwise, rings clockwise:
/// `entity`'s convention), so at a locus point the direction a face
/// leaves along is `n × t`, with `t` its boundary's direction there.
/// For two faces whose outward normals are ALIGNED along the locus,
/// which the caller has verified (the rim routing, [`super::route_rim`];
/// a line is read only by the unit rows here, since a line tangency
/// routes no junction), those departures
/// are opposite exactly when the two boundaries run the locus in
/// opposite directions: the sign of each riding half-edge's direction
/// against the locus's own tangent (`axis × (p − centre)` on a rim,
/// `dir` on a line).
///
/// **The read is tied to where the faces touch.** A face's edge on the
/// locus names the face's side only where the face ENDS there; an edge
/// elsewhere on the same line or circle says nothing about the stretch
/// where the two faces meet. So the locus is cut, along its own
/// parameter (the line parameter, or the rim angle), at every point
/// where either face's membership can change: the ends of each face's
/// riding edges, each face's vertices on the locus, and each point
/// where one of its other boundary edges crosses the locus. Between two
/// consecutive cuts each face either contains the locus or does not,
/// and either rides it or does not; the piece is read at its midpoint
/// (a riding edge's span, else the face's own containment door, whose
/// boundary reading there places nothing: [`Departure::EdgeKindUnread`]
/// where the other face may hold the piece). The TOUCH is the pieces
/// both faces contain. On each touch piece both faces must ride the
/// locus ([`Departure::NoEdge`] otherwise), and their traversals there
/// give the piece's departure.
///
/// **A pair meeting at a point only** is read at that point when both
/// faces END there, their riding edges abutting: a declaration over
/// face sets pairs a half-wall with the half-cap across the rim, and
/// the boolean reads that pair's cover where the four faces meet at the
/// rim's vertex, so the pair is declared and must verify. Beside the
/// point each face ends at the locus, and their traversals compare the
/// banks there. A pair that shares no stretch and no such point is
/// [`Departure::Untouched`]: the declaration meets no contact.
///
/// Inside one body the question never arises: an edge's two half-edges
/// run it in opposite directions by construction.
///
/// # Errors
///
/// [`Indeterminate`]: the deciding predicate when a riding, crossing,
/// containment or traversal margin lands in the band, or
/// `seam_cover_unread` (a label) when a face's containment door has no
/// verdict for a curved face's chart.
pub(crate) fn departures<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    locus: Locus<T>,
    band: Band,
) -> Result<Departure, Indeterminate> {
    let (Some(ra), Some(rb)) = (reach(a, fa, locus, band)?, reach(b, fb, locus, band)?) else {
        return Ok(Departure::EdgeKindUnread);
    };
    let tau = T::from_f64(core::f64::consts::TAU);
    let mut cuts: Vec<T> = ra.cuts.iter().chain(&rb.cuts).copied().collect();
    if let Locus::Rim(_) = locus
        && cuts.is_empty()
    {
        cuts.push(T::zero());
    }
    let scale = match locus {
        Locus::Rim(rim) => rim.radius,
        Locus::Line { .. } => T::one(),
    };
    // Ascending, by decided differences: the scalars carry no order.
    sort_params(&mut cuts, scale, band)?;
    let pieces: Vec<(T, T)> = match locus {
        Locus::Line { .. } => cuts.windows(2).map(|w| (w[0], w[1])).collect(),
        Locus::Rim(_) => {
            let mut out: Vec<(T, T)> = cuts.windows(2).map(|w| (w[0], w[1])).collect();
            if let (Some(&first), Some(&last)) = (cuts.first(), cuts.last()) {
                out.push((last, first + tau));
            }
            out
        }
    };
    let (mut opposite, mut same, mut touched) = (false, false, false);
    for (s0, s1) in pieces {
        // A piece of no length between two cuts is a point, and so is
        // one within the band of none: two cuts that close are one
        // place on the locus, and no stretch a seam claims lives
        // between them.
        if !matches!(
            crate::validate::decide("seam_cover_piece", Margin::of((s1 - s0) * scale), band),
            Ok(geom_core::Sign::Positive)
        ) {
            continue;
        }
        let mid = (s0 + s1) * T::from_f64(0.5);
        let ca = ra.at(a, fa, locus, mid, scale, band)?;
        let cb = rb.at(b, fb, locus, mid, scale, band)?;
        if !(ca.contains() && cb.contains()) {
            continue;
        }
        touched = true;
        match (ca, cb) {
            (Cover::Unread, _) | (_, Cover::Unread) => return Ok(Departure::EdgeKindUnread),
            (Cover::RidesBothWays, _) | (_, Cover::RidesBothWays) => {
                return Ok(Departure::Mixed);
            }
            (Cover::Rides(x), Cover::Rides(y)) if x == y => same = true,
            (Cover::Rides(_), Cover::Rides(_)) => opposite = true,
            _ => return Ok(Departure::NoEdge),
        }
    }
    if !touched {
        // No stretch in common: the faces may still meet at a POINT of
        // the locus where both END, their riding edges abutting there
        // (a half-wall and the half-cap across the rim's vertex). Each
        // face ends at the locus right beside that point, so their
        // traversals there compare the two banks locally.
        for &(alo, ahi, sa) in &ra.rides {
            for &(blo, bhi, sb) in &rb.rides {
                let mut abut = false;
                for (x, y) in [(ahi, blo), (alo, bhi)] {
                    let gap = (locus_point(locus, x) - locus_point(locus, y)).norm();
                    abut |= matches!(
                        crate::validate::decide("seam_cover_abut", Margin::of(gap), band)?,
                        geom_core::Sign::Zero
                    );
                }
                if abut {
                    touched = true;
                    if sa == sb {
                        same = true;
                    } else {
                        opposite = true;
                    }
                }
            }
        }
    }
    Ok(match (touched, opposite, same) {
        (false, _, _) => Departure::Untouched,
        (true, true, true) => Departure::Mixed,
        (true, true, false) => Departure::Opposite,
        (true, false, _) => Departure::Same,
    })
}

/// Sort locus parameters ascending by decided differences (metered in
/// meters: `scale` is the rim's radius, one on a line). Two that decide
/// equal keep their order; the piece between them has no length and
/// the caller skips it.
///
/// # Errors
///
/// [`Indeterminate`] where a difference lands in the band.
fn sort_params<T: Decide>(params: &mut Vec<T>, scale: T, band: Band) -> Result<(), Indeterminate> {
    let mut out: Vec<T> = Vec::with_capacity(params.len());
    for &p in params.iter() {
        let mut at = out.len();
        for (i, &q) in out.iter().enumerate() {
            if crate::validate::decide("seam_cover_order", Margin::of((p - q) * scale), band)?
                == geom_core::Sign::Negative
            {
                at = i;
                break;
            }
        }
        out.insert(at, p);
    }
    *params = out;
    Ok(())
}

/// What one face does at a piece of the locus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cover {
    /// A riding edge covers the piece, run with this sign.
    Rides(geom_core::Sign),
    /// Riding edges cover it run BOTH ways.
    RidesBothWays,
    /// No riding edge covers it; the face contains the piece or not.
    Contains(bool),
    /// No riding edge covers it, and the face's read does not place the
    /// piece (its midpoint on the face's boundary, or at an arc's end
    /// its corners disagree with).
    Unread,
}

impl Cover {
    /// Whether the face may hold the piece: every reading but a
    /// definite `Out`.
    fn contains(self) -> bool {
        !matches!(self, Self::Contains(false))
    }
}

/// Where one face meets a locus, in the locus's own parameter.
struct Reach<T: Real> {
    /// Each riding half-edge's span (`lo ≤ hi`; on a rim `lo` in
    /// `[0, 2π)`) and its traversal sign.
    rides: Vec<(T, T, geom_core::Sign)>,
    /// Every parameter where the face's membership can change.
    cuts: Vec<T>,
}

impl<T: Decide> Reach<T> {
    /// At locus parameter `s` (a piece's midpoint, never a cut): the
    /// riding edges covering it, every one of them, else whether the
    /// face contains the locus point there. Spans compare in meters
    /// (`scale`, as [`sort_params`]).
    fn at(
        &self,
        body: &Body<T>,
        face: FaceKey,
        locus: Locus<T>,
        s: T,
        scale: T,
        band: Band,
    ) -> Result<Cover, Indeterminate> {
        let tau = T::from_f64(core::f64::consts::TAU);
        let wraps: &[f64] = match locus {
            Locus::Rim(_) => &[-1.0, 0.0, 1.0],
            Locus::Line { .. } => &[0.0],
        };
        let inside = |m: T| -> Result<bool, Indeterminate> {
            Ok(
                crate::validate::decide("seam_cover_ride", Margin::of(m * scale), band)?
                    == geom_core::Sign::Positive,
            )
        };
        let mut seen: Option<geom_core::Sign> = None;
        for &(lo, hi, sign) in &self.rides {
            for &k in wraps {
                let s = s + tau * T::from_f64(k);
                if inside(s - lo)? && inside(hi - s)? {
                    if seen.is_some_and(|x| x != sign) {
                        return Ok(Cover::RidesBothWays);
                    }
                    seen = Some(sign);
                }
            }
        }
        Ok(match seen {
            Some(sign) => Cover::Rides(sign),
            None => match contains(body, face, locus_point(locus, s), band)? {
                Some(inside) => Cover::Contains(inside),
                None => Cover::Unread,
            },
        })
    }
}

/// The locus point at parameter `s`.
fn locus_point<T: Decide>(locus: Locus<T>, s: T) -> Point3<T> {
    match locus {
        Locus::Line { origin, dir } => origin + dir * s,
        Locus::Rim(rim) => {
            let (sin, cos) = s.sin_cos();
            rim.center + (rim.u_ref * cos + rim.axis.cross(rim.u_ref) * sin) * rim.radius
        }
    }
}

/// The locus parameter of a point on (or decided at) the locus. On a
/// rim it is reduced into `[0, τ)` up to rounding, so a point at the seam
/// reads `0` or, where its angle comes back a hair negative, a hair
/// under `τ`. The readers are invariant under which: the cuts partition
/// the rim cyclically (the last piece wraps to the first cut a turn on,
/// and a piece of no length is dropped), and a ride is looked up one
/// turn either side of the piece's midpoint. A riding arc's whole turn is
/// its own parameter span `t1 − t0`, carried as the ride's length.
fn locus_param<T: Decide>(locus: Locus<T>, p: Point3<T>) -> T {
    match locus {
        Locus::Line { origin, dir } => (p - origin).dot(dir),
        Locus::Rim(rim) => {
            let d = p - rim.center;
            let th = d.dot(rim.axis.cross(rim.u_ref)).atan2(d.dot(rim.u_ref));
            let tau = T::from_f64(core::f64::consts::TAU);
            th - tau * (th / tau).floor()
        }
    }
}

/// Whether the face holds `p`, a piece's midpoint on its carrier: `In`
/// is `true`, `Out` is `false`, and a point the read does not place is
/// `None`.
///
/// **A midpoint on the face's boundary is not placed.** The cut walk
/// puts every point where the face's boundary meets the locus at a cut,
/// so between two cuts the face holds the whole piece or none of it, and
/// a boundary reading at the midpoint is the door and the walk parting:
/// a boundary edge within the band of the locus without meeting it (an
/// arc tangent to it at a cut, a corner that barely turns off it), or,
/// where the run's `K` is under two, a cut within the band of the
/// midpoint. Neither says the locus runs through the face's interior,
/// nor that it misses the face.
fn contains<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    p: Point3<T>,
    band: Band,
) -> Result<Option<bool>, Indeterminate> {
    let unread = Indeterminate {
        margin: geom_core::MarginDiag::INVALID,
        band,
        predicate: Some("seam_cover_unread"),
        terminal_sliver: false,
    };
    let lift = |e: super::contain::ContainError| match e {
        super::contain::ContainError::Escalated { diag, .. } => diag,
        super::contain::ContainError::StaleFace(face) => super::contain::driver_face_stale(face),
        _ => unread,
    };
    let f = body.get_face(face).ok_or(unread)?;
    let read = match body.get_surface(f.surface) {
        Some(geom::Surface::Plane { normal, .. }) => {
            super::contain::contfp(body, face, *normal, p, band).map(Some)
        }
        Some(_) => super::contain::curved_face_containment(body, face, p, band),
        None => return Err(unread),
    };
    // The boundary pass placed the point at an arc's end and clear of
    // its corners, two definite readings no verdict reconciles at this
    // band: the read does not place the point, as it does not place a
    // boundary curve outside its envelope.
    let read = match read {
        Err(super::contain::ContainError::ArcEndClearOfCorners(_)) => return Ok(None),
        read => read.map_err(lift)?,
    };
    match read {
        Some(super::contain::FaceContainment::In) => Ok(Some(true)),
        Some(super::contain::FaceContainment::Out) => Ok(Some(false)),
        Some(
            super::contain::FaceContainment::OnEdge(_)
            | super::contain::FaceContainment::OnVertex(_),
        ) => Ok(None),
        None => Err(unread),
    }
}

/// The face's reach along `locus`; `None` when one of its boundary
/// edges is a curve the read does not place (neither a line nor a
/// circle).
fn reach<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    locus: Locus<T>,
    band: Band,
) -> Result<Option<Reach<T>>, Indeterminate> {
    let mut out = Reach {
        rides: Vec::new(),
        cuts: Vec::new(),
    };
    let tau = T::from_f64(core::f64::consts::TAU);
    for (he, curve) in face_boundary_arcs(body, face) {
        let (t0, t1) = curve.params();
        let (p0, p1) = (curve.carrier().eval(t0), curve.carrier().eval(t1));
        // Each end on the locus is a cut.
        for p in [p0, p1] {
            if on_locus(locus, p, band)? {
                out.cuts.push(locus_param(locus, p));
            }
        }
        if rides(curve, locus, band)? {
            let mid = (t0 + t1) * T::from_f64(0.5);
            let p = curve.carrier().eval(mid);
            let along = curve.carrier().deriv(mid) * if he { T::one() } else { -T::one() };
            let tangent = match locus {
                Locus::Rim(rim) => rim.axis.cross(p - rim.center),
                Locus::Line { dir, .. } => dir,
            };
            let cos = along.dot(tangent) / (along.norm() * tangent.norm());
            let name = match locus {
                Locus::Rim(_) => "seam_rim_traversal",
                Locus::Line { .. } => "seam_line_traversal",
            };
            let sign = crate::validate::decide(name, Margin::of(cos), band)?;
            if sign == geom_core::Sign::Zero {
                return Err(Indeterminate {
                    margin: geom_core::MarginDiag::INVALID,
                    band,
                    predicate: Some("seam_traversal_unread"),
                    terminal_sliver: false,
                });
            }
            let span = match locus {
                Locus::Line { .. } => {
                    let (u, w) = (locus_param(locus, p0), locus_param(locus, p1));
                    (u.min(w), u.max(w))
                }
                // The arc's span in the rim's own sense: from where the
                // traversal against the rim's tangent says it starts.
                Locus::Rim(_) => {
                    let len = t1 - t0;
                    let start = match curve_sense(curve, locus, band)? {
                        geom_core::Sign::Negative => locus_param(locus, p1),
                        _ => locus_param(locus, p0),
                    };
                    let start = start - tau * (start / tau).floor();
                    (start, start + len)
                }
            };
            out.cuts.push(span.0);
            out.cuts.push(span.1 - tau * (span.1 / tau).floor());
            out.rides.push((span.0, span.1, sign));
            continue;
        }
        // A non-riding edge: where its interior crosses the locus.
        let Some(points) = crossings(curve, locus, band)? else {
            return Ok(None);
        };
        for q in points {
            out.cuts.push(locus_param(locus, q));
        }
    }
    Ok(Some(out))
}

/// The sense in which a circle edge's PARAMETER runs about the rim's
/// axis: `Positive` with it, `Negative` against.
fn curve_sense<T: Decide>(
    curve: &geom_brep::EdgeCurve<T>,
    locus: Locus<T>,
    band: Band,
) -> Result<geom_core::Sign, Indeterminate> {
    match (locus, curve.carrier()) {
        (Locus::Rim(rim), geom::Curve3::Circle { axis, .. }) => {
            crate::validate::decide("seam_rim_axis_sense", Margin::of(axis.dot(rim.axis)), band)
        }
        _ => Ok(geom_core::Sign::Positive),
    }
}

/// Whether `p` lies on the locus.
fn on_locus<T: Decide>(locus: Locus<T>, p: Point3<T>, band: Band) -> Result<bool, Indeterminate> {
    let off = match locus {
        Locus::Line { origin, dir } => {
            let d = p - origin;
            (d - dir * d.dot(dir)).norm()
        }
        Locus::Rim(rim) => {
            let d = p - rim.center;
            let h = d.dot(rim.axis);
            let radial = (d - rim.axis * h).norm() - rim.radius;
            (h.powi(2) + radial.powi(2)).sqrt()
        }
    };
    Ok(
        crate::validate::decide("seam_cover_on_locus", Margin::of(off), band)?
            == geom_core::Sign::Zero,
    )
}

/// The points where a line or circle edge's INTERIOR meets the locus
/// (its ends are taken as vertices); `None` for an edge of any other
/// kind, whose crossings this read does not solve. Candidates are
/// solved in closed form, then kept where they decide on the locus and
/// strictly inside the edge.
fn crossings<T: Decide>(
    curve: &geom_brep::EdgeCurve<T>,
    locus: Locus<T>,
    band: Band,
) -> Result<Option<Vec<Point3<T>>>, Indeterminate> {
    let (t0, t1) = curve.params();
    let p0 = curve.carrier().eval(t0);
    // The closed forms' own branch tests are lenient in the band: a
    // datum that might be zero takes the zero branch, and one that might
    // not takes the other, both where the band cannot tell. Each
    // candidate either branch yields is then KEPT only where it decides
    // on the locus and strictly inside the edge, so a lenient branch
    // adds candidates and never a verdict.
    let read = |m: T| crate::validate::decide("seam_cover_crossing", Margin::of(m), band);
    let maybe_zero = |m: T| {
        !matches!(
            read(m),
            Ok(geom_core::Sign::Positive | geom_core::Sign::Negative)
        )
    };
    let maybe_nonzero = |m: T| !matches!(read(m), Ok(geom_core::Sign::Zero));
    let maybe_not_positive = |m: T| !matches!(read(m), Ok(geom_core::Sign::Positive));

    // Strictly inside an edge: a root within the band of an end is that
    // end, which is read as a vertex (a cut where it lies on the locus),
    // so the band answers "not inside" here rather than escalating. A
    // meridian tangent to the rim's plane at its end puts its double
    // root there.
    let inside = |m: T| matches!(read(m), Ok(geom_core::Sign::Positive));
    let mut candidates: Vec<Point3<T>> = Vec::new();
    match (curve.carrier(), locus) {
        (geom::Curve3::Line { .. }, _) => {
            let p1 = curve.carrier().eval(t1);
            let w = p1 - p0;
            match locus {
                Locus::Line { origin, dir } => {
                    // The closest approach of the segment's line to the
                    // locus.
                    if maybe_nonzero(w.cross(dir).norm()) {
                        candidates.push(crate::census::segment_cross_point(
                            (p0, p1),
                            (origin, origin + dir),
                        ));
                    }
                }
                Locus::Rim(rim) => {
                    let nw = rim.axis.dot(w);
                    let h0 = rim.axis.dot(p0 - rim.center);
                    if maybe_nonzero(nw) {
                        candidates.push(p0 + w * (-h0 / nw));
                    }
                    if maybe_zero(nw) && maybe_zero(h0) {
                        // In the rim's plane: |p0 + λw − c|² = r².
                        let d = p0 - rim.center;
                        let (qa, qb, qc) = (
                            w.dot(w),
                            d.dot(w) * T::from_f64(2.0),
                            d.dot(d) - rim.radius.powi(2),
                        );
                        let disc = qb.powi(2) - qa * qc * T::from_f64(4.0);
                        if maybe_not_positive(-disc) {
                            let root = disc.max(T::zero()).sqrt();
                            for sgn in [-1.0, 1.0] {
                                candidates.push(
                                    p0 + w
                                        * ((-qb + root * T::from_f64(sgn))
                                            / (qa * T::from_f64(2.0))),
                                );
                            }
                        }
                    }
                }
            }
            let len = w.norm();
            let mut kept = Vec::new();
            for q in candidates {
                let lambda = (q - p0).dot(w) / (len.powi(2));
                if inside(lambda * len)
                    && inside((T::one() - lambda) * len)
                    && on_locus(locus, q, band)?
                {
                    kept.push(q);
                }
            }
            Ok(Some(kept))
        }
        (
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                ..
            },
            _,
        ) => {
            let (ce, ae, re) = (*center, *axis, *radius);
            let e1 = {
                let d = p0 - ce;
                d / d.norm()
            };
            let e2 = ae.cross(e1);
            let at = |phi: T| {
                let (sn, cs) = phi.sin_cos();
                ce + (e1 * cs + e2 * sn) * re
            };
            match locus {
                Locus::Line { origin, dir } => {
                    let ad = ae.dot(dir);
                    if maybe_nonzero(ad) {
                        candidates.push(origin + dir * (ae.dot(ce - origin) / ad));
                    }
                    if maybe_zero(ad) && maybe_zero(ae.dot(origin - ce)) {
                        let foot = origin + dir * (ce - origin).dot(dir);
                        let h = (foot - ce).norm();
                        if maybe_not_positive(h - re) {
                            let half = (re.powi(2) - h.powi(2)).max(T::zero()).sqrt();
                            candidates.push(foot - dir * half);
                            candidates.push(foot + dir * half);
                        }
                    }
                }
                Locus::Rim(rim) => {
                    // n·(p(φ) − c) = A cos φ + B sin φ + C.
                    let n = rim.axis;
                    let (qa, qb, qc) = (n.dot(e1) * re, n.dot(e2) * re, n.dot(ce - rim.center));
                    let rr = (qa.powi(2) + qb.powi(2)).sqrt();
                    if maybe_nonzero(rr) {
                        let ratio = -qc / rr;
                        if maybe_not_positive(ratio.abs() - T::one()) {
                            let alpha = qb.atan2(qa);
                            let delta = ratio.max(-T::one()).min(T::one()).acos();
                            candidates.push(at(alpha + delta));
                            candidates.push(at(alpha - delta));
                        }
                    }
                    if maybe_zero(rr) && maybe_zero(qc) {
                        // Coplanar circles: the two intersection points.
                        let d = rim.center - ce;
                        let dist = d.norm();
                        if maybe_nonzero(dist) {
                            let x = (dist.powi(2) + re.powi(2) - rim.radius.powi(2))
                                / (dist * T::from_f64(2.0));
                            let y2 = re.powi(2) - x.powi(2);
                            if maybe_not_positive(-y2) {
                                let ux = d / dist;
                                let uy = ae.cross(ux);
                                let y = y2.max(T::zero()).sqrt();
                                candidates.push(ce + ux * x + uy * y);
                                candidates.push(ce + ux * x - uy * y);
                            }
                        }
                    }
                }
            }
            // Strictly inside the arc: its angle from the start, about
            // the edge's own axis, inside the parameter span.
            let span = t1 - t0;
            let tau = T::from_f64(core::f64::consts::TAU);
            let mut kept = Vec::new();
            for q in candidates {
                let d = q - ce;
                let ang = ae.dot(e1.cross(d)).atan2(e1.dot(d));
                let ang = ang - tau * (ang / tau).floor();
                if inside(ang * re) && inside((span - ang) * re) && on_locus(locus, q, band)? {
                    kept.push(q);
                }
            }
            Ok(Some(kept))
        }
        _ => Ok(None),
    }
}

/// Whether a boundary curve rides `locus`: a circle decided the rim's
/// own, or a line segment whose two ends lie on the locus line.
fn rides<T: Decide>(
    curve: &geom_brep::EdgeCurve<T>,
    locus: Locus<T>,
    band: Band,
) -> Result<bool, Indeterminate> {
    match (locus, curve.carrier()) {
        (
            Locus::Rim(rim),
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            },
        ) => same_circle(
            Rim {
                center: *center,
                axis: *axis,
                radius: *radius,
                u_ref: *u_ref,
            },
            rim,
            band,
        )
        .map(|one| one.is_some()),
        (Locus::Line { origin, dir }, geom::Curve3::Line { .. }) => {
            let (t0, t1) = curve.params();
            for p in [curve.carrier().eval(t0), curve.carrier().eval(t1)] {
                let d = p - origin;
                let off = (d - dir * d.dot(dir)).norm();
                if crate::validate::decide("seam_line_collinear", Margin::of(off), band)?
                    != geom_core::Sign::Zero
                {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}

/// Each boundary half-edge of `face` with a certified curve: whether it
/// is its edge's `he_plus` (so runs the curve forward), and the curve.
/// Empty for a face key that no longer resolves.
///
/// # Panics
///
/// Where a record past `face` does not resolve or a loop walk does not
/// close (D2 row 4): its loops, a lone vertex's point, their members,
/// each member's edge and curve. A torn curve is not null scaffolding.
/// The links hold at rest and, mid-operation, by
/// [`crate::live::OPERATORS_KEEP_LINKS`].
pub(crate) fn face_boundary_arcs<T: Real>(
    body: &Body<T>,
    face: FaceKey,
) -> Vec<(bool, &geom_brep::EdgeCurve<T>)> {
    let mut out = Vec::new();
    let Some(f) = body.get_face(face) else {
        return out;
    };
    for member in body.face_boundary_linked(face, f) {
        let crate::live::BoundaryMember::Edge { he, ek, edge, .. } = member else {
            continue;
        };
        if let Some(c) = body.edge_curve_linked(ek, edge).certified() {
            out.push((edge.he_plus == he, c));
        }
    }
    out
}

/// The circle carriers of a face's boundary edges.
fn face_boundary_circles<T: Real>(body: &Body<T>, face: FaceKey) -> Vec<Rim<T>> {
    face_boundary_arcs(body, face)
        .into_iter()
        .filter_map(|(_, c)| match *c.carrier() {
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => Some(Rim {
                center,
                axis,
                radius,
                u_ref,
            }),
            _ => None,
        })
        .collect()
}

/// **The routing itself**: the wedge the two faces' material subtends
/// across the rim, sampled around the circle.
///
/// **The first-order screen runs FIRST, and it is the precondition the
/// imported fold is only valid behind.** Check 4's material arm reads
/// `sign(n̂₊ · n̂₋)`, which distinguishes one material side from two —
/// and NOTHING else. On a pair whose tangent planes are definitely
/// distinct that sign is still perfectly well defined and perfectly
/// meaningless: a sphere cut by a plane at 53° has aligned normals at
/// its rim and would read "seam", which is a false statement about the
/// geometry rather than a missing verdict. Tier 3 never asks the
/// question there because it reaches the material arm only after every
/// interior sample classified definitely `Smooth`; this door owes the
/// same screen, so `classify_dihedral` runs per sample before anything
/// second-order is computed. Importing a fold means importing what it
/// is defined over.
///
/// The five outcomes are the validator's own, one for one:
///
/// - every sample `Transverse` ⇒ [`RimRouting::Transverse`], set
///   DIRECTLY and never through the fold (which cannot produce it —
///   the material arm is not consulted on a genuine corner);
/// - every sample `Smooth` ⇒ the material arm, whose verdicts are the
///   seam, the two cusp ends and the lamina;
/// - samples that DISAGREE ⇒ escalated, naming `dihedral_wedge`: a rim
///   that is a corner in one place and a tangency in another is not
///   one contact, and no single routing is honest for it;
/// - a `classify_dihedral` escalation ⇒ passed through verbatim;
/// - a spline-chart face on either side ⇒ escalated: the implicit
///   gradient is poison on a fit, so neither the screen nor the arm can
///   run, and answering from a poisoned normal is worse than declining.
///
/// The samples are the certification schedule's
/// (`geom_brep::CERT_SAMPLES`), taken at uniform phase around the CLOSED
/// rim starting at its own `u_ref`. **That schedule differs from tier
/// 3's on purpose**: an edge is an open arc whose endpoints are vertices
/// already classified by other rules, so the validator samples its
/// interior (`geom_brep::interior_stations`); a rim is a closed circle
/// with no endpoint to exclude, so every sample is interior to it and
/// the phase-zero sample is an ordinary point of the contact, not a
/// boundary site. Sampling `1..n-1` here would drop two of nine
/// readings for a reason that does not apply. The schedule is the only
/// difference: the material arm is tier 3's own per-station reads
/// (`validate::MaterialStations`) walked through the one
/// `geom_brep::second_order_walk`, and every sample must agree — a rim
/// whose own samples disagree escalates rather than being decided by
/// majority, which is the fold's rule.
///
/// `reach` is the lever arm the wedge's margins are metered at — the
/// two faces' extent ([`fn@crate::wedge_reach`]), as every reader of the
/// wedge between an edge's faces levers it.
///
/// `sense_plus`/`sense_minus` are the two faces' `Face::sense` BITS,
/// passed through to the `geom_brep` doors that mint the outward
/// normals themselves; a bit rather than a `T` ±1 for the reason
/// [`geom_brep::OutwardNormal`]'s doc gives at its one constructor.
/// (Named that way round on purpose: the constructor's own spelling in
/// a file under `topo/src` reds `face_normal.rs`'s anti-re-fork row,
/// which reads raw text.)
///
/// # Errors
///
/// [`Indeterminate`] naming the predicate that could not decide.
pub(crate) fn classify_shared_rim<T: Decide>(
    s_plus: &geom::Surface<T>,
    sense_plus: bool,
    s_minus: &geom::Surface<T>,
    sense_minus: bool,
    rim: Rim<T>,
    reach: T,
    band: Band,
) -> Result<RimRouting, Indeterminate> {
    let Rim {
        center,
        axis,
        radius,
        u_ref: u,
    } = rim;
    let v = axis.cross(u);
    let n = geom_brep::CERT_SAMPLES;
    let phase = |i: u32| core::f64::consts::TAU * (f64::from(i) / f64::from(n));
    // The sample points and their rim tangents, derived once: the
    // first-order screen and the material arm must read the SAME
    // stations, or the screen certifies a rim the arm did not judge.
    let station = |i: u32| {
        let theta = T::from_f64(phase(i));
        let (s, c) = (theta.sin(), theta.cos());
        (center + (u * c + v * s) * radius, (v * c - u * s) * radius)
    };

    // ---- The first-order screen (docs above): the precondition the
    // material arm is only defined behind. ----
    //
    // A spline chart on either side is exempt by KIND and says so: the
    // implicit gradient both stages read is poison on a fit, so this is
    // "the question cannot be posed here", not "the answer is no".
    if s_plus.spline_chart().is_some() || s_minus.spline_chart().is_some() {
        return Err(Indeterminate {
            margin: geom_core::MarginDiag::INVALID,
            band,
            predicate: Some("dihedral_wedge"),
            terminal_sliver: false,
        });
    }
    let mut all_transverse = true;
    let mut all_smooth = true;
    for i in 0..n {
        let (p, _) = station(i);
        match geom_brep::classify_dihedral(s_plus, s_minus, p, reach, band)
            .map_err(|escalation| escalation.diag())?
        {
            geom_brep::DihedralClass::Transverse => all_smooth = false,
            geom_brep::DihedralClass::Smooth => all_transverse = false,
        }
    }
    if all_transverse {
        // Set DIRECTLY, exactly as the validator does: a genuine corner
        // is a first-order verdict and the material arm has no say in
        // it. This is also why the fold below can never return
        // `Transverse` — nothing routes to it there.
        return Ok(RimRouting::Transverse);
    }
    if !all_smooth {
        // Corner at one station, tangency at another: not one contact,
        // and no single routing is honest for it.
        return Err(Indeterminate {
            margin: geom_core::MarginDiag::INVALID,
            band,
            predicate: Some("dihedral_wedge"),
            terminal_sliver: false,
        });
    }

    // ---- The material arm, now validly posed: check 4's own reads,
    // walked over the rim's stations. ----
    let mut stations = MaterialStations::new(s_plus, sense_plus, s_minus, sense_minus, band);
    let jet_determinate = match geom_brep::second_order_walk(
        s_plus,
        s_minus,
        (0..n).map(station),
        reach,
        band,
        &mut stations,
    ) {
        geom_brep::SecondOrderWalk::Determinate => true,
        geom_brep::SecondOrderWalk::UnderDetermined => false,
        geom_brep::SecondOrderWalk::InBand(cause) => return Err(cause),
        geom_brep::SecondOrderWalk::Stopped(stop) => return Err(stop.cause),
    };
    match stations.outcome(jet_determinate) {
        MaterialArmOutcome::Wedge(MaterialWedge::Seam) => Ok(RimRouting::Seam),
        // `Transverse` is unreachable from the fold BY CONSTRUCTION —
        // the validator sets it directly off the first-order screen and
        // so does the screen above, so nothing routes to it here. Kept
        // total rather than `unreachable!`: a verdict this door cannot
        // account for is reported, never assumed away.
        MaterialArmOutcome::Wedge(MaterialWedge::Transverse) => Ok(RimRouting::Transverse),
        MaterialArmOutcome::Wedge(w) => Ok(RimRouting::Cusp(w)),
        MaterialArmOutcome::Lamina => Ok(RimRouting::Lamina),
        MaterialArmOutcome::Split { predicate } => Err(Indeterminate {
            margin: geom_core::MarginDiag::INVALID,
            band,
            predicate: Some(predicate),
            terminal_sliver: false,
        }),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod cover_rows {
    use super::*;
    use geom_core::Tol;

    /// **A face that runs one stretch of the locus both ways is mixed.**
    /// The two half-edges of one edge on the locus (a chart seam, a
    /// bridge) both cover the piece, with opposite traversals; the
    /// first one found must not stand for the face. No body puts such a
    /// face on a seam's locus through the operations, so the reach is
    /// stated here, against a line, with the face's own containment
    /// never asked (a riding edge answers first).
    #[test]
    fn a_stretch_ridden_both_ways_reads_mixed() {
        let band = Band::linear(Tol::witness()).unwrap();
        let body: Body<f64> =
            crate::test_support_fixtures::brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
        let face = body.faces().next().map(|(k, _)| k).unwrap();
        let line = Locus::Line {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 0.0, 0.0),
        };
        let at = |rides: Vec<(f64, f64, geom_core::Sign)>| {
            Reach {
                rides,
                cuts: Vec::new(),
            }
            .at(&body, face, line, 0.5, 1.0, band)
            .unwrap()
        };
        use geom_core::Sign::{Negative, Positive};
        assert_eq!(at(vec![(0.0, 1.0, Positive)]), Cover::Rides(Positive));
        assert_eq!(
            at(vec![(0.0, 1.0, Positive), (0.0, 1.0, Negative)]),
            Cover::RidesBothWays
        );
        assert_eq!(
            at(vec![(0.0, 1.0, Positive), (0.2, 0.8, Positive)]),
            Cover::Rides(Positive),
            "two edges one way are one side"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod redfirst {
    use super::*;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn circle(center: [f64; 3], axis: [f64; 3], radius: f64) -> Rim<f64> {
        Rim {
            center: Point3::from_array(center),
            axis: Vec3::from_array(axis),
            radius,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// The rim-identity comparison, driven directly on two circles —
    /// the three-outcome ε row every one of its margins owes, stated in
    /// BAND UNITS so it means the same thing at every ε the matrix
    /// runs.
    ///
    /// `shared_rim` reads circles off face boundaries, which no unit
    /// fixture in this crate can mint; the comparison it performs is
    /// this one, so the rows drive it through the same `decide` calls
    /// with the same names and the same band.
    fn rim_identity(ra: &Rim<f64>, rb: &Rim<f64>, band: Band) -> Result<bool, Indeterminate> {
        let mut same = true;
        for (name, margin) in [
            ("rim_circle_radius", Margin::of(ra.radius - rb.radius)),
            ("rim_circle_center", Margin::norm3(ra.center - rb.center)),
            (
                "rim_circle_axis_parallel",
                Margin::levered(ra.axis.cross(rb.axis).norm(), ra.radius + rb.radius),
            ),
        ] {
            match crate::validate::decide(name, margin, band)? {
                geom_core::Sign::Zero => {}
                _ => {
                    same = false;
                    break;
                }
            }
        }
        Ok(same)
    }

    /// **MIN-3, the swallowed escalation.** A radius perturbed INTO the
    /// band is the sliver case: it must escalate typed naming the
    /// predicate, never read as "not this pair", which is what a
    /// definitely-different radius reads as. Three outcomes, one datum.
    #[test]
    fn rim_radius_epsilon_row_three_outcomes() {
        let b = band();
        let base = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.06);
        assert!(rim_identity(&base, &base, b).unwrap(), "exact: one rim");
        let in_band = circle(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            5.06 + (b.zero() + b.escalate()) * 0.5,
        );
        match rim_identity(&base, &in_band, b) {
            Err(d) => assert_eq!(d.predicate, Some("rim_circle_radius")),
            Ok(v) => panic!("in-band must escalate, not answer {v}"),
        }
        let definite = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.06 + b.escalate() * 1e6);
        assert!(
            !rim_identity(&base, &definite, b).unwrap(),
            "definite: not this pair"
        );
    }

    /// The centre datum's own three outcomes.
    #[test]
    fn rim_center_epsilon_row_three_outcomes() {
        let b = band();
        let base = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.06);
        let in_band = circle(
            [(b.zero() + b.escalate()) * 0.5, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            5.06,
        );
        match rim_identity(&base, &in_band, b) {
            Err(d) => assert_eq!(d.predicate, Some("rim_circle_center")),
            Ok(v) => panic!("in-band must escalate, not answer {v}"),
        }
        let definite = circle([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], 5.06);
        assert!(!rim_identity(&base, &definite, b).unwrap());
    }

    /// **MIN-2, the lever arm.** The angular datum is metered at the
    /// rim's own diameter, so the SAME tilt answers differently on a
    /// small rim and a large one — which is the whole content of
    /// "levered at the consumption extent". A literal unit arm would
    /// give one answer for both.
    #[test]
    fn rim_axis_tilt_is_decided_at_the_rims_own_diameter() {
        let b = band();
        // A tilt sub-band over a 0.12 m rim and definite over a 2000 km
        // one, stated in band units.
        let tilt = b.zero() * 0.5;
        let small = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.06);
        let small_tilted = circle([0.0, 0.0, 0.0], [tilt, 0.0, 1.0], 0.06);
        assert!(
            rim_identity(&small, &small_tilted, b).unwrap(),
            "at a 0.12 m rim the tilt is below the band"
        );
        let big = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0e6);
        let big_tilted = circle([0.0, 0.0, 0.0], [tilt, 0.0, 1.0], 1.0e6);
        assert!(
            !matches!(rim_identity(&big, &big_tilted, b), Ok(true)),
            "the same tilt over a 2000 km rim is not the same circle"
        );
    }

    /// **The two sense words route the same geometry to different
    /// arms**, which is what makes the door's `sense` parameters
    /// load-bearing rather than decoration. The unit sphere about the
    /// origin and the coaxial unit cylinder are tangent all along the
    /// equator. With both chart normals kept, the two outward normals
    /// agree: one material side, the π seam. Reverse the cylinder and
    /// they oppose — the sphere's material is inside it, the
    /// cylinder's is outside — so together they fill everything except
    /// the crescent between the two sheets, which is the wedge-2π
    /// knife slit.
    #[test]
    fn the_two_senses_route_a_tangent_rim_to_different_arms() {
        let sphere = geom::Surface::Sphere {
            center: Point3::origin(),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let cylinder = geom::Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let equator = Rim {
            center: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let same = classify_shared_rim(&sphere, true, &cylinder, true, equator, 2.0, band())
            .expect("the equator tangency routes");
        let opposed = classify_shared_rim(&sphere, true, &cylinder, false, equator, 2.0, band())
            .expect("and so does its reversed reading");
        assert_eq!(
            same,
            RimRouting::Seam,
            "aligned material sides are the seam"
        );
        assert_eq!(
            opposed,
            RimRouting::Cusp(geom_brep::MaterialWedge::Slit),
            "the crescent between the sheets is the void, so the material is the slit"
        );
        assert_ne!(
            same, opposed,
            "anti-vacuity: a door that ignored its sense bits would answer the same twice"
        );
    }

    /// RED-FIRST (MAJ-1): a unit sphere cut by z = 0.6 is a DEFINITE
    /// 53-degree crossing, not a tangency. The routing must not call
    /// it a seam.
    #[test]
    fn a_transverse_rim_is_not_a_seam() {
        let sphere = geom::Surface::Sphere {
            center: Point3::origin(),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let plane = geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.6),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let rim = Rim {
            center: Point3::new(0.0, 0.0, 0.6),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.8,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let got = classify_shared_rim(&sphere, true, &plane, true, rim, 1.6, band());
        println!("transverse rim answers {got:?}");
        assert!(
            !matches!(got, Ok(RimRouting::Seam)),
            "a definite 53-degree crossing is not a smooth seam: {got:?}"
        );
        let flipped = classify_shared_rim(&sphere, true, &plane, false, rim, 1.6, band());
        println!("transverse rim, reversed sense, answers {flipped:?}");
        assert!(
            !matches!(flipped, Ok(RimRouting::Cusp(_))),
            "reversing a face sense cannot turn a crossing into a cusp: {flipped:?}"
        );
    }
}

/// R2 review probes for MATE-7a (PR #1477). Not the unit's rows —
/// reviewer measurements of what the routing answers on inputs the
/// unit's own fixtures do not cover.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod r2_probes {
    use super::*;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn sphere() -> geom::Surface<f64> {
        geom::Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
        }
    }

    fn cut_plane(sign: f64) -> geom::Surface<f64> {
        geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.6),
            normal: Vec3::new(0.0, 0.0, sign),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn cut_rim() -> Rim<f64> {
        Rim {
            center: Point3::new(0.0, 0.0, 0.6),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.8,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// **The tangency screen, at R2's own fixture.** `validate`'s
    /// check-4 material arm — whose fold this module imports — is
    /// entered only after `classify_dihedral` reports every interior
    /// sample definitely `Smooth`. R2 found `classify_shared_rim`
    /// running that fold with no such precondition, so a rim where the
    /// two surfaces genuinely CROSS was classified anyway: the unit
    /// sphere and the plane `z = 0.6`, meeting at a definite 53
    /// degrees, came back `Seam`.
    ///
    /// **INVERTED at the fix pass**, which is what this row is for. It
    /// was written to demonstrate the defect and asserted the defect's
    /// output; the screen now runs, so the same fixture answers
    /// `Transverse` and the row asserts THAT. The fixture, the numbers
    /// and the argument are R2's; only the expected value moved, and it
    /// moved because the code was fixed rather than because the row was
    /// weakened — a `Seam` here would red it again.
    #[test]
    fn r2_a_plainly_transverse_rim_is_not_classified_as_a_seam() {
        let got = classify_shared_rim(
            &sphere(),
            true,
            &cut_plane(1.0),
            true,
            cut_rim(),
            1.6,
            band(),
        );
        println!("[r2] transverse sphere/plane rim routes to {got:?}");
        assert_eq!(
            got.expect("the routing answers rather than escalating"),
            RimRouting::Transverse,
            "a definite 53-degree crossing is a genuine corner, not a wedge-pi seam"
        );
    }

    /// R2's sharper half: the same crossing with the plane's stored
    /// normal REVERSED used to be sent to the OTHER unbuilt arm, so
    /// which arm a transverse rim earned depended on a stored sign
    /// rather than on the geometry.
    ///
    /// **INVERTED for the same reason, and this is the stronger claim
    /// of the two**: transversality is a first-order fact and a face
    /// sense cannot touch it, so both orientations must now give the
    /// SAME answer. Asserted as an equality between the two runs rather
    /// than as two separate expectations, so a future routing that made
    /// them differ again reds here however it did it.
    #[test]
    fn r2_the_same_crossing_cannot_flip_arm_with_the_stored_normal() {
        let up = classify_shared_rim(
            &sphere(),
            true,
            &cut_plane(1.0),
            true,
            cut_rim(),
            1.6,
            band(),
        );
        let down = classify_shared_rim(
            &sphere(),
            true,
            &cut_plane(-1.0),
            true,
            cut_rim(),
            1.6,
            band(),
        );
        println!("[r2] normal up routes to {up:?}; normal down routes to {down:?}");
        assert_eq!(
            up.expect("the routing answers"),
            down.expect("the routing answers"),
            "a stored sign cannot change a first-order verdict"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod arc_end_rows {
    use super::*;
    use crate::test_support_fixtures::{half_disc_cap, prism_z};
    use geom_core::{Tol, Vec3};

    /// The point the band `(ε, 1.25ε)` reads at the half-disc cap's arc
    /// end and clear of its corner: outside the circle and along it, each
    /// by `0.95ε`, so `≈ 1.34ε` from the corner and as far from the chord.
    fn beside_the_arc_end(end: Point3<f64>, zero: f64) -> Point3<f64> {
        end + Vec3::new(-0.95 * zero, 0.95 * zero, 0.0)
    }

    /// **A touch piece at an arc's end its corners disagree with is not
    /// read**: the boundary pass places the point at the end and clear of
    /// the corner, so `contains` has no verdict, and the seam cover
    /// refuses the claim as outside its envelope
    /// ([`Departure::EdgeKindUnread`], the claim's `unsupported`) rather
    /// than reading the piece as the face's interior (`NoEdge`, a decided
    /// contradiction downstream).
    ///
    /// - At `contains`, the finding's arm on a half-disc cap the public
    ///   operators built: an end off both corners, read at a band finer
    ///   than the run's.
    /// - The band's own parting (`K = 1.25`) no longer parts the end from
    ///   its corner: the boundary reading puts a point at the end only on
    ///   its raw chord, the distance the vertex pass reads to the corner.
    ///   The point beside the end escalates at `contains` on the end's
    ///   straddle (`curve_pair_span_end_straddle`), and so does the touch
    ///   piece through `departures`: a line along `x` through the point,
    ///   which crosses the cap's arc just past it, and a block's top in
    ///   the cap's plane whose left edge stands the same distance before
    ///   it, so the one touch piece's midpoint is the point.
    #[test]
    fn a_touch_at_an_arc_end_clear_of_its_corners_is_not_read() {
        let tol = Tol::witness();
        let zero = Band::linear(tol).unwrap().zero();
        let off = zero / 2.0;
        let (body, cap, end) = half_disc_cap(off, tol);
        let fine = Band::new(off / 50.0, off / 5.0).unwrap();
        assert!(
            matches!(
                crate::boolean::contfp(&body, cap, Vec3::unit_z(), end, fine),
                Err(crate::boolean::ContainError::ArcEndClearOfCorners(ref g)) if g.off_corner()
            ),
            "the end lies off both corners at the finer band"
        );
        assert_eq!(contains(&body, cap, end, fine), Ok(None), "off its corner");
        let (body, cap, end) = half_disc_cap(0.0, tol);
        let q = beside_the_arc_end(end, zero);
        let narrow = Band::new(zero, 1.25 * zero).unwrap();
        let straddle = |d: &Indeterminate| d.predicate == Some("curve_pair_span_end_straddle");
        assert!(
            matches!(
                contains(&body, cap, q, narrow),
                Err(ref d) if straddle(d)
            ),
            "the band's parting escalates: {:?}",
            contains(&body, cap, q, narrow)
        );
        // The line crosses the unit circle at `x = -sqrt(1 - y²)`, `delta`
        // past the point; the block's left edge stands `delta` before it.
        let delta = -(1.0 - q.y * q.y).sqrt() - q.x;
        let h = 0.1;
        let block = prism_z::<f64>(
            &[
                (q.x - delta, q.y - h),
                (q.x + h, q.y - h),
                (q.x + h, q.y + h),
                (q.x - delta, q.y + h),
            ],
            -1.0,
            0.0,
            tol,
        );
        let locus = Locus::Line {
            origin: q,
            dir: Vec3::unit_x(),
        };
        let got = departures(&body, cap, &block.body, block.top_face, locus, narrow);
        assert!(
            matches!(got, Err(ref d) if straddle(d)),
            "the touch piece at the arc's end escalates: {got:?}"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod boundary_midpoint_rows {
    use super::*;
    use crate::test_support_fixtures::{Prism, arc_cap, prism_z};
    use geom_core::{Tol, Vec3};

    /// A block in the plane `z = 0` (its top face there) over the polygon
    /// `corners`, counterclockwise about `+z`.
    fn block(corners: &[Point3<f64>], tol: Tol) -> Prism<f64> {
        let profile: Vec<(f64, f64)> = corners.iter().map(|p| (p.x, p.y)).collect();
        prism_z::<f64>(&profile, -1.0, 0.0, tol)
    }

    /// A frame in the plane `z = 0`: the point on the unit circle at
    /// angle `theta`, the circle's tangent `t` there and its outward
    /// normal `n` (`t × n = +z`), as a map `(u, v) ↦ p + u·t + v·n`.
    fn frame(theta: f64) -> (Locus<f64>, impl Fn(f64, f64) -> Point3<f64>) {
        let p = Point3::new(theta.cos(), theta.sin(), 0.0);
        let n = Vec3::new(theta.cos(), theta.sin(), 0.0);
        let t = Vec3::new(theta.sin(), -theta.cos(), 0.0);
        (Locus::Line { origin: p, dir: t }, move |u, v| {
            p + t * u + n * v
        })
    }

    /// The tangent line's poses, as angles on the unit circle.
    const POSES: [f64; 3] = [core::f64::consts::FRAC_PI_2, 1.0, 2.3];

    /// The gap between a touch piece's midpoint and the face boundary
    /// that does not meet it, in units of the run's zero: on the
    /// boundary, in the band, and clear of it.
    const GAPS: [f64; 3] = [0.25, 3.0, 20.0];

    /// What the seam cover answers at each of [`GAPS`]: the piece not
    /// placed, the door's own escalation naming `predicate`, and the
    /// geometry's answer `clear`.
    fn assert_rows(
        got: [Result<Departure, Indeterminate>; 3],
        predicate: &str,
        clear: Departure,
        what: &str,
    ) {
        assert_eq!(
            got[0],
            Ok(Departure::EdgeKindUnread),
            "{what}: a midpoint on the boundary is not placed, never `NoEdge`"
        );
        assert!(
            matches!(got[1], Err(Indeterminate { predicate: Some(p), .. }) if p == predicate),
            "{what}: a midpoint in the band escalates on `{predicate}`, got {:?}",
            got[1]
        );
        assert_eq!(
            got[2],
            Ok(clear),
            "{what}: a midpoint clear of the boundary"
        );
    }

    /// **A touch piece whose midpoint the face's door reads on its
    /// boundary is not placed**, along a line. The other face rides the
    /// piece; the face read there does not meet the line inside it, and
    /// its door places the midpoint on an edge that comes within the band
    /// of the line without meeting it:
    ///
    /// - an ARC tangent to the line at the piece's start (the unit
    ///   circle's half-disc cap): `δ²/8` from the midpoint of a piece of
    ///   length `δ`. The cap touches the line at a point and nowhere
    ///   else, so clear of the band the pair is `Untouched`.
    /// - a CORNER that turns off the line by the slope `s = 4K·zero`: a
    ///   block rides the line up to the piece's start and leaves it
    ///   there, `s·δ/2` from the midpoint. Clear of the band the pair
    ///   meets only along the ride, on opposite banks: `Opposite`.
    ///
    /// Neither is the face's interior, where the read is `NoEdge`.
    #[test]
    fn a_boundary_midpoint_on_a_line_is_not_placed() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let zero = band.zero();
        let (cap_body, cap, _) = crate::test_support_fixtures::half_disc_cap(0.0, tol);
        let s = 4.0 * tol.k() * zero;
        for theta in POSES {
            let (locus, at) = frame(theta);
            let rider = |delta: f64| {
                block(
                    &[at(-1.0, 0.0), at(delta, 0.0), at(delta, 0.5), at(-1.0, 0.5)],
                    tol,
                )
            };
            let arc = GAPS.map(|gap| {
                let delta = (8.0 * gap * zero).sqrt();
                if gap < 1.0 {
                    assert_eq!(
                        contains(&cap_body, cap, at(delta / 2.0, 0.0), band),
                        Ok(None),
                        "theta {theta}: the cap's door reads the midpoint on its arc"
                    );
                }
                let b = rider(delta);
                departures(&cap_body, cap, &b.body, b.top_face, locus, band)
            });
            assert_rows(
                arc,
                "curve_carrier_gap",
                Departure::Untouched,
                &format!("tangent arc, theta {theta}"),
            );
            let corner = block(
                &[
                    at(-1.0, -1.0),
                    at(1.0, -1.0),
                    at(1.0, -s),
                    at(0.0, 0.0),
                    at(-1.0, 0.0),
                ],
                tol,
            );
            let turn = GAPS.map(|gap| {
                let delta = 2.0 * gap * zero / s;
                let b = rider(delta);
                departures(
                    &corner.body,
                    corner.top_face,
                    &b.body,
                    b.top_face,
                    locus,
                    band,
                )
            });
            assert_rows(
                turn,
                "bool_contact_edge",
                Departure::Opposite,
                &format!("shallow corner, theta {theta}"),
            );
        }
    }

    /// **A midpoint within the band of a cut is not placed either**, at a
    /// `K` under two (`(zero, 1.25·zero)`): two blocks ride the line on
    /// opposite banks up to the piece's start, where one leaves it and
    /// the other runs on `1.5·zero`. The piece decides positive, its
    /// midpoint `0.75·zero` from each end, so it is inside neither ride
    /// and both doors read it at a corner. At the run's `K` the piece is
    /// in the band and the walk escalates on its order.
    #[test]
    fn a_midpoint_at_a_corner_is_not_placed() {
        let tol = Tol::witness();
        let zero = Band::linear(tol).unwrap().zero();
        let narrow = Band::new(zero, 1.25 * zero).unwrap();
        let d = 1.5 * zero;
        for theta in POSES {
            let (locus, at) = frame(theta);
            let a = block(
                &[at(-1.0, -1.0), at(0.0, -1.0), at(0.0, 0.0), at(-1.0, 0.0)],
                tol,
            );
            let b = block(&[at(-1.0, 0.0), at(d, 0.0), at(d, 0.5), at(-1.0, 0.5)], tol);
            let mid = at(d / 2.0, 0.0);
            for (body, face, which) in [(&a.body, a.top_face, "A"), (&b.body, b.top_face, "B")] {
                assert!(
                    matches!(
                        crate::boolean::contfp(body, face, Vec3::unit_z(), mid, narrow),
                        Ok(crate::boolean::FaceContainment::OnVertex(_))
                    ),
                    "theta {theta}: {which}'s door reads the midpoint at its corner"
                );
                assert_eq!(
                    contains(body, face, mid, narrow),
                    Ok(None),
                    "theta {theta}: {which}"
                );
            }
            assert_eq!(
                departures(&a.body, a.top_face, &b.body, b.top_face, locus, narrow),
                Ok(Departure::EdgeKindUnread),
                "theta {theta}: the corner piece is not placed, never `NoEdge`"
            );
            let run = departures(
                &a.body,
                a.top_face,
                &b.body,
                b.top_face,
                locus,
                Band::linear(tol).unwrap(),
            );
            assert!(
                matches!(
                    run,
                    Err(Indeterminate {
                        predicate: Some("seam_cover_order"),
                        ..
                    })
                ),
                "theta {theta}: at the run's K the piece escalates, got {run:?}"
            );
        }
    }

    /// **The same along a rim**: a cap on the unit circle rides it from
    /// `alpha - 1` to `alpha + δ`; a cap on a circle of radius `1/2`
    /// tangent to it from inside at `alpha` meets it only there, `β²/2`
    /// from the rim at `β = δ/2` past the tangent point. Clear of the
    /// band the pair is `Untouched`.
    #[test]
    fn a_boundary_midpoint_on_a_rim_is_not_placed() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let zero = band.zero();
        let rim = Rim {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        for alpha in [0.3, 2.0, 4.0] {
            let e = Vec3::new(alpha.cos(), alpha.sin(), 0.0);
            let small = geom::Curve3::Circle {
                center: Point3::origin() + e * 0.5,
                axis: Vec3::unit_z(),
                radius: 0.5,
                u_ref: e,
            };
            let (a, fa) = arc_cap(small, (-1.0, 1.0), Vec3::new(0.0, 0.0, 0.0), tol);
            let got = GAPS.map(|gap| {
                let beta = (2.0 * gap * zero).sqrt();
                let unit = geom::Curve3::Circle {
                    center: rim.center,
                    axis: rim.axis,
                    radius: rim.radius,
                    u_ref: rim.u_ref,
                };
                let (b, fb) = arc_cap(
                    unit,
                    (alpha - 1.0, alpha + 2.0 * beta),
                    Vec3::new(0.0, 0.0, 0.0),
                    tol,
                );
                departures(&a, fa, &b, fb, Locus::Rim(rim), band)
            });
            assert_rows(
                got,
                "curve_carrier_gap",
                Departure::Untouched,
                &format!("tangent circle, alpha {alpha}"),
            );
        }
    }
}
