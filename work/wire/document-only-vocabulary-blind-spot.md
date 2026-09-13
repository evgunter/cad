---
id: document-only-vocabulary-blind-spot
kind: issue
title: The construct-hop censuses are anchored only on the kernel vocabulary: a document-only variant that launders into a kernel form is invisible
status: closed
opened: 2026-09-12
branch: wire/census-localise
pr: 2501
closed: 2026-09-13
---

## Finding

`editor-core`'s two vocabulary censuses at the profile construct hop
(`crates/editor-core/tests/switch_program_vocabulary.rs`) are both
anchored on the KERNEL vocabulary and only on it:
`every_arc_mode_is_a_document_program` walks `profile::ArcMode::ALL`
through `mode_witness`, and `every_target_form_is_a_document_program`
walks `profile::TargetKind::ALL` through `target_witness` (D364). That
is the right anchor for the failure those censuses exist to catch — a
kernel variant the document vocabulary never learns — and it is
deliberate.

The direction it does not cover is the mirror one. `ProgramArcData`
(`crates/editor-core/src/program.rs`, near `ProgramTarget`) and
`ProgramTarget` are document-layer enums in their own right. A variant
added to either must be discharged by the exhaustive matches that
consume it — `res_spec` / `res_target` (the construct hop), the wire
conversions, `spec_lit`, `spec_slots`, the content-key hashers — so it
cannot ship un-noticed. But every one of those arms may legally
resolve the new document variant into an EXISTING kernel variant, and
when it does:

- both censuses stay green, because `ArcMode::ALL` and
  `TargetKind::ALL` are still fully witnessed;
- the corpus clauses stay green, for the same reason;
- and the document form silently authors something the author did not
  write — the same laundering failure the censuses catch in the other
  direction.

The two witness functions are matches on the kernel tag, so nothing
forces a document-only variant to acquire a witness at all.

## Why it is not D364's diff

D364 gave `profile::Target` its tag and re-anchored the target census
on it. Closing THIS direction needs a document-side tag
(`ProgramTargetKind` / `ProgramArcMode`) with its own `ALL` — a third
spelling of each vocabulary in a crate that already spells them twice
by G1 layering, which is a design call about where the anchor belongs
rather than a test change. D364 states the blind spot in the census's
own doc comment and cites this file.

## Shape of a fix, if it is wanted

Either a document-side tag projected from the document declaration the
same way (two more macro invocations, two more `ALL`s), or a census
clause that pairs each kernel form with the SET of document variants
that resolve to it and asserts that set is exactly witnessed — which
still needs a way to enumerate the document variants.

## Closed 2026-09-13 (PR 2501)

Closed with an anchor, not a floor, and without the third spelling.

`declared_variants` reads `ProgramArcData`'s and `ProgramTarget`'s
variant names out of `editor-core/src/program.rs` through
`test_utils::source`, and that IS the document vocabularies' `ALL`:
the set still has exactly one home and it is still the declaration
itself. `every_document_arc_spec_is_witnessed` and
`every_document_target_is_witnessed` are set equalities between that
set and the set this suite witnesses — a bijection, so neither an
unwitnessed variant nor a stale witness passes, where a count would
have allowed both. Each asserts the scanned set non-empty on its own
first.

A document-only variant's witness goes in `document_only_arc_specs` /
`document_only_targets`, each entry a pair naming the kernel form its
resolution is DECLARED to produce — so the laundering is written down
and checked rather than silent. Both are empty today, which is the
honest state: every document variant is one per kernel variant, so
the kernel-keyed witnesses already cover them.

Why not the document-side tag: minting `ProgramArcMode` /
`ProgramTargetKind` with their own `ALL` is a third spelling of each
vocabulary in a crate that spells them twice by G1 layering, which
this row itself calls a design call about where the anchor belongs
rather than a test change. It stays available; nothing here forecloses
it.

Measured. `ProgramArcData::Chord` laundering into `ArcData::Radius`
and `ProgramTarget::Origin` laundering into `Target::Start`, both
discharged at every site the compiler named (including the new
`spec_label` arm, which forces a LABEL and not a witness — the row's
point exactly): five of the seven clauses in the file stayed GREEN,
and only the two new ones went red.

    declared and unwitnessed (give each a `document_only_arc_specs`
    line naming the kernel mode it resolves to): ["Chord"]

The stated limit is the lexer's: an enum whose variants come from a
macro is invisible to a textual walk. Both are written out today, and
that is where the door says to look if either stops being.

## Re-taken at review R1: the anchor is compile-time, and the verb side was missing

The first close read the declaration as TEXT. The review broke it: a
variant carrying **any attribute** — `#[doc(hidden)]`, `#[cfg]`,
`#[serde]`, `#[allow]` — puts the attribute in front of the name, the
walk reads an empty name off the front of the range, the filter drops
it, and the two sides agree over a set missing exactly the variant the
census exists to catch. Measured: the same laundering mutant went from
`5 passed; 2 failed` to **`7 passed; 0 failed`** with `#[doc(hidden)]`
added and nothing else. A silent green is the one failure mode a census
must not have, and the non-emptiness assertion did not cover it — it
catches a TOTAL scan failure, and this is a partial one.

**The anchor is now `ALL_NAMES`**, projected from each enum's
declaration by `document_vocabulary!` in
`crates/editor-core/src/program.rs` — the same construction `profile`'s
`transition_table!`, `arc_modes!` and `target_forms!` give the kernel
vocabularies, and a derived constant rather than a third spelling: the
enums keep their declarations, their variants, their docs and their
derives. A variant that reaches no witness fails at COMPILE-derived
comparison, and nothing in front of its name changes what the macro
projects.

The review also found the row closed for two of its three vocabularies.
**`ProgramStep` has the same hole at `res_step`**, and `chain_steps()`
is a `Vec` that forces no verb — its own doc says so. Measured:
`ProgramStep::Dash` laundered to `Step::Tangent` and discharged at every
site the compiler named passed **7 of 7** on the old shape. All three
vocabularies are censused now.

**The empty allow-lists are gone.** The file's own verb census says
*"an empty escape hatch is a hatch that will be used"* fifteen tests up,
and two empty ones were shipping below it. A document-only variant now
reds until it has a witness in `chain_steps`, which is also what makes
the wire round-trip and the slot bijection cover it — an allow-listed
variant would have been DECLARED and not COVERED. The day a real one
exists, the census needs a shape that can hold it, decided then with its
argument.

Measured at R1, all three at once, each variant carrying `#[doc(hidden)]`:
five clauses green, three red, one per vocabulary.

    declared and unwitnessed — give it a witness in `chain_steps` ...: ["Dash"]
    ... ["Chord"] ... ["Origin"]

## The roster went the same way as the set (R1, second pass)

`ALL_NAMES` closed *a variant arrives without a witness* and left *a
VOCABULARY arrives without a census* open: three hand-typed call sites,
nothing saying why three was the right number. That is the same soft
edge one level up, and the same rule applies — prefer a bijection.

**A bijection was available.** The three enums were already contiguous
in `program.rs`, so they are declared through ONE
`document_vocabulary!` invocation, which projects
`DOCUMENT_VOCABULARIES` alongside each enum's `ALL_NAMES`. The census
iterates that constant. A fourth vocabulary declared through the macro
arrives in the census rather than waiting for a fourth call site.

The invocation is **single by construction within one module**: the
roster constant is emitted once per invocation, so a second invocation
in the same module is an `E0428` duplicate. The qualifier is
load-bearing and was missing from the first statement of this — `E0428`
is scoped to a module's value namespace, so a second invocation in a
CHILD module compiles clean and projects a second roster the census
never reads. `program.rs` has no child modules, so the list is complete
today; that fact is what makes it complete, not the macro.

The witness SETS cannot be projected — only the suite knows which walk
of `corpus()` answers for which vocabulary — so they are bijected
instead: the names the test can supply a witness for and the names the
roster carries are compared as sets. Measured, with a fourth vocabulary
added to the invocation and no witness line:

    declared and uncensused — add the walk of `corpus()` that answers for it
    to `corpus_vocabulary` and its line to `witnesses` above; until then
    nothing in this file says anything about it: ["ProgramHatch"]

The loop also collects rather than asserts per vocabulary, so three
offenders are reported as three. Measured, with a document-only variant
in each vocabulary, each carrying `#[doc(hidden)]`: **`3 of the 3
document vocabularies are short`**, naming `["Dash"]`, `["Chord"]` and
`["Origin"]` in one run. Asserting inside the loop would have named
only the first — the defect this row's own class is about, re-introduced
by the loop that fixed a different one.

**The residual is disclosed at the site and filed**: an enum declared
with a plain `pub enum` rather than through the macro has no
`ALL_NAMES`, is absent from the roster, and nothing detects that it
should have been in either. Closing that needs a walk over the file's
declarations — a text scan, which is what this PR removed, measured, for
being silently wrong on an attribute.
`work/docm/a-document-vocabulary-declared-outside-the-macro-is-uncensused.md`.

## The fourth vocabulary, found by the delta round

The disclosure above stated its residual class faithfully and
**hypothetically**, while the live instance sat four lines below the
invocation's closing brace. `LoopProgram::resolve` is a fourth construct
hop of exactly the described shape — it matches the document vocabulary
and builds `Step::Circle` / `Step::CircleSplit` — so a carrier form
added to `LoopProgram` alone launders into an existing kernel step with
every clause in the file green, and `corpus_vocabulary` explicitly
declined to witness it.

Decided on that evidence: **`LoopProgram` is a document vocabulary**, it
is declared through the macro, and it is witnessed from `corpus()`'s own
loops — all three of its variants were already in the corpus, so what
the witness cost was the decision, not the code. The membership test is
now stated at the site: *does a variant launder into an existing kernel
form at a construct hop*. `ProgramRefusal` and `RecordedProgramError`
fail that test — no construct hop builds a kernel form out of them — and
the file now says so rather than leaving it to be inferred.

Measured: a fourth `LoopProgram` carrier resolving into `Step::Circle`
reds this census and nothing else.
