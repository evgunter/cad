---
id: evaluationerror-stub-lists-five-reasons-and-the-door-raises-six
kind: issue
title: pncad.pyi's EvaluationError docstring lists five reasons; the door raises six
status: open
opened: 2026-09-15
priority: P3
cost: E
---



Found by CENSUS-TAG-REACH (2026-09-15), which gave the evaluation
door's reason vocabulary a machine census and left this hand-written
copy of it unchanged.

## The two lists

`crates/pncad-py/pncad.pyi`, `class EvaluationError`, opens:

> `reason` is `unknown_node`, `wrong_kind`, `empty_boolean`,
> `node_failed`, or `poisoned`.

That reads as the whole vocabulary — five words, closed by "or". The
door raises **six**: `crate::errors::EvalReason` also carries
`NodeNotEvaluated`, whose word `node_not_evaluated` is what
`Evaluation.value` answers for a node a canceled run never reached.
The stub documents that raise in a different place
(`Evaluation.value`'s own docstring says so, and
`crates/pncad-py/tests/test_cancellation.py` asserts
`reason == "node_not_evaluated"` at three sites), so the word is not
undocumented — the CLASS's list of it is short.

## Why it is a row and not a one-line edit in that unit

The list is a hand-written census of a population that now has a
machine one (`crate::tags::eval_reason_tag` over `EvalReason`, pinned
by `TAG_INVENTORY`). Editing the sentence fixes today's count and
leaves the next `EvalReason` arm to be copied across by hand again.
What the row is worth is the general question: **which stub
docstrings restate a vocabulary the Rust side now enumerates, and can
the restatement be derived or checked rather than typed?**
`crates/pncad-py/src/surface_census.rs` is the instrument that already
reads the stub, so there is somewhere for such a check to live —
which is one end of
`work/census/four-censuses-of-python-visible-vocabulary-in-one-crate.md`,
the question of whether this crate's four vocabulary censuses should
be four.

CENSUS-TAG-REACH's spec forbade it from touching `pncad.pyi`'s
surface, which is why the sentence is still five words long.

## The Rust-side half of that general question (CENSUS-PY-GETTERS, 2026-09-15)

The question this row asks of `pncad.pyi`'s docstrings —
which restate a vocabulary the Rust side now enumerates — has a
measured answer on the Rust side, filed as
`work/census/tag-vocabularies-restated-in-py-doc-comments.md`: eight
getters under `src/py/` whose own doc comments restate their map's
words. Those doc comments are Python docstrings too, since pyo3 puts
them on the getter.

One stub instance beside this row's, found by that unit and not
measured further: `pncad.pyi`'s `Subgroup.variant` docstring
hand-lists all seven subgroup words, which
`crate::tags::subgroup_tag` now holds and `TAG_INVENTORY` pins.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.
