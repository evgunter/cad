//! **A point against a closed chain of chart pieces, by ray parity**
//! ([`ray_parity_as`]): the one loop both readers of a point against a
//! loop in a face's chart run, the ring door's on an analytic chart
//! (`crate::pcurves`' `Outer::side`) and the spline trim's
//! ([`crate::spline_trim`]), each over pieces of its own family
//! ([`ChainPiece`]).

use geom_core::predicate::Band;
use geom_core::{Decide, InfSpeed, Margin, Real, Sign};

use crate::pcurves::{PIECE_DEPTH, Side};
use crate::validate::decide;

/// The query a closed chain is read against ([`ray_parity_as`]), on the
/// value channel: the point its ray leaves from, and the rectangle it
/// stands for (a point, or a cell's parameter box).
#[derive(Clone, Copy, Debug)]
pub(crate) struct RayQuery {
    /// The point the ray along `+u` leaves from, `[u, v]`.
    pub(crate) centre: [f64; 2],
    /// The query's rectangle, `[u, v]`.
    pub(crate) rect: [(f64, f64); 2],
}

/// A piece's box against a query ([`ChainPiece::boxed`]).
pub(crate) struct PieceBox<T> {
    /// The way from the query to the box in each channel, `[u, v]`, in
    /// chart units: zero where they meet.
    pub(crate) gap: [T; 2],
    /// The box on the value channel, `[u, v]`.
    pub(crate) extent: [(f64, f64); 2],
}

/// **One piece of a closed chain, as [`ray_parity_as`] reads it.** Each
/// chart family supplies its own: a lifted row's span or a joint's
/// segment boxed by [`geom_brep::Pcurve::chart_box`] on an analytic chart
/// (`crate::pcurves`' ring reader), a
/// rational Bézier piece boxed by its control points and their rounding
/// on a spline chart ([`crate::spline_trim`]).
pub(crate) trait ChainPiece<T: Real>: Clone {
    /// What a piece's box is measured from.
    type Query: Copy;
    /// The query on the value channel: the point its ray leaves from,
    /// and the rectangle it stands for.
    fn ray(q: Self::Query) -> RayQuery;
    /// A box holding every point of the piece, against `q`; `None` where
    /// the piece has none.
    fn boxed(&self, q: Self::Query) -> Option<PieceBox<T>>;
    /// Its two ends on the value channel, in the order its span runs.
    /// Wherever two pieces of the chain share an end it is bit-identical
    /// in both, which the parity's crossing count rests on: each
    /// family's impl says how it holds it.
    fn ends(&self) -> ([f64; 2], [f64; 2]);
    /// The two halves of its span.
    fn halves(&self) -> (Self, Self);
}

/// The gap between two closed intervals, zero where they meet.
pub(crate) fn gap((a0, a1): (f64, f64), (b0, b1): (f64, f64)) -> f64 {
    (b0 - a1).max(a0 - b1).max(0.0)
}

/// **The ray parity of `at` along `+u` over the closed chain `pieces`**,
/// every piece decided clear of the query first: clear where either
/// channel's gap alone is decided past the band under `name`, metered by
/// that channel's arm (the caller has decided every way out of the
/// window the arms hold over past the band, so each gap lower-bounds the
/// way along the chart to the piece). A clear piece wholly past the
/// centre in `u` crosses the ray as often, mod 2, as its ends lie either
/// side of `v = v_c` under the half-open rule `v ≥ v_c`; any other clear
/// piece misses it. A piece not clear is halved, down to
/// [`PIECE_DEPTH`] halvings, or until it meets a query of some size and
/// is no wider than the query in either channel (a box of exactly the
/// query's size included), where the query is undecided: a point query
/// is the case of size zero, which this stop never ends. `visits`
/// bounds the pieces read.
///
/// `ray_walk`, the chart region's parity, reads a polygon's straight
/// segments vertex by vertex and sets a grazing ray aside; these pieces
/// are curved and read off their boxes, so the crossing count is read
/// off each clear piece's two ends, whose shared ends are bit-identical
/// across the chain.
pub(crate) fn ray_parity_as<T: Decide, P: ChainPiece<T>>(
    name: &'static str,
    pieces: &[P],
    q: P::Query,
    (au, av): (InfSpeed<T>, InfSpeed<T>),
    visits: usize,
    band: Band,
) -> Side {
    let at = P::ray(q);
    let [cu, cv] = at.centre;
    let [su, sv] = at.rect.map(|(lo, hi)| hi - lo);
    let mut stack: Vec<(P, usize)> = pieces.iter().rev().map(|p| (p.clone(), 0)).collect();
    let (mut parity, mut visited) = (false, 0usize);
    while let Some((piece, depth)) = stack.pop() {
        visited += 1;
        if visited > visits {
            return Side::Near(None);
        }
        let Some(b) = piece.boxed(q) else {
            return Side::Near(None);
        };
        let mut diag = None;
        let mut clear = false;
        for (g, arm) in [(b.gap[0], au), (b.gap[1], av)] {
            match decide(name, Margin::metered(g, arm), band) {
                Ok(Sign::Positive) => clear = true,
                Ok(_) => {}
                Err(d) => diag = diag.or(Some(d)),
            }
        }
        let [(u0, u1), (v0, v1)] = b.extent;
        if clear {
            if v0 > cv || v1 < cv || u1 < cu {
                continue;
            }
            if u0 > cu {
                let (s, e) = piece.ends();
                parity ^= (s[1] >= cv) != (e[1] >= cv);
                continue;
            }
        }
        let meets = gap(at.rect[0], b.extent[0]) == 0.0 && gap(at.rect[1], b.extent[1]) == 0.0;
        if depth == PIECE_DEPTH
            || (meets && (su > 0.0 || sv > 0.0) && u1 - u0 <= su && v1 - v0 <= sv)
        {
            return Side::Near(diag);
        }
        let (l, r) = piece.halves();
        stack.push((r, depth + 1));
        stack.push((l, depth + 1));
    }
    if parity { Side::In } else { Side::Out }
}
