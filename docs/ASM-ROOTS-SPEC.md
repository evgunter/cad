# ASM-ROOTS — explicit product roots (binding spec)

Binds ASSEMBLY-DESIGN A10 (ratified 2026-08-10) at its part-document
scope: the ordered root list, its invariants, automatic maintenance,
and the whole-document product gather. The A11-era cluster-frame
rider on the component record is NOT this unit (its design
conversation, #356, is still open); nothing here may foreclose it.
Substrate anchors: cad-work/asm-r1-substrate/report.md §1, §6, §7.
Difficulty class: **M**. Deviations are reported in the PR, never
silently absorbed.

## D-1: the field and the schema bump

`Doc` gains `roots: Vec<RecipeNodeId>` (ordered, no duplicates).
SCHEMA_VERSION → 6, clean break per the standing precedent
(migration table stays empty; v≤5 refuses typed with the
regenerate recourse). Fixtures re-bless through their pipelines
only. The pin covers `roots` automatically (D-3 include-by-default
— state in a test that a root reorder MOVES the pin; order is
semantic because it is product-solid order).

## D-2: invariants, validator-enforced

Checked by the shared save/load validator AND at every edit apply:

- **Coverage**: every node is ancestor-of-or-equal-to some root
  (over the existing `inputs()` reference edges). No silently dead
  subgraphs.
- **Ancestor-freedom**: no root is a strict ancestor of another
  (typed refusal naming both).
- No duplicate entries; every entry names a live node.

A document failing these cannot save, load, or be produced by an
edit — the maintenance rules below make the invariant-violating
states unreachable through the normal doors; the validator is the
backstop, not the mechanism.

## D-3: automatic maintenance (recorded, deterministic)

Root-list updates are part of each `DocEdit`'s apply, recorded so
undo restores the prior list exactly:

- **InsertNode** consuming no existing node → the new node appends
  to the root list.
- **InsertNode** consuming one or more current roots → the new node
  REPLACES them: it takes the list position of the earliest
  consumed root; the other consumed roots drop out (tip transfer).
  Consumed non-root inputs change nothing.
- **DeleteNode** of a root → its orphaned direct inputs that become
  sinks re-root, in document order, expanding at the deleted
  root's position. DeleteNode of a non-root behaves as today.
- **Explicit override**: a new `DocEdit::SetRoots(Vec<RecipeNodeId>)`
  arm — validator-checked, recorded, undoable. This is the
  designate/undesignate door; no partial add/remove arms (one
  total edit keeps ordering explicit).

Every rule is a pure function of (document, edit) — no ambient
state, D9-deterministic.

## D-4: the product gather

New evaluation-layer door in editor-core:
`product(doc, &Evaluation) → Result<Body, ProductError>`:

- Walk `roots` in list order; for each BODY-DENOTING value —
  `Body`/`Boolean` (each of its solids), `Instances` (each placed
  body), `Split` (each piece) — graft its solids into one
  aggregate `Body`, per-solid, mirroring the step-import loop
  (per-solid validation when N>1, then aggregate validation:
  exactly the F8/D7 posture). Non-body roots (profiles, datums)
  contribute nothing.
- Provenance and names ride through untouched — `Instances`
  gathers must preserve `GeomSource::placed(node, i)` and the
  `Instance(i)` name wrapping already minted by the pattern.
- **No body-denoting root → typed `ProductError::NoBodyRoots`.**
- A root whose node FAILED evaluation → typed, naming the node
  (no partial products).
- If the per-solid graft loop hits a genuine kernel wall on some
  multi-solid source, scope down to what ships TODAY with a typed
  refusal naming ASM-2b as the flip condition — reported as a
  deviation, never silently narrowed.

`pncad` gains the document-layer convenience
`export_document_step(&Evaluation, &Doc)` = product → the existing
single-body export path when the product is one solid, and the
multi-solid STEP path used by import round-trips otherwise. The
existing per-node export door is unchanged (its multi-body refusal
stays correct).

## D-5: what this retires

The A10 text's promise becomes executable: `Instances` at a root
materializes into the product (the C1 disposition — `Node::Pattern`
semantics untouched, "patterns do not implicitly union" still
true: the product is disjoint solids, no boolean). The tour's
pattern demos gain a whole-document product without changing
their existing per-node consumption.

## D-6: Python surface — mechanical only

Tag arms for new error variants; NO new Python doors (recorded as
bindings-parity pickups). Python `Doc()` docs regenerate under v6.

## Acceptance rows (executable falsifiers, in-suite)

1. Maintenance: each D-3 rule its own row — no-consumer insert
   appends; consuming insert replaces at earliest position;
   root delete re-roots orphans in order; SetRoots overrides;
   each UNDONE restores the prior list exactly.
2. Invariants: ancestor-freedom refusal names both nodes;
   coverage refusal on a hand-crafted save text; duplicate entry
   refuses.
3. Gather: two disjoint extrudes → 2-solid product, volumes
   additive vs the parts; pattern (`Instances`) at a root →
   N solids with provenance indices and Instance(i) names
   preserved; Split at a root → both pieces present.
4. NoBodyRoots typed refusal (profile-only document).
5. Root reorder MOVES the pin; root-neutral edits leave the
   product solid order stable across two evaluations (D9).
6. v5 file refuses typed with regenerate recourse; fixtures
   re-blessed through pipelines, byte-stable across two blesses.
7. Cold clippy for touched crates; hosted CI green.

## Standing brief lines (verbatim obligations)

OUTPUT DISCIPLINE: ≤~150 lines per tool call, chunked reads,
skeleton-first writes, report ≤150 lines. Run every build/battery
row as a synchronous FOREGROUND Bash call, one at a time, reading
each result before the next; NEVER arm waiters, monitors, or
background chains for your own builds/tests; when the build-slot
queue is busy, a BLOCKING foreground wait is the correct state —
re-issue a timed-out call rather than parking (kill your own
previous waiter first, or use -n/--express). Merge origin/main
immediately before opening the PR and re-merge whenever main
moves; after any push confirm checks STARTED. If the k-lint gate
fires, do NOT change geometry to silence it — escalate. Comments
state the INVARIANT, not the history. Commit and push after every
coherent unit.
