---
id: LIB-DOORS-3
kind: unit
title: three doors: Node.union, DocEdit.set_members, and the v-degree slot
status: review
opened: 2026-09-08
branch: lib/doors-3
refs: [n-ary-union-and-set-members-have-no-python-door, structural-slots-without-a-binding-door]
---



Three doors, all of them arms behind a bound name and so invisible to
`tests/test_binding_census.py` (rule 1 accounts `Node` and `DocEdit`
WHOLE). The roster that sees them is
`test_north_star.py::test_the_bound_vocabulary_is_exactly_this`, and
it is extended here.

- **`Node.union(members, declare=None)`** — the n-ary fold over a
  member LIST, the fold order the list's (D9), with the same optional
  `declare` slot `Node.boolean` carries. 22 of the kernel's 23 `Node`
  variants now have a Python constructor; `Node::Sweep` is the stated
  exception, `wire_sweep` refusing unconditionally (U4/LQ3), so a door
  for it could not succeed.
- **`DocEdit.set_members(node, members)`** — the rewrite of a list
  input's membership, which is what makes the member list DATA rather
  than a chain of booleans' shape. Its three tags
  (`set_members_on_non_list`, `too_few_members`, `duplicate_input`)
  were minted at LIB-DOORS-1 with no Python caller able to provoke
  them; one test row each now does, asserting `variant` and the
  payload the edit door projects.
- **`DocEdit.bind_v_degree_param(node, name)`** — `VDegree`'s door
  beside `bind_count_param` and `bind_instance_param`, each naming its
  own slot. `Stations` is the fourth structural slot and stays
  undoored: its only node is `Node::Sweep`, which has no constructor
  to aim an edit at, and `bind_count_param`'s prose now says so.

## Delivered

- `crates/pncad-py/src/py/doc.rs` — the three constructors and their
  docstrings; `Node.loft`'s prose gains the two doors that now edit its
  slots; `bind_count_param`'s door-per-slot paragraph names all four
  structural slots and which three have doors; the `DocEdit` class
  docstring's list of exposed edits is restated whole (it named five of
  the ten that existed).
- `crates/pncad-py/pncad.pyi` — the three stanzas, and one DEVIATION:
  the duplicated `@staticmethod` decorator above `set_roots` (inert,
  from 45bd5fc75) is deleted, inside the stanza this unit edits.
- `crates/pncad-py/src/tags.rs` — the list-input arms' comment said the
  Python surface for `Node.union` and `SetMembers` "is LIB's build";
  it is built, so the comment now states what the tags are reached
  through.
- `crates/pncad-py/tests/test_union.py` (new) — the fold against
  `boolean(boolean(a, b), c)` and against the three boxes'
  inclusion-exclusion closed form (16.40625 m³), and one row per
  refusal at both doors.
- `crates/pncad-py/tests/test_north_star.py` — the roster rows, and
  `TestTheVDegreeParamBinding` beside `TestLoftPrism`: the same
  sections skinned at a BOUND degree enclose 9 m³ at degree 2 and 8.75
  at degree 1, one `set_doc_param` apart.
- `docs/GUIDE.md` — one sentence at the loft rung, which the ladder
  already passes. No step added (LIB-SMALL owns the guide's missing
  steps).

Not carried, and not needed: no façade list moved (`Vec<NodeId>` and
`Optional[NodeId]` were already the loft's and the boolean's argument
types), no projection changed, no `variant`/`kind` value moved.
