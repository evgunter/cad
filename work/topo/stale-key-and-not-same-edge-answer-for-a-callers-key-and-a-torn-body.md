---
id: stale-key-and-not-same-edge-answer-for-a-callers-key-and-a-torn-body
kind: issue
title: StaleKey, StaleGeometry and NotSameEdge are one variant each for a caller's key and a torn body, so reports_tier1_corruption answers true for a caller's mistake
status: dispatched
opened: 2026-10-01
priority: P3
cost: M
blocked_on: []
refs: [euler-op-corruption-refusals-end-in-a-tag, cycle-walks-refuse-loop-cycle-broken-for-a-stale-next-link, S14]
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

## Ruled

Ev, PR 4006, 2026-10-04 (quoted in full on `work/pipe/S14.md`): a state
that only a kernel bug reaches panics, by the current best understanding
however many parts it rests on, and that covers the whole torn-body
refusal class. S14 closes first, by staging the graft.

## The final state

Which input failed is known at every raise site from the code alone: an
**argument** (a key or pairing the caller passed) or a **record** (a field
the operator read out of the body). The shared lookups throw that away
today.

1. **A record miss is `unreachable!`** (D2 row 4). Its message names the
   record (holder, link, key) and why it cannot dangle. That covers:
   - the record half of `StaleKey`, `StaleGeometry` and `NotSameEdge`;
   - every walk that meets an unresolved link (so both directions of
     the probe above end alike);
   - every existing torn-body variant: `LoopCycleBroken`, `OrbitBroken`,
     `FanOrbitBroken`, `LoopNotCycle`, `EmptyAnchorsCollide`,
     `KillLeavesDangling`, `NotOwned`, `PcurveMint::Corrupt`,
     `readback::DanglingRef::Geometry`, `ShellError::Corrupt`,
     `ReplaceFaceError::Corrupt`, the boolean's `corrupt_at`, and the
     graft's torn-source `JoinDesync` ("graft source is not a well-formed
     body", `combine.rs`), which `voids.rs`'s `_ =>` arm still answers as
     "D9: never a panic on an error path" (PR 4022's review, NOTE-3).
   The proofs behind them stay; only the reaction changes.
2. **An argument miss stays typed (row 1):** `EulerOpError::Argument(BadArgument)`
   with `Stale { role, key }` and `NotMates { he1, he2 }`. It states the
   fact, with no defect claim and no recourse.
   This includes the graft's dead *destination* solid, a caller's key that
   PR 4022 refuses as `JoinDesync` (`combine.rs` `graft_staged`, "graft
   destination solid does not resolve"). At the void doors that becomes
   `VoidInsertError::Corrupt`, the same variant as a torn cavity. It splits
   off into the argument class (PR 4022's review, MINOR-1).
3. **One crate-internal lookup takes the key's source** (`KeyFrom::Arg(role)`
   or `KeyFrom::Link { holder, link }`), so no site resolves a key without
   saying where it came from. `Walk::Broken` names the hop that failed.
4. **A kernel driver passing a bad key is itself a kernel bug.** Its
   `From<EulerOpError>` sends `Argument(_)` to `unreachable!` and passes real
   operation refusals through. A driver's own lookups of keys it minted
   are `unreachable!` too, not a borrowed `EulerOpError::StaleKey`.
5. **`reports_tier1_corruption` is deleted.** `merge_faces`' `OpPlacement`
   loses its arena-fault arm (an arena read that fails now panics).
6. **The tests invert.** The torn-body sweeps in `review_d18` and the
   `corrupt input (release profile)` CI job assert today that a torn body
   refuses typed and never panics; they assert the panic and its message
   instead. Atomicity rows on real argument and operation refusals stay.

Sequencing: `graft-stages-into-a-fresh-body-and-commits-on-success` first.
The conversion is large, so split it by door family when dispatching.

The designers' two rounds recommended a typed `Torn` refusal; Ev chose the
panic. Their argument-vs-record split, the source-taking lookup and the
argument variant carry over.
