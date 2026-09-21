---
id: check-rs-hand-copied-predicate-sweep-undercounts
kind: issue
title: The validate_document predicate sweep classified only some of its sites: at least three more are hand-copied
status: closed
opened: 2026-09-16
refs: [three-door-predicates-are-hand-copied-not-shared, load-shaped-doors-outside-check-rs-may-duplicate-edit-predicates]
closed: 2026-09-16
pr: 2772
branch: edit/one-predicate-round-two
---


Filed by the review lane on PR 2772 (`edit/one-predicate-round-two`),
at head `4ad10d6fc`.

`three-door-predicates-are-hand-copied-not-shared` says its sweep
"read every `return Err(SnapshotError::…)` in
`crates/editor-core/src/persist/check.rs`'s `validate_document` and
asked, per site, whether the predicate is DELEGATED … or hand-written
at this door", and reports **four delegated and three not**. PR 2772's
`## Closed` table re-affirms that count ("its three hits are …"), and
closing the sweep row retires the record with it.

The count is short. Reading the same function at 2772's head, these
sites are hand-written at the load door AND have an edit-door twin
that spells the same rule a second time:

- **The witness site.** `persist/check.rs:791-795` refuses
  `SnapshotError::WitnessSite` where
  `!matches!(doc.nodes.get(&node), Some(Node::Profile(_)))`;
  `edit.rs:2451-2457` (`check_witness_site`) is the same test, split
  into `EditError::UnknownNode` / `EditError::WitnessOnNonSketch`.
  This is *structurally identical* to the placement site arm
  (`PlacementFault::NotAnInstance`) that 2772 just gave one home — a
  registry key naming a node of the required kind.
- **ε.** `persist/check.rs:662` is
  `!(doc.epsilon.is_finite() && doc.epsilon > 0.0)`;
  `edit.rs:2270` is `!(eps.is_finite() && *eps > 0.0)`. The same
  expression, character for character, in two files, refusing
  `SnapshotError::EpsilonInvalid` and `EditError::InvalidTolerance`.
  Nothing at either site says the other exists.
- **A `Continuous` param declared `Count`.**
  `persist/check.rs:665-674` and `edit.rs:1596-1601` are the same
  `DocParam::Continuous { dim: Dimension::Count, .. }` match, refusing
  `SnapshotError::CountContinuous` and
  `EditError::ContinuousParamCannotBeCount`.

Two more are the weaker shape and belong in the same reading rather
than assumed out: `SnapshotError::MetadataUnversioned` and
`EditError::MetaUnversioned` both call the one `require_versioned`
(the rule is shared; the WALK is duplicated — which is exactly what
2772 found for `Alignment::is_finite`), and `SnapshotError::
DanglingInput` restates `EditError::UnresolvedInput`'s liveness test
over the whole document.

**Why this is a row and not a footnote.** The sweep row it corrects is
being CLOSED by 2772, and `work/README.md` says a residue disclosed in
a `## Closed` section reads as work done and dies with the directory.
The classification is the artifact other lanes will cite for "the
`validate_document` door is swept", and it is wrong by at least three.

**What the next reader should do**: re-run the classification over
every `SnapshotError` site in `validate_snapshot` (not only those the
2026-09 sweep listed), and record the per-site answer in the tree
rather than in an item body, so the census is re-readable.

## Related, outside `validate_document`

`Node::placement_rule_fault` (`node.rs:3047-3055`) spells the frame
half of `doc::placement_fault` a third time — `!frame.is_finite()`,
then `determinant <= 0.0` — and says so in prose at the copy site
("A11/A6 parity: a placement frame is held to exactly what
`SetPlacement` holds a cluster frame to"). `placement_rule_fault` is
one of the FOUR predicates the sweep classified as already delegated,
so its own internal copy of the newly-homed rule was invisible to the
classification.

## Closed (2026-09-16, `edit/one-predicate-round-two`)

Every entry in this row is moved or dispositioned, and the
classification it corrects is replaced rather than patched: the
`three-door-predicates-are-hand-copied-not-shared` row now carries a
**fresh census** of `validate_document`, read at head `f305afee6`,
naming every refusal the function can produce with its predicate and
its edit-door twin.

| This row's entry | Disposition |
| --- | --- |
| The witness site (`SnapshotError::WitnessSite` against `check_witness_site`) | **Moved.** `doc::witness_site_fault`, both doors, two arms each. The load door's fold went with it: `WitnessOnMissingNode` is its own arm. Rows: `a_witness_on_a_non_sketch_node_is_refused_at_both_doors`, `a_witness_on_a_missing_node_is_refused_at_both_doors`. |
| ε (`is_finite() && > 0.0`, character for character in two files) | **Moved.** `doc::epsilon_admissible`, both doors. Row: `a_non_positive_epsilon_is_refused_at_both_doors`. |
| A `Continuous` param declared `Count` | **Moved.** `DocParam::is_continuous_count`, both doors. Row: `a_continuous_parameter_declared_count_is_refused_at_both_doors`, which also MEASURES that the load door's `CountContinuous` arm is unreachable — the display-unit walk refuses first. Filed as `count-continuous-arm-is-shadowed-by-the-display-unit-walk`. |
| `MetadataUnversioned` — rule shared, walk duplicated | **Irreducible, stated at both sites.** The rule is `MetaValue::require_versioned` and both doors already call it; a walk over the one value a door is about to write is not a walk, so there is nothing to unify. |
| `DanglingInput` restating `UnresolvedInput`'s liveness | **Irreducible, stated at both sites.** The rule IS `doc.nodes.contains_key`, which both sites already call; what differs is the subject, not the predicate. |
| `Node::placement_rule_fault`'s internal copy of the frame rule | **Moved.** `Frame::admission_fault` on `placement.rs` is the one frame predicate; `doc::PlacementFault` and `node::PlacementRuleFault` keep their arms and delegate, and both forward its one sentence. |

The residue the census turned up is filed rather than left in prose:
`doc-param-float-walk-is-hand-written-at-both-doors` (a doc param's
nominal is `is_finite`'d at both doors with no function between them)
and the two rows named above.
