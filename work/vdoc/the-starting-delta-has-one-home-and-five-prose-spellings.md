---
id: the-starting-delta-has-one-home-and-five-prose-spellings
kind: issue
title: The starting delta has one home and five prose spellings nothing keeps in step
status: open
opened: 2026-09-21
priority: P2
cost: D
---



## Finding

`scene::INITIAL_DELTA` is `1.0e-4` and, since CHROME-ONE-NUMBER, has
exactly one home — the copy `crates/viewer/tests/display_budget.rs`
held is gone and the suite reads the constant. **The prose did not
follow.** The number is spelled out as `0.1 mm` in five places that no
longer have any connection to it:

- `crates/viewer/src/scene.rs`, `TRIANGLE_BUDGET`'s doc, twice —
  *"the same 0.1 mm the ..."* and *"The ring at 0.1 mm is ~1.6·10⁵
  triangles"*, plus a third mention in the same block's bullet.
- `crates/viewer/src/scene.rs`, `fit_delta`'s doc — *"tour's die at
  0.1 mm"*.
- `crates/viewer/tests/display_budget.rs`, the module header —
  *"at the starting 0.1 mm"*.
- `crates/viewer/tests/display_budget.rs`,
  `the_gallery_ring_at_the_starting_delta_is_inside_the_budget` — the
  row's NAME says "the starting delta" and reads `INITIAL_DELTA`,
  while its own `expect` string says *"the ring draws at 0.1 mm"*.
- `crates/viewer/tests/display_budget.rs`,
  `OVER_BUDGET_DELTA`'s doc — *"~1.6·10⁵ triangles at 0.1 mm"*.

## Measured, not speculative

CHROME-ONE-NUMBER's reviewer set `INITIAL_DELTA` to `2.0e-4` on the
fixed tree. Two rows went red
(`the_budget_commits_the_delta_it_always_has` and
`no_probe_out_tessellates_the_picture_it_sizes`), and **every one of
the five sentences above went silently wrong** — each now names a δ
the code does not open at, in a file the mutation compiled.

That is the whole finding: the fix that gave the number one home also
made the prose the only remaining hand-synced copy of it, and the
prose is the half no build can check.

## What is being asked for is a decision, not a refactor

**A doc-comment number cannot be derived.** There is no spelling of
`0.1 mm` inside a `///` that reads `INITIAL_DELTA`, so no mechanical
fix exists and this row should not be read as asking for one. The
choices, none of them free:

- **Say the name instead of the number** — *"at the starting δ"*
  rather than *"at 0.1 mm"*. Cheapest, and it costs a reader the
  magnitude at the point they are reading, which is often why the
  number is there.
- **Keep the numbers and accept them as prose**, with the row's cost
  recorded rather than paid.
- **Keep them and gate them**, the way the repo gates other
  measured claims — a check that a sentence naming `0.1 mm` in these
  files matches `INITIAL_DELTA`'s value. That is a real mechanism
  (`local-scripts/measured-claim-sweep.py` exists) and a real
  maintenance cost.

The `expect("the ring draws at 0.1 mm")` case is the one with no
argument on the other side: the row already reads `INITIAL_DELTA`
three lines above, so its own failure text contradicting the value it
ran at is pure defect whichever way the rest is decided.

## Home

VDOC — claims the tree makes about itself. Three of the five
sentences are in `crates/viewer/src/scene.rs`, which is outside
VDOC's declared paths, so per this program's keep_out a unit taking
this row announces the doc-comment edits to the owner of that file
(`scene.rs` is claimed by chrome, vgeom, view) and takes their word on
what the code now does.

## Found by

CHROME-ONE-NUMBER's style reviewer (PR 3022, 2026-09-21), on the tree
that gave `INITIAL_DELTA` its single home. Filed from that unit under
implementer-discipline §6.
