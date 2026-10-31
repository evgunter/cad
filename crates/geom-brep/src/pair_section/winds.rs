//! **How a fitted loop winds about a surface**: its integer turns about
//! each of the surface's essential channels, read over the loop's
//! uniqueness tube in ℝ³ — about the axis on a cylinder; about the axis
//! and about the core circle on a ring torus. A sphere has none: every
//! loop on it is null. On the surface these are the loop's turns in the
//! chart's periodic parameters; they are read here in ℝ³, where they
//! stay defined off the surface.
//!
//! # The lever
//!
//! A channel is the angle of a point about a centre in a plane: about
//! the axis, the point's projection on the plane square to it; about the
//! core, its meridian coordinates `(ρ − R, h)`. The loop is read over
//! its pieces, each a stretch of the carrier beside a convex set holding
//! it, carried through the map in interval arithmetic into an interval
//! rectangle. Where that rectangle misses the centre, the angle turns by
//! less than `π` along the stretch, and its turning is the principal
//! difference of its ends' angles. The pieces' turnings sum to the
//! loop's, a whole number of turns where the loop closes.
//!
//! **The ends are read at `f64`**, uncertified: a piece's end is `C(t)`
//! evaluated, within its evaluation rounding of the true point, and the
//! rectangle puts the stretch at least the tube radius from the centre
//! (the pieces are tube boxes, below), so its angle is off by at most
//! that rounding over the radius. The premise is that the radius (`1e-4`
//! m and up at the run's ε) is many orders above the rounding (`1e-15`
//! of the loop's size). Then a principal difference read within `3π/4`
//! of zero is the true turning to within far less than the `π/4` left
//! before a wrong branch, and a piece read past it refuses. Neighbouring
//! pieces share their end, evaluated once each side at the same
//! parameter, so the ends' errors cancel along the loop and only the
//! loop's own closing gap is left in the sum.
//!
//! # The pieces
//!
//! The pieces are the loop's uniqueness tube: its certificate's box
//! chain, each box padded by the tube radius (C2 limb 3), which is the
//! cover the march's exhaustiveness accounts the section to. Each box
//! is convex and holds its stretch of the carrier (the hull's
//! convex-hull property). Limb 3 proves the section's points in the
//! chain one arc: one connected piece of it in each box, consecutive
//! boxes' pieces sharing a point. Walk the component through the pieces
//! in the chain's order, from a shared point to the next inside each
//! piece; the straight homotopy from that walk to the carrier, stretch
//! by stretch, stays inside the boxes, so where no box's rectangle
//! reaches a centre the walk winds as the carrier does. That needs no
//! bound on how far a point of the component lies from the carrier.
//!
//! **The walk winds as many times round the component as it goes
//! round it**, a number `m` limb 3 does not fix: it puts a piece in
//! every box and links neighbours, but a walk could double back
//! (`m = 0`) or go round twice. So the carrier's turns are `m` times
//! the component's. Where they are not zero the component is essential,
//! whatever `m` is. Where the box holding the whole tube clears a
//! centre, the component, inside it, is null about that channel. A null
//! reading that neither settles needs `m ≠ 0`, and the walk is then
//! shown once round ([`once_round`], [`gated`]), or the reading refuses
//! `Unordered`. A [`Winding`]'s numbers are the carrier's turns, the
//! component's where the walk is once round.
//!
//! Once round by runs ([`once_round`]): group the
//! chain into runs of consecutive boxes; each run's pieces form one
//! connected arc of the component (neighbours share a point), so the
//! runs cover the component by arcs. Where no box of a run meets a box
//! of a run that is not its neighbour, and there are at least four runs,
//! each arc meets only its two neighbours, so the arcs lie round the
//! component in the runs' order, and the walk, stepping from each to
//! the next, goes round once. Its turns are the component's, up to the
//! sign the component's orientation takes, which essentiality does not
//! read.
//!
//! Where the tube comes back near itself (two lobes of a loop near a
//! node), runs that far apart meet, and the walk is shown once round by
//! a gate instead ([`gated`]): one piece the walk must cross, at a
//! slice of its box that no other box reaches.

use crate::ssi::enclose::Box3;
use geom::{Curve3, Surface};
use geom_core::{Bounds as _, Interval, Point3, Vec3};

/// **One piece of a loop**: a box holding the carrier over the
/// parameter span beside it and the section's stretch there, and the
/// axis limb 3 read that stretch a graph over the slices square to
/// (module docs).
pub(crate) type Piece = (Box3, Vec3<f64>, (f64, f64));

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
    /// A piece's rectangle reaches a channel's centre, or turns too far
    /// to be read from its ends.
    Unseparated,
    /// The pieces do not tile the carrier's span in order.
    Untiled,
    /// The walk through the pieces is not shown to run round the
    /// component once ([`once_round`]).
    Unordered,
    /// The loop's turning is not a whole number of turns: it does not
    /// close.
    Open,
}

/// **The largest principal difference trusted as a piece's turning**:
/// `3π/4`, leaving `π/4` before the branch cut, which an end's angle
/// error (its rounding over the tube radius, module docs) cannot cross.
const TRUSTED_TURN: f64 = 0.75 * core::f64::consts::PI;

/// **How far the turns may miss a whole number and still close**, in
/// radians: the loop's closing gap (its fitted ends one point to the
/// fit's rounding) over its distance from the centre, at least the
/// tube radius, is many orders below `0.25`; a sum past it is a loop
/// that does not close.
const CLOSING_SLACK: f64 = 0.25;

/// One channel: the angle of a point about its centre, and a piece's
/// box clearing it.
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

/// Whether the interval `x` excludes zero; never where it is refused.
fn clears(x: Interval) -> bool {
    x.is_certified() && (x.lo() > 0.0 || x.hi() < 0.0)
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

    /// Whether the box `b` clears the centre in the channel's plane.
    fn clear(self, b: Box3) -> bool {
        let b = Point3::new(b.x, b.y, b.z);
        let w = |o: Point3<f64>| Vec3::new(b.x - lift(o.x), b.y - lift(o.y), b.z - lift(o.z));
        match self {
            Self::Axis { origin, e1, e2 } => {
                let w = w(origin);
                clears(w.dot(lift3(e1))) || clears(w.dot(lift3(e2)))
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
                clears(rho - lift(big_r)) || clears(h)
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

/// **The winding of the closed carrier over `span` about `surface`**,
/// read over `pieces` (module docs), which tile the span in order and
/// whose walk runs round the component once.
///
/// # Errors
///
/// [`WindsRefusal`].
pub(crate) fn winding(
    surface: &Surface<f64>,
    carrier: &Curve3<f64>,
    (lo, hi): (f64, f64),
    pieces: &[Piece],
) -> Result<Winding, WindsRefusal> {
    let chans = channels(surface);
    if chans.is_empty() {
        return Ok(Winding::default());
    }
    let Curve3::Nurbs(net) = carrier else {
        return Err(WindsRefusal::NotSpline);
    };
    let tiled = pieces.first().is_some_and(|(_, _, (a, _))| *a == lo)
        && pieces.last().is_some_and(|(_, _, (_, b))| *b == hi)
        && pieces.windows(2).all(|w| w[0].2.1 == w[1].2.0);
    if !tiled {
        return Err(WindsRefusal::Untiled);
    }
    let mut turns = [0i64; 2];
    for (k, &chan) in chans.iter().enumerate() {
        let mut total = 0.0;
        for &(enclosure, _, (a, b)) in pieces {
            let delta = {
                let d = chan.angle(net.eval(b)) - chan.angle(net.eval(a));
                (d + core::f64::consts::PI).rem_euclid(core::f64::consts::TAU)
                    - core::f64::consts::PI
            };
            if !chan.clear(enclosure) || delta.abs() > TRUSTED_TURN {
                return Err(WindsRefusal::Unseparated);
            }
            total += delta;
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
    // A null reading the whole tube's box does not settle needs the
    // walk once round (module docs).
    let hull = hull(pieces);
    let unsettled = chans
        .iter()
        .zip(turns)
        .any(|(chan, t)| t == 0 && !hull.is_some_and(|h| chan.clear(h)));
    if unsettled && !(once_round(pieces) || gated(pieces)) {
        return Err(WindsRefusal::Unordered);
    }
    Ok(Winding {
        about_axis: turns[0],
        about_core: turns[1],
    })
}

/// **Whether the walk through `pieces` runs round once** (module docs):
/// for some run length `k` (`1, 2, 4, …`) giving at least four runs of
/// consecutive pieces (the last taking the remainder, the runs read
/// round the closed chain), every pair of boxes that are not certainly
/// apart lies in one run or in neighbouring ones.
fn once_round(pieces: &[Piece]) -> bool {
    let n = pieces.len();
    let met: Vec<(usize, usize)> = (0..n)
        .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
        .filter(|&(i, j)| !matches!(meet(pieces[i].0, pieces[j].0), Meet::Apart))
        .collect();
    let mut k = 1;
    while n / k >= 4 {
        let runs = n / k;
        let run = |i: usize| (i / k).min(runs - 1);
        let near = |i: usize, j: usize| {
            let d = run(j) - run(i);
            d <= 1 || d == runs - 1
        };
        if met.iter().all(|&(i, j)| near(i, j)) {
            return true;
        }
        k *= 2;
    }
    false
}

/// The box holding every piece, `None` where one is refused. Both
/// sides certified, the hull of their brackets is a sound bracket of
/// the hull, so minting it from their endpoints launders nothing.
fn hull(pieces: &[Piece]) -> Option<Box3> {
    let join = |p: Interval, q: Interval| {
        (p.is_certified() && q.is_certified())
            .then(|| Interval::from_bounds(p.lo().min(q.lo()), p.hi().max(q.hi())))
    };
    let (first, rest) = pieces.split_first()?;
    rest.iter().try_fold(first.0, |h, (b, _, _)| {
        Some(Box3 {
            x: join(h.x, b.x)?,
            y: join(h.y, b.y)?,
            z: join(h.z, b.z)?,
        })
    })
}

/// Two boxes against each other: certainly apart, meeting in a box, or
/// not read (a refused side).
enum Meet {
    Apart,
    In(Box3),
    Unread,
}

/// `a ∩ b`. Both sides certified, the intersection of their brackets
/// is a sound bracket of the intersection, so minting it from their
/// endpoints launders nothing.
fn meet(a: Box3, b: Box3) -> Meet {
    let mut out = [lift(0.0); 3];
    for (k, (p, q)) in [(a.x, b.x), (a.y, b.y), (a.z, b.z)].into_iter().enumerate() {
        if !(p.is_certified() && q.is_certified()) {
            return Meet::Unread;
        }
        let (lo, hi) = (p.lo().max(q.lo()), p.hi().min(q.hi()));
        if lo > hi {
            return Meet::Apart;
        }
        out[k] = Interval::from_bounds(lo, hi);
    }
    Meet::In(Box3 {
        x: out[0],
        y: out[1],
        z: out[2],
    })
}

/// The range of `x · e` over the box `b`, `None` where refused.
fn along(b: Box3, e: Vec3<f64>) -> Option<(f64, f64)> {
    let s = b.x * lift(e.x) + b.y * lift(e.y) + b.z * lift(e.z);
    s.is_certified().then(|| (s.lo(), s.hi()))
}

/// **Whether some piece is a gate the walk crosses once** (module
/// docs): a box `K` whose meets with its two neighbours lie, along its
/// axis `e`, on either side of a slice `x · e = σ` that no other box's
/// meet with `K` reaches. The piece in `K` is a graph over the slices,
/// so the component crosses that slice inside `K` at one point `x`, on
/// no other box; the walk passes `x` only on its stretch through `K`,
/// which runs from one side of the slice to the other, monotone along
/// `e`, so once. The walk's degree is one.
fn gated(pieces: &[Piece]) -> bool {
    let n = pieces.len();
    n >= 3
        && (0..n).any(|i| {
            let (k, e, _) = pieces[i];
            if !(e.x.is_finite() && e.y.is_finite() && e.z.is_finite()) {
                return false;
            }
            let side = |j: usize| match meet(pieces[j].0, k) {
                Meet::In(m) => along(m, e),
                Meet::Apart | Meet::Unread => None,
            };
            let (Some(a), Some(b)) = (side((i + n - 1) % n), side((i + 1) % n)) else {
                return false;
            };
            let gap = if a.1 < b.0 {
                (a.1, b.0)
            } else if b.1 < a.0 {
                (b.1, a.0)
            } else {
                return false;
            };
            let mut others = Vec::new();
            for j in (0..n).filter(|&j| j != i && j != (i + 1) % n && j != (i + n - 1) % n) {
                match meet(pieces[j].0, k) {
                    Meet::Apart => {}
                    Meet::In(m) => match along(m, e) {
                        Some(r) => others.push(r),
                        None => return false,
                    },
                    Meet::Unread => return false,
                }
            }
            others.sort_by(|p, q| p.0.total_cmp(&q.0));
            let mut from = gap.0;
            for (lo, hi) in others {
                if hi <= from {
                    continue;
                }
                if lo > from {
                    return from < gap.1.min(lo);
                }
                from = hi;
            }
            from < gap.1
        })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::{Box3, Piece, Winding, WindsRefusal, lift, winding};
    use geom::{Curve3, Surface};
    use geom_core::{Interval, Point3, Vec3};

    /// The loop's pieces: each knot span's control hull, padded by `pad`.
    fn pieces(c: &Curve3<f64>, pad: f64) -> Vec<Piece> {
        let Curve3::Nurbs(net) = c else {
            return Vec::new();
        };
        let lifted = net.map_scalar(lift);
        let g = Interval::from_bounds(-pad, pad);
        let knots: Vec<f64> = net.knot_values().runs().map(|(v, _)| v).collect();
        knots
            .windows(2)
            .map(|w| {
                let b = lifted.span_enclosure(w[0], w[1]).expect("a hull");
                (
                    Box3 {
                        x: b.x + g,
                        y: b.y + g,
                        z: b.z + g,
                    },
                    (net.eval(w[1]) - net.eval(w[0])).normalize(),
                    (w[0], w[1]),
                )
            })
            .collect()
    }

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
    /// about the axis, a meridian once about the core, a `(1, 1)` curve
    /// once about each, and a small loop on the tube's outer side about
    /// neither; a longitude run twice round reads its carrier's two
    /// turns.
    #[test]
    fn a_torus_loop_turns_as_it_runs() {
        let (big_r, r) = (2.0, 0.5);
        let on = |u: f64, v: f64| {
            let rho = big_r + r * v.cos();
            Point3::new(rho * u.cos(), rho * u.sin(), r * v.sin())
        };
        let read = |c: &Curve3<f64>| winding(&torus(), c, (0.0, 1.0), &pieces(c, 1e-9));
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

    /// **The tube's own chain reads as the knot spans do**: the winding
    /// over `ssi::certify::tube_chain`'s boxes (the march's handover) of
    /// a longitude, a meridian and a null loop on the torus, at a tube
    /// radius of `1e-3`; and boxes padded past the tube refuse.
    #[test]
    fn the_tube_chain_is_read_as_pieces() {
        let (big_r, r) = (2.0, 0.5);
        let on = |u: f64, v: f64| {
            let rho = big_r + r * v.cos();
            Point3::new(rho * u.cos(), rho * u.sin(), r * v.sin())
        };
        let chain = |c: &Curve3<f64>, radius: f64| match c {
            Curve3::Nurbs(net) => crate::ssi::certify::tube_chain(net, radius),
            _ => Vec::new(),
        };
        for (what, c, want) in [
            ("a longitude", polygon(64, |s| on(s, 0.3)), (1, 0)),
            ("a meridian", polygon(64, |s| on(0.4, s)), (0, 1)),
            (
                "a null loop",
                polygon(32, |s| on(0.1 * s.cos(), 0.2 * s.sin())),
                (0, 0),
            ),
        ] {
            let pieces = chain(&c, 1e-3);
            assert!(pieces.len() >= 32, "{what}: the chain has its boxes");
            assert_eq!(
                winding(&torus(), &c, (0.0, 1.0), &pieces),
                Ok(Winding {
                    about_axis: want.0,
                    about_core: want.1,
                }),
                "{what}"
            );
        }
        let meridian = polygon(64, |s| on(0.4, s));
        assert_eq!(
            winding(&torus(), &meridian, (0.0, 1.0), &chain(&meridian, 0.6)),
            Err(WindsRefusal::Unseparated),
            "a meridian's boxes padded past the tube's radius reach the core"
        );
    }

    /// **A loop whose pieces reach a centre refuses**: a loop round the
    /// cylinder's axis read over boxes padded past its radius; a null
    /// path whose walk is not shown once round refuses `Unordered`; and
    /// pieces that leave a gap in the span refuse.
    #[test]
    fn a_piece_reaching_the_axis_refuses() {
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
            winding(&wall, &ring, (0.0, 1.0), &pieces(&ring, 1e-9)).map(Winding::essential),
            Ok(true),
            "a ring round the wall"
        );
        assert_eq!(
            winding(&wall, &ring, (0.0, 1.0), &pieces(&ring, 0.6)),
            Err(WindsRefusal::Unseparated),
            "boxes past the radius"
        );
        // Out along half the ring and back along the same arc: no turn,
        // and no box clears the axis, but the walk is not shown once
        // round. A thin loop over the same half, out above and back
        // below, is once round and null.
        let half = |s: f64| core::f64::consts::FRAC_PI_2 * (1.0 - s.cos());
        let back = polygon(64, |s| {
            Point3::new(0.5 * half(s).cos(), 0.5 * half(s).sin(), 0.0)
        });
        assert_eq!(
            winding(&wall, &back, (0.0, 1.0), &pieces(&back, 1e-9)),
            Err(WindsRefusal::Unordered),
            "a path out and back along one arc"
        );
        let thin = polygon(64, |s| {
            Point3::new(0.5 * half(s).cos(), 0.5 * half(s).sin(), 0.1 * s.sin())
        });
        assert_eq!(
            winding(&wall, &thin, (0.0, 1.0), &pieces(&thin, 1e-9)),
            Ok(Winding::default()),
            "a thin loop over the half ring"
        );
        let mut gap = pieces(&ring, 1e-9);
        gap.remove(7);
        assert_eq!(
            winding(&wall, &ring, (0.0, 1.0), &gap),
            Err(WindsRefusal::Untiled),
            "a span no piece covers"
        );
    }
}
