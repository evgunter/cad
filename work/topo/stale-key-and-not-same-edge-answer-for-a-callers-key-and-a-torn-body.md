---
id: stale-key-and-not-same-edge-answer-for-a-callers-key-and-a-torn-body
kind: issue
title: StaleKey, StaleGeometry and NotSameEdge are one variant each for a caller's key and a torn body, so reports_tier1_corruption answers true for a caller's mistake
status: open
opened: 2026-10-01
priority: P3
cost: M
design: true
needs_ev: true
refs: [euler-op-corruption-refusals-end-in-a-tag, cycle-walks-refuse-loop-cycle-broken-for-a-stale-next-link]
---


(TOPO implementer, from `euler-op-corruption-refusals-end-in-a-tag`.)

## What

Three `EulerOpError` variants (`crates/topo/src/euler.rs`) are raised
both for a caller's mistake on a sound body and for a torn body, and
nothing in the value says which:

- `StaleKey`: an argument key that does not resolve (a key the caller
  kept past a kill, or another body's), and a key the plan read from a
  record (`next`, `prev`, a spine parent, a start). The shared lookups
  serve both: `Body::resolve_half_edge`, `Body::require_live`
  (`live.rs`), `Body::resolve_vertex_point`, and the `get_*`
  `.ok_or(StaleKey { .. })` sites, about 170 production raises across
  `euler.rs`, `euler_kill.rs`, `euler_ring.rs`, `attach.rs`,
  `movefac.rs`, `merge_faces.rs`, `null.rs`, `split.rs` and
  `splitting/classify.rs`, plus `sweep`'s own.
- `StaleGeometry`: a `FaceSurface::Shared` key the caller passed, and
  a point or surface key a record holds. `readback::DanglingRef` keeps
  the two apart (`Entity` against `Geometry` reached from a live
  entity) and its `From` maps both across onto these two variants.
- `NotSameEdge`: `kemr`'s two arguments equal, or halves of two edges
  (`Body::kemr`, `euler_ring.rs`, before `require_halves`), and every
  arm `require_halves` refuses through `Body::proven_mate` (`kev`'s
  and `kef`'s plans, `movefac`), which only a torn bijection reaches.

`EulerOpError::reports_tier1_corruption` answers `true` for all three,
which is right for a driver passing keys it read from the body
(`merge_faces`' `OpPlacement` delegates them) and wrong for a caller's
stale key. The unit that unified the corruption endings
(`euler-op-corruption-refusals-end-in-a-tag`) could not give these
three `geom_core::KERNEL_DEFECT_ENDING` without telling a caller with a
stale key that the kernel is defective, and no conditional text is
true at every raise site either, so they state the fact and claim
neither a recourse nor a defect
(`euler::tests::corruption_refusals_end_in_the_kernel_defect_ending`
pins both halves).

## The driver case

The caller is usually a kernel driver, not the person at the GUI. In
all 11 feature-tree chains that carry one of these variants
(`crates/editor-core/tests/refusal_concision_chains.rs`'s
`FILED_NO_RECOURSE`: `Boolean/Join/Euler`, `Split/Finish/Euler`,
`Split/Join/Euler`, `Split/Reduce/CrossingInsertion`,
`Split/Reduce/Euler`, `Blend/Op`, `Extrude/Op`, `Loft/Euler`,
`Revolve/Op`, `Shell/Partition`, `Shell/Rim`) the keys come from the
extrude, revolve, loft, blend, split, Boolean or shell driver. A
recourse to "pass keys this body holds" speaks to nobody the user can
be, and `Shell/Partition`'s wrapper (`ShellError::Partition`, "could
not be partitioned out (kernel bug)") contradicts it outright.

Sampling 12 `StaleKey` raise sites (PR 3621's review, C2) found three
shapes:

- caller-only: `mekr_both_empty`'s `get_loop(target)`,
  `set_null_face_pair`'s face and loops (`null.rs`), `split_edge`'s
  `get_edge` (`split.rs`);
- torn-only: `euler_ring.rs`'s `parent_loop` reads, `split.rs`'s row
  `Stale`, `Body::rebased_carrier`, `splitting/classify.rs`'s
  half-edge read;
- driver-minted: `sweep`'s `extrude.rs` (`base.hes[j]`, which the
  driver minted itself, and its rim `get_edge`) and
  `revolve/full.rs`'s seam `c_plus`. Here the body is sound and the
  driver's bookkeeping is the defect, so "the body is torn" is false
  as well.

Neither direction below fixes the driver case: keys a kernel driver
mints are a caller's keys under both, so the caller's variant would
still answer them. Only the wrapper (`ExtrudeError::Op`,
`ShellError::Partition`, …) knows the caller was the kernel, so the
wrapper has to say so, whichever way the variant splits.

## The question

Which of an operator's inputs failed to hold is known at every raise
site from the code alone. It is an **argument** (a key or pairing the
caller passed in) or a **record** (a field the operator read out of the
body: `next`/`prev`, `parent_loop`, a loop's `first`, a face's surface
key, an edge's mate slot). The shared lookups throw that away.
`cycle-walks-refuse-loop-cycle-broken-for-a-stale-next-link` has the same
cause: `Walk` tells `Broken` from `Overrun`, and `loop_cycle` merges them.

The designers' final state:

1. **`EulerOpError::Torn(TornBody)`**: the body is not tier-1-valid.
   - `Dangling { from, link, to }`: a record whose field names nothing.
     It takes the record half of `StaleKey`/`StaleGeometry` and every
     walk that meets an unresolved link, so both directions of the probe
     above refuse the same value.
   - A bijection arm beside `UnclaimedHalfEdge`: the record half of
     `NotSameEdge`.
   - `LoopCycleBroken`, kept only for a walk whose links all resolve but
     that does not close.
   - The existing corruption variants, moved in unchanged.

   Its Display ends in `KERNEL_DEFECT_ENDING` once.
2. **`EulerOpError::Argument(BadArgument)`**: `Stale { role, key }` and
   `NotMates { he1, he2 }` (`kemr`'s pair). It states the fact, with no
   defect claim and no recourse.
3. **One crate-internal lookup** takes `KeyFrom::Arg(role)` or
   `KeyFrom::Link { holder, link }` and builds the refusal from it.
   - `Walk::Broken` carries the hop that failed.
   - `require_live`, `require_halves`, `proven_mate` and
     `resolve_vertex_point` take the source they are given.
4. **`KernelCalled(EulerOpError)`** is the field type of every kernel
   driver's wrapper (11 wrapper types, 15 fields today: sweep, boolean,
   splitting, merge_faces, shell, replace_face, step-import).
   - Its Display, written once in topo, ends `Torn` and `Argument` in
     the defect ending. A bare `write!(f, "{source}")` is therefore
     right, and so is each driver's `From`.
   - A bare `EulerOpError` field reads as "this door forwards a caller's
     keys". None does today.
   - `Reading` stays a render parameter, because step-import reads at
     `Adopt`.
5. **Drivers' own lookups** of keys they minted use the driver's own
   defect variant, not `EulerOpError::StaleKey`. `ShellError::Corrupt`
   is the precedent.
6. **`reports_tier1_corruption` is deleted.**
   - "Is this a kernel defect" is `KernelCalled::is_defect`, meaning
     `Torn | Argument`.
   - `merge_faces`' `OpPlacement` keeps its exhaustive match and reads
     an `Argument` from keys it read off the body as an arena fault.
   - The `FILED_NO_RECOURSE` admissions for the Euler chains in
     `refusal_concision_chains.rs` retire.

**For Ev:** the sentence this PR adds to the D2 addendum's rows 4/5 note
in `docs/DESIGN.md`. It records what the tree does today: row 1 says
"reachable by input" and a torn body is not, while row 4 needs a proof
made in the same call. The code above does not depend on it.
