# ASM-2B — multi-solid referenced products (binding spec)

Lifts ASM-2A's one deliberate restriction: an `InstantiatePart`
whose referenced document's product holds N > 1 solids currently
refuses typed naming this unit. After 2B, sub-assemblies and
multi-root parts instantiate. Everything else (nesting, naming,
memo, placements, refusals) shipped in #414 and does NOT reopen.
Binds A2/A3 (materialization), the #381 graft door, the #414 name
bridge. Difficulty: **M / structural** (pre-logged). Deviations
reported, never absorbed.

## D-1: lift the refusal, per-solid materialization

The instantiate evaluation path takes the referenced product's N
solids through `transform_rigid` + `graft_disjoint_all_keyed` as a
unit: one rigid map, N solids, N entries of graft keys, per-solid
+ aggregate validation exactly as the (already-shipped) loop shape.
The `MultiSolidRoot`-class refusal FLIPS (not deleted): its row
becomes the success assertion, and the refusal text's flip
condition is discharged in the same commit.

## D-2: name fidelity across all N solids

`RoleSeg::InPart` re-minting covers every stable name of every
product solid via the keyed graft (the #414 mechanism, N-solid
case). Uniqueness argument stays the master's-names-are-unique
one (#381's D-2). A doubly-nested multi-solid case is REQUIRED
evidence: assembly → sub-assembly (2 instances of a part) → part,
yielding 4 solids whose names are doubly `InPart`-wrapped, all
distinct, each resolving to its own copy.

## D-3: what does NOT change

No schema bump (no new fields or node kinds — v7 as merged, or
whatever main carries; state which). No new error variants unless
genuinely forced (tag arms mechanical if so). The single-solid
fast path stays bit-identical (a D9 row: a single-solid assembly
evaluated before/after this unit pins the same bytes and product).

## Acceptance rows

1. Refusal flip: the 2A multi-solid row asserts SUCCESS; reverting
   the lift reds it.
2. Sub-assembly e2e: workspace with part P (1 solid) and assembly
   B (2 instances of P); assembly A instantiates B twice at
   different frames → 4 solids, volumes 4×P bit-equal, solid order
   deterministic across two fresh processes (D9).
3. Names: the doubly-wrapped set from D-2, cross-wiring
   mutation-probed; persistence round-trip.
4. Placements and pins: SetPlacement on a multi-solid instance
   moves all its solids rigidly; the pin moves.
5. Single-solid bit-identity row (D-3).
6. Refusals unchanged: NoResolver / PinMismatch / ε-seam /
   ReferenceCycle rows still green (no regression).
7. Cold clippy: CI scope + `-p pncad-py --features python` +
   the interval graph. k-lint fires → report, never silence.

## Standing brief lines (verbatim obligations)

OUTPUT DISCIPLINE: ≤~150 lines per tool call, chunked reads,
skeleton-first writes, report ≤150 lines. Every build/battery row
a synchronous FOREGROUND Bash call, one at a time; NEVER arm
waiters/monitors/background chains for your own builds — poll a
harness-backgrounded call's output file with foreground reads;
a BLOCKING foreground wait on a busy slot queue is correct —
re-issue a timed-out call rather than parking, and kill targets
come from YOUR OWN recorded PIDs only, never pgrep
pattern-matching. Build tooling lives under `local-scripts/`
(with-build-slot.sh, new-lane.sh — the 2026-08-11 split). Merge
origin/main before opening the PR; re-merge on movement; confirm
checks START. Comments state the INVARIANT, not the history.
Commit and push after every coherent unit.
