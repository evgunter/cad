---
id: a-pierce-strut-after-half-may-be-moved-by-the-other-run
kind: issue
title: vtxfac's bisector-only pierce run reads its strut corner 'after' off the entry table after the other run's fan may have moved it
status: dispatched
opened: 2026-10-05
priority: P2
cost: M
branch: cleave/pierce-strut-after
---


Found by the sweep in the PR closing
`work/topo/rc-wide-battery-panics-at-the-orbit-step-unreachable`, which
was the same shape in `insert.rs`: a later run at one vertex reading a
sector half that an earlier run's fan had already carried to its copy
vertex. Not reproduced; a question to prove or refute.

## The read

`crates/topo/src/boolean/vtxfac.rs`, the piercing-side loop over
`runs` (up to two Out-runs at the piercing vertex, minted in turn): a
run with no real-edge member hangs a strut at
`after = entries[(run.0 + run.1) % n].he`, read off the entry table
built before either run was minted. `mev_null` then splices the spike
at `after`'s start vertex, whatever that now is; nothing checks it is
still `vertex`. (A run with real-edge members reaches `mev` through
`run_site`, so a moved half there refuses `Euler(FanStartMismatch)`.)

## Why it might be reachable

Entries are pieces of physical sectors; twins share their sector's
`he`. If the first run's last real-edge member is the entry that crosses
into physical sector X (so its fan moves `X.he`), and the second run is
bisector-only inside X with a non-Out twin between them, then the
second run's `after` is `X.he`, already at the first run's copy. The
strut then hangs at the copy, mis-placed silently: the same state that,
in `insert.rs`, the join refused downstream as `JoinDesync` on all 56
battery poses that reached it.

## Done when

Either a proof in `vtxfac.rs` that the two runs never share a physical
sector this way, or the same typed refusal `insert.rs` `mint_directed`
gives (`ClassificationInvariant`, "an earlier run at the vertex carried
a strut's corner to its copy") when `after` no longer starts at
`vertex`, with a witness pose if one exists.

## Built (branch cleave/pierce-strut-after)

**Proved, not refused: the two runs never share a physical sector this
way.** The witness above does not exist. A bisector-only run is one
entry, the end bound of physical sector P's second piece (a sector
holds one bisector), so `after = entries[(run.0 + run.1) % n].he` is
the chord of P's orbit successor Q, not of P; and since runs are
maximal, neither P's real entry nor Q's is Out. The pose described
(the first run's last real member is X's chord, the second run X's
bisector, a non-Out twin between) cannot occur: X's chord and X's
bisector are adjacent entries, so both Out makes them one run. The
other run's mint moves only halves whose real entries are Out and
splices only before its first half and before the orbit successor of
its last (or, if it is a bisector run too, before its own `after`, the
successor of a different sector), and none of those is Q.

`vtxfac.rs`'s bisector arm states that invariant at the site and
checks it: `after` still starts at `vertex` and is still the orbit
step from the run's own sector half, else `unreachable!`. A fan that
moves one half past its run's last member trips it (mutation, run
once and reverted).

**Measured.** Main's suites never mint a bisector run second: with a
probe at the arm, all 508 sweep-suite hits, the 162 in
`pierce_runs_battery`, the 114 in `corner_pairs_battery`, the prism
profile turned through its six starts and mirrored, and the L
prism built by a subtraction all mint it first, because the fixtures'
orbit start puts the reflex sector's bisector at entry 2 of 4.
`vtxfac.rs` `a_bisector_run_after_the_other_run_keeps_its_corner`
moves the reflex corner's orbit start through all three halves. One
start puts the bisector entry last, so the bisector run mints after
the fan (the probe confirmed it, 6 of the 18 ops). Every op in both
orders, at every start, gives the same volume and keeps
inclusion-exclusion. The check only reads, so these are main's answers
too.
