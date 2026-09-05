# MSOLVE — the mate solve's correctness residue (plan)

**STATUS: OPEN (2026-09-04).** Successor to S-MATE, which closed while
its residue was still being measured. Live state is `log.md`'s tail and
the item files beside this plan, never this file.

Branch prefix: **`msolve/`**. Away-channel tag `(MSOLVE orchestrator)`.

## Why this is not DOCM

DOCM took `mate.rs` and `mate/*` into its `paths` at S-MATE's exit, and
its charter is *"the editor-core document layer — the persisted recipe
vocabulary, the `DocEdit` set, document identity, and the frames and
selectors the viewer and the mate tool consume"*. That is **custody of
the files**, and it is the right home for the document-layer questions.

What is left over is not that. It is whether the solve computes the
right pose — a question about assembly semantics, answered by measuring
what a document evaluates to, with no document-vocabulary content at
all. Ev's steer, and the reason this program exists rather than four
items on DOCM's slate.

**The overlap is real and unresolved.** Both programs' globs name
`mate.rs` and `mate/*` until DOCM cedes them or declines to. That is
announced, not assumed — and it is exactly the state
`scripts/work.py territory` is blind to
(`work/issues/territory-cannot-see-a-path-two-programs-both-claim`), so
it will not warn anyone.

## The ruling this program runs on (Ev, in chat, 2026-09-05)

A mate reference carries the node it is read at, the shape the
measurement reference already has (`MeasureRef { at, name }`, one type
for both once MSOLVE-1 lands). That operand is an A12 READING edge —
stored on the mate, never consuming, so the mated bodies stay A10
roots — and the solve composes the map of every pose-bearing node
between the operand and the minting instance. N1 is untouched: a
transform still mints no segment, because a segment marks a new entity
and a transform moves one. The alternatives weighed and rejected are in
`log.md`'s entry of that date.

## The slate, in dependency order

1. **`MSOLVE-1`** — the operand on the mate, the walk, the transform's
   map composed, pattern-of-transform admitted; deletes
   `fix_xblind_probe.rs`. Spec `docs/MSOLVE-1-SPEC.md`. Answers
   `mate-solve-is-transform-blind`.
2. **`MSOLVE-2`** — the member chain: nested patterns, sibling
   distinctness at every level, the loop-closure rows. Parked on 1.
   Lands the rest of the PR 1731 ruling
   (`nested-pattern-mate-heads-refuse`).
3. **`MSOLVE-3`** — the `DanglingHead` catch-all closes: one variant
   carrying the evaluation layer's typed refusal. Ruled in by this
   program as S-MATE's successor; sequenced after 1 because the arm it
   replaces is rewritten there.
4. **`MSOLVE-4`** — a mate's memo key carries the solve's answer
   (`mate-memo-key-does-not-carry-the-solve`: a blamed mate reads `Ok`
   in the evaluation that blames it; CHROME's viewer-side guard
   retires with it). Spec `docs/MSOLVE-4-SPEC.md`; parked on 1 only
   for the shared key arm, dispatches at 1's merge.
5. **`mate-lever-needs-the-parts-extent`** — the mated parts' extent
   reaching `Alignment::lever_arm`; a schema question (authored beside
   the datum, or resolved through the part store) before it is a unit.
   `[ev]` when it is next in line.
6. **`aq8-skip-half-is-cited-as-ratified-and-is-not`** — a ratification
   for Ev: the SKIP half joins the weld half in `ASSEMBLY.md`'s AQ8
   clause with its condition stated. A short `[ev]` PR, not gated on
   the units.
7. **`mate1-sweep-inferred-a-remap-from-a-refuted-reachability`** — a
   record correction; lands with the first state-sync.

## Territory

Ev, in chat, 2026-09-05: touch whatever the units need and resolve
merge conflicts with DOCM if they arise. The overlap on `mate.rs` and
`mate/*` stands as announced; MSOLVE-1 also touches `node.rs`,
`edit.rs`, `refactor.rs`'s remap, the content key, the viewer's mate
tool and the Python mate door, and says so in its spec's fence.

## Where the extended vocabulary is written down

Answered without a PR: A11 rule (5)'s last sentence in
`crates/editor-core/ASSEMBLY.md` is the member vocabulary's ratified
home ("a reference head is a live `InstantiatePart` or a pattern's
`Instance(i)` …"). Each unit that extends the vocabulary edits that
sentence.

## Review posture

Inherited from FIX: **one style review per unit, plus a correctness
arm where a unit moves a kernel answer rather than its rendering.**
MSOLVE-1, -2 and -3 all move kernel answers. No A/B rows; the band is
claimed for bookkeeping only. This orchestrator runs on a remote box:
`[ev]` PRs get a PR subscription rather than the local away-channel
monitor.
