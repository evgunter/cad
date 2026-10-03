---
id: assert-f6-dump-lists-are-hand-written-mirrors-of-error-enums
kind: unit
title: assert_f6's dumps lists mirror whole error enums by hand, with nothing to say the enum grew
status: closed
opened: 2026-09-12
branch: tint/1-assert-f6-dumps
closed: 2026-09-15
pr: 2648
---



Found by DOOR's `dimension-all-has-readers-outside-the-viewer` sweep,
which converted the one `dumps` list in this file that had an `ALL` to
read (`a_dimension_reaches_refusal_prose_as_a_word_not_as_its_variant`,
now `Dimension::ALL.iter().map(|dim| format!("{dim:?}"))`). The
neighbours have no `ALL` to read, so they are a different fix and a
different row.

## The finding

`crates/editor-core/tests/display_contract.rs`'s `assert_f6` takes a
`dumps: &[&str]` of the variant identifiers that must NOT appear in a
rendering — the F6 claim that a refusal reads as a sentence and never
as a struct dump. Every caller writes that list by hand, and each list
is a complete mirror of one error enum's variant names:

- `interrogate_error_display_names_its_content_not_its_struct` — ten
  `InterrogateError` identifiers (`:161`).
- `parse_error_display_names_its_content_not_its_struct` — ten
  `ParseError` identifiers (`:306`).
- `select_refusal_display_names_its_content_not_its_struct`
  (`SelectRefusal`, `:224`),
  `node_pick_error_display_names_its_content_not_its_struct`
  (`NodePickError`, `:75`),
  `resolve_indeterminate_display_names_its_content_not_its_struct`
  (`ResolveIndeterminate`, `:116`),
  `resolve_fault_display_names_its_content_not_its_struct`
  (`ResolveFault`, `:284`) and
  `declare_error_display_names_its_content_not_its_struct`
  (`DeclareError`, `:144`).

Nothing ties any of them to the enum. A variant added tomorrow is not
in its list, so an arm that renders it through `Debug` passes this
suite green while the header says the opposite — the same failure mode
the `Dimension` list had, one class up.

## Why it is not fixed where it was found

The `Dimension` fix was to read a published `Dimension::ALL`. These
enums publish nothing of the kind, and `assert_f6`'s `dumps` cannot be
derived from a type in safe Rust, so the answer here is a decision
(publish an `ALL` per error enum? a census row per enum, as
`m4_pr1_dims::all_is_every_dimension` is for `Dimension`? read the
identifiers out of the source, as `test_utils::source` already lets
this tree do?) rather than a substitution. That is a unit, not a
one-line conversion, and this file is S-TINT's ground.

Note the ceiling on any answer: the `cases` fixtures beside each
`dumps` are hand-written too, so a ban list that covered every variant
would still only bite on a rendering the fixture list reaches. The two
halves want deciding together.

## Re-derived (2026-09-15, lane C)

**VERDICT: REPRODUCES**, and the drift the row predicted has already
happened in **three of the seven** lists. Nothing was fixed; the row now
has live instances rather than a forecast.

**The mechanism is unchanged.** `crates/editor-core/tests/display_contract.rs`'s
`fn assert_f6<E: Debug + Display>(err, wants, dumps)` still takes a
`dumps: &[&str]`, and all seven callers still write it by hand. None of
the three shapes the row weighs (an `ALL` per error enum, a census row
per enum, reading the identifiers out of the source through
`test_utils::source`) is in the tree.

**Each list against its enum, re-derived by name.** Variants taken from
the `pub enum` declaration in each source file; `dumps` entries counted
in `display_contract.rs`:

| row | enum (and where it is declared) | variants | in `dumps` | missing |
| --- | --- | --- | --- | --- |
| `interrogate_error_display_names_its_content_not_its_struct` | `InterrogateError`, `crates/editor-core/src/names/interrogate.rs` | 10 | 10 | — |
| `parse_error_display_names_its_content_not_its_struct` | `ParseError`, `crates/editor-core/src/parse.rs` | **11** | 10 | **`Dimension`** |
| `select_refusal_display_names_its_content_not_its_struct` | `SelectRefusal`, `crates/editor-core/src/names/geompred.rs` | **8** | 8 entries, but one is `"Angle"` (a `Dimension` identifier, deliberately) | **`Band`** |
| `node_pick_error_display_names_its_content_not_its_struct` | `NodePickError`, `crates/editor-core/src/resolve/pick.rs` | 5 | 5 | — |
| `resolve_indeterminate_display_names_its_content_not_its_struct` | `ResolveIndeterminate`, `crates/editor-core/src/resolve/mod.rs` | 3 | 3 | — |
| `resolve_fault_display_names_its_content_not_its_struct` | `ResolveFault`, `crates/editor-core/src/part.rs` | 3 | 3 | — |
| `declare_error_display_names_its_content_not_its_struct` | `DeclareError`, `crates/editor-core/src/names/flush.rs` | **3** | 2 | **`Edit`** |

**Three live holes**: `ParseError::Dimension { .. }`,
`SelectRefusal::Band(BandError)` and `DeclareError::Edit(EditError)` are
each a variant whose identifier no `dumps` list bans, so an arm that
rendered one through `Debug` passes this suite green while the file's F6
claim says the opposite. Each is a newer variant than its list — exactly
the sentence *"A variant added tomorrow is not in its list"*, three times
over, with nobody noticing.

Note the second and third are payload-carrying wrappers
(`Band(BandError)`, `Edit(EditError)`), so the F6 exposure is doubled:
neither the wrapper identifier nor anything about the inner error's own
rendering is banned.

**The stated ceiling still holds and is now measurable.** The `cases`
fixtures beside each `dumps` are still hand-written, so even a complete
ban list bites only on the renderings those fixtures reach — and for the
three missing variants there is no fixture either, so completing the ban
lists alone would change nothing until a case is added for each.

**How this was derived, and its blind spot.** Variant names extracted
from each `pub enum` body at depth 1 and eyeballed against the declaration
(`sed`/`grep '^    [A-Z]'` over the enum body); `dumps` arrays read in
full. **What that could not match**: a variant declared behind a `#[cfg]`
(none of these enums has one today), a variant whose identifier is a
substring of another so that `assert_f6`'s `contains` check accidentally
covers it, and any eighth `assert_f6` caller outside
`display_contract.rs` — `grep -rn "assert_f6"` finds the helper and its
callers only in that file.

**Recommendation (orchestrator's call).** Keep open and raise its
priority: the row is no longer a forecast. The three holes are evidence
for whichever of the three shapes is chosen, and a unit that publishes an
`ALL` (or a source-read census) would have to name them as its
regression test.

## Closed — TINT-1, PR #2648 (2026-09-15)

**The three forecast holes were live and are shut; a fourth and a fifth
turned up while shutting them.** `ParseError::Dimension`,
`SelectRefusal::Band` and `DeclareError::Edit` were each unbanned by a
hand-written `dumps` list its enum had outgrown. Closed, with cases.

**What the guard now is, stated exactly — the spec overstated this and
the overstatement is withdrawn.** `docs/TINT-1-SPEC.md` said *"use the
compiler"* and *"a variant added tomorrow makes this file fail to
COMPILE"*, and the first implementation put *"rustc is the census"* in a
doc comment. **That is false and the review caught it.** rustc checks a
`match`'s PATTERNS; it cannot check that the author then did the right
thing. What exists is three things welded in a chain:

1. a wildcard-free exhaustiveness token per enum, which **forces the
   author to open the file** when a variant is added — and nothing more;
2. the identifier read off each value's own derived `Debug`
   (`test_utils::f6::variant_identifier`), which is **ground truth**
   rather than a hand-typed string;
3. a set difference between the rendered cases and the `*_VARIANTS`
   roster, which therefore **enforces the roster's spelling** instead of
   trusting it.

**The first implementation had only (1) and (3), and the review proved
the gap empirically**: the arms returned hand-typed strings, so an arm
reading `ParseError::UnknownUnitSymbol { .. } => "UnknownUnit"` — the
shape a RENAME produces, since rustc forces the pattern and nothing
forces the string — left the suite **green while banning a dead
identifier and leaving the live one unbanned**. Demonstrated on the
pre-fix file, not argued. Step (2) is what closed it.

**The one hole that remains, disclosed at the helper and in the PR:**
add a variant, add its arm (compiler-forced), then add **neither a case
nor a roster entry** — both sets stay empty of it and the difference is
silent. Closing that needs the variant list itself to be derivable,
which safe Rust does not offer from a test crate over a type it does not
own, without a macro or derive. Every other path is caught: variant
added (compile error), renamed (roster/`Debug` disagree), roster entry
without a case, case without a roster entry, misspelt roster entry, and
a `Display` arm that dumps.

**Two live holes found while fixing, both now closed in the same diff:**

- `a_predicate_flip_names_its_signs_as_words` banned `["Positive",
  "Negative"]` against a **three-variant** `Sign` — `Zero` unbanned. The
  fourth instance of this row's own class, inside the file being
  repaired.
- `NodePickError::Tessellate` was asserted outside the `cases` loop with
  a bare `contains` pair, so it had **never run the F6 shape at all**.

**And the predicted divergence had already happened.** `m4_pr4_hit.rs`
banned `"node:"`; `display_contract.rs` banned `"node:"` and `"name:"` —
two spellings of one predicate, drifted, exactly as
`work/view/f6-display-predicate-is-spelled-three-times-with-no-home`
said. The predicate now has the home that row names
(`crates/test-utils/src/f6.rs`); evidence was added to that row rather
than opening a second, and it stays open for
`crates/viewer/tests/panel_edits.rs`, which is outside this fence.

**What the unit did NOT do**, so nobody reads more into it: it did not
touch `crates/*/src/**` at all; it did not give `SelectRefusal`
compiler-enforced coverage (it is `#[non_exhaustive]`, so ADDITION stays
unchecked from the test crate — rename and removal are caught like
anywhere else, and the wildcard `panic!` is not a guard); and it did not
sweep the sibling suites it found.

## Residues, each with its own file

- `work/wire/select-refusal-coverage-is-not-compiler-enforced-from-the-test-crate` —
  WIRE's ground (`crates/editor-core/src/names/geompred.rs`).
- `work/tint/sibling-display-contract-suites-hand-mirror-their-enums-too` —
  this slate. Carries a **live, exercised** hole:
  `crates/mesh/tests/errors.rs` bans 14 of `TessellateError`'s 15 and the
  suite already constructs the missing `Band` case. Also records that
  this unit ADDED two instances of the const-shaped ban list its own
  sweep blind spot names, so the next sweep looks for both shapes.
- `work/edit/persist-check-renders-enum-variants-through-debug-into-user-prose` —
  EDIT's ground; `slot {slot:?}` puts a variant identifier in a
  user-facing sentence.

Review: one style review against `docs/prompts/reviewer-style-lane.md`,
no A/B row — the ratified posture. It returned fourteen style findings
and refuted part of this seat's own reading; eight were adjudicated into
a fix pass and the rest are recorded on the PR.
