---
id: LIB-B-NOTATION
kind: unit
title: binding census family B-NOTATION
status: review
branch: lib/b-notation
opened: 2026-09-06
pr: 2189
---

Queued mechanical census family (the B-RESOLVE shape): sweep the
family's bindings against the census contract, construct the
previously unconstructible pins where the surface now allows, and
re-cut the census rows honestly. Families share the census/tags/test
files, so at most two run concurrently, staggered.

## Derived scope (stated before any code changed)

### The measurement, first, with the bytes

`crates/pncad-py/tests/test_binding_census.py` charters `B-NOTATION`
in `FAMILIES` (`:738`) and TWO `NOT_BOUND` entries cite it —
`WrittenAngle` and `WrittenLength` (`:1795`/`:1796`). Both are curated
through `crates/pncad/src/prelude.rs:154`.

Authored from Python today, through the wheel built at this branch's
merge base:

```python
d.apply(DocEdit.set_doc_param(ParamName("width"),
                              DocParam.length(25 * mm)))
```

`repr` answers `DocParam(Length 0.025 m)` and `Doc.save()` writes

```json
"width": {"Continuous": {"dim": "Length", "value": 0.025,
                         "display_unit": "m"}}
```

The `mm` is gone at the `Length` door and the document records the
canonical row — the charter's claim, executed. `DocParam.angle(90 *
deg)` is the same with `rad`.

### Why the existing quantity spelling CANNOT carry the unit

The orchestrator's read offered two branches, and the code answers the
second. `crates/pncad-py/src/py/quantity.rs:349` is
`LengthUnit::__rmul__ -> Length(value * self.0)`: a Python `Length`
wraps `quantity::Length`, which is canonical metres and nothing else,
so `25 * mm` erases at construction exactly as `25.0 * MM` does in
Rust.

It cannot be made to carry one either, and the reason is not
implementation cost. `crates/quantity/src/written.rs`'s module docs
rule the point directly: there is **deliberately no arithmetic** on
`WrittenLength`/`WrittenAngle`, because "there is no answer to what
notation the sum of a millimetre and an inch is written in, and a type
that silently picked one would be inventing an authored fact nobody
authored". Python's `Length` HAS that arithmetic (`__add__`,
`__sub__`, `__neg__`, `__mul__`, `__truediv__`, all of
`py/quantity.rs`'s `continuous_quantity!`), and it is the type every
node constructor takes. Teaching it a unit would either invent the
notation of a sum or introduce the "canonical, notation unknown" state
the same module docs say does not exist. Equality would fork too:
`WrittenLength` compares BOTH halves, `Length` compares the canonical
number.

So the notation crosses as the second type, which is what the census's
own two rows already name. That is not a second quantity spelling
invented here — it is the curated Rust pair, bound at its own name.

### Bound at these spellings

- `WrittenLength` / `WrittenAngle`, top-level, spelled identically:
  static `in_unit(value, unit)` (the multiply, `written.rs`'s
  "library authoring spelling") and `canonical_in(value, unit)`
  (already-canonical plus the notation), properties `length`/`meters`/
  `unit` and `angle`/`radians`/`unit`.
  `canonical_in` takes a `Length`/`Angle` where Rust takes an `f64`,
  for the reason `py/quantity.rs`'s module docs give for `format`'s
  receiver: Rust reaches this module from below with the newtype
  already unwrapped, and what a Python caller holds is the quantity.
  `from_meters`/`from_radians` are NOT bound: they are
  `in_unit(x, m)` without the multiply by one, and their reason to
  exist in Rust is `const` context, which Python has no analogue of.
  The `in_unit` name collides with `Length.in_unit` in the opposite
  direction; the collision is the kernel's own and deliberate there
  ("the inverse of `Length::in_unit`"), so it is inherited rather than
  renamed.
- `DocParam.written_length(w)` / `DocParam.written_angle(w)` —
  `d::DocParam::written_length` / `written_angle`
  (`crates/editor-core/src/doc.rs:152`/`:164`), total, no dimension
  argument.
- `DocParam.unit -> str | None` — the read of the authored notation,
  as the SYMBOL. That spelling is not a choice made here: the census
  files `UnitSym` as `different-shape` with the reason "a notation
  reaches Python as its SYMBOL" (`:1412`), and this door is what that
  sentence has been promising. `None` for a `Count` (the arm carries
  no notation at all); `""` for the dimensionless row, which is the
  absence `DocParam.__repr__` already prints.
- `Doc.params -> dict[ParamName, DocParam]` — `Doc::params()`
  (`crates/editor-core/src/doc.rs:463`). Python had NO read door for a
  document parameter at all; see the blind spot below.
- `LengthUnit.__eq__`/`__hash__` and `AngleUnit.__eq__`/`__hash__`,
  on the table index Rust's derived `PartialEq` uses. Required, not
  ornamental: without them `WrittenLength.unit == mm` is FALSE for
  every value that did not come from the module constant object
  itself, because the classes carry no `__richcmp__` and Python fell
  back to identity.

### Not bound, and why

- **`Expr::written_length` / `written_angle` need no binding: the text
  door already reaches them.** `Doc.parse_expr("25 mm")` runs the
  checking parser, which multiplies once and calls
  `Expr::literal_with_unit`; measured on the merge base,
  `parse_expr(s).text` reads back `"25 mm"`, `"2 m"`, `"90 deg"` and
  `"0.5 pi rad"` verbatim. `py/expr.rs`'s module docs already rule
  that the individual builders stay out — "they would be a second
  spelling of one grammar" — and binding a written-literal
  constructor would be exactly that, for the one grammar row that
  already round-trips.
- **`DocParamValue` gets no written door, deliberately.** The display
  unit rides with the DECLARATION (`doc.rs:44-70`), which is why
  `SetDocParamValue` leaves it alone; a written value door would be
  offering to re-notate through the one edit whose entire contract is
  that it does not touch the declaration.
- **The mis-dimensioned written value is UNREPRESENTABLE at the
  authoring door, not refused** — `written.rs`'s "#650 ruling applied
  one layer out". A `WrittenLength` holds a `LengthUnit`, so
  `written_length` cannot mint a mismatch and `DisplayUnitMismatch` is
  unreachable through it. The refusal that IS reachable from Python is
  the document invariant at the persist walk, and it already works:
  `load` of a hand-edited file pairing `dim: Angle` with
  `display_unit: "mm"` raises `PersistError` with variant
  `display_unit`, and an off-table symbol raises variant `unreadable`
  at the token. Both are pinned as rows rather than assumed.
- **`UnitSym` stays `different-shape`.** Not re-opened; `DocParam.unit`
  is that row's own sentence being kept.

### What the census cannot see, and could not have reported

Both rows this family owns are TOP-LEVEL names, so unlike B-PART and
B-FACE-FRAME the roster is honest about its own two entries. The blind
spots are on everything the family needed BESIDES them, and they are
the same rule-1 blind spot one and two levels in:

- **`Doc.params` is a METHOD of a curated type.** `Doc` is curated and
  `pncad.pyi` declares a top-level `Doc`, so rule 1 accounts it WHOLE.
  Python had no way to read a document parameter back — not the map,
  not one by name — and nothing here would ever have said so. Without
  it the notation crosses in and is observable only by saving to text
  and parsing the JSON by hand, which is not a door. That is B-PART's
  "the entries an id owns are not always the entries a unit must
  move", arriving on the READ side.
- **`LengthUnit`/`AngleUnit` comparing by identity is a MISSING
  DUNDER**, one level further in than a method: `test_stubs.py`'s
  operator half checks only stub-declared operators against the class,
  so an operator neither side declares is invisible to both rosters.
- **Fifty-two node-slot literals record the canonical row**, and every
  one of them is behind `Node`, `PatternKind` or `GeomPred` — names
  rule 1 accounts whole. Out of scope here (the charter is
  parameter-scoped and these are not `DocParam` doors), banked as
  `work/lib/node-slot-literals-erase-the-authored-notation.md`.

### Decisions honored, not relitigated

`crates/pncad/tests/all.rs`'s `NOT_CARRIED` says nothing about this
family — `WrittenLength`/`WrittenAngle` are CARRIED, through
`prelude.rs:154`. `UnitSym` stays `SHAPE`. `Length`/`Angle` keep their
canonical-only payload and their existing equality and hashing, the
signed-zero and poison questions in
`work/lib/the-quantity-boundary-compares-and-hashes-as-if-poison-and-signed-zero-cannot-arrive.md`
included: `WrittenLength.__eq__`/`__hash__` follow `DocParam`'s
already-settled shape (IEEE equality, `-0.0` folded before hashing)
rather than `Length`'s unsettled one.

### Census delta predicted

`WrittenLength` and `WrittenAngle` leave `NOT_BOUND` **entirely**
under rule 1 — `pncad.pyi` will declare both at the same spelling —
rather than moving to `BOUND_AS`; the `B-NOTATION` charter leaves
`FAMILIES` with them, and the closure paragraph records the three
blind spots above. Both decay guards stay green in both directions.

## Outcome

The family closed, and it took SIX doors where the charter named two
names — the two it named are exactly the two the roster held, and
everything else was surface neither roster could see:

- `WrittenLength` / `WrittenAngle`, spelled identically, with
  `in_unit` and `canonical_in` as the two constructors, the erasure
  door (`length` / `angle`), the canonical number and the notation.
  No arithmetic, mirroring the Rust types and for their reason.
- `DocParam.written_length(w)` / `DocParam.written_angle(w)`.
- `DocParam.unit -> str | None` — the notation as its SYMBOL, which
  is the sentence `UnitSym`'s `different-shape` row had been
  promising; `None` only for a `Count`, `""` for the dimensionless
  row.
- `Doc.params -> dict[ParamName, DocParam]` — NOT in the charter, and
  required by it. Python had no read door for a document parameter at
  all, so a document could remember `mm` and no caller could ask;
  binding the authoring half alone would have shipped a memory
  observable only by saving to text and parsing the JSON by hand.
- `LengthUnit.__eq__` / `__hash__` and `AngleUnit.__eq__` / `__hash__`
  — also not in the charter, also required by it. The unit classes
  carried no comparison, so `mm == mm` held only by identity and a
  unit READ BACK off a value compared unequal to the constant it was
  written in. A read door for a notation is unusable without them.

The measurement, before and after, on the same authoring: the saved
row for `width` read `"display_unit": "m"` and now reads `"mm"`.
`DocParam.length` is untouched and still records the canonical row —
a caller with a number and no notation says so rather than having one
guessed, which `test_the_erasing_door_still_erases` pins.

`crates/pncad-py/tests/test_notation.py` is the positive form: 30
tests over the saved BYTES wherever the claim is about what a document
records, the two-halves equality and its `-0.0` fold, the value door
leaving the notation alone against the create-or-replace door
restating it, the text door already carrying it, and the
mis-dimensioned row refusing at LOAD in two different ways
(`display_unit` at the walk, `unreadable` at the token). Five rows in
each ty fixture, where the unrepresentable pairing is the static half
of the claim the runtime has no refusal for.

Census delta, exactly as predicted: both entries leave `NOT_BOUND`
entirely under rule 1, the `B-NOTATION` charter leaves `FAMILIES`, and
the closure paragraph records the three blind spots — a METHOD behind
a curated type, a missing DUNDER invisible to both rosters, and the
52 node-slot literal sites.

One finding banked rather than taken:
`work/lib/node-slot-literals-erase-the-authored-notation.md` — every
Python door that takes a `Length` into a node SLOT records the
canonical row, 52 sites in 18 doors, which needs a written seat at
each and is a decision about the whole authoring lattice rather than a
census row.

Deviations from the brief, both argued above rather than taken:
`Expr::written_length` / `written_angle` are not bound (the text door
already reaches them, and `py/expr.rs` already rules the individual
builders out), and `DocParamValue` gets no written door (the notation
rides with the declaration, which is the value door's entire
contract).
