# CHROME log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/chrome/plan.md`. A/B band 1600–1699
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose CHROME section is the
charter this plan restates. Opens now. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `viewer-render-pipeline-creation-untested` from `work/issues/`
- `viewer-chrome-not-in-nextest-archive` from `work/issues/`
- `placed-union-has-no-session-op` from `work/issues/`
- `probe-bounds-lacks-driven-slot-guard` from `work/issues/`
- `pickindex-per-part-window-twins` from `work/issues/`
- `viewer-mate-tool-refuses-pattern-picks` from `work/issues/`
- `refused-mate-badges-every-instance-row` from `work/issues/`
- `doc-params-carry-no-display-unit` from `work/issues/`
- `viewer-first-light-on-real-hardware` from `work/issues/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## The slate opens (2026-09-04)

Orchestrator seated. Review posture is the plan's: batched style
review, no A/B row. Two units additionally carry a correctness lane —
`pickindex-per-part-window-twins` (a pure refactor whose failure mode
is #1098's silently-wrong name, so the lane's whole job is a
differential against the pre-refactor index) and
`placed-union-has-no-session-op` (a new op entering the replay/undo
vocabulary, so the lane's job is the round-trip). Nothing in the slate
is kernel ground and nothing takes an adversarial lane.

Three premises in `plan.md` re-checked against the tree before any
dispatch, per the dispatcher's-exposure rule:

- **Unit 1's precondition is met.** The `EdgePass` depth-bias fix is
  in-tree: the bias is applied in `vs_edge` as a relative clip-z
  shrink (`crates/viewer/src/gpu.rs:339`) and no pass requests a
  `DepthBiasState` on a `LineList`. The unit is the smoke row, not the
  fix.
- **Unit 8 is half-done, and its contended half is the half that
  landed.** `DocParam::Continuous` already carries `display_unit`
  beside `dim`, with `written_length`/`written_angle` as total
  authoring doors and the pairing checked by `persist::check`
  (`crates/editor-core/src/doc.rs`). So there is no new persisted
  field, and the GQ3-versioning announcement the plan schedules is
  owed on nothing. What remains is the PANEL half — which
  `crates/viewer/src/props.rs:34` states as the residue in its own
  module docs — plus one `DocEdit` door in `editor-core`, because
  `SetDocParam` is create-or-replace and a unit-only edit through it
  would silently delete a parameter's `Distribution`. That door is
  outside this program's `paths`; it is announced here rather than
  taken silently.
- **Unit 9's hardening PR merged.** `run()` builds an explicit
  `ViewportBuilder` (`crates/viewer/src/app.rs:5384`) rather than the
  bare `NativeOptions::default()` the 2026-08-28 comment names. The
  item's Ev-only residue is §2 (the culling flip, both pipelines) and
  §4's failure 2 and 3 (`R32Uint` clear semantics, readback cost on a
  real driver).

## First wave: four units on three PRs (2026-09-04)

`probe-bounds-lacks-driven-slot-guard` (PR 1746),
`viewer-mate-tool-refuses-pattern-picks` (PR 1748),
`viewer-render-pipeline-creation-untested` and
`viewer-chrome-not-in-nextest-archive` (PR 1755, one branch — they
interlock: one is a row, the other is where a row can run).

**The archive measurement, which `plan.md` names as the slate's only
decision.** Verdict: a row beside the app-feature clippy step under the
same `run_viewer_toolkit` axis, NOT `--features app` in the archive.
The archive is downloaded by every leg of the `test` matrix, so its
+179 MB is paid per leg and buys nothing for rows that already gate.
Numbers, both readings and their blind spots, are in PR 1755's body.

**Three corrections the lanes made to the orchestrator, all upheld.**
Recorded because the pattern is the point: every one of them was a
premise the orchestrator asserted and a lane checked.

- The dependency-graph delta was published as 71 → 265 crates. It is
  **65 → 211**. The orchestrator's `cargo tree | sort -u` did not
  strip `(*)` duplicate-subtree markers, so a crate appearing in two
  subtrees counted twice. Verdict unchanged; the number was wrong.
- The argument that the app-feature test row is nearly free in `fmt`
  ("that job already compiles this graph") rests on a false
  sub-premise: `cargo clippy` is CHECK semantics and leaves metadata,
  not rlibs. Consequence in
  `work/chrome/fmt-cache-carries-the-toolkit-codegen`, filed rather
  than decided — the deciding measurement does not exist.
- `viewer-chrome-not-in-nextest-archive` named ONE silent skip and
  there were three; and the cause the orchestrator suspected
  (`tests/all.rs`, `autotests = false`) was checked and cleared. The
  invisibility was purely the `#![cfg(feature = "app")]` inner
  attribute.

**What the style lane bought, stated plainly** because the posture was
chosen over the A/B protocol and should be judged. Twenty-two
findings. The three that justify the lane on their own were not
correctness findings at all: a PR body claiming a finding was
"recorded against that item" when the item was byte-identical to main
(the record existed on a branch that does not ride that PR); an item
whose title and cited cost were both false, so it scheduled finished
work while understating what was left; and `refactor.rs:1224`
restating the same vocabulary PR 1748 had just given one home, in the
crate that PR edited, self-declared in twenty lines of comment nobody
reads. None is reachable by asking whether a claim holds.

**The fence CHROME had to amend.** `keep_out` said *"editor-core
mate.rs and assembly.rs vocabulary is read and not edited"*, and PR
1748 edits `mate.rs`. The first disclosure named only "outside
`paths`" — the weaker boundary — which the style lane caught. The
clause is amended with its argument in `program.md`: the viewer's gate
was a RESTATEMENT of the rule `mate.rs` owns, so honouring the fence
would have preserved the defect it was written to keep out. This is
the session's one decision an orchestrator should not be able to take
alone, and it is recorded as such rather than buried in a diff.

**Items filed this wave**, all from lane or review findings that would
otherwise have existed only in a session's context:
`mate-member-vocabulary-restated-in-refactor` (issues/),
`session-gesture-guard-spelled-thirteen-times`,
`probe-rows-assert-in-one-direction-only`,
`fmt-cache-carries-the-toolkit-codegen`, plus four live residues
re-cut onto `doc-params-carry-no-display-unit`, whose scope shrank:
the persisted field the plan schedules an announcement for had already
landed in another program's PR.

**Inherited red, on PR 1748 only.** `pncad-py
tests::the_whole_tag_table_matches_its_committed_inventory` is red on
`main` itself (`work/lib/pncad-py-tag-inventory-misses-two-measure-tags`,
which asks for an instance rather than a repair — appended, not
repaired). PR 1746 is GREEN on the same base, so that item's "billed
to every code-tier PR" is too strong: it is billed to every PR whose
closure REACHES `pncad-py`. 1748 does only because it re-exports
through `crates/pncad`.

## Four of nine landed (2026-09-04)

Merged: `probe-bounds-lacks-driven-slot-guard` (1746),
`viewer-mate-tool-refuses-pattern-picks` (1748),
`viewer-render-pipeline-creation-untested` and
`viewer-chrome-not-in-nextest-archive` (1755). In review:
`placed-union-has-no-session-op` (1762, green),
`pickindex-per-part-window-twins` (1768). Dispatched:
`refused-mate-badges-every-instance-row`.

**The session's worst near-miss, recorded because nothing red it.**
PR 1755 sat CONFLICTING against main for three commits, and a
conflicting PR gets NO check runs at all — silently. Two pushes went
into the void; the PR looked fine, its last green run pointing at an
older head. What found it was ruling the alternatives out: other
programs' runs were healthy in the same minutes, and this program's
OTHER PR was gating normally, which left something specific to the
branch. `git merge-tree` against main then named the file.

Two lessons worth more than the fix. **The standard ways to force a
run are both forbidden here and would both have failed anyway** — an
empty commit and a close/reopen do not resolve a conflict. What
unblocked it was doing work that was genuinely owed (two filed style
findings), which forced a legitimate run attempt. And **the conflict
resolution was a UNION, not a side**: both branches had ADDED
different tests at one location, so taking either would have deleted a
test in a file the taker never edited — invisible to any diff review.

**A guard caught a guard.** The fix that bound the pipeline census
mechanically introduced a site reading Rust source as text, and
`reader_census` exists so such a site cannot arrive silently. Its own
docs gave the disposition: a new hand-rolled reader owes the SHARED
LEXER, not a ledger line. Routing through `test_utils::source::code_only`
also repaired a sensitivity the row already carried — the raw
`matches` counted the needle in comments and in the row's own
literals, so it answered about prose rather than calls. Both
directions controlled: a planted comment no longer reds it, and the
three real calls still count.

**Twice this session a green PR nearly merged without its style
review**, both times because green-and-mergeable reads as done. The
first catch was worth it immediately: that review found a PR body
claiming a record its own diff did not contain, and an item scheduling
work that was already finished. The posture costs a round trip and has
paid for itself every time.

## The landed units are closed, and this log reaches main (2026-09-04)

Six items moved `review` → `closed` for the five PRs that merged:
`probe-bounds-lacks-driven-slot-guard` (1746),
`viewer-mate-tool-refuses-pattern-picks` (1748),
`viewer-render-pipeline-creation-untested` and
`viewer-chrome-not-in-nextest-archive` (both 1755),
`placed-union-has-no-session-op` (1762),
`pickindex-per-part-window-twins` (1768).

**Why they needed a sweep of their own, and why that is structural
rather than an oversight.** A unit's state-sync rides that unit's own
PR, so the last status a unit can write for itself is `review` — at
the moment its PR is authored the PR is not merged, and `closed` would
be a claim about the future. Closing is therefore necessarily a
post-merge act with no unit PR left to ride, and if the orchestrator
does not carry it nobody does. The board read six CHROME items in
`review` with five merged PRs behind them.

**And the same shape had swallowed this log.** Everything above this
section was written on the orchestrator's own branch, which opens no
PR — so `work/chrome/log.md` on `main` still read *"No unit is cut and
no branch exists yet"* after six units had landed. That is the session's
recurring defect in its largest instance: **a record's home decides
whether it is a record at all.** It was caught three times at unit
scale (state-sync written on the orchestrator branch, twice after
being corrected once; then class findings nearly filed there) and each
catch fixed the instance rather than the practice. The sections above
are carried onto this branch verbatim, and from here the orchestrator's
narrative lands through PRs like everything else.

**Still open at this point.** `refused-mate-badges-every-instance-row`
(1769) — its style review found the PR incomplete against its own
item's second sentence (the prose half: reached rows still recite the
cause's full refusal text), so a fix pass is owed before it merges.
`doc-params-carry-no-display-unit` (1776) — green, style review out.
`viewer-first-light-on-real-hardware` (1771) — `[ev]`, and not a lane
unit: it is a checklist only a real GPU can answer.

## The prose-residue sweep, run on the six just closed (2026-09-04)

PR 1776's style review found the shape `work/README.md:100-106`
legislates against — a residue disclosed inside an item's own `Fixed`
prose and filed nowhere, which reads as a record of work done, is
invisible to the re-homing sweep, and dies when the program directory
is deleted. That finding is about one item, but the rule bites hardest
on items already CLOSED, so the six closed in this PR were swept
before it merges.

**Verdict: clean.** Every deferred thread in those six already has its
own file — `mispaired-ids-exempts-the-empty-window` (the `MispairedIds`
check still exempting the zero case),
`probe-rows-assert-in-one-direction-only`,
`session-gesture-guard-spelled-thirteen-times`,
`fmt-cache-carries-the-toolkit-codegen`, and in `work/issues/`,
`mate-member-vocabulary-restated-in-refactor` and
`viewer-free-move-misses-pattern-placed-mates`.

**Four disclosures were read and judged NOT to be residues**, recorded
because the judgement is the part a later reader cannot redo:

- `placed-union-has-no-session-op`'s "`DocEdit` carries no
  replace-or-convert variant … reported rather than crossed" is a
  REJECTED alternative, not deferred work: the pattern-consuming
  spelling was refused on an independent ground (two sources of truth
  for one rule), so the missing door is not wanted.
- `viewer-render-pipeline-creation-untested`'s standing cost — the
  app-feature run is red on a box with no Vulkan ICD — is a trap for a
  future third invocation, but it warns at the point of failure: the
  panic names `mesa-vulkan-drivers`. That is a durable home.
- `viewer-chrome-not-in-nextest-archive`'s measurement that the two
  `oracle-inari` suites execute only for a non-`push` event touching
  `interval-transcendentals/` is a fact about a ratified sampling
  posture, not a finding against it. Filing it would manufacture an
  issue out of a measurement.
- The same item's "what the pattern could not match" list names two
  suites gating per-ITEM inside ungated files. Those files still
  compile and their ungated rows still run, so the suites are not
  invisible the way the three viewer ones were.

**The general lesson, since this is the second shape of it today.** A
disclosure is not a schedule, and a record's home decides whether it is
a record at all. The rule `work/README.md` writes down for residues is
the same rule that stranded this log on a branch with no PR; both are
the failure to ask *who reads this after the session ends*.

## Unit 8's plan row was directing readers at work that does not exist

`plan.md`'s row 8 still read *"a display unit beside `DocParam`,
`SetParamUnit` mirroring `SetSlotUnit`; one persisted field under the
GQ3 versioning rule (announce)"*. The item was re-cut at dispatch —
the storage half had already landed in another program's PR, so there
is no new persisted field and the GQ3 announcement the row schedules
is owed on nothing — and the plan was never brought along. PR 1776's
style review found it.

Worth a line because of WHERE it sat. `plan.md` is the file a program
is executed from, so a stale row there does not merely go unread: it
tells the next reader to announce a persisted field that does not
exist. The correction points at the item rather than restating it,
which is the only version that cannot rot again — the item is where
the re-cut lives.

## Unit 7 landed, and what its CI run taught about reading green (2026-09-04)

`refused-mate-badges-every-instance-row` merged as PR 1769 after a fix
pass; its item is closed here. The style review had found it
incomplete against its own item's second sentence, and the second
blocking finding — `Poisoned`'s invariant strengthened on one of its
two producers — turned out to be a LIVE DEFECT the reviewer could only
mark `likely`, not the documentation gap the orchestrator described.
The document that exhibits it renders a boolean pointing at a row the
tree itself draws POISONED, reciting that row's copy of the fault: the
filed defect's own string, on a row whose badge denies it, one row
below the row denying it. Details in the PR.

**The run looked too fast, and checking why was worth it.** Both `test`
shards on the merged head finished in under half a minute, which reads
like the documented "green job name over skipped step" blind spot. It
was not. The change filter had scoped the run to the crates the diff
touches, so the two shards executed 230 + 230 viewer tests — the whole
viewer suite, split — in three seconds of wall time each. Two things
made that legible from the log rather than assumed: the slowest-20
table is entirely `viewer::all` rows, and `every_suite_file_is_aggregated`
is among the tests that PASSED, which is what forecloses a suite file
being silently absent from the aggregator. Recording the chain because
next time the timing will look wrong again.

**One report was genuinely absent and says so.** The "what this PR adds
to the test suite" block skipped: no run had published a listing for
the base tree in this lane. It states in its own output that nothing
may be inferred from the absence and that it gates nothing — which is
the right shape for a report that cannot run, and the opposite of the
silent feature-skip this program's unit 2 existed to close.

## Hand-off from DOCM (2026-09-04)

Five items re-homed here by header-preserving `git mv` (ids unchanged),
each a viewer build of a ruling in `crates/editor-core/REFERENCES.md`
or `crates/editor-core/IDENTITY.md` (both `docs/DOCM-*-DESIGN.md` at
the time): `add-profile-mints-no-frame` and
`add-profile-placement-on-picked-face-frame` (DM1/DM2; the kernel half
is `DOCM-1`), `save-a-copy-duplicate-id-bricks-store` (DI4),
`no-persistent-setplacement-session-op` (DI5: the gesture's release IS
the placement edit), `document-seam-no-in-session-change-detection`
(DI2: `Reevaluate` re-mounts the store; its two chrome edges). Signed
(DOCM orchestrator).
## Eight of nine landed; what the slate cost and what it found (2026-09-04)

`doc-params-carry-no-display-unit` merged as PR 1776 after a fix pass
and is closed here. That closes every unit the plan cut except
`viewer-first-light-on-real-hardware`, which is not a lane unit: it is
a checklist only a real GPU can answer, and it sits with Ev as PR 1771.
**The program cannot close until Ev runs it**, so the exit walk waits
on hardware rather than on work.

**The slate landed and the territory grew.** Twenty items are open in
this directory and exactly one is a slate unit (`viewer-first-light-on-
real-hardware`). Fourteen were filed BY the slate — style-review class
findings, fence artifacts, and residues the units disclosed — and five
arrived the same day as DOCM's hand-off, which is not this slate's
doing at all. That is not a failure of estimation. Every one has a `file:line` and a reason, and the
alternative was not fewer defects but the same defects unrecorded: the
`work/README.md:100-106` rule this program tripped over twice says
exactly that a residue with no file dies with the directory. Anyone
reading this as "CHROME ended with more open than it started" should
read it as "CHROME wrote down what it found."

**What the style-review posture bought, now that the slate is done.**
Ev's instruction was style reviews by default, correctness
double-checks only for particularly tricky units, and no A/B row. The
posture was right and the reason is specific: **on every unit that got
a review, the review found something the unit's own evidence did not.**
Not correctness bugs, mostly — a PR body claiming a record its own diff
did not contain; an item scheduling work that had already landed; a
`keep_out` disclosure naming the weaker of two boundaries; two residues
disclosed in prose the tracker rules say is not scheduling. None of
those is reachable by asking whether the code is right.

Two of them WERE correctness, and both came from the reviewer refusing
to accept a claim rather than from a correctness lane. The badge unit's
`Poisoned` invariant was strengthened on one of its two producers, and
the reviewer could only mark the consequence `likely` — the fix pass
built the document and found it not merely reachable but rendering a
self-contradiction. The parameter unit's probe seeded from
`committed_doc()` while searching `doc()`; latent, held up by three
invariants in three functions, and the review is why it is now a
construction. **A style lane that verifies claims finds correctness
defects as a side effect of checking whether the words are true.**

**The orchestrator was corrected seven times, all upheld**: a crate-count
delta published wrong (the `(*)` markers), a false sub-premise about
clippy's caching, a sweep miscounted four ways in one PR body, a
differential described as independent that was a transcription, a
`field_drag_tick` caller count wrong twice in a row, a `.speed(` sweep
given to a reviewer at half its size — and the paragraph above, whose
first draft said fourteen open and thirteen filed when the board said
fifteen and fourteen. Recording that one rather than quietly fixing it,
because it is the seventh instance of this shape and it happened INSIDE
the paragraph naming the shape. Reading a total off a table is exactly
the "asserted from a command whose semantics were not checked" move,
and knowing the pattern did not stop it; only the recount did. Recorded together because
the shape is one shape: every one was a number or a claim the
orchestrator asserted from a command it had not checked the semantics
of, and every one was caught downstream. The lanes are not merely doing
the work; they are the error-correction on the dispatch.

## The one item whose whole point is Ev seeing it was invisible to Ev

`viewer-first-light-on-real-hardware` carries `needs_ev: true` — on PR
1771's branch. PR 1771 is `[ev]`, so it waits for Ev and does not merge;
so the flag was not on main; so `STATUS.md`'s **Waiting on Ev** table
did not list it, and this program's `on Ev` column read empty while its
last open unit was a request for Ev.

`work/README.md` states the invariant this breaks: *"`STATUS.md` lists
every open `needs_ev` oldest first, so the two views (the PR list
filtered on `[ev]`, and the tracker) always name the same set."* Only
the PR view named it. Every other program's Ev-waiting item appears in
that table, so the convention is not in doubt — this one was missing
because of where its flag lived.

The flag is landed here on its own, **byte-identical to the frontmatter
1771 already carries** (`needs_ev`, `branch`, `pr`), so Ev's eventual
merge of that PR is a no-op on those lines and cannot conflict. The
PR's substance — the re-cut of what the checklist still asks — is
untouched and stays Ev's to accept: recording that a question is
outstanding is not answering it.

This is the same defect as the stranded orchestrator log and the
prose-only residues, and it is the fourth instance today. It landed on
the one item where the cost is not a lost record but an unasked
question, which is the argument for treating the pattern rather than
the instances.

## Parked against VIEW's split, and one item re-homed to LIB (2026-09-04)

Ev asked whether CHROME's remainder should move into VIEW or stay put
with the dependency recorded. **Stay put, parked** — and the tracker's
own vocabulary is the argument rather than a preference.

**Why not move.** `work/README.md` re-homes residue when a program
CLOSES; CHROME is not closing (it holds an open Ev unit and owns
`crates/viewer/tests/*`, which VIEW's `paths` do not cover). More
decisively, moving now would assert these are VIEW's work before
VIEW's unit 1 has ratified — and a module split is exactly the kind of
change that can DELETE a finding rather than inherit it.
`session-gesture-guard-spelled-thirteen-times` is the clearest case:
the split's own charter names "gesture-safety as data", which if ruled
that way dissolves the item instead of relocating it. Whether each of
these survives the split is the split's to determine, and CHROME's plan
already said the residue *rides* it — riding is not being absorbed.
Two of them also name `crates/editor-core`, outside VIEW's territory as
well, so moving them into VIEW would file work in a program that cannot
touch its ground.

**Why not merely say so in prose, either.** `parked` is a real status
and it requires `blocked_on` to be non-empty, so the dependency becomes
machine-readable and lint enforces that the reference resolves. A new
orchestrator reading the board sees nine parked rows naming their
trigger instead of nine open rows that look available and are not.
Prose in this log would have been the session's own recurring defect a
fifth time.

**What is parked**: the nine whose ground is `session.rs` or `app.rs` —
the two files VIEW unit 1 splits, and where its plan says *"Nothing
else in this program lands in those files before the split does."*
`parameter-row-field-has-no-text-door` carries a second trigger, the
`editor-core` door filed as `doc-param-unit-edit-has-no-door`.

**What stays open**, and is dispatchable today without touching VIEW's
ground: `probe-rows-assert-in-one-direction-only` (tests only — VIEW's
`paths` are `src/*` and the README), `fmt-cache-carries-the-toolkit-
codegen` (CI, names no crate file), `mispaired-ids-exempts-the-empty-
window` (`scene.rs`), `viewer-const-all-tables-have-no-exhaustiveness-
guard` (`combine.rs`), `edge-cost-claims-name-a-search-that-is-gone`
(`blend.rs`, `pick.rs`), `band-refusal-still-badges-every-row` and
`mate-fault-subject-spelled-in-three-crates` (both reaching
`editor-core`, which this program's `keep_out` fences — a slate for
whoever owns the kernel, not for CHROME), and the two `add-profile`
hand-offs.

**And one item was in the wrong directory entirely.**
`save-a-copy-duplicate-id-bricks-store` arrived in DOCM's hand-off, but
CHROME's `paths` do not cover `crates/pncad` at all, so the program
holding it could not have taken it. Its own Home section — written
before the hand-off — already named LIB, and its account of the fix is
an identity design question, which is the library contract's ground.
Moved to `work/lib/`. Its viewer half is a rider owed AFTER the
identity question is ruled, which is a hand-off in the other direction.

## Nine of nine: the hardware run, and two things it opened (2026-09-04)

Ev ran the first-light checklist on real hardware — Windows-native,
D3D12, Intel Iris Plus Graphics, driver 31.0.101.1999. All three
readings are answered on the item and in `gpu.rs`'s module comment,
which had said outright that culling was off because nobody knew and
instructed whoever found out to replace it. `FrontFace::Ccw`; the
`R32Uint` clear is clean; the blocking readback costs a median 0.803 ms
over 293 samples, about 5% of a frame, so the movement gate already in
place is sufficient. **The startup panic that opened this item does not
reproduce.** The scene and id passes now cull.

**Two of the three readings were binary tells and one was not**, which
the checklist did not say. The culling flip and the clear semantics are
settled by whether a thing is drawn and whether a message appears — an
operator's eye is the instrument and it is adequate. The readback cost
is quantitative in substance and the item asked it qualitatively (*"a
frozen or crawling frame rate"*); at 60 fps "fine" and "8 ms per hover
frame" look identical, and there was **no timing instrument anywhere in
`crates/viewer/src`** — no `Instant`, no frame-time, no counter, with
`IdQueryLog` recording a serial rather than a duration. Ev closed that
by adding a timing `eprintln!` for the run. A checklist mixing the two
kinds without marking which is which invites the third being ticked off
as "looked fine".

**Enabling culling made a visual property load-bearing for the first
time.** With `cull_mode: None` an inverted patch still drew, shaded
oddly but present and pickable; culled, it is absent from both the
picture and the id buffer — so a rendering fault now presents as a
MODELLING error. Nothing in this repo has ever asserted a rendered
pixel: PR 1755's smoke row deliberately builds pipelines and paints
nothing, and Ev's own run notes that the whole frame path still escapes
CI. Filed as `culling-is-load-bearing-with-no-pixel-test`, with the
lavapipe id-buffer readback named as the buildable close. Its sibling
is `chrome-weight-is-outside-the-palette` — two visual properties now
carrying meaning with nothing watching either.

**And the run's Vulkan aside was disclosed in prose and filed nowhere**
— the fifth instance today of the shape `work/README.md:100-106`
legislates against, arriving in the last item to close. `app::run`
states its window intent explicitly and its GPU intent not at all, and
that machine enumerates an Intel Vulkan adapter that access-violates
during device creation, which is not a catchable Rust error. Not an
observed crash — the default picked D3D12 and ran clean — but the
ingredient is demonstrated. Filed as
`viewer-expresses-no-gpu-adapter-preference`. The precedent is in that
same function: its window intent is stated rather than negotiated, and
the WSLg arm prefers X11 by hand, both added by this same first-light
item. The GPU half of the argument was never made.

**Where that leaves the program.** All nine units are answered. CHROME
does NOT close with them: a closed program may hold only closed items,
and this directory holds nine parked on VIEW's split plus the open
residue. The exit walk is Ev's to ratify and the re-homing depends on
what the split leaves standing, so the honest state is a program whose
slate is complete and whose residue is scheduled.

## The ninth unit closes; the slate is complete (2026-09-04)

`viewer-first-light-on-real-hardware` merged as PR 1771 with Ev's
hardware readings on it. Closed here, and `needs_ev` cleared: the
question it carried has been answered, so the flag would otherwise
leave the board saying CHROME waits on Ev for something Ev has already
done — the most misleading state a tracker can hold, and the one a
VIEW orchestrator would read first.

This is the closure gap PR 1778 set out, arriving one last time and in
its sharpest form. A unit's state-sync rides that unit's own PR, so the
last status a unit can write about itself is `review`; closing is a
post-merge act with no unit PR left to ride. For the eight code units
the cost of missing it was a stale count. For this one it would have
been a false claim about Ev.

**All nine units are answered.** CHROME does not close with them:
twelve items are open here and nine are parked on VIEW's split, and a
closed program may hold only closed items. The exit walk is Ev's to
ratify, and what the residue re-homes to depends on what the split
leaves standing — so the honest end state of this session is a program
whose slate is complete and whose remainder is scheduled against a
named trigger.

## 2026-09-15 — the program resumes, and the slate is re-read against eleven days of tree

Ev asked for an orchestrator on CHROME again, reaffirming the standing
posture in the same breath: **no A/B protocol, style-only reviews for
the easier units.** That matches what this program has done since it
opened, so nothing is being changed — but `docs/MODEL-AB-LOG.md`'s
roster line states it too narrowly (*"infra-only and test-only units in
CIW and CHROME record no row"*), and the program-level truth is no
duals at all. Recorded in `program.md` rather than by editing the
roster, which is shared ground and would be a second program's file to
touch for one sentence.

**The first act was not to dispatch anything.** The log's previous
entry is dated 2026-09-04; `main` had moved roughly eight hundred PRs.
Every one of the 29 live rows was re-read against the tree before any
unit was cut, in four parallel audits.

**Nothing was dead.** VIEW closed none of CHROME's rows in eleven days
of working the same crate. It moved the *addresses* of about twelve and
narrowed the stake of one. That is worth stating plainly because the
opposite was the reasonable expectation: a program dormant while a
sibling refactors its territory should expect attrition, and got none.

**What it did find is that rows rot in two ways, and the second is the
dangerous one.** The first is citation rot — line bands that now hold
unrelated code, and one row citing `crates/viewer/src/pick.rs`, a file
that no longer exists. That is visible the moment anyone looks. The
second is **premise rot**: `add-profile-placement-on-picked-face-frame`
asserted that the shipped tool authors on world XY only and that the
kernel deliberately answers no "is this face planar" verdict. Both had
been falsified by work that landed after the filing —
`SessionOp::AddProfile` carries a picked `RecipeNodeId`, and
`face_carrier_kind` exists and is reachable. A lane handed that row on
trust would have written a detailed, plausible fix for a defect that
is not there. Same shape one row over:
`mate-fault-subject-spelled-in-three-crates` counted three spellings,
where one had since been deleted by LIB and a second
(`tags::mate_fault_tag`) answers *which arm fired*, never *which mate*
— so it was never an instance of this finding at all. **One third of
that row was wrong on the day it was written.**

And the counts inside rows rot quietly:
`chrome-weight-is-outside-the-palette` says `ui.weak` is spelled 49
times in `app.rs`. It is spelled **three** times; the other 47 went to
`pane/*` in the split. I re-derived that one myself rather than take
the audit's word, because the whole point of this entry is that
asserted numbers decay.

## The VIEW carve-out — taken, not asked

CHROME and VIEW both claim `crates/viewer/src/*`, both `keep_out`s name
the other, and **both clauses were false.** CHROME's said *"CHROME goes
first"*; VIEW went first, eleven days ago. VIEW's said CHROME *"has
been dormant since 07:00"* and discharged the wait clause on that
basis; true when written, false the moment this session opened. Lint
cannot see either, because it checks that the pair is recorded on both
sides and not that the sentences are true.

A sequencing decision with a recommendation does not wait on Ev
(`memories/orchestration-model.md`), so: **the two programs divide the
files rather than take turns.** CHROME works `datums.rs`, `bounds.rs`,
`combine.rs::denotes_body`, `tree.rs::blamed_mates` and three test
files; it cedes `scene.rs`, `gpu.rs`, `theme.rs`, `pane/features.rs`,
`marks.rs`, `blend.rs` and the whole spine. The cessions are not
politeness — **each one is a file where VIEW holds an OPEN row on the
same ground**, and in one case (`renamed-module-leaves-citations-in-two-
other-programs`) a VIEW row claims a CHROME row by id and line number.
The alternative was two programs fixing one defect twice.

The carve-out cost CHROME three files it had been assigned that
morning. `scene.rs` and `gpu.rs` went because VIEW opened
`scene-mesh-carries-an-identity-index-buffer` *that same day*, and it
deletes one of the two sites `gpu-index-counts-substitute-u32-max` is
about. Whoever takes either should take both; neither program can see
that from its own slate alone. Ev is carrying the carve-out to VIEW.

## Four rows re-homed, one of them re-cut first

DOCM exited on 2026-09-14 and its territory divided, which left CHROME
rows pointing at ground that had changed owner underneath them. Moved
(`git mv`, ids unchanged, a `## Re-homed` section on each saying why):

- `fmt-cache-carries-the-toolkit-codegen` → **CIW**. Names no crate
  file at all; its whole subject is the `fmt:` job in `ci.yml`.
- `a-declared-union-has-no-one-pass-authoring-path` → **EDIT**. The row
  offered a viewer seat or a kernel edit as alternatives; only the
  second can exist, because **no `DocEdit` writes `declare`** — the
  whole vocabulary was enumerated to check.
- `degenerate-triangle-normal-is-substituted` → **MESH**. Its own
  preferred resolution is a guarantee sentence about what `crates/mesh`
  may emit. The viewer half resolves either way once the kernel answers.
- `mate-fault-subject-spelled-in-three-crates` → **MSOLVE**, *after*
  correcting its premise in place. Moving a row with a false premise
  only relocates the error, and this one's cost argument was arithmetic
  on a count that was wrong.

**One row filed**, from a finding one hop off the union row:
`addboolean-doc-names-a-vocabulary-that-does-not-exist`.
`SessionOp::AddBoolean`'s doc says a declaration *"is added afterwards
through the vocabulary that owns it"*. There is no such vocabulary, and
the sentence is the stated *reason* for a design decision rather than a
decoration. Filed rather than fixed in passing: the file is ceded to
VIEW, and the row records that the **class** — a comment promising a
door that does not exist — has never been swept in this crate, so one
instance is not evidence about the population.

## An orchestrator error worth recording: two lanes, one working tree

I dispatched the citation-repoint lane and the datums lane into the
**same checkout**, with no worktree isolation. The second lane found
the first's uncommitted work in its tree, correctly declined to
`git checkout` over it — that would have destroyed another lane's
work — and said so in its report. No work was lost, and only because
the lane was careful; nothing in my dispatch made it safe.

The fix is `isolation: worktree` per lane, and the orchestrator's own
edits in a worktree of their own. The reviews dispatched after this
point use it. Recording it because
`memories/agent-lane-operations.md`'s existing rule is about *target
directories* — a lane serving another lane's binary — and this is the
same failure one level up, in the source tree rather than the build
tree, which that rule does not name.

**A second correction to myself, smaller.** I told the audit lanes the
clone was shallow past 2026-09-13 and that history archaeology was
therefore unavailable. It has ~150 graft points and reaches 2026-09-02,
which covers the whole CHROME/VIEW period. Two lanes worked around the
caveat and one checked it and told me I was wrong. Asserted from one
commit's diff without checking what `.git/shallow` actually contained —
the same "asserted from a command whose semantics were not checked"
shape this log has now recorded eight times.

**Where the program stands.** 25 open and one parked on this branch,
after the four moves and the one filing; the repoint unit closes two
more and is in review. Nothing waits on Ev.

## Four rows re-pointed by subject; the two reports that found the rot are closed (2026-09-15)

`chrome/citation-repoint`. Six rows, no source file touched.

Four rows carried citations that rotted when
`viewer-session-god-module-split` (#1830) took `app.rs` and
`session.rs` apart. All four are re-pointed **by symbol name and
file**, with no line number written anywhere —
`docs/prompts/implementer-discipline.md` §7, and the shape
`work/view/citation-repoint-shifted-a-number-the-lane-knew-was-wrong`
set after a sibling lane shifted a number it had just declared wrong:

- `drag-tick-has-three-homes` — the finding was intact and every
  address dead. Its three homes are `forms.rs` (the rule and the four
  constants), `pane/properties.rs` (the two converted panel fields)
  and the hand-picking spread across `pane/create.rs`, `widgets.rs`
  and `pane/properties.rs`. Population re-derived: 45 grep lines, **38
  real call sites**, and the grep's blind spot named in the row.
- `parameter-row-field-has-no-text-door` — four bands past the end of
  `app.rs`; all four subjects are in `pane/properties.rs`. Still
  `parked`; its blocker moved to `work/edit/` and is open.
- `add-parameter-form-authors-canonical-only` — re-pointed, and **one
  supporting bullet struck rather than re-pointed**: the form's tick is
  no longer a hand-named constant, `add_param_ui` derives it from
  `FieldWriting::of`. The head claim (canonical-only DECLARATION) is
  untouched and still true.
- `add-profile-placement-on-picked-face-frame` — **premise re-cut, row
  kept open.** `SessionOp::AddProfile` takes a picked frame node, not
  world XY; `sketch::frames` already admits `Datum::FaceFrame`; and
  `names::interrogate::face_carrier_kind` answers the planarity
  question the row said nothing answered. What is actually missing is
  one seat: `DatumSpec` (`session/author.rs`) has no `FaceFrame`
  variant, so no chrome can mint the node every other layer accepts.

Closed with them: `drag-tick-row-cites-app-rs-for-a-finding-that-lives-in-forms-rs`
and `parameter-row-field-cites-a-pre-split-app-rs`, the two VIEW-filed
reports that existed only to say the rows had rotted. Every location
either of them named checked out. One sentence in the first does not:
*"`app.rs` holds none of them"* is off by a re-export path, which that
row's `## Closed` section records — and which a later review showed had
more behind it than this log first claimed (below).

Filed on the way: `add-profile-ui-doc-comment-states-a-premise-the-tree-falsified`
(the same two falsehoods, still standing in `pane/create.rs`'s own doc
comment, which the fence of this unit put out of reach), and a sixth
instance plus a correction added to
`work/issues/dead-work-citations-from-shipped-code-and-docs.md`.

## The citation-repoint fix pass: a repointing unit minted its own errors (2026-09-15)

Same branch, after a style review returned 14 findings, three of them
on text the repoint pass itself wrote. Worth recording as a pattern
rather than a list: **a unit whose subject is stale claims is not
immune to making them**, and two of the three were the exact shape it
existed to close — a fresh census written into a row while repointing
it, and a replacement reproduction command that misleads whoever runs
it just as the dead one did.

- `add-profile-placement-on-picked-face-frame` asserted that a
  face-frame arm committing two nodes would break one-submit-one-edit.
  **False, and against a ratified page that names the row by id**:
  `crates/editor-core/REFERENCES.md` DM1 rules that both "on a new XY
  frame" and "on this face" are two inserts in one `commit_action`,
  and `DocSession::commit_action` ships — all-or-nothing, one history
  state, *"one user action is one undo"*. The constraint is struck in
  both that row and `add-profile-mints-no-frame`, which is where the
  claim originated and which the re-cut row had inherited it from.
  Removing it pushes the two rows apart: one wants a node kind the
  chrome can already mint, the other a seat that does not exist.
- `add-profile-mints-no-frame`'s reason for inaction — *"work thrown
  away if that fork goes the derived way"* — was spent: the fork went
  derived in PR 1829. Announced in the row rather than left standing.
- `drag-tick-has-three-homes`: caller list read as a census and named
  three of four; `git grep -c` described as printing a total when it
  prints four `file:count` lines; a per-file figure contradicting the
  row's own subtraction; and "two doc-comment mentions" in `app.rs`
  where there is one `//` line comment and one mention of the MODULE.
  All four corrected, with the derivations shown so the next reader
  can check them rather than trust them.

Two findings landed in territory ceded to VIEW and were filed, not
fixed: evidence added to
`work/view/every-crate-root-reexport-is-a-second-path-not-the-only-one`
(`app.rs`'s re-export comment names a test as its only consumer while
`props.rs` links through the same path in production, at the wrong
home) — which also corrects this program's earlier judgement that the
re-export wrinkle left no open thread; it left two — and a new row,
`forms-rs-fieldwriting-doc-says-the-creation-forms-do-not-use-it`.

That new row makes **three** known instances of "a viewer comment
asserting something the tree falsified", with
`add-profile-ui-doc-comment-states-a-premise-the-tree-falsified` and
the orchestrator's `addboolean-doc-names-a-vocabulary-that-does-not-exist`.
All three were found incidentally by lanes doing something else, which
is what an un-enumerated population looks like. **The class sweep is
still unrun and should be scheduled.**

## Announced seam from WIRE (2026-09-15)

WIRE's `nobodyroots-classification-has-two-homes` gave the
empty-document reading of a gather refusal ONE home:
`ProductErrorKind::means_no_body` in
`crates/editor-core/src/product.rs` (WIRE's), with the argument moved
onto it. A predicate with no caller would be the very defect this
program has an open row for
(`work/wire/frame-linear-generic-door-has-no-consumers.md`), so the
consumers that re-derived the partition now cite it. Four did; the three sites below are yours.

**`crates/viewer/src/frame.rs` and `crates/viewer/src/session.rs`
(CHROME's and VIEW's), two files, three edits.**

- `frame::product_badge`'s filter: the `ProductError::NoBodyRoots`
  alternative leaves the `matches!` and becomes
  `fault.kind().means_no_body() || matches!(…)` over the other
  three. The four declined arms are the same four.
- `product_badge`'s doc, the first "arms that stay silent" paragraph:
  the *"EMPTY, not malformed / a fresh document is in that state / one
  whose last feature was just deleted"* argument becomes a citation of
  `pncad::document::ProductErrorKind::means_no_body`, **worked examples
  included** — the paragraph now says only what is the chrome's: the
  blank viewport is already the picture of this state, so a badge here
  would make an ordinary state look like a failure. The examples are
  MOVED, not copied; leaving them on both sides is the defect this unit
  closes, one size smaller. **The second paragraph is untouched**: the
  three per-node arms are declined because the Features pane already
  badges them with a typed cause, which is not the same reason and is
  not WIRE's to move.
- `DocSession`'s landing: `matches!(fault, ProductError::NoBodyRoots)`
  becomes `fault.kind().means_no_body()`, and the comment above it
  cites the predicate instead of restating *"has no product and no
  failure either"*.

**No signature moved.** `product_badge`, `run_checks`,
`DocSession`'s landing and `checks_report` keep their signatures,
their arms and their behaviour — `means_no_body` is true of exactly
`NoBodyRoots` and of nothing else, which this lane pins as a census
test over `product::tests::every_arm`. The doc/comment edits replace a
re-argument of the shared classification with a citation of it and
leave every site-specific sentence standing (the viewer's three
per-node arms stay the viewer's chrome policy, argued where they are).

Filed while sweeping, on FIX's slate:
`work/fix/subject-refused-accepts-the-one-refusal-that-must-not-go-through-it.md`
— `Subject::refused` is public and takes the one arm that must not
reach `Subject::Unavailable`.

Signed (WIRE implementer lane `wire-n1`, PR #2629).

## Filed from WIRE (2026-09-15): the viewer's empty-document rule, and the badge filter that cannot red

`viewer-states-the-empty-document-rule-in-four-places-and-the-one-that-gates-cannot-red`
lands on CHROME's slate, out of PR 2629's style review.

WIRE gave the empty-document classification one home on
`ProductErrorKind::is_empty_document` and reduced four consumers to
citing it, two of them in `crates/viewer` (the seam was announced here
when it landed). The review then found the viewer still argues the rule
in three places the citation did not reach, and — the part that makes it
a row rather than a tidy-up — that `product_badge`'s remaining
`matches!` over `RootFailed | RootPoisoned | UnknownNode` is **the one
construct in the whole arrangement that cannot red when a tenth
`ProductError` arm lands**. The `editor-core` side now reds twice by name
for a new arm; the viewer side, where a user sees the consequence, reds
not at all.

To be clear about what is NOT being asked: those three arms are chrome
policy and belong in the viewer — the Features pane badges them at the
node with a typed cause, one deliberately quiet — and nothing about that
should move to `editor-core`. The ask is the instrument: an exhaustive
`fn` over `ProductErrorKind` living here, so the local policy is
compile-checked the way the cited rule now is.

Filed on CHROME rather than VIEW because `product_badge` is the gating
site and what the chrome badges is CHROME's charter; `frame.rs` and
`pickindex.rs` are claimed jointly, so re-home if that reading is wrong.

Signed (WIRE orchestrator).

## 2026-09-15 — VIEW corrects the carve-out, and a claim CHROME put in the tracker that nobody made

The carve-out went to VIEW and came back with one correction and one
confirmation, both worth recording because the correction is a failure
this program has its own row about.

**CHROME asserted that a VIEW row claims `edge-cost-claims-name-a-
search-that-is-gone` by id. No VIEW row claims it.** Three cite it —
`renamed-module-leaves-citations-in-two-other-programs`,
`stale-file-citations-after-the-split` and `the-citation-receipts-
summary-numbers-are-not-re-derivable` — and all three cite it as **an
example of a stale citation needing a repoint**, because it points at a
`pick.rs` that no longer exists. *Tracking a row as needing repointing
is not claiming its subject* (VIEW, 2026-09-15).

The audit this came from was accurate: it said a VIEW row "names this
row by id and line", which is true. **The orchestrator turned *names*
into *claims* and shipped it** — into `plan.md`, into `program.md`'s
`keep_out`, and into a merged PR body. Both files are corrected here.

This matters beyond the word. Re-homing on "VIEW already claims it"
would have put a claim in the tracker that nobody made, and a row moved
on a manufactured claim reads to its next owner as settled work. That
is closing a finding by re-description, which is the exact shape
`work/view/a-module-named-for-its-spine-type-is-unfalsifiable` was
opened about. The row still moves to VIEW — the files are VIEW's and
CHROME does not intend to work them — but **the reason is now the true
one**, and the moved row says so in as many words.

The ninth instance of the shape this log keeps recording, and the first
where the false step was a single verb.

**The shift-map rule is now sharper than either program had it**, and
the credit is VIEW's. VIEW had *"a shift map is arithmetic that returns
an answer for every input without asking what it names"*, measured at 2
wrong subjects in 4 handed-over maps. CHROME measured the other half
independently: the closed report's six numbers were wrong by **exactly
+76**, a perfectly uniform delta, and the unit that used that map then
minted four fresh wrong claims, two in the shape it existed to close.

So: **consistency is not corroboration.** A uniform delta is what a
whole-file insertion above the citations produces, and it is equally
what a map naming the wrong subjects produces when they all moved
together. The tail is the part worth keeping: **a unit whose whole
subject is stale claims is the most likely to mint them, not the
least.** In `plan.md` with VIEW's attribution.

**Three rows parked rather than left open.** Dropping `gpu.rs`,
`scene.rs` and `theme.rs` from the working list did not remove the rows
sitting on them, and a row nobody intends to work that reads `open` is
what `parked` exists to prevent. `gpu-index-counts-substitute-u32-max`
and `mispaired-ids-exempts-the-empty-window` park on VIEW's
`scene-mesh-carries-an-identity-index-buffer`;
`chrome-weight-is-outside-the-palette` parks on
`tone-is-a-value-in-frame-and-a-comment-in-two-panes`. Each names a real
item lint can watch close, and each carries a note saying what the
trigger will do to it — VIEW's index row DELETES one of two sites in
the first, which moves that row rather than closing it.

**Confirmed by VIEW, and it settles a home:** `projection_fault` is
real, at three sites in `pane/viewport.rs` as a clear/set pair. That is
the obvious home for `a-datum-the-view-cannot-scale-vanishes-without-a-
word`.

**Sequencing on the two rows the datums lane filed.** Both need
`pane/viewport.rs`; VIEW's viewport-adapter lane is holding that file
now, narrowing statements nowhere near the datum call sites. VIEW's ask
is one branch in that file at a time, and offered either program as the
actor. **CHROME keeps both rows** — they are findings about the datums
refusal seam and the understanding of why the call sites need changing
lives with the unit that made them refuse — and does not touch
`pane/viewport.rs` until VIEW says its lane has merged. Not `parked`:
the trigger is a VIEW branch, and `blocked_on` takes an item or a PR
number, not a promise in a conversation. When VIEW names the row or the
PR, these get parked on it properly.

## `datums.rs`, the substitution sweep (branch `chrome/datums-substitution-sweep`)

The whole file rather than one row. `datums.rs` held **five** members
of the class *a value the function did not compute, returned in the
shape of one it did*, two of them filed:

- `View::metres_per_pixel_at`'s `.max(f64::MIN_POSITIVE)` — now
  `Option<f64>`. The ripple stayed inside the file: all three scale
  doors are private and `grid_pitch` and `datum_view` keep their
  signatures.
- `rule_patch`'s `((last - first) as usize)` under an INCLUSIVE range
  — now an exclusive range over bounds checked finite, with
  `last < first` ruling none. Three distinct zeros the cast merged,
  not the two the row named.
- `half_patch_at`'s `viewport_px.max(1.0)` — floor dropped; a viewport
  that is not a positive number of pixels reaches the product check
  and is refused there.
- `datum_view`'s `height_px.max(1.0)` — floor dropped.
- `datum_view`'s `width_px.max(height)` — unfiled and unnamed by the
  dispatch: `f64::max` answers with the other operand against a NaN,
  so a width that is not a number was reported as the HEIGHT.

Swept and closed by argument, not changed: `unit`'s zero-length
fallback and `basis`'s seed choice (a `UnitVec3` cannot hold a
non-finite direction — `topo::query::UnitVec3Error::NonFiniteLength`
is refused at construction), and `grid_pitch`'s `best = decade` seed
(`decade` is itself a rung of the ladder, so the seed is a member of
the answer set rather than a substitute for one; brute-forced over the
subnormal band and the top of the normal range with no non-positive or
non-finite rung).

Filed on the way: `datum-view-propagates-rather-than-refusing-by-name`
and `a-datum-the-view-cannot-scale-vanishes-without-a-word`, both
needing an edit in `pane/viewport.rs`, which is VIEW's.

### Fix pass on the same branch, after the style review

Nineteen findings; six changed the tree.

- **The flagship row certified a wrong drawing.** A datum at
  `f64::MAX` on the `z = 0` plane, looked at from the origin, drew
  **27 zero-length segments** per plane-like kind: the patch's ends
  `cv ± half` both round to `cv` at that magnitude, so the extent is
  lost and every segment's two endpoints coincide. Finite, in the
  right plane, not lines — and a row asserting only `is_finite` gets
  EASIER as that degrades. `rule_patch` now asks the emitted geometry
  whether it is geometry and commits a direction's ruling whole or not
  at all; the row asserts positive segment length and carries a
  near-datum control so a total refusal cannot satisfy it.
- **`MAX_GRID_LINES`' effective maximum moved from 97 to 96** and
  nothing said so. Now stated on the closing row: the const's doc said
  96 all along and was false by one before this change.
- `PATCH_COVER` gained `patch_cover()`, on `Camera::pitch_limit`'s
  argument — a loose bound in a test is a hand-synced copy with a
  fudge factor.
- `reach`, the instrument three refusal rows measure with, folded with
  `f64::max` and would have reported a drawing containing `NaN` as
  reaching however far its finite positions did.
- `View`'s two field docs now carry the contract the sweep changed,
  instead of a justification sitting a screen away on `datum_view`.
- `unit`'s comment claimed a cross-product bound of `1/√3`; the bound
  on the cross is `√(2/3)` and `1/√3` bounds the COMPONENT. That
  comment is one of the two sites the sweep closed by argument.

Filed rather than fixed: `max-grid-lines-truncates-a-ruling-and-calls-it-one`,
`four-spellings-of-one-finiteness-predicate-in-datums-rs` (the sweep
added the third and fourth), and
`viewer-substituted-value-class-is-crate-wide`, which carries the
population the next sweep should start from — six unslated members in
`sketch.rs`, `scene.rs`, `bounds.rs`, `camera.rs` and `app.rs`, all
VIEW's ground this week.

## 2026-09-15 — the slate lands, and two rows go to VIEW rather than wait on it

Seven PRs merged: the orchestrator re-cut (2640), the memory deletion
(2641), the citation repoint and its own fix pass (2642), the carve-out
corrections (2643), the datums substitution sweep (2644), the Band
refusal's first coverage (2684), and the bounds honesty pass with its
CI fix (2683).

**Two rows and one boundary question moved to VIEW's slate, rather than
being held for VIEW's agreement.** That correction is worth recording
because the instinct behind the delay was wrong in a specific way.

`datum-view-propagates-rather-than-refusing-by-name` and
`a-datum-the-view-cannot-scale-vanishes-without-a-word` both act in
`pane/viewport.rs`, ceded to VIEW. CHROME had been holding them to
`park` them on VIEW's viewport-adapter lane, which needed a trigger id
CHROME could not identify — nineteen open VIEW rows cite that file,
none matches the statements VIEW described narrowing, and none of the
three rows in `review` cites those lines. **The search was the wrong
activity.** `work/README.md` says a finding goes onto the slate of the
program whose ground it lands on, and that a lane does not need the
owner's permission to put it there. Re-homing beats parking: VIEW
sequences its own work and nothing waits on a handshake. The
`datums.rs` halves stay CHROME's and are inside CHROME's fence.

**And unit 5 exposed a real gap in the carve-out**, filed as
`work/view/the-file-level-carve-out-cannot-express-a-row-whose-work-
crosses-a-call-site`. The division is **by file**, and
`body-seat-reads-through-the-placer-chain`'s fix crosses it **through a
call site**: `denotes_body`'s only production caller is
`session/refuse.rs`, VIEW's. A CHROME lane can write the whole fix and
cannot make the crate build. This will recur for any CHROME row whose
fix changes a signature in its own half, so the row asks for a standing
rule rather than a per-instance negotiation. `plan.md`'s unit 5 now
says it is not dispatchable as written.

**Where the program stands.** More rows are open than when the session
began, and that is the honest outcome rather than a failure — the
program's own 2026-09-04 entry already made this argument, and it holds
again: the alternative was not fewer defects but the same defects
unrecorded. Every row filed today carries a `file:line`, a population
where one was measurable, and a stated blind spot where it was not.

**The session's one durable lesson, stated once.** Every unit was
corrected by the layer below it, and five of those corrections changed
the answer rather than a detail: the orchestrator's diagnosis of the
datums defect, its cluster-scoping of the Band fault, its claim that
the README documented POISONED attribution, its count of the shifted
citations, and its "file, don't fix" on ground that was never ceded.
None was reachable by asking whether the code was right. Each was found
by someone opening a file to check whether the **words** were true —
which is what the style-review posture is for, and is now evidenced
rather than asserted. The sixth came from CI, which ran a configuration
the local battery did not and caught an assertion that held at one eps
row only because the floor happens to sit at zero there.

## 2026-09-19 — Ev's arrow rows landed (PR 2856)

Orchestrated from a session taking Ev's high-priority GUI rows across
the paused viewer programs. One unit took both rows. It was not an
A/B-protocol unit, by Ev's instruction. The implementer started on
Fable, hit the account's Fable limit mid-work and was finished on
Opus from the dead lane's uncommitted diff. The review was style-only,
per Ev: small diff, low correctness risk. The fix pass took six of its
nine findings:
- the grid floor is derived from the ladder, not restated as `2.5`
- a runtime sweep of `grid_pitch`
- one home for the pitch band (`MIN_CELL_PX`'s "three to thirteen"
  was wrong; it is about 4.2..10.5)
- a tip-separation check on the doubled head
- the unvaried spacing knob dropped
- a history aside removed

Declined: the integration tests' hand-copied barb counts. The
constants are private, so the copy cannot be avoided.
`TIP_MARK_PX` was left alone and put to Ev in chat.

## 2026-09-21 — seam announced by AUTHOR, and three rows filed

**Seam.** AUTHOR widened its territory to the files its units actually
work: `crates/viewer/src/pane/create.rs`, `pane/properties.rs`,
`drafts.rs`, `session/author.rs`, `session/refuse.rs` — all shared
with CHROME, legitimately (`work/README.md`, 2026-09-20). AUTHOR's
opening `paths` named three files and none of them was where AUTH-1 or
AUTH-2 landed, so `work.py territory` could not have warned either
lane. AUTHOR's `work/author/program.md` records it; no `keep_out` was
written on either side, because nothing about the overlap needs
explaining beyond that.

**Live AUTHOR lanes on this ground as of this note**: `author/face-frame-seat`
(PR 2955, in review) in `pane/create.rs`, `forms.rs`, `drafts.rs`,
`session/author.rs`, `session/refuse.rs`; `author/param-notation` in
`pane/properties.rs`, `props.rs`, `session/op.rs`, `session.rs`.

**Three rows filed on this slate** by AUTH-1's style reviewer, none of
them AUTHOR's to take:

- `a-fifth-spelling-of-this-seat-is-empty` (P1) — `seat_line`'s own
  doc says the sentence is composed centrally *"because two copies is
  how the two drift"*, and there are now five copies. Two rows already
  here are the same shape with different subjects
  (`viewer-states-the-empty-document-rule-in-four-places-…`,
  `four-spellings-of-one-finiteness-predicate-in-datums-rs`); whether
  the three are one unit is CHROME's call.
- `four-pick-state-vocabularies-in-one-create-module` (P1) — the Q8
  end-to-end read of `pane/create.rs` at ~1175 lines.
- `a-per-kind-sentence-lives-in-the-widget-not-on-the-choice` (P4) —
  and AUTH-1 minted the newest instance while closing a different one,
  which is the fresh-instance trap landing where the reviewer brief
  says it lands.

AUTHOR is fixing its own instances of the last one inside PR 2955; the
CLASS is CHROME's and is what these rows carry.
