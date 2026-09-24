---
id: lane-3-shell-lane-folded
kind: unit
title: LANE-3: ShellLane folds into AtRestPolicy — the shell door is a value the policy answers, the verb takes it, the witness is a function
status: closed
opened: 2026-09-21
closed: 2026-09-24
branch: scalar/lane-3
pr: 3049
---


## What

The last of the three kernel lane traits goes (`H5` §RATIFIED ruling
3: "`ShellLane` folds into `AtRestPolicy`"). `ShellLane`'s one required
method was `Some(verb.run_shell(operand, tol))` on the four certifying
arms and `None` on `Dual`; its provided `witness` was
`fold_shell_error(error, Self::end)` over editor-core's `Lane`. The
plan's sentence cannot be done literally (`topo` cannot name `verbs`;
`Lane` is a capability ruling 3 leaves alone), so the fold is by shape:
`topo::ShellDoor<T>` — one private fn-pointer field with
`topo::shell_open`'s signature, one constructor `ShellDoor::certified()`
at `Decide + CertifiedBounds + AtRestPolicy` — answered by
`AtRestPolicy::shell_door() -> Option<ShellDoor<Self>>` (`Some` on
`f64`, `Probe`, `Interval`, `Sym<T: CertifiedBounds>`; `None` on `Dual`,
each arm's reason moving with it); `verbs::Verb::run_shell` takes the
door by value; `wire_shell` reads the policy and keeps
`ShellLaneUnsupported { lane }` exactly on `None`; the witness becomes
a function over `Lane`; `EvalScalar` loses its eleventh term and the
`e4_dual_door.rs` set rows are re-written; the `NOT_CARRIED` roster
92 → 91. Every evaluation output at every scalar unchanged; the
refusal's name, tag and `Display` unchanged; DL3's list gains the shell
door beside the offset fit's (naming-only, the mechanism it describes).
Spec: `docs/LANE-3-SPEC.md` (deleted at merge). Block SCALAR-B6 slot 1.
Ground: the unowned `editor-core/src/verbs/shell.rs`, `lib.rs`,
`lane.rs` (prose); WIRE (`eval/mod.rs`, `eval/wire.rs`,
`verbs/src/run.rs`, `verbs/README.md`); the unowned `topo/src/props.rs`;
PROPS (DL3's list; `e4_dual_door.rs` with TCOST/TINT); TCOST/TINT
(`pncad/tests/all.rs`, the shell test files); GUARD
(`evalscalar-allowlist.sh`, naming-only); announced.

## Closed (2026-09-24) — PR 3049

`ShellLane` deleted with its five impls, the re-export, `EvalScalar`'s
eleventh term and the `NOT_CARRIED` entry (92 → 91). `topo::ShellDoor<T>`
beside `QuadLane` in `props.rs` — `Copy`, one private fn-pointer field
with `shell_open`'s signature, one `const fn certified()` at `Decide +
CertifiedBounds + AtRestPolicy`, a `pub` reader at `Decide` — answered by
`AtRestPolicy::shell_door() -> Option<ShellDoor<Self>>` (`Some` on `f64`,
`Probe`, `Interval`, `Sym<T: CertifiedBounds>`; `None` on `Dual`, each arm
stating its reason). `Verb::run_shell(.., door)` sits in the general
`impl` block, the second block gone: the guard moved from the signature
to the door value's provenance, and every route to a
`ShellDoor<Dual64>` is refused (both reviews, by execution).
`wire_shell` reads the policy and keeps `ShellLaneUnsupported { lane }`
exactly on `None`; the witness is `fold_shell_error_at`, a `pub(crate)`
function over `Lane`. Pinned: the pointer identity at `f64`, `Sym<f64>`,
`Probe`, `Interval`, the `Dual` `None`, a `compile_fail` doctest, a
25-row red-first; the corpus dumps and an authored cup byte-identical at
`f64`, the `Dual` refusal alone and identical. DL3's list gains the shell
door (naming-only). Reviews: dual, both APPROVE WITH FIXES, no MAJOR on
either arm, no tally candidate; nine items, none declined; the fence
extended for `real.rs`'s SEAT-9 paragraph (re-worded; the PROPS row it
had filed closed) and the enclosure census's header (four door values,
two rosters read); the SCALAR census row corrected and left for LANE-4,
which folds the four pin sites.
