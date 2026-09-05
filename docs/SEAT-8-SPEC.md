# SEAT-8 — split migrates onto the Verb substrate (unit spec)

Executes `docs/VERB-SEAT-DESIGN.md` §2 V4 for `Node::Split` — the
fourth migration, and the first whose verb yields TWO results. The
SEAT-4 prime directive carries over verbatim: **substrate in, behavior
pinned** — wire format untouched, content tag numerically identical
(`Node::Split` is tag 7 today, `editor-core/src/eval/mod.rs`),
evaluation results, names and refusals identical. Faithful elaboration
of the ratified plan; self-merges. A deviation from a ledger answer at
implementation time is Ev-gated (an `[ev]` PR), never carried.

## What split is, measured (`crates/topo/src/splitting/mod.rs:581`, `editor-core/src/eval/wire.rs:2109`)

`topo::split(operand: &Body<T>, plane: &SplitPlane<T>, tol) ->
SplitResult { above: SplitPart, below: SplitPart, naming }`, each part
`Body | Empty`, with the D7 pinch lane inside the door. The lowering:
one BODY operand (`target`), one datum-plane operand resolved upstairs
to `SplitPlane { origin, normal }` (the `WrongOperand` refusal is
document semantics and stays upstairs, like the boolean's
`resolve_declarations`), the run, then BOTH halves stamped in ONE
minted-index space (`stamp_minted_from` with a running counter — the
section planes are two descriptions with two sources, never one), then
`names::name_split(id, above, below, &naming, target, &target_table,
&body, normal, tol)`, then `ValuePayload::Split { above, below }` for
the DM3 projection node to pick from.

## S8-1 — the verb (`crates/verbs`)

`Verb::Split { plane: SplitPlane<T> }` — the plane is a kernel value in
the payload exactly as the boolean carries `declare`; the body stays an
operand (arity One). **The two-sided result is this unit's structural
decision**: `VerbOut` carries one body, `PairOut` typed the boolean's ∅
per door — split needs a per-door out-type that carries two
`Body | Empty` sides and ONE record (`SplitOut`-shaped, argued in the
PR: why a third out-type rather than widening `VerbOut`, and what the
DM3 projection contract needs from it). `VerbRecord::Split` carries
`SplitResult::naming` by value, never restated (a new variant is a
compile-forced visit — SEAT-5's fix made that real; state the visits).
`VerbError::Split(SplitError)`. `verb_content_tag` gains 7, pinned to
the pre-change constant with the injectivity and node-tag-space census
rows extended. `param_flow`: an explicit empty row with the reason —
the plane is a datum value, not a scalar parameter, and a section plane
has no stored scalar field (`SurfaceField` names none for planes).

## S8-2 — the correspondence and lowering (`editor-core`)

`verbs/split.rs` in the per-instance-data pattern (no vocabulary match
in the module): the operand resolution, the plane-from-datum reading,
the emitter (`name_split`) and the foreign-record sentence as data;
`wire_split` lowers through it and the split door. State whether this
is a fourth lowering or fits `wire_blend`'s one-operand shape — SEAT-5's
Row-6 honesty applies. The one-index-space stamping across both halves
is a write the lowering performs: it is a red-first mutation row (drop
the counter carry so the second half restarts at 0 — the digest must
red), not a comment.

## Acceptance

- Tag 7 identical; a saved document with a split (and a projection off
  it) round-trips byte-identical.
- Provenance-extended digest rows for a split corpus document — measure
  whether the corpus has one; if not, author one whose plane produces
  TWO bodies AND one whose plane misses the body (an `Empty` side), so
  the Empty token is a channel that can red (the SEAT-5 lesson, twice
  learned: every fed channel reds under its deletion mutation). The
  differential on the extracted merge base reproduces every constant.
- Red-first rows: the stamp counter carry; `stamp_minted_from` deleted;
  the record dropped; the plane's normal orientation flipped (the D7
  lane must still agree with the direct run — pin, don't assume).
- Both feature graphs; `python3 scripts/work.py lint` green; the
  costing table per design §6 (files touched per baseline row as
  amended by SEAT-5 and SEAT-7; what the two-sided out-type added to
  the substrate) and the updated next-verb statement.

## Out of scope

Shell (see `docs/SEAT-9-NOTE.md`); any change to `topo::split`'s math
or the D7 lane; the DM3 projection node; SELECT/flush; ParamSource
attach (no scalar flow exists here).
