---
id: declared-pairs-retire
kind: issue
title: D10 stage 4 PR F: declare, DeclaredPair, BooleanCoincidence, BooleanDeclarations, SetDeclare, the flush detector's declare protocol and the declared seats on booleans are deleted
status: parked
opened: 2026-10-08
priority: P0
cost: M
blocked_on: [booleans-glue-on-zero]
---

INTENT stage 4, PR F. Ev approved the design in PR 4323 (fork log row
94, FORK-S4U); `docs/INTENT-STAGE4-SPEC.md` §7 predates it, and where
they disagree this row governs.

**A union's order is the author's.** The members stay a list, folded in
the author's stated order (no sort). The pairwise pass stays: each pair
whose closed boxes meet is judged as `m ∪ n`, its refusals are the
union's in every order, and its rows, in member space, are the union's
coincidence rows. **One verdict per carrier pair, read through
lineage**: the fold reads the pass's verdict for each carrier pair
through each face's member parents (`emit_union::Parents`, lineage, not
names) and decides no pair again, so a piece is never re-judged at its
own extent; there is no backing assertion (`fold_step_refusal` goes). A
three-member coincidence is the union's row, spelled by its member
cells. A refusal raised at a fold step names the member whose step
refused; a glue keeps the earlier member's description, as a pair
boolean keeps operand A's.

**Compile-driven deletion**: `Node::Boolean/Union.declare`,
`DeclaredPair`, `BooleanCoincidence`, `BooleanDeclarations`,
`SetDeclare`, the declared payload-walk arms, the flush detector's
declare protocol, the declared seats and carried declared records,
`ImportOptions::declared_contacts`, the Python/viewer/tour surfaces.
Every digest bit-equal to E's; ids after the first boolean move.
Releases 16 rows (spec §12).

**Order-independence is tested and asserted** (Ev, approving #4323:
"would be good to have either good tests or possibly debug asserts to
detect behavior that we think should remain order independent"). This
unit (and E, for unions) lands both:
- a test row family that evaluates each union in the union corpus
  under every permutation of its members (or a seeded sample where n! is
  large), and asserts equal across orders: the pair rows and their
  refusals, the three-member rows spelled by member cells, the names,
  the topology, and that a refusal, when one occurs, is raised either by
  the pass in every order or by a fold step that names its step;
- a debug assertion at the fold's verdict site that every carrier-pair
  verdict the fold uses comes from the pass's table through `Parents`,
  never a fresh decision on a piece.
What follows the list on purpose (the kept description's bits) is
excluded by name, so the test states what is order-free.

**Measurements.** The fan-out experiment in
`union-refuses-in-some-member-orders-and-publishes-in-others` is the
measurement for the verdict reuse. N2's links stay the judgements'
`merge_groups` and `covered`; measure once on the union corpus whether
links-from-rows would differ (a corner touch). Re-measure
`wire/union-pairwise-refusal-names-its-pair-in-digest-id-order`: ids
order by mint ordinal first (`mint.rs`), so its premise may be stale.

When this unit lands, DM4's bullets lose the declared, contradicted and
certified sentences, "The declaration channel, sited at the members" and
"Merges and order"; "The refusal names member faces" becomes "A row
names member cells"; N2's union paragraph reads "the pairwise judgement
of their two members (DM4) merged or covered them".
