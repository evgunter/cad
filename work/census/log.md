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
