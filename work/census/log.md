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
