---
id: whole-orbit-fan-end-has-three-spellings
kind: issue
title: The fan end next(mate(last)) is computed in three near-copies with three whole-orbit behaviours, and strut facing has two spellings
status: open
opened: 2026-10-02
priority: P1
cost: M
---

## What

Found in the review and fix pass of PR 3770 (`join/star-desync`).

**The fan end.** Three null-edge insertions compute a run's fan end as
`next(mate(last))`, each in its own copy. They answer a run that holds
every real edge of the orbit (`he2 == first.he`, an empty fan by
`MevSite`'s contract) in three ways:

- `boolean/insert.rs`, `mint_run`: refuses typed ("null-edge run spans
  the entire vertex orbit") after `run_degenerates` has tried the
  reverse run;
- `boolean/vtxfac.rs`, `classify_vertex_on_face`: records the strut
  `mev_null` builds (side Below, `he_minus` facing the start germ);
- `splitting/insert.rs`, `insert_null_edges`: records the strut with
  the copy as the below end.

**Strut facing.** Two spellings decide which spike half faces which
germ:

- `boolean/insert.rs`: the geometric `bool_strut_order` comparison
  (`spike_from_first` in `mint_directed`);
- `boolean/vtxfac.rs`: hard-coded (`he_minus` at the start germ). It
  serves two geometrically opposite placements: the bare strut of an
  Out run inside one sector (the `_` arm) and the whole-orbit strut
  in the In sector.

**Untested arm.** No test pins the bare `_` arm of
`classify_vertex_on_face`'s site match. In the PR 3770 review, all
four side/facing labellings of that arm passed the six sweep tests
that reach it.

## The shape to give

Give the fan end and the whole-orbit answer one home that all three
insertions call. Give strut facing one rule, derived from the splice
corner's arrival edge as `bool_strut_order` does, or proved equal to
the hard-coded one on both placements. Pin the `_` arm with a row
that one wrong labelling would fail.

## Built

**The fan end has one home.** `Body::run_site(first, last)` reads a
run's fan end, the orbit successor of `last` (`Body::orbit_step`), and
returns the typed `RunSite::WholeOrbit { corner }` when that successor
is `first`. The whole-orbit answer is the strut that
`mev(Fan { corner, corner })` splices into the one corner the run does
not cross. The splitter, the pierce and `insert::run_degenerates` build
that strut. The boolean planner mints it as the reverse empty run, so
`mint_run` still refuses a whole-orbit run as a run-selection bug.
Every caller matches on the variant.

**Strut facing has one rule.** In `insert::strut_faces_first`,
`he_plus` faces the germ the corner meets first, walking from its
arrival edge. Three readings decide it, in order:
- the structural vote (`strut_facing`);
- otherwise the germs' entry order (`precedes`);
- otherwise, for two germs in one entry, `strut_order`.

Both minters call it. On the whole-orbit placement it equals the old
hard-coded `he_minus`. On the bare-bisector `_` arm it does not: there
the hard-coded facing refused `JoinDesync` on every undeclared pose
that reaches the arm without a vote, and the walk order builds.

**Pinned.** `crates/topo/tests/join_pierce_strut_facing.rs` pins both
arms with exact volumes, and `body::tests::run_site_reads_the_fan_end_
and_names_the_whole_orbit` pins the home. Mutants turn them red; the PR
lists them. JOIN-1's batteries and `rc_wide_battery` are byte-identical
on main and head.

**Filed.** `a-pierce-with-two-out-runs-at-one-vertex-refuses-every-op`
(P0/H): the ring struts are a third facing spelling.
