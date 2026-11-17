---
id: a-slow-row-whose-assertion-moves-without-its-name-line-is-not-gated
kind: issue
title: A slow-set row whose assertion or doc moves without an added line spelling its name is not selected on the PR, so a PR that moves a pinned count in a slow row is never gated on it
status: open
opened: 2026-11-17
priority: P1
cost: C
---


## Finding

Found by EMBED's lane `embed/conic-arcs` (PR 7089), by its review.

`scripts/slow_set.py`'s `is_new` selects a slow-set row on a PR only
when an added `.rs` line spells the test's leaf name, or a module on its
path that the diff declares (`check-fast-set-times.py`'s
`added_tokens`). A PR that edits a slow row's assertion or doc, but not
the line that names it, does not select it.

The instance: PR 7089 moved
`self_intersection_proves_injectivity::a_cylinder_cylinder_tee_builds_and_its_injectivity_refuses_typed_at_a_vertex_its_junction_shares_alone`'s
pinned count from `refused == 12` to `10`. The test is in the `ci`
slow filter of `.config/nextest.toml`, and the edited lines were its
`assert_eq!` and its doc, so its first CI run did not run it. The
review ran it by hand. The lane then got it selected by naming it on an
added line.

So a PR that moves a pinned count in a slow row is not gated on it,
and the nightly is the first run to see a wrong pin.

What a fix needs: also select a slow-set test whose body or doc the
diff touches, by reading the hunks' enclosing `fn` (the `@@` context
line `git diff` prints) as well as the added lines' identifiers.
