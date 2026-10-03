---
id: topo-the-ops-cube-has-four-spellings-and-no-shared-home
kind: issue
title: The bare-topology ops cube (mev_line/mef_chord) is written out four times in topo, with no shared door
status: open
opened: 2026-09-16
priority: P3
cost: E
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

## A sixth spelling, and it is in another crate (2026-09-17, `dup/one-prism-builder`)

`crates/mesh/tests/r2_mesh6_probes.rs`'s
`r2_scaffold_strut_body_through_tessellate` (~`:168`) spells the same
bare-topology ladder — `mvfs`, three `mev_line`s off `MevSite::Lone`
then `MevSite::Fan`, the same
`find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)`, `mef_chord` over
`{he1: he_dc, he2: e_ab.he_plus}` — at the same unit corners, with the
struts left unroofed on purpose (the body is deliberately not closed;
that is the probe's subject).

**It changes this row's remedy, not just its count.** The other five
sites are inside `topo`, where `tests/common` is a home one of them
could reach. A `mesh` test binary cannot name `topo`'s `tests/common`
at all, so a shared `ops_cube` door that lives there does nothing for
it. The home that serves all six is
`crates/topo/src/test_support_impl.rs` behind `feature = "test-support"`
— which is link 3 of
`work/dup/brick-has-two-constructions-and-two-homes.md`, and this is a
second independent consumer arguing for that link.

Found by re-running this row's own instrument tree-wide at
`9ddd24842`, plus a shape grep that row did not use:
`git grep -n 'find_half_edge(seed'` with no path argument, which is the
cube ladder's one distinctive call and returns 35 hits over 21 files.
**What the shape grep cannot match**: a ladder that closes its bottom
cap by any other means, one that names the half-edge through a local
binding rather than `seed`, and the `mesh`/`sweep` builders that never
seed with `mvfs` at all.

Three in-`src` candidates the ladder census also returned, which this
row does not list and which have not been read:
`src/review_m1_pr2/cube_independent.rs`'s
`independent_cube_full_verification` (~`:28`),
`src/review_m1_pr2/atomicity.rs`'s
`raw_corruption_paths_leave_the_body_deep_equal` (~`:208`) and
`src/review_m1_pr3.rs`'s `build_box` (~`:159`). Candidates, not
members: the census counts call sites, so a looping fixture's counts
understate it and none of the three was opened.
