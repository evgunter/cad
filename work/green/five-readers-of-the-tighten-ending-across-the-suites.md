---
id: five-readers-of-the-tighten-ending-across-the-suites
kind: issue
title: Five suites read the offered tolerance out of a refusal's "tighten the tolerance below" ending, each its own way
status: open
opened: 2026-11-22
---

Filed by SCALAR's lane on PR 7267 (2026-11-22), from that PR's review.

## What

The offered tolerance is read off the sentence the user reads ("… tighten the tolerance below v m") by five separate readers. Each parses the ending in its own way:

- `test_utils::offer::offered_below` (`crates/test-utils/src/offer.rs`): `split_once`, then the first whitespace-separated token;
- `geom_core::test_support::offered_tolerance` (`crates/geom-core/src/test_support.rs`): `rsplit_once`, then `strip_suffix(" m")`;
- `crates/topo/src/tier3_tests.rs` (about line 2644): `.split("tighten the tolerance below ")`;
- `crates/topo/src/boolean/refusal_routes.rs` `offered_below` (about line 1707): `split_once("tighten the tolerance")`;
- `crates/topo/src/merge_faces.rs` `a_zero_winding_that_leaves_no_outline_tells_the_winding_story` (about line 5491): an inline `split_once` and `strip_suffix(" m")`.

The two leading readers disagree on a sentence with two endings: `split_once` takes the first, `rsplit_once` takes the last. They also disagree on whether the unit must follow.

When PR 7159 changed how an offer is spelled (a told offered bound), each of these readers was exposed separately.

## Owed

One reader, in `test_utils::offer` (it depends on nothing, so every suite can reach it). Every suite reads the ending through it, and the "first or last ending" choice is stated once.
