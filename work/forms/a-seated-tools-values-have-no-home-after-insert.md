---
id: a-seated-tools-values-have-no-home-after-insert
kind: issue
title: The values a seated tool sets at create have no faithful home after insert: a split plane and a circular pattern axis are frozen frames, a linear pattern's spacing is minted per axis, a clearance bound is payload
status: open
opened: 2026-11-02
priority: P1
cost: H
design: true
parent: creating-a-thing-and-editing-it-are-two-implementations
---


## Finding

Mapped read-only on origin/main (2026-11-02, FORMS orchestrator's
scout) against the ratified rule in `crates/viewer/README.md`, "A
create gesture commits the node". That rule says every value is
edited on the node's edit surface after insert, and that a value the
insert writes into a variable it mints for the node is edited from
the node's own rows. Several seated tools have no such home.

- **Split plane** (`DocSession::add_split` → `read_off` → `frame_off`,
  `crates/viewer/src/session.rs`). The form's point and normal
  formulas are evaluated to numbers (`nominals`) and dropped. The
  stored `Node::Split.tool` is a `RoleRead` (role Xy) of an anonymous
  pose chain: `Spin`/`Slide` moves (`moved_frame`,
  `editor-core/src/mate/pin.rs`) over `Through { Axis(edge E),
  Point(vertex V) }`, where E and V are whichever edge and vertex the
  body offered first. The only scalars are those moves' `by`s,
  relative to that arbitrary frame. `Node::slots()` gives a split no
  rows ("this feature carries no parameters"). A pose variable cannot
  be redefined: `VarDecl` is Free | Defined | Index only.
- **Circular pattern axis** (`add_pattern`, `axis_operand`): the same
  frozen chain, read through Z, minted twice (the mate's `on` and
  `to`).
- **Linear pattern** (`combine::row_steps`): the spacing is minted
  again on every nonzero axis, so one value is stored as up to three.
  No direction or spacing row exists; a zero axis is the constant 0.
- **Pattern count**: the named index `k` has a door
  (`props::index_count_edit`), but the `Selection::Variable` arm of
  the Properties pane draws nothing for an Index, so no widget reaches
  it.
- **Clearance bounds**: these are payload
  (`Claim::Clearance { bounds }`), not slots. `Node::slots()` is empty
  for an Assertion, so no edit addresses them. The bound's kind
  (at least, at most, within) is not editable either.
- **Blend size** is a proper slot (`SlotId::Radius` /
  `ChamferDistance`) and needs nothing. **Mirror** reads the picked
  face's plane live and has no values.

## Why this is a design question

What a split plane or a pattern axis *is* in the document, so that a
person can edit it after insert, is not settled by the rule. It
reaches editor-core: pose definitions, VR rules, what a slot can
hold. Designers weigh it first (`docs/prompts/designer.md`).
