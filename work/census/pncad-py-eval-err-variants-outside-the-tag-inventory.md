---
id: pncad-py-eval-err-variants-outside-the-tag-inventory
kind: unit
title: TAG_INVENTORY cannot see a refusal variant minted at an eval_err call site, and measure_unavailable is pinned nowhere
status: closed
opened: 2026-09-04
branch: census/tag-reach
closed: 2026-09-15
---


Found while repairing the code-tier red in M10's closed
`pncad-py-tag-inventory-misses-two-measure-tags` (`docs/DOC-LEDGER.md`
sweep 13). Not that
item's defect and not repaired with it: that one is two inventory
lines, this is a question about the gate's reach.

**The gate reads one file.** `the_whole_tag_table_matches_its_committed_inventory`
lexes `crates/pncad-py/src/tags.rs` and nothing else
(`crates/pncad-py/src/tests.rs:24`, "the shared Rust-source lexer:
`src/tags.rs` is READ by the tag-table…", and every panic in the reader
is spelled `tags.rs: …`). But a Python-visible refusal variant does not
have to come from `tags.rs`: `eval_err` takes the variant as a `&str`,
and eight call sites under `crates/pncad-py/src/py/` pass a STRING
LITERAL instead of a `tags::` function's answer. They mint four words:
`wrong_kind` (five sites, e.g. `crates/pncad-py/src/py/value.rs:586`,
`:622`, `:691`), `empty_boolean` (`value.rs:580`), `unknown_node`, and
`measure_unavailable` (`value.rs:721`). Those words reach Python as
`.variant` exactly like a `tags.rs` word does, and the inventory cannot
see any of them.

**Three of the four are covered by accident, and the fourth is not.**
`wrong_kind`, `empty_boolean` and `unknown_node` are each pinned in
`crates/pncad-py/src/tests.rs` anyway (3, 2 and 7 occurrences) and each
is named in `crates/pncad-py/pncad.pyi`. **`measure_unavailable` is named
in exactly one place in the repository** — the call site that mints it,
`crates/pncad-py/src/py/value.rs:721` — and in no test, no `.pyi`, no
Python test and no doc. It was added by M10-6 (PR 1685, commit
`7cb46c6ba`, MINOR-4) as the read door's own arm, in the same PR that
added the two `node_error_tag` values the sibling item is about; those
two the gate caught, this one it structurally cannot.

**Why it is probably not a bug today, and why it is still worth
recording.** The door that mints it — `Value.measure` on a
`min_clearance` — is unreachable from Python until measure AUTHORING
ships, which `crates/pncad-py/tests/test_binding_census.py` records as
the `B-MEASURES` census gap by name (`MeasureUnavailableAt`,
`MinClearanceRefusal`). So there is nothing a Python test could observe
yet, and the absence is the same "correct surface" answer the sibling
item got. What is NOT settled is the gate's reach: the next literal
variant added at an `eval_err` call site will be public Python
vocabulary that no inventory looks at, and the sibling item's whole
lesson is that a word which reaches Python with no gate over it is a
red waiting for an unrelated branch to find.

**Two ways to close it**, either of which is a small change:

1. widen the reader to lex the literal-variant `eval_err` call sites
   under `crates/pncad-py/src/py/` into their own inventory row, so
   every Python-visible variant is in exactly one table; or
2. rule that a literal at a call site is *deliberately* out of scope,
   say so where the reader's `tags.rs`-only scope is stated
   (`crates/pncad-py/src/tests.rs:24`), and pin `measure_unavailable`
   somewhere so the one uncovered word stops being uncovered.

Territory note: `crates/pncad-py/*` is LIB's fence and the gate is
LIB's (`434964dfa`), but LIB is not active and the uncovered word is
M10-6's, so this is filed to M10 on the same reasoning Ev gave for
re-homing the sibling item (2026-09-04).

## The uncovered word closed itself; measured 2026-09-15 at spec time

**`measure_unavailable` is no longer minted as a literal and is no
longer uncovered.** `crates/pncad-py/src/py/value.rs` routes
`ValuePayload::MeasureUnavailable` through
`super::measure::measure_unavailable_at_err`
(`crates/pncad-py/src/py/measure.rs`), whose `variant` field reads
`measure_unavailable_at_tag(reason)` — a `tags.rs` function, inside the
inventory's reach. The word appears 19 times across the tree and is
pinned in `src/tests.rs`. The body above cites `value.rs:721` as the one
place naming it; that line is today `Datum::__repr__`.

Nobody closed this row when the code closed its instance. **That is the
decay direction that inflates a board**: the row still read as a live
leak with a word ungated, when what it actually holds is a design
question about the gate's reach.

**What remains, measured:** seven literal sites in `value.rs` minting
three words — `wrong_kind` (`:884`, `:920`, `:995`, `:1034`, `:1047`),
`empty_boolean` (`:878`), `unknown_node` (`:1171`) — not the eight sites
and four words above. All three are covered by accident, in
`src/tests.rs` and `pncad.pyi`, so **no word is uncovered today**.

## The disposition, taken at spec time (orchestrator, 2026-09-15)

Neither of the two closes the body offers. `value.rs:1164`, in the same
function as two of the literals, already passes `NODE_NOT_EVALUATED` —
a `pub const` imported from `crate::tags` — and the gate already reaches
consts: `src/tests.rs` carries `TAG_CONSTS`, the committed inventory of
`tags.rs`'s `pub const` tag words, its reader parses
`pub const NAME: &str = "value";` beside the `pub fn` form, and a new one
reds.

So the fix **moves the word, not the reader**: each literal becomes a
`pub const` in `tags.rs` and is imported at the call site. Widening the
lexer to scan `src/py/` would mint a new source-text reader, and a reader
needs a guard of its own — this program's trap, sprung by both previous
units. Ruling literals permanently out of scope documents a second
channel instead of closing it. `docs/CENSUS-TAG-REACH-SPEC.md` carries
the argument.

## Re-homed at M10's exit sweep (2026-09-13)

Here because the class is this program's charter exactly: a vocabulary spelled by
hand in more than one place, and the census that cannot see one of the spellings —
`TAG_INVENTORY` lexes `tags.rs` alone while eight `eval_err` call sites mint four
words as string literals. `crates/pncad-py/*` is LIB's territory and this program's
`keep_out` already says its pncad-py rows are announced there.

From `work/m10/` at M10's close (`docs/DOC-LEDGER.md` sweep 13; the walk and the directory are recoverable at the SHA it names). The id is unchanged.


## Landed, and what the lane corrected (2026-09-15)

**The spec's criterion was narrower than the class, on the sharpest
point of all: the attribute.** This row and the spec both say the
words reach Python as `.variant`. They do not. `eval_err` writes a
field named `reason` and the stub declares `EvaluationError.reason`
(`crates/pncad-py/pncad.pyi`); `.variant` is a DIFFERENT attribute,
carried by other doors. Sweeping for `variant` alone would have found
none of these seven and would have reported a clean negative.

**The seven sites and the three words are correct**, re-measured. What
the criterion "an `eval_err` call site" hid is that the same file mints
two MORE `EvaluationError.reason` words at direct `typed_err` sites —
`node_failed` and `poisoned`, the same door and the same vocabulary —
and `crates/pncad-py/src/tests.rs`'s own inventory doc already listed
all six as one family.

**The count, stated once and correctly** (the first version of this
paragraph reached ten two different ways): `py/value.rs` held **ten
sites minting `EvaluationError.reason`** — eight `eval_err` calls, of
which **seven passed a string literal** (`wrong_kind` ×5,
`empty_boolean`, `unknown_node`) and one passed the `NODE_NOT_EVALUATED`
`pub const` from `crate::tags`, plus the two direct `typed_err` raises
in `node_failure` and `poisoning`, which spelled `node_failed` and
`poisoned` as literals. **Nine literals, one const, ten sites, one
door, six words.** `mass_properties_failed` is an eleventh literal
`reason` in the same file and is NOT this door — it is a
`ValidationError` — so it is outside this count and filed.

**The disposition held, one type deeper.** Moving the words to
`pub const`s would have left the answer to "what stops the next word"
at *nothing*, and the lane executed the spec's probe to prove it: with
a fresh literal `eval_err` site minting `probe_empty_boolean_datum`,
85 Rust tests and 832 Python tests passed. So `eval_err`'s reason is a
TYPE — `crate::errors::EvalReason`, the door's whole vocabulary, mapped
by `crate::tags::eval_reason_tag`, an exhaustive `match` the tag-table
guard already reads. `NODE_NOT_EVALUATED` folded into it and
`TAG_CONSTS` is now empty; the reader's `pub const` branch is held by
`the_tag_table_reader_recognises_every_form_it_claims`, a fixture-driven
self-test the lane added because the file stopped supplying an instance.

**And the wall moved from the function to the door (fix pass).** As
landed, the type guarded `eval_err` alone: `node_failure` and
`poisoning` called `typed_err(py, ErrorClass::Evaluation, …)`
directly and passed `("reason", eval_reason_tag(…))` by hand, so a
fourth such site — in this file or a new one — could still spell a
word of its own, and the review executed exactly that and got 86
passing tests. `ErrorClass::Evaluation` now CARRIES the `EvalReason`,
so the class cannot be named without naming the reason; `typed_err`
mints the word and attaches it last, and asserts against a raise site
that passes one too. Probed: a new file under `src/py/` minting
`probe_bypass_word` at a direct `typed_err` site does not compile
(`E0308`, *expected `ErrorClass`, found enum constructor*).

Six residues are filed rather than swept:
`work/census/py-reason-and-variant-literals-outside-any-enum.md`,
`work/census/datum-kind-vocabulary-is-hand-spelled-and-uncensused.md`,
`work/census/evaluationerror-stub-lists-five-reasons-and-the-door-raises-six.md`,
and — from the style review and this unit's fix pass —
`work/census/py-discriminant-getters-under-src-py-are-outside-every-inventory.md`,
`work/census/four-censuses-of-python-visible-vocabulary-in-one-crate.md`,
`work/census/pncad-py-tests-rs-is-six-thousand-lines-and-carries-two-unremeasured-floors.md`.

## The blind-spot list was short again, and by how much (fix pass, 2026-09-15)

The disclosed sweep was **a literal beside a `"reason"` or
`"variant"` key**, and its blind-spot statement claimed the shapes it
could not match. Measured against the tree, it could not see four more
shapes, and three of them hold words:

* **a `fn … -> &'static str` getter**, which is neither a literal
  beside a key nor a struct field: **30** such functions under
  `src/py/`, six of them minting **23 distinct Python-visible words**,
  two of the six named `*_tag` and living outside `src/tags.rs`;
* **a word in tuple position** feeding a `variant` field
  (`py/mesh.rs`'s `not_utf8`);
* **a word passed as a `&'static str` ARGUMENT** to a raise helper
  (`py/doc.rs`'s `name_serialize` through `boundary_edit_err`, and the
  four `ValidationError.door` words through `run_validator`);
* **an `Option<&'static str>` getter** — 17 of them, and this one is a
  clean negative: every one delegates to `crate::tags`.

This is the third unit running whose blind-spot list was short, which
is `work/census/log.md`'s point 2. The two rows above carry the
population.

**Class E was right**, and only because the disposition was made in the
spec. The design call — which of the three closes — is the whole
difficulty of the row; with it made, the diff is one crate and no
judgement.
