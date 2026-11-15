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
//! The rule is one, and so is the ray parity that reads a point against
//! the outer loop ([`ray_parity_as`]); each chart family supplies its
//! pieces' boxes and the window's arms. A spline chart's are its trim's
//! ([`crate::spline_trim`], `Trim::place_in_cover`): the rows' Bézier
//! pieces, metered by the chart's cells, which the spline trim places
//! its own rings by too. An analytic chart's are [`Outer::side`]'s: the
//! lifted rows' chart boxes ([`Pcurve::chart_box`]), every clearance
//! decided in metres through the chart's inf-side arms, so a point
//! within the band of the outer loop, along the chart, is undecided
//! rather than read on a side.
//! The work bounds below are the one home of both readers' bounds.
//!
//! **A row whose box does not tighten.** An `IsoArc`'s box is its whole
//! segment's, a `Fitted` or `General` image's its whole net's, and a
//! `Spiric` image's its whole period's ([`Pcurve::chart_box`]), so
//! halving such a piece never clears it of a point inside that box: the
//! reading spends [`PIECE_VISITS`] and answers `Near`, so the ring
//! answers [`RingPlacementError::Undecided`]. The measured outer loops
//! are all `Harmonic`
//! (`work/tesslane/an-analytic-outer-loops-whole-span-boxes-never-tighten-under-the-ring-reader.md`).

use geom::Surface;
use geom_brep::{ChartWindow, Pcurve};
use geom_core::predicate::Band;
use geom_core::{Decide, Indeterminate, InfSpeed, Margin, Point2, Real, Sign};

use super::{ChartPeriods, Lift, LiftGap, polar_arm};
use crate::body::Body;
use crate::chain_parity::{ChainPiece, PieceBox, RayQuery, ray_parity_as};
use crate::entity::{FaceKey, LoopKey};
use crate::joint::{Deck, Winding};
use crate::validate::decide;

/// A point's clearance from an analytic face's outer loop, along the
/// chart.
pub(crate) const RING_PLACE_CLEAR: &str = "ring_place_clear";
/// An analytic chart's inf-side lever arm over a window, gated as a
/// length.
pub(crate) const RING_PLACE_ARM: &str = "ring_place_arm";

/// A work bound, not a fact about the face, on two different counts:
/// the shifts one closed channel may try in [`place_ring`], where a
/// channel past it answers [`RingMiss::Copies`]; and the product of a
/// query's copies over both channels that the spline trim reads
/// (`crate::spline_trim`'s `Trim::copies`, which reads it as `COPIES`),
/// where a product past it reads `Near(None)`. Raising it
/// moves both.
pub(crate) const SHIFTS: usize = 9;
/// How many times a piece of an outer loop is halved before a point it
/// is not clear of reads near it: a work bound, read as `Near`.
pub(crate) const PIECE_DEPTH: usize = 52;
/// The piece visits one reading may make: a work bound, read as `Near`.
pub(crate) const PIECE_VISITS: usize = 1 << 14;
/// How many times the window round a point is halved for its arms: a
/// work bound, read as `Near` where no window's arms gate positive.
pub(crate) const HALVINGS: u32 = 6;
/// The first window round a point is this share of the reach in each
/// channel.
pub(crate) const WINDOW_SHARE: f64 = 16.0;

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

/// One ring of a [`FaceLift`]: a placement the door decided, so it is
/// built by the door alone.
#[derive(Clone, Debug)]
pub struct PlacedRing<T: Real> {
    r#loop: LoopKey,
    by: Deck,
    lift: Lift<T>,
}

impl<T: Real> PlacedRing<T> {
    /// The ring.
    #[must_use]
    pub fn ring(&self) -> LoopKey {
        self.r#loop
    }

    /// The deck translation that moved the ring's lift
    /// ([`crate::Body::loop_lift`]) into its outer loop's region.
    #[must_use]
    pub fn by(&self) -> Deck {
        self.by
    }

    /// The moved lift: each row's lift is [`Self::by`] composed after
    /// the one [`crate::Body::loop_lift`] reads, and each image moved
    /// by that sum from the stored row ([`Lift::read_from`]), so it is
    /// rounded afresh, not the read image plus `k·p` bit for bit.
    #[must_use]
    pub fn lift(&self) -> &Lift<T> {
        &self.lift
    }
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
    /// A ring does not close in its chart. A guard no body reaches: an
    /// outer loop that winds zero bounds a disc with holes in the lifted
    /// sheet, so every ring inside it winds zero too. A ring that winds
    /// needs an outer loop that winds, and the door answers
    /// [`Self::OuterOpen`] first.
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
    /// rule tries ([`SHIFTS`], a work bound), or a shift's count of
    /// periods has no `i32`.
    Copies {
        /// The ring.
        ring: LoopKey,
    },
    /// A shift's point is within the band of the outer loop along the
    /// chart, or its reading ran past its work bounds (a row whose box
    /// does not tighten, module docs), or no window round it has arms
    /// (a sphere ring stored on the twin sheet, past the pole rows): the
    /// in-band decision, if one stopped it.
    Undecided {
        /// The ring.
        ring: LoopKey,
        /// The in-band decision.
        cause: Option<Indeterminate>,
    },
    /// The face's chart is a spline and its trim is not read there
    /// ([`SplineTrimGap`]): the outer loop's (`ring` is `None`), or a
    /// ring's chain.
    SplineTrimUnread {
        /// The ring whose chain is not read; `None` for the outer loop's
        /// trim or the chart.
        ring: Option<LoopKey>,
        /// Why.
        cause: SplineTrimGap,
    },
}

/// Why a spline face's trim is not read
/// ([`RingPlacementError::SplineTrimUnread`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplineTrimGap {
    /// The chart is a placeholder, mid-operation.
    Placeholder,
    /// The chart has no Bézier cells: a degree of zero, a knot past its
    /// degree's multiplicity or with no value, a weight not shown
    /// positive, or an insertion plan that refuses.
    NoCells,
    /// A channel the chart is decided closed in states no period, or
    /// another one.
    Periods,
    /// A loop's lift bounds no region: it is empty, lacks a joint
    /// element, or winds.
    NoClosedLift,
    /// A row is moved by a period on a channel the chart states but does
    /// not decide closed, or onto the twin sheet.
    Moved,
    /// A row's image is of a kind the trim reads no pieces of, or its
    /// Bézier decomposition does not run.
    NoPieces,
    /// A piece's box has no value.
    NoBox,
}

/// Why [`place_ring`] placed no ring: the rule's answer, before a ring
/// key names it, since the spline trim reads the rule with no door to
/// answer through ([`RingMiss::of`] is the door's).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum RingMiss {
    /// No shift places it.
    NoShift,
    /// Two do.
    Shifts,
    /// Too many shifts to try, or a count with no `i32`.
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

/// Where a query lies against a closed chain in the chart's cover: an
/// outer loop the ring rule reads, or a spline face's trim.
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
/// undecided, or a channel has more than [`SHIFTS`] candidates or a
/// count of periods past `i32`.
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
        (0..=n as usize)
            .map(|i| {
                let m = lo + i as f64;
                Some((whole(m)?, m * p))
            })
            .collect()
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

/// `m`, a whole number, as an `i32`, where it has one.
fn whole(m: f64) -> Option<i32> {
    // `i32::MIN` and `i32::MAX + 1` are exact in `f64`.
    #[allow(clippy::cast_possible_truncation)] // in range, and whole
    (m >= f64::from(i32::MIN) && m < -f64::from(i32::MIN)).then_some(m as i32)
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
        let rings: Vec<(LoopKey, &Lift<T>)> = rings.iter().copied().zip(&read).collect();
        crate::spline_trim::ring_decks(body, face, &outer, &rings, band)?
    } else {
        let periods = ChartPeriods::of(surface, band);
        let reader = Outer::of(&outer, surface, periods.v());
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
    // A ring that stays is the lift already read; a moved one is read
    // again in its new gauge, from the stored rows.
    let rings = rings
        .into_iter()
        .zip(read)
        .zip(decks)
        .map(|((ring, lift), by)| {
            let lift = if by == Deck::IDENTITY {
                lift
            } else {
                Lift::read_from(body, ring, by).map_err(RingPlacementError::Gap)?
            };
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
}

impl<T: Decide> ChainPiece<T> for Piece<'_, T> {
    type Query = Point2<T>;

    /// A point: its ray leaves from it, and it is a rectangle of size
    /// zero.
    fn ray(q: Point2<T>) -> RayQuery {
        let [u, v] = [q.x.value_channel(), q.y.value_channel()];
        RayQuery {
            centre: [u, v],
            rect: [(u, u), (v, v)],
        }
    }

    fn boxed(&self, q: Point2<T>) -> Option<PieceBox<T>> {
        let b = self.bbox();
        Some(PieceBox {
            gap: [
                (b.u_min - q.x).max(q.x - b.u_max).max(T::zero()),
                (b.v_min - q.y).max(q.y - b.v_max).max(T::zero()),
            ],
            extent: [
                (b.u_min.value_channel(), b.u_max.value_channel()),
                (b.v_min.value_channel(), b.v_max.value_channel()),
            ],
        })
    }

    /// A shared end is bit-identical: the halves of an image span share
    /// the one midpoint parameter and read it off the one image, and a
    /// joint's segment copies the two row ends [`Outer::of`] evaluated
    /// at the parameters the rows' own spans end at.
    fn ends(&self) -> ([f64; 2], [f64; 2]) {
        let (s, e) = match self {
            Piece::Image { pcurve, span } => (pcurve.eval(span.0), pcurve.eval(span.1)),
            Piece::Segment(a, b) => (*a, *b),
        };
        (
            [s.x.value_channel(), s.y.value_channel()],
            [e.x.value_channel(), e.y.value_channel()],
        )
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
    /// The second channel's period, where the chart is closed in it.
    v_period: Option<geom_brep::Period<T>>,
    /// The chain's hull, `[u, v]`.
    reach: [(f64, f64); 2],
}

impl<'a, T: Decide> Outer<'a, T> {
    fn of(
        lift: &'a Lift<T>,
        surface: &'a Surface<T>,
        v_period: Option<geom_brep::Period<T>>,
    ) -> Self {
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
                span: row.params.ends(),
            });
            let next = ends[(i + 1) % ends.len()].0;
            pieces.push(Piece::Segment(ends[i].1, next));
        }
        Self::from_pieces(pieces, surface, v_period)
    }

    /// The reader of the closed chain `pieces`.
    fn from_pieces(
        pieces: Vec<Piece<'a, T>>,
        surface: &'a Surface<T>,
        v_period: Option<geom_brep::Period<T>>,
    ) -> Self {
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
            v_period,
            reach,
        }
    }

    /// **Where `q` lies against the chain, in the cover**: `In` or `Out`
    /// only where every piece is decided clear of it along the chart past
    /// the band. A window round `q` is taken whose chart arms gate
    /// definitely positive, halved until they do; every way out of it is
    /// decided past the band, so a piece's gap inside it, metered by the
    /// window's arms, lower-bounds the way to the piece. The arms are
    /// read at `q`'s copy within half a `v` period of 0: a deck
    /// translation is an isometry of the chart.
    fn side(&self, q: Point2<T>, band: Band) -> Side {
        let size = self.reach.map(|(lo, hi)| hi - lo);
        let v = match self.v_period {
            None => q.y,
            Some(p) => {
                let length = p.length().value_channel();
                let Some(k) = whole((q.y.value_channel() / length + 0.5).floor()) else {
                    return Side::Near(None);
                };
                q.y - p.times(k)
            }
        };
        let mut pending = None;
        for halving in 0..HALVINGS {
            let scale = WINDOW_SHARE * f64::from(1u32 << halving);
            let (ru, rv) = (
                T::from_computed(size[0] / scale),
                T::from_computed(size[1] / scale),
            );
            let arms = match self.arms(v - rv, v + rv, band) {
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
            return ray_parity_as(RING_PLACE_CLEAR, &self.pieces, q, arms, PIECE_VISITS, band);
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
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use core::f64::consts::{FRAC_PI_2, PI, TAU};

    use geom::Surface;
    use geom_brep::Period;
    use geom_core::predicate::Band;
    use geom_core::{Point2, Point3, Tol, Vec3};

    use super::{Outer, Piece, RingMiss, Shift, Side, place_ring, whole};

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

    /// The second `In` is scripted: no outer loop the readers read gives
    /// two copies of one point inside it, so a rule that took the first
    /// `In` and went on is red here alone.
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

    /// The rectangle `u ∈ [π, 2π]`, `v ∈ [−1.2, 1.2]` on the unit
    /// sphere's chart, closed in both channels, as straight pieces.
    fn sphere_half_band(surface: &Surface<f64>) -> Outer<'_, f64> {
        let corners = [(PI, -1.2), (TAU, -1.2), (TAU, 1.2), (PI, 1.2)];
        let at = |i: usize| Point2::new(corners[i % 4].0, corners[i % 4].1);
        let pieces = (0..4).map(|i| Piece::Segment(at(i), at(i + 1))).collect();
        Outer::from_pieces(pieces, surface, Some(Period::new(TAU)))
    }

    #[test]
    fn an_analytic_outer_loop_reads_a_point_in_out_or_near_and_a_copy_past_the_pole() {
        let eps = Tol::witness().eps();
        let sphere = Surface::Sphere {
            center: Point3::origin(),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let outer = sphere_half_band(&sphere);
        let band = Band::linear(Tol::witness()).unwrap();
        let read = |u: f64, v: f64| match outer.side(Point2::new(u, v), band) {
            Side::In => "in",
            Side::Out => "out",
            Side::Near(_) => "near",
        };
        assert_eq!(
            [
                read(4.5, 0.3),
                read(1.0, 0.3),
                read(4.5 + TAU, 0.3),
                read(4.5, 0.3 + TAU),
                read(PI + 1e-3 * eps, 0.3),
            ],
            ["in", "out", "out", "out", "near"],
            "inside; a u period off; past the outer loop's reach; a v period off, its window \
             read between the pole rows; within the band of the outer loop"
        );
        let placed = place_ring([Some(TAU), Some(TAU)], outer.reach, [4.5 - TAU, 0.3], |s| {
            let [du, dv] = s.offset;
            outer.side(Point2::new(4.5 - TAU + du, 0.3 + dv), band)
        })
        .map(|s| [s.deck.u, s.deck.v]);
        assert_eq!(
            placed,
            Ok([1, 0]),
            "a ring stored a u period below is moved up one"
        );
    }

    /// The closed chain through `corners` on `surface`'s chart, as
    /// straight pieces, closed in `v` by `v_period` where given.
    fn chain<'a>(
        surface: &'a Surface<f64>,
        corners: &[(f64, f64)],
        v_period: Option<f64>,
    ) -> Outer<'a, f64> {
        let n = corners.len();
        let at = |i: usize| Point2::new(corners[i % n].0, corners[i % n].1);
        let pieces = (0..n).map(|i| Piece::Segment(at(i), at(i + 1))).collect();
        Outer::from_pieces(pieces, surface, v_period.map(Period::new))
    }

    fn word(side: Side) -> &'static str {
        match side {
            Side::In => "in",
            Side::Out => "out",
            Side::Near(_) => "near",
        }
    }

    fn unit_sphere() -> Surface<f64> {
        Surface::Sphere {
            center: Point3::origin(),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// **The analytic reader against chains a bug in its clearance or its
    /// parity would misread.** A clearance that counts any positive chart
    /// gap as clear reads the points a thousandth of `ε` off an edge, on
    /// an edge, and in the sliver thinner than the band as a side; each
    /// must be near. The offsets are in the run's `ε`, so the rows read
    /// the same at every eps row.
    /// A parity that counts a grazing ray or a vertex twice misreads the
    /// notch's iso row and the diamond's vertices.
    ///
    /// Two near readings are the reader being conservative, not the
    /// geometry being in band: a point `1e-4` below a sphere's pole row,
    /// and one `1e-3` above a cone's apex row. The window must hold a
    /// positive azimuth arm, and no window it halves to (the reach over
    /// `16·2⁵`) stays off the pole or the apex there.
    #[test]
    fn the_analytic_reader_reads_notches_vertices_slivers_poles_and_apexes() {
        let band = Band::linear(Tol::witness()).unwrap();
        // Offsets in the run's ε, which the eps rows sweep: well past the
        // band on the clear side, well inside it on the other.
        let eps = Tol::witness().eps();
        let (clear, inside) = (1e3 * eps, 1e-3 * eps);
        let sphere = unit_sphere();
        let read = |o: &Outer<'_, f64>, u: f64, v: f64| word(o.side(Point2::new(u, v), band));
        let notch = chain(
            &sphere,
            &[
                (PI, -1.2),
                (TAU, -1.2),
                (TAU, 0.3),
                (5.5, 0.3),
                (5.5, 0.8),
                (TAU, 0.8),
                (TAU, 1.2),
                (PI, 1.2),
            ],
            Some(TAU),
        );
        assert_eq!(
            [
                read(&notch, 4.0, 0.3),
                read(&notch, 6.0, 0.5),
                read(&notch, 4.0, 0.8),
                read(&notch, 6.0, 0.29),
                read(&notch, 6.0, 0.81),
                read(&notch, 4.0, 1.2),
                read(&notch, 4.0, 1.2 - clear),
                read(&notch, 4.0, 1.2 - inside),
            ],
            ["in", "out", "in", "in", "in", "near", "in", "near"],
            "the notch: its iso rows' rays, its mouth, under and over it, on the top edge, \
             a thousand ε and a thousandth of ε inside it"
        );
        let diamond = chain(
            &sphere,
            &[(4.5, -0.5), (5.5, 0.3), (4.5, 1.0), (3.5, 0.3)],
            Some(TAU),
        );
        assert_eq!(
            [
                read(&diamond, 4.5, 0.3),
                read(&diamond, 3.0, 0.3),
                read(&diamond, 5.0, 1.0),
                read(&diamond, 4.5, -0.5 + clear),
                read(&diamond, 4.5, -0.5 - clear),
            ],
            ["in", "out", "out", "in", "out"],
            "the diamond: the ray through two vertices, outside it, and either side of its \
             lowest vertex"
        );
        let cap = chain(
            &sphere,
            &[(0.0, 1.0), (TAU, 1.0), (TAU, FRAC_PI_2), (0.0, FRAC_PI_2)],
            Some(TAU),
        );
        assert_eq!(
            [
                read(&cap, 1.0, 1.3),
                read(&cap, 1.0, 1.55),
                read(&cap, 1.0, FRAC_PI_2 - 1e-4),
                read(&cap, 1.0, 0.9),
                read(&cap, 1.0, 1.3 + TAU),
                read(&cap, 1.0 + TAU, 1.3),
            ],
            ["in", "in", "near", "out", "out", "out"],
            "the polar cap: inside, near the pole row, 1e-4 under it, under the cap, and a \
             period off in either channel"
        );
        let sliver = chain(
            &sphere,
            &[(4.0, 0.0), (5.0, 0.0), (5.0, inside), (4.0, inside)],
            Some(TAU),
        );
        assert_eq!(
            read(&sliver, 4.5, 0.5 * inside),
            "near",
            "a sliver thinner than the band"
        );
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: 0.4,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let wedge = chain(
            &cone,
            &[(0.0, 0.0), (TAU, 0.0), (TAU, 1.0), (0.0, 1.0)],
            None,
        );
        assert_eq!(
            [
                read(&wedge, 1.0, 0.5),
                read(&wedge, 1.0, 1e-3),
                read(&wedge, 1.0, inside),
                read(&wedge, 1.0 + TAU, 0.5),
            ],
            ["in", "near", "near", "out"],
            "the cone from its apex row: inside, 1e-3 and a thousandth of ε above the apex, a \
             period off"
        );
    }

    /// **A sphere ring stored on the twin sheet answers undecided.** Its
    /// point `(u + π, π − v)` names the same point of the sphere as one
    /// inside the half band, but no translation reaches it, and its window
    /// lies past the pole rows, where the azimuth arm is zero
    /// (`azimuth_lever_inf` holds `|v|` at `π/2`).
    #[test]
    fn a_sphere_ring_on_the_twin_sheet_answers_undecided() {
        let band = Band::linear(Tol::witness()).unwrap();
        let sphere = unit_sphere();
        let half = chain(
            &sphere,
            &[(PI, -1.2), (TAU, -1.2), (TAU, 1.2), (PI, 1.2)],
            Some(TAU),
        );
        let at = [4.5 - PI, PI - 0.3];
        let placed = place_ring([Some(TAU), Some(TAU)], half.reach, at, |s| {
            half.side(Point2::new(at[0] + s.offset[0], at[1] + s.offset[1]), band)
        })
        .map(|s| s.deck);
        assert_eq!(
            placed,
            Err(RingMiss::Undecided(None)),
            "the twin sheet's ring"
        );
    }

    /// **A torus places a ring by a move in both channels**, and an outer
    /// loop nearly a whole `v` period tall still holds one copy.
    #[test]
    fn a_torus_places_a_ring_in_both_channels() {
        let band = Band::linear(Tol::witness()).unwrap();
        let torus = Surface::Torus {
            center: Point3::origin(),
            major_radius: 3.0,
            minor_radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let square = chain(
            &torus,
            &[(0.5, 0.5), (1.5, 0.5), (1.5, 1.5), (0.5, 1.5)],
            Some(TAU),
        );
        let place = |o: &Outer<'_, f64>, at: [f64; 2]| {
            place_ring([Some(TAU), Some(TAU)], o.reach, at, |s| {
                o.side(Point2::new(at[0] + s.offset[0], at[1] + s.offset[1]), band)
            })
            .map(|s| [s.deck.u, s.deck.v])
        };
        assert_eq!(
            place(&square, [1.0 - TAU, 1.0 + TAU]),
            Ok([1, -1]),
            "stored a period off in each channel"
        );
        let tall = chain(
            &torus,
            &[(0.5, -3.0), (1.5, -3.0), (1.5, 3.0), (0.5, 3.0)],
            Some(TAU),
        );
        assert_eq!(place(&tall, [1.0, 0.0]), Ok([0, 0]), "a loop 6 rad tall");
    }

    /// **A count of periods with no `i32` answers typed, never
    /// saturated**: past `i32`, NaN and ±∞ have no whole, so the rule
    /// answers `Copies` for a ring point stored that far, and the reader
    /// answers `Near` for a point whose latitude copy it cannot count.
    #[test]
    fn a_count_past_i32_answers_copies_and_near() {
        assert_eq!(
            [
                whole(3.0),
                whole(-2_147_483_648.0),
                whole(2_147_483_648.0),
                whole(-2_147_483_649.0),
                whole(f64::NAN),
                whole(f64::INFINITY),
                whole(f64::NEG_INFINITY),
            ],
            [Some(3), Some(i32::MIN), None, None, None, None, None],
            "a whole in range, the least i32, one past each end, NaN, ±∞"
        );
        let reach = [(0.0, TAU), (0.0, 1.0)];
        for at in [1e12, f64::NAN, f64::INFINITY] {
            assert_eq!(
                place_ring([Some(TAU), None], reach, [at, 0.5], |_| Side::In).map(|s| s.deck.u),
                Err(RingMiss::Copies),
                "a ring point stored at u = {at}"
            );
        }
        let band = Band::linear(Tol::witness()).unwrap();
        let sphere = unit_sphere();
        let half = chain(
            &sphere,
            &[(PI, -1.2), (TAU, -1.2), (TAU, 1.2), (PI, 1.2)],
            Some(TAU),
        );
        for v in [1e12, f64::NAN] {
            assert!(
                matches!(half.side(Point2::new(4.5, v), band), Side::Near(None)),
                "a point at v = {v} has no latitude copy to read its arms at"
            );
        }
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
