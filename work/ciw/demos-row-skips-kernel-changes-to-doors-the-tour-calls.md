---
id: demos-row-skips-kernel-changes-to-doors-the-tour-calls
kind: issue
title: The PR gate runs the demos row only when demos/ changes, so a kernel change to a door the tour calls reaches the tour only nightly
status: open
opened: 2026-10-02
priority: P3
cost: E
---


## What

`.github/workflows/ci.yml`'s change filter keys the demos row on
`flag demos "touched '^demos/'"` (the classify step, near line 112),
so the PR gate runs `demos/tour` and `demos/wild` only when the diff
touches `demos/`. A kernel change to a door the tour calls is gated
only by the nightly. Instance: PR 3773 rewrote `topo::query::rim_of`,
which `demos/tour/src/bodies.rs` calls (the bud's mouth rim), and
`RimError`'s arms; its gate skipped the demos row, and its reviewer
ran the tour suite by hand (86/86).

The implementer discipline already asks a lane changing a public
signature to run the two demos' clippy by hand; nothing asks for
their tests, and a behaviour change with no signature change gets no
demos run before merge at all.

## The shape to give

Seed the demos row from the crates the demos depend on (the way the
`topo` row is `seeded topo`), or at least from the crates whose public
doors the tour calls; or state in the discipline that a kernel door
change runs the tour suite before push. Weigh against the row's cost.

## Evidence: the same gap for a tree-wide reader (2026-10-02)

PR 3821 (`51f0b1f29`, `crates/viewer/src/idpass.rs` `NameAndPath`) gated `PKGS=viewer`, so `pncad-py`'s `prose_census` (which reads every crate's source and depends on none of them) never ran; main went red on `every_site_this_census_cannot_decide_is_named_with_its_reason` and the next PR whose closure held `pncad-py` (3803) heard it. That is the cost `work/ciw/latency-cut.md` accepted when it dropped the read reach ("the other seven a PR hears about when it touches their crate, or from the nightly"); main's push runs only prime the cache, so main reads green meanwhile. Fixed by `tquery/prose-census-idpass`.

## Evidence: the mirror direction, a tour stop arriving (2026-10-02)

PR 3787 (`fd031b83c3`) added the `snowman` stop. Its diff was `demos/`,
`docs/tess-budget-data/`, `tools/` and `work/` only, so
`scripts/ci-filter.py` classified it `TIER=aux` with empty `PKGS`, and
`crates/pncad/tests/all.rs::the_north_star_audit_has_a_row_for_every_tour_stop`
(which reads `demos/tour/src/` at run time) did not run. Main went red
on it. This guard can only be tripped by a demos diff, and a demos diff
never puts `pncad` in the closure, so as filtered today it can never
fire on the PR that trips it. The fix may differ from the kernel→demos
direction: the guard could run on the demos row, or `demos/tour/src/**`
could select `pncad`'s `test(north_star)` pair. Fixed by
`show/snowman-audit-row`.
