# MESH-PROBEGATE — gate probe_stats at the module boundary (binding spec)

> **EXECUTED (#579), AND ITS SUBJECT NO LONGER EXISTS (#709).** This
> spec is the historical record of how `mesh::probe_stats` was gated;
> it binds nothing now, because SMELL-SCAN **S30** deleted the module
> and the `probe-stats` feature outright. Read it for the reasoning,
> not for the shape of the tree.
>
> What survives, and where: the live/inert module gate is
> `mesh::budget`'s (D-1's worked example became the only example). The
> per-triangle certificate falsification D-3 protects is
> `probe_review::z1_per_triangle_certificate_falsification`, which now
> drives the meter's deviation mode and asserts on `worst_ratio` — a
> per-face reduction of the per-triangle claim — in the SUITE rather
> than in the tessellation lane, so the `assert!` D-1 wanted deleted
> from default builds is deleted from **every** build. Its CI row is
> `mesh budget meter + certificate falsifier (feature = budget)`,
> still unconditional (M8-5's MIN-1). D-2's public-surface ruling was
> revisited: `mesh::budget` is now `pub` in both configurations
> because its measurement TYPES are the contract with
> `tools/tess-meter`, while `arm`/`take` remain live-half-only — the
> argument is in `mesh::budget`'s module docs.

Executes issue **#558** (the register-class residue #562/#560 left):
`mesh::probe_stats` is still unconditionally compiled and `pub`
(re-exported through `pncad`), so a shipped build carries the
per-triangle sampling block, its `assert!` in `trimmed`, and the
public arming API. Side-lane unit run by the ASM orchestrator on
Evan's direct assignment; pre-logged **S-M / STRUCTURAL**.
Deviations reported, never absorbed.

## D-1: the gate

The worked example is `mesh::budget` (#320 via #560): gate at the
MODULE boundary with a live/inert split — a probe feature carries
the live module; the inert half ships no-op signatures with
`armed()` folding to `const false`, so the tessellation lane keeps
**zero `#[cfg]`** in `tessellate.rs`/`trimmed.rs` and the
optimizer deletes the sampling block and its `assert!` from
default builds. Which feature: reuse the existing probe-class
feature if `mesh` already has the right one (`budget` is
budget-specific — judge whether probe_stats joins it or gets its
own `probe-stats`/shared `probe` feature; REPORT the choice and
why; the discipline allowlist and CI lanes must know the answer).

## D-2: the public surface

The default build exposes NO arming API: `pub mod probe_stats`
becomes feature-`pub` (`pub(crate)` or gone in the inert half —
mirror budget's shape), and the `pncad` re-export goes behind the
same gate or away entirely (check who consumes it: the suites arm
through `probe_stats::arm` in-crate; if nothing outside `mesh`
uses the re-export, delete it and say so).

## D-3: what must not move

- `cargo test -p mesh` default row keeps its full count; the
  self-arming falsifier `z1_per_triangle_certificate_falsification`
  still runs and still falsifies (it may need the probe feature in
  its row — if so, the hosted gate must still run it: touch the CI
  lane config consciously and state the wiring).
- Default-build output trees byte-identical pre/post (the #562
  measurement discipline — re-run the comparison or pin an
  equivalent artifact identity).
- The #562 discipline gate (no-ambient-environment) stays green.
- Zero `#[cfg]` lands in the tessellation lane files.

## Acceptance rows

1. Compile-error proof: a default-build reference to the arming
   API fails (the #560 E0603 pattern) — as a test or a documented
   executed probe in the PR body.
2. `armed()` folds inert: demonstrate the inert half is
   `const false` (compile-time evidence or the optimizer-visible
   shape), and the live half still arms.
3. The falsifier runs in the hosted gate configuration and still
   goes red on a planted certificate violation (execute the
   plant, then restore).
4. Default suite count preserved; probe-feature suite green.
5. Byte-identity or equivalent artifact-identity evidence for a
   default-build tessellation output pre/post.
6. Cold clippy: CI scope + interval; plus the probe feature's own
   clippy lane (the unreachable!-under-probe-features class bit
   twice before — clean it cold).

## Standing brief lines

As ASM-4-SPEC's, verbatim (OUTPUT DISCIPLINE; foreground rows;
poll harness-backgrounded output files; kill by recorded PID only;
local-scripts/ tooling; merge-before-open + re-merge on movement +
confirm checks START; invariant comments; commit+push per unit;
PR bodies from lane-private paths, never the shared scratchpad).
