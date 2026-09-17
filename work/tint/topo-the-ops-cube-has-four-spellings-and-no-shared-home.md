---
id: topo-the-ops-cube-has-four-spellings-and-no-shared-home
kind: issue
title: The bare-topology ops cube (mev_line/mef_chord) is written out four times in topo, with no shared door
status: open
opened: 2026-09-16
---


## Finding

- **Where**: `crates/topo/tests/cube_by_hand.rs` (`build_cube`, ~`:55`),
  `crates/topo/tests/review_m3_pr1.rs` (`ops_cube_public`, ~`:190`, and
  the ring-grown box in the function above it, ~`:136`),
  `crates/topo/tests/m3_pr1_surgery.rs` (`cube_with_inner_box`, ~`:23`)
  — with `crates/topo/src/lib.rs`'s crate-level doc example (~`:92`) the
  fifth spelling.
- **Importance**: medium
- **Confidence**: sure the four spell the same 1 `mvfs` + 7 `mev` +
  5 `mef` sequence at the same corners; **unmeasured** whether the
  bodies are equal arena for arena
- **Raised by**: the `dup-cube-seq` lane (S-DUP), 2026-09-16, out of the
  shape instrument it swept its own class with

`tests/common/mod.rs` has a shared door for the **geometric** cube
(`cube_ops`, behind `geometric_cube` and `cube_into`). It has none for
the **bare-topology** cube — the same sequence through `mev_line` /
`mef_chord`, chord lines and placeholder surfaces, tiers 1–2 only. Each
of the four sites writes it out.

`m3_pr1_surgery.rs`'s copy says out loud what it is: *"The ops cube from
the crate example (structural geometry: chord lines + placeholder
surfaces; tiers 1–2 only)"*.

## Why no sweep had found it

Three instruments had run over this file set and none could see it:

- a **name** grep (`geometric_cube|mapped_cube|cube_into|GeoCube`)
  misses every one — not one of them names a shared door;
- a **prose** grep (`topo-tests-self-declared-fixture-copies-census`)
  finds only `cube_by_hand.rs`, whose prose is about being by hand;
- a **construction** grep on `.mev(` / `.mef(` misses all four, because
  they spell `mev_line` / `mef_chord` — the bare-topology doors, which
  are different functions.

What found them: parse every tracked `.rs` into top-level `fn` bodies
and count `{mvfs, mev|mev_line, mef|mef_chord}` per body, then look at
everything with `mvfs >= 1` and `mef >= 2`. That returns 36 sites
tree-wide; these four are the closed-box ones, the rest are laminae,
digons, tetrahedra, prisms and op-machinery fixtures. At `mef >= 3` the
list is 13 and **`cube_by_hand.rs` drops off it**, because four of its
seven `mev_line`s and four of its five `mef_chord`s are spelled inside
closures — the instrument counts call sites, not operators, so a floor
set by operator arithmetic is wrong by however much a fixture loops.

## What the taker owes

**Not a merge on sight.** `cube_by_hand.rs`'s whole subject is the
by-hand construction — it validates after every operator and asserts
counts, provenance and lineage — so a shared builder would delete what
that suite measures, exactly as a careless `geometric_cube` unification
would have. The same question has to be asked at each of the other
three. The likely shape is one shared `ops_cube` door for the sites
that only want the body, and `cube_by_hand` keeping its own.

The doc example in `src/lib.rs` is the one spelling that is clearly
right as it stands: a crate-level example has to read standalone.
