---
id: the-pair-verbs-declared-merge-is-asymmetric-in-its-operands
kind: issue
title: "A declared merge is asymmetric in the pair verb's operands: which member's rims fragment follows the A/B assignment"
status: open
opened: 2026-09-06
refs: [DOCM-7, 2028]
---


## What

Found by both DOCM-7 reviewers, independently and by execution (R1's
`r1_c6_the_reorder_asymmetry_is_the_pair_verbs`, R2's
`r2_the_pair_boolean_is_asymmetric_in_its_operands` — the second on a
bare `Node::Boolean` with no union in the picture at all).

When a coincident face pair is DECLARED, the pair verb's emitter is not
symmetric in its two operands:

- the merged face keeps operand A's carrier — the same plane, with a
  different `origin`;
- the rim edges the merge splits are operand A's, so the
  `Fragment(OrderAlong)` rows sit on A's rims and not on B's.

Swapping the two operands therefore changes the result's NAMES and its
descriptions, while the volume, the face/edge/vertex counts and the set
of `Merged` rows are identical.

Under `Node::Union` the fold is left-associative and the routing takes
no A/B freedom (the accumulation is always operand A, the joining member
always operand B), so reordering the member list is what moves the
asymmetry: `a_declared_pair_routes_by_member_id_and_survives_a_reorder`
(`crates/editor-core/tests/docm7_union_declare.rs`) now ASSERTS it —
fragments on `a` under `[a, far, b]`, on `b` under `[b, a, far]`.

## Where it contradicts what is written

`crates/editor-core/src/names/role.rs`, `RoleSeg::FromMember`'s doc said
"a member's names are a function of the member's identity alone —
neither its position nor how many members precede it". That is true of
the WRAPPER and false of the table: which rows exist is the pair verb's
answer, and it depends on the operand seat. DOCM-7 rewrote the paragraph
to say exactly that and to point here.

Two other sites carry the unqualified sentence and are NOT this unit's
to edit:

- `docs/DOCM-7-SPEC.md:107` (D9) — the spec is deleted at merge, so it
  goes with it.
- `docs/DOCM-REFERENCES-DESIGN.md`, DM4's bullet — the ratified design.
  Its sentence is narrower than the one in `role.rs` ("the order is the
  list's, and the list is data"), so it is not falsified outright; what
  it does not say is that the ORDER shows in the names.

## The question

Whether the pair verb's merge should be symmetric — pick the surviving
carrier and the fragmented rims by a canonical rule over the two names
rather than by operand seat, the way `collapse` already canonicalizes a
`Seam`'s two sides. That would make a union's names order-free, which is
what DM4's spirit asks for, and would move `Fragment` rows in every
existing declared-merge document (goldens, corpus).

Or a ruling that a boolean's names are the operands' and the order is
the author's, with DM4 saying so.

## Where it stands

Open, on DOCM's slate, for Ev — it is the pair verb's behaviour, older
than DOCM-7, surfaced by it. Nothing blocks DOCM-7: the asymmetry is
asserted, and `role.rs` states the measured truth.
