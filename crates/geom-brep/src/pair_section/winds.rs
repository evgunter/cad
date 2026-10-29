//! **How a fitted loop winds about a surface**: its integer turns about
//! each of the surface's essential channels, read off the loop in ℝ³ —
//! about the axis on a cylinder; about the axis and about the core
//! circle on a ring torus. A sphere has none: every loop on it is null.
//! On the surface these are the loop's turns in the chart's periodic
//! parameters; they are read here on the loop itself, which lies off
//! the surface, and stay defined off it.
//!
//! # The lever
//!
//! A channel is the angle of a point about a centre in a plane: about
//! the axis, the point's projection on the plane square to it; about the
//! core, its meridian coordinates `(ρ − R, h)`. Both maps are 1-Lipschitz
//! from ℝ³. The loop is cut into pieces, and each piece's control hull is
//! enclosed in interval arithmetic (`NurbsCurve3::span_enclosure`) and
//! carried through the map, so the piece's image lies in an interval
//! rectangle. Where that rectangle, padded by `pad`, misses the centre,
//! it is a convex set missing it, so the angle turns by less than `π`
//! along the piece, and the piece's turning is the principal difference
//! of its ends' angles. The pieces' turnings sum to the loop's, a whole
//! number of turns where the loop closes.
//!
//! **The ends are read at `f64`**, uncertified: a piece's end is
//! `C(t)` evaluated, within its evaluation rounding of the true point,
//! and the rectangle puts it at least `pad` from the centre, so its angle
//! is off by at most that rounding over `pad`. The premise is that
//! `pad` (a tube radius, `1e-4` m and up at the run's ε) is many orders
//! above the rounding (`1e-15` of the loop's size). Then a principal
//! difference read within `3π/4` of zero is the true turning to within
//! far less than the `π/4` left before a wrong branch, and a piece read
//! past it is halved rather than trusted. Neighbouring pieces share their
//! end, evaluated once each side at the same parameter, so the ends'
//! errors cancel along the loop and only the loop's own closing gap is
//! left in the sum.
//!
//! # The pad
//!
//! `pad` is the loop's [`super::CarriedComponent::pad`], within which
//! every point of the section's component lies of the carrier (closeness,
//! argued there). The straight homotopy from a point of the component to
//! its nearest carrier point stays within `pad` of the carrier, inside
//! the padded rectangles, so the component winds as the carrier does.

use geom::{Curve3, Surface};
use geom_core::{Bounds as _, Interval, Point3, Vec3};

/// **A loop's turns about one surface's channels**: about its axis
/// (cylinder and torus) and about its core circle (torus); zero on a
/// channel the surface does not have.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Winding {
    /// Turns about the axis.
    pub about_axis: i64,
    /// Turns about the core circle.
    pub about_core: i64,
}

impl Winding {
    /// Whether the loop is essential on the surface: it turns about one
    /// of its channels.
    #[must_use]
    pub fn essential(self) -> bool {
        self.about_axis != 0 || self.about_core != 0
    }
}

/// **Why a loop's winding was not read.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindsRefusal {
    /// The carrier is not a spline.
    NotSpline,
    /// A piece's enclosure, halved to the cut budget, still reaches the
    /// channel's centre within the pad.
    Unseparated,
    /// The loop's turning is not a whole number of turns: it does not
    /// close.
    Open,
}

/// **The halvings one reading may make**: a liveness bound, never a
/// soundness one (running out refuses `Unseparated`). A loop whose
/// rectangles clear at all needs pieces about as short as its distance
/// from the centre less `pad`; `2¹⁴` covers a loop some ten thousand of
/// those lengths long, far past any section a torus's ball holds at the
/// tube radii the march proves.
const PIECE_BUDGET: usize = 1 << 14;

/// **The largest principal difference trusted as a piece's turning**:
/// `3π/4`, leaving `π/4` before the branch cut, which an end's angle
/// error (its rounding over `pad`, module docs) cannot cross.
const TRUSTED_TURN: f64 = 0.75 * core::f64::consts::PI;

/// **How far the turns may miss a whole number and still close**, in
/// radians: the loop's closing gap (its fitted ends one point to the
/// fit's rounding) over its distance from the centre, at least `pad`,
/// is many orders below `0.25`; a sum past it is a loop that does not
/// close.
const CLOSING_SLACK: f64 = 0.25;

/// One channel: the angle of a point about its centre, and a piece's
/// enclosure clearing the centre by `pad`.
#[derive(Clone, Copy, Debug)]
enum Channel {
    /// About the axis through `origin` along `axis`, in the frame
    /// `(e1, e2)` square to it.
    Axis {
        origin: Point3<f64>,
        e1: Vec3<f64>,
        e2: Vec3<f64>,
    },
    /// About the core circle of radius `big_r` about `axis` through
    /// `center`.
    Core {
        center: Point3<f64>,
        axis: Vec3<f64>,
        big_r: f64,
    },
}

/// A computed value lifted to its point interval: the scalar's own lift
/// of a computed value (`Real::from_computed`, the point itself on the
/// interval lane).
fn lift(v: f64) -> Interval {
    <Interval as geom_core::Real>::from_computed(v)
}

fn lift3(v: Vec3<f64>) -> Vec3<Interval> {
    v.map(lift)
}

/// Whether the interval `x`, padded by `pad`, excludes zero.
fn clears(x: Interval, pad: f64) -> bool {
    x.lo() > pad || x.hi() < -pad
}

impl Channel {
    /// The plane coordinates of `p` about the channel's centre.
    fn coords(self, p: Point3<f64>) -> (f64, f64) {
        match self {
            Self::Axis { origin, e1, e2 } => {
                let w = p - origin;
                (w.dot(e1), w.dot(e2))
            }
            Self::Core {
                center,
                axis,
                big_r,
            } => {
                let w = p - center;
                let (_, rho) = super::square_part(w, axis);
                (rho - big_r, w.dot(axis))
            }
        }
    }

    fn angle(self, p: Point3<f64>) -> f64 {
        let (x, y) = self.coords(p);
        y.atan2(x)
    }

    /// Whether the enclosure `b` of a piece, padded by `pad`, clears the
    /// centre in the channel's plane.
    fn clear(self, b: Point3<Interval>, pad: f64) -> bool {
        let w = |o: Point3<f64>| Vec3::new(b.x - lift(o.x), b.y - lift(o.y), b.z - lift(o.z));
        match self {
            Self::Axis { origin, e1, e2 } => {
                let w = w(origin);
                clears(w.dot(lift3(e1)), pad) || clears(w.dot(lift3(e2)), pad)
            }
            Self::Core {
                center,
                axis,
                big_r,
            } => {
                use geom_core::Real as _;
                let w = w(center);
                let a = lift3(axis);
                let h = w.dot(a);
                let perp = a.cross(w.cross(a));
                let rho = perp.dot(perp).sqrt();
                clears(rho - lift(big_r), pad) || clears(h, pad)
            }
        }
    }
}

/// The essential channels of `surface`.
fn channels(surface: &Surface<f64>) -> Vec<Channel> {
    match *surface {
        Surface::Cylinder {
            origin,
            axis,
            u_ref,
            ..
        } => vec![Channel::Axis {
            origin,
            e1: u_ref,
            e2: axis.cross(u_ref),
        }],
        Surface::Torus {
            center,
            axis,
            major_radius,
            u_ref,
            ..
        } => vec![
            Channel::Axis {
                origin: center,
                e1: u_ref,
                e2: axis.cross(u_ref),
            },
            Channel::Core {
                center,
                axis,
                big_r: major_radius,
            },
        ],
        _ => Vec::new(),
    }
}

/// **The winding of the closed carrier over `span` about `surface`**
/// (module docs), every piece's image cleared of each channel's centre
/// by `pad`.
///
/// # Errors
///
/// [`WindsRefusal`].
pub fn winding(
    surface: &Surface<f64>,
    carrier: &Curve3<f64>,
    (lo, hi): (f64, f64),
    pad: f64,
) -> Result<Winding, WindsRefusal> {
    let chans = channels(surface);
    if chans.is_empty() {
        return Ok(Winding::default());
    }
    let Curve3::Nurbs(net) = carrier else {
        return Err(WindsRefusal::NotSpline);
    };
    let lifted = net.map_scalar(lift);
    let mut breaks: Vec<f64> = net
        .knot_values()
        .runs()
        .map(|(v, _)| v)
        .filter(|&v| lo < v && v < hi)
        .collect();
    breaks.insert(0, lo);
    breaks.push(hi);
    let mut turns = [0i64; 2];
    for (k, &chan) in chans.iter().enumerate() {
        let mut total = 0.0;
        let mut cuts = 0usize;
        let mut stack: Vec<(f64, f64)> = breaks.windows(2).rev().map(|w| (w[0], w[1])).collect();
        while let Some((a, b)) = stack.pop() {
            let delta = {
                let d = chan.angle(net.eval(b)) - chan.angle(net.eval(a));
                (d + core::f64::consts::PI).rem_euclid(core::f64::consts::TAU)
                    - core::f64::consts::PI
            };
            let read = lifted
                .span_enclosure(a, b)
                .is_some_and(|enc| chan.clear(enc, pad))
                && delta.abs() <= TRUSTED_TURN;
            if read {
                total += delta;
                continue;
            }
            cuts += 1;
            let mid = 0.5 * (a + b);
            if cuts > PIECE_BUDGET || !(a < mid && mid < b) {
                return Err(WindsRefusal::Unseparated);
            }
            stack.push((mid, b));
            stack.push((a, mid));
        }
        let whole = (total / core::f64::consts::TAU).round();
        if (total - whole * core::f64::consts::TAU).abs() > CLOSING_SLACK {
            return Err(WindsRefusal::Open);
        }
        #[allow(clippy::cast_possible_truncation)]
        {
            turns[k] = whole as i64;
        }
    }
    Ok(Winding {
        about_axis: turns[0],
        about_core: turns[1],
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::{Winding, WindsRefusal, winding};
    use geom::{Curve3, Surface};
    use geom_core::{Point3, Vec3};

    /// A closed spline through the points of `f` at `n` stations round
    /// a turn, interpolated piecewise-linearly (degree 1), so the test
    /// fixes what the loop is.
    fn polygon(n: usize, f: impl Fn(f64) -> Point3<f64>) -> Curve3<f64> {
        use core::num::NonZeroUsize;
        let one = NonZeroUsize::new(1).unwrap();
        let two = NonZeroUsize::new(2).unwrap();
        #[allow(clippy::cast_precision_loss)]
        let t = |i: usize| i as f64 / n as f64;
        let mut runs = vec![(0.0, two)];
        runs.extend((1..n).map(|i| (t(i), one)));
        runs.push((1.0, two));
        let knots = geom_core::spline::Knots::new(runs, 1).unwrap();
        let control: Vec<_> = (0..=n)
            .map(|i| f(core::f64::consts::TAU * t(i % n)))
            .collect();
        let net = geom::NurbsCurve3::new(knots, control, vec![1.0; n + 1]).unwrap();
        Curve3::Nurbs(std::sync::Arc::new(net))
    }

    fn torus() -> Surface<f64> {
        Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius: 2.0,
            minor_radius: 0.5,
            u_ref: Vec3::unit_x(),
        }
    }

    /// **The torus's two channels read apart**: a longitude turns once
    /// about the axis (twice, run twice round), a meridian once about the
    /// core, a `(1, 1)` curve once about each, and a small loop on the
    /// tube's outer side about neither.
    #[test]
    fn a_torus_loop_turns_as_it_runs() {
        let (big_r, r) = (2.0, 0.5);
        let on = |u: f64, v: f64| {
            let rho = big_r + r * v.cos();
            Point3::new(rho * u.cos(), rho * u.sin(), r * v.sin())
        };
        let read = |c: &Curve3<f64>| winding(&torus(), c, (0.0, 1.0), 1e-9);
        let longitude = polygon(64, |s| on(s, 0.3));
        let twice = polygon(128, |s| on(2.0 * s, 0.3));
        let meridian = polygon(64, |s| on(0.4, s));
        let diagonal = polygon(128, |s| on(s, s));
        let small = polygon(32, |s| on(0.1 * s.cos(), 0.2 * s.sin()));
        let w = |a, c| {
            Ok(Winding {
                about_axis: a,
                about_core: c,
            })
        };
        assert_eq!(read(&longitude), w(1, 0), "a longitude");
        assert_eq!(read(&twice), w(2, 0), "a longitude run twice round");
        assert_eq!(read(&meridian), w(0, 1), "a meridian");
        assert_eq!(read(&diagonal), w(1, 1), "a (1, 1) curve");
        assert_eq!(read(&small), w(0, 0), "a null loop");
    }

    /// **A loop the pad cannot part from a centre refuses**: a loop
    /// round the cylinder's axis read with a pad past its radius.
    #[test]
    fn a_pad_reaching_the_axis_refuses() {
        let wall = Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 0.5,
            u_ref: Vec3::unit_x(),
        };
        let ring = polygon(64, |s| {
            Point3::new(0.5 * s.cos(), 0.5 * s.sin(), 0.1 * s.sin())
        });
        assert_eq!(
            winding(&wall, &ring, (0.0, 1.0), 1e-9).map(Winding::essential),
            Ok(true),
            "a ring round the wall"
        );
        assert_eq!(
            winding(&wall, &ring, (0.0, 1.0), 0.6),
            Err(WindsRefusal::Unseparated),
            "a pad past the radius"
        );
    }
}
