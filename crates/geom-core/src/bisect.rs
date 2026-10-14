//! **The bisection walk** the kernel's localizations take: a stack of
//! pieces read depth first, earlier first, each either settled by its
//! visit, cut in two, or answering the walk's question, under one piece
//! budget (D9). The visit is the parameter — what clears a piece, how
//! small a piece may get, and what a piece too small to cut answers are
//! the caller's (e.g. a residual round a turn, or a curve against a
//! surface, a point or another curve). Beside it, the two lengths a walk
//! near a tangency reads alike: a piece's [`reach`] and the
//! [`tangency_floor`].
//!
//! The core owns the budget and not a depth: what "too deep" means is
//! each walk's. A walk may cap a piece's depth (answering there as at the
//! budget), count cuts only to choose which side to halve, or stop on a
//! decided length. A cap the core imposed would be a second budget,
//! binding where the first already does.

use crate::{Margin, Real, SupSpeed};

/// The most pieces one walk reads before it gives up. Sized from a
/// measurement taken once, on 2026-10-13, that nothing re-takes: a count
/// is a property of the bounds and not a contract, so no row pins one
/// and the figures are dated. The walks are not in this crate, so no
/// row here can re-take it either; what GUARDS the budget is the `topo`
/// and `sweep` suites the figures come from, where a walk that ran out
/// before answering refuses and the row that expected its answer reds.
/// Over the `topo` and `sweep`
/// suites at ε 1e-9 a walk that answered read at most 117 pieces in
/// `half_angle`, 249 in `carrier_touch`, 271 in `spiric_arc`'s point
/// and ray readings, 367 in `census::curved` and 2559 in `spiric_arc`'s
/// segment reading. Three rows run a walk past it by construction, each
/// a reading with no answer outside the band: `sweep`'s
/// `a_ruling_in_band_of_the_wall_but_not_on_it_is_not_on`
/// (`carrier_touch`) and `a_spiric_bounded_face_answers_off_its_spiric_edge`
/// (`spiric_arc`'s ray reading), and `topo`'s
/// `a_spiric_arc_never_clears_a_segment_through_it` (its segment
/// reading). The tangency floor and the
/// tangency's own half-length both scale with `√(K·ε)`, so the pieces a
/// touch keeps open per level do not depend on K; the levels down to the
/// floor grow as `½·log₂(1/(K·ε))`, a few more per decade of either.
pub const PIECE_BUDGET: usize = 4096;

/// What a visit makes of a piece.
pub enum Step<P, R> {
    /// Nothing more to read in it.
    Settled,
    /// Read its two halves, the first before the second.
    Halve([P; 2]),
    /// The walk's question is answered, by `R`: stop.
    Stop(R),
}

/// The walk ran past [`PIECE_BUDGET`].
#[derive(Clone, Copy, Debug)]
pub struct Exhausted;

/// Walks `roots` in order, each depth first, earlier half first, until
/// every piece is settled (`Ok(None)`) or a visit stops the walk with
/// its answer (`Ok(Some(_))`).
pub fn bisect<P, R>(
    roots: impl IntoIterator<Item = P>,
    mut visit: impl FnMut(P) -> Step<P, R>,
) -> Result<Option<R>, Exhausted> {
    let mut stack: Vec<P> = roots.into_iter().collect();
    stack.reverse();
    let mut read = 0;
    while let Some(piece) = stack.pop() {
        read += 1;
        if read > PIECE_BUDGET {
            return Err(Exhausted);
        }
        match visit(piece) {
            Step::Settled => {}
            Step::Halve([first, second]) => stack.extend([second, first]),
            Step::Stop(answer) => return Ok(Some(answer)),
        }
    }
    Ok(None)
}

/// The most metres a point of a piece `(a, b)` lies from its middle
/// point, on a carrier whose speed is at most `hi`: half the run,
/// metered.
pub fn reach<T: Real>(hi: SupSpeed<T>, (a, b): (T, T)) -> T {
    Margin::metered_sup((b - a).abs(), hi).value() * T::from_f64(0.5)
}

/// **The piece size a walk near a tangency stops at**, in metres: an
/// eighth of `√(ρ·escalate)`, `ρ` a lower bound on the radius over which
/// the two curves part (the reciprocal of their summed curvature; a
/// surface's reach). A curve tangent to another stays within `escalate`
/// of it over a half-length of about `√(2ρ·escalate)`, and no bound
/// clears a piece in there, so cutting finer only multiplies pieces;
/// an eighth of it keeps the piece that holds the touch within
/// `escalate/128` of the other.
pub fn tangency_floor<T: Real>(rho: T, escalate: f64) -> T {
    (rho * T::from_f64(escalate)).sqrt() / T::from_f64(8.0)
}
