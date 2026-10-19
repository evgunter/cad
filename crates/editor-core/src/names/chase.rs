//! The walk that follows a chain of lineage rows to its root — a split
//! piece to the edge it was split from, a vertex copy to the vertex it
//! copies — written once, so every chain is bounded and refuses a
//! cycle the same way.
//!
//! Every chain the emitters chase is made of rows the kernel's own split
//! or boolean wrote, and each caller's `# Panics` says why its rows
//! cannot cycle, so no input reaches a cycle: one is a kernel bug,
//! observable here as a walk that outlasts its bound, and it panics
//! (`docs/DESIGN.md` `[failure.taxonomy]`, row 4).

use core::fmt::Debug;
use core::ops::ControlFlow;

/// The key the chain from `start` ends at. `step` reads one key and
/// answers `Continue(next)` to go on to `next`, or `Break(root)` to end
/// at `root`.
///
/// # Errors
///
/// What `step` refuses with, at the first key it refuses.
///
/// # Panics
///
/// Where `step` is still going after `bound` steps, which an acyclic
/// chain of at most `bound` hops cannot be: the rows cycle. `why` points
/// at the caller's argument that they cannot, and the message carries it.
#[track_caller]
pub(super) fn to_root<K: Copy + Debug, E>(
    start: K,
    bound: usize,
    why: &str,
    mut step: impl FnMut(K) -> Result<ControlFlow<K, K>, E>,
) -> Result<K, E> {
    let mut at = start;
    for _ in 0..=bound {
        match step(at)? {
            ControlFlow::Continue(next) => at = next,
            ControlFlow::Break(root) => return Ok(root),
        }
    }
    unreachable!(
        "the chain from {start:?} is still going at {at:?} after {bound} steps, so its rows \
         cycle, which only a kernel bug makes them do ({why})"
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::to_root;
    use core::convert::Infallible;
    use core::ops::ControlFlow;
    use std::collections::BTreeMap;

    fn up(rows: &BTreeMap<u8, u8>) -> impl FnMut(u8) -> Result<ControlFlow<u8, u8>, Infallible> {
        |k| {
            Ok(match rows.get(&k) {
                Some(&p) => ControlFlow::Continue(p),
                None => ControlFlow::Break(k),
            })
        }
    }

    #[test]
    fn a_chain_of_bound_hops_reaches_its_root() {
        let rows = BTreeMap::from([(3, 2), (2, 1), (1, 0)]);
        assert_eq!(to_root(3, rows.len(), "", up(&rows)).unwrap(), 0);
        assert_eq!(
            to_root(9, rows.len(), "", up(&rows)).unwrap(),
            9,
            "a key no row holds"
        );
    }

    #[test]
    #[should_panic(expected = "a kernel bug makes them do (the rows are a test's)")]
    fn a_cycle_panics() {
        let rows = BTreeMap::from([(2, 1), (1, 2)]);
        let _ = to_root(2, rows.len(), "the rows are a test's", up(&rows));
    }

    #[test]
    fn a_step_refusal_passes_through() {
        let r = to_root(5u8, 3, "", |k| {
            if k == 3 {
                Err(k)
            } else {
                Ok(ControlFlow::Continue(k - 1))
            }
        });
        assert_eq!(r, Err(3));
    }
}
