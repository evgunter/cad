---
id: dimension-error-op-carries-twelve-words-minted-at-call-sites
kind: issue
title: DimensionError.op carries twelve words minted as literals at call sites of two &'static str door parameters, in two vocabularies the stub documents as one
status: open
opened: 2026-09-15
priority: P3
cost: E
---



Found by CENSUS-PY-RAISE-LITERALS' sweep (2026-09-15), which closed
the nine words that row named and measured this population on the way
past. Not fixed there: it is a different attribute on a different
class, three doors, and three times the words.

## The population

`DimensionError.op` is Python-visible (`pncad.pyi` declares `op: str`)
and reaches a caller from a `&'static str` PARAMETER, spelled as a
literal at each call site. Two parameters, three doors, twelve words:

| words | parameter | call sites |
| --- | --- | --- |
| `+`, `-`, `*`, `/`, `<=>` | `mismatch`'s `op` (`py/quantity.rs`) | five, inside the operator macro |
| `Distribution.band`, `Distribution.uniform`, `Distribution.truncated_normal`, `AnalyzedBox.box_mass` | `agreed`'s and `dimension_mismatch`'s `door` (`py/analysis.rs`) | four `agreed` calls plus one direct `dimension_mismatch` |
| `DocParam.length`, `DocParam.angle`, `DocParam.scalar` | `continuous`'s `door` (`py/doc.rs`), forwarded to `dimension_mismatch` | three |

**One attribute, two vocabularies.** Five are OPERATOR SYMBOLS and
seven are PYTHON DOOR NAMES, and the stub documents only the first
kind: *"the operator that was attempted"*. A caller branching on `op`
meets `"+"` from the quantity boundary and `"DocParam.length"` from a
parameter constructor, with nothing saying the second shape exists.

**Covered by accident, six of twelve.** Asserted on `.op` somewhere in
the Python suite: `+`, `*`, `<=>`, `Distribution.band`,
`AnalyzedBox.box_mass`, `DocParam.length`. Asserted nowhere: `-`, `/`,
`Distribution.uniform`, `Distribution.truncated_normal`,
`DocParam.angle`, `DocParam.scalar` — the sibling of an asserted word
in every case, which is what "by accident" means here. No Rust test
names any of the twelve, and `TAG_INVENTORY` lexes `src/tags.rs`
alone, so nothing reds if a word is renamed, and nothing reds if a
thirteenth is added.

## The same shape one crate out

`MeasureUnavailableAt.door` reaches Python the same way from
`editor-core`: `eval/wire.rs` mints `door: "clearance::min_separation"`
as a struct-field literal on `MeasureUnavailableAt::NeedsEnclosure`,
and `py/measure.rs` hands it to the attribute unchanged. Its two
siblings on that arm do NOT have the defect — `verb` comes from
`MeasurePrimitive::verb` and `scalar` from `Lane::NAME` — so the arm is
one literal among two maps. `tests/test_measures.py` asserts the word
on `.door`, so it is covered by accident like the rest — the point is
that nothing reds if it is renamed or a second one is added. WIRE's
ground, filed here because the class is this program's.

## Shape of the fix, if it is taken

The disposition CENSUS-PY-RAISE-LITERALS proved for `ValidationError`
transfers, and this is the better subject for it: the seven door names
are wholly the binding's own vocabulary, with no kernel enum behind
them, which is the condition under which
`errors::ErrorClass::Evaluation`'s shape applies — the class carries an
enum, `crate::tags` holds the exhaustive map, and a raise cannot be
written without naming a variant. The five operator symbols are the
harder half: they are minted inside a macro and are not identifiers,
so whether they want the same enum or a separate one is the question
this row holds open.

Territory: `crates/pncad-py/*` is LIB's fence and `crates/editor-core/*`
WIRE's; this program's `keep_out` announces its pncad-py rows there.
