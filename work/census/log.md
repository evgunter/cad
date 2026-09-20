# CENSUS log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/census/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3). Seven rows moved in by
`git mv`, each with a `## Re-homed` record: two from `work/issues/`,
five from `work/code-quality/`.

The class was legible in the pile — several rows literally say "spelled
N times" in their titles — and had no owner because it belongs to no
crate. It is the smallest track the cut opened, and deliberately: the
rows that share its shape but whose fix is already written went to DOOR,
and the ones whose population is every file in the workspace went to
COMB.

## A row arrives from M10 (2026-09-13)

M10 closed at its exit sweep (`docs/DOC-LEDGER.md` sweep 13) and
`pncad-py-eval-err-variants-outside-the-tag-inventory` came here by
header edit and `git mv` (id unchanged). It is this program's class
stated in one file: `TAG_INVENTORY` lexes `crates/pncad-py/src/tags.rs`
and nothing else, while eight `eval_err` call sites under
`src/py/` mint four refusal words as string literals that reach Python
exactly as a `tags.rs` word does — three covered by accident and
`measure_unavailable` pinned nowhere. `crates/pncad-py/*` is LIB's
territory and this program's `keep_out` already announces its
pncad-py rows there.

## A second row arrives, from DOCM (2026-09-13)

`a-document-vocabulary-declared-outside-the-macro-is-uncensused` came
here at DOCM's exit sweep (`docs/DOC-LEDGER.md` sweep 14) by header edit
and `git mv`, id unchanged. It was not logged at the time and the slate
in `plan.md` did not carry it; both are corrected here rather than
left for a later reader to notice the board and the plan disagree.

Its live instance is already resolved — WIRE's review of PR 2501 found
`LoopProgram` four lines below the macro invocation and moved it inside,
and `ProgramRefusal` / `RecordedProgramError` are dispositioned out at
the site. **What this program inherits is the general case**: nothing
detects the next plain `pub enum` that should have been a vocabulary.
Every door the row names is a walk over source text, and PR 2501 removed
exactly such a walk after measuring it report agreement over a set
missing the variant it existed to catch. That is the charter's trap in
its purest form and the row is ordered fourth with it stated.

## Posture settled (2026-09-15)

Ev, in chat, on the session opening this program's dispatching:

- **No A/B protocol.** The band stays claimed for bookkeeping; nothing
  draws an ordinal. `docs/MODEL-AB-LOG.md` already recorded this program
  as style-reviews-with-a-correctness-arm, so no amendment is owed there.
- **A full correctness review is for the hardest units only** — the two
  **H** rows on today's slate.
- **The scan-set rule moves into the style brief.** `plan.md`'s posture
  section previously bought a correctness arm mechanically for any unit
  changing what an instrument SCANS. Asked which test to run where the
  two diverge, Ev's answer was to take the hardest-units test for the
  full review and "get some of the benefit" of the other "by telling the
  style reviewer to also watch out for specifically silent omission
  errors". So the obligation survives as a named question in the brief
  rather than as a second lane: *what does this instrument no longer
  read after this diff, and what would it report if the population it
  watches went missing entirely?* `plan.md` §Review posture now states
  it and lists the units that carry it.

## Tracker sync (2026-09-15)

`plan.md`'s slate carried the seven rows of the 2026-09-11 cut and
neither row that arrived after it. Both are now in the slate table with
a class estimate (`pncad-py-eval-err-…` **E**, `a-document-vocabulary-…`
**M**) and both are placed in the order, third and fourth.

One obligation is recorded here rather than discharged: **S57's guard
half has no file on GUARD's slate.** `plan.md` says the anti-re-fork
guard in `scripts/gates/*` is filed on GUARD and never landed from here,
and `work/guard/` carries no such row. Filing it now would guess at the
guard's shape, which the call-site fix decides; it is filed at the
moment S57 is specced, and this line is the record that it is owed in
the meantime.

## CENSUS-INERT-DENY (2026-09-15) — the first unit, and the trap sprung once

`inert-deny-unknown-fields-on-unit-enums` landed on PR 2634. 59 attribute
sites dispositioned (22 removed, 37 untouched, **0 kept**), 16 prose sites
(5 rewritten, 11 left), and an instrument —
`crates/test-utils/tests/deny_unknown_fields_census.rs`, reading Rust
through `test_utils::source` with its line in the reader census.

**The rule the row stated was wrong and the sweep is what found it.**
`deny_unknown_fields` needs a NAMED field, so it is inert on a unit enum,
a tuple-variant-only enum and a tuple struct alike — not the row's
"unit-vs-struct". Established by execution both ways: removing all 22
left `editor-core` bit-identical; removing one governing attribute
reddened three tests.

**The program's trap sprang on its own first unit, in the three lines the
PR body cited as proof it had not.** `word()` called the shared
`boundary_before` for the leading edge and hand-spelled the identical
predicate for the trailing one, while `source.rs` says at that very site
that a guard spelling its own version of a shared operation is the defect
the module argues against. The style lane caught it; the repair added
`boundary_after` to the ratified home. **This is the record that naming
the trap in a PR body does not prevent it** — the body named it.

The style lane also refuted the guard's own blind-spot list, which claimed
"the one false-green shape". Four executed counterexamples, one of them
the reviewer's untested hypothesis, and the fix pass found a fifth that
mattered most: **raw identifiers**. `r#type { w: f64 }` read as having no
struct variant, so the census would have advised removing an attribute
that governs. Every other false green invented a named field; that one
hid a real one.

The sharpest finding was in the prose half, which is what the row was
always about: the rewrite of `persist/mod.rs` **hardened a false sentence**
— it made the precondition the named field when it is the attribute — and
a sibling doc 240 lines down asserted the unqualified version and had been
dispositioned "true and load-bearing". One sweep, two dispositions of one
sentence. Both now say the same true thing, with the format's unenforced
intent kept rather than deleted (Q4's second sub-case).

### Decisions taken here

- **`ATTRIBUTE_SITES_TODAY` is a sorted `(path, count)` tally, not a
  scalar.** The fix pass took the middle offered and proved it: a
  compensating blindness (five sites dark in `names/`, five planted
  elsewhere) is GREEN on a scalar and reds on the tally, naming the file
  that went dark. Reconciled at the site with `aggregator_headers.rs`'s
  ruling that a hand-written count is a second unchecked copy — that set
  has an owner to read instead; this population has none.
- **`persist/mod.rs` cites both tracker rows**, the msolve instance and
  the CENSUS class, with the note that a path deleted at its program's
  close resolves through `docs/DOC-LEDGER.md`. The fix pass raised the
  rot; the ledger already answers it, and the class row is what a reader
  of that header wants next.
- **The new H row is placed with the H group and coupled to none of it.**
  The argument for pulling it forward (its instrument is built and warm,
  and the longer it waits the more a one-directional census reads as the
  finished answer) is recorded in `plan.md` rather than taken.

### Open with Ev

`plan.md` §Review posture said the scan-set obligation was "carried by the
style brief instead". It was not: `docs/prompts/reviewer-style-lane.md`
has no such clause, and the obligation reached the reviewer only through
the dispatch prose. **The plan asserted a durable home that did not
exist — this program's own defect class, in its own plan**, and the style
lane caught it by checking the claim against the tree. The plan now says
what is true. Whether the clause should become standing in
`docs/prompts/` is Ev's, and is asked separately; nothing under
`docs/prompts/` was touched.

### Filed

`census-sees-an-inert-attribute-but-not-a-missing-one` (H, this slate) —
the census is one-directional: nothing reds when a named field arrives
with NO attribute. `work/msolve/mate-primitive-accepts-a-stray-field-the-module-docs-say-refuses.md`
is the one confirmed instance, filed by the lane and left unfixed because
closing it changes what a document accepts.

## CENSUS-INERT-DENY merged (2026-09-15)

PR 2634 merged as `d345325b`, green on run `34952933839`. The spec is
deleted per `docs/DOC-LEDGER.md`'s per-merge convention, recoverable at
`git show 6cf25b356:docs/CENSUS-INERT-DENY-SPEC.md`; the item is closed
and carries the three premise corrections that outlive it.

**One thing the merge cost, recorded because it was not this unit's.**
The first attempt red'd on `gate ok` — the single required check — over
a run whose other 38 jobs were green. `k-lint (gate, release-default)`
concluded two seconds before the gate started, and the jobs API served a
snapshot eleven seconds stale. That is the documented false-red half of
`work/ciw/gate-ok-has-no-expected-job-roster.md`, second instance in
three days on the same matrix row; this run's timings are appended there,
with the observation that **the gate's own failure text misnames the
cause** — it says to add the job to `needs:`, and `k-lint` is already in
`needs:`. Not fixed from here: `ci.yml` and `scripts/check-run-jobs.py`
are CIW's, and the fix is the design call their row is holding open.
`rerun-failed-jobs` returned 403, so the lane could not clear it; the
evidence commit re-triggered CI, which passed.

## Next

`hand-listed-debug-censuses-in-geom-core-geom-and-topo` is second in the
order and is not yet specced. The pattern is proven by PR 2093 and the
row carries a nine-impl hit list; what needs deciding at spec time is the
`PartialEq` half, which the row names and leaves unswept — an `Eq` that
misses a field answers wrong, where a `Debug` that misses one only
misleads.

**What this unit teaches the next one**, beyond the pattern: the guard's
population here was not compiler-known, which is what earned its tally a
place. `Debug` and `PartialEq` impls ARE compiler-known — an exhaustive
destructure makes a new field an E0027 — so the instrument that unit
leaves should be the compiler wherever it can be, and a census only where
it cannot. Reaching for this unit's shape there would be the charter's
trap in its other direction: a hand-maintained reader standing in for a
check the language already performs.

## CENSUS-DEBUG (2026-09-15) — the trap sprang inside the fix, again

`hand-listed-debug-censuses-in-geom-core-geom-and-topo` on PR 2655.
12 `Debug` impls destructured (4 with `_`-bound fields and the
terminator corrected to `finish_non_exhaustive()`), 8 `PartialEq`
siblings likewise, and an arrival census in `crates/test-utils/`.

**The row's own hit list had decayed.** Written 2026-09-06 saying eight
rows and nine impls; its own enumeration rule gave 14 on 2026-09-15,
four of them new members of the class. Nothing observed four arrivals in
nine days — the row's thesis arriving as evidence about the row, and the
argument for the instrument half.

**The trap sprang for the second unit running, and the code argued
against itself.** The census's suppression list was keyed `(path, trait)`,
and the doc three lines above it explained why: *"a suppression that
grows silently is the shape this census exists to refuse."* The key did
not implement that — it narrowed the growth from per-file to
per-(file, trait) and left the direction open. The style lane executed it:
a fresh hand-listed impl appended to an already-suppressed file left all
five rows green. **That is the one event the census exists to detect.**
The fix pass re-ran the probe against the OLD key to confirm the finding
independently before changing anything, then keyed on the self type.

Two units, two springs, both caught only by a reader who did not write
the fix. This is now a pattern of the program rather than one lane's
slip, and the next spec should say so.

**The design claim was half wrong, which is the more useful correction.**
The unit was dispatched on "the per-field question is compiler-known
after Half A, so the census must not re-ask it". True for the twelve
impls Half A touched; **false for the population the census then declares
clean.** Four body shapes answered green, and one was de-listed at the
site as something the classifier answers: `self.0.name`, a newtype
reading a NAMED field of its inner type. Live instance `NameRef::eq`,
which drops `Held::stamp`. Three of the four are now closed in the
classifier, that one is on the blind-spot list where it belongs, and the
header no longer asserts E0027 holds an enum struct-variant arm
unconditionally — `..` defeats it there exactly as in `Self { a, .. }`.

**Second blind-spot list in two units to claim exclusivity and be
short.** Unit 1's was wrong by a factor of five; this one by four.

### Decisions taken here

- **The self-type key**, with its cost stated at the site: the type is
  read as written, so a rename makes an entry stale and reds the sight
  row. Loud direction, and it buys that a homogeneous trait has at most
  one impl per type, so a key names exactly one impl and cannot cover a
  second.
- **`GeometryWitness` downgraded from a live wrong answer to a missing
  tie.** The style lane found the row contradicting itself in the
  paragraph carrying its severity; `eq` compares `a_point`/`b_point`
  coordinate by coordinate, so the stated scenario already compares
  unequal. The surviving scenario is narrower and arguably correct
  today; what is wrong is that nothing holds it. The row went to
  another program's slate and had to be accurate first.
- **The new out-of-fence row placed fifth, early for its class**, because
  most of it is routing and routing decays — it records two sites
  claimed by no open program, and every program that closes moves an
  owner.

### Two things the fix pass found that nobody asked for

`SKIPPED_DIRS`' component test was on the ABSOLUTE path, so a checkout
under a hidden ancestor would have skipped the whole tree and left every
row passing over nothing — a silent vacuity in the guard itself.
And `repo_root`'s eight lines had **five** copies, not the three the
review found; all five are now `test_utils::source`.

## CENSUS-DEBUG merged (2026-09-15)

PR 2655 merged as `265a4e8b`, green on run `34965144463`. The spec is
deleted per the ledger, recoverable at
`git show 67e56e2cf:docs/CENSUS-DEBUG-SPEC.md`; the item is closed and
carries its premise corrections.

## Two units in, and what the program has learned about itself

Both units are closed and the pattern across them is worth stating
before a third is specced, because it is about THIS program and not
about either row:

1. **The trap sprang on both units, and naming it did not prevent
   either.** Unit 1 hand-spelled a shared predicate one line after
   calling its other half. Unit 2 keyed its suppression list so that a
   suppression could grow silently, three lines under a doc explaining
   why it must not. Both were caught by a reader who did not write the
   fix, and by nothing else. **A spec for a CENSUS unit should stop
   treating this as a warning and start treating it as a prediction**:
   name the specific growth direction the unit's own instrument will
   have, and require the lane to execute a probe against it.

2. **Both blind-spot lists claimed exclusivity and were short** — unit
   1's by a factor of five, unit 2's by four. A list that says "the one
   shape" or "what is NOT on this list, because the classifier answers
   it" has been wrong every time it has been written here. The next
   spec should require the list to be arrived at by execution rather
   than by reasoning, and should treat an exclusivity claim as a
   finding in review.

3. **The orchestrator's spec premises decay too.** Unit 1's spec was
   wrong in three places, unit 2's in three more, and one of unit 2's
   was a CRITERION rather than a count — the sharper failure, because a
   wrong criterion silently re-scopes the whole unit. Measuring before
   writing caught a great deal; it did not catch everything, and the
   standing instruction to the lane ("the spec's numbers are a
   hypothesis; correct me") earned its place both times.

4. **What the instrument should be is a per-unit question, not a
   house style.** Unit 1's population was not compiler-known and earned
   a reader with a pinned tally; unit 2's split, and copying unit 1
   wholesale would have re-asked a question `E0027` already answers.
   The third unit should make that judgement explicitly rather than
   inheriting it.

## Next

`pncad-py-eval-err-variants-outside-the-tag-inventory` is third in the
order: the smallest row on the slate, one crate, and its one call —
widen the reader to lex the literal-variant `eval_err` sites, or rule a
call-site literal deliberately out of scope and pin the one uncovered
word — is the orchestrator's to make in the spec rather than the lane's,
because it is a question about the gate's REACH and answering it is the
unit.

## CENSUS-TAG-REACH merged (2026-09-15)

PR 2660 merged as `6b432a15`, green on run `34976120254`. The spec is
deleted per the ledger; the item is closed carrying its corrections.

**The probe change worked, and it cost the spec's disposition.** The
spec ruled each word become a `pub const` and required the lane to test
by execution what would stop the next one. Against that disposition
**nothing red**: a fresh site minting a word that had never existed
passed 85 Rust and 832 Python tests. The word became a type.

**And the trap sprang a THIRD consecutive time.** The type guarded
`eval_err` — which is not the door; `typed_err` is. Two raise sites
reached the map by convention, and **the same commit converted them to
hand-call it**, re-minting the defect while closing another instance of
it. The false claim shipped in doc comments, not PR prose, and the gap
was visible inside one sentence: *"`eval_err` takes `EvalReason` … so no
raise site can spell a word of its own"* — premise about one function,
conclusion about all of them, argued in eight places.

Closed by moving the wall to the door: `ErrorClass::Evaluation` now
CARRIES the `EvalReason`, so a raise site cannot name the class without
naming a variant. Probe executed in a file that did not exist: `E0308`.

**What this unit adds to the program's standing findings** (the four
points recorded at CENSUS-DEBUG's close still hold; these sharpen them):

5. **Point 1 needs its scope widened.** Naming the trap as a prediction
   and requiring a probe DID work — the lane caught an instance unaided
   for the first time, and its probe defeated the orchestrator's own
   disposition. What it did not catch is the trap one level out: the
   probe tested the function the spec named, and the defect was that the
   spec named the wrong unit of guarding. **A probe inherits the spec's
   fence.** The next spec should require the lane to say what the
   probe's own blind spot is before running it.

6. **A blind-spot list has now been short three times for three
   different reasons** — unit 1's by shape, unit 2's by shape, unit 3's
   because the sweep's PATTERN (a literal beside a key) could not see a
   getter, a tuple position or an argument. The lesson is not "look
   harder"; it is that a sweep shaped like the defect you already found
   finds that defect again.

7. **Filed rows overclaimed on all three units.** They go to other
   programs' slates and are read by people who did not watch them being
   written. Unit 3's went out saying "in a door that has no such enum"
   when two of three ride doors that DO have exhaustive maps, one of
   which mints the colliding word itself. **A filed row should be
   re-read against the tree before the PR goes up**, and that is now
   part of what a fix pass owes.

## Next

The **`pncad-py` block**, five rows, placed in `plan.md` §Order as a
block rather than by class: their fix shape is proven rather than
hypothetical after this unit, and two of them record facts with a shelf
life. `four-censuses-…` stays with the H rows — it asks whether four
instruments should be fewer, and three of the five change what those
instruments see.

## CENSUS-PY-GETTERS merged (2026-09-15)

PR 2663 merged as `7a187c74`, green on run `34988487218`. Seven
discriminant maps moved to where the inventory reads them; the spec is
deleted per the ledger and the item closed.

**Two lessons, and the first is about this role rather than about a
lane.**

8. **The orchestrator's verification reproduced the finding's shape
   instead of testing it.** The row counted "lowercase discriminant
   words"; the spec's independent re-measurement used a
   lowercase-anchored regex; both missed `dimension_name`'s four
   CAPITALISED words, which reach Python as `Measurement.dimension` and
   are asserted at four sites in the Python suite. The spec then
   reported the count as verified, and that claim reached Ev. **A check
   shaped like the claim it checks is not a check** — finding 6 arriving
   one level up, at the desk that wrote finding 6.

9. **An adjudication can overclaim exactly as a filed row can.** This
   unit's review reported 27 `#[pyclass]` enums minting Python-visible
   vocabulary as Rust identifiers, invisible to both sweep arms, and the
   orchestrator adjudicated it as "the sixth blind spot, bigger than the
   unit that found it" and directed a row. **The fix pass refused, with
   a probe**: `tests/test_stubs.py` holds all 114 member names against
   `pncad.pyi` name-for-name in both directions, and renaming
   `ArcSweep::Ccw` reds it plus sixteen call sites. Filing would have
   been this program's fifth overclaiming row, authored by the desk that
   wrote the rule against them. The count was 24 enums / 114 names, not
   27. **A lane refusing an orchestrator's instruction with executed
   evidence is the process working**, and it should be said plainly
   rather than absorbed.

**The trap sprang a fourth consecutive time**, in the argument again: the
unit added a scoping rule to `tags.rs`'s header — *"a tag word is scoped
to the map that mints it"* — decided over seven words and asserted over a
file that mints **61** words two or more maps speak. 54 pairs were
blanketed by a rule nobody read them against, which is a hand-written
prose census shipped by the unit closing hand-written lists. Closed with
an instrument derived from `TAG_INVENTORY` rather than with a narrowed
sentence, and the 54 are filed with their measured list.

Also overturned by execution: a stated blind spot ("a map moved wholesale
out of `tags.rs`") that the inventory's GONE branch catches, reasoned
rather than run; and the siting argument for `measurement_dimension_tag`,
which claimed a uniqueness contradicted 370 lines below it in its own
file.

**What went right and is worth keeping:** the lane's sweep was keyed on
the WORD rather than on syntax, ran a second arm *because* the first was
capitalisation-blind, and that second arm is what caught `dimension_name`.
Its filed rows were re-read against the tree and its own first draft
corrected before pushing — and the reviewer re-drove those corrections
and they held. That is standing finding 7 working without a reviewer
catching it first, for the first time.

## Next

The `pncad-py` block continues, now seven rows: the two opened here join
it. `sixty-one-tag-words-…` carries the sharper obligation — its
instrument shipped and what is missing is the READING of 54 pairs, the
kind of debt that stops looking urgent once the instrument is green.

## The silent-omission obligation stays per-dispatch (Ev, 2026-09-15)

Asked directly whether it should become a standing clause in
`docs/prompts/reviewer-style-lane.md`, Ev ruled it should not: *"this
doesn't go in reviewer-style-lane because most implementation work does
not refer to such instruments."* Signal-to-noise — a clause firing on
every unit in the repo taxes every lane for a case the majority never
meet, and a skimmed rule is worse than one written per-unit by someone
who has read the diff. `plan.md` §Review posture records it and the
question is closed.

**One thing the orchestrator overstated when putting the question**, and
it is corrected here because the log is where the reasoning lives: the
argument for a standing clause was that the obligation "depends on the
orchestrator recognising the unit", citing CENSUS-TAG-REACH as a case
where that recognition failed. It did not. **All four units so far
carried the obligation**; what failed on CENSUS-TAG-REACH was the
PROBE'S FENCE — the spec scoped it to `eval_err` when the door was
`typed_err` — which is standing finding 5, not a recognition failure.
The recognition has not missed yet.

So the residual risk is narrower than it was put to Ev: not *"will the
orchestrator notice an instrument"* but *"will the obligation be scoped
to the right unit of guarding once noticed."* The trigger is now stated
mechanically in `plan.md` (a unit carries it if it lands, changes,
removes or relies on an instrument, **including one it creates itself**)
so the first half stops depending on judgement, and finding 5 already
governs the second.

## CENSUS-PY-RAISE-LITERALS merged (2026-09-15)

PR 2682 merged as `0ac65a9e`, green on run `35006650306`. Nine raise-site
words given homes in three shapes; the spec is deleted per the ledger
and the item closed.

**The trap sprang a fifth consecutive time, and this one was predicted in
writing on this program's own slate.** `ValidationRefusal::attribute` is
a fifth `-> &'static str` map in `errors.rs` minting the Python-visible
words `"door"` and `"reason"` — and
`errors-rs-holds-four-python-visible-maps-and-nothing-enumerates-that-file`,
opened by the PREVIOUS unit, says in terms that a fifth such map arrives
with no pin and nothing saying so. The diff added exactly that, unpinned,
and left the row's table of four stale. **A row predicting the next
instance did not stop the next instance**, because nothing reads a row at
the moment a lane writes code; that is worth more than another
restatement of the trap.

It also produced the sharpest counter-example the row needed: its
proposed loose reader would walk `pub fn`s, and both this map and
`ErrorClass::class_name` are inherent methods inside `impl` blocks.

10. **Lesson: "verified the example, asserted the class."** The
    orchestrator checked `unclassified` — correct — and carried the row's
    generalisation over `wireframe` and `not_utf8`, which return nothing
    from `tags.rs`. Same shape as CENSUS-PY-GETTERS' lowercase-anchored
    check, different mechanism: there the pattern matched the claim's
    shape, here a sample of one stood for three. **The check must cover
    the claim's whole population, not its strongest member.**

11. **A stale count can be born stale.** `step_import_error_tag`'s
    "Twenty-two arms" was never right: `git log -S` puts the sentence at
    a commit where the map already had 23. The program has been treating
    prose counts as things that DRIFT; this one never matched. The filed
    row (`prose-counts-of-a-populations-size-…`) carries the class, and
    its sweep shape is the reusable part — doc-comment BLOCKS joined
    across lines, which is what hid a second stale `61` from a line grep.

### What went right

The fix pass closed F1 **class-wide** rather than narrowing the claim to
fit: `class_discriminant` now carries the class's whole attribute set,
`ValidationRefusal::ATTRIBUTES` is held to the image of `attribute()`
over `ALL` in both directions, and the chain was executed link by link —
each of four instruments shown reding in turn. The reviewer's own probe,
green before, panics now. It also made `class_discriminant` exhaustive,
so the next carrying class stops the build rather than falling into
`None`.

And the sweep that found the half-finished doc corrections found three
the brief had not listed, one of them a count that was never true.

### A process hazard worth recording

The fix pass reported using `git checkout <file>` twice to revert a probe
and losing uncommitted edits in that file both times, re-applying from
its own scripts and re-verifying. Nothing was lost from the pushed tree.
Worth knowing for any lane that reverts a probe on a file it is also
editing: stash or copy first, because `checkout` takes the whole file.

## Next

The `pncad-py` block, now nine rows with the three filed here. The block
remains coherent and its fix shapes are proven; the two rows carrying
facts with a shelf life (`prose-counts-…`'s ten unverified counts,
`sixty-one-tag-words-…`'s 54 unread pairs) are the ones that decay while
nobody touches them.

## CENSUS-ERRORS-ARRIVAL merged (2026-09-15)

PR 2691 merged as `17a68a0fa`, green on run `35023408135` at
`fa49e26b0` (35 success, 4 skipped, full matrix). The spec is deleted
per the ledger and the item closed.

**The disposition is one no bullet on the spec named, and its argument
is the unit.** Every instrument this crate had aimed at its vocabulary
was keyed on a FORM, and each went blind to the arrival that did not
wear it — the tag reader strips `pub fn `, and all five maps in
`errors.rs` are `pub const fn`. So the alarm is keyed on the file's
LITERALS: `test_utils::source` says which bytes are inside one, and the
reader attributes each to the enclosing item. The spec declined to
pre-decide among three shapes, having had two of its last four
pre-decisions overturned by a lane's probe; declining is what let a
fourth shape be found, and that is the spec practice this unit
vindicates.

**And its central claim was false, and was executed as false twice.**
The PR was first written on *"there is no form a word can arrive in
that the reader was not taught, because there is no form"*, and on *"a
wrong attribution is loud: it invents a name the roster does not
carry"*. The style reviewer falsified both against the real tree: a
CHAR literal was read and dropped along with the item spelling nothing
else, and an attribute literal was charged to the ROSTERED item above
it, where a deletion in the same item cancelled it to nothing. Both
silent. **Both are better than form-keying and both were wrong**, which
is the honest reading — the argument for literals stands, the
absolutism did not.

That is this program's **fourth consecutive short exclusivity list**
(finding 2, finding 6). Four for four, on four different lanes, each
time discovered by a reviewer executing the claim rather than reading
it.

**The unit sprang its own trap in the tracker, not in the code.** The
diff grew `crates/pncad-py/src/tests.rs` from 6798 to 7795 lines — +997,
14.7% — and left `pncad-py-tests-rs-is-six-thousand-lines-…` saying
6317, untouched. That row is on this program's own slate and says that
file is too big. It is the same failure the unit was dispatched to
punish, one level up: **nothing reads a row at the moment a lane writes
code**, including the row about the file the lane is writing in.

**The duplicate-name key was this program's own, dropped.** Two trait
`fmt` impls on one type collided and the census hard-stopped with
*"Qualify them apart in the same diff"* — which Rust gives no way to do.
`crates/test-utils/tests/hand_written_impl_census.rs`, **this program's
unit 2**, keys on `(path, trait, self type)` and says at the site why
the trait is in the key. The fix took that key back, and went one wider
than the review: the trait's GENERIC ARGUMENTS have to stay in it, or
`PartialEq<Other>` and `PartialEq` collide on exactly the case the trait
was added for.

12. **A correction can be born stale in the sentence that corrects a
    stale count.** Finding 11 said a stale count can be born stale. The
    fix pass, repairing the 6317, wrote 7779/+981 — measured before its
    last two edits to that file and committed after them, so the
    corrected number was wrong in its own commit. Caught by the
    orchestrator against `git show <sha>:… | wc -l`, and **caught a
    third time** at close-out: the item file still said "nine items and
    52 literals" after the fix pass had taken the population to ten and
    53. Three instances of one shape inside one unit. The general form
    is that a number written by hand is stale the moment anything else
    in the same diff moves, so **a count belongs with the command that
    re-derives it and the SHA it was taken at**, which is what the size
    row carries now.

    The lane's answer to "why not pin it with a check" is the right one
    and is worth keeping: a committed line count held to `wc -l` reds on
    every commit to a file expected to change, and its repair is a
    hand-edited number — the row's own complaint at per-commit
    frequency. The two vacuity FLOORS are the opposite case: they are
    assertions, they already run, and deriving them from the committed
    inventory costs nothing. That distinction is the row's remaining
    ask.

### What went right

The fix pass closed both MAJORs **wider than the charge** rather than to
fit it. Closing the char hole made the population every literal, which
surfaced an item the census had never seen (`is_bare_camel_token`, one
char literal) and took the file from nine items/52 literals to ten and
53 — so the reviewer's counterexample was also a measurement. Closing
the attribute hole turned up that the same walk mis-attributes
attributes on a `struct`, `enum`, variant or field, which the review had
not named, and added a **stray-attribute refusal** for the case where
absorbing into the item below is not available.

The shared-reader sweep was wider than its charge too: the review named
two duplications, and going to fix it found a third — one `rfind('\n')`
line-start fold at three sites. Four operations now live in
`crates/test-utils/src/source.rs`. One fix nobody asked for: `identifier`
had only the plain-alphanumeric half and read `r#fn` as `r`.

And a filed row corrected the reviewer rather than copying it: the
field-brace sweep's count was re-taken and came back 13 raw hits, of
which 7 are executable checks in 5 crates.

### A process note

`scripts/doc-gate.sh` was red on the fix pass's first push and fixed on
the next before the gate reported. Worth the lane's own note that the
doc gate belongs before a push, not after. Separately, one CI run was
cancelled by the lane's own subsequent push — a concurrency supersede
that surfaces as a `gate ok` failure event, which is the shape to read
in the run list rather than in the job logs.

### The slate had drifted from the item files, both directions

Found at close-out by comparing `work/census/*.md`'s statuses against
the slate table, which is what the Order section reasons over.

**Two OPEN rows had never reached the slate at all** —
`tag-vocabularies-restated-in-py-doc-comments` and
`ring-contact-and-census-contact-share-two-words-by-prose-alone`, both
filed by CENSUS-PY-GETTERS two units ago. So the order for units 5 and
6 was computed over a set that did not contain them. Both are placed
now, with the block.

**Four CLOSED rows were still listed as if open**, while two others
said "Landed" in their description — so the slate marked landing on
two of six. All six carry an explicit `(closed — UNIT, #PR)` now.

This is the program's own charter turned on the program's own tracker:
one population, spelled by hand in a second place, drifting in both
directions with nothing reding. It is the orchestrator's miss, not a
lane's.

**And the sweep I ran to size it reproduced finding 8 immediately.** I
measured every program's plan against its item files, got
25-open-unlisted in `blend` and 44 in `bool`, and was one step from
filing a repo-wide META row — before reading `work/README.md`, which
says `plan.md` is a *narrative, present state only* and `STATUS.md` is
the generated board. A plan is not an index and was never meant to be
one, so there is no repo-wide defect and no row to file. What is true
is narrower and local: **CENSUS's plan calls its table "the slate" and
its Order section reasons over that table**, so for this program an
absent row is an absent row. The check I demand of every lane —
measure the claim against the contract before generalising from the
example — is the check that stopped this, one step before the row.

## Next

The `pncad-py` block, and the four rows this unit filed, ordered in
`plan.md` — `the-errors-arrival-blind-spot-list-…` first of them and
soon, because its whole value is four probes already run; then
`payload-attribute-names-…` with the block;
`the-field-brace-fingerprint-…`, which is routing and ties to the
prose-census pair; and `dimension-mismatch-sentence-…`, E by size but a
design question about two crates, on a file no open program claims.

## CENSUS-ARRIVAL-RESIDUE merged (2026-09-16)

PR 2704 merged as `6b10d748d`, green on `1634ae3f9` (39 jobs, 35
success / 4 skipped, full matrix, python suite ran). Spec deleted per
the ledger, item closed. Unit 6's four disclosed residues: three
repaired, one measured and the measurement chose the disposition.

**The fifth consecutive blind-spot list was short, and by two shapes.**
A `trait` body was not a scope, so an associated `const` or a defaulted
method landed under its bare name — *verbatim* residue 3's own
complaint, closed for `mod` and left open for `trait`. **The unit had
written the counterexample into its own fixture**: a `pub trait
Declared` added for the `impl Trait` argument position, with the bare
name pinned as the expected answer one row below, and nobody read it.
And `declaration_heads` was still keyed on line starts, so
`read_minting_items("impl Subject { pub const ALL… }")` answered `{}`.

**That second one is where the form-keying came back, and it is not
where the orchestrator pointed.** The spec named `starts_an_item` as
the risk; it survived nineteen hand-built cases, admitting no type
position and rejecting no stable-Rust item form. The defect was one
reader over. Residue 1 had promoted `declaration_heads` from the
attribution rule to the POPULATION key, which turned a mis-charged
literal into a missing row — so the reader unit 6 freed from form-keying
was not the reader that defines the population. **The conclusion was
right and the location was wrong**, which is the useful shape: the
question was worth asking and the answer was worth executing.

**The hard stop kept unit 6's false premise and this unit widened its
reach.** *"Rust rejects two such items in one crate"* is false for
three legal shapes, and putting `mod` in the key exposed
`#[cfg(test)] mod` — the commonest item shape in the language. Unit 6
did the same thing with *"Qualify them apart in the same diff"*; this
diff **reworded that message and kept the premise**. Seventh
consecutive unit where the fix mints a fresh instance of the defect it
closes.

13. **A negative control that passes is a finding, not a relief.** The
    fix pass mutated `trait_key` expecting no red, as a control — and
    got none. Its generic arguments, which its own doc calls *"exactly
    the case the trait was put in the key for"*, were pinned by
    nothing. The same happened to `declares_test`'s boundaries: the
    first probes (`fnx probe`, `xfn probe`) were refused earlier in the
    pipeline and so proved nothing about the boundary at all. Both are
    the style review's own lesson one level in — **a probe shaped like
    the thing it checks** — and the general form is that a guard is
    unpinned until an input exists that fails with THAT GUARD ALONE
    removed. The unit ends with a mutation table of fifteen, each
    reding a different named test.

Standing finding 12 reached instances **five and six inside this
unit** (`tests.rs:6872` and `:7833`, both born stale at 10 when the
roster had moved to 13), in a unit whose spec said *"do not make it
five."* And a seventh mechanism the row had not carried: the size
row's count went stale **at a merge** — 7795 was true at
`fa49e26b0` and the merge `17a68a0fa` carried another lane's
`ortho_frame_error_tag` row on the other parent, so the number was
right at the SHA it named and wrong at main. *State the command and the
SHA* does not survive a concurrent change to the same file.

### The file is the program's largest debt now, and it is not ours to split

`crates/pncad-py/src/tests.rs` has gone **6798 → 7805 → 9162 over two
units, about +35%**, while `pncad-py-tests-rs-is-six-thousand-lines-…`
on this slate says it is too big. Both units updated that row in the
same diff, as asked, and much of this unit's growth is the per-entry
blind-spot list the spec REQUIRED at the site — the device that caught
this unit's trap twice. So the growth bought something.

But the debt is now the largest thing on this slate and it grows
because CENSUS keeps working in that file. **The split is not CENSUS's
to take**: `crates/pncad-py/*` is LIB's fence and this program claims
no paths. The honest next move is to hand that row to LIB rather than
take a third unit in the same file — recorded here as the
orchestrator's reading, not as a decision, because re-homing a row is
the owner's conversation.

### What went right

The measurement decided residue 1 **against the disclosure that named
it**: the old disclosure offered a map forwarding `crate::tags`' and a
word built from a kernel `Display`, and `errors.rs` holds no instance
of either, while a live case — `EvalReason::ATTRIBUTES` — sat
unrostered. A fig leaf in the precise sense. The old *"cannot see a
word that is not a literal"* test went red when the fix landed, which
is the evidence rather than the claim.

Writing residue 2's pin found a **live, user-visible defect**:
`ValidationError` is raised with `reason` and `pncad.pyi` declares only
`door`, `failure_count`, `findings`. The lane refused to just add the
line — that swaps a missing promise for a false one, since a
validator-failure raise carries the other three — and filed it with the
suppression reding if the stub gains it.

And `scope_spans` took the free scan with the shared `boundary_*`
guards that `hand_written_impl_census.rs` has always used, which is the
**second** mechanism unit 2's census had solved and unit 6 re-invented
more narrowly, after the `(path, trait, self type)` key.

### The orchestrator's own charter defect, twice

At unit 6's close-out the slate had drifted from the item files in both
directions and was reconciled by hand. **At this one it had drifted
again** — two more filed rows absent — because the first reconciliation
was a spelling correction that left no instrument, which is exactly
what this program exists to prosecute. Filed on META as
`a-plan-whose-table-orders-the-work-has-no-check-that-it-lists-the-work`,
because `scripts/work.py` is META's and CENSUS can only keep
re-spelling the table by hand. The row records the measurement that
kills the obvious rule: a plan is a NARRATIVE (`work/README.md:24`) and
`blend` and `bool` have 25 and 44 open-and-unlisted items apiece, so
the check has to be opt-in for programs whose plan is their order's
input.

## Next

Before another unit in `crates/pncad-py/src/tests.rs`, settle the size
row with LIB (above). Otherwise the `pncad-py` block, with
`validation-error-reason-…` first of this unit's residue — the only row
on the slate naming a live user-visible defect rather than a missing
instrument.

## CENSUS-HAND-LISTED-SIBLINGS merged (2026-09-16)

PR 2712 merged as `32b6af999`, green on `0687260bc` (35 success, 4
skipped). Spec deleted per the ledger, item closed. Six hand-listed
walks tied to their declarations, the added-field case executed at each
before and after, and the four `KNOWN_HAND_LISTED` entries deleted in
the same diff.

**The unit's biggest finding is about an instrument this program has
been relying on, and it is not ours.** `every_known_hand_listed_impl_is_still_found`
was documented as supplying per-impl blindness detection, marked
*"Measured"*. A one-line mutation — stop after each file's first
`Debug`/`PartialEq` impl — reds it on `origin/main` and leaves this
branch **entirely green**. So the arrival census has no per-impl sight
now and, on the measurement, never had more than **one impl's worth**:
the failure names exactly one entry, `mate/coset.rs`, the only one of
five with a compliant sibling above it. Filed on TINT, whose file it
is. What this costs CENSUS is that a row here can no longer say "the
census would catch a partial blindness"; nothing does.

**The fix minted a fresh instance of the defect it closes, for the
eighth consecutive unit, and this time at the site the unit called the
hard one.** `SignCertificate`'s render prints `VolumeEnclosure`'s
complete three-field roster under their own names — a field list, just
a different type's — under a 28-line justification concluding *"there
is no correspondence to be short of."* A fourth field on that type
compiled `topo` clean. The style review found it; the unit's own
destructure satisfied the census (`HandListed` → `Destructured`, entry
deleted, row green) while the reading that actually renders stayed
untied. Closed, and the one reader no pattern can bind — `FaceRun::open_at`
through an iterator adaptor — is named rather than papered over.

14. **A count written to correct an overclaim is the likeliest place
    for the next one.** Standing finding 12 said a stale count can be
    born stale; this is the sharper version. The sentence retracting
    *"the anchor supplies per-impl sight"* asserted that four of five
    entries had a compliant sibling — the mutation's own failure
    message names one. It reached three places at once (the census
    header, the filed TINT row whose measurement table contradicted it,
    and the PR body) because a retraction gets copied where the claim
    was. **Read the failure output, not the theory of the failure.**

Also falsified by execution, and worth keeping as shapes: a doc that
excludes an impl from a group for a reason true of its neighbours and
false of it (`Serialize` returns `Result`, not `&StableName`); a tie
claimed to reach a leaf it does not (`coset.rs` — a fourth `Vec3`
component compiles `editor-core` clean, the comparison being a file
away in `vec_eq`); and **doc rot a change creates about itself**
(`expr.rs`'s *"adding a `[u8; 6]` field here still compiles"* was true
until this diff's own destructure made it an E0027 fifteen lines
below, and the lane quoted the sentence approvingly in the same diff).

### What went right

The added-field probe was run **before** each repair as well as after,
so the silence is executed rather than asserted — that is what made
`Lit`'s case legible (one `E0063` at the sole constructor, the
`size_of` assertion beside it silent, nothing at the walk).

And the unit **refused the easy win**: a pattern in `SketchPlane`'s
`eq` would have turned the census green while the reading stayed in
`bit_eq`, and the lane declined it and said so at the site. That is the
spec's second trap shape, declined rather than taken — the first time
in this program a lane has named a trap and walked past it rather than
into it.

## Next

Before another unit in `crates/pncad-py/src/tests.rs`, settle the size
row with LIB. Otherwise the `pncad-py` block, with
`validation-error-reason-…` first — the only row on the slate naming a
live user-visible defect rather than a missing instrument.

## Rows arriving from FIX, 2026-09-20

Ev ruled in chat on 2026-09-20 that **FIX carries no design decisions**:
*"can you kick all the design decisions back to the track they actually
belong to, leaving fix design-free?"* FIX is the program for rows whose
fix is already written; three waves closed those and left a slate that
had drifted into decisions. Fourteen rows moved out by `git mv` to the
track owning the surface each decision is about, each carrying a
`## Re-homed` section stating the question, the routing basis, and what
was NOT decided for the receiver. Routing was taken from
`python3 scripts/work.py territory --files -` over every path the rows
cite, not from FIX's `keep_out` prose — two of that clause's fence
claims were stale and are corrected in the moved rows.

**Four rows, and three of them edit one file.**

- `prose-census-undecided-residue` — the `UNDECIDED` roster's three
  classes, each with two directions and no decision.
- `census-cannot-type-a-nested-pattern-binding` — seven of those
  twenty-eight rows are a fourth class the residue row does not name, and
  were misattributed to it until a lane read them.
- `collapsed-continuation-guard-belongs-in-the-prose-census` — home
  settled (the needle is inside string literals, so `gate_rust_code`'s
  code-only view cannot see it) and threshold measured by PR 2364 at run
  >= 9, not the 4 the closed row guessed.
- `a-new-kind-pair-arrives-unguarded-by-default` — the residue of
  `kind-mirrors-have-no-single-declaration`, which Ev DECLINED on
  2026-09-12 after its scale check came back feasible. Do not re-open the
  macro from that feasibility answer.

**The first three all edit `crates/pncad-py/src/prose_census.rs`** (LIB's
path; the instrument's *contents* are CENSUS's charter). They moved
together on purpose: two programs dispatching into that file in one week
is a merge conflict, and one board can order them. The collapsed-
continuation row is arguably design-free already — if you would rather
hand it back as a written fix, FIX will take it.

**One half of the kind-pair row is answered.** Ev ruled in the same
sitting that the convention sentence's home is **the owning crate's
`README.md`**, not `docs/DESIGN.md`. Still open: the sentence, and whether
the two purpose-built guards (REACH's `boolean/mod.rs:2878`, WIRE's
`product.rs:963`, ~230 lines with their `label()` tables) earn their keep
having never fired. That second half wants those two programs' assent, not
an announcement.

Signed (FIX orchestrator).
