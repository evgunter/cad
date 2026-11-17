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
//!   continuity. Any other carrier has no knots, and its chain is
//!   [`CLOSED_FORM_SPANS`] uniform spans of its interval.
//! - **The feet.** One at each Greville abscissa of the chain, so the
//!   interpolant is on the chain's knots exactly. They are found by
//!   **continuation**: the first is the wall's nearest point to the
//!   carrier's start, and each next one is Newton's from the last
//!   ([`follow`]), accepted where the chart step agrees with the step the
//!   wall's own tangents predict for the carrier's and otherwise reached
//!   through the halfway parameter. So the path stays on the sheet of a
//!   folded wall it started on, and reaches a closed wall's seam from the
//!   side it came by.
//! - **The start on a seam.** A start whose nearest point lies on a side
//!   of the wall's rectangle that the wall closes across has two chart
//!   points; it takes the one the path leaves into the rectangle from
//!   ([`start`]).
//! - **The miss.** Midway between each pair of feet the image is read
//!   against the carrier, net of the carrier's own standoff at the two
//!   feet ([`excess`]). A chain whose every reading is within
//!   [`IMAGE_MISS`] of the band's zero is the image. Otherwise each
//!   knot span holding a larger one takes a knot at its middle and the
//!   feet are followed again, at most [`IMAGE_ROUNDS`] times; past that
//!   the image is refused with the miss it read
//!   ([`ImageRefusal::Miss`]).
//!
//! Everything here is the `f64` structure lane (C6): the image is
//! evidence, and what makes it sound is that its consumer bounds
//! `sup_t |S(P(t)) − C(t)|` over the whole span.

use geom::{Curve3, NurbsCurve2, NurbsSurface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Bounds, Decide, Point2, Point3, Vec3};

use crate::edge_nurbs::CarrierDomainRefusal;

/// The image's least degree. A carrier of a higher degree is imaged at
/// its own.
pub(crate) const IMAGE_DEGREE: usize = 3;

/// The spans of a carrier that has no knots (a line, a conic, a periodic
/// spline over an interval of its loop): its chain before any miss asks
/// for more (D9).
pub(crate) const CLOSED_FORM_SPANS: usize = 8;

/// The share of `band.zero()` a chain's miss, read midway between its
/// feet net of the carrier's standoff, must fall within: the
/// certificate's hull reads the miss between those midpoints too, and
/// over-reads it.
pub(crate) const IMAGE_MISS: f64 = 1.0 / 16.0;

/// The most rounds of knot insertion (D9). Each halves the spans that
/// miss, so the last round's are `2⁻²⁴` of the chain's.
pub(crate) const IMAGE_ROUNDS: usize = 24;

/// The most knots an image holds past its chain's (D9).
pub(crate) const IMAGE_KNOTS: usize = 4096;

/// How many times [`follow`] halves the way to a foot it cannot reach
/// from the last one (D9).
const FOLLOW_HALVINGS: u32 = 16;

/// The share of the carrier's step by which a foot's chart step, in
/// metres through the wall's tangents, may differ from the predicted
/// one ([`follow`]).
const FOLLOW_AGREEMENT: f64 = 0.5;

/// Why a carrier has no image on a wall.
#[derive(Clone, Copy, Debug, PartialEq)]
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
    /// The image still missed the carrier's foot path past the cap.
    Miss {
        /// The largest miss the last chain read, in metres.
        miss: f64,
    },
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
/// wall's chart point there, and the carrier's standoff, the wall's
/// point less the carrier's.
#[derive(Clone, Copy, Debug)]
struct Foot {
    at: f64,
    uv: Point2<f64>,
    gap: Vec3<f64>,
}

fn mid<T: Bounds>(x: T) -> f64 {
    geom::mid_param(x.lo(), x.hi())
}

fn mid3<T: Bounds>(v: Vec3<T>) -> Vec3<f64> {
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
    let mut chain = chain(carrier, domain).ok_or(ImageRefusal::Fit)?;
    let first = chain.len();
    let tol = IMAGE_MISS * band.zero();
    for round in 0..=IMAGE_ROUNDS {
        let knots = KnotVector::clamped(chain.clone(), degree).map_err(|_| ImageRefusal::Fit)?;
        let nodes = greville(&chain, degree);
        let feet = path(&read, wall, &nodes, band)?;
        let collocation = geom::Collocation::on(&nodes, knots).map_err(|_| ImageRefusal::Fit)?;
        let uv: Vec<Point2<f64>> = feet.iter().map(|f| f.uv).collect();
        let image = NurbsCurve2::<f64>::interpolate_on(&uv, &collocation)
            .map_err(|_| ImageRefusal::Fit)?;
        // The spans that miss, by the knot at each one's middle, and the
        // largest miss read.
        let (mut add, mut worst) = (Vec::new(), 0.0_f64);
        for pair in feet.windows(2) {
            let at = 0.5 * (pair[0].at + pair[1].at);
            let miss = excess(&read, wall, pair, image.eval(at));
            worst = worst.max(miss);
            if miss > tol
                && let Some(k) = span_middle(&chain, at)
            {
                add.push(k);
            }
        }
        if worst <= tol {
            return Ok((image, domain));
        }
        add.sort_by(f64::total_cmp);
        add.dedup();
        if add.is_empty() || round == IMAGE_ROUNDS || chain.len() + add.len() > first + IMAGE_KNOTS
        {
            return Err(ImageRefusal::Miss { miss: worst });
        }
        chain.extend(add);
        chain.sort_by(f64::total_cmp);
    }
    unreachable!("the last round returns")
}

/// The degree a carrier is imaged at.
fn chain_degree<T: Bounds>(carrier: &Curve3<T>) -> usize {
    match carrier {
        Curve3::Nurbs(net) => net.degree().max(IMAGE_DEGREE),
        _ => IMAGE_DEGREE,
    }
}

/// The chain's clamped knots on the unit interval: a spline's own, each
/// interior knot at the multiplicity that keeps its continuity at the
/// image's degree, and any other carrier's uniform spans. `None` where a
/// spline's domain has no width.
fn chain<T: Bounds>(carrier: &Curve3<T>, (t0, t1): (f64, f64)) -> Option<Vec<f64>> {
    let degree = chain_degree(carrier);
    let mut knots = vec![0.0; degree + 1];
    match carrier {
        Curve3::Nurbs(net) => {
            let width = t1 - t0;
            if !(width > 0.0) {
                return None;
            }
            let raise = degree - net.degree();
            for (k, m) in net.knots_f64_interim().interior_knots() {
                let at = (k - t0) / width;
                if at > 0.0 && at < 1.0 {
                    knots.extend(core::iter::repeat_n(at, (m + raise).min(degree)));
                }
            }
        }
        _ => {
            #[allow(clippy::cast_precision_loss)]
            knots.extend((1..CLOSED_FORM_SPANS).map(|i| i as f64 / CLOSED_FORM_SPANS as f64));
        }
    }
    knots.extend(core::iter::repeat_n(1.0, degree + 1));
    Some(knots)
}

/// The Greville abscissae of a clamped knot list: one per control point,
/// the first and last the domain's ends exactly.
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
            #[allow(clippy::cast_precision_loss)]
            let at = knots[j + 1..=j + degree].iter().sum::<f64>() / degree as f64;
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

/// The image's miss midway between two feet, net of the standoff they
/// carry: its gap from the carrier at the middle, less the mean of the
/// feet's gaps, as vectors. A foot is the wall's nearest point, so its
/// gap is the carrier's offset from the wall, which no finer image
/// lowers; subtracting it as a vector leaves a miss across that offset
/// whole. Poison reads `+∞`.
fn excess<T>(read: &Read<'_, T>, wall: &NurbsSurface<T>, pair: &[Foot], p: Point2<f64>) -> f64
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    let at = 0.5 * (pair[0].at + pair[1].at);
    let gap = mid3(wall.eval(T::from_computed(p.x), T::from_computed(p.y)) - read.point(at));
    let d = (gap - (pair[0].gap + pair[1].gap) * 0.5).norm();
    if d.is_nan() { f64::INFINITY } else { d }
}

/// The foot path at `nodes`, by continuation from the start.
fn path<T>(
    read: &Read<'_, T>,
    wall: &NurbsSurface<T>,
    nodes: &[f64],
    band: Band,
) -> Result<Vec<Foot>, ImageRefusal>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    let mut feet = Vec::with_capacity(nodes.len());
    let Some((&at0, rest)) = nodes.split_first() else {
        return Err(ImageRefusal::Fit);
    };
    let mut last = start(read, wall, (at0, rest.first().copied()), band)?;
    feet.push(last);
    for (i, &at) in rest.iter().enumerate() {
        #[allow(clippy::cast_possible_truncation)]
        let node = i as u32 + 1;
        last = follow(read, wall, last, at, FOLLOW_HALVINGS).map_err(|last_distance| {
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

/// A converged projection as a foot.
fn foot_of<T>(at: f64, p: Point3<T>, proj: &geom::SurfaceProjection<T>) -> Foot
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    Foot {
        at,
        uv: Point2::new(proj.u, proj.v),
        gap: mid3(proj.foot - p),
    }
}

/// **The path's first foot**: the wall's nearest point to the carrier's
/// start. Where that lies on a side of the rectangle the wall closes
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
    let refused = |e: geom::SurfaceProjectionInconclusive| {
        let (t0, t1) = read.domain;
        ImageRefusal::Foot {
            t: t0 + (t1 - t0) * at,
            node: 0,
            last_distance: e.last_distance,
        }
    };
    let proj = wall.project(p).map_err(refused)?;
    let mut best = (foot_of(at, p, &proj), mid(proj.distance));
    let domains = [
        wall.knots_u_f64_interim().domain(),
        wall.knots_v_f64_interim().domain(),
    ];
    for (axis, (lo, hi)) in domains.into_iter().enumerate() {
        let here = if axis == 0 { best.0.uv.x } else { best.0.uv.y };
        let far = if here == lo {
            hi
        } else if here == hi {
            lo
        } else {
            continue;
        };
        let across = if axis == 0 {
            Point2::new(far, best.0.uv.y)
        } else {
            Point2::new(best.0.uv.x, far)
        };
        let closes = mid(
            (wall.eval(T::from_computed(across.x), T::from_computed(across.y))
                - wall.eval(T::from_computed(best.0.uv.x), T::from_computed(best.0.uv.y)))
            .norm(),
        ) <= band.zero();
        if !closes {
            continue;
        }
        let Ok(other) = wall.project_from_seed(p, across.x, across.y) else {
            continue;
        };
        let other = (foot_of(at, p, &other), mid(other.distance));
        let tied = (other.1 - best.1).abs() <= band.zero();
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
        if (tied && leaves(&best.0) && !leaves(&other.0)) || (!tied && other.1 < best.1) {
            best = other;
        }
    }
    Ok(best.0)
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
    let det = a * d - b * b;
    if !(det > 1e-12 * a * d) {
        return None;
    }
    let (fu, fv) = (su.dot(dc), sv.dot(dc));
    Some(((d * fu - b * fv) / det, (a * fv - b * fu) / det))
}

/// **The next foot of the path**, at the unit fraction `at`, from the
/// last one: Newton's from the predicted chart point, kept where the
/// chart step it took agrees with the predicted one to within
/// [`FOLLOW_AGREEMENT`] of the carrier's step, in metres through the
/// wall's tangents. A foot on another sheet of a folded wall, or across
/// a closed wall's seam, is a chart step the tangents do not predict; it
/// is refused and the path is followed through the halfway parameter
/// instead, `halvings` deep. Where the tangents predict nothing, the
/// foot Newton reaches from the last one stands.
///
/// # Errors
///
/// The last distance the projection read, where no halving reaches a
/// foot.
fn follow<T>(
    read: &Read<'_, T>,
    wall: &NurbsSurface<T>,
    last: Foot,
    at: f64,
    halvings: u32,
) -> Result<Foot, f64>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    let p = read.point(at);
    let dc = mid3(p - read.point(last.at));
    let step = predicted(wall, last.uv, dc);
    let seed = step.map_or(last.uv, |(du, dv)| Point2::new(last.uv.x + du, last.uv.y + dv));
    let direct = wall.project_from_seed(p, seed.x, seed.y);
    let last_distance = match &direct {
        Ok(proj) => {
            let agrees = step.is_none_or(|_| {
                // The seed as the projection clamped it, so a path along
                // a side of the rectangle is not read as leaving it.
                let (ud, vd) = (
                    wall.knots_u_f64_interim().domain(),
                    wall.knots_v_f64_interim().domain(),
                );
                let (eu, ev) = (
                    proj.u - seed.x.clamp(ud.0, ud.1),
                    proj.v - seed.y.clamp(vd.0, vd.1),
                );
                let jet = wall.ders(T::from_computed(proj.u), T::from_computed(proj.v));
                let off = (mid3(jet.du) * eu + mid3(jet.dv) * ev).norm();
                off <= FOLLOW_AGREEMENT * dc.norm()
            });
            if agrees {
                return Ok(foot_of(at, p, proj));
            }
            mid(proj.distance)
        }
        Err(e) => e.last_distance,
    };
    let half = 0.5 * (last.at + at);
    if halvings == 0 || !(last.at < half && half < at) {
        return Err(last_distance);
    }
    let through = follow(read, wall, last, half, halvings - 1)?;
    follow(read, wall, through, at, halvings - 1)
}
