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
    shape EVAL-9/10 gave the evaluator). Spec `docs/MSOLVE-7-SPEC.md`
    (2026-09-19), with item 14 folded in; EVAL-6 had already landed
    the flat-index decomposition, so the first item closes by
    citation and a ruling (the walk keeps the flat `Part`).
11. **`MSOLVE-8` — `levered-clash-margins-hide-their-arm`**: three
    coset clash margins reach `Contradictory` with `lever: None`, and
    the socket is typed radians while a sine, a Frobenius departure
    and a reach are pure numbers. The decision the item names (a
    second arm in the sentence for dimensionless residuals, a typed
    unit, or a small-angle argument) is this program's; sequenced after
    MSOLVE-6 because the arm those margins would carry is the one it
    just changed. Small. Spec `docs/MSOLVE-8-SPEC.md` (2026-09-19),
    with item 15 folded in: a closed `Lever { Roll, Residual }` enum
    (ruled), the witness at the frame read, the `MateFault` sentence.
    Dispatches after MSOLVE-7 merges.
12. **`MSOLVE-9` — `mate-frames-resolve-from-a-face-at-evaluation`**:
    Ev's ruling (F) on `[ev]` PR 2256 — `MateFrame` gains a `FromFace
    { face, reference }` arm resolved at evaluation through
    `topo::readback::face_pose`; the solve runs over resolved frames.
    A design unit: it revises A11's inputs sentence (DESIGN.md wording
    drafted by this program and discussed with Ev before ratifying)
    and reaches every consumer of `MateFrame`. Spec last, on top of
    MSOLVE-6's reach road (the same `PartCache` answers both the
    extent and the face pose). LIB's façade and Python half follow it.
    Ratified by Ev on `[ev]` PR 2895 (2026-09-20); dispatches from
    main after MSOLVE-10.
    Spec `docs/MSOLVE-9-SPEC.md` and the A11 sentence drafted
    2026-09-19 on an `[ev]` PR; dispatches after Ev's sign-off and
    after MSOLVE-8.

**Routed onto this slate 2026-09-13 … 09-17 by DOCM's exit sweep,
CHROME, CENSUS-INERT-DENY, EDIT and SCALAR's class sweep; triaged
2026-09-19:**

13. Closed at triage, as records — the tracker edit is the change:
    `memo-key-rows-tree-rs-citation-now-lands-on-the-opposite-claim`
    (the closed memo-key row now cites `downstream_of_mate` by subject
    and says the guard is gone because that row's own fix retired it)
    and `msolve5-read-below-a-root-rows-replaced-by-face-typed-rows`
    (ruled: an ordering over one question is empty; EDIT's typed head
    removed the kind question from runtime, the file's header says so,
    and the product ladder that remains is what MSOLVE-5 measured).
14. **Into `MSOLVE-7`'s lane**, a fourth item:
    `mate-primitive-accepts-a-stray-field-the-module-docs-say-refuses`
    — `#[serde(deny_unknown_fields)]` on `MatePrimitive`, the one
    field-bearing hole in the mate wire, under the persist module's
    own ruled policy (a stale reader must not silently drop data); a
    load-door row on the stray key. Small; it changes what a document
    accepts, which is why it was filed rather than taken, and this
    program's mate wire is where that is decided.
15. **Into `MSOLVE-8`'s lane**, two items beside the margins' arm:
    `subgroup-directions-are-unit-by-prose` (`coset.rs`'s `Subgroup`
    directions take `geom_core::UnitVec3` where the constructor
    already holds one — `parallel`/`perpendicular` lever a sine or
    cosine by the very arm MSOLVE-8 is about, so an unnormalized
    direction scales a decided margin silently; the carrier-field
    reads stay bare under `geom`'s at-rest rule, per SCALAR's
    ratified ruling), and `mate-fault-subject-spelled-in-three-crates`
    — **ruled here**: no `subject()` on `MateFault`. CHROME's evidence
    is decisive that a bare `Option<RecipeNodeId>` erases the one fact
    both consumers exist to carry (`Band` reaches every row of the
    document; `PosesOfAnotherDocument` reaches none), and a richer
    enum for two consumers, one already exhaustive, is a three-program
    change buying nothing. The row closes by the form it sanctions: a
    sentence on `MateFault` naming its two consumers
    (`viewer::tree::blamed_mates`, `pncad_py::MateFaultPayload`) and
    stating the asymmetry once, so it has one home instead of two
    comments. Lands with MSOLVE-8 because that unit rewrites the
    `Contradictory` arm's lever sentence on the same enum.
16. **`mate-clocking-has-no-gui-path`**, split by half. Half (1) — a
    nonzero clocking rider on `FrameCoincidence` is refused by the
    coset table STATICALLY, so `AddMate` refuses it typed at authoring
    time instead of committing an edit the next evaluation is certain
    to fail — is this program's, small, lands as **`MSOLVE-10`** after
    MSOLVE-8 (the door's refusal names the table's own predicate).
    Half (2) — how a mate's roll is turned: documented roll-reference
    conventions or a rotate-mate affordance — is MSOLVE-9's question
    in its kernel half: `FromFace { face, reference }` names a roll
    reference, and its spec states the convention the row says is
    undocumented; the affordance itself is CHROME's viewer seam and
    is handed there when MSOLVE-9's convention is ratified. Closed
    on PR 2913 with both halves recorded: MSOLVE-10 merged
    2026-09-20 (spec into the ledger at the unit head), the reviews
    settling the principle that the doors decide edits and the solve
    decides states.

**Routed onto this slate 2026-09-20 … 09-23 by PROPS, CHROME and
PORT; triaged 2026-09-24:**

17. **`msolve-9-spec-prescribes-an-untagged-wire`** (PORT) — rides
    with MSOLVE-9. The spec's wire sentence is amended on the unit
    branch: `MateFrame` is externally tagged and the tracked corpus
    regenerates, under Ev's ruling on PR 3123 and PR 2702's
    no-backtracking gate.
18. **`MSOLVE-11`** — the solve's escalations reach a node's log, and
    the refusals it sites name the node that failed. Gathers
    `mate-lane-escalations-reach-no-nodes-log` (PROPS, P1: the
    whole-document solve runs outside any node's bracket, and
    `coset::parallel` mints an `Indeterminate` by hand) and
    `placer-refused-names-the-pattern-for-a-part-index-that-does-not-evaluate`
    (CHROME, P3: `check_reference` sites a `Part`'s own index at the
    pattern below it — one condition, two seats, the class MSOLVE-7
    closed for the axis). Specs after MSOLVE-9 merges; both touch
    `mate/solve.rs` and `mate/member.rs`.

The exit walk waits on 10–12, 14–16 and 17–18: the program closes when the
lever, the member residue (with the wire hole), the margins' arm
(with the witness and the `MateFault` note), the face-resolved frame
and the static clocking refusal are in.

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
