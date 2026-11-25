//! **The root an edge's end owes a surface it ends on**, factored out,
//! and the window past it certified clear.
//!
//! An edge whose end lies on a curve where its own face and a face of
//! the other operand meet as a junction (both faces end on that curve)
//! meets the other face's carrier at that end with a root of order at
//! least the junction's contact. The root doors refuse there: a double
//! root reads as a graze or as no count. This door reads the end
//! instead, as the cone door reads its apex's double root
//! (`super::conic_quadric`'s deflated form and its spread):
//!
//! - **The composite.** The carrier, measured from the end into its
//!   span, composed with the surface's canonical form in its chart frame:
//!   a polynomial in the line's own parameter, or in the half angle
//!   `τ = tan(φ/2)` of a circle turned `φ` from the end, the circle then
//!   `X(τ)/W(τ)` with `W = 1 + τ²`. Its coefficients `c_j` are the
//!   composite's Taylor coefficients at the end.
//! - **The order** `m` is the least `j ≥ 1` whose coefficient decides
//!   nonzero, each read as the separation `c_j·L^j` its term makes over
//!   the window's local length `L`, over the form's gradient at the end
//!   (`curve_face_end_root_order`). A coefficient decided Zero is a coincidence the
//!   door acts on, as the glue does (D10), and the walk steps past it; one
//!   in band refuses, and the walk never steps on it. Every coefficient
//!   decided Zero is the carrier lying on the surface.
//! - **The window** `[0, s_w]`, read against a pivot term `p ≥ m`,
//!   halved from `L` until the terms past `p` are at most half the `p`-th
//!   over it (`curve_face_end_root_tail`), so the quotient
//!   `Q(s) = Σ_{j≥p} c_j s^{j−p}` keeps `c_p`'s sign there with
//!   `|Q| ≥ |c_p|/2`. A circle's `L` is a quarter turn at most
//!   (`curve_face_end_root_run`); the window is the whole span only where
//!   `L` is the span's own and no halving was taken. The pivots are tried
//!   from `m` up and the first window that holds is taken: a higher one
//!   only where `c_m` is small against its tail, as near a point where
//!   the two carriers curve alike, whose quotient has a root of its own
//!   near the end that no window short of it holds clear at the band.
//! - **The spread.** The terms below the pivot are dropped and each is
//!   charged at its value plus the zero band's width at its power over
//!   `L`, the most a coefficient read at its value admits: `c₀` with the
//!   end decided on the surface, the Zeros the walk stepped past, and from
//!   `c_m` up to the pivot with no verdict read on them, so a small true
//!   coefficient cannot move a root out of what is charged. The roots in the window then lie where `s^p·|c_p|/2` does not
//!   exceed the charge, which must end strictly inside the window
//!   (`curve_face_end_root_spread`). At the order's own pivot that is a
//!   double root's neighbourhood, `√(2ε/κ)` long and not `ε`, which no
//!   reading at the band shortens; at a higher pivot it also holds the
//!   quotient's roots near the end. The door returns the
//!   spread's length; what its roots are is the caller's to read (a
//!   junction reads the edge's side of the rim over it).
//! - **The window's far end** must stand definitely off the surface
//!   (`curve_face_end_root_window_clear`, the linearized residual), so no root of
//!   the rest of the span sits within the band of where the window ends.
//!   The rest of the span is the root doors', unchanged.
//!
//! What it reads: a line or a circle against a plane, a cylinder wall, a
//! sphere, a cone or a torus. Anything else is [`EndRootRefusal::NoLane`].

use geom::{Curve3, Surface};
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};

/// Which end of the span the root sits at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpanEnd {
    /// The span's first parameter.
    Start,
    /// Its last.
    End,
}

/// The window the door certified past the end: the carrier's parameter
/// where it ends, and the root's order at the end.
#[derive(Clone, Copy, Debug)]
pub struct EndWindow<T> {
    /// The carrier parameter at the window's far end, inside the span or
    /// at its other end.
    pub at: T,
    /// The window reaches the span's other end (`at` is it): the span holds
    /// no root but those within the spread of the end.
    pub whole: bool,
    /// The order of the end's root: the least coefficient decided nonzero.
    pub order: u32,
    /// The term the window was read against, the order or one past it:
    /// every term below it is dropped and charged.
    pub pivot: u32,
    /// How far from the end, in metres along the carrier at most, the
    /// roots the dropped terms can move may lie: the end's root and, at a
    /// pivot past the order, any root of the quotient near the end.
    pub spread: T,
}

/// Why the door read no window.
#[derive(Clone, Copy, Debug)]
pub enum EndRootRefusal {
    /// No lane for this carrier and surface kind.
    NoLane,
    /// A coefficient the order walk would step on read in band.
    Order(Indeterminate),
    /// The roots the dropped terms can move reach the window's far end.
    Spread(Option<Indeterminate>),
    /// Every coefficient decided Zero: the carrier lies on the surface.
    LiesOn,
    /// At no pivot does a window short of the halving budget hold the
    /// pivot's term against the rest, with its spread inside it and its far
    /// end clear: the order's own pivot's reading.
    Window(Option<Indeterminate>),
}

/// The order walk's predicate.
pub const END_ROOT_ORDER: &str = "curve_face_end_root_order";
/// The spread's predicate.
pub const END_ROOT_SPREAD: &str = "curve_face_end_root_spread";
/// Whether a circle's span is at most the quarter turn its window
/// starts from; anything short of proven reads as longer.
pub const END_ROOT_RUN: &str = "curve_face_end_root_run";
/// The window's tail: the terms past the pivot at most half its term.
pub const END_ROOT_TAIL: &str = "curve_face_end_root_tail";
/// The window's far end, off the surface.
pub const END_ROOT_WINDOW_CLEAR: &str = "curve_face_end_root_window_clear";
/// The halvings each of the door's three searches takes at most (the
/// window against its tail, the window's far end, the spread's
/// bisection): a double's mantissa, past which a halving moves the
/// window's length by less than the start's own rounding.
const HALVINGS: u32 = 52;

type Poly<T> = Vec<T>;

fn add<T: Real>(a: &[T], b: &[T]) -> Poly<T> {
    let n = a.len().max(b.len());
    (0..n)
        .map(|i| {
            let x = a.get(i).copied().unwrap_or_else(T::zero);
            let y = b.get(i).copied().unwrap_or_else(T::zero);
            x + y
        })
        .collect()
}

fn scale<T: Real>(k: T, a: &[T]) -> Poly<T> {
    a.iter().map(|&x| k * x).collect()
}

fn sub<T: Real>(a: &[T], b: &[T]) -> Poly<T> {
    add(a, &scale(-T::one(), b))
}

fn mul<T: Real>(a: &[T], b: &[T]) -> Poly<T> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![T::zero(); a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] = out[i + j] + x * y;
        }
    }
    out
}

/// `Σ |c_j| s^j` over `js`.
fn abs_terms<T: Real>(a: &[T], js: impl Iterator<Item = usize>, s: T) -> T {
    js.fold(T::zero(), |acc, j| {
        acc + a.get(j).copied().unwrap_or_else(T::zero).abs() * pow(s, j)
    })
}

fn pow<T: Real>(s: T, j: usize) -> T {
    s.powi(i32::try_from(j).unwrap_or(i32::MAX))
}

/// The carrier from the end: its homogeneous point `X/W` as polynomials
/// in the local parameter, the local length the window starts from,
/// whether that run reaches the span's far end, and the map back to the
/// carrier's parameter.
struct Local<T: Real> {
    x: [Poly<T>; 3],
    w: Poly<T>,
    run: T,
    covers: bool,
    t_other: T,
    kind: LocalKind,
    t_end: T,
    toward: T,
}

#[derive(Clone, Copy)]
enum LocalKind {
    Line,
    HalfAngle,
}

impl<T: Decide> Local<T> {
    /// The local parameter's top speed in metres: the carrier's own
    /// ([`geom::carrier_speed::speed_hi`]), twice it on a half angle,
    /// whose `dφ/dτ = 2/(1 + τ²)` is at most two.
    fn speed(&self, carrier: &Curve3<T>) -> Option<T> {
        let top = geom::carrier_speed::speed_hi(carrier)?;
        Some(match self.kind {
            LocalKind::Line => top,
            LocalKind::HalfAngle => top * T::from_f64(2.0),
        })
    }

    fn param(&self, s: T) -> T {
        match self.kind {
            LocalKind::Line => self.t_end + self.toward * s,
            LocalKind::HalfAngle => self.t_end + self.toward * T::from_f64(2.0) * s.atan(),
        }
    }
}

fn local<T: Decide>(carrier: &Curve3<T>, (t0, t1): (T, T), end: SpanEnd) -> Option<Local<T>> {
    let (t_end, t_other, toward) = match end {
        SpanEnd::Start => (t0, t1, T::one()),
        SpanEnd::End => (t1, t0, -T::one()),
    };
    let span = (t_other - t_end).abs();
    match carrier {
        Curve3::Line(line) => {
            let p = line.origin + line.dir.get() * t_end;
            let d = line.dir.get() * toward;
            Some(Local {
                x: [vec![p.x, d.x], vec![p.y, d.y], vec![p.z, d.z]],
                w: vec![T::one()],
                run: span,
                covers: true,
                t_other,
                kind: LocalKind::Line,
                t_end,
                toward,
            })
        }
        Curve3::Circle(circle) => {
            let r = circle.radius();
            let (p, d) = carrier.ders1(t_end);
            // The unit radial at the end, read through the circle's own
            // axis: `d = r·(axis × radial)`, so `radial = (d × axis)/r`.
            let u = d.cross(circle.axis()) / r;
            let c = p - u * r;
            let v = d * (toward / r);
            // A quarter turn at most: the window lives near the end, and
            // the half angle stays at most one. A longer span is not
            // covered: the rest of it is the root doors'.
            let quarter = T::from_f64(core::f64::consts::FRAC_PI_2);
            let covers = matches!(
                geom_core::k_stats::prove(END_ROOT_RUN, Margin::sign_of(quarter - span)),
                Ok(Sign::Positive | Sign::Zero)
            );
            let turned = if covers { span } else { quarter };
            let two = T::from_f64(2.0);
            let w = vec![T::one(), T::zero(), T::one()];
            let coord = |ci: T, ui: T, vi: T| vec![ci + r * ui, two * r * vi, ci - r * ui];
            Some(Local {
                x: [
                    coord(c.x, u.x, v.x),
                    coord(c.y, u.y, v.y),
                    coord(c.z, u.z, v.z),
                ],
                w,
                run: (turned / two).tan(),
                covers,
                t_other,
                kind: LocalKind::HalfAngle,
                t_end,
                toward,
            })
        }
        _ => None,
    }
}

/// The surface's canonical form composed along `local`, and its
/// gradient's norm at the end.
fn composite<T: Real>(surface: &Surface<T>, local: &Local<T>) -> Option<(Poly<T>, T)> {
    let (o, e) = crate::pcurve_cache::projected::chart_frame(surface)?;
    let ow = |k: usize| -> Poly<T> {
        let ok = match k {
            0 => o.x,
            1 => o.y,
            _ => o.z,
        };
        scale(ok, &local.w)
    };
    let rel: [Poly<T>; 3] = [0, 1, 2].map(|k| sub(&local.x[k], &ow(k)));
    let framed: [Poly<T>; 3] = e.map(|ei: Vec3<T>| {
        add(
            &add(&scale(ei.x, &rel[0]), &scale(ei.y, &rel[1])),
            &scale(ei.z, &rel[2]),
        )
    });
    let [y0, y1, y2] = &framed;
    let at = |p: &Poly<T>| p.first().copied().unwrap_or_else(T::zero);
    let (a, b, c) = (at(y0), at(y1), at(y2));
    let two = T::from_f64(2.0);
    let radial = add(&mul(y0, y0), &mul(y1, y1));
    let w2 = mul(&local.w, &local.w);
    Some(match *surface {
        Surface::Plane { .. } => (y2.clone(), T::one()),
        Surface::Cylinder { radius, .. } => (
            sub(&radial, &scale(radius.powi(2), &w2)),
            two * (a.powi(2) + b.powi(2)).sqrt(),
        ),
        Surface::Sphere { radius, .. } => (
            sub(&add(&radial, &mul(y2, y2)), &scale(radius.powi(2), &w2)),
            two * (a.powi(2) + b.powi(2) + c.powi(2)).sqrt(),
        ),
        Surface::Cone { half_angle, .. } => {
            let (sin, cos) = half_angle.sin_cos();
            let (s2, c2) = (sin.powi(2), cos.powi(2));
            (
                sub(&scale(c2, &radial), &scale(s2, &mul(y2, y2))),
                two * (c2.powi(2) * (a.powi(2) + b.powi(2)) + s2.powi(2) * c.powi(2)).sqrt(),
            )
        }
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => {
            let rr = major_radius.powi(2);
            let shifted = add(
                &add(&radial, &mul(y2, y2)),
                &scale(rr - minor_radius.powi(2), &w2),
            );
            let n = sub(
                &mul(&shifted, &shifted),
                &scale(T::from_f64(4.0) * rr, &mul(&radial, &w2)),
            );
            let big_a = a.powi(2) + b.powi(2) + c.powi(2) + rr - minor_radius.powi(2);
            let four = T::from_f64(4.0);
            let eight = T::from_f64(8.0);
            let g0 = four * big_a * a - eight * rr * a;
            let g1 = four * big_a * b - eight * rr * b;
            let g2 = four * big_a * c;
            (n, (g0.powi(2) + g1.powi(2) + g2.powi(2)).sqrt())
        }
        _ => return None,
    })
}

/// **The window past the end of `[t0, t1]` the door certifies**: the
/// module docs' reading.
///
/// # Errors
///
/// [`EndRootRefusal`] naming why no window was read.
pub fn end_window<T: Decide>(
    carrier: &Curve3<T>,
    span: (T, T),
    end: SpanEnd,
    surface: &Surface<T>,
    band: Band,
) -> Result<EndWindow<T>, EndRootRefusal> {
    let local = local(carrier, span, end).ok_or(EndRootRefusal::NoLane)?;
    let speed = local.speed(carrier).ok_or(EndRootRefusal::NoLane)?;
    let (n, g) = composite(surface, &local).ok_or(EndRootRefusal::NoLane)?;
    let run = local.run;
    // The order: the least `j ≥ 1` decided nonzero; a coefficient in band
    // stops the walk.
    let mut order = None;
    for (j, &c) in n.iter().enumerate().skip(1) {
        let term = c * pow(run, j);
        match decide(END_ROOT_ORDER, Margin::over_lever(term, g), band) {
            Ok(Sign::Positive | Sign::Negative) => {
                order = Some(j);
                break;
            }
            Ok(Sign::Zero) => {}
            Err(diag) => return Err(EndRootRefusal::Order(diag)),
        }
    }
    let m = order.ok_or(EndRootRefusal::LiesOn)?;
    // Every term a window drops is charged at its value plus the zero
    // band's width at its power: the most a coefficient read at its value
    // admits, whatever verdict was read on it.
    let zero = T::from_f64(band.zero());
    let charge: Vec<T> = n
        .iter()
        .enumerate()
        .map(|(j, c)| c.abs() + zero * g / pow(run, j))
        .collect();
    // The pivots from the order up; where none holds, the order's own
    // refusal is the door's.
    let read = |p: usize| {
        Pivot {
            n: &n,
            charge: &charge[..p],
            p,
        }
        .window(carrier, surface, &local, band)
        .map(|(at, whole, spread)| EndWindow {
            at,
            whole,
            order: u32::try_from(m).unwrap_or(u32::MAX),
            pivot: u32::try_from(p).unwrap_or(u32::MAX),
            spread: spread * speed,
        })
    };
    let refusal = match read(m) {
        Ok(window) => return Ok(window),
        Err(refusal) => refusal,
    };
    (m + 1..n.len()).find_map(|p| read(p).ok()).ok_or(refusal)
}

/// The window read against the composite's term `p`, every term below it
/// dropped and charged.
struct Pivot<'a, T> {
    n: &'a [T],
    charge: &'a [T],
    p: usize,
}

impl<T: Decide> Pivot<'_, T> {
    /// The window's far end as a carrier parameter, whether it is the
    /// whole span, and the spread in the local parameter.
    fn window(
        &self,
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        local: &Local<T>,
        band: Band,
    ) -> Result<(T, bool, T), EndRootRefusal> {
        let (n, p) = (self.n, self.p);
        let lead = n[p].abs() * T::from_f64(0.5);
        // The window: the terms past `p` at most half the `p`-th.
        let mut s_w = local.run;
        let mut unhalved = true;
        let mut held = false;
        for _ in 0..HALVINGS {
            let tail =
                (p + 1..n.len()).fold(T::zero(), |acc, j| acc + n[j].abs() * pow(s_w, j - p));
            if geom_core::k_stats::prove(END_ROOT_TAIL, Margin::sign_of(lead - tail))
                == Ok(Sign::Positive)
            {
                held = true;
                break;
            }
            s_w = s_w * T::from_f64(0.5);
            unhalved = false;
        }
        if !held {
            return Err(EndRootRefusal::Window(None));
        }
        // The charge below the pivot against the pivot's term. Its sign
        // changes once, from negative to positive: `excess(s) = s^p·(lead −
        // Σ_{j<p} charge_j·s^{j−p})` and every `s^{j−p}` falls as `s` grows.
        // So positive at `s` is positive past it, which the spread's
        // bisection below reads. The window is halved further until its far
        // end stands off the surface, the spread inside it.
        let excess = |s: T| lead * pow(s, p) - abs_terms(self.charge, 0..p, s);
        let mut in_band = None;
        for _ in 0..HALVINGS {
            if geom_core::k_stats::prove(END_ROOT_SPREAD, Margin::sign_of(excess(s_w)))
                != Ok(Sign::Positive)
            {
                return Err(EndRootRefusal::Spread(None));
            }
            // The window is the whole span exactly when its run is the
            // span's own, unhalved: its far end is then the span's other end.
            let whole = unhalved && local.covers;
            let at = if whole {
                local.t_other
            } else {
                local.param(s_w)
            };
            let far: Point3<T> = carrier.eval(at);
            match decide(
                END_ROOT_WINDOW_CLEAR,
                Margin::of(crate::implicit_residual(surface, far)),
                band,
            ) {
                Ok(Sign::Positive | Sign::Negative) => {
                    // The spread, bracketed from above: the least `s` the
                    // excess is proven positive at.
                    let (mut lo, mut hi) = (T::zero(), s_w);
                    for _ in 0..HALVINGS {
                        let mid = geom_core::mid_param(lo, hi);
                        if geom_core::k_stats::prove(END_ROOT_SPREAD, Margin::sign_of(excess(mid)))
                            == Ok(Sign::Positive)
                        {
                            hi = mid;
                        } else {
                            lo = mid;
                        }
                    }
                    return Ok((at, whole, hi));
                }
                Ok(Sign::Zero) => {}
                // A nearer window may clear; the first reading in band is
                // the refusal's where none does.
                Err(diag) => {
                    in_band.get_or_insert(diag);
                }
            }
            s_w = s_w * T::from_f64(0.5);
            unhalved = false;
        }
        Err(EndRootRefusal::Window(in_band))
    }
}

/// The predicate [`is_the_circle`] reads.
pub const END_ROOT_RIM_CARRIER: &str = "curve_face_end_root_rim_carrier";

/// **Whether `carrier` is the circle of `radius` about `center` in the
/// plane normal to the unit `axis`**: a circle whose points a third of a
/// turn apart each lie on that circle, their distance from it decided
/// Zero (`curve_face_end_root_rim_carrier`), three points fixing a
/// circle. Any other carrier is not.
#[must_use]
pub fn is_the_circle<T: Decide>(
    carrier: &Curve3<T>,
    (center, axis, radius): (Point3<T>, Vec3<T>, T),
    band: Band,
) -> bool {
    let (Curve3::Circle(_), Some(period)) = (carrier, carrier.period()) else {
        return false;
    };
    let third = period.length() / T::from_f64(3.0);
    [T::zero(), third, third * T::from_f64(2.0)]
        .into_iter()
        .all(|t| {
            let w = carrier.eval(t) - center;
            let height = w.dot(axis);
            let radial = (w - axis * height).norm() - radius;
            decide(
                END_ROOT_RIM_CARRIER,
                Margin::of((radial.powi(2) + height.powi(2)).sqrt()),
                band,
            ) == Ok(Sign::Zero)
        })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod rows {
    //! The rows read the elbow's own geometry: a pipe `x² + y² = 1` along
    //! `z ≤ 2`, and the bend that leaves its cap `z = 2` G1, the torus
    //! about the line `x = 3, z = 2` (along `y`) with `R = 3`, `r = 1`.
    use super::*;
    use geom_core::{Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn bend() -> Surface<f64> {
        super::super::conic_oracle::torus(
            Point3::new(3.0, 0.0, 2.0),
            Vec3::new(0.0, 1.0, 0.0),
            3.0,
            1.0,
        )
    }

    fn pipe() -> Surface<f64> {
        Surface::Cylinder {
            frame: geom::Frame::new(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            radius: 1.0,
        }
    }

    /// The pipe's ruling through the rim point at angle `theta`, from
    /// `z = 0` up to the rim: its end is the span's.
    fn ruling(theta: f64) -> Curve3<f64> {
        Curve3::Line(geom::Line {
            origin: Point3::new(theta.cos(), theta.sin(), 0.0),
            dir: geom_core::UnitVec3::exact_z(),
        })
    }

    /// A ruling ending on the rim away from the bend's neutral points is
    /// a double root: the junction's contact, and its curvature apart.
    #[test]
    fn a_ruling_ending_on_the_rim_is_a_double_root() {
        let w = end_window(&ruling(0.0), (0.0, 2.0), SpanEnd::End, &bend(), band())
            .expect("a window past the rim end");
        assert_eq!(w.order, 2, "the junction's double root");
        assert!(
            w.whole,
            "the bend meets the ruling's carrier again only past the span"
        );
    }

    /// **A root of the rest of the span is not covered over.** A pipe
    /// longer than `√12` has its ruling meet the bend's carrier again
    /// inside the span, at `z = 2 − √12`: the window ends short of it,
    /// so the root is the root doors' to find and place.
    #[test]
    fn a_long_rulings_second_root_lies_past_the_window() {
        let w = end_window(&ruling(0.0), (-2.0, 2.0), SpanEnd::End, &bend(), band())
            .expect("a window past the rim end");
        let second = 2.0 - 12.0_f64.sqrt();
        assert_eq!(w.order, 2);
        assert!(!w.whole, "the window is not the span");
        assert!(
            w.at > second && w.at < 2.0,
            "window to {} keeps {second} out",
            w.at
        );
    }

    /// **At a neutral point the order steps past the Zeros.** Through
    /// `(0, 1, 2)` the bend's distance from the ruling is
    /// `(√(9 + s²) − 3)² ≈ s⁴/36`: the quadratic and cubic terms decide
    /// Zero and the quartic decides the order.
    #[test]
    fn a_ruling_ending_at_a_neutral_point_is_a_fourth_order_root() {
        let w = end_window(
            &ruling(core::f64::consts::FRAC_PI_2),
            (0.0, 2.0),
            SpanEnd::End,
            &bend(),
            band(),
        )
        .expect("a window past the neutral point");
        assert_eq!(w.order, 4, "the quartic decides");
    }

    /// **A quadratic term in band refuses; the walk never steps on it.**
    /// The ruling turned off the neutral point by `δ` meets a relative
    /// curvature `κ ≈ δ/3`, whose quadratic term separates the two by
    /// `κ·L²/2 = (2/3)·δ` over the span `L = 2`; `δ` puts that mid-band.
    #[test]
    fn a_ruling_whose_quadratic_term_is_in_band_refuses() {
        let b = band();
        let delta = 1.5 * 0.5 * (b.zero() + b.escalate());
        let got = end_window(
            &ruling(core::f64::consts::FRAC_PI_2 - delta),
            (0.0, 2.0),
            SpanEnd::End,
            &bend(),
            b,
        );
        assert!(
            matches!(got, Err(EndRootRefusal::Order(_))),
            "in band at δ = {delta}: {got:?}"
        );
    }

    /// The bend's sweep arc from the rim point `(1, 0, 2)`, a quarter
    /// turn about the bend's axis, against the pipe: a double root.
    #[test]
    fn an_arc_leaving_the_rim_is_a_double_root_on_the_pipe() {
        let arc = Curve3::circle(
            Point3::new(3.0, 0.0, 2.0),
            Vec3::new(0.0, 1.0, 0.0),
            2.0,
            Vec3::new(-1.0, 0.0, 0.0),
        );
        let w = end_window(
            &arc,
            (0.0, core::f64::consts::FRAC_PI_2),
            SpanEnd::Start,
            &pipe(),
            band(),
        )
        .expect("a window past the rim end");
        assert_eq!(w.order, 2);
        assert!(w.at > 0.0, "past the end: {}", w.at);
    }

    /// **A circle past a quarter turn is not read whole.** The arc
    /// leaves the pipe's rim point `(1, 0, 2)` tangent to the wall and
    /// meets the wall again at `φ = acos(−1/3)`, past a quarter turn and
    /// inside its span: the window stops short of the span's end, so that
    /// root is the root doors' to find.
    #[test]
    fn an_arc_past_a_quarter_turn_is_not_read_whole() {
        let arc = Curve3::circle(
            Point3::new(-0.5, 0.0, 2.0),
            Vec3::new(0.0, 1.0, 0.0),
            1.5,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let second = (-1.0_f64 / 3.0).acos();
        let w = end_window(&arc, (0.0, 2.2), SpanEnd::Start, &pipe(), band())
            .expect("a window past the end");
        assert!(!w.whole, "a span with a second root at {second}: {w:?}");
        assert!(w.at < second, "the window keeps {second} out: {w:?}");
    }

    /// The same inside the wall: a circle of radius 1.4 tangent to it
    /// along a ruling crosses it again at `acos(−0.6/1.4)`, inside a span
    /// of 2.5.
    #[test]
    fn an_arc_inside_the_wall_past_a_quarter_turn_is_not_read_whole() {
        let rho = 1.4;
        let arc = Curve3::circle(
            Point3::new(1.0 - rho, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            rho,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let cross = ((rho - 2.0) / rho).acos();
        let w = end_window(&arc, (0.0, 2.5), SpanEnd::Start, &pipe(), band())
            .expect("a window past the end");
        assert!(
            !w.whole && w.at < cross,
            "the arc crosses the wall at {cross}: {w:?}"
        );
    }

    /// **The order walk decides `c₁`**: an edge crossing the wall square
    /// at its end is a simple root, never stepped past.
    #[test]
    fn a_transverse_end_is_a_simple_root() {
        let line = Curve3::Line(geom::Line {
            origin: Point3::new(0.0, 0.0, 1.0),
            dir: geom_core::UnitVec3::exact_x(),
        });
        let w = end_window(&line, (0.0, 1.0), SpanEnd::End, &pipe(), band())
            .expect("a window past the end");
        assert_eq!(w.order, 1, "{w:?}");
    }

    /// The spread a pivot `p` reads, solved here: where `|c_p|·σ^p/2`
    /// meets every term below it charged at its value plus the zero band's
    /// width at its power over the run `L = 2`. The coefficients are the
    /// door's own; the charges are this solve's.
    fn spread_at_pivot(c: &Curve3<f64>, b: Band, p: usize) -> f64 {
        let local = local(c, (0.0, 2.0), SpanEnd::End).unwrap();
        let (n, g) = composite(&bend(), &local).unwrap();
        let charge = |s: f64| -> f64 {
            (0..p)
                .map(|j| (n[j].abs() + b.zero() * g / 2.0_f64.powi(j as i32)) * s.powi(j as i32))
                .sum()
        };
        let (mut lo, mut hi) = (0.0_f64, 2.0_f64);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if 0.5 * n[p].abs() * mid.powi(p as i32) > charge(mid) {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        hi
    }

    /// **The spread is where the pivot's half term meets the charge**: at
    /// the neutral point (order 4, `c₁…c₃` decided Zero) and at `δ = √ε/3`
    /// off it (order 2, pivot 4, `c₂` a part in 10⁴ of its value above its
    /// width at `ε = 1e-9`).
    #[test]
    fn the_spread_meets_the_charge_at_each_dropped_power() {
        let b = Band::linear_at(Tol::witness(), 1e-9).unwrap();
        for (delta, order) in [(0.0, 4), (1e-9_f64.sqrt() / 3.0, 2)] {
            let c = ruling(core::f64::consts::FRAC_PI_2 - delta);
            let w = end_window(&c, (0.0, 2.0), SpanEnd::End, &bend(), b).expect("a window");
            assert!(
                w.order == order && w.pivot == 4,
                "δ {delta:e}: the quartic's pivot: {w:?}"
            );
            let want = spread_at_pivot(&c, b, 4);
            assert!(
                (w.spread - want).abs() <= 1e-9 * want,
                "δ {delta:e}: spread {} against {want}",
                w.spread
            );
        }
    }

    /// The ruling turned `δ` off the neutral point meets the bend's
    /// carrier again `√(12·sin δ)` below the rim: there
    /// `√((3 − sin δ)² + s²) = 3 + sin δ`.
    fn near_root(delta: f64) -> f64 {
        (12.0 * delta.sin()).sqrt()
    }

    /// **Every root in the window lies within the spread.** Off the
    /// neutral point by `δ` the quotient after the end's double root has a
    /// root of its own at [`near_root`], and between the two the ruling
    /// stands at most `δ²/2` off the bend: below `δ ≈ √(2Kε)` no window
    /// short of it clears the band. So the window holds it inside its
    /// spread or ends short of it, at every `δ`, which a charge too small
    /// for the terms it drops breaks: the spread then ends short of a
    /// root the window holds. Across the belt the window pivots on the
    /// quartic and holds the whole span; well past it the order's own
    /// window holds and stops short of the near root, the root doors'. At
    /// `ε` = 1e-6, 1e-9 and 1e-12, `δ` in band units.
    #[test]
    fn every_root_in_the_window_lies_within_the_spread() {
        for eps in [1e-6, 1e-9, 1e-12] {
            let b = Band::linear_at(Tol::witness(), eps).unwrap();
            let root_eps = eps.sqrt();
            let quartic = [
                3.0 * b.escalate(),
                30.0 * b.escalate(),
                0.1 * root_eps,
                root_eps,
            ];
            let scan = [
                3.0 * root_eps,
                5.0 * root_eps,
                10.0 * root_eps,
                100.0 * root_eps,
            ];
            for delta in quartic.into_iter().chain(scan) {
                let what = format!("ε {eps:e}, δ {delta:e}");
                let w = end_window(
                    &ruling(core::f64::consts::FRAC_PI_2 - delta),
                    (0.0, 2.0),
                    SpanEnd::End,
                    &bend(),
                    b,
                )
                .unwrap_or_else(|e| panic!("{what}: a window: {e:?}"));
                assert_eq!(w.order, 2, "{what}: the end's root is double");
                let root = near_root(delta);
                assert!(
                    root < w.spread || root > 2.0 - w.at,
                    "{what}: the near root at {root} lies in the window past its spread: {w:?}"
                );
                if quartic.contains(&delta) {
                    assert!(
                        w.pivot == 4 && w.whole && root < w.spread,
                        "{what}: the quartic holds the whole span, the near root in the spread: {w:?}"
                    );
                }
                if delta == 100.0 * root_eps {
                    assert!(
                        w.pivot == 2 && root > 2.0 - w.at,
                        "{what}: the order's window stops short of the near root: {w:?}"
                    );
                }
            }
        }
    }

    /// A carrier the door has no lane for.
    #[test]
    fn an_ellipse_has_no_lane() {
        let e = Curve3::ellipse(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            2.0,
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
            band(),
        )
        .unwrap();
        assert!(matches!(
            end_window(&e, (0.0, 1.0), SpanEnd::Start, &pipe(), band()),
            Err(EndRootRefusal::NoLane)
        ));
    }
}
