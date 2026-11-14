//! **A face's rings in its outer loop's sheet** ([`crate::Body::face_lift`]).
//!
//! Each loop's lift starts on one of its own rows ([`Lift`]: where the
//! chain starts is gauge), so on a chart closed in a channel a ring can
//! be stored whole periods from the region its outer loop's lift bounds.
//! A ring is moved by the one deck translation — a whole number of
//! periods per closed channel — that puts its first point in that
//! region, decided against the outer loop alone, in the chart's cover
//! ([`place_ring`]). One point decides the whole ring: a ring meets no
//! edge of its face, so its lift crosses no row of the outer loop's.
//!
//! The rule is one; where a point lies against the outer loop is read
//! the chart's way, as a joint is (`decide_joint` reads a spline chart's
//! joints apart from an analytic chart's). A spline chart's reading is
//! its trim's (`crate::boolean::spline_region`): the ray parity over the
//! rows' Bézier pieces, metered by the chart's cells. An analytic
//! chart's is [`Outer::side`]: the same parity over the lifted rows'
//! chart boxes ([`Pcurve::chart_box`]), every clearance decided in
//! metres through the chart's inf-side arms, so a point within the band
//! of the outer loop, along the chart, is undecided rather than read on
//! a side.

use geom::Surface;
use geom_brep::{ChartWindow, Pcurve};
use geom_core::predicate::Band;
use geom_core::{Decide, Indeterminate, InfSpeed, Margin, Point2, Real, Sign};

use super::{ChartPeriods, Lift, LiftGap, polar_arm};
use crate::body::Body;
use crate::entity::{FaceKey, LoopKey};
use crate::joint::{Deck, Winding};
use crate::validate::decide;

/// A point's clearance from an analytic face's outer loop, along the
/// chart.
pub(crate) const RING_PLACE_CLEAR: &str = "ring_place_clear";
/// An analytic chart's inf-side lever arm over a window, gated as a
/// length.
pub(crate) const RING_PLACE_ARM: &str = "ring_place_arm";

/// The shifts one closed channel may try.
const SHIFTS: usize = 9;
/// How many times a piece of the outer loop is halved before a point it
/// is not clear of reads near it.
const PIECE_DEPTH: usize = 52;
/// The piece visits one reading may make.
const PIECE_VISITS: usize = 1 << 14;
/// How many times the window round a point is halved for its arms.
const HALVINGS: u32 = 6;

/// **A face's loops in one sheet** ([`crate::Body::face_lift`]): its
/// outer loop's lift as read, and each ring's moved into the region the
/// outer loop's bounds.
#[derive(Clone, Debug)]
pub struct FaceLift<T: Real> {
    outer: Lift<T>,
    rings: Vec<PlacedRing<T>>,
}

impl<T: Real> FaceLift<T> {
    /// The outer loop's lift, as [`crate::Body::loop_lift`] reads it.
    #[must_use]
    pub fn outer(&self) -> &Lift<T> {
        &self.outer
    }

    /// The rings, in the face's loop order after its outer loop.
    #[must_use]
    pub fn rings(&self) -> &[PlacedRing<T>] {
        &self.rings
    }
}

/// One ring of a [`FaceLift`].
#[derive(Clone, Debug)]
pub struct PlacedRing<T: Real> {
    /// The ring.
    pub r#loop: LoopKey,
    /// The deck translation that moved the ring's lift
    /// ([`crate::Body::loop_lift`]) into its outer loop's region.
    pub by: Deck,
    /// The moved lift: each row's lift is `by` composed after the one
    /// [`crate::Body::loop_lift`] reads.
    pub lift: Lift<T>,
}

/// Why a face's rings have no place in its outer loop's sheet
/// ([`crate::Body::face_lift`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RingPlacementError {
    /// A loop has no lift to read.
    Gap(LiftGap),
    /// The outer loop does not close in its chart (a seamless band):
    /// its lift bounds no region to place a ring in.
    OuterOpen {
        /// How far it winds.
        winding: Winding,
    },
    /// A ring does not close in its chart.
    RingOpen {
        /// The ring.
        ring: LoopKey,
        /// How far it winds.
        winding: Winding,
    },
    /// No whole-period shift puts the ring's first point in the outer
    /// loop's region.
    NoShift {
        /// The ring.
        ring: LoopKey,
    },
    /// More than one does.
    Shifts {
        /// The ring.
        ring: LoopKey,
    },
    /// The outer loop's reach spans more shifts of a channel than the
    /// rule tries.
    Copies {
        /// The ring.
        ring: LoopKey,
    },
    /// A shift's point is within the band of the outer loop along the
    /// chart, or its reading ran past its caps: the in-band decision, if
    /// one stopped it.
    Undecided {
        /// The ring.
        ring: LoopKey,
        /// The in-band decision.
        cause: Option<Indeterminate>,
    },
    /// A spline chart's trim reads no piece of a row, or the chart's
    /// periods as stated and as decided part (`spline_region`'s
    /// `Trim`).
    Unread,
}

/// Why [`place_ring`] placed no ring.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum RingMiss {
    /// No shift places it.
    NoShift,
    /// Two do.
    Shifts,
    /// Too many shifts to try.
    Copies,
    /// A shift's reading is undecided.
    Undecided(Option<Indeterminate>),
}

impl RingMiss {
    /// The door's error for `ring`.
    pub(crate) fn of(self, ring: LoopKey) -> RingPlacementError {
        match self {
            Self::NoShift => RingPlacementError::NoShift { ring },
            Self::Shifts => RingPlacementError::Shifts { ring },
            Self::Copies => RingPlacementError::Copies { ring },
            Self::Undecided(cause) => RingPlacementError::Undecided { ring, cause },
        }
    }
}

/// Where a point lies against the outer loop, in the cover.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Side {
    /// Inside, decided.
    In,
    /// Outside, decided.
    Out,
    /// Undecided: the in-band decision, if one stopped the reading.
    Near(Option<Indeterminate>),
}

/// One candidate move of a ring: its deck translation, and the chart
/// offset it is in `f64`, `k·p` per closed channel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Shift {
    /// Whole periods per channel.
    pub(crate) deck: Deck,
    /// `(k_u·p_u, k_v·p_v)`.
    pub(crate) offset: [f64; 2],
}

/// **The ring rule**: the one shift that places a ring whose first point
/// is `at`, against an outer loop whose chart hull is `reach`, on a chart
/// whose closed channels have `periods`. On each closed channel the
/// candidates are the whole numbers of periods that bring `at` within
/// `reach`; each candidate is read by `side`, in order, and the first
/// reading that is neither a first `In` nor an `Out` ends the rule.
///
/// # Errors
///
/// [`RingMiss`]: no candidate reads in, a second one does, one is
/// undecided, or a channel has more than [`SHIFTS`] candidates.
pub(crate) fn place_ring(
    periods: [Option<f64>; 2],
    reach: [(f64, f64); 2],
    at: [f64; 2],
    mut side: impl FnMut(Shift) -> Side,
) -> Result<Shift, RingMiss> {
    let shifts = |k: usize| -> Option<Vec<(i32, f64)>> {
        let Some(p) = periods[k].filter(|p| *p > 0.0) else {
            return Some(vec![(0, 0.0)]);
        };
        let (r0, r1) = reach[k];
        let (lo, hi) = (((r0 - at[k]) / p).floor(), ((r1 - at[k]) / p).ceil());
        let n = (hi - lo).max(0.0);
        if n > SHIFTS as f64 {
            return None;
        }
        #[allow(clippy::cast_possible_truncation)] // within `SHIFTS` of a floor
        Some(
            (0..=n as usize)
                .map(|i| {
                    let m = lo + i as f64;
                    (m as i32, m * p)
                })
                .collect(),
        )
    };
    let (Some(us), Some(vs)) = (shifts(0), shifts(1)) else {
        return Err(RingMiss::Copies);
    };
    let mut found = None;
    for &(ku, du) in &us {
        for &(kv, dv) in &vs {
            let shift = Shift {
                deck: Deck {
                    u: ku,
                    v: kv,
                    twin: false,
                },
                offset: [du, dv],
            };
            match side(shift) {
                Side::In if found.is_none() => found = Some(shift),
                Side::Out => {}
                Side::In => return Err(RingMiss::Shifts),
                Side::Near(cause) => return Err(RingMiss::Undecided(cause)),
            }
        }
    }
    found.ok_or(RingMiss::NoShift)
}

/// **The door** ([`crate::Body::face_lift`]).
#[track_caller]
pub(crate) fn face_lift<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<FaceLift<T>, RingPlacementError> {
    let view = body.proven_face(face);
    let mut loops = view.loops().map(|(lk, _, _)| lk);
    let Some(outer_loop) = loops.next() else {
        unreachable!("a face's record holds its outer loop")
    };
    let rings: Vec<LoopKey> = loops.collect();
    let outer = Lift::read(body, outer_loop).map_err(RingPlacementError::Gap)?;
    let winding = outer
        .winding()
        .unwrap_or_else(|| unreachable!("a read lift is a whole loop"));
    if !winding.is_zero() {
        return Err(RingPlacementError::OuterOpen { winding });
    }
    let mut read = Vec::with_capacity(rings.len());
    for &ring in &rings {
        let lift = Lift::read(body, ring).map_err(RingPlacementError::Gap)?;
        let winding = lift
            .winding()
            .unwrap_or_else(|| unreachable!("a read lift is a whole loop"));
        if !winding.is_zero() {
            return Err(RingPlacementError::RingOpen { ring, winding });
        }
        read.push(lift);
    }
    if rings.is_empty() {
        return Ok(FaceLift {
            outer,
            rings: Vec::new(),
        });
    }
    let surface = view.surface();
    let decks = if surface.spline_chart().is_some() {
        crate::boolean::spline_region::ring_decks(body, face, band)?
    } else {
        let periods = ChartPeriods::of(surface, band);
        let reader = Outer::of(&outer, surface);
        let lengths = [periods.u(), periods.v()].map(|p| p.map(|p| p.length().value_channel()));
        let mut decks = Vec::with_capacity(rings.len());
        for (&ring, lift) in rings.iter().zip(&read) {
            let at = first_point(lift);
            let shift = place_ring(
                lengths,
                reader.reach,
                [at.x.value_channel(), at.y.value_channel()],
                |shift| {
                    let moved = |k: i32, p: Option<geom_brep::Period<T>>| {
                        p.map_or_else(T::zero, |p| p.times(k))
                    };
                    let q = Point2::new(
                        at.x + moved(shift.deck.u, periods.u()),
                        at.y + moved(shift.deck.v, periods.v()),
                    );
                    reader.side(q, band)
                },
            )
            .map_err(|miss| miss.of(ring))?;
            decks.push(shift.deck);
        }
        decks
    };
    let rings = rings
        .into_iter()
        .zip(decks)
        .map(|(ring, by)| {
            let lift = Lift::read_from(body, ring, by).map_err(RingPlacementError::Gap)?;
            Ok(PlacedRing {
                r#loop: ring,
                by,
                lift,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(FaceLift { outer, rings })
}

/// The point a lift's chain starts at: its first row's entry.
fn first_point<T: Decide>(lift: &Lift<T>) -> Point2<T> {
    let Some(row) = lift.rows().first() else {
        unreachable!("a ring's loop holds a half-edge")
    };
    row.pcurve.eval(row.entry_exit().0)
}

/// One piece of an outer loop's chain: a span of a lifted row's image,
/// or the straight joint between two rows' ends (a joint that closes
/// within the band, or a run along a pole or apex row).
#[derive(Clone)]
enum Piece<'a, T: Real> {
    Image { pcurve: &'a Pcurve<T>, span: (T, T) },
    Segment(Point2<T>, Point2<T>),
}

impl<T: Decide> Piece<'_, T> {
    /// Its two ends, in the order its span runs. Bit-identical wherever
    /// two pieces share one: each is read off the same parameter of the
    /// same image, or copied.
    fn ends(&self) -> (Point2<T>, Point2<T>) {
        match self {
            Piece::Image { pcurve, span } => (pcurve.eval(span.0), pcurve.eval(span.1)),
            Piece::Segment(a, b) => (*a, *b),
        }
    }

    /// A box holding every point of the piece.
    fn bbox(&self) -> ChartWindow<T> {
        match self {
            Piece::Image { pcurve, span } => pcurve.chart_box(span.0, span.1),
            Piece::Segment(a, b) => ChartWindow {
                u_min: a.x.min(b.x),
                u_max: a.x.max(b.x),
                v_min: a.y.min(b.y),
                v_max: a.y.max(b.y),
            },
        }
    }

    fn halves(&self) -> (Self, Self) {
        let half = T::from_f64(0.5);
        match self {
            Piece::Image { pcurve, span } => {
                let mid = span.0 + (span.1 - span.0) * half;
                (
                    Piece::Image {
                        pcurve,
                        span: (span.0, mid),
                    },
                    Piece::Image {
                        pcurve,
                        span: (mid, span.1),
                    },
                )
            }
            Piece::Segment(a, b) => {
                let m = Point2::new(a.x + (b.x - a.x) * half, a.y + (b.y - a.y) * half);
                (Piece::Segment(*a, m), Piece::Segment(m, *b))
            }
        }
    }
}

/// **An analytic face's outer loop as the ring rule reads it**: its
/// lift's rows and the joints between them, closed.
struct Outer<'a, T: Real> {
    pieces: Vec<Piece<'a, T>>,
    surface: &'a Surface<T>,
    /// The chain's hull, `[u, v]`.
    reach: [(f64, f64); 2],
}

impl<'a, T: Decide> Outer<'a, T> {
    fn of(lift: &'a Lift<T>, surface: &'a Surface<T>) -> Self {
        let mut pieces = Vec::with_capacity(2 * lift.rows().len());
        let ends: Vec<(Point2<T>, Point2<T>)> = lift
            .rows()
            .iter()
            .map(|row| {
                let (entry, exit) = row.entry_exit();
                (row.pcurve.eval(entry), row.pcurve.eval(exit))
            })
            .collect();
        for (i, row) in lift.rows().iter().enumerate() {
            pieces.push(Piece::Image {
                pcurve: &row.pcurve,
                span: row.params,
            });
            let next = ends[(i + 1) % ends.len()].0;
            pieces.push(Piece::Segment(ends[i].1, next));
        }
        let mut reach = [(f64::INFINITY, f64::NEG_INFINITY); 2];
        for piece in &pieces {
            let b = piece.bbox();
            reach[0] = (
                reach[0].0.min(b.u_min.value_channel()),
                reach[0].1.max(b.u_max.value_channel()),
            );
            reach[1] = (
                reach[1].0.min(b.v_min.value_channel()),
                reach[1].1.max(b.v_max.value_channel()),
            );
        }
        Self {
            pieces,
            surface,
            reach,
        }
    }

    /// **Where `q` lies against the chain, in the cover**: `In` or `Out`
    /// only where every piece is decided clear of it along the chart past
    /// the band. A window round `q` is taken whose chart arms gate
    /// definitely positive, halved until they do; every way out of it is
    /// decided past the band, so a piece's gap inside it, metered by the
    /// window's arms, lower-bounds the way to the piece.
    fn side(&self, q: Point2<T>, band: Band) -> Side {
        let size = self.reach.map(|(lo, hi)| hi - lo);
        let mut pending = None;
        for halving in 0..HALVINGS {
            let scale = 16.0 * f64::from(1u32 << halving);
            let (ru, rv) = (
                T::from_computed(size[0] / scale),
                T::from_computed(size[1] / scale),
            );
            let arms = match self.arms(q.y - rv, q.y + rv, band) {
                Ok(arms) => arms,
                Err(diag) => {
                    pending = pending.or(diag);
                    continue;
                }
            };
            for (room, arm) in [(ru, arms.0), (rv, arms.1)] {
                match decide(RING_PLACE_CLEAR, Margin::metered(room, arm), band) {
                    Ok(Sign::Positive) => {}
                    Ok(_) => return Side::Near(pending),
                    Err(diag) => return Side::Near(Some(diag)),
                }
            }
            return self.parity(q, arms, band);
        }
        Side::Near(pending)
    }

    /// The chart's inf-side arms over the `v` window `[v_lo, v_hi]`, each
    /// gated definitely positive: the azimuth's least lever there
    /// ([`geom_brep::azimuth_lever_inf`]) and the second channel's exact
    /// rate (the polar radius where `v` is an angle, 1 where it is a
    /// length); a plane's coordinates are metres.
    fn arms(
        &self,
        v_lo: T,
        v_hi: T,
        band: Band,
    ) -> Result<(InfSpeed<T>, InfSpeed<T>), Option<Indeterminate>> {
        let (u, v) = match *self.surface {
            Surface::Plane { .. } => (T::one(), T::one()),
            Surface::Nurbs(_) | Surface::Approx(_) => {
                unreachable!("a spline chart's rings are placed by its trim")
            }
            _ => (
                geom_brep::azimuth_lever_inf(self.surface, v_lo, v_hi).get(),
                polar_arm(self.surface).unwrap_or_else(T::one),
            ),
        };
        let floor = |x: T| x.structure_bracket().map_or(f64::NAN, |(lo, _)| lo);
        let gate = |arm: T| {
            crate::chart_region::gate_arm_as(RING_PLACE_ARM, arm, floor(arm), band).map_err(
                |gate| match gate {
                    crate::chart_region::ArmGate::Unbounded => None,
                    crate::chart_region::ArmGate::Escalated(diag) => Some(diag),
                },
            )
        };
        Ok((InfSpeed::new(gate(u)?), InfSpeed::new(gate(v)?)))
    }

    /// The ray parity of `q` along `+u`, every piece decided clear of it
    /// first: a clear piece wholly past `q` in `u` crosses the ray as
    /// often, mod 2, as its ends lie either side of `v = q_v` under the
    /// half-open rule `v ≥ q_v`; any other clear piece misses it. A piece
    /// not clear is halved.
    fn parity(&self, q: Point2<T>, (au, av): (InfSpeed<T>, InfSpeed<T>), band: Band) -> Side {
        let (cu, cv) = (q.x.value_channel(), q.y.value_channel());
        let mut stack: Vec<(Piece<'_, T>, usize)> =
            self.pieces.iter().rev().map(|p| (p.clone(), 0)).collect();
        let (mut parity, mut visits) = (false, 0usize);
        while let Some((piece, depth)) = stack.pop() {
            visits += 1;
            if visits > PIECE_VISITS {
                return Side::Near(None);
            }
            let b = piece.bbox();
            let gu = (b.u_min - q.x).max(q.x - b.u_max).max(T::zero());
            let gv = (b.v_min - q.y).max(q.y - b.v_max).max(T::zero());
            let mut diag = None;
            let mut clear = false;
            for (g, arm) in [(gu, au), (gv, av)] {
                match decide(RING_PLACE_CLEAR, Margin::metered(g, arm), band) {
                    Ok(Sign::Positive) => clear = true,
                    Ok(_) => {}
                    Err(d) => diag = diag.or(Some(d)),
                }
            }
            if clear {
                if b.v_min.value_channel() > cv
                    || b.v_max.value_channel() < cv
                    || b.u_max.value_channel() < cu
                {
                    continue;
                }
                if b.u_min.value_channel() > cu {
                    let (s, e) = piece.ends();
                    parity ^= (s.y.value_channel() >= cv) != (e.y.value_channel() >= cv);
                    continue;
                }
            }
            if depth == PIECE_DEPTH {
                return Side::Near(diag);
            }
            let (l, r) = piece.halves();
            stack.push((r, depth + 1));
            stack.push((l, depth + 1));
        }
        if parity { Side::In } else { Side::Out }
    }
}

#[cfg(test)]
mod tests {
    use super::{RingMiss, Shift, Side, place_ring};

    /// The rule over a scripted reading: the shifts it tries and what it
    /// answers.
    fn run(
        periods: [Option<f64>; 2],
        reach: [(f64, f64); 2],
        at: [f64; 2],
        read: impl Fn([i32; 2]) -> Side,
    ) -> (Result<[i32; 2], RingMiss>, Vec<[i32; 2]>) {
        let mut tried = Vec::new();
        let out = place_ring(periods, reach, at, |s: Shift| {
            let k = [s.deck.u, s.deck.v];
            tried.push(k);
            read(k)
        })
        .map(|s| [s.deck.u, s.deck.v]);
        (out, tried)
    }

    const TAU: f64 = core::f64::consts::TAU;

    #[test]
    fn one_shift_in_places_the_ring_and_its_offset_is_whole_periods() {
        let (out, tried) = run(
            [Some(TAU), None],
            [(3.1, 6.3), (0.0, 1.0)],
            [-1.5, 0.5],
            |k| if k == [1, 0] { Side::In } else { Side::Out },
        );
        assert_eq!(out, Ok([1, 0]), "the one move in; tried {tried:?}");
        assert!(
            tried.contains(&[0, 0]) && tried.iter().all(|k| k[1] == 0),
            "{tried:?}"
        );
        let offset = place_ring(
            [Some(TAU), None],
            [(3.1, 6.3), (0.0, 1.0)],
            [-1.5, 0.5],
            |s| {
                if s.deck.u == 1 { Side::In } else { Side::Out }
            },
        )
        .map(|s| s.offset);
        assert_eq!(
            offset,
            Ok([TAU, 0.0]),
            "the offset is k periods, and none on an open channel"
        );
    }

    #[test]
    fn no_shift_two_shifts_and_an_undecided_one_answer_typed() {
        let reach = [(0.0, TAU), (0.0, TAU)];
        let periods = [Some(TAU), Some(TAU)];
        let (none, _) = run(periods, reach, [1.0, 1.0], |_| Side::Out);
        assert_eq!(none, Err(RingMiss::NoShift), "no move places it");
        let (two, _) = run(periods, reach, [1.0, 1.0], |k| {
            if k[1] == 0 { Side::In } else { Side::Out }
        });
        assert_eq!(two, Err(RingMiss::Shifts), "two moves place it");
        let (near, _) = run(periods, reach, [1.0, 1.0], |k| {
            if k == [0, 0] {
                Side::In
            } else {
                Side::Near(None)
            }
        });
        assert_eq!(near, Err(RingMiss::Undecided(None)), "a move in band");
        let (wide, tried) = run(
            [Some(0.1), None],
            [(0.0, 5.0), (0.0, 1.0)],
            [0.0, 0.0],
            |_| Side::In,
        );
        assert_eq!(
            (wide, tried.len()),
            (Err(RingMiss::Copies), 0),
            "more shifts than the rule tries"
        );
    }

    #[test]
    fn a_chart_with_no_closed_channel_reads_the_ring_where_it_is() {
        let (out, tried) = run([None, None], [(0.0, 1.0), (0.0, 1.0)], [9.0, 9.0], |_| {
            Side::In
        });
        assert_eq!(
            (out, tried),
            (Ok([0, 0]), vec![[0, 0]]),
            "one candidate, read"
        );
    }
}
