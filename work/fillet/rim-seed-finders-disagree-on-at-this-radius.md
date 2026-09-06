---
id: rim-seed-finders-disagree-on-at-this-radius
kind: issue
title: four disagreeing tolerances spell "the circle at this radius" across the tree
status: open
opened: 2026-09-05
---

## The shape

PR 1821 gave the tree ONE door for "which rim does this arc belong to"
(`topo::query::rim_of`, exact, no tolerance at all). What it did not
touch — deliberately, and this issue is the disposition — is the half
ABOVE it: **which arc a caller means in the first place.** Every seed
finder in the tree still spells "the circle at radius `r`" with a
hand-chosen window, and the four windows in use disagree by six orders
of magnitude with no stated reason for any of them.

The windows, measured on this branch:

- `crates/sweep/src/test_support.rs:190` — `closed_plane_sphere_rim`,
  `(radius - rim_r).abs() < 1e-6`.
- `crates/sweep/src/test_support.rs:289` — `arcs_at` (and so
  `rim_arcs_at`, which seeds from it), both enclosure ends against
  `1e-9`.
- `demos/tour/src/teapot.rs:482` — `rim_at`, station AND radius against
  `1e-12`.
- `demos/tour/tests/blend1_r1_wall6_probes.rs` — `rims_of_radius`,
  `5e-4`. The loosest by far, on a body whose rims BOTH reviewers of PR
  1821 measured bit-exact at their analytic radii — so the window is
  inherited slack rather than a property of the geometry. Recorded at
  the site in that file's doc comment.
- The review-probe helpers that carry their own copies:
  `crates/sweep/tests/review_arms2_r1_probes.rs` (`closed_rims`, `1e-9`),
  `crates/sweep/tests/verbs_arms2_bud.rs` (`closed_rims`, `1e-9`),
  `crates/sweep/tests/verbs_arms1_r1_probes.rs` (`rim_at`, `1e-9` on the
  station), `crates/sweep/tests/verbs_rim_closed_lever.rs`
  (`find_rim`, `1e-9`), `crates/sweep/tests/review_arms3_r1_probes.rs`
  (`arcs_of_radius`, `1e-9`),
  `demos/tour/tests/review_blend2_r1_probes.rs` (`arcs_at`, `1e-9`).

## Why it is an issue and not a defect

None of these is wrong today: every one of them is a FIXTURE selector
choosing among analytically-stated radii on a body the same file built,
which is why PR 1821's spec calls that comparison "a fixture-selection
tolerance and not a kernel predicate". The defect shape is that the
tree has no ONE spelling for the question, so a caller copying any of
them copies a number nobody chose, and the `5e-4` one shows where that
ends: a window wide enough to admit a neighbouring rim on a body with
closely-spaced radii, on a probe whose subject is not the selection.

## Why not in PR 1821

Scope: that unit's fence is the door BELOW this question (the spec's
"Out of scope" names the naming half), and sweeping ten call sites
across two workspaces to normalise a tolerance would have put an
unreviewed change into every rim row in the tree on a branch whose
subject is the exact door. Ruled by the orchestrator at the fix pass.

## What would close it

A decided answer to "what does a caller name a rim BY". The candidates
are in the item this unit came from
(`no-public-rim-arc-selector`, options 2 and 3: keyed by the carrier
circle with an explicit band, keyed by the support pair) and the real
answer is probably the names vocabulary — a rim with a NAME needs no
radius at all. Until then: one homed seed finder with one stated
tolerance and one reason, and the copies above deleted in its favour.
