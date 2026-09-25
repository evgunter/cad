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

## A note from DOOR (2026-09-21) — evidence added to one of your rows

DOOR's `node-placer-field-docs-say-body-where-instances-are-accepted`
(PR 2985) fixed the kernel-side members of the class your
`body-seat-reads-through-the-placer-chain` row names, and appended
evidence there rather than opening a fourth row: **`viewer/src/session/op.rs`
carries VERBATIM copies of the two field docs just corrected** —
`/// The body placed.` and `/// The body replicated.` — and they are not
among the three sentences your row already lists. `session/op.rs` is
also VSEAM's, which has its own prose rows there, so a `git mv` is
reasonable if you would rather they sat elsewhere.

**One correction to your row's own framing, offered because the next
reader will take it literally.** Its opening parenthetical reads
*"ruling 2137 (the placers are shape-preserving over the value:
`Transform` and `Pattern` accept `Instances` and yield `Instances`)"*.
Measured against the code by this unit: `wire_pattern` returns
`ValuePayload::Instances` **unconditionally**, so a one-body master
yields N instances — `Pattern` is not shape-preserving, it is
instances-producing. **Your row's TITLE already says this exactly**
(*"a Transform's value is a body iff its input's is and a Pattern's
never is"*); it is only the parenthetical that generalises. The gate
you are building reads on the title's version.

The orchestrator's dispatch for that unit carried the same imprecise
sentence, taken from the same summary, and the lane caught it by reading
`wire.rs` instead — which is why this note exists rather than a quiet
edit.

Signed (DOOR orchestrator).

## 2026-09-21 — track picked up; the carve-out retired, three units dispatched

Orchestrator session opened on `claude/chrome-orchestrator-setup-24xn6w`
(harness-pinned; unit branches keep the `chrome/` prefix). Program
`ready` → `active`. Ev's instruction at the start of the session:
**no A/B protocol** (the account is out of Fable), and the units
should not be tricky — so this wave is three E units with no
undecided design fork in any of them.

**The 2026-09-15 VIEW carve-out is retired, and that is the session's
largest act.** Every cession clause in it rested on *"VIEW holds an
open row on the same ground"*. VIEW re-scoped on 2026-09-17 — 86 of
its 94 live rows went to VNEWS, VGEOM, VSEAM, VDOC and seven other
programs — and its own `keep_out` now reads *"this program does not
dispatch new units; its remaining act is the exit walk."* Meanwhile
`work/README.md` settled the general question on 2026-09-20 (Ev, in
chat): two open programs may claim one path, and **neither owes the
other a `keep_out`** — what is owed is awareness while a lane is LIVE,
which is a per-branch question `work.py territory` answers.

So CHROME cedes nothing. What replaces the file list is a per-wave
read of who is actually live, carried in each dispatch with a date on
it. A standing clause could not do that job: it would be false within
the day, which is precisely how the 2026-09-15 clause and the one
before it both went false.

**Three rows were held by the cession alone and are now available**:
`gpu-index-counts-substitute-u32-max` and
`mispaired-ids-exempts-the-empty-window` (both ended on *"CHROME does
not work it"*), and `band-refusal-still-badges-every-row`, whose
blocker was *"a new `RowStatus` variant does not land inside CHROME's
fence"*. Each carries a note saying so. The COST arguments inside them
are untouched and were re-affirmed rather than waved through: the
`RowStatus` row's 19-sites-in-9-files measurement is still the reason
to prefer a variant over a field, and is re-taken at dispatch rather
than trusted. For the first two the live question is now *whose slate*
— `scene.rs` and `gpu.rs` are VGEOM's since the re-scope — and that
call belongs to the wave that takes them.

**Dispatched** (specs deleted at merge per `docs/DOC-LEDGER.md`):

1. `chrome/datum-honesty` — `datums.rs`, two rows: the finiteness
   predicate's homes and `MAX_GRID_LINES`' silent truncation.
2. `chrome/empty-document-gate` — `frame.rs` and `pickindex.rs`: the
   empty-document rule cited once, and a gate that can red on a new
   `ProductError` arm. The behavioural half (`session.rs`) is held
   out; see below.
3. `chrome/one-number-one-home` — `bounds.rs`, `app.rs`, `scene.rs`,
   `tests/display_budget.rs`: the panel's divide with one home, and
   three private constants a suite copied.

**Two premises falsified by reading the tree before writing the
specs.** `max-grid-lines-…` asserts `MAX_GRID_LINES` is 96 and builds
its whole reachability argument on that number; it is **512**. And
`four-spellings-…` lists four `is_finite` sites in `datums.rs`; there
are **eight** — the row predicted that growth in the sentence *"the
next lane to touch this file will add a fifth unless there is a door
to route through"*, and it happened. Both are in the dispatches with
an instruction to re-take the census rather than trust the correction
either. This check costs minutes; AUTHOR's log makes the same point
from four falsified premises, and it is now what this program does
before every dispatch too.

**`certify-affordance-on-the-bounds-panel` re-priced E → H.** Its own
body describes a long-running query needing progress, cancel, a
panel-side budget, and eleven unrendered `RangeRefusal` arms, measured
at 3.4–17 s per leaf. Nothing about the finding changed; the board was
offering a design pass as cheap work.

**Held out of this wave, deliberately.**
`at-rest-badge-reports-an-empty-document-as-a-refusal` is unit 2's
behavioural twin and lives in `session.rs`, which AUTHOR's live
`author/profile-frame` (AUTH-3) has in scope this hour — a scheduling
conflict, not a fence, and wave 2's to take.
`a-split-circle-fixture-sits-inside-the-1e-6-escalation-band` is a
live red at `1e-6` on `main` today and is small; it is wave 2's first
row.

**Live seams announced to this wave.** AUTHOR's `author/profile-frame`
holds `pane/create.rs`, `tree.rs`, `blend.rs`, `session.rs`,
`session/op.rs`, `session/author.rs`, `drafts.rs`; VGEOM's PR 3007
(`vgeom/p0-fields`) is ~160 lines inside `pickindex.rs`. Unit 2 is
told to merge `origin/main` before opening and to expect that file to
move under it.

Signed (CHROME orchestrator).

## A note from VIEW (2026-09-21) — two of Ev's reported defects re-homed here

VIEW is winding down and does not dispatch. Two rows on its slate are
CHROME's by your own `keep_out` — *CHROME keeps the viewer's reported
defects and its entrenching architecture* — and both were reported by
Ev in chat on 2026-09-17, from the same failed union of a dumbbell
document. Moved by `git mv` with ids, bodies and history unchanged.
Ev approved the move in chat.

- **`error-and-check-text-overflows-its-region` (P0).** Two halves.
  The layout half — error and check text runs off the page or wraps
  back to the window's left edge — is the sibling of VIEW's
  `the-toolbar-row-does-not-wrap`, which closed 2026-09-14 and whose
  fix may be the same egui cause (text laid out against the full
  available width rather than its container's). The concision half
  reaches past the viewer into kernel `Display` impls in `profile`
  and `editor-core`; the row says whoever takes it splits off
  per-crate rows rather than editing kernel prose from a viewer lane.
  **Left whole rather than split** — the row records that the
  concision half was never investigated, so drawing the boundary now
  would be a guess.

- **`a-refusal-offers-no-action-in-the-viewer` (P1).** A kernel refusal
  is shown as a long paragraph with its recourse buried at the end,
  and none of the three recourses it names is something the viewer
  offers as an action. AUTHOR was considered and rejected: its charter
  is the authoring goal's GUI half (minting a `FaceFrame`, `AddPart`,
  `AddBoolean`), and a refusal's presentation is chrome architecture.
  The kernel gap behind the worked example (torus×plane) is CURVED's
  and already scheduled.

Neither was investigated beyond what its body records. The worked
example for both is in Ev's 2026-09-17 chat; a torus face against a
plane face in a union reproduces it.

Signed (VIEW orchestrator).

## 2026-09-21 — CHROME-DATUM-HONESTY landed (PR 3018)

`datums.rs`: one finiteness door became two, and `MAX_GRID_LINES`
stopped returning a truncated ruling in the shape of a complete one.
Rows closed: `four-spellings-of-one-finiteness-predicate-in-datums-rs`
and `max-grid-lines-truncates-a-ruling-and-calls-it-one`.

**The review blocked this unit, and the reason is the thing to
remember.** The first cut shrank the over-cap patch **centred on the
region** — and the region is not centred on what the camera is aimed
at, because at a grazing seat the near edge is a metre away and the
far edge sits at the `MIN_CELL_PX` cut-off. Measured through
`datums::draws`: past a pane about 6900 px tall the grid was a
complete, closing rectangle floating in front of the reader with
**nothing at the aim point**, monotone in height so it worsened. Worse
than a defect, it was a REGRESSION — the truncation it replaced kept
the span from the near end and did cover the aim. And it defeated the
disposition the row chose: *"a coarse grid might still orient a
reader"* is worthless if the grid is not where the reader is. The unit
built to end complete-looking-and-wrong shipped complete-looking-and-
wrong.

`capped_span` now takes the aim — `grid` already computed the
looked-at point for the pitch — and clamps the survivor inside the
region. All three failing windows now rule the span the truncation
did.

**Three premises of this program's own rows fell to measurement.**
`MAX_GRID_LINES` is 512, not the 96 its row asserts (caught at
dispatch). The `is_finite` census was eight sites, not four — the row
predicted that growth in writing and it happened. And the cap is NOT
dead at every ordinary view: crossed between 6000 and 6200 px, an
ordinary orbit seat on an 8K panel. The row's *"about 26 lines"* came
from the head-on case, where extent scales with eye height rather than
with the cell cut-off. **The sentence that caused all of it is one
line of the const's own rustdoc** — *"Not a budget the design expects
to spend"* — which is why the previous sweep read the site as a
backstop and left it standing. It is deleted, not appended to.

**The lane then failed one of its own claims, unprompted.** Dropping
the region clamp reddened nothing, so *"the patch never leaves the
region"* was a claim at the code with no row behind it — the same
defect the reviewer had just caught on the aim, inside the fix for it.
The new assertion derives its tolerance from the drawing (the pitch
read off the minimum gap between adjacent lines) rather than choosing
one. Five mutations now, one per claim.

**A finding that reaches every lane, routed to Ev.** The lane counted
six `test (…)` jobs and went looking for the narrowing
`docs/prompts/implementer-discipline.md` told it to hunt. There was
none: the interval lane runs from a called workflow
(`ci.yml:3581`), so six of the twelve are named `interval / test
(interval, eps = …, n/2)`. The discipline handed to every lane by path
is wrong about how to count a full run, and it tells readers that six
is the signature of a break. `docs/prompts/` is Ev's, so it went out
as PR 3033 (`[ev]`) with the row on CIW's slate carrying
`needs_ev: true`; the substantive half of the amendment is that
counting job NAMES is the wrong instrument at all, since a workflow
refactor renames every job it moves without touching a step.

**Drive-by, disclosed here because it is not CHROME's ground.**
`work/props/log.md` carried a complete committed conflict block on
`main` — three marker lines around ~146 lines, two orchestrators'
appends, neither side having deleted anything. Repaired by deleting
the three lines and keeping both narratives; noted on PROPS's log and
added as the second instance to
`work/ciw/committed-conflict-markers-reach-main`, whose guard would
have to cover `work/**/log.md` to catch it. Found by running the
tree-wide marker grep against `main` rather than against a lane's own
resolution, which is the gap that row now names.

**Rows filed by this unit**:
`camera-project-answers-with-a-screen-position-for-a-projection-that-
overflowed` (P1 — driven to `Ok(Some([6.502, 1.394, 3.44e-304]))` for
a point 1e300 m away, through a door documented to answer `None`
there, with `pickindex.rs` as the consumer),
`features-share-row-asserts-a-conjunct-its-neighbour-subsumes` (P3),
and `positive-finite-predicate-has-six-homes-outside-datums-rs`
extended to cover both doors and re-swept — that re-sweep found
`camera.rs`'s `finite` and `op_finite`, the typed refusal hand-copied
into two differing only in error type.

Signed (CHROME orchestrator).

## 2026-09-21 — CHROME-EMPTY-DOC landed (PR 3021)

`frame.rs` and `pickindex.rs`: the empty-document rule is cited once,
and the site that gates on it reds when `ProductError` grows an arm.
Row closed:
`viewer-states-the-empty-document-rule-in-four-places-and-the-one-that-gates-cannot-red`.

**Correction to this log.** The entry of 2026-09-15 (and the row, and
the dispatch) named the classification's home
`ProductErrorKind::is_empty_document`. **That function does not
exist** — WIRE renamed it `means_no_body` inside PR 2629's own lane,
so the name was already dead when the row was written citing it. Zero
hits in `crates/` today. The unit corrected the row's body; this
paragraph is the log's correction, since the log is append-only. Five
live documents carried the dead spelling; the two WIRE hits are the
closed row that PROPOSED the name, which even says *"or whatever it is
called"*, so nothing of WIRE's is stranded.

**The design call was a three-valued enum**, not a `bool`:
`badge_site(ProductErrorKind) -> BadgeSite` with `Frame`,
`FeatureTree`, `NotAFault`. The two silences are silent for unrelated
reasons, and under a `bool` the mutation that moves `NoBodyRoots`
between them is **invisible** — both spellings answer `false` and
`product_badge` is `None` either way. Verified by the reviewer, not
argued: it reds only under the enum.

**And the shape mattered more than the enum.** A naive exhaustive
match would have re-named the no-body classes and made the citation
decorative. The `match` answers the three tree-owned classes from
local policy and every remaining class reaches its answer **through a
call to `means_no_body`**, so the cited rule stays load-bearing. The
correctness arm confirmed the ordering moves no badge: `product_badge`
driven over all ten classes on the head and on the merge base gives
**identical tables**, and it is algebraically forced.

**The trap landed here too — four instances, one found by the lane.**
`BadgeSite::NotAFault`'s doc restated `means_no_body`'s own sentence
at a brand-new site with no citation at all, in the diff whose subject
is a rule restated where its citation did not reach. Two more were
restate-then-cite. The lane found the fourth itself while filing the
row about it: a ``[`badge_site`]`` in a `//` comment, the checked
spelling on an unchecked citation.

**CI caught what neither the lane nor the orchestrator had.**
`scripts/gates/viewer-vocab-declared-once.sh` reds on an unratified
`const` array of `Type::Variant` entries under `crates/viewer/src`,
and the lane's ten-class census was exactly that. The gate was right:
`crates/viewer/README.md` ratifies two kinds of hand-written list, a
complete census of another crate's enum is neither, and a third kind
is an amendment to that page — Ev's, not a lane's. The census moved
inline and both halves of the guard were re-proved red afterwards.

**Rows filed**: `has-faults-cannot-red-on-a-new-rowstatus` (P2) and
`a-citation-in-a-line-comment-is-not-checked` (P3). The first is
cross-referenced **both ways** with `band-refusal-still-badges-every-row`,
and the pair is the useful part: that row's fix is to ADD a
`RowStatus` variant, and `tree.rs::has_faults` is a `matches!` over
`RowStatus` that cannot red when one arrives. Neither row could see
that from its own side.

**A correction the lane made to itself, worth more than the row it
came with**: its instrument sweep excluded `has_faults` on the word
*external*, and *external* was doing no work — a `matches!` over a
viewer-owned enum is exactly as silent as one over another crate's.

**And a drive-by whose framing I got wrong.** A tree-wide marker grep
turned up a committed conflict block in `work/props/log.md` on `main`;
it was repaired in PR 3018 (three lines, both narratives kept). But I
appended to `work/ciw/committed-conflict-markers-reach-main` an
argument that the class needs a gate, **having read neither its status
nor its closing section**: the row is CLOSED on Ev's explicit call of
2026-09-04, and its title already says three instances, not the "one"
my note claimed. Retracted in place. LANE-1 had filed the instance
correctly — citing the closed class without reopening it — and that
row is now closed as repaired. The repair was right; the argument
around it was re-litigating a settled decision.
## 2026-09-21 — CHROME-ONE-NUMBER landed (PR 3022); wave 1 complete

`bounds.rs`, `scene.rs`, `app.rs` and `tests/display_budget.rs`: the
panel's divide has one home and three constants a suite copied have
theirs. Rows closed: `bounds-reading-respells-the-panels-one-divide`
and `display-budget-rows-restate-three-private-constants`.

**The lane overturned half my adjudication, with measurement, and it
was right.** The review found that `display_budget.rs`'s rung bound
now read `PROBE_FACTOR` from the code it tests — agree-by-construction
about the probe placement, the mirror of the argument the same lane
used correctly to leave `reads_back_as_a_delta` alone. I recommended
restating `8.0` deliberately. The lane instead put `PROBE_FACTOR`
**back to private** and exposed the derived contract,
`scene::placed_rung_cost() = TRIANGLE_BUDGET / PROBE_FACTOR`. That is
`Camera::pitch_limit`'s precedent exactly — `pitch_limit` exposes
`FRAC_PI_2 - POLE_MARGIN` and hides `POLE_MARGIN`; this exposes the
bound and hides the knob — so the review's objection to widening a
library crate's public API for one in-repo suite disappears rather
than being traded away. And it is not circular: the row compares the
door against a rung's MEASURED count from a real body, where
`reads_back_as_a_delta` would have derived both sides from one
constant. Proved by mutation: dividing `fit_delta`'s placement by 8
reds with *"a single rung tessellated 234516 triangles, past the
125000 a rung is placed at"*.

**It also declined to derive `OVER_BUDGET_DELTA`, and the reason is
better than the fix I asked for.** I read it as a live instance of the
unit's own blind spot — a literal that is an arithmetic consequence of
a constant, three lines from the import that closes it. It is not: it
is a threshold the ROW chooses. Deriving it as `INITIAL_DELTA / 10.0`
would break the row the moment the starting δ coarsened, because a
decade under a coarser start is a δ the same file records as INSIDE
the budget. The doc now says chosen-not-derived, and the class was
swept with seven hits disposed one line each.

**A claimed sweep that could not have produced its own hit list.** The
committed description said every numeric `const` in `src` was matched
against numerically equal literals in `tests` — which would have
returned 55 hits in the file it edited, plus six more elsewhere, not
the one it reported. The real filter (a third stage keying on a
name-match or a comment within seven lines) was in the lane's report
to me and not in the row. §5 is exactly this: a sweep whose blind spot
is unstated is an unverified claim. The row now states all three
stages and names stage 3 as itself a blind spot — a copy with neither
a matching name nor a nearby comment is invisible to it.

**The `ANSWERS` table, 56 restatements of the starting δ in the file
this unit edited, is the argued opposite and is now recorded as one.**
It is a golden, and it is precisely what reddens under the
`INITIAL_DELTA` mutation. A census that leaves the largest population
of its own shape unmentioned reads as a guarantee.

**`MM_PER_METRE`'s doc adjudicated as doc rot, by ruling out the other
reading rather than assuming.** Reviewer-style-lane Q4 asks which of
two a stale sentence is. The lane checked: the sentence is accurate
about the two sites it names, and setting `MM_PER_METRE` to `1.0e6`
reds four rows — so divergence IS caught and this is not a latent
defect wearing a documentation costume. What is missing is a HOME for
the inverse factor, not a guard. The four production sites that spell
it went as evidence to VGEOM's existing row.

**A duplicate row deleted rather than filed.** The camera-readout
finding was already a section on
`work/vgeom/renders-that-multiply-a-finite-guarded-length-spell-the-product-inf`,
attributed to the same AUTH-2 sweep the bounds row cites. §6 says add
evidence to the existing row; the lane's evidence went there and its
own file was removed.

**Rows filed**:
`work/vdoc/the-starting-delta-has-one-home-and-five-prose-spellings`
(P2 — five prose spellings of `0.1 mm` that a mutation leaves silently
wrong while two tests red; it asks for a DECISION, since a doc-comment
number cannot be derived) and
`a-flat-rung-row-uses-an-absolute-epsilon-on-a-scaled-value` (P3 — an
`f64::EPSILON` admitting ~2000 neighbours at magnitude 8e-4).

**Merge note.** This branch conflicted with `main` in `scene.rs` after
PR 3018 landed: `main` had added `use crate::narrowing::Narrow;` while
this branch narrowed the `MM_PER_METRE` doc sentence on the adjacent
line. Resolved as a union — both kept, nothing chosen between — and
the merged tree re-checked with clippy before pushing rather than the
diff alone.

**And this entry's own merge was the defect this session repaired in
another program's log**: two CHROME log entries appended on two
branches, conflicting in `work/chrome/log.md`, resolved the same way
and for the same reason — a log is append-only narrative, neither side
deleted anything, so it is a union. Reordered to chronological, since
PR 3021 merged before PR 3022. Worth one line because the class showed
up twice in one afternoon, in two programs, which is the population
Ev's closed ruling already priced as rare-and-self-limiting: found on
sight, repaired in the same breath, nothing built on it meanwhile.

Wave 1 is complete: three units, five rows closed, seven rows filed
across four programs. What the wave evidences about this program's
review posture is in `work/chrome/plan.md`, stated once.

Signed (CHROME orchestrator).

## 2026-09-21 — withdrawn: the interval job-prefix "finding" was not one

Filed as a CIW row and an `[ev]` PR (3033) earlier today, then
withdrawn on Ev's questioning. Recorded here rather than as a closed
row, because `work/README.md` is explicit that the tracker exists so
work is not forgotten and not so work is recorded — and there is no
work here.

**The claim.** The interval lane now runs from a called workflow
(`ci.yml` -> `interval.yml`), so six of the twelve `test (…)` jobs are
named `interval / test (interval, eps = …, n/2)`.
`docs/prompts/implementer-discipline.md` tells every lane to expect
twelve and says that six-where-twelve-was-expected is the signature of
a break. I argued this sends lanes hunting a non-problem.

**Why it was wrong, in the order the errors were made.**

1. **A lane's PREDICTION was promoted to a finding.** The lane wrote
   *"this will bite other lanes"* and I adopted it without testing it.
   What the lane actually reported was *"I nearly reported a narrowing
   that was not one"* — it caught itself inside the same
   investigation, before reporting anything. The other lane in the
   same wave hit the identical rename and disposed of it in one
   clause. **Two encounters, zero wrong conclusions.**
2. **The "measurement" measured invented failure modes.** Asked to
   justify it, I compared counting methods and reported that an
   anchored match returns six. No agent would write
   `startswith("test (")` against a doc quoting `test (…)`; the
   natural spelling is a substring, which returns **twelve** and
   notices nothing. My own check-runs filter used a substring all day
   and never saw a problem. I constructed the failure and then cited
   it as evidence.
3. **The disqualifying argument was already in hand.** Ev's ruling on
   `work/ciw/committed-conflict-markers-reach-main` (2026-09-04) says
   a defect that is loud on sight, repaired in the same breath, with
   nothing built on it meanwhile, is a poor subject for machinery. I
   quoted that argument in this same session, against my own note on
   that row — and did not apply it here, where it holds *more*
   strongly, because what I was asking for was a change to a file that
   binds every lane plus a ruling from Ev.

**The general shape, which is the only thing worth carrying.** An
orchestrator reading lane reports is reading claims, and a lane's
forecast about OTHER lanes is the least tested thing in its report.
The cost of adopting one is a file, a PR, a reader and a ruling
request; the cost of testing it is asking whether it has ever actually
cost anything. Neither encounter here had.

Signed (CHROME orchestrator).

## 2026-09-22 — handoff: the track picked up, a phantom dispatch corrected, and four stale branches cleared

A successor orchestrator took this track today. Three things the
handoff established, recorded because none of them is visible from the
board.

**No handoff file.** [[orchestrator-switch-runbook]] makes the outgoing
orchestrator finalize `cad-work/handoff-prompt-*.md` with the live
resting state and per-lane resume instructions. There is none on this
host, and `~/.local/share/cad-work/` does not exist. Everything below
was re-derived from the tree rather than read off a handoff.

**The slate's only P0 was reported in flight with nothing behind it.**
`error-and-check-text-overflows-its-region` — Ev's own request, filed
2026-09-17 — was set `status: dispatched` with
`branch: chrome/wrap-in-region` by `5d1f724ca`, which is on `main`.
**That branch has never existed.** It is absent from
`git ls-remote --heads origin`, and `git log --all --grep=wrap-in-region`
returns nothing, so no commit and no merge in this history has ever
named it. The row is returned to `open` here, and dispatched properly
below.

**The general shape, which is the part worth carrying.** `lint` cannot
see this and should not be asked to: `branch:` is a string, the remote
is not in the tree, and a checker that resolved it would red `main` for
every lane between its dispatch and its first push. What the field
actually promises is only that somebody wrote it down. So a `dispatched`
row is a claim by whoever dispatched it, exactly as an `active` program
is — `work/README.md` says that of the program status in as many words
(*"it is the program's own word and lint takes it as given"*) and the
same is true one level down. **A successor re-derives in-flight state
from the remote, never from the header.** One `git ls-remote` per
dispatched row settles it.

**The four `chrome/` branches carrying commits off `main` are stale
tips, not pending work** — checked one at a time rather than inferred,
because a week-stale branch's diff against `main` is dominated by
`main`'s own progress and says nothing about whether the branch landed:

- `chrome/band-refusal-badging` — its coverage work is on `main`
  (`tree_badges.rs` carries
  `a_band_refusal_reaches_the_whole_document_and_blames_no_row` and its
  re-exec'd child), as is the `Measured 2026-09-15` section it wrote on
  the Band row.
- `chrome/bounds-honesty` — every row it worked is closed on `main`
  (`bounds-reading-respells-the-panels-one-divide`,
  `probe-bounds-lacks-driven-slot-guard`,
  `display-budget-rows-restate-three-private-constants`).
- `chrome/frame-arrows` — landed as PR 2856; both of Ev's arrow rows are
  closed on `main` with that number on them.
- `chrome/close-out` — superseded by the carve-out being spent
  (`work/chrome/plan.md`, 2026-09-21), and its three findings all
  reached `main` anyway, one of them re-homed to `work/meta/`.

Nothing is owed to any of them.

Signed (CHROME orchestrator).

## 2026-09-22 — Wave 2 dispatched: three units, four rows

Dispatched against the item files directly, with no `docs/<ID>-SPEC.md`.
These four rows carry their own `## What a taker owes`, and this
program's standing failure is rotted premises — a spec restating a
complete row just gives its premises a second place to rot, and the
lane then has two documents disagreeing about the tree. Where a
dispatch added anything, it added the live-ground map and the
verify-don't-trust instruction, both of which are true only today and
belong in the dispatch rather than in a file.

- **`chrome/wrap-in-region`** — `error-and-check-text-overflows-its-region`
  (P0, Ev's own request, and the row this sitting found phantom-
  dispatched). The LAYOUT half only. The concision half reaches typed
  refusals' `Display` impls in `profile`, `editor-core` and others, so
  the lane files per-crate rows rather than editing kernel prose from a
  viewer lane, which is what the row itself asks for.
- **`chrome/rowstatus-exhaustive`** —
  `has-faults-cannot-red-on-a-new-rowstatus` with
  `band-refusal-still-badges-every-row`, one unit, **in that order**.
  The guard goes exhaustive first so the Band row's new `RowStatus`
  variant cannot land silently; both rows say to take them together and
  neither can see that from its own side.
- **`chrome/split-circle-eps`** —
  `a-split-circle-fixture-sits-inside-the-1e-6-escalation-band`. Live
  red on `main` at `1e-6` today, with a green gate over it.

**Three premises checked against `main` before dispatch, because Wave 1
handed a wrong one to all three of its units.** `tree::has_faults` is
still the two-arm `matches!`; `RowStatus` still has exactly four
variants; and `tree_badges.rs` does carry the Band coverage its row
claims, which the Band row asserts from a branch that never merged — the
content reached `main` by another route, so the claim is true and its
citation is not. Each lane was told the check was mine and to re-take it
rather than trust it.

**What the dispatches carried instead of a fence.** `plan.md`'s
territory section promises a per-wave read of who is live rather than a
file list, and this is the first wave to owe one. Live in open PRs 3052,
2960 and 2961 today: `app.rs`, `session.rs`, `session/*`,
`pane/create.rs`, `pane/properties.rs`, `combine.rs`, `drafts.rs`,
`forms.rs`, `seats.rs`, `tools.rs`; most of `crates/viewer/tests/*` is in
2929. The three lanes were fenced off each other's files too, which the
territory tool cannot say because both sides are CHROME.

**`at-rest-badge-reports-an-empty-document-as-a-refusal` was held back
for the second wave running, for the same reason**: its whole subject is
`session.rs` and two open PRs are in it. Recorded rather than left
implicit, because "held for a live lane" and "forgotten" look identical
on the board a week later. It goes the moment 3052 and 2960 land.

**Not dispatched, and it is the next decision.** This slate measures
61.5 points against a 30-point ceiling with 35 open rows — over budget
by more than double, where `work/README.md` says a program that grows
past its budget splits, along its priority seam. Wave 2 does not touch
that: dispatching four rows moves the load by about eight points. The
split is a sitting's work and it is the one I would spend the next
sitting on.

Signed (CHROME orchestrator).

## 2026-09-22 — the live-ground map I handed Wave 2 was built wrong

PR 3055's style reviewer found that its seam announcement named the
wrong PR. It did, and the error was mine rather than the lane's: the
lane announced the seam the dispatch gave it.

**How.** I built the map with
`git diff --name-only origin/main...origin/<branch>` per open PR. A
three-dot diff is taken from the MERGE BASE, and for a branch a week
stale that base is far behind `main` — so every file `main` itself
changed since then is attributed to the branch. The map named files as
live that were not, and missed at least one that was.

Measured after the fact: **#2929 does not touch
`crates/viewer/tests/tree_badges.rs`** — its diff is 40 files and that
is not among them, and its branch differs there only by carrying
pre-`main` content. **#2934 touches that file AND
`crates/viewer/src/tree.rs`**, and `tree.rs` is what I told the lane was
clear ground and is where its entire change lives.

**What to use instead.** The PR's own file list from the API, which
GitHub computes against the real merge base. `git merge-base` is not a
sufficient substitute: a branch that has merged `main` has SEVERAL merge
bases and `git` picks one, which is why a corrected re-derivation on
this same map still disagreed with the API about #2929. For the question
that actually matters — will these two conflict —
`git merge-tree --write-tree A B` answers it directly and cheaply, and
is what this program should use from here.

**The same root cause reached this sitting twice, from opposite
directions**, which is why it is a log entry and not just a fix. The
other instance is `work.py territory --base main` answering against a
local ref 981 commits stale, filed as
`work/meta/territory-base-main-is-stale-in-every-agent-checkout` — found
independently by two of the three Wave 2 lanes. A stale base ref does
not fail; it answers confidently about the wrong tree. One instance
reached a merged artifact before a reader caught it.

## 2026-09-22 — Wave 2 unit 1 landed: the exhaustive guard, and the Band variant priced

PR 3055. Closes `has-faults-cannot-red-on-a-new-rowstatus`; leaves
`band-refusal-still-badges-every-row` **open** with its cost measured
rather than estimated. Four rows filed.

**What shipped.** `tree::has_faults` is an exhaustive `match`, with
`Unevaluated` and `Ok` each stated as the policy they are rather than
left to the complement of a pattern. One test added — the only place in
the tree where a `Poisoned` row stands alone, so that arm had never been
exercised.

**The unit's real deliverable turned out to be the measurement, not the
guard.** Closing the Band symptom needs a fifth `RowStatus`; the lane
added a scratch one and found it reds in exactly three files, two
mechanical and one not. `frame.rs`'s
`the_tree_still_has_exactly_the_three_states_this_policy_pairs_with`
asserts `left_to_the_tree == states`, and a fifth state breaks
`badge_site`'s **one-for-one pairing** claim by arithmetic rather than
by a missing arm. Whether a run-refusal state pairs with a
`ProductErrorKind` or is deliberately unpaired is a design decision, so
the lane declined it rather than making it in passing. The Band row now
carries that, and the `has_faults` prerequisite is off its bill.

**Three corrections worth keeping, in ascending order of how much they
cost to learn.**

1. **The row and the doc both claimed a role nothing plays.**
   `has_faults` has **no `src/` caller** — 22 call sites, all in
   `tests/` and `examples/r1_e2e.rs`, 15 of them the `!has_faults(…)`
   clean gate. The chrome's actual "is this building" answer is
   `pane::features`'s badge draw and `frame::product_badge`. So the
   defect was never "the GUI would lie to a user"; it was a silently
   wrong **test oracle**, which would have let fifteen gates keep
   passing on documents broken in the new way. Smaller claim, true one.
2. **The measurement the unit shipped was itself incomplete, and the
   gap it disclosed is where the miss was.** `pane/features.rs` has
   TWO `RowStatus` matches: the exhaustive badge draw, and thirteen
   lines below it a `_ => None` wildcard that does not red. The unit
   declared gaps (c) and (d) unsearched on the ground that the
   instrument was a clippy lint and therefore a unit of its own —
   but `grep -rn '^\s*_ =>' crates/viewer/src` is 41 hits and one
   second, and it is what found the wildcard **inside the set the
   unit had just measured**. `work/meta/a-stated-sweep-blind-spot-is-
   never-swept` records two prior lanes where the stated gap was
   exactly where the finding was. This is the third.
3. **The criterion was the defect, twice, and the second time was
   ours.** The originating row says an earlier sweep's criterion said
   *external* enum and the word was doing no work. This unit's own
   criterion said *multi-arm*, and that word was doing no work either:
   `tools.rs`'s `ToolKind::commits` is six arms and `refuse::admits`
   is five, both excluded by a filter that was looking at the wrong
   property. Re-taken, the population is **21 sites in eight modules**,
   not six in four.

**A lane overturned an adjudication of mine with measurement, and was
right — the fourth time in two waves.** The review found the new
fixture violates `Poisoned::through`'s documented invariant by pointing
at an `Ok` row, and I told the fix pass to point it at a `Failed` one.
That is self-defeating: a `Failed` row in the tree makes `has_faults`
answer true through *that* row, so the `Poisoned => false` mutation
stops reddening and the test stops pinning what it exists to pin. The
lane measured it (the mutated policy passes) and used
`Poisoned { message: None }` instead — the one shape the module has for
*"poisoned with no failed cause in this tree"*, which `poisoned_through`'s
absence arm mints and `Poisoned::message`'s doc names in as many words.
Better than my instruction and better than the original: the fixture is
now a state the module renders rather than one it declares broken, and
the hand-written third spelling of `downstream_wording`'s sentence
disappears entirely rather than being replaced by a call.

**Rows filed**:
`a-wildcard-match-decides-viewer-policy-in-five-places` (the 41-hit
sweep, its exclusion criterion, and gap (d) — given its own file rather
than a section, because closing the sweep row would otherwise close it
with the gap unswept),
`two-is-this-broken-readings-argue-opposite-on-poisoned` (sharpened by
the lane from the review's framing: the two docs are reconcilable and
diverge on exactly one state — `Poisoned { message: None }`, which is
the state this unit's fixture now builds),
`downstream-wording-spells-node-where-node-number-forbids-it`, and
`the-exhaustive-on-purpose-argument-is-restated-twenty-times`.

## 2026-09-22 — Wave 2 unit 2 landed: a message wraps inside its region (Ev's P0)

PR 3058, the layout half of
`error-and-check-text-overflows-its-region`. The row stays **open**,
retitled to the concision half it still carries — a title naming both
halves would read on the board as if neither were done, and the board
shows titles.

**One cause, two faces, and it is `egui::Ui::wrap_mode` rather than any
per-label flag.** In a non-wrapping row the mode is `Extend`, which
lays a galley out at infinite width: measured at 318 points past a
220-point region's right edge. In a wrapping row `Label::layout_in_ui`
places the whole galley at `ui.max_rect().left()` and indents only row
zero: measured in the REAL toolbar at 271 points of drift, first line
at x = 279 and second at x = 8, the window's own left edge. That second
number is Ev's sentence read back as a measurement.

**The sibling row that closed in September CAUSED the second symptom.**
`work/view/the-toolbar-row-does-not-wrap` made the toolbar wrap, which
is what put the status line into a wrapping layout. The same class,
fixed one layer too shallow, and the fix created the second face of the
defect it was next to. Worth carrying: a layout fix moves every text in
that layout, and nothing in the process asks what else was in it.

**What the review caught, and it is the most useful thing this unit
produced.** The three tests shipped in the first cut were all one-sided
— `past <= SLACK`, `rows.len() > 1`, `stray <= SLACK` — and every one
gets EASIER as the galley is laid out narrower. The reviewer hardcoded
the wrap width to `150.0` and all three passed. So the unit's central
sentence, *"which region `available_width` names is egui's answer, not
a choice made here"*, was exactly what the suite could not distinguish
from a constant. `docs/prompts/reviewer-style-lane.md` Q3 names this
shape — an assertion monotone in the wrong direction — and it landed on
the one claim the unit existed to make.

The fix pass answered it properly: a row that measures the widest line
at three region widths and requires it to GROW, with a negative control
on a plain label that requires it not to; plus the toolbar measured in
`app.rs`'s real `toolbar_with` harness, which already existed and which
the first cut should have used, since that status line is one of the
two places Ev actually named.

**The fresh-instance trap landed again — in the file the check did not
cover.** `pane::headless` already held two verbatim copies of a shape
walker; the fix added a third and a second driver that strictly
subsumed the existing one. The unit's own check looked only at the
helpers it had deleted in `pane/profile.rs`. Folded to one walker in
the fix pass. **This program has now recorded the trap on every
structural-fix unit it has run, and not once has the lane that wrote
the fix caught its own** — only a reader who did not write it ever has.

**The census was wrong by a factor of four and a half.** Eleven in the
original row, thirteen enumerated, ten more from the reviewer; re-taken
with a WRITTEN test for name-vs-sentence it is **fifty**. And the three
`ui.link` sites the first census dismissed by hand as short names are
all three sentences under that test — they interpolate a user-authored
parameter name, which nothing bounds. Writing the criterion down is
what found them; classifying by hand is what hid them.

**CI reded once, and the guard that caught it was the one this unit
asked for.** The fix pass built the roster guards by reading the
crate's own source, and `test-utils`'s `reader_census` keeps a ledger
of every source-reading site and asserts set EQUALITY with the tree.
Two new readers arrived unlisted and it went red — a census that mints
a census, caught by a census. One ledger line; reproduced red locally
before fixing and green after.

**Rows filed** (six): the four-character-ribbon regime at narrow widths
(a message carrying `0.30000000000000004` breaks inside the number),
the width-versus-characters fork that `number_text` and `message` now
answer differently forty lines apart — with the concision half named as
the same fork's other arm — the auto-sized-window case where
`available_width` is last frame's content, `row_label`'s unbounded
pose, the toolbar's own unconverted sentence, and the text-style
resolution difference.

Signed (CHROME orchestrator).

## 2026-09-22 — Wave 2 unit 3 landed: the split-circle fixture, and a brief that asked for something impossible

PR 3059. Closes `a-split-circle-fixture-sits-inside-the-1e-6-escalation-band`
— a red live on `main` at `1e-6`, under a green gate, because the
viewer's `app`-feature rows run at one eps row of three.

**The dispatch asked for a constant radius and there is none.** I
briefed the lane to find one clearing all three bands. The figure is
conditioned purely relatively — the sagitta and the carrier-identity
residue are both ∝ r and both read against `(ε, K·ε)` — so the
admissible radii are an interval IN ε and both walls are walls in
`r/ε`. ε = 1e-6 wants `r > 2.129 m`; ε = 1e-12 wants `r < 1.467 m`.
The lane established that rather than working around it.

**It is disjoint by 1.45×, and the reviewer made that the finding.**
The window is 5.84 decades at its narrowest against the 6.00 a constant
needs, and the miss hangs entirely on the SOFTER wall — the empirical
f64 residue, not the closed form. Had that residue come out at
3.2e12·ε, a constant would have worked and the brief would have been
right. A comment reading *"the interval is six decades wide and NO
CONSTANT sits in it"* was the narrowest possible miss dressed as a
comfortable result.

**The multiplier moved on a ratified clause, and that is the useful
part.** The first cut took `1.5e9·ε`, the geometric centre of the
window — a numeric criterion with a physical consequence nobody had
priced: a **15 km** circle at the 1e-5 row. `docs/DESIGN.md` D4
(`:609`) ratifies **micron-to-kilometre** coverage. Solving the
envelope over the nine swept eps instead gives `m ∈ [1e7, 1e8]`, and
`1e8` is the round choice: 47× clear of the exact lower wall, ~1.47e4×
of the empirical upper one, 1 km at ε = 1e-5 and 10 µm at 1e-13. The
trade is the right way round — margin spent on the wall that is exact
and ε-independent, to buy it on the wall the whole conclusion rests on.

**Two of my own adjudications overturned with measurement, both
correctly.** (Fifth and sixth in two waves.)

1. I wrote that `r_min ∝ n²` means *"any circle meets it at a count
   four times smaller for each halving of the radius"*. That inverts
   it: `n_max ∝ √r`, so QUARTERING the radius halves the offending
   count, and halving the count buys 4× in radius.
2. I passed on the review's claim that every ε-scaled literal in the
   tree is a small multiple where the multiplier IS the margin, so this
   reading was new. True of the citations, false of the workspace —
   `crates/sweep/tests` spells `1.0e9 * eps` at 12 sites and
   `1.0e12 * eps` at 5. What is actually local is only that here the
   multiplier is a WINDOW PLACEMENT rather than a size or a margin, so
   it reads off nothing; which is why the derivation beside it is long.

**A coverage claim of mine was closed by execution rather than
disclosed.** I had written into CIW's row that the lane *"swept the
whole viewer app-feature population at nine ε values"*. It had not:
`--lib` ran at six extra eps and `--test all` at four, and the two
missing were **1e-8 and 1e-10** — the neighbours of the DEFAULT gated
row — for 82% of the population. An overstatement of coverage on a row
whose subject is invisible coverage gaps. `--test all` is ~95 s, so the
fix pass ran both populations at all nine: 27 runs, all green.

**The standing trap landed on the disclosure this time.** The unit's
pass-2 instrument is "run the population at neighbouring eps, so an
absolutely-scaled fixture near a wall reds there" — valid for the other
833 rows. But an **ε-relative fixture passes at every ε by
construction** and is invisible to that instrument forever, and this
fix introduces the crate's only one. The PR listed three blind spots
and not that one. It is now written into the CIW row — the durable
home, since CHROME's rows die at close — with what does guard it: the
fixture's own `expect`, which reds under controls on both walls.

**Rows filed**:
`the-circle-split-cap-offers-counts-the-document-refuses` (at the
AUTHORABLE cap and the DEFAULT eps, a circle under 2.12 mm split 1024
ways is refused in the shipping app; the boundary is a curve in (r, n)
and the cap, not the geometry, is what keeps the exposure small) and
`the-kernel-takes-any-count-has-four-homes-in-the-viewer`, whose fourth
home is the doc comment eight lines above its own refutation in the
file this unit edited.

Wave 2 is complete: three units, four rows closed, PRs 3055, 3058, 3059.

## 2026-09-22 — the phantom dispatch class, a second time, and I caused this one

This session opened by correcting a `dispatched` row whose branch had
never existed. It nearly closed by reinstating one, from the other
direction, and the mechanism is worth the lines because nothing catches
either.

**What happened.** At dispatch I set
`band-refusal-still-badges-every-row` to `dispatched` on this
orchestrator branch. The unit then declined that half with a
measurement, and its lane correctly set the row back to `open` on the
UNIT branch, which is what merged to `main` in PR 3055. So `main` was
right. But the orchestrator branch still carried my `dispatched` plus
its `branch:` line, and when I merged `main` back in, **git resolved
that file without a conflict and kept my side.** Had I merged this
branch to `main` unread, the board would once again have reported a
live P1 row as in flight with nothing behind it — the exact defect the
first entry above describes, re-created by the orchestrator that
corrected it.

**Why no conflict.** The two sides never edited the same line in the
same direction: `open` → `dispatched` here, and on the unit branch the
lane's edit was relative to a base that already read `dispatched`, so
from git's view only one side changed the line. A clean merge is not
agreement; it is the absence of a textual collision, and a stale state
marker is exactly the shape that slips through one.

**The rule this adds to the first entry's.** That one said a successor
re-derives in-flight state from the remote rather than from the header.
This adds: **an orchestrator branch that outlives a wave accumulates
state the units have already superseded**, and merging `main` into it
does not clear that — it preserves it. Before any orchestrator branch
merges, diff its `work/<program>/` headers against `main`'s and take
`main`'s on every row a unit has touched. The units are the authority
on their own rows; the orchestrator branch is only the authority on the
ones no unit took.

Caught here by running `work.py status` after the merge and reading the
`dispatched` column — two rows, where only one unit was still in
flight. That count is the cheapest check there is, and it is the one
that found it.

Signed (CHROME orchestrator).

## 2026-09-22 — the track picked up again, and cut along its priority seam

A new orchestrator sitting (Ev, at hand-over: *"no AB protocol"* — the
posture this program has always had). Nothing was in flight: the
board's `dispatched` column was empty and no `chrome/` branch is open,
which is the check the previous entry says to make, made.

**The slate measured 88 points against the 30-point ceiling**, and the
previous sitting named the split as this one's work. Seven rows filed
by Wave 2's wrap unit carried no band and no cost, so they were charged
the 2.5-point default; they were priced first, because where they land
decides the seam. All seven are residue of Ev's P0 overflow report:
the fifty unconverted sentences and the toolbar's canceled line are
the defect Ev reported, still live, so **P0**; the width-versus-
characters fork, the ribbon floor, the auto-sized window and the
unbounded row label are **P1**; the text-style divergence is latent
and **P3**.

**P0 and P1 alone then came to 55**, so a pure priority cut would have
left CHROME over budget on its own spine, and the seam had to run
inside P1. It ran where the slate already divided:

- **CHROME keeps 28 points** — the text the viewer lays out and the
  badges it draws, and the subset-policy class behind the badges
  (`work/chrome/plan.md`, *The slate after the 2026-09-22 cut*).
- **FORMS** (new, 23 points, band 10000-10099) — the creation forms'
  vocabulary and the seats they gate: eleven rows.
- **OFFER** (new, 18.5 points, band 10100-10199) — what the viewer
  offers a person to do about what it shows, and what it asks of the
  platform: seven rows.
- **By the re-homing rule**, to live programs whose charter names
  them: VGEOM four (the substituted-value class, `Camera::project`'s
  overflow, the finiteness predicate's homes, `metres_per_pixel`'s
  hand-rolled norm), FIT two (the flat-rung row's absolute epsilon, and
  `mispaired-ids-exempts-the-empty-window`), VACUITY two, HELPER one,
  BLIND two. Every receiving program's log carries a note.

**One call was corrected against the receiver's own words.** The first
draft sent seven rows to VGEOM. VGEOM's log had already declined
`mispaired-ids-exempts-the-empty-window` on 2026-09-21 on a stated
charter test — *a row belongs here only if a wrong number, or no
number, reaches the screen* — and two more of the seven failed the same
test (a missing pixel test, a missing adapter preference). They went to
FIT, BLIND and OFFER instead. A receiving program's charter test is the
thing to read before a `git mv`, not after.

**Twenty-six files cited the moved rows by path**, six of them code
comments (`crates/viewer` and two in `crates/editor-core`, EDIT's —
comment-only repoints, announced in the PR). All repointed; the
append-only logs were left as written.

**The one live edit on a moved row**: AUTH-4 (#3052) appends evidence
to `body-seat-reads-through-the-placer-chain` at its old path. Rename
detection should carry it; AUTHOR's log says what to do if it does
not.

Signed (CHROME orchestrator).

## 2026-09-22 — Wave 3 landed: three units, eight rows closed, one P0 reopened honestly

Dispatched after the cut, against the item files, with the live-ground
map taken from each open PR's own GitHub file list (the rule the Wave 2
entry above wrote down). No A/B duals, no row in `docs/MODEL-AB-LOG.md`.
Each unit got a style review; the badge unit also got the correctness
arm, because its failure mode is the tree pointing at the wrong row.
**All three reviews returned MERGE-AFTER-FIXES, all three fix passes
landed green, and every review found something real.**

- **`chrome/message-floor`, PR 3089.** Ev's floor-then-scroll ruling.
  A message lays out at `max(available_width, message_floor)`, where
  the floor is `widgets::widest_number` plus one space advance, and the
  number alphabet now has one home (`readout::GLYPHS`,
  `readout::widest_render`), which `pane/view.rs`'s δ field reads too.
  Also landed: the toolbar's P0 canceled line, and `message_indent`,
  which is the ruling's GUI half. Closed five rows.
  **The review's MAJOR was a false headline claim.** "No break ever
  falls inside a word" was false for any word containing `.` or `-`.
  epaint tests for overflow before it records a break candidate, so
  the overflow fires on the trailing space and falls back to
  dash/punctuation inside the word. The test's all-digit fixture only
  ever reached the `any` fallback. The fix dropped the lane's
  widest-word term, which had gone past the ruling and would have
  widened every line to one long path. The claim narrowed to what the
  ruling protects: numbers.
- **`chrome/badge-attribution`, PR 3090.** `downstream_wording`
  composes `node_number`. `bounds::Verdict` names `tree::has_faults`.
  `feature_row`'s link decision is exhaustive. `tree.rs`'s header leads
  with the fact that settles blame: mate-only faults reach one row.
  Two rows closed. Two rows stay open with questions for Ev:
  `band-refusal-still-badges-every-row` (Q1) and
  `blamed-mates-sends-the-eye-past-the-node-the-fault-says-to-fix` (Q2).
  The reviewer corrected Q1's framing: a loud document-level at-rest
  badge already exists, and it lacks only the band's words.
  Filed `work/msolve/placer-refused-names-the-pattern-for-a-part-index-that-does-not-evaluate`.
- **`chrome/concision`, PR 3088.** Ev's ruling: rewrite the kernel
  prose at the source. The worked example went from 277 words to 65.
  The whole `BooleanError` Display and `PointInSolidError` are in one
  voice, with no stage prefixes and no key dumps outside kernel-bug
  arms. A rendered-text budget test covers 30 arms at 75 words
  (`editor-core/tests/refusal_concision.rs`); 21 of the 30 go red on
  `main`'s old text. Sixteen per-program rows were filed, each pointing
  at the one standard on this program's row.
  **The review's MAJOR was a new false statement.**
  `GermFrameCylinderPinch` asserted a pinch at a site that never
  compares radii. The old text had been right. A rewrite for concision
  is a rewrite of claims, and it needs the correctness arm.

**The P0 row is reopened, and that is the honest state.** The lane
first closed it on a threshold of 50 literal words. The review showed
the census could not see rendered text: nested payloads and forwarded
refusals. The fix pass measured rendered text for the chains it
touched and reopened the row for the ones it could not:
`NodeErrorKind`'s other kernel arms, `EditError`, and the checks
window.

**Three of Ev's questions are open in chat**, and each is written on
its row: the long pose in a tree row (a/b/c, on
`feature-tree-row-labels-draw-an-unbounded-pose-in-an-extend-row`),
Band badging (Q1), and placer blame (Q2).

**A process note: the disk.** Three builders and three reviewers with
private targets exhausted the 28 GB allowance. One reviewer ran no
tests at all because of it. The orchestrator now reclaims a target
the moment its lane's report is final, per `agent-lane-operations`.
That should have been done from the first report.

Signed (CHROME orchestrator).

## 2026-09-23 — Ev's three answers, and Wave 4 landed: four PRs, and a P0 row that stays open on purpose

**Ev answered the three questions of Wave 3 in chat** (PR 3096). The long
pose keeps scrolling, so that row is closed. `MateFault::Band` is left
alone: it is reachable only at an ε within a factor K of `f64::MAX`, and
the row is deferred on that ratification. Placer blame took option (c):
blame stays on the mate row, which now links to the placer.

**Wave 4 was three units, and all three landed after review:**

- **`chrome/placer-link`, PR 3100.** `TreeRow.repair_at` and
  `tree::repaired_at`, with one arm per fault; only `PlacerRefused`
  links. The review found a real wrinkle, now disclosed. For a Part index
  that does not evaluate, the kernel names the PATTERN, so the new link
  lands on an `Ok` row. The kernel row
  `work/msolve/placer-refused-names-the-pattern-for-a-part-index-that-does-not-evaluate`
  now records that the link raises that defect's cost. The fix pass also
  routed a fresh copy of the headless click harness back through
  `pane::headless`.
- **`chrome/properties-messages`, PR 3101.** Twenty `properties.rs`
  sentences, plus the view and profile panes. Per the second half of
  Ev's floor ruling, every sentence that sat beside a control in a
  non-wrapping row got its own line under it (`exists_notice`,
  `slot_notes`, `bounds_notes`). The P0 row
  `messages-in-the-creation-and-properties-panes-still-draw-past-their-row`
  stays OPEN for `create.rs`, which is still live in AUTH-4 (#3052).
- **`chrome/concision-chains`, PR 3108.** Chains 1 to 3 were rendered
  and rewritten at the source. They now measure: feature tree 360 rows,
  longest 74 words; edit refusals 99 rows, longest 72; checks window 23
  rows, longest 71; blend details 362 rows, longest 74.
  `crates/test-utils/src/refusal.rs` holds the rendered-text guard:
  budget, stage-prefix SHAPE, Debug structs, arena keys, and one recourse
  per message. `geom_core` now holds the shared recourse constants.

**The review of 3108 found three false statements, and a second review
of its fix pass found no MAJOR but nine MINORs.** That is the third time
in two waves that a concision rewrite changed a claim:

1. the split said a plane "crosses" a face at a gate that refuses any
   curved face anywhere in the body;
2. the fillet contact recourse inverted a branch;
3. the fillet assembly recourse dropped "trivalent" and so endorsed a
   corner the kernel refuses.

Each was caught only by a correctness reader checking the sentence
against its raise site. **A concision unit in this repo gets the
correctness arm, and a fix pass that rewrites more prose gets a delta
read before merge.** The delta read earned its keep: "declare" had been
dropped from `CoincidentSurfaces` on a comment that was itself false.

**The P0 concision row stays OPEN, and that is the honest state.** The
remainder is prefixes and keys in `topo/src/props.rs` and
`geom-brep/src/certify.rs`. No program owns them, and #2861 and #3049
are reworking them. The row names its closing check: remove the
`FILED` / `KERNEL_KEYED` entries, and the guard goes red until the
residue is gone. The guard's own blind spots are filed as
`the-refusal-shape-guard-has-blind-spots` (P3).

**Disk, again.** Three builders and a reviewer took the 28 GB allowance
to 1.3 GB free mid-wave. The fix was to reclaim a target the moment its
lane is only polling CI, and to size reviews to run one after another
in a single clone. Both are now how this program runs.

Signed (CHROME orchestrator).

## Seam note from AUTHOR (AUTH-4 fix pass, PR 3052, 2026-09-24)

**What a seated tool takes from a viewport pick changed, on this
program's ground** (`crates/viewer/src/tools.rs`,
`crates/viewer/src/session/select.rs`).

`tools::on_node_pick` — the one route every seated tool's pick takes —
now reads a new accessor, `Selection::seat_node`: the tree's node for a
tree click, and for a viewport pick the node whose DRAWN body the ray
met (`FaceSelection::node` / `EdgeSelection::node`). It used to read
`Selection::node`, which answers the feature that MINTED the face.
**`Selection::node` is unchanged** and still serves the feature tree's
highlight, the property panel's rows and the extrude form — it is the
right answer there.

Tools whose behaviour changes, all for the better and all held by
`combine_ops::a_viewport_pick_seats_the_drawn_body_in_every_body_seat`:
the **boolean, split, transform and pattern** tools (a face on a moved
copy or a filleted body now seats that body, not the upstream extrude),
and the two new ones, **projection** and **duplicate**. Unchanged: the
**revolve** tool (its seats are a profile and an in-sketch axis, which
no ray meets), and the **mate** and **blend** tools, which never took
this route — they read the face and the edge whole.
