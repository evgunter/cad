# BOOL-6 — issue 368: the per-slab stacking fold in loft.rs

**Binding at dispatch** (S-BOOL program, `work/bool/plan.md`; difficulty
logged pre-draw: **M**). Read `docs/prompts/implementer-discipline.md`
in full before starting. The primary specification is the Q2 ruling
(Ev, in-chat, 2026-09-01 — `work/bool/plan.md` §Rulings, "decide now —
Helix is coming": a per-slab stacking fold with margin = min over
slabs, replacing the ends-only statement whose wall is exactly π) and
the issue item `work/bool/loft-stacking-trilean-is-end-to-end.md`
(issue 368).

## Situation

`crates/sweep/src/loft.rs`'s stacking trilean is END-TO-END: the mean
top-vertex displacement of the outer loop against the BASE normal,
decided once under `loft_stacking` (`Positive` builds, `Zero` refuses
`DegenerateStacking`, `Negative` refuses `ReversedStacking`). A loft
along a planar spine that turns past π therefore refuses as reversed
although every slab advances honestly (issue 368's executed signature:
lily leaf-A, 17 stations, curls 2.8 and 3.0 rad refused; the per-slab
margins already exist in the M8-14 meter). Helical sweeps clear the same
check through their pitch. The ruling: the statement becomes a PER-SLAB
fold — each adjacent section pair decided against ITS OWN base normal —
with the module's margin the MIN over slabs, so a curl past π builds and
a genuinely reversed or degenerate slab still refuses, naming which.

## FIRST, before the build — the consumer audit, reported

Enumerate every reader of the stacking verdict and of the end-to-end
displacement: `LoftError::{ReversedStacking, DegenerateStacking,
StackingEscalated}` and their consumers (`crates/sweep/tests/
review_m6_3_loft_probes.rs`, `crates/editor-core/tests/
lib_doors_node_result.rs`, `crates/pncad-py/src/tags.rs` — the tag
census), the module's Orientation paragraph, and anything that reads
the end-to-end value as a FEATURE (cap orientation? the seam carrier's
direction? the `Section` order recourse text?). For each: does it want
the per-slab min, the first slab, or the end-to-end value — and is any
consumer's correctness resting on the end-to-end sign (then the fold
must keep a stated end-to-end fact beside the per-slab one, or that
consumer changes with it). Report the table before building. Also
report the straddle set: spines whose slabs individually advance but
whose end-to-end dot is ≤ 0 (curls in (π, 2π)), and the M8-14 rows that
pinned refusals for them.

STOP conditions: a consumer that needs the end-to-end sign for a reason
the fold cannot supply; a slab decision that would need to read
anything but the two adjacent sections (never-infer: the fold is over
authored sections).

## Deliverables

1. **The fold**: for sections `k−1, k` the slab displacement (mean
   top-vertex displacement of the outer loop) against section `k−1`'s
   normal, decided under `loft_stacking` per slab; the loft's margin is
   the MIN over slabs; `Negative` on any slab refuses `ReversedStacking`
   NAMING the slab (a typed payload — the pair index — so the reorder
   recourse says which sections), `Zero` refuses `DegenerateStacking`
   naming the slab, escalation carries the slab. The two-section loft
   is the degenerate case of the fold and must decide bitwise as today.
2. **Cap and wall orientation** read the FIRST slab's normal for the
   bottom cap and the LAST slab's for the top (state it in the
   Orientation paragraph); nothing else about orientation moves — the
   D9 digest proves it.
3. **Rows**: the issue's executed signature (leaf-A-shaped planar spine
   at curls 0.45 … 3.0, then 3.5 and 4.0 past π) builds; a genuinely
   reversed middle slab refuses naming it; a degenerate slab refuses
   naming it; a two-section loft bitwise as base; helical sweeps
   untouched (bitwise); the M8-14 refusal pins flip to build rows with
   the reason at each; red-first every row against the unchanged fold.
4. **VERBS coordination**: `loft.rs` is sweep ground — announce the seam
   in `work/bool/log.md`'s next entry via the orchestrator (you do not
   edit `work/`); if a VERBS unit is live in `loft.rs`, report before
   editing (the orchestrator checks at dispatch: none is).
5. **D9**: the release tour byte-identical (`diff -rq`); MESH-4's digest
   identical at three ε rows; the corpus documents byte-identical; row
   counts move only by the rows added.
6. **ε posture** (issue 1356): no new key — `loft_stacking` decides per
   slab with the same margin form and band; state it; the Python tag
   census unchanged unless the payload adds a name (then the census row).
7. **Class sweep** (discipline §5): every other end-to-end statement in
   the sweep family that a per-segment fold should replace (extrude's
   direction check, revolve's angle headroom, the skin's stacking if
   any) — measure, report, do not act.

## Acceptance

The audit reported before the build; curls past π build; reversed and
degenerate slabs refuse naming the slab; two-section and helical cases
bitwise unchanged; D9 identical; hosted CI green; gate record per head.

## Hard rules

- NO `Co-Authored-By`, no model names; no closing keywords; "issue 368"
  spelled out (the orchestrator closes the item).
- Scope fence: `crates/sweep/src/loft.rs` (the stacking trilean, the
  error payloads, the Orientation docs), the sweep loft suites, the
  M8-14 refusal pins that flip, `crates/pncad-py/src/tags.rs` only if a
  payload name is added, `crates/editor-core/tests/lib_doors_node_result.rs`
  only if the refusal Display changes. NOT: the skin, extrude, revolve,
  the NURBS lift, `Section` ordering, the loft's cap/wall construction
  beyond reading the slab normals.
- Merges on green after the dual (no design surface: the ruling is
  Ev's; the slab payload's shape is stated, not asked).
- Re-merge main before opening the PR.
