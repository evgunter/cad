//! **The printed carrier on its built one** (`docs/DESIGN.md` `[import]`
//! step 2): the importer verifies that the curve the file printed lies
//! within ε_in of the edge's description, then rebuilds from the
//! description. The build ([`crate::build`]) constructs each edge's
//! carrier from its description; this module answers the verification as
//! one number, a CERTIFIED upper bound in metres on the distance of every
//! point of the printed carrier over its trim from the built carrier's
//! locus, read at the file's band by the one decision
//! `step_edge_on_description`.
//!
//! The certificate reads the locus alone: what the printed carrier is
//! evidence of, never its parameter or its kind's choice of arm. Each arm
//! is a whole-trim inequality, no sampling:
//!
//! - **a printed spline on a built line or circle**: the ring composites
//!   of the curve recognizer ([`crate::recognize_curve`], INV-C3 for the
//!   line, INV-C1 and INV-C2 for the circle) built over the printed
//!   spline and cut at its trim's ends
//!   ([`crate::recognize_curve::composite_sup_within`]), so the sup is
//!   over exactly the spans the edge uses. The built carrier's frame is
//!   read as exact, as the recognizer reads its own estimate
//!   (`docs/DESIGN.md` `[simple.conditioned]`);
//! - **a printed spline on the identical spline** (a wall's column the
//!   file repeats bit for bit): zero;
//! - **a printed line on a built line**: the farther of the printed
//!   trim's two ends from the built line. The distance from a line is
//!   convex along a segment, so the ends bound every point between them;
//! - **a printed conic (circle or ellipse) on a built conic**: each
//!   carrier is `c + cos θ·A + sin θ·B` for a centre `c` and conjugate
//!   semi-diameters `A`, `B`, and `θ ↦ Q(θ)` names a point of the built
//!   conic for every θ under each of its own reparameterizations chosen
//!   below. So `|P(θ) − Q(θ)| ≤ |c₁ − c₂| + |A₁ − A₂| + |B₁ − B₂|` bounds
//!   the distance of every printed point from the built conic, over the
//!   whole printed conic and so over its trim. The built circle's `A₂`
//!   is the printed seam direction projected into its plane, at the
//!   built radius; a built ellipse's is the best of its eight principal
//!   pairings `(±a·u, ±b·v)`, `(±b·v, ±a·u)`. The frames' `f64` closed
//!   forms are read as exact; the sums are outward.
//!
//! Every other pair (a printed spline on a built spline that is not its
//! twin bit for bit, a spline on an ellipse or a spiric, a curve of one
//! conic kind on a line) has no certificate here, and [`printed_on`]
//! answers `None`: the build then reads the printed carrier's ends and
//! middle (`work/exch/the-printed-on-built-certificate-has-no-arm-for-a-built-spline.md`).

use geom::{Curve3, NurbsCurve3};
use geom_core::interval::certification::Certification as _;
use geom_core::interval::{cross, max_bound, norm_sup};
use geom_core::spline::compose::{CurveCertData, linear_composite};
use geom_core::{Bounds, Interval, Point3, Vec3};

use crate::entities::EdgeSpec;
use crate::recognize_curve::circle_locus_within;

/// **The certified sup of the printed carrier's distance from `built`
/// over the printed trim** (module docs); `None` where no arm serves the
/// pair, or the arm certifies no finite bound.
pub(crate) fn printed_on(spec: &EdgeSpec, built: &Curve3<f64>) -> Option<f64> {
    let trim = (spec.t0, spec.t1);
    let bound = match (&spec.carrier, built) {
        (Curve3::Nurbs(printed), Curve3::Line(line)) => hull_within(printed, trim)?
            .into_iter()
            .map(|p| line_distance_sup(p, line.origin, line.dir))
            .fold(0.0_f64, max_bound),
        (Curve3::Nurbs(printed), Curve3::Circle(circle)) => circle_locus_within(
            printed,
            (circle.frame.origin, circle.frame.axis, circle.radius()),
            trim,
        ),
        (Curve3::Nurbs(printed), Curve3::Nurbs(built)) => {
            if bitwise_twin(printed, built) {
                0.0
            } else {
                return None;
            }
        }
        (Curve3::Line(_), Curve3::Line(line)) => trim_ends(spec)
            .into_iter()
            .map(|p| line_distance_sup(p, line.origin, line.dir))
            .fold(0.0_f64, max_bound),
        (printed, built) => {
            let (Some(printed), Some(built)) = (Conic::of(printed), Conic::of(built)) else {
                return None;
            };
            built.bound_from(&printed)?
        }
    };
    bound.is_finite().then_some(bound)
}

/// Whether two splines are one bit for bit: knots, controls and weights.
pub(crate) fn bitwise_twin(a: &NurbsCurve3<f64>, b: &NurbsCurve3<f64>) -> bool {
    let bits3 = |p: &Point3<f64>| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
    let (ka, kb) = (a.knots_f64_interim(), b.knots_f64_interim());
    ka.degree() == kb.degree()
        && ka.knots().len() == kb.knots().len()
        && ka
            .knots()
            .iter()
            .zip(kb.knots())
            .all(|(x, y)| x.to_bits() == y.to_bits())
        && a.control().len() == b.control().len()
        && a.control()
            .iter()
            .zip(b.control())
            .all(|(x, y)| bits3(x) == bits3(y))
        && a.weights().len() == b.weights().len()
        && a.weights()
            .iter()
            .zip(b.weights())
            .all(|(x, y)| x.to_bits() == y.to_bits())
}

/// **The printed carrier's distance from the segment `[a, b]` over its
/// trim**, certified: every point of a printed line's trim or a printed
/// spline's lies in the hull of [`trim_ends`] or [`hull_within`], and the
/// distance from a segment is convex, so the hull's farthest point bounds
/// it. `None` for any other printed carrier, or where no bound certifies.
pub(crate) fn printed_on_segment(spec: &EdgeSpec, ends: (Point3<f64>, Point3<f64>)) -> Option<f64> {
    let hull = match &spec.carrier {
        Curve3::Line(_) => trim_ends(spec).to_vec(),
        Curve3::Nurbs(printed) => hull_within(printed, (spec.t0, spec.t1))?,
        _ => return None,
    };
    let bound = hull
        .into_iter()
        .map(|p| segment_distance_sup(p, (pt(ends.0), pt(ends.1))))
        .fold(0.0_f64, max_bound);
    bound.is_finite().then_some(bound)
}

/// A printed line's trim ends, as exact enclosures.
fn trim_ends(spec: &EdgeSpec) -> [[Interval; 3]; 2] {
    [spec.t0, spec.t1].map(|t| pt(spec.carrier.eval(t)))
}

/// **The rational Bézier control points of `curve`'s spans inside
/// `trim`**, as enclosures: the curve cut at the trim's ends
/// ([`CurveCertData::with_breaks`], exact in ℝ, no control re-rounded),
/// each span's weighted coordinate rows over its weight row
/// ([`linear_composite`] of each coordinate). With every weight row
/// certified positive the span is a convex combination of these points,
/// so the part of the curve inside `trim` lies in their hull. `None`
/// where a weight is not certified positive, a composite refuses, or no
/// span lies inside `trim`.
fn hull_within(curve: &NurbsCurve3<f64>, (t0, t1): (f64, f64)) -> Option<Vec<[Interval; 3]>> {
    let coords = curve.certified_coords();
    let cuts = [t0, t1];
    let data = CurveCertData::new(curve.knots_f64_interim(), curve.weights(), &coords)
        .ok()?
        .with_breaks(&cuts);
    let axis = |d: usize| {
        let mut e = [0.0; 3];
        e[d] = 1.0;
        linear_composite(&data, &e, 0.0).ok()
    };
    let (x, y, z) = (axis(0)?, axis(1)?, axis(2)?);
    let at = x.num.breaks();
    let weights = x.den.spans();
    let mut hull = Vec::new();
    // Fixed ascending span and coefficient order (D9).
    for (i, w) in weights.iter().enumerate() {
        let (&lo, &hi) = (at.get(i)?, at.get(i + 1)?);
        if !(lo >= t0 && hi <= t1) {
            continue;
        }
        for (k, &wk) in w.iter().enumerate() {
            if !(wk.lo() > 0.0) {
                return None;
            }
            let row = |c: &geom_core::spline::compose::CompositeForm| {
                c.num.spans().get(i).and_then(|r| r.get(k)).map(|&n| n / wk)
            };
            hull.push([row(&x)?, row(&y)?, row(&z)?]);
        }
    }
    (!hull.is_empty()).then_some(hull)
}

/// The componentwise enclosure of `p` (exact).
fn pt(p: Point3<f64>) -> [Interval; 3] {
    [p.x, p.y, p.z].map(Interval::point)
}

/// The componentwise enclosure of `v` (exact).
fn vc(v: Vec3<f64>) -> [Interval; 3] {
    [v.x, v.y, v.z].map(Interval::point)
}

/// `a − b`, componentwise and outward.
fn sub(a: [Interval; 3], b: [Interval; 3]) -> [Interval; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// A certified upper bound on the distance of the enclosed point `p`
/// from the line through `origin` along `dir` (any nonzero length):
/// `|(p − o) × d| / |d|`, outward. `NaN` where it certifies none.
fn line_distance_sup(p: [Interval; 3], origin: Point3<f64>, dir: Vec3<f64>) -> f64 {
    let d = vc(dir);
    let num = norm_sup(&cross(&sub(p, pt(origin)), &d));
    let den = geom_core::interval::norm_sq(&d).sqrt();
    if !(den.lo() > 0.0) {
        return f64::NAN;
    }
    (Interval::point(num) / den).hi()
}

/// A certified upper bound on the distance of the enclosed point `p` from
/// the segment `[a, b]`: its distance from the segment's line and how far
/// its projection runs past either end, `hypot` of the two, outward; from
/// `a` where the segment is not certified to have a length. `NaN` where
/// it certifies none.
pub(crate) fn segment_distance_sup(p: [Interval; 3], (a, b): ([Interval; 3], [Interval; 3])) -> f64 {
    let off = sub(p, a);
    let d = sub(b, a);
    let length = geom_core::interval::norm_sq(&d).sqrt();
    if !(length.lo() > 0.0) {
        return geom_core::interval::add_up(norm_sup(&off), norm_sup(&d));
    }
    let across = Interval::point(norm_sup(&cross(&off, &d))) / length;
    let along = geom_core::interval::dot(&off, &d) / length;
    let past = (Interval::zero() - along)
        .hi()
        .max((along - length).hi())
        .max(0.0);
    (across.sqr() + Interval::point(past).sqr()).sqrt().hi()
}

/// [`segment_distance_sup`] of an exact point.
pub(crate) fn point_segment_sup(p: Point3<f64>, (a, b): (Point3<f64>, Point3<f64>)) -> f64 {
    segment_distance_sup(pt(p), (pt(a), pt(b)))
}

/// **A conic as `c + cos θ·A + sin θ·B`** (module docs): its centre, its
/// two conjugate semi-diameters, and its plane normal.
struct Conic {
    center: Point3<f64>,
    a: Vec3<f64>,
    b: Vec3<f64>,
    normal: Vec3<f64>,
    /// A circle's radius, which any seam direction in its plane names;
    /// `None` for an ellipse, whose seam is a principal direction.
    radius: Option<f64>,
}

impl Conic {
    /// The circle's or ellipse's form on its own parameter; `None` for
    /// any other curve.
    fn of(curve: &Curve3<f64>) -> Option<Self> {
        match curve {
            Curve3::Circle(c) => {
                let (u, n, r) = (c.frame.u_ref, c.frame.axis, c.radius());
                Some(Self {
                    center: c.frame.origin,
                    a: u * r,
                    b: n.cross(u) * r,
                    normal: n,
                    radius: Some(r),
                })
            }
            Curve3::Ellipse(e) => {
                let (u, n) = (e.frame.u_ref, e.frame.axis);
                Some(Self {
                    center: e.frame.origin,
                    a: u * e.semi_u(),
                    b: n.cross(u) * e.semi_v(),
                    normal: n,
                    radius: None,
                })
            }
            _ => None,
        }
    }

    /// The bound of the module docs on how far `printed` stands from this
    /// (built) conic, over the least of this conic's reparameterizations
    /// read; `None` where none is read (a printed seam normal to a built
    /// circle's plane).
    fn bound_from(&self, printed: &Self) -> Option<f64> {
        let centre = norm_sup(&sub(pt(printed.center), pt(self.center)));
        let gap = |a: Vec3<f64>, b: Vec3<f64>| {
            norm_sup(&sub(vc(printed.a), vc(a))) + norm_sup(&sub(vc(printed.b), vc(b)))
        };
        let pairs: Vec<(Vec3<f64>, Vec3<f64>)> = match self.radius {
            Some(radius) => {
                let n = self.normal;
                let seam = printed.a - n * printed.a.dot(n);
                let (u, _) = seam.checked_direction().ok()?;
                let turn = if printed.normal.dot(n) < 0.0 {
                    -1.0
                } else {
                    1.0
                };
                vec![(u * radius, n.cross(u) * (radius * turn))]
            }
            None => {
                let (a, b) = (self.a, self.b);
                let mut pairs = Vec::with_capacity(8);
                for (x, y) in [(a, b), (b, a)] {
                    for sx in [1.0, -1.0] {
                        for sy in [1.0, -1.0] {
                            pairs.push((x * sx, y * sy));
                        }
                    }
                }
                pairs
            }
        };
        let least = pairs
            .into_iter()
            .map(|(a, b)| gap(a, b))
            .fold(f64::INFINITY, geom_core::interval::min_bound);
        let sum = Interval::point(centre) + Interval::point(least);
        Some(sum.hi())
    }
}
