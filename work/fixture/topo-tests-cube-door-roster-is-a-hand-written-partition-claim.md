---
id: topo-tests-cube-door-roster-is-a-hand-written-partition-claim
kind: issue
title: cube_doors_agree.rs's door roster is hand-written; a new builder in common joins it silently as nothing
status: open
opened: 2026-09-16
priority: P4
cost: E
---


## Finding

- **Where**: `crates/topo/tests/cube_doors_agree.rs`, the `doors`
  vector in `every_box_door_builds_one_body` (~`:88–104`), against
  `crates/topo/tests/common/mod.rs`'s builder set.
- **Importance**: low-medium
- **Confidence**: sure about the gap; **no verdict** on whether there
  is a practical fix inside `tests/`
- **Raised by**: the full review of PR #2727 (S-DUP), 2026-09-16

The roster is a hand-written list of five builders. Nothing connects it
to `common/mod.rs`, so **a sixth box builder added there joins this
guard silently as no builder at all** — the test stays green, and the
only record that it was meant to be exhaustive is the doc comment above
it.

This is a **partition claim**, and partition claims rot by omission
rather than by contradiction: the failure mode is a row that keeps
passing while covering less, which is the same shape as the defect PR
#2727 was written to prevent (a suite that stops being reachable rather
than starting to fail). The lane that closed a hand-written list closed
it with another hand-written list.

## What a fix would have to overcome

Rust gives a test binary no reflection over a sibling module's items,
and `tests/common/mod.rs` is a plain module, so "enumerate every `pub
fn` returning `Body<f64>`" is not expressible in the language at this
site. The candidates, none obviously right:

- a **source-level census**, like `tests/all.rs`'s
  `every_suite_file_is_aggregated`, which walks the directory and fails
  when a file is unlisted. `test_utils::source` already does this kind
  of walk for suite files, so the machinery exists; the question is
  whether "a builder that belongs in the roster" is recognisable from
  source text without becoming its own guessing game.
- a **macro** in `common/mod.rs` that declares each box builder and
  emits the roster, which moves the hand-written list next to the
  builders rather than removing it.
- **accepting it** and saying so at the site, which is what the file
  does today.

**This row is the finding, not the verdict.** If the answer is that no
mechanism beats the doc comment, that is a legitimate close — written
down, so the next reader does not re-open it.
