# ASM-2A — `InstantiatePart`, single-solid parts (binding spec)

Binds ASSEMBLY-DESIGN A2 (materialized evaluation), A3 (the node),
A4 (pins at the seam), A9–A11 (cluster-level placement), and the
merged substrate: ASM-1's `DocRef`/`Workspace` (#364), ASM-ROOTS'
product gather (#383), ASM-2K's `graft_disjoint_all` (#381).
Difficulty class: **L** — the program's largest unit. Deviations
reported in the PR, never silently absorbed.

## D-1: the node

`Node::InstantiatePart { doc_ref: DocRef }` — a LEAF (no inputs;
`inputs()` empty), no frame field ON the node (A11: placement is
cluster data, not instance data). Extension points per the
substrate map: variant + ctor, append-only eval content-key tag,
wire dispatch, names emitter (D-4), export kind-handling
(an InstantiatePart node's own value is body-denoting — see D-3).

## D-2: cluster placement storage (the A11 registry rider, v1 form)

`Doc` gains `placements: BTreeMap<RecipeNodeId, Frame>` — keyed by
the instantiate node's id, because in the mate-less v1 every
instance is a SINGLETON cluster (A11's placement clusters; the key
generalizes to cluster representatives at R2, which is a recorded
follow-on, not this unit's problem). Missing entry = identity
frame (a legal, complete state — no refusal). New recorded edit
`DocEdit::SetPlacement { node, frame }` (undoable; refuses typed
on a non-InstantiatePart target). Improper frames (det = −1)
REFUSE typed in v1 — A6 admits them only behind the equivariance
audit, which is R4's named prerequisite; the refusal names it.
The pin covers `placements` automatically (include-by-default).

## D-3: evaluation — materialize through the shipped doors

- `evaluate` gains an optional RESOLVER (a trait object the
  document layer implements with `Workspace`); an InstantiatePart
  evaluated with no resolver refuses typed (`NoResolver`, naming
  the node) — kernel-layer tests use a stub resolver.
- Per instance node: resolver.resolve(doc_ref) → the referenced
  `Doc` (ASM-1's door already verifies the pin → `PinMismatch`
  surfaces through a typed evaluation error, not a panic; the
  ambient-ε reconciliation refusal at load IS the A2 ε-seam
  error — evidence row required, no new machinery).
- Evaluate the referenced doc at its own parameters (AQ4: no
  args in v1) and take its A10 PRODUCT (the gather — uniformity:
  what a document means is its product, one rule everywhere).
- **Single-solid scope**: a referenced product with N ≠ 1 solids
  refuses typed naming ASM-2b as the flip condition (the name
  bridge is 2b's; do not partially support multi).
- `transform_rigid` by the cluster frame (identity fast-path
  legal), then `graft_disjoint` into the evaluating document's
  materialization. The node's VALUE is the placed single-solid
  `Body` (body-denoting, so A10 roots/gather and the existing
  export door consume it with zero new arms).
- **Memoization**: a per-evaluation cache keyed by (pin, ε) so N
  instances of one part evaluate the referenced doc ONCE —
  in-process, distinct from both the pin vocabulary and the FNV
  memo `ContentKey` (three vocabularies, each with its
  do-not-unify comment). Cross-instance sharing of the EVALUATED
  part body is required evidence (a counter, not a timing claim).

## D-4: instance-qualified naming (the name bridge lands here)

The GQ4 wrapper composed with N1–N7: every stable name of the
referenced part's product body is re-minted under the instance,
as a new RoleSeg (working name `RoleSeg::InPart { node, of }`
wrapping the part-local `StableName`; the referenced document's
IDENTITY rides in the node's `doc_ref`, not in every segment).
Mechanism: extend the graft door's caller contract so the
solid/face/edge key mapping survives the graft (the `GraftMap`
bridge ASM-2K banked — make it `pub(crate)`-plus-accessor or
return it from `graft_disjoint`, implementer's choice, reported).
Resolution: an instance-qualified name resolves to the placed
copy's geometry; a part-document edit that breaks a local name
surfaces through the N1–N7 diagnosis ladder unchanged. Selector
and census vocabulary get NO new arms in this unit (recorded as
R2-era pickups) — but names must round-trip persistence.

## D-5: validity evidence (honest boundary)

Per-solid + aggregate gates exactly as the import loop; the PR
and tests must NOT claim inter-solid overlap detection (#382's
honest boundary — disjointness is per-solid tiers + aggregate
gates as shipped; contact/overlap arrive with R2/R3 census work).

## D-6: schema v7, clean break

The node variant + `placements` field ⇒ SCHEMA_VERSION 7,
migration table empty, v≤6 refuses typed with regenerate
recourse; fixtures re-bless through pipelines only. The save
validator learns the new referential checks (placements keys name
live InstantiatePart nodes; DocRef shape).

## D-7: Python surface — mechanical only

Tag arms for new error variants; NO new Python doors (parity
program's pickup). VERIFICATION MUST INCLUDE
`cargo clippy -p pncad-py --features python --all-targets -- -D
warnings` — the exhaustive tag maps only compile under the python
feature (ASM-ROOTS' one red; this line is now standing).

## Acceptance rows (executable falsifiers, in-suite)

1. E2E: author a part (extrude bracket), save into a workspace;
   an assembly doc with TWO InstantiatePart nodes at different
   frames evaluates to a 2-solid product; volumes bit-equal to
   2× the part's; solid order = root order (D9 across two fresh
   processes).
2. Memo: the referenced doc evaluates ONCE for the two instances
   (counter evidence).
3. Naming: instance-qualified names exist for both copies, all
   distinct; each resolves to ITS copy's geometry (cross-wiring
   mutation probe reds); names round-trip save/load.
4. SetPlacement: moves the copy (volume-preserving, position
   probe); undo restores; improper frame refuses typed naming
   the R4 prerequisite; non-instance target refuses typed.
5. Refusals, each its own row: NoResolver; PinMismatch at
   evaluate (stale pin after a part edit — A4's gate observed
   end-to-end); ε-mismatch at the seam (a bank doc with a
   different recorded ε); multi-solid referenced product naming
   ASM-2b.
6. Pin semantics: the assembly pin moves on SetPlacement and on
   pin-bump of a reference; an untouched-content re-save leaves
   it fixed.
7. Persistence: v7 round-trip incl. placements + the node; v6
   fixture refuses typed; fixtures re-blessed via pipelines,
   byte-stable across two blesses.
8. Cold clippy: CI's exact crate list AND the pncad-py
   python-feature lane; k-lint fired → report, never silence.

## Standing brief lines (verbatim obligations)

OUTPUT DISCIPLINE: ≤~150 lines per tool call, chunked reads,
skeleton-first writes, report ≤150 lines. Run every build/battery
row as a synchronous FOREGROUND Bash call, one at a time, reading
each result before the next; NEVER arm waiters, monitors, or
background chains for your own builds/tests — poll a
harness-backgrounded call's output file with foreground reads
rather than waiting on its wake (lost wakes are endemic); when
the build-slot queue is busy, a BLOCKING foreground wait is the
correct state — re-issue a timed-out call rather than parking
(kill your own previous waiter first, or use -n/--express).
Merge origin/main immediately before opening the PR and re-merge
whenever main moves; after any push confirm checks STARTED.
Comments state the INVARIANT, not the history. Commit and push
after every coherent unit.
