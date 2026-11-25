//! **The image of a carrier on a spline wall** (C2): one function of the
//! carrier, the wall and the band, which reads nothing about the edge's
//! other surface. The trace's certificate, the pcurve mint and the
//! at-rest certificate all call [`carrier_image`], so the same carrier on
//! the same wall has the same image wherever it is asked for, and a
//! stored pcurve is that value remembered.
//!
//! The image is the wall's **foot path** of the carrier, interpolated on
//! the carrier's own knots:
//!
//! - **The chain.** A spline carrier's knot vector, its node grading
//!   included, at the carrier's degree and no lower than
//!   [`IMAGE_DEGREE`]; a knot of a lower-degree carrier keeps its
//!   continuity; a periodic spline's knots are its clamped view's over
//!   the span, the net the edge certificate reads. A closed form (a
//!   line, a conic, a spiric) has no knots, and its chain is
//!   [`CLOSED_FORM_SPANS`] uniform spans of its interval.
//! - **The feet.** One at each Greville abscissa of the chain, so the
//!   interpolant is on the chain's knots exactly. They are found by
//!   **continuation**: the first is the wall's nearest point to the
//!   carrier's start, and each next one is the projection's from the
//!   last ([`follow`], `NurbsSurface::project_from_seed`), never the
//!   wall's nearest point found afresh. So the path stays on
//!   the sheet of a folded wall it started on, and reaches a closed
//!   wall's seam from the side it came by.
//! - **The start on a seam.** A start whose nearest point lies on a side
//!   of the wall's rectangle that the wall closes across has two chart
//!   points; it takes the one the path leaves into the rectangle from
//!   ([`start`]).
//! - **The miss.** Midway between each pair of feet the image is read
//!   against the path: the distance on the wall between the image's
//!   point and the carrier's foot there, followed from the image's
//!   ([`miss`]). The carrier's own standoff from the wall is no part of
//!   it. A chain whose every reading is within
//!   [`IMAGE_MISS`] of the band's zero, or [`IMAGE_STANDOFF_SHARE`] of
//!   the carrier's standoff there where that is the larger, is the image. Otherwise each
//!   knot span holding a larger one takes a knot at its middle, or,
//!   where the path crosses a knot line of the wall there across which
//!   the wall is less smooth than the image, a break at the crossing,
//!   whose foot is put on the line so each span beside it lies in one
//!   cell of the wall ([`crossing`]). The break is the knot at the
//!   image's degree (C⁰), whatever the wall's continuity across the
//!   line: only a knot of full multiplicity has a control point at the
//!   knot, which is what puts the image's point there on the line to
//!   the bit. The feet are followed again, at most [`IMAGE_ROUNDS`]
//!   times. Past that cap the function refuses, on the last round's
//!   reading in limb 2's words ([`ImageRefusal::Miss`]): a sampled
//!   reading, not a bound.
//!
//! Everything here is the `f64` structure lane (C6): the feet, the
//! knots and the miss are read at the middles of the caller's scalar's
//! brackets. The carrier and the wall are still evaluated at that
//! scalar, so the image is one per scalar: at `f64` and at `Interval`
//! the middles differ in the last places, and an image can take a knot
//! at one that it does not at the other. The image is
//! evidence, and what makes it sound is that its consumer bounds
//! `sup_t |S(P(t)) − C(t)|` over the whole span.

use geom::{Curve3, NurbsCurve2, NurbsSurface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Bounds, Decide, Point2, Point3, Vec3};

use crate::edge_nurbs::CarrierDomainRefusal;

/// The image's least degree. A carrier of a higher degree is imaged at
/// its own.
pub(crate) const IMAGE_DEGREE: usize = 3;

/// The spans of a carrier that has no knots (a line, a conic, a spiric):
/// its chain before any miss asks for more (D9).
pub(crate) const CLOSED_FORM_SPANS: usize = 8;

/// The share of `band.zero()` a chain's miss of the foot path, read
/// midway between its feet, must fall within: the certificate's hull
/// reads the miss between those midpoints too, and over-reads it.
pub(crate) const IMAGE_MISS: f64 = 1.0 / 16.0;

/// The share of the carrier's own standoff from the wall, at a reading,
/// that the image's miss there may be where that is the larger allowance
/// (D9).
pub(crate) const IMAGE_STANDOFF_SHARE: f64 = 0.25;

/// The most rounds of knot insertion (D9). Each halves the spans that
/// miss, so the last round's are `2⁻²⁴` of the chain's.
pub(crate) const IMAGE_ROUNDS: usize = 24;

/// The most knots an image holds past its chain's (D9).
pub(crate) const IMAGE_KNOTS: usize = 4096;

/// How many of the seed grid's nearest points [`nearest`] settles a foot
/// from (D9).
const START_SEEDS: usize = 8;

/// How many halvings place a path's crossing of a wall's knot line
/// ([`crossing`]): to `f64`'s resolution of the unit interval (D9).
const CROSSING_HALVINGS: u32 = 60;

/// How near a chain's knot, on the unit interval, a break of the path
/// is that knot's ([`crossing`]).
const CREASE_SNAP: f64 = 1e-13;

/// Why a carrier has no image on a wall.
#[derive(Debug)]
pub(crate) enum ImageRefusal {
    /// The wall is the placeholder: nothing to project onto.
    PlaceholderWall,
    /// The carrier's interval is not one the knot door accepts.
    CarrierDomain(CarrierDomainRefusal),
    /// The foot path could not be followed to the carrier's parameter
    /// `t`: the projection would not converge there, or left the sheet
    /// it was on at every halving.
    Foot {
        /// The parameter the path stopped short of.
        t: f64,
        /// The chain's foot it stopped short of.
        node: u32,
        /// `|S − P|` at the last iterate (NaN when poisoned or never
        /// read).
        last_distance: f64,
    },
    /// The interpolation through the feet refused.
    Fit,
    /// The image still missed the carrier's foot path at the cap. The
    /// refusal is in limb 2's words
    /// ([`crate::ssi::certify::image_miss`]) and carries the largest
    /// miss the last round sampled midway between its feet: a sampled
    /// reading, not a bound on the image's miss, and not a verdict of
    /// the certificate's limb 2, which was never run.
    Miss(crate::ssi::SsiError),
}

/// **A lane's vocabulary for an image's refusal**: the five words every
/// reader of the image states, each handed all the refusal carries.
/// [`ImageRefusal::into_lane`] is the one conversion; a lane that drops
/// a datum does so here, where its word is written.
pub(crate) trait ImageLane: Sized {
    /// The wall is the placeholder.
    fn placeholder_wall() -> Self;
    /// The carrier's interval is not one the knot door accepts.
    fn carrier_domain(refusal: CarrierDomainRefusal) -> Self;
    /// The foot path stopped short of the carrier's parameter `t`, at
    /// the chain's foot `node`.
    fn foot(t: f64, node: u32, last_distance: f64) -> Self;
    /// The interpolation through the feet refused.
    fn fit() -> Self;
    /// The image missed past the cap: the sampled reading, in limb 2's
    /// words.
    fn miss(read: crate::ssi::SsiError) -> Self;
}

impl ImageRefusal {
    /// This refusal in a lane's vocabulary ([`ImageLane`]): the trace's,
    /// the mint's and rest's are all read through this one match.
    pub(crate) fn into_lane<E: ImageLane>(self) -> E {
        match self {
            Self::PlaceholderWall => E::placeholder_wall(),
            Self::CarrierDomain(refusal) => E::carrier_domain(refusal),
            Self::Foot {
                t,
                node,
                last_distance,
            } => E::foot(t, node, last_distance),
            Self::Fit => E::fit(),
            Self::Miss(read) => E::miss(read),
        }
    }
}

/// A carrier read for its image: the evaluator, the `f64` span and its
/// ends as the carrier holds them ([`crate::carrier_net::carrier_span`]).
struct Read<'a, T: geom_core::Real> {
    eval: &'a dyn Fn(T) -> Point3<T>,
    domain: (f64, f64),
    ends: (T, T),
}

impl<T: geom_core::Real> Read<'_, T> {
    /// The carrier's point at the unit fraction `at` of its span, the
    /// ends read where the carrier holds them.
    fn point(&self, at: f64) -> Point3<T> {
        let (t0, t1) = self.domain;
        (self.eval)(if at <= 0.0 {
            self.ends.0
        } else if at >= 1.0 {
            self.ends.1
        } else {
            T::from_computed(t0 + (t1 - t0) * at)
        })
    }
}

/// A foot of the path: the unit fraction of the carrier's span, the
/// wall's chart point there, and the carrier's distance from it.
#[derive(Clone, Copy, Debug)]
struct Foot {
    at: f64,
    uv: Point2<f64>,
    standoff: f64,
}

/// A settled foot: its chart point, the wall's point there, and the
/// carrier's distance from it.
type Settled<T> = (Point2<f64>, Point3<T>, f64);

/// How many ulps of a wall point's largest coordinate a length there is
/// resolved to ([`resolution`]): a foot whose tangential offset is under
/// it is settled ([`settle`]), and a miss under it asks no knot
/// ([`miss`]). A rounding charge, owed a re-posing
/// (`work/sectarms/the-wall-images-foot-resolution-counts-ulps-at-the-walls-point.md`):
/// the projection's own thresholds are absolute, and settle no foot that
/// stands off a wall hundreds of metres from the origin.
const RESOLUTION_ULPS: f64 = 64.0;

/// The length `f64` resolves at the wall point `p`
/// ([`RESOLUTION_ULPS`]).
fn resolution<T: Decide + Bounds>(p: Point3<T>) -> f64 {
    RESOLUTION_ULPS
        * f64::EPSILON
        * mid(p.x).abs().max(mid(p.y).abs()).max(mid(p.z).abs())
}

/// **The wall's foot of `p` from the seed `(u, v)`**: Newton's
/// ([`NurbsSurface::project_from_seed`]). Where Newton's own acceptance
/// is not met, its last iterate stands as the foot if both its
/// tangential offsets from the nearest point, `|S_u · r| / |S_u|` and
/// `v`'s, are within [`RESOLUTION_ULPS`] ulps of the wall point's
/// largest coordinate.
///
/// # Errors
///
/// The last distance read, where the last iterate is no such foot.
fn settle<T>(wall: &NurbsSurface<T>, p: Point3<T>, (u, v): (f64, f64)) -> Result<Settled<T>, f64>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    match wall.project_from_seed(p, u, v) {
        Ok(proj) => Ok((Point2::new(proj.u, proj.v), proj.foot, mid(proj.distance))),
        Err(e) => {
            let jet = wall.ders(T::from_computed(e.last_u), T::from_computed(e.last_v));
            let floor = resolution(jet.point);
            let (su, sv) = (mid3(jet.du).norm(), mid3(jet.dv).norm());
            let settled = e.last_orthogonality_u <= floor * su
                && e.last_orthogonality_v <= floor * sv
                && e.last_distance.is_finite();
            if settled {
                Ok((Point2::new(e.last_u, e.last_v), jet.point, e.last_distance))
            } else {
                Err(e.last_distance)
            }
        }
    }
}

fn mid<T: Decide + Bounds>(x: T) -> f64 {
    geom::mid_param(x.lo(), x.hi())
}

fn mid3<T: Decide + Bounds>(v: Vec3<T>) -> Vec3<f64> {
    Vec3::new(mid(v.x), mid(v.y), mid(v.z))
}

/// **The image of `carrier` on `wall`** (module docs), on the carrier's
/// certificate span ([`crate::carrier_net::carrier_span`]).
///
/// # Errors
///
/// [`ImageRefusal`].
pub(crate) fn carrier_image<T>(
    carrier: &Curve3<T>,
    params: (T, T),
    wall: &NurbsSurface<T>,
    band: Band,
) -> Result<NurbsCurve2<T>, ImageRefusal>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    let (image, (t0, t1)) = on_unit(carrier, params, wall, band)?;
    crate::edge_nurbs::on_carrier_domain(&image, t0, t1).map_err(|e| domain_refusal(t0, t1, &e))
}

/// **The images of `carrier` on two walls**, each [`carrier_image`]'s
/// and then written on the union of the two's knots (evaluation-invariant
/// in ℝ), so the two-chart tube reads them span for span.
///
/// # Errors
///
/// [`ImageRefusal`], the first wall's first.
pub(crate) fn carrier_images<T>(
    carrier: &Curve3<T>,
    params: (T, T),
    (a, b): (&NurbsSurface<T>, &NurbsSurface<T>),
    band: Band,
) -> Result<(NurbsCurve2<T>, NurbsCurve2<T>), ImageRefusal>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    let (pa, (t0, t1)) = on_unit(carrier, params, a, band)?;
    let (pb, _) = on_unit(carrier, params, b, band)?;
    let both = NurbsCurve2::refine_to_union([&pa, &pb]).map_err(|_| ImageRefusal::Fit)?;
    let [pa, pb] = <[_; 2]>::try_from(both).map_err(|_| ImageRefusal::Fit)?;
    let lift = |p| {
        crate::edge_nurbs::on_carrier_domain(&p, t0, t1).map_err(|e| domain_refusal(t0, t1, &e))
    };
    Ok((lift(pa)?, lift(pb)?))
}

fn domain_refusal(t0: f64, t1: f64, e: &geom_core::spline::SplineError) -> ImageRefusal {
    ImageRefusal::CarrierDomain(CarrierDomainRefusal::of(t0, t1, e))
}

/// The image on the unit interval, in `f64`, and the carrier's span it
/// is re-expressed on.
fn on_unit<T>(
    carrier: &Curve3<T>,
    params: (T, T),
    wall: &NurbsSurface<T>,
    band: Band,
) -> Result<(NurbsCurve2<f64>, (f64, f64)), ImageRefusal>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    if wall.is_placeholder() {
        return Err(ImageRefusal::PlaceholderWall);
    }
    let (domain, ends) = crate::carrier_net::carrier_span(carrier, params);
    let (t0, t1) = domain;
    // The domain door, asked before any foot: a width that overflows
    // would otherwise reach the path as NaN parameters and refuse as a
    // foot, hiding the carrier's own fault behind the projection's.
    KnotVector::unit_segment(core::num::NonZeroUsize::MIN)
        .on_domain(t0, t1)
        .map_err(|e| domain_refusal(t0, t1, &e))?;
    let eval = |t| carrier.eval(t);
    let read = Read {
        eval: &eval,
        domain,
        ends,
    };
    let degree = chain_degree(carrier);
    let mut chain = chain(carrier, params, domain).ok_or(ImageRefusal::Fit)?;
    let chain_knots = chain.len();
    let tol = IMAGE_MISS * band.zero();
    // The first foot, from the chain's own first step.
    let lines = knot_lines(wall, degree);
    let first = start(
        &read,
        wall,
        (0.0, greville(&chain, degree).get(1).copied()),
        band,
    )?;
    // The crossings of the wall's knot lines the chain holds a knot at:
    // the unit fraction, the line's axis and its value.
    let mut on_line: Vec<(f64, usize, f64)> = Vec::new();
    for round in 0..=IMAGE_ROUNDS {
        let knots = KnotVector::clamped(chain.clone(), degree).map_err(|_| ImageRefusal::Fit)?;
        // The chain on the carrier's own span, before any foot is read
        // at its parameters: knots that collide there are the span's
        // fault, not the path's.
        let n = knots.control_count();
        NurbsCurve2::new(knots.clone(), vec![Point2::new(0.0, 0.0); n], vec![1.0; n])
            .and_then(|flat| flat.on_domain(t0, t1))
            .map_err(|e| domain_refusal(t0, t1, &e))?;
        let nodes = greville(&chain, degree);
        let mut feet = path(&read, wall, &nodes, first)?;
        // A foot at a crossing of a wall's knot line lies on the line:
        // the spans either side of it each end in their own cell.
        for foot in &mut feet {
            if let Some(&(_, axis, k)) = on_line.iter().find(|(at, ..)| *at == foot.at) {
                foot.uv = if axis == 0 {
                    Point2::new(k, foot.uv.y)
                } else {
                    Point2::new(foot.uv.x, k)
                };
            }
        }
        let collocation = geom::Collocation::on(&nodes, knots).map_err(|_| ImageRefusal::Fit)?;
        let uv: Vec<Point2<f64>> = feet.iter().map(|f| f.uv).collect();
        let image =
            NurbsCurve2::<f64>::interpolate_on(&uv, &collocation).map_err(|_| ImageRefusal::Fit)?;
        // The image lies in the wall's rectangle: a control point the
        // interpolation puts past a side is put on it. Its feet lie
        // inside, so where its net does it is inside whole, and what the
        // move costs is read with the rest of its miss below.
        let [ud, vd] = rectangle(wall);
        let image =
            image.map_points(|p| Point2::new(p.x.clamp(ud.0, ud.1), p.y.clamp(vd.0, vd.1)));
        // The spans that miss, by the knot each takes (at its middle, or
        // on a line of the wall the path crosses in it), and the largest
        // miss read.
        let (mut add, mut creases, mut worst) = (Vec::new(), Vec::new(), 0.0_f64);
        for (i, pair) in feet.windows(2).enumerate() {
            let at = 0.5 * (pair[0].at + pair[1].at);
            #[allow(clippy::cast_possible_truncation)]
            let (miss, standoff) = miss(&read, wall, (at, i as u32 + 1), image.eval(at))?;
            // Where the carrier itself stands off the wall, the image is
            // held to a share of that standoff: a finer image changes
            // nothing a certificate reads of such a carrier, and the
            // path of one that ends on a side of the rectangle bends
            // there by a share of it.
            let tol = tol.max(IMAGE_STANDOFF_SHARE * standoff);
            if miss <= tol {
                continue;
            }
            worst = worst.max(miss);
            match crossing(&read, wall, &lines, (pair[0], pair[1])) {
                Some(crease) => creases.push(crease),
                None => add.extend(span_middle(&chain, at)),
            }
        }
        if worst == 0.0 {
            return Ok((image, domain));
        }
        add.sort_by(f64::total_cmp);
        add.dedup();
        // A crease's knot at the image's degree, a C⁰ break whatever the
        // wall's continuity across the line (`KnotLine`): the knot's own
        // control point is what stands on the line. Less what the chain
        // holds of it.
        for (k, multiplicity, axis, line) in creases {
            // A break found again beside the knot it already has is that
            // knot's: two knots a rounding apart solve no collocation.
            let k = chain
                .iter()
                .copied()
                .find(|x| (x - k).abs() <= CREASE_SNAP)
                .unwrap_or(k);
            if !(k > 0.0 && k < 1.0) {
                continue;
            }
            add.retain(|x| (x - k).abs() > CREASE_SNAP);
            on_line.push((k, axis, line));
            let held =
                chain.iter().filter(|x| **x == k).count() + add.iter().filter(|x| **x == k).count();
            add.extend(core::iter::repeat_n(k, multiplicity.saturating_sub(held)));
        }
        if add.is_empty()
            || round == IMAGE_ROUNDS
            || chain.len() + add.len() > chain_knots + IMAGE_KNOTS
        {
            // At the cap the image is refused whatever the miss reads, in
            // limb 2's words, on the last round's sampled miss.
            return Err(ImageRefusal::Miss(crate::ssi::certify::image_miss::<T>(
                worst, band,
            )));
        }
        chain.extend(add);
        chain.sort_by(f64::total_cmp);
    }
    unreachable!("the last round returns")
}

/// A line of the wall's chart across which the wall is less smooth than
/// the image: the chart axis it is constant on (`0` for `u`), its value,
/// and the multiplicity a knot of the image takes there, its degree: a
/// break, so a foot of the path stands on the line itself and each span
/// beside it lies in one cell of the wall.
type KnotLine = (usize, f64, usize);

/// The wall's interior knot lines whose continuity, `C^(p−m)` at a knot
/// of multiplicity `m` in a degree-`p` direction, is below the image's
/// `C^(degree−1)`: the foot path is only as smooth as the wall across
/// one, and an image smoother than that misses it by its kink.
fn knot_lines<T: Decide + Bounds>(wall: &NurbsSurface<T>, degree: usize) -> Vec<KnotLine> {
    [wall.knot_values_u(), wall.knot_values_v()]
        .into_iter()
        .enumerate()
        .flat_map(|(axis, knots)| {
            let p = knots.degree();
            interior(knots).into_iter().filter_map(move |(k, m)| {
                let continuity = p.saturating_sub(m);
                (continuity + 1 < degree).then_some((axis, k, degree))
            })
        })
        .collect()
}

/// A held knot vector's interior runs, each value at its bracket's
/// middle with its multiplicity: structure the image's chain and breaks
/// are placed by.
fn interior<T: Decide + Bounds>(knots: &geom_core::spline::Knots<T>) -> Vec<(f64, usize)> {
    let runs: Vec<(f64, usize)> = knots.runs().map(|(k, m)| (mid(k), m)).collect();
    runs.get(1..runs.len().saturating_sub(1))
        .unwrap_or(&[])
        .to_vec()
}

/// The wall's knot rectangle, `u`'s interval first, each end at its
/// bracket's middle.
fn rectangle<T: Decide + Bounds>(wall: &NurbsSurface<T>) -> [(f64, f64); 2] {
    [wall.knot_values_u(), wall.knot_values_v()].map(|knots| {
        let (lo, hi) = knots.domain();
        (mid(lo), mid(hi))
    })
}

/// **Where the path crosses a line of the wall between two feet**: the
/// unit fraction of the crossing, found by halving the way between the
/// feet [`CROSSING_HALVINGS`] times with the path followed to each
/// middle, with the line's multiplicity, axis and value. `None` where the feet lie on
/// one side of every line, or a middle's foot is not reached. Of two
/// lines crossed, the first listed.
fn crossing<T>(
    read: &Read<'_, T>,
    wall: &NurbsSurface<T>,
    lines: &[KnotLine],
    (a, b): (Foot, Foot),
) -> Option<(f64, usize, usize, f64)>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    let along = |f: &Foot, axis: usize| if axis == 0 { f.uv.x } else { f.uv.y };
    let &(axis, k, multiplicity) = lines
        .iter()
        .find(|(axis, k, _)| (along(&a, *axis) - k) * (along(&b, *axis) - k) < 0.0)?;
    let start = along(&a, axis) > k;
    let (mut near, mut far) = (a, b);
    for _ in 0..CROSSING_HALVINGS {
        let at = 0.5 * (near.at + far.at);
        if !(near.at < at && at < far.at) {
            break;
        }
        let foot = follow(read, wall, near, at).ok()?;
        if (along(&foot, axis) > k) == start {
            near = foot;
        } else {
            far = foot;
        }
    }
    Some((far.at, multiplicity, axis, k))
}

/// The degree a carrier is imaged at.
fn chain_degree<T: geom_core::Real>(carrier: &Curve3<T>) -> usize {
    match carrier {
        Curve3::Nurbs(net) => net.degree().max(IMAGE_DEGREE),
        // A periodic carrier is a cubic, as its clamped view is.
        _ => IMAGE_DEGREE,
    }
}

/// The chain's clamped knots on the unit interval: a spline's own (a
/// periodic spline's clamped view's over the span), each interior knot
/// at the multiplicity that keeps its continuity at the image's degree,
/// and a closed form's uniform spans. `None` where a spline's domain has
/// no width or a periodic one has no view over the span.
fn chain<T>(carrier: &Curve3<T>, params: (T, T), (t0, t1): (f64, f64)) -> Option<Vec<f64>>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    let degree = chain_degree(carrier);
    let mut knots = vec![0.0; degree + 1];
    // A spline's net over the span: a clamped spline's own, a periodic
    // one's clamped view, which the edge certificate reads too.
    match carrier.clamped_net(params) {
        Some(net) => {
            let net = net.ok()?;
            let width = t1 - t0;
            if width.is_nan() || width <= 0.0 {
                return None;
            }
            let raise = degree.checked_sub(net.degree())?;
            for (k, m) in interior(net.knot_values()) {
                let at = (k - t0) / width;
                if at > 0.0 && at < 1.0 {
                    knots.extend(core::iter::repeat_n(at, (m + raise).min(degree)));
                }
            }
        }
        None => {
            #[allow(clippy::cast_precision_loss)]
            knots.extend((1..CLOSED_FORM_SPANS).map(|i| i as f64 / CLOSED_FORM_SPANS as f64));
        }
    }
    knots.extend(core::iter::repeat_n(1.0, degree + 1));
    Some(knots)
}

/// The Greville abscissae of a clamped knot list: one per control point,
/// the first and last the domain's ends exactly, and a knot of full
/// multiplicity exactly itself.
fn greville(knots: &[f64], degree: usize) -> Vec<f64> {
    let n = knots.len() - degree - 1;
    (0..n)
        .map(|j| {
            if j == 0 {
                return 0.0;
            }
            if j == n - 1 {
                return 1.0;
            }
            let window = &knots[j + 1..=j + degree];
            // A knot of full multiplicity is its own abscissa, to the
            // bit: the mean of equal doubles need not be that double.
            if window.iter().all(|k| *k == window[0]) {
                return window[0];
            }
            #[allow(clippy::cast_precision_loss)]
            let at = window.iter().sum::<f64>() / degree as f64;
            at
        })
        .collect()
}

/// The middle of the chain's knot span holding `at`, where that span is
/// wide enough to hold a knot strictly inside it.
fn span_middle(knots: &[f64], at: f64) -> Option<f64> {
    let hi = knots.partition_point(|k| *k <= at).min(knots.len() - 1);
    let (a, b) = (knots[hi.checked_sub(1)?], knots[hi]);
    let k = 0.5 * (a + b);
    (a < k && k < b).then_some(k)
}

/// **The image's miss of the foot path** at the unit fraction `at`,
/// short of the chain's foot `node`, where the image reads the chart
/// point `p`: the distance between the
/// wall's point at `p` and its point at the carrier's foot there, which
/// Newton reaches from `p`; zero where that is under [`resolution`].
/// Poison reads `+∞`. Beside it, the carrier's distance from that foot.
///
/// # Errors
///
/// [`ImageRefusal::Foot`] where that foot does not converge.
fn miss<T>(
    read: &Read<'_, T>,
    wall: &NurbsSurface<T>,
    (at, node): (f64, u32),
    p: Point2<f64>,
) -> Result<(f64, f64), ImageRefusal>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    let (_, foot, standoff) =
        settle(wall, read.point(at), (p.x, p.y)).map_err(|last_distance| {
            let (t0, t1) = read.domain;
            ImageRefusal::Foot {
                t: t0 + (t1 - t0) * at,
                node,
                last_distance,
            }
        })?;
    let here = wall.eval(T::from_computed(p.x), T::from_computed(p.y));
    let d = mid((here - foot).norm());
    if d.is_nan() {
        return Ok((f64::INFINITY, standoff));
    }
    // A miss the wall's own evaluation does not resolve is none.
    Ok((if d <= resolution(here) { 0.0 } else { d }, standoff))
}

/// The foot path at `nodes`, by continuation from its first foot.
fn path<T>(
    read: &Read<'_, T>,
    wall: &NurbsSurface<T>,
    nodes: &[f64],
    first: Foot,
) -> Result<Vec<Foot>, ImageRefusal>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    let mut feet = Vec::with_capacity(nodes.len());
    let Some((&at0, rest)) = nodes.split_first() else {
        return Err(ImageRefusal::Fit);
    };
    let mut last = Foot { at: at0, ..first };
    feet.push(last);
    for (i, &at) in rest.iter().enumerate() {
        #[allow(clippy::cast_possible_truncation)]
        let node = i as u32 + 1;
        last = follow(read, wall, last, at).map_err(|last_distance| {
            let (t0, t1) = read.domain;
            ImageRefusal::Foot {
                t: t0 + (t1 - t0) * at,
                node,
                last_distance,
            }
        })?;
        feet.push(last);
    }
    Ok(feet)
}

/// A settled foot ([`settle`]) as a foot of the path.
fn settled<T: geom_core::Real>(at: f64, (uv, _, standoff): Settled<T>) -> Foot {
    Foot { at, uv, standoff }
}

/// **The wall's nearest point to `p`**, over the whole wall: Newton's
/// foot from each of the [`START_SEEDS`] nearest points of the
/// projection's own seed grid (`geom::PROJECT_SEEDS_PER_SPAN` a side of
/// every span cell, scanned in ascending order), the nearest of them
/// kept and the first of equals. One seed's foot can be a far sheet's
/// where a wall folds back within a grid step of itself; the carrier's
/// start lies on the wall, so its own sheet's foot is the nearer.
///
/// # Errors
///
/// The last distance read, where no seed settles.
fn nearest<T>(wall: &NurbsSurface<T>, p: Point3<T>) -> Result<Settled<T>, f64>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    let breaks = [wall.knot_values_u(), wall.knot_values_v()]
        .map(|knots| knots.breakpoints().map(mid).collect::<Vec<f64>>());
    let n = geom::PROJECT_SEEDS_PER_SPAN;
    #[allow(clippy::cast_precision_loss)]
    let at = |a: f64, b: f64, i: usize| a + (b - a) * (i as f64 / (n - 1) as f64);
    let mut seeds: Vec<(f64, f64, f64)> = Vec::new();
    for su in breaks[0].windows(2) {
        for sv in breaks[1].windows(2) {
            for i in 0..n {
                for j in 0..n {
                    let (u, v) = (at(su[0], su[1], i), at(sv[0], sv[1], j));
                    let d = wall.eval(T::from_computed(u), T::from_computed(v)) - p;
                    let d2 = mid(d.dot(d));
                    if !d2.is_nan() {
                        seeds.push((d2, u, v));
                    }
                }
            }
        }
    }
    // A stable sort: equal distances keep the scan's order.
    seeds.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut best: Result<Settled<T>, f64> = Err(f64::NAN);
    for &(_, u, v) in seeds.iter().take(START_SEEDS) {
        match (settle(wall, p, (u, v)), &best) {
            (Ok(foot), Ok(held)) if foot.2 < held.2 => best = Ok(foot),
            (Ok(foot), Err(_)) => best = Ok(foot),
            (Err(d), Err(_)) => best = Err(d),
            _ => {}
        }
    }
    best
}

/// Whether the wall's points at two chart points are one, within the
/// band: the two sides of a seam the wall closes across.
fn closes<T>(wall: &NurbsSurface<T>, a: Point2<f64>, b: Point2<f64>, band: Band) -> bool
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    let at = |p: Point2<f64>| wall.eval(T::from_computed(p.x), T::from_computed(p.y));
    mid((at(a) - at(b)).norm()) <= band.zero()
}

/// **The path's first foot**: the wall's nearest point to the carrier's
/// start ([`nearest`]). Where that lies on a side of the rectangle the wall closes
/// across, the same point of the wall is on the far side too: the nearer
/// of the two is taken, and of two as near as the band tells apart, the
/// one the path's first step leaves into the rectangle from.
fn start<T>(
    read: &Read<'_, T>,
    wall: &NurbsSurface<T>,
    (at, next): (f64, Option<f64>),
    band: Band,
) -> Result<Foot, ImageRefusal>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    let p = read.point(at);
    let refused = |last_distance| {
        let (t0, t1) = read.domain;
        ImageRefusal::Foot {
            t: t0 + (t1 - t0) * at,
            node: 0,
            last_distance,
        }
    };
    let mut best = nearest(wall, p)
        .map(|foot| settled(at, foot))
        .map_err(refused)?;
    for (axis, (lo, hi)) in rectangle(wall).into_iter().enumerate() {
        let here = if axis == 0 { best.uv.x } else { best.uv.y };
        let far = if here == lo {
            hi
        } else if here == hi {
            lo
        } else {
            continue;
        };
        let across = if axis == 0 {
            Point2::new(far, best.uv.y)
        } else {
            Point2::new(best.uv.x, far)
        };
        if !closes(wall, best.uv, across, band) {
            continue;
        }
        let Ok(other) = settle(wall, p, (across.x, across.y)) else {
            continue;
        };
        let other = settled(at, other);
        let tied = (other.standoff - best.standoff).abs() <= band.zero();
        let leaves = |f: &Foot| {
            // Whether the path's first step, as the wall's tangents at
            // `f` predict it, leaves the rectangle across this side.
            let Some(next) = next else { return false };
            let Some(step) = predicted(wall, f.uv, mid3(read.point(next) - p)) else {
                return false;
            };
            let (x, d) = if axis == 0 {
                (f.uv.x, step.0)
            } else {
                (f.uv.y, step.1)
            };
            (x == lo && d < 0.0) || (x == hi && d > 0.0)
        };
        if (tied && leaves(&best) && !leaves(&other)) || (!tied && other.standoff < best.standoff) {
            best = other;
        }
    }
    Ok(best)
}

/// The chart step the wall's tangents at `uv` predict for the carrier's
/// step `dc`: its least-squares pull-back. `None` where the tangents do
/// not span a plane there (a collapsed row, a pole).
fn predicted<T>(wall: &NurbsSurface<T>, uv: Point2<f64>, dc: Vec3<f64>) -> Option<(f64, f64)>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    let jet = wall.ders(T::from_computed(uv.x), T::from_computed(uv.y));
    let (su, sv) = (mid3(jet.du), mid3(jet.dv));
    let (a, b, d) = (su.dot(su), su.dot(sv), sv.dot(sv));
    let det = a * d - b.powi(2);
    if det.is_nan() || det <= 1e-12 * a * d {
        return None;
    }
    let (fu, fv) = (su.dot(dc), sv.dot(dc));
    Some(((d * fu - b * fv) / det, (a * fv - b * fu) / det))
}

/// **The next foot of the path**, at the unit fraction `at`: Newton's
/// from the last foot's chart point, so it is the last one's own
/// sheet's and side's, not the wall's nearest point found afresh.
///
/// # Errors
///
/// The last distance the projection read, where it does not settle.
fn follow<T>(
    read: &Read<'_, T>,
    wall: &NurbsSurface<T>,
    last: Foot,
    at: f64,
) -> Result<Foot, f64>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    settle(wall, read.point(at), (last.uv.x, last.uv.y)).map(|foot| settled(at, foot))
}
