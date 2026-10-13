//! **A bisection walk**: a stack of pieces read depth first, earlier
//! half first, each either settled by its visit or cut in two, under one
//! piece budget (D9). The visit is the parameter: what clears a piece (a
//! first- or second-order bound on a distance), how small a piece may
//! get, and what a piece too small to cut answers are the caller's.
//! `boolean::carrier_touch` (a curve against a surface's carrier) and
//! `census::curved` (a curve against a point or another curve) walk it;
//! `boolean::circle_roots` (`SUBDIVISION_BUDGET`) and
//! `splitting::spiric_arc` (`MAX_PIECES`) keep walks of their own.

/// The most pieces one walk reads before it gives up. Sized from a
/// measurement taken once, on 2026-10-13 at ε 1e-6, 1e-9 and 1e-12 (K =
/// 10), with nothing that re-takes it — no row counts a walk's pieces,
/// since a count is a property of the bounds and not a contract, so the
/// figures are dated: `pierce_tangent_off_face` spent 70 to 150 pieces a
/// meeting, and `census::curved`'s rows at most 447 a walk, its tangency
/// rows 43 to 191. The tangency floor and the tangency's own half-length
/// both scale with `√(K·ε)`, so the pieces a touch keeps open per level
/// do not depend on K; the levels down to the floor grow as
/// `½·log₂(1/(K·ε))`, a few more per decade of either.
pub(crate) const PIECE_BUDGET: usize = 4096;

/// What a visit makes of a piece.
pub(crate) enum Step<P> {
    /// Nothing more to read in it.
    Settled,
    /// Read its two halves, the first before the second.
    Halve([P; 2]),
    /// The walk's question is answered: stop.
    Stop,
}

/// The walk ran past [`PIECE_BUDGET`].
#[derive(Clone, Copy, Debug)]
pub(crate) struct Exhausted;

/// Walks `root` depth first, earlier half first, until every piece is
/// settled or a visit stops it.
pub(crate) fn bisect<P>(root: P, mut visit: impl FnMut(P) -> Step<P>) -> Result<(), Exhausted> {
    let mut stack = vec![root];
    let mut read = 0;
    while let Some(piece) = stack.pop() {
        read += 1;
        if read > PIECE_BUDGET {
            return Err(Exhausted);
        }
        match visit(piece) {
            Step::Settled => {}
            Step::Halve([first, second]) => stack.extend([second, first]),
            Step::Stop => break,
        }
    }
    Ok(())
}
