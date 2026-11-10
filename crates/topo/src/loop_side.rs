//! **Which side of a boundary a loop lies on, read at every vertex.**
//!
//! A validator asks whether a loop lies inside a region: a ring inside
//! its face's outer loop (check 9), a surface cycle on its face's side of
//! the face's other cycles (tier 3's one-region reading). A loop that
//! does not cross the region's boundary places every vertex alike, so
//! reading one vertex would do, but in a validator that premise is the
//! thing being checked. So the reading here takes every vertex
//! ([`ladder::climb_agreeing`]), and a loop whose vertices place on both
//! sides crosses the boundary: that is a finding at a vertex placed
//! outside, as a loop lying wholly outside is. Its verdict does not
//! depend on which vertex the walk starts at.
//!
//! Construction sites that read a loop's side on the operation's own
//! premise (a ring the cut did not cross) are not this module's: they
//! read the first vertex that places.

use geom_brep::ladder::{self, Climb, Disagree, Kept, Rung};

use crate::entity::VertexKey;

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
    /// No vertex placed and none was kept.
    Unplaced,
}

/// **Whether every vertex of a loop places inside a region**: `place`
/// reads one vertex, `Some(true)` inside, `Some(false)` outside, `None`
/// for a vertex it does not place, and the kept reading for one it does
/// not decide. `place` has no way to stop the reading, so every vertex
/// in `vertices` (every vertex of the loop) is read, and a loop
/// placed inside at one vertex and outside at another is
/// [`EveryVertex::Outside`] at the outside one, whichever comes first.
/// A vertex kept in band beside a decided reading is superseded, as
/// [`ladder::climb_agreeing`] records it.
pub(crate) fn inside_at_every_vertex<E>(
    vertices: &[VertexKey],
    mut place: impl FnMut(VertexKey) -> Result<Option<bool>, Kept<E>>,
) -> EveryVertex<E> {
    let read = ladder::climb_agreeing(
        vertices.iter().copied(),
        |v| match place(v) {
            Ok(Some(inside)) => Rung::Answers((inside, v)),
            Ok(None) => Rung::Passes,
            Err(kept) => kept.into(),
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
        Ok(Climb::Passed) => EveryVertex::Unplaced,
        Ok(Climb::Undecided(e)) => EveryVertex::Undecided(e),
        Err(Disagree { first, other }) => EveryVertex::Outside {
            at: if first.0 { other.1 } else { first.1 },
        },
    }
}

#[cfg(test)]
#[allow(clippy::panic, clippy::expect_used)]
mod tests {
    use super::*;

    fn keys(n: usize) -> Vec<VertexKey> {
        let mut arena = slotmap::SlotMap::<VertexKey, ()>::with_key();
        (0..n).map(|_| arena.insert(())).collect()
    }

    /// The verdict at every rotation of a walk of `sides` (`Some(inside)`
    /// placed, `None` passed).
    fn every_rotation(sides: &[Option<bool>]) -> Vec<EveryVertex<()>> {
        let vs = keys(sides.len());
        (0..sides.len())
            .map(|r| {
                let order: Vec<usize> = (r..sides.len()).chain(0..r).collect();
                let walk: Vec<VertexKey> = order.iter().map(|&i| vs[i]).collect();
                inside_at_every_vertex(&walk, |v| {
                    let i = vs.iter().position(|&w| w == v).expect("a vertex");
                    Ok(sides[i])
                })
            })
            .collect()
    }

    /// A loop placed on both sides reads outside in every walk order, at
    /// its outside vertex; one placed alike reads that side; one placed
    /// nowhere is unplaced.
    #[test]
    fn a_crossing_loop_reads_outside_at_its_outside_vertex_in_every_walk_order() {
        let vs = keys(3);
        for verdict in every_rotation(&[Some(true), None, Some(false)]) {
            assert!(
                matches!(verdict, EveryVertex::Outside { .. }),
                "a crossing reads outside: {verdict:?}"
            );
        }
        let crossing = inside_at_every_vertex(&vs, |v| Ok::<_, Kept<()>>(Some(v != vs[2])));
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
                .all(|v| *v == EveryVertex::Unplaced),
            "placed nowhere"
        );
    }
}
