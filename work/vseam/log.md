# VSEAM — log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/vseam/plan.md`. A/B band 5400–5499
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-17) — opened in VIEW's re-scope

Opened by VIEW's orchestrator as one of four successors on VIEW's
ground, per `work/README.md`: *residue is re-homed before the sweep …
to a new program opened for it when the residue coheres into a track of
its own (a dozen items on one territory are a successor's opening
slate, and the closing program opens it)* (Ev, 2026-09-06).

**VIEW is not closed by that act and is not closed by this one.** Its
`Order`'s six units are done, deferred or handed off; its ninety-four
live rows were review accretion on one crate, and four of them cohere
into tracks. VIEW stays open with eight rows, its exit walk is a
separate ratified step, and `docs/DOC-LEDGER.md` records the sweep when
it happens.

14 rows arrived from `work/view/`, each by `git mv` with its body,
its id and its history unchanged — no row's prose was edited on the way
past, and the item schema carries no `program:` field, so a re-home is
the move and nothing else:

- `adjacent-same-typed-arguments-are-the-same-swap`
- `evalservice-coalescing-rule-is-prose-no-implementor-is-held-to`
- `index-request-and-index-inputs-are-one-concept-twice`
- `new-document-owes-the-reframe-open-gets`
- `no-persistent-setplacement-session-op`
- `patternrulespec-is-a-partial-mirror-with-no-growth-alarm`
- `pick-priority-filter-vocabulary`
- `projection-fault-has-no-sweeper`
- `refused-a5-gate-eats-the-body-the-fit-then-regathers`
- `revolve-tool-unreachable-no-axisinplane-form`
- `session-save-is-two-acts`
- `the-two-drags-name-their-gestures-in-two-shapes`
- `ui-thread-work-after-the-index-seam`
- `viewerapp-document-derived-state-has-no-boundary`

The charter that makes these one program, and the sentence that is true
of them and false of the other three tracks' rows, is `plan.md`
§Charter. The band 5400–5499 is claimed in `docs/MODEL-AB-LOG.md` in the
commit that opens this program, per that entry's own rule.

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`
if the posture ever changes; under the inherited posture it records no
row.

## 2026-09-19 — Ev's Save As row landed (PR 2858)

Orchestrated from a session taking Ev's high-priority GUI rows across
the paused viewer programs. Not an A/B-protocol unit, by Ev's
instruction. The Fable implementer hit the account limit mid-work; Opus
finished from its uncommitted diff, keeping the rule, the helper, the
prefs key and the tests. It dropped a zenity/portal backend split, which
the headless test showed the probe cannot decide. The review was
style-only. Its fix pass took ten findings. The main ones:
- the door also owns remembering
- one home for the empty-parent rule
- the sibling prefs keys rendered through TOML, which fixed an older
  unescaped-preset bug
- a rename sweep over prose and open rows
- a GUARD row for the gate's `current_dir` blind spot
- the chooser-probe row restated to match its evidence

Candidate order kept as document → remembered → launch; flagged to Ev.
The profile-editor row is still in flight.

## 2026-09-19 — Ev's profile-editor row landed (PR 2862)

Orchestrated from the same session as PR 2858. This was not an
A/B-protocol unit, by Ev's instruction. The Fable lane hit the account
limit mid-work, and Opus finished the unit from its uncommitted diff.

It had a full review, correctness and style. The review found:

- **MAJOR:** the editor hid the slot rows, which were the only GUI door
  to driving an argument by an expression, per-slot units, and the
  range probe.
- **MINOR:** the greedy write order refused edits that had a valid
  order (7 of 52 in the reviewer's probe).
- **MINOR:** a disabled `DragValue` clamped a document's split-circle
  count to the authoring cap.

The fix pass fixed all three:

- the slot rows are kept, folded; the per-field version is filed;
- the write order is now an exact memoized search, capped at 12 writes;
- the clamp is off for document values.

It also:

- added the base-program guard in the op;
- merged the doubled preview pipeline into one;
- gave the lowering map one home;
- moved the editor into its own module;
- wired VGEOM's `except` seam.

The delta review held on all six checks. It ran on reading only,
because the build slot never freed in 1h45m. It found one new MINOR
(an outline freezes during a slot-row drag), filed rather than cycled.
Two NOTEs are recorded here:

- `ORDER_SEARCH_CAP`'s "fraction of a second" is unmeasured (worst
  case about 49k `apply` calls);
- the cap path's apply loop is a second copy of `commit_action`'s.

The shape lock waits on Ev (EDIT's whole-program-edit row).

## 2026-09-21 — the plan recorded a wait, a count and a parked PR, all stale

Found by the review of #2965, which was asked to check whether VSEAM's
Order still named a wait that had ended. It did, and three more claims
beside it.

| claim in `plan.md` | what was true |
|---|---|
| Order item 7 *"waits on `two-hand-written-copies-of-the-g1-gesture-machine`"* | that row's PR #2672 merged 2026-09-15; `g1.rs` has been on `main` since |
| §Inbound *"**Four** rows in `review`"* | the list under it holds **five** |
| §Inbound *"their lanes are in flight"* | all five PRs had merged before the sentence was written |
| *"#2762 is parked on a ruling from Ev"* | the ruling landed 2026-09-17 and #2762 merged carrying it |

All four corrected. The rows themselves were closed by PR #2976.

**The shape is one shape.** Every one of these is a *status recorded in
prose* that outlived the event it described, and prose has no lint. The
row files have `status:` frontmatter that `work.py lint` reads; a plan's
sentence saying a thing is in flight is checked by nobody, so it stays
true-looking for as long as no one re-derives it.

It compounded in a specific way worth naming: the cut (#2806) declined
to re-home these five rows **because** they read `review`, so a stale
frontmatter field became a stale plan sentence became a program's
schedule waiting on a trigger that had fired. Three artefacts wrong from
one un-updated field.

**What would have caught it.** `work.py lint` can see a row whose
`status:` is `review` while its `pr:` is merged — that is a query against
the tracker plus the forge, not a judgement — and it is the check that
would have fired six days ago. Filed as
`work/meta/a-row-in-review-whose-pr-has-merged-is-lintable.md`; META owns
`scripts/work.py`, so this program reports it rather than writing it.
