---
id: boolean-kind-not-published-at-the-python-door
kind: issue
title: the Python checks door returns a separation refusal as prose only — no boolean_error_tag beside path_error_tag, so the FFI consumer substring-matches or nothing
status: open
opened: 2026-09-04
---


Found by the style review of PR 1806 (`boolean-error-has-no-fieldless-kind`),
which added `topo::BooleanErrorKind` and carried it at
`editor-core`'s checks door. The Python door one step further out was
not carried and is now the last place where the defect that item
describes is still true.

**The defect.** `crates/pncad-py/src/py/checks.rs:344` — `reason()`
returns the rendered prose and nothing else; the `kind` the evidence
now carries is discarded by the `..` in its pattern.
`crates/pncad-py/src/tags.rs:998` — `check_evidence_tag` returns
`"separation_unavailable"`, which names the EVIDENCE ARM, not the
boolean class inside it. So a Python consumer that wants to know
*which* kernel refusal made separation unavailable has the same two
options the item calls the defect: substring-match the sentence, or do
without.

**The fix shape is already written, 900 lines up in the same file.**
`tags.rs:88`'s `path_error_tag` is an exhaustive match over
`PathErrorKind` with zero `_` arms. `boolean_error_tag` beside it,
over `BooleanErrorKind`'s 41 variants, plus an accessor on the
evidence object, is the whole change.

**Why PR 1806 did not carry it.** Two reasons, both worth re-deciding
rather than inheriting. First, `crates/pncad-py/*` is LIB's territory
and that PR was already across three fences for one-line threads;
this is a new published surface, not a thread. Second, 41 FFI names is
a vocabulary decision — `PathErrorKind`'s own doc warns that a tag
minted for a phantom publishes a name no refusal can carry — and the
kinds most worth naming to Python may be a curated subset rather than
all 41. Neither reason survives a caller actually wanting the class.

Note the asymmetry this leaves: `path_error_tag` publishes its kernel
class to Python and the boolean one does not, so the two error
families read differently at the same door.
