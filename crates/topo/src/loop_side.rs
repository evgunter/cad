//! **Which side of a boundary a loop lies on**, read vertex by vertex:
//! the one home of "is loop A inside loop B" for the readers that ask it
//! of a loop's vertices. They differ on one fact, who vouches that A
//! does not cross B, and each reading is named for its answer to it.
//!
//! - **A validator** vouches for nothing: that the loop does not cross
//!   the boundary is the thing being checked. [`inside_at_every_vertex`]
//!   reads every vertex ([`ladder::climb_agreeing`]), and a loop whose
//!   vertices place on both sides crosses the boundary: a finding at a
//!   vertex placed outside, as a loop lying wholly outside is. Its
//!   verdict does not depend on which vertex the walk starts at. A loop
//!   every vertex of which reads on the boundary is told apart from one
//!   the instrument placed nowhere ([`EveryVertex`]): the first meets
//!   the boundary, the second is the instrument's silence.
//! - **A construction site** reads a loop its own operation did not
//!   cross: the operation's division (a cut, a chord run) is the
//!   premise, so every vertex that places reads the loop alike and a
//!   touch switches no side. [`side_of_a_loop_the_division_does_not_cross`]
//!   takes the first vertex that places.
//!
//! Ordering two loops already known nested (the inner of a concentric
//! pair, a hole whose outline is disjoint from it by proof) is not
//! containment and is not read here: one point settles it at its site.
//!
//! [`of_loop`] is the one reading of a vertex against a planar loop's
//! walk ([`crate::splitting::containment::point_in_loop`]) as the rung
//! both readings climb.

use geom_brep::ladder::{self, Answering, Climb, Disagree, Rung};

use crate::entity::VertexKey;
use crate::splitting::containment::{LoopContainment, PointInLoopError};

/// **One vertex's reading against a planar loop as its rung**: inside
/// answers `Ok(true)`, outside `Ok(false)`, a vertex on the loop passes
/// (it says nothing about the side), and a refusal stands by its own
/// standing ([`PointInLoopError::refusal`]).
pub(crate) fn of_loop(
    reading: Result<LoopContainment, PointInLoopError>,
) -> Rung<Result<bool, PointInLoopError>, PointInLoopError> {
    let side = |at| match at {
        LoopContainment::In => Some(true),
        LoopContainment::Out => Some(false),
        LoopContainment::OnBoundary => None,
    };
    Rung::answers(reading.map(side), PointInLoopError::refusal)
}

/// **A loop read against a region at every vertex** ([`inside_at_every_vertex`]).
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EveryVertex<E> {
    /// Every vertex placed reads inside, and at least one placed.
    Inside,
    /// A vertex placed outside: the loop lies outside, or crosses the
    /// region's boundary. `at` is the first such vertex read.
    Outside {
        /// The vertex placed outside.
        at: VertexKey,
    },
    /// No vertex placed, and one was kept undecided: the ladder's ranked
    /// evidence ([`ladder::Evidence`]).
    Undecided(E),
    /// Every vertex read lies on the region's boundary, and at least one
    /// was read: the loop meets the boundary. `at` is the first.
    OnBoundary {
        /// The first vertex read on the boundary.
        at: VertexKey,
    },
    /// No vertex placed and none was kept, and the instrument had no
    /// answer at one at least (or the loop has no vertex).
    NoAnswer,
}

/// **Whether every vertex of a loop places inside a region**: `place`
/// reads one vertex as its rung ([`of_loop`] for a planar loop's walk),
/// `None` where the instrument has no answer there. `place` has no way
/// to stop the reading: a rung that would abort it (a refusal of the
/// whole question) is kept as a limit, so every vertex in `vertices`
/// (every vertex of the loop) is read, and a loop placed inside at one
/// vertex and outside at another is [`EveryVertex::Outside`] at the
/// outside one, whichever comes first. A vertex kept in band beside a
/// decided reading is superseded, as [`ladder::climb_agreeing`] records
/// it.
pub(crate) fn inside_at_every_vertex<E>(
    vertices: &[VertexKey],
    mut place: impl FnMut(VertexKey) -> Option<Rung<Result<bool, E>, E>>,
) -> EveryVertex<E> {
    let (mut no_answer, mut first) = (vertices.is_empty(), None);
    let read = ladder::climb_agreeing(
        vertices.iter().copied(),
        |v| {
            first.get_or_insert(v);
            match place(v) {
                None => {
                    no_answer = true;
                    Rung::Passes
                }
                Some(Rung::Answers(Ok(inside)) | Rung::Aborts(Ok(inside))) => {
                    Rung::Answers((inside, v))
                }
                Some(Rung::Answers(Err(e)) | Rung::Aborts(Err(e)) | Rung::Blocked(e)) => {
                    Rung::Blocked(e)
                }
                Some(Rung::Passes) => Rung::Passes,
                Some(Rung::InBand(e)) => Rung::InBand(e),
                Some(Rung::Unsettled(e)) => Rung::Unsettled(e),
            }
        },
        |a, b| a.0 == b.0,
    );
    match read {
        Ok(Climb::Answered {
            answer: (true, _), ..
        }) => EveryVertex::Inside,
        Ok(Climb::Answered {
            answer: (false, at),
            ..
        }) => EveryVertex::Outside { at },
        Ok(Climb::Passed) => match first {
            Some(at) if !no_answer => EveryVertex::OnBoundary { at },
            _ => EveryVertex::NoAnswer,
        },
        Ok(Climb::Undecided(e)) => EveryVertex::Undecided(e),
        Err(Disagree { first, other }) => EveryVertex::Outside {
            at: if first.0 { other.1 } else { first.1 },
        },
    }
}

/// **The side of a loop the operation's division does not cross**, read
/// at the first vertex that places: `read` gives one vertex's rung
/// ([`of_loop`] for a planar loop's walk, answering `Ok(true)` inside).
///
/// The premise is the caller's: its own division (the cut that made the
/// region, the chord run it joined) did not cross the loop, so the loop
/// lies in one component of the plane less the division, and every
/// vertex that places reads that one side ([`Answering::Any`]). It may
/// touch the division at a vertex (a pinch), so a vertex on the boundary
/// passes and the next is read; a kept refusal refuses only where no
/// vertex places ([`ladder::climb`]). Where every vertex passes the
/// ladder is [`Climb::Passed`], whose meaning (a loop on the division, a
/// loop nothing placed) is the caller's.
pub(crate) fn side_of_a_loop_the_division_does_not_cross<A, E>(
    vertices: impl IntoIterator<Item = VertexKey>,
    read: impl FnMut(VertexKey) -> Rung<A, E>,
) -> Climb<A, E> {
    ladder::climb(Answering::Any, vertices, read)
}

#[cfg(test)]
#[allow(clippy::panic, clippy::expect_used)]
mod tests {
    use super::*;

    fn keys(n: usize) -> Vec<VertexKey> {
        let mut arena = slotmap::SlotMap::<VertexKey, ()>::with_key();
        (0..n).map(|_| arena.insert(())).collect()
    }

    /// One vertex's rung: `Some(inside)` placed, `None` on the boundary.
    fn rung(side: Option<bool>) -> Option<Rung<Result<bool, ()>, ()>> {
        Some(side.map_or(Rung::Passes, |inside| Rung::Answers(Ok(inside))))
    }

    /// The verdict at every rotation of a walk of `sides` (`Some(inside)`
    /// placed, `None` on the boundary).
    fn every_rotation(sides: &[Option<bool>]) -> Vec<EveryVertex<()>> {
        let vs = keys(sides.len());
        (0..sides.len())
            .map(|r| {
                let order: Vec<usize> = (r..sides.len()).chain(0..r).collect();
                let walk: Vec<VertexKey> = order.iter().map(|&i| vs[i]).collect();
                inside_at_every_vertex(&walk, |v| {
                    let i = vs.iter().position(|&w| w == v).expect("a vertex");
                    rung(sides[i])
                })
            })
            .collect()
    }

    /// A loop placed on both sides reads outside in every walk order, at
    /// its outside vertex; one placed alike reads that side; one on the
    /// boundary at every vertex meets it, and one the instrument has no
    /// answer for at a vertex is not read as meeting it.
    #[test]
    fn a_crossing_loop_reads_outside_at_its_outside_vertex_in_every_walk_order() {
        let vs = keys(3);
        for verdict in every_rotation(&[Some(true), None, Some(false)]) {
            assert!(
                matches!(verdict, EveryVertex::Outside { .. }),
                "a crossing reads outside: {verdict:?}"
            );
        }
        let crossing = inside_at_every_vertex(&vs, |v| rung(Some(v != vs[2])));
        assert_eq!(
            crossing,
            EveryVertex::Outside { at: vs[2] },
            "the outside vertex is named, not the first read"
        );
        assert!(
            every_rotation(&[Some(true), None, Some(true)])
                .iter()
                .all(|v| *v == EveryVertex::Inside),
            "placed inside alike"
        );
        assert!(
            every_rotation(&[None, None])
                .iter()
                .all(|v| matches!(v, EveryVertex::OnBoundary { .. })),
            "on the boundary at every vertex"
        );
        let silent = inside_at_every_vertex(&vs, |v| if v == vs[1] { None } else { rung(None) });
        assert_eq!(
            silent,
            EveryVertex::NoAnswer,
            "a vertex without an answer is not on the boundary"
        );
        assert_eq!(
            inside_at_every_vertex::<()>(&[], |_| None),
            EveryVertex::NoAnswer,
            "no vertex read"
        );
    }

    /// A refusal of the whole question does not stop the every-vertex
    /// reading: a later vertex placed outside is still the verdict.
    #[test]
    fn an_aborting_vertex_does_not_hide_a_later_outside_one() {
        let vs = keys(2);
        let got = inside_at_every_vertex(&vs, |v| {
            Some(if v == vs[0] {
                Rung::Aborts(Err(()))
            } else {
                Rung::Answers(Ok(false))
            })
        });
        assert_eq!(got, EveryVertex::Outside { at: vs[1] });
    }
}
