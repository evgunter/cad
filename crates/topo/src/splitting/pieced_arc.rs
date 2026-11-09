//! **A curved edge of a planar loop read piece by piece** — the boundary
//! reading and the ray crossing count the in-plane walk
//! ([`super::containment`]) runs on a carrier with no closed-form ray
//! intersection it could certify cheaply: a spiric oval's arc
//! ([`super::spiric_arc`]) and a spline's ([`super::spline_arc`]).
//!
//! Both readings subdivide the arc's parameter window and certify each
//! piece from what its carrier says over the piece's own window
//! ([`PiecedArc`]): a ball holding the piece, with a point of the arc in
//! it, for the boundary reading; and whether a ray crosses the piece,
//! for the count. A piece that settles neither way is halved, until its
//! ball is within the band's coincidence threshold (the boundary
//! reading), [`MAX_DEPTH`], or [`geom_core::bisect`]'s piece budget.
//! The boundary reading is its own walk over that ball cover, not
//! [`geom::Curve3::point_walk`]: a piece's ball is what the carrier says
//! over the piece, tighter than the walk's speed-bound reach, and its
//! question is where the point sits (an end, on, clear), not a distance. A
//! reading that runs out of depth or budget is not a reading in the
//! band: the walk names it as the edge it could not cross
//! ([`super::containment::Uncrossable`]).

use geom_brep::ladder::{Answering, Climb, Evidence, Refusal, Rung, climb};
use geom_core::bisect::{Step, bisect};
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::validate::decide;

/// The rows a caller reads a pieced edge's boundary under — its own,
/// per carrier, so each population stays separable in the telemetry.
#[derive(Clone, Copy)]
pub(crate) struct ArcRows {
    /// The distance from the point to either end of the arc.
    pub(crate) end: &'static str,
    /// A piece's ball's clearance from the point.
    pub(crate) clear: &'static str,
    /// The distance from the point to a point of the arc in the ball.
    pub(crate) on: &'static str,
    /// A piece's ball's radius: within the band's coincidence threshold
    /// the piece is a point, and is not halved again.
    pub(crate) leaf: &'static str,
}

/// How many times a piece is halved before a reading gives up on it:
/// past the point where `f64` can halve a window of a few units at all
/// (its parameters are then neighbouring floats), so the depth never
/// binds before the arithmetic does. The ray count reaches it only where
/// a crossing sits within the band of a halving point or the ray grazes
/// the arc; the boundary reading, whose pieces stop at the band, only on
/// a carrier whose bounds outrun `2⁶⁰` band widths.
pub(crate) const MAX_DEPTH: u32 = 60;

/// A walk that stopped on a piece past [`MAX_DEPTH`]: no answer.
struct TooDeep;

/// A ball holding a piece of the arc, and a point of the arc in it.
pub(crate) struct PieceBall<T: geom_core::Real> {
    pub(crate) center: Point3<T>,
    pub(crate) reach: T,
    pub(crate) on: Point3<T>,
}

/// The ray `q + d·t`, `t > 0`, in the loop's plane: `side` is the
/// in-plane unit normal to `d`.
#[derive(Clone, Copy)]
pub(crate) struct Ray<T: geom_core::Real> {
    pub(crate) q: Point3<T>,
    pub(crate) d: Vec3<T>,
    pub(crate) side: Vec3<T>,
}

/// **An arc its carrier reads piece by piece.** The two readings are
/// provided; a carrier states its pieces and what it certifies of each.
pub(crate) trait PiecedArc<T: Decide> {
    /// A piece of the arc's parameter window.
    type Piece;

    /// The arc's two end points.
    fn ends(&self) -> [Point3<T>; 2];

    /// The pieces the whole arc is read from.
    fn roots(&self) -> Vec<Self::Piece>;

    /// How many halvings made `piece`.
    fn depth(piece: &Self::Piece) -> u32;

    /// A piece's two halves.
    fn halves(&self, piece: Self::Piece) -> [Self::Piece; 2];

    /// A ball holding the piece, and a point of the arc in it.
    fn ball(&self, piece: &Self::Piece) -> PieceBall<T>;

    /// How many times `ray` crosses the piece, where the piece settles
    /// it: `None` halves it.
    fn cross(&self, piece: &Self::Piece, ray: Ray<T>, band: Band) -> Option<usize>;

    /// **Where `q` sits against the arc.** `End` within the band of an
    /// end; `On` within the band of a point of the arc, both ends decided
    /// clear of `q`; `Off` once every piece's ball is definitely clear of
    /// `q`. A placement on the arc's own ball cover, not a clearance by
    /// the carrier's reach ([`geom::Curve3::clear_of`]): `On` is an
    /// answer, so the walk goes on past its first in-band piece.
    ///
    /// The ends are the piece walk's premise, not rungs beside it: `On`
    /// says `q` is clear of both ends, so an end read in band blocks it.
    /// The two ends are one ladder ([`climb`], [`Answering::Any`]: either
    /// end decided at `q` is `End`), and the walk runs only where both
    /// ends are decided clear.
    ///
    /// # Errors
    ///
    /// An end read in band where the other end is not decided at `q`:
    /// [`ArcRefusal::Escalated`], the ends' kept reading ranked
    /// ([`Evidence`]: a poisoned margin a limit). Else, where no piece
    /// places `q` definitely, the walk's kept reading ranked:
    /// [`ArcRefusal::Escalated`] for a piece whose ball is within the
    /// band's coincidence threshold settled neither way, carrying the
    /// margin it was read on (a poisoned one a limit, ahead of every
    /// margin in band); else [`ArcRefusal::Unsettled`] where a piece
    /// outlasts the depth or the piece budget before its ball is within
    /// the band.
    fn contact(&self, q: Point3<T>, rows: ArcRows, band: Band) -> Result<ArcHit, ArcRefusal> {
        let ends = climb(Answering::Any, self.ends(), |end| {
            match decide(rows.end, Margin::norm3(q - end), band) {
                Ok(Sign::Zero) => Rung::Answers(ArcHit::End),
                Ok(_) => Rung::Passes,
                Err(diag) => Rung::kept(diag).map_kept(ArcRefusal::Escalated),
            }
        });
        match ends {
            Climb::Answered { answer, .. } => Ok(answer),
            Climb::Undecided(why) => Err(why),
            Climb::Passed => self.pieces_contact(q, rows, band),
        }
    }

    /// The piece walk of [`Self::contact`]: a point of the arc within the
    /// band of `q` answers `On`, every piece clear reads `Off`, and
    /// otherwise what the leaves no reading settled kept refuses, ranked
    /// ([`Evidence`]).
    fn pieces_contact(
        &self,
        q: Point3<T>,
        rows: ArcRows,
        band: Band,
    ) -> Result<ArcHit, ArcRefusal> {
        let mut kept = Evidence::default();
        let walk = bisect(self.roots(), |p| {
            let b = self.ball(&p);
            let mut read = None;
            match decide(
                rows.clear,
                Margin::of((q - b.center).norm() - b.reach),
                band,
            ) {
                Ok(Sign::Positive) => return Step::Settled,
                Ok(_) => {}
                Err(diag) => read = Some(diag),
            }
            match geom_core::k_stats::decide_reported(rows.on, Margin::of((q - b.on).norm()), band)
            {
                Ok(geom_core::Decided {
                    sign: Sign::Zero,
                    margin,
                }) => return Step::Stop(margin),
                Ok(_) => {}
                Err(diag) => read = Some(diag),
            }
            if matches!(decide(rows.leaf, Margin::of(b.reach), band), Ok(Sign::Zero)) {
                match read {
                    Some(diag) => kept.keep(Refusal::Margin(diag), ArcRefusal::Escalated(diag)),
                    None => kept.unsettled(ArcRefusal::Unsettled),
                }
                return Step::Settled;
            }
            if Self::depth(&p) >= MAX_DEPTH {
                kept.unsettled(ArcRefusal::Unsettled);
                return Step::Settled;
            }
            Step::Halve(self.halves(p))
        });
        match walk {
            Ok(Some(margin)) => return Ok(ArcHit::On(margin)),
            Ok(None) => {}
            Err(_) => kept.unsettled(ArcRefusal::Unsettled),
        }
        kept.refusal().map_or(Ok(ArcHit::Off), Err)
    }

    /// **How many times the ray `q + d·t`, `t > 0`, crosses the arc**,
    /// counted in the loop's plane (`side` the in-plane unit normal to
    /// `d`), each piece settled by [`Self::cross`]. `None` where a piece
    /// meeting the ray settles neither its crossing nor its miss by the
    /// depth or the budget — a graze, or a crossing at a piece's end —
    /// which grazes the ray (why that is sound: the ray loop in
    /// [`super::containment`]'s walk). The walk calls it only for a
    /// point [`Self::contact`] read `Off`.
    fn crossings(&self, q: Point3<T>, d: Vec3<T>, side: Vec3<T>, band: Band) -> Option<usize> {
        let ray = Ray { q, d, side };
        let mut count = 0;
        let walk = bisect(self.roots(), |p| {
            if let Some(n) = self.cross(&p, ray, band) {
                count += n;
                return Step::Settled;
            }
            if Self::depth(&p) >= MAX_DEPTH {
                return Step::Stop(TooDeep);
            }
            Step::Halve(self.halves(p))
        });
        matches!(walk, Ok(None)).then_some(count)
    }
}

/// Where a point sits against a pieced arc — [`PiecedArc::contact`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ArcHit {
    /// Definitely off the arc.
    Off,
    /// Within the band of a point of the arc, clear of its ends, by the
    /// margin that decided it.
    On(geom_core::MarginDiag),
    /// Within the band of one of the arc's two ends.
    End,
}

/// Why [`PiecedArc::contact`] places no point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ArcRefusal {
    /// A decision fell in the band's gap.
    Escalated(Indeterminate),
    /// A piece outlasted the reading's depth or budget before its ball
    /// came within the band.
    Unsettled,
}

impl From<Indeterminate> for ArcRefusal {
    fn from(diag: Indeterminate) -> Self {
        Self::Escalated(diag)
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::{Band, MarginDiag, Point3};

    use super::{ArcHit, ArcRefusal, ArcRows, PieceBall, PiecedArc, Ray};

    const ROWS: ArcRows = ArcRows {
        end: "point_in_arc_loop_spline_end",
        clear: "point_in_arc_loop_spline_clear",
        on: "point_in_arc_loop_spline_on",
        leaf: "point_in_arc_loop_spline_leaf",
    };

    fn band() -> Band {
        Band::new(1e-9, 1e-8).expect("a valid band")
    }

    fn at(x: f64) -> Point3<f64> {
        Point3::new(x, 0.0, 0.0)
    }

    /// An arc read as hand-placed leaves: each piece a ball of no
    /// reach, so every piece is a leaf and none is halved.
    struct Placed {
        ends: [Point3<f64>; 2],
        leaves: Vec<Point3<f64>>,
    }

    impl PiecedArc<f64> for Placed {
        type Piece = usize;
        fn ends(&self) -> [Point3<f64>; 2] {
            self.ends
        }
        fn roots(&self) -> Vec<usize> {
            (0..self.leaves.len()).collect()
        }
        fn depth(_: &usize) -> u32 {
            0
        }
        fn halves(&self, _: usize) -> [usize; 2] {
            panic!("every placed piece is a leaf")
        }
        fn ball(&self, piece: &usize) -> PieceBall<f64> {
            let c = self.leaves[*piece];
            PieceBall {
                center: c,
                reach: 0.0,
                on: c,
            }
        }
        fn cross(&self, _: &usize, _: Ray<f64>, _: Band) -> Option<usize> {
            panic!("contact reads no crossing")
        }
    }

    /// **The ends are the walk's premise** ([`PiecedArc::contact`]): the
    /// second end decided at the point answers past the first in band,
    /// since either end is `End`; but an end in band refuses ahead of a
    /// piece through the point, since `On` says the point is clear of
    /// both ends.
    #[test]
    fn an_end_in_band_blocks_the_walk_but_not_the_other_end() {
        let q = at(0.0);
        let end_at = Placed {
            ends: [at(5e-9), at(0.0)],
            leaves: vec![at(10.0)],
        };
        assert!(
            matches!(end_at.contact(q, ROWS, band()), Ok(ArcHit::End)),
            "the second end at the point answers past the first in band"
        );
        let piece_on = Placed {
            ends: [at(5e-9), at(10.0)],
            leaves: vec![at(0.0)],
        };
        assert!(
            matches!(
                piece_on.contact(q, ROWS, band()),
                Err(ArcRefusal::Escalated(_))
            ),
            "an end in band refuses ahead of a piece through the point"
        );
        let clear = Placed {
            ends: [at(10.0), at(20.0)],
            leaves: vec![at(0.0)],
        };
        assert!(
            matches!(clear.contact(q, ROWS, band()), Ok(ArcHit::On(_))),
            "with both ends decided clear, a piece through the point answers"
        );
    }

    /// **A poisoned leaf is a limit, ranked ahead of a leaf in band read
    /// before it** (the ladder's rule for a margin, `Kept::of`): the walk
    /// refuses with the poisoned reading.
    #[test]
    fn a_poisoned_leaf_outranks_an_earlier_leaf_in_band() {
        let arc = Placed {
            ends: [at(10.0), at(20.0)],
            leaves: vec![at(5e-9), at(f64::NAN)],
        };
        match arc.contact(at(0.0), ROWS, band()) {
            Err(ArcRefusal::Escalated(diag)) => assert_eq!(
                diag.margin,
                MarginDiag::INVALID,
                "the poisoned leaf's reading refuses, not the in-band one's"
            ),
            other => panic!("the leaves refuse in band, not {other:?}"),
        }
    }
}
