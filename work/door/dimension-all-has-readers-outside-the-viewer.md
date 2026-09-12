---
id: dimension-all-has-readers-outside-the-viewer
kind: issue
title: Dimension::ALL has ten hand-written mirrors that could read it, five files, one of them inside crates/viewer
status: closed
opened: 2026-09-11
closed: 2026-09-12
branch: door/dimension-all-readers
---


Found by the sweep for
`dimension-radio-row-is-an-unforced-mirror-of-a-kernel-enum` (PR
2391), which published `editor_core::Dimension::ALL` and retired the
one mirror that row named. The sweep was scoped to `crates/viewer/src`
by the unit's brief; re-run over `crates/` it returns five more
complete hand-written enumerations of the same four variants, none of
which the unit touched.

## The hits

Two are in ANOTHER crate, which makes them the same defect as the row
that is closed — a complete mirror of a vocabulary the mirroring crate
does not own:

- `crates/pncad-py/src/tests.rs:45` —
  `dimension_tags_match_the_kernel_prose` iterates a four-entry array.
  Its own doc calls the tag and the prose word "two spellings of one
  closed list"; a fifth dimension leaves the pin covering four of five
  and green.
- `crates/pncad-py/src/tests.rs:1362` —
  `literal_refusals_come_from_the_kernel_with_stable_tags`, whose
  comment says "the reachable set, exhaustively: every dimension". A
  fifth dimension makes that sentence false with nothing red.

Both are `for dim in [ …four… ]` and both become `for dim in
Dimension::ALL` unchanged.

Three are inside the declaring crate, so the compiler at least stands
over them; they are still second copies of a list the crate now
publishes:

- `crates/editor-core/tests/switch_display_units.rs:417` — a `dims`
  local of all four, under a doc that also carries the prose count
  "6 rows x 4 dimensions". Reads `Dimension::ALL` directly.
- `crates/editor-core/tests/u8a_parse.rs:482` and `:725` — proptest
  `prop_oneof![Just(Length), Just(Angle), Just(Scalar), Just(Count)]`.
  Not a drop-in: the projection is `proptest::sample::select(&
  Dimension::ALL[..])`, a strategy of a different type, so each call
  site wants reading rather than substituting.

A sixth site is deliberately NOT one of these and must not be
projected: `crates/editor-core/tests/u8a_parse.rs:506`'s `rt_params`
maps four arbitrary parameter NAMES onto the four dimensions. The names
are the fixture's own invention, so there is nothing to project from —
what it wants, if anything, is a growth alarm, which is the sibling
`mate-primitives-is-a-partial-mirror-with-no-growth-alarm`'s question
and not this row's.

Also not this row: `crates/viewer/README.md:1500` spells
`Dimension = Length | Angle | Count | Scalar` in prose. That is the
GQ5 design-question recap, where the variant identifiers are the
design vocabulary rather than words shown to a user, and no gate reads
it. A doc mirror is a different class from a code one.

## Why it is not fixed in the unit that found it

DOOR's posture is one PR is one row, with the mirror class's two rows
in one file as its single ruled exception (`work/door/plan.md`,
**Territory** and **Order**). `crates/pncad-py/src/*` is a third
crate's ground and the two `editor-core` suites are neither the file
the unit changed nor its own tests; widening into them would be a lane
minting a second exception for itself.

## What it is worth

Low, and it should be said plainly: every hit is test code, so the
consequence of a fifth dimension is a pin that silently narrows rather
than a user-visible row that silently shrinks. What makes it worth a
file rather than a sentence is that two of the five assert
exhaustiveness in their own prose, so the failure mode is a test whose
doc says "every dimension" while it covers four of five.

## Three more sites, and the blind spot that hid them (2026-09-11, the DOOR orchestrator)

Added by the review of PR #2391. **The sweep behind the list above was
shaped for `[…]` arrays**, the PR body disclosed that, and the file did
not — which matters, because `work/README.md` is explicit that the file
is what survives and a sentence in a merged PR body is not a schedule.
The disclosure belongs here:

**Blind spot: the sweep matched bracketed arrays only.** A complete
enumeration written as consecutive statements has no brackets and was
invisible to it. Re-run with a fourteen-line window over any spelling,
three more turn up, each a complete hand-written enumeration of all four
variants that asserts its own exhaustiveness in prose and would go
silently four-of-five on a fifth dimension:

- `crates/pncad-py/src/tests.rs:32-35` — `dimension_tags_are_stable`,
  four consecutive `assert_eq!`s over `dimension_tag`. **Thirteen lines
  above `:45`**, which the list above already names.
- `crates/pncad-py/src/tests.rs:62-65` —
  `canonical_units_match_the_gq5_ratification`, the same shape over
  `canonical_unit`.
- `crates/viewer/tests/panel_display.rs:876-881` — four hand-written
  `FieldWriting::of` calls under the comment *"Each dimension keeps its
  own tick"*. **This one is in `crates/viewer`**, the crate the closed
  row was about, so the row's own territory was not swept clean by the
  PR that closed it.

**And this file's own count is a floor.** The heading above says five;
with these it is eight, and the second sweep has a blind spot too — it
cannot see an enumeration spread across more than fourteen lines, or one
routed through a helper that takes a dimension and is called four times.
Whoever takes this row states the population its own instrument finds
rather than inheriting either number.

## Closed (2026-09-12, branch `door/dimension-all-readers`)

**Eight sites now read `Dimension::ALL`; two named above are left as
they are, with the argument below. The population is ten, not five and
not eight** — the count this row's own instrument finds, per the
instruction the addendum left.

### The premise, checked before anything changed

`Dimension::ALL` is public, is an inherent const on the enum (so it is
reachable from every crate that can name the type — `pncad-py` reaches
it through `pncad::document::Dimension`, a plain re-export), and is
**hand-written**. Its own rustdoc says so: *"A list cannot be derived
from a match in safe Rust, so SOMEONE writes it by hand; the only
question is where."* That does not make pointing sites at it worse,
because it is not a SECOND list — it is the first one, sited beside the
declaration in the crate whose exhaustive matches red on a fifth
variant, with `m4_pr1_dims::all_is_every_dimension` putting a
wildcard-free match right beside it.

**But it changes what conversion buys, and this row said it wrong.** A
converted site does not go red on a fifth dimension. It goes red on
nothing; it simply COVERS the fifth dimension, because the one list it
reads has grown. What forces a human to look is the census's compile
error, and that row forces the visit and not the edit
(`work/door/all-census-idiom-forces-the-visit-not-the-update`). So the
value here is: eight places that had to be remembered, now zero.

### Converted

- `crates/pncad-py/src/tests.rs`,
  `dimension_tags_match_the_kernel_prose` and
  `literal_refusals_come_from_the_kernel_with_stable_tags` — both were
  `for dim in [ …four… ]` and both became `for dim in Dimension::ALL`
  unchanged, exactly as this row said. Verified per site, not assumed.
- `crates/editor-core/tests/switch_display_units.rs`,
  `a_display_unit_is_accepted_exactly_on_its_own_dimension` — the
  `dims` local is gone; the cross-product and the refusal count both
  read `Dimension::ALL`. **The count assertion was the sharp end**: it
  was `UNITS.len() * (dims.len() - 1)`, an expectation derived from the
  very list whose completeness was in question, so it could not have
  noticed a short list. The doc's "6 rows × 4 dimensions" is now "every
  table row × every dimension".
- `crates/editor-core/tests/u8a_parse.rs`, both proptest generators —
  `proptest::sample::select(&Dimension::ALL[..])`, the spelling this
  row proposed; it compiles as written.
- `crates/viewer/tests/panel_display.rs`,
  `a_parameter_field_is_written_the_way_its_declaration_says` — the
  four `FieldWriting::of` calls under *"Each dimension keeps its own
  tick"*. Now the ticks of `Dimension::ALL` with every pair asserted
  distinct, which is also strictly more than the old three lines
  claimed (they never compared angle against scalar).

Two more the sweep found that this row did not have, both converted:

- `crates/editor-core/tests/display_contract.rs`,
  `a_dimension_reaches_refusal_prose_as_a_word_not_as_its_variant` —
  `let dumps = ["Length", "Angle", "Count", "Scalar"]`, the banned
  identifiers, now `Dimension::ALL.iter().map(|dim| format!("{dim:?}"))`.
- `crates/editor-core/tests/switch_display_units.rs`,
  `wire_door_refuses_a_tabled_unit_on_the_wrong_dimension` — the
  `!text.contains("Length") && …` chain, now an `all` over
  `Dimension::ALL`'s `Debug`.

**Both were invisible to both of this row's sweeps for the same reason,
and it is not the one the addendum names.** The blind spot recorded
above is span length ("bracketed arrays only", then "no more than
fourteen lines"). Span length hid nothing: the instrument used here
(smallest balanced `{}`/`[]`/`()` span mentioning all four variants,
over every tracked file, so an enumeration of any length or shape is
one cluster) finds no site that a wider window would have found. What
hid these two is **spelling**: the variants appear as string literals,
not as `Dimension::X` paths, so a sweep keyed on the path form misses
them however wide its window.

### Not converted, and why

- `crates/pncad-py/src/tests.rs`, `dimension_tags_are_stable` and
  `canonical_units_match_the_gq5_ratification` — the addendum's first
  two extra sites. **Churn.** Each is four flat value pins, not an
  enumeration with a uniform body: converting means writing a second
  exhaustive `match` in the test to supply the expected word, which is
  more code for an alarm the crate now has anyway — its converted
  sibling `dimension_tags_match_the_kernel_prose` iterates
  `Dimension::ALL` and pins every dimension's tag against the kernel's
  own `Display`, a fifth one included. **The addendum is also wrong
  about these two**: it says each *"asserts its own exhaustiveness in
  prose"*. `dimension_tags_are_stable` carries no doc at all, and
  `canonical_units_match_the_gq5_ratification`'s only comment is
  *"GQ5 / §L4: canonical metres and radians underneath"*. Neither
  claims to cover every dimension; they anchor four specific words, and
  a fifth dimension's canonical unit is not a fact GQ5 ratified.
- `crates/editor-core/tests/u8a_parse.rs`'s `rt_params` — as this row
  says, nothing to project from. Unchanged.
- `crates/pncad-py/src/py/quantity.rs`'s `dimension_of` — the same
  shape as `rt_params` (Python classes, not kernel variants) but in
  production code, so it is filed rather than left in prose:
  `work/lib/python-quantity-classes-map-to-dimension-by-hand-with-no-growth-alarm`.
- The prose mirrors — `crates/viewer/README.md`'s GQ5 recap (this row
  already excluded it), `crates/pncad-py/pncad.pyi`'s two docstrings,
  `py::value::Measurement`'s field doc, `test_binding_census.py`'s
  comment. A doc mirror is a different class, as this row says.
- `crates/editor-core/tests/display_contract.rs`'s SEVEN OTHER `dumps`
  lists — the same defect one class up, over `InterrogateError`,
  `ParseError`, `SelectRefusal`, `NodePickError`,
  `ResolveIndeterminate`, `ResolveFault` and `DeclareError`, none of
  which publishes an `ALL`. Not a substitution, so filed:
  `work/tint/assert-f6-dump-lists-are-hand-written-mirrors-of-error-enums`.

### The sweep, and what it could not match

Instrument: every tracked `.rs`/`.py`/`.pyi`/`.md`/`.sh`/`.toml`/
`.json` file; for Rust, the smallest balanced-delimiter span mentioning
all four variant identifiers, then triaged by hand; for the rest, a
40-line window over the identifiers or the lowercase words. Ten
projectable sites in five files, all under `crates/`. `demos/`,
`tools/`, `benches/`, `interval-transcendentals/` and the other roots
`scripts/doc-gate.sh --print-roots` lists were checked explicitly and
hold no enumeration at all — every `Dimension::` use out there is a
single-variant literal in a tour model.

**What it cannot match**: a dimension enumerated through four calls to
a helper that spells no variant name (the helper's own body would be
one cluster, its four call sites none); an enumeration in a file whose
extension is outside the list above; and a mirror in generated or
vendored text, which `git ls-files` does not carry.

### Red-first evidence

A fifth variant `Dimension::Mass` was added to the enum and to
`Dimension::ALL` (the correct author's edit — the census forces the
visit that prompts it), and the arms the compiler demanded were filled
in. Probe reverted; `git diff` is clean of it.

**What went red, unconverted**: `all_is_every_dimension`'s match,
`u8a_parse::arb_text_of`'s match, and two digest matches in the M10
corpora — all four inside `editor-core`'s own test tree, all of them
matches. **What did not**: every one of the ten sites above. They
compiled and PASSED, covering four dimensions of five, which is what
this row claims and it is true.

Two mutations then separated converted from unconverted on one tree:

- `Expr::literal_with_unit` made to accept any tabled unit on a `Mass`
  literal. `a_display_unit_is_accepted_exactly_on_its_own_dimension`
  reading `Dimension::ALL` FAILS — *"mm was accepted on a Mass
  literal"*. The same test with its `dims` array restored, same
  binary otherwise, PASSES.
- `pncad-py`'s `dimension_tag` given `Mass => "Mass"` — the capitalised
  slip the kernel's prose rule exists to prevent.
  `dimension_tags_match_the_kernel_prose` reading `Dimension::ALL`
  FAILS — *left `"Mass"`, right `"mass"`*. With its four-entry array
  restored it PASSES, and so does `dimension_tags_are_stable`, which
  is the evidence for leaving that one alone: the converted sibling is
  what fences the fifth dimension's tag.

`u8a_parse::grammar_generated_text_parses_to_the_expected_dimension`
needed no mutation: converted, it reds on the probe alone, because the
property now generates `Mass` source and finds it parses as `Scalar`.

### Territory

Every file is another program's: `crates/pncad-py/src/tests.rs` is
LIB's; `crates/editor-core/tests/*` is S-TCOST's and S-TINT's;
`crates/viewer/tests/panel_display.rs` is CHROME's, VIEW's, S-TCOST's
and S-TINT's. Announced in the PR, per this program's `keep_out`.
