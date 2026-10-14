---
id: binding-drops-a-standings-through-at-three-refusals
kind: issue
title: SelectRefusal and ChecksError cross a node's standing into Python without through as a value
status: open
opened: 2026-09-29
priority: P4
cost: E
---

Filed by EDIT's C6 "no usable value" member (PR 3463, fix pass), on
the review's MINOR-4. The kernel's refusals now carry a node's
standing (`editor_core::NodeStanding`), whose poisoned arm names
`through`, the failed ancestor the repair is at. Most binding doors
project it as the `node`/`through` attributes, through one helper
(`crate::py::standing_fields`, `crates/pncad-py/src/py/mod.rs`). Three
do not, so a Python caller can read the repair node only out of the
message:

- `SelectRefusal`'s `datum_has_no_value` (`py/select.rs`,
  `select_refusal`, the `R::DatumHasNoValue` arm) sets `datum` and no
  `through`. The class has no `through` attribute
  (`pncad.pyi`, `class SelectRefusal`).
- `SelectRefusal`'s `node_has_no_value` (the same function, the
  `R::NodeHasNoValue` arm, new in the fix pass: the flush query refuses
  a node with no value) sets neither the node nor `through`.
- `ChecksError`'s `root_without_value` (`py/checks.rs`, `checks_err`)
  sets `node` and no `through`.

Adding an attribute to a Python class is this program's surface call,
which is why the C6 member stopped at the message.

**Evidence:** the review's red probe,
`crates/pncad-py/tests/test_standing_rev_probe.py`
(`test_a_poisoned_datum_projects_its_node_and_through`, on
`origin/review/standing-rev`): `getattr(err, "through", None)` is `None`
for a poisoned datum whose standing names `through`.

**Recourse:** give the three a `through` attribute (and
`node_has_no_value` its node) through `standing_fields`, declare them in
`pncad.pyi`, and adopt the probe as a row.
