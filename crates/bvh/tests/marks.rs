//! **Each query marks the candidates it hands on** in the open
//! `k_stats` frame (`geom_core::k_stats::mark_candidates`), in the order
//! it hands them, then its end: the first-turn reading
//! (`editor_core::resolve::vdiff`) reads these marks to tell two runs
//! whose pruning parted from two on one path.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/bvh/src/", "crates/geom-core/src/k_stats.rs"];

use bvh::Bvh;
use bvh::test_support::boxed;
use geom_core::k_stats::Bracket;

#[test]
fn a_query_marks_the_candidates_it_hands_on_then_its_end() {
    let boxes = [
        boxed([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]),
        boxed([5.0, 5.0, 5.0], [6.0, 6.0, 6.0]),
        boxed([0.5, 0.5, 0.5], [2.0, 2.0, 2.0]),
    ];
    let tree = Bvh::build(&boxes);
    let bracket = Bracket::open();
    let hit = tree.overlapping(&boxed([0.8, 0.8, 0.8], [0.9, 0.9, 0.9]));
    let none = tree.overlapping(&boxed([9.0, 9.0, 9.0], [9.5, 9.5, 9.5]));
    let log = bracket.finish();
    assert_eq!(hit, [0, 2]);
    assert!(none.is_empty());
    assert_eq!(
        log.marks
            .iter()
            .map(|m| (m.site, m.item))
            .collect::<Vec<_>>(),
        [
            ("bvh_overlapping", Some(0)),
            ("bvh_overlapping", Some(2)),
            ("bvh_overlapping", None),
            ("bvh_overlapping", None),
        ],
        "each candidate handed on, then each query's end"
    );
    assert!(log.verdicts.is_empty(), "a query decides nothing");
}
