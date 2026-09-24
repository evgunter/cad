---
id: ray-torus-root-search-finds-a-counterexample-at-eps-1e-12
kind: issue
title: the ray-torus root search disagrees with its geometric oracle on a rare pose at eps = 1e-12
status: open
opened: 2026-09-16
priority: P0
cost: H
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
- **The failing pose, copied out of the job log before it expires.**
  An issue without its counterexample is not one, and a run's logs do
  not outlive their retention. The row is a torus at `R = 1`,
  `r = 0.9`; 20 rays drawn, 4 certified, 16 miss, 0
  uncertain/escalated, 2 disagreements — both on ONE ray:

  ```
  [fuzz] boolean::solid_contain::r1_generic_poses:
      seed=0x2ce3095461764e3a effort=1
      (replay: CAD_FUZZ_SEED=0x2ce3095461764e3a CAD_FUZZ_EFFORT=1)

  R1 root-count probe (generic poses): 20 rays, 4 certified,
      16 miss, 0 uncertain/escalated, 2 disagreements

  ROOT [generic] R=1 r=0.9
    o = (0.6165109851873778, -0.4322368608327216, -1.7665919240171966)
    d = (-0.7793351131793784, 3.340316168992811e-5, -0.6266073573218832)
    code -9.322586888006783e-1 vs oracle -9.322557990412323e-1
        (gap 6.11595378747298e-1)
    code -3.206575305754397e-1 vs oracle -3.206604202939344e-1
        (gap 6.11595378747298e-1)

  panicked at crates/topo/src/boolean/solid_contain/r1_generic_poses.rs:72:5:
  2 disagreements with the oracle at generic poses
  ```

  Two things to read off it. The ray is all but PERPENDICULAR to the
  torus's axis of revolution — `d.y = 3.3e-5` against components of
  order `1` — which is the pose where the quartic's two inner roots
  approach each other. And the two roots disagree by `2.9e-6` and
  `2.9e-6` while the reported `gap` is `0.61`: the roots agree to six
  digits, and it is the row's own agreement criterion, whose window is
  written in units of a band that narrows with `eps`, that calls
  `2.9e-6` a disagreement at `eps = 1e-12` and would not at `1e-6`.
  Whoever picks this up should decide which of the three is wrong
  before touching `line_torus_roots`.

  The `code` values are the roots `line_torus_roots` returns and
  `oracle` the geometric probe's; the pose replays from the seed.
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
