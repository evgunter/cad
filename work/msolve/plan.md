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
5. **`mate-lever-needs-the-parts-extent`** — asked on `[ev]` PR 2086
   (2026-09-06), ruled B by Ev (2026-09-07): the extent resolved from
   the mated part's own evaluated body. Lands as **`MSOLVE-6`**, spec
   `docs/MSOLVE-6-SPEC.md`; the item is parked on it.
6. **`aq8-skip-half-is-cited-as-ratified-and-is-not`** — closed
   (PR 1914): the SKIP half was ratified on PR 592's addendum and now
   sits in `ASSEMBLY.md`'s AQ8 clause; no `[ev]` was needed.
7. **`mate1-sweep-inferred-a-remap-from-a-refuted-reachability`** —
   closed as a record correction (2026-09-06); the finding is the
   record.
8. **`MSOLVE-5`** — the at-rest gate asks the operand's own table
   before it says a name vanished, and refuses a mate read below a
   product root in the operand's voice (`ReadBelowARoot { at }`);
   `NodeGone` deleted. Spec `docs/MSOLVE-5-SPEC.md`. Answers
   `assembly-gate-refuses-vanished-on-a-mate-read-below-a-pattern`
   (MSOLVE-1's review, NOTE-4). Dispatches from main after MSOLVE-3.

9. **`MSOLVE-6`** — the lever is the mated parts' own extent, read
   from each part's evaluated body through one reach trait; the edit
   door takes the reach (Ev's ruling (a) with the replay refinement on
   `[ev]` PR 2118: the log records the maintenance, replay never
   solves). Spec `docs/MSOLVE-6-SPEC.md` with its amendment. PR 2116,
   in review 2026-09-12. Closes `mate-lever-needs-the-parts-extent`
   and `reconcile-solves-with-no-resolver`.

**Routed onto this slate while the orchestrator was idle
(2026-09-08 … 09-12), triaged 2026-09-12, in the order they run:**

10. **`MSOLVE-7` — `member.rs` residue** (one lane, three items):
    `part-over-a-nested-pattern-reads-the-flat-index-at-check-reference`
    (EVAL-6: a `Part(k)` over a nested pattern's `Instances` selects
    the flat body `j·M + i`; `check_reference` reads `k` as the
    structural copy — a false refusal for `k ≠ j` and, until EVAL-6's
    seam landed, a silently wrong copy for `k = j`; the check's own
    account of the index space is owed, with the four-case row),
    `axis-datum-names-the-pattern-where-the-evaluation-names-the-transform`
    (the recipe road sites a dangling transform input at the pattern
    where the evaluation sites it at the transform; a sited refusal
    naming the transform), and
    `mate-solve-rebuilds-the-nominal-environment-per-check` (build
    `param_env` once per `solve_document`, pass it down — the same
    shape EVAL-9/10 gave the evaluator). Spec after MSOLVE-6 merges;
    the walk and the check are the files MSOLVE-2 and -6 just left.
11. **`MSOLVE-8` — `levered-clash-margins-hide-their-arm`**: three
    coset clash margins reach `Contradictory` with `lever: None`, and
    the socket is typed radians while a sine, a Frobenius departure
    and a reach are pure numbers. The decision the item names (a
    second arm in the sentence for dimensionless residuals, a typed
    unit, or a small-angle argument) is this program's; sequenced after
    MSOLVE-6 because the arm those margins would carry is the one it
    just changed. Small.
12. **`MSOLVE-9` — `mate-frames-resolve-from-a-face-at-evaluation`**:
    Ev's ruling (F) on `[ev]` PR 2256 — `MateFrame` gains a `FromFace
    { face, reference }` arm resolved at evaluation through
    `topo::readback::face_pose`; the solve runs over resolved frames.
    A design unit: it revises A11's inputs sentence (DESIGN.md wording
    drafted by this program and discussed with Ev before ratifying)
    and reaches every consumer of `MateFrame`. Spec last, on top of
    MSOLVE-6's reach road (the same `PartCache` answers both the
    extent and the face pose). LIB's façade and Python half follow it.

The exit walk waits on 10–12: the program closes when the lever, the
member residue, the margins' arm and the face-resolved frame are in.

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
