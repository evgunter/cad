---
id: py-discriminant-getters-under-src-py-are-outside-every-inventory
kind: unit
title: 27 Python-visible discriminant words are minted by getters under src/py/, outside TAG_INVENTORY
status: closed
opened: 2026-09-15
branch: census/py-getters
closed: 2026-09-15
---


Found by the style review of CENSUS-TAG-REACH (2026-09-15) and
re-measured against the tree by that unit's fix pass. It is the same
class as `py-reason-and-variant-literals-outside-any-enum` and a
different SHAPE, which is why that row's sweep — a literal beside a
`"reason"` or `"variant"` key — found none of it.

## The population, measured

`crates/pncad-py/src/py/` holds **30** functions returning
`&'static str`. **Nine of them mint their words as string literals**
rather than delegating to `crate::tags` — the fix pass re-measured
this and the row said seven. **Seven of the nine reach a Python
caller** as a discriminant, and between them they spell **27 distinct
words**. Six of the seven, and 23 of the 27, are the lowercase table
below:

| site | words |
| --- | --- |
| `py/mate.rs`, `Subgroup::variant` | 7 — `se3`, `planar`, `cylindrical`, `prismatic`, `revolute`, `trivial`, `empty` |
| `py/mate.rs`, `primitive_tag` | 4 — `coaxial`, `planar_rest`, `clocking`, `frame_coincidence` |
| `py/mate.rs`, the gauge-edit `variant` | 4 — `join`, `split`, `drop`, `gauge_rewrite` |
| `py/mate.rs`, `ClassAdmission::variant` | 3 — `mints`, `no_at_rest_record`, `not_admitted` |
| `py/assembly.rs`, `entity_kind_tag` | 4 — `face`, `edge`, `vertex`, `body` |
| `py/refactor.rs`, the refactor `variant` | 1 — `mate` |

**The seventh Python-visible map the row missed** is
`py/value.rs`'s `dimension_name`: four words — `Length`, `Angle`,
`Count`, `Scalar` — reaching Python as `Measurement.dimension`,
exhaustive over `d::Dimension`, and asserted by four sites in the
Python suite. It is **capitalized**, which is why it did not move to
`src/tags.rs` with the other six: that file's reader refuses a tag
value that is not lower snake case, and the claim is the file's, not
the reader's convenience. Its home is `src/errors.rs` beside
`dimension_tag`, the lower-case spelling of the same four dimensions,
where a derived pin now holds the two alphabets to one list.

The eighth is `py/path.rs`'s `StartToken::__repr__`, which mints
`Start`: a repr, not a discriminant a caller branches on, and out of
scope for that reason rather than by not being found.

The ninth, `py/doc.rs`'s `_binds_every_kernel_window`, is **not** in
that count and its two words (`arc`, `full`) are not Python-visible:
the function is a never-called compile-time tripwire over
`d::TubeWindow`, as its own doc says. The style review that opened
this row counted it, which is a two-word overstatement corrected here
rather than repeated.

Every one of the 23 is named in `crates/pncad-py/pncad.pyi`, so every
one is public Python vocabulary a caller branches on. **That does not
extend to the 27**: `Measurement.dimension`'s four are named nowhere
in the stub, which declares the attribute `-> str` and lists no word.

**Two of the six
functions are literally named `*_tag` and live outside
`src/tags.rs`** — `entity_kind_tag` and `primitive_tag` — which is the
whole of the naming convention that is supposed to say "this word is
inventoried".

**Sixteen of the 23 appear nowhere in `src/tags.rs`.** The other
seven are the sharper half: `face`, `edge`, `vertex`, `empty`, `join`,
`split` and `no_at_rest_record` are ALSO minted in `src/tags.rs`, by a
different map, for a neighbouring concept — so they are a second
spelling of one word rather than a word outside every list.
`py/assembly.rs`'s `entity_kind_tag` and `crate::tags::entity_id_tag`
agree on `face`/`edge`/`vertex` by hand today, over two different
kernel enums; `ClassAdmission::NoAtRestRecord` and
`MintRefusal::NoAtRestRecord` do the same for
`no_at_rest_record`. Nothing holds either pair equal.

## Two shapes beside it, one of them a clean negative

* **The `Option<&'static str>` getters are clean.** The same sweep
  over the 17 `fn … -> Option<&'static str>` under `src/py/` (a
  population the `-> &'static str` pattern does not match) finds
  **zero** direct literals: every one delegates to `crate::tags`.
  Executed, not reasoned.
* **A word minted in tuple position is invisible to both sweeps.**
  `py/mesh.rs`'s `stl_err` builds `(variant, message, …)` per arm and
  takes six of its seven variants from `crate::tags`; the seventh,
  `NotUtf8`, spells `"not_utf8"` in the tuple. That one is a raise-site
  literal and belongs to the sibling row, which now carries it.

## Why no census sees any of this

`TAG_INVENTORY` (`crates/pncad-py/src/tests.rs`) lexes `src/tags.rs`
as text: a word spelled anywhere else is not in its population.
`NODE_KIND_ROSTER` beside it covers `crate::node_kind` alone.
`src/surface_census.rs` matches kernel tags against `pncad.pyi` — its
device (*"the witness is a MATCH on the kernel tag, not a list"*) is
the closest fit of the four instruments and reaches none of these
because they are not kernel tags. So a rename of `se3` to `se_3` is
green everywhere in Rust and breaks every Python caller.

## Shape of the fix, if it is taken

Not obviously "move them to `src/tags.rs`": several are getters on a
`#[pyclass]` whose `match` is already exhaustive over a kernel enum,
which is the existence half; what is missing is a guard over the
VALUES. Either they become `crate::tags` maps (and the inventory
covers them for free), or the inventory's population stops being "one
file" and becomes "every exhaustive `&'static str` map in the crate" —
which is a reader over `src/py/`, and a reader needs a guard of its
own, this program's standing trap. The choice is the row.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.

## Verified at spec time, and sharpened (orchestrator, 2026-09-15)

The counts above were re-measured independently and **hold** — 30
functions, six Python-visible minting six maps, 23 words, the `doc.rs`
tripwire correctly excluded. First row in this program whose numbers
survived that check.

**The sharpening the row does not make:** all six functions are
exhaustive matches over a kernel enum with **zero wildcard arms**, so
`E0004` already holds each map to its kernel type — a new variant stops
the build today. What they are outside is the INVENTORY, which pins a
word's TEXT. So the live defect is a **rename**, not an addition:
change `revolute` to `hinge` and every test passes while public Python
vocabulary moves.

That narrows the mechanical half to a **relocation** — a map in
`tags.rs` is lexed, pinned in `TAG_INVENTORY` and reds on an addition or
a rename — which CENSUS-TAG-REACH proved one door over. The design half
is the seven second spellings, and that call is the lane's, per pair,
because the pairs are not alike.

Also for the sweep: `mate.rs`'s subgroup getter hand-lists its four
words in prose four lines above the delegation to the map that holds
them, with nothing tying the two.

`docs/CENSUS-PY-GETTERS-SPEC.md` binds the unit.

## What the fix pass did (2026-09-15)

Six maps moved into `src/tags.rs` as `entity_kind_tag`,
`mate_primitive_tag`, `class_admission_tag`, `subgroup_tag`,
`cluster_maintenance_tag` and `interface_crossing_tag`; the seventh
(`dimension_name`) moved to `src/errors.rs` as
`measurement_dimension_tag`, for the case reason above. No word
changed value; `pncad.pyi` is untouched and the 832-test Python suite
is green.

The seven second spellings were dispositioned **four words pinned and
three scoped**, which is two map-pairs pinned and three called
coincidence. Counted either way it is not "five and two", and the
count is written out here because the earlier sentence at this spot
said that and the PR table said otherwise:

| words | pair | verdict |
| --- | --- | --- |
| `face`, `edge`, `vertex` | `entity_kind_tag` / `entity_id_tag` | **pinned** |
| `no_at_rest_record` | `class_admission_tag` / `mint_refusal_tag` | **pinned** |
| `empty` | `subgroup_tag` / `band_error_tag` | scoped |
| `join` | `cluster_maintenance_tag` / `boolean_error_tag`, `split_op_error_tag` | scoped |
| `split` | `cluster_maintenance_tag` / `node_error_tag` | scoped |

`src/tags.rs`'s header states the rule the file had been following
silently — a tag word is scoped to the map that mints it — and the two
pins are by construction in `src/tests.rs`:
`the_entity_kind_and_entity_id_maps_agree_where_both_speak` and
`the_class_table_predicts_the_mint_refusal_in_its_own_words`.

## What the style review's fix pass corrected (2026-09-15)

### The blind-spot list was reasoned, not executed, and one entry was false

The unit's rename probe was published with five blind spots written
before it ran, and named two of them the real risks here — a map
*moved wholesale out of `tags.rs`* (*"the reader stops reading it and
reports agreement over what is left"*) and a word minted in a file the
reader does not read. The first is **false, and was falsifiable in one
run**. `src/tests.rs`'s inventory guard has an explicit GONE branch
over `TAG_INVENTORY`: for every pinned function the reader no longer
finds, it complains. Executed here — `interface_crossing_tag` deleted
from `tags.rs` — it says:

> tag function `interface_crossing_tag` is GONE from src/tags.rs — the
> words ["mate"] no longer reach Python from it

The real blind spot is narrower: **a map that was never inventoried**,
which is the pre-state this unit fixed, and a map in a file the reader
does not read, which is the second risk it named and which stands.

**The lesson is the one the program already recorded and this unit did
not take.** Standing finding 6 asks for a blind-spot list arrived at
by EXECUTION rather than by reasoning. This list was reasoned; four
entries happened to be right and the fifth carried an argument ("this
PR is that operation, in the safe direction") that a one-line probe
would have dissolved. A blind spot worth stating is worth running.

### The entity-kind vocabulary has a third Python-visible spelling

`py/select.rs`'s `EntityKind` is a fieldless `#[pyclass]` mirror whose
four member names reach Python as `pncad.EntityKind.Face` and so on —
the same vocabulary `entity_kind_tag` and `entity_id_tag` spell, over
the same kernel enum, doc'd with the same sentence. `tags.rs`'s header
claimed the pinned set was complete; it now says which set it is.

**That third spelling turned out to be GUARDED, by something neither
sweep looked at.** `tests/test_stubs.py`'s depth-2 walk compares every
stub class's attributes against the compiled class name-for-name, in
both directions, and `pncad.pyi` declares each enum member as a
`Final` class constant. Executed: renaming `ArcSweep::Ccw` to
`Anticlockwise` in `py/path.rs` reds
`test_stubs.TestStubClassDrift.test_class_attributes_agree_name_for_name`
plus 16 call sites with `AttributeError: type object 'pncad.ArcSweep'
has no attribute 'Ccw'`. So the crate's **24** `#[pyclass]` enums and
their 114 member names are held to the stub exactly as `tags.rs`'s
words are held to `TAG_INVENTORY` — a deliberate two-place diff, not a
silent rename. No row was filed for them, because there is no gap.

### The class table predicts ONE ARM of the mint refusal

`the_class_table_predicts_the_mint_refusal_in_its_own_words` and its
doc claimed the class table predicts the mint door's refusal. The mint
door (`editor_core::assembly::mint`) refuses through a wildcard
`other =>` arm, so `ClassAdmission::NotAdmitted` also reaches a caller
as `MintRefusal::NoAtRestRecord`. Executed by narrowing that arm to
`NoAtRestRecord` alone: `error[E0004]: non-exhaustive patterns:
ClassAdmission::NotAdmitted not covered`. The pin stands — the two
`no_at_rest_record` spellings must agree — and the doc above it now
says the tool-matching claim is right on that arm, silent on `mints`
and wrong on `not_admitted`.

### The relocation's own risk, said honestly

14 of the 27 moved words had **no pin of any kind at the moment they
moved** — `face`, `clocking`, `coaxial`, `frame_coincidence`,
`planar_rest`, `not_admitted`, `cylindrical`, `prismatic`, `revolute`,
`se3`, `trivial`, `gauge_rewrite`, `Count`, `Scalar`: no occurrence in
`tests/` or `examples/`, and in the crate nothing that a wrong
spelling would have failed — the literal being moved, plus a prose
restatement in the same doc comment for three of them. (`face` does
occur elsewhere in the crate, minted by `shell_error_tag` and
`entity_id_tag` and pinned there; neither pin can see a typo in
`entity_kind_tag`'s copy, which is the point.) And `TAG_INVENTORY`'s new rows were taken
**from the guard's own output on the post-move source**, so the guard
was populated FROM the relocated text rather than against it. A typo
introduced by the move would have been inventoried as correct and
every row would have been green.

Nothing is wrong — the review diffed old and new bodies and every word
arrived intact. The point is that **nothing would have said so**. What
a relocation of this shape owes is an independent re-derivation from
the PRE-move source: lex the words out of the old file at the merge
base, and compare that set against the inventory rows the guard
proposes. That is the check this unit did not run and the next
relocation should.

### The scoping rule claimed more than was read

`tags.rs`'s header asserted *"a tag word is scoped to the map that
mints it"* over the whole file, having examined seven words. **61
words in `tags.rs` are minted by two or more maps** — `band` in 16
maps, `escalated` in 10 — measured twice, once by a per-function sweep
of the source and once over `TAG_INVENTORY`'s 98 rows, agreeing. The
header now scopes its claim, and
`every_word_two_tag_maps_share_is_on_the_committed_roster` holds the
population to a roster derived from the inventory so a new cross-map
collision reds and names itself.
`sixty-one-tag-words-are-minted-by-two-or-more-maps-and-seven-are-read`
carries the 54 nobody has read.
