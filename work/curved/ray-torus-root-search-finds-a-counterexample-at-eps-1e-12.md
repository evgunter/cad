---
id: ray-torus-root-search-finds-a-counterexample-at-eps-1e-12
kind: issue
title: the ray-torus root search disagrees with its geometric oracle on a rare pose at eps = 1e-12
status: open
opened: 2026-09-16
---


## The finding

`r1_generic_poses_agree_with_the_geometric_oracle`
(`crates/topo/src/boolean/solid_contain/r1_generic_poses.rs`) is the
counterexample-SEARCH half of the ray-torus root claim, and it is
written with a VARYING seed on purpose — "successive runs explore new
poses instead of replaying one lattice forever", as its own doc says.
On 2026-09-16 it found a pose it disagrees with `r1_probes`' geometric
oracle on, in the `eps = 1e-12` row.

Seen on run `35152647206`, job `test (eps = 1e-12, 2/2)`
(<https://github.com/evgunter/cad/actions/runs/35152647206>), the only
red of that run's twelve `test (…)` jobs: `2361 passed, 1 failed`, the
failure being this row. The other eleven rows — including the other
two `eps` rows on the same shard and the same lane — were green on the
same archived binaries, so this is an `eps = 1e-12` pose and not a
build.

**It is not the branch that found it.** The run was
`edit/pick-t-interval` (EDIT-PICK3, PR #2786), whose diff is
`crates/editor-core/src/resolve/pick.rs`, `crates/pncad-py/` and
`crates/viewer/tests/` — none of them on this row's `gated_to!` list,
and `topo` does not depend on `editor-core` at all. The lane that hit
it is reporting it, not owning it.

## What is known and what is not

- **Not reproduced locally**: six runs of
  `CAD_TOLERANCE_EPS=1e-12 cargo test -p topo --lib -- boolean::solid_contain::r1_generic_poses`
  at the shipped `CAD_FUZZ_EFFORT` all passed. Each run draws its own
  poses, so a handful of local runs is a weak search — the hosted row
  draws four poses per shape per run across every PR, which is why it
  is the lane that found this.
- **The failing pose is in the job log**, not here: the panic text sits
  inline at test 1502/2362 of the job above, which is where whoever
  picks this up should start. It was not copied into this row because
  reading it costs a full-log fetch and the row's value is the
  existence of the counterexample, not a transcription of it.
- **Which half is wrong is open**: `line_torus_roots` and its `cbrt`
  chain (`crates/topo/src/boolean/solid_contain.rs`), the geometric
  oracle it is held against (`crates/topo/src/boolean/r1_probes.rs`),
  or the row's own agreement criterion in units of a band that moves
  with `eps`. The row asserts the CERTIFIED root count and the roots
  themselves, and `eps = 1e-12` is the row of the battery that narrows
  every band by three orders, so an oracle whose agreement window is
  written in those units is the first thing to read.

## Why it is worth a row rather than a re-run

A varying-seed search that reds once and passes on the re-run has
still found something: the pose exists in the space the row samples,
and the next run that draws near it reds again on somebody else's
branch. Re-running is how a real counterexample becomes a rumour about
a flaky test.
