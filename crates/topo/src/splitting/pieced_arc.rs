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
//! reading), [`MAX_DEPTH`], or [`geom_core::bisect`]'s piece budget. A
//! reading that runs out of depth or budget is not a reading in the
//! band: the walk names it as the edge it could not cross
//! ([`super::containment::Uncrossable`]).

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
    /// end; `On` within the band of a point of the arc; `Off` once every
    /// piece's ball is definitely clear of `q`.
    ///
    /// # Errors
    ///
    /// [`ArcRefusal::Escalated`]: an in-band distance to an end; or,
    /// where a piece whose ball is within the band's coincidence
    /// threshold is settled neither way and no other piece puts `q` on
    /// the arc, the in-band margin it was read on.
    /// [`ArcRefusal::Unsettled`] where a piece outlasts the depth or the
    /// piece budget before its ball is within the band.
    fn contact(&self, q: Point3<T>, rows: ArcRows, band: Band) -> Result<ArcHit, ArcRefusal> {
        for end in self.ends() {
            if decide(rows.end, Margin::norm3(q - end), band)? == Sign::Zero {
                return Ok(ArcHit::End);
            }
        }
        // The first leaf no reading settled, by the margin it was read
        // in the band on.
        let mut in_band: Option<Indeterminate> = None;
        let mut exhausted = false;
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
                    Some(diag) => {
                        in_band.get_or_insert(diag);
                    }
                    None => exhausted = true,
                }
                return Step::Settled;
            }
            if Self::depth(&p) >= MAX_DEPTH {
                exhausted = true;
                return Step::Settled;
            }
            Step::Halve(self.halves(p))
        });
        match walk {
            Ok(Some(margin)) => return Ok(ArcHit::On(margin)),
            Ok(None) => {}
            Err(_) => exhausted = true,
        }
        if let Some(diag) = in_band {
            return Err(ArcRefusal::Escalated(diag));
        }
        if exhausted {
            return Err(ArcRefusal::Unsettled);
        }
        Ok(ArcHit::Off)
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
