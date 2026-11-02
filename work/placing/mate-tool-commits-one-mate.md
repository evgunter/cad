---
id: mate-tool-commits-one-mate
kind: issue
title: the mate tool commits one mate and its values, not a placement's whole bundle
status: parked
opened: 2026-10-31
priority: P1
cost: H
blocked_on: [the-mate-tool-offers-two-of-the-five-mate-kinds]
refs: [the-viewers-mate-remove-takes-the-structural-door, removing-a-value-constraint-is-a-second-spelling-of-setting-it]
---


## Finding

Ev (in chat, 2026-10-31), trying to align the corners of two stacked
bodies, found three things in the mate tool:

1. **The tool commits one mate.** Its draft is one mate plus values on
   the freedoms that mate's kind leaves (`matetool::MateFreedom`,
   `crates/viewer/src/pane/create.rs`). To place a copy by two mates the
   person commits one, then opens the tool again for the second.
2. **The second mate is refused** (`MateFault::Overconstrained`): every
   bundle pins its copy, so the first commit writes `0` values on what it
   left, and the second mate then takes freedoms values already hold.
   The kernel half of this (whether consistent mates compose, and the
   missing edge-aligning kind) is
   `work/msolve/a-consistent-second-mate-after-a-rest-is-refused.md`.
3. **The tool offers two of the five kinds and picks faces only**:
   `work/placing/the-mate-tool-offers-two-of-the-five-mate-kinds.md`.

## Ev's direction

Keep the property that no half-finished placement is ever committed
(every bundle pins its copy). Make the mate tool like the profile
editor: a draft that holds **several** mates and values for one
placement and commits them together as one bundle, one undo step. A
mate may stand **in place of** a slide or a spin the earlier mates
leave, rather than the panel offering only numbers for those freedoms.

## Also found

- The tree's `remove` button sits on every constraint row, value rows
  included. On a value row it is a no-op: `remove_constraint`
  (`crates/viewer/src/session.rs`) rewrites the bundle without it and
  writes the freed freedom as `0`, which is the value it removed. The
  startup `Place` is six value rows of `0`, so all six buttons record a
  history state and change nothing, and nothing says so (reproduced
  with `examples/drive.rs`: `click "remove" 0`, history 1 → 2 states,
  tree unchanged).
- The flip control is the checkbox "faces rest against each other"
  (`crates/viewer/src/pane/create.rs`, `drafts.mate_flip`), which reads
  as a yes/no about contact rather than as the mate's sense. Ev's
  direction (2026-10-31): a **sense** choice with two named values,
  along the lines of `flush` (normals the same way, faces level) and
  `opposed` (normals facing each other, faces resting together),
  writing the same `Flip`.

## Decided (2026-11-01, PLACING orchestrator; design fork row 232)

Two designers weighed this, reconciled over two rounds, and agreed. No ratified text changes. The design elaborates Ev's direction above, so the orchestrator decided it with no `[ev]` PR.

**The premise is corrected first.** Finding 2's cause is not the zeros. The bundle door drops the values a new mate takes (`AddConstraint`, `crates/editor-core/src/edit.rs`); it does not refuse them. The refusal is between the two mates. A11 (1) (`crates/editor-core/ASSEMBLY.md`) admits two mates only where their families meet transversally, which of all the pairs only a ball (point) and a direction do. D10 `[placement]` says a bundle holds at most one mate fixing a direction. So a rest plus a flush second plane is `Overconstrained` even inside one draft. Whether such a pair can compose, or a residual kind (edge-in-plane) is added, is MSOLVE's `a-consistent-second-mate-after-a-rest-is-refused`. The editor below shows the door's verdict whatever that row decides. Ev's corner case has a one-mate answer today: `Frame`–`Frame`, which this design makes authorable.

**The final state: a placement editor whose draft is the placement's whole bundle**, one editor with two doors, as the profile editor has:

- **The doors.**
  - *Create*: a body not yet placed. Commits one `DocEdit::place` holding the whole bundle.
  - *Edit*: opened from a placement's tree row or its constraint rows, or by picking on a copy. Commits one `SetConstraints`, keeping the ids of rows loaded from the bundle.
  - Either commit is one undo step. Nothing enters the document before Apply (G1).
  - A draft over a bundle the document has since changed is stale and reloads with a notice, as the profile editor's does.
- **The draft is mate rows, with value rows derived beneath them.**
  - A mate row holds a kind (all five), a sense and its picks.
  - The value rows are exactly the freedoms the mates leave, by the kernel's own rule (`Placed`'s residual). No viewer-side freedom table exists.
  - Each value is prefilled where the copy stands (the snap, generalised to the bundle) and editable. A number typed on a freedom that survives is kept.
  - "A mate in place of a slide or a spin" is the *add a mate* button: the value rows shrink to what the admitted mates leave.
- **Picks are seated and routed by body.**
  - A pick on the draft's copy fills an `on` seat, and a pick elsewhere fills a `to` seat. Order carries nothing.
  - Each kind names its seats:
    - plane: a face;
    - axis: a face or an edge;
    - point: a vertex, or a face or edge with a centre;
    - direction: a face or an edge;
    - frame: an axis seat (an edge) and a point seat (a vertex), written `PoseDef::Through`. Within that two-seat side, an edge fills the axis seat and a vertex the point seat.
  - A lost pick empties its seat, typed (`crates/viewer/src/seats.rs`'s rule). The current rule promoting a surviving `to` pick into the `on` role goes.
- **Sense** is a two-valued choice per mate row, written as `Flip` on `to`:
  - plane: `opposed` (resting together, the default) or `flush`;
  - axis and direction: `same` or `reversed`;
  - frame: `same` or `turned over`;
  - point: no sense.
- **The door judges the draft live.** Every change re-runs the one write on a scratch `Recording`, the pattern `DocSession::edit_profile_report` uses.
  - The panel names the refusal before Apply: the mates that repeat an equation (`Overconstrained`), the freedoms left unpinned, the two spaces that differ, a side that is not on the copy.
  - The same scratch evaluation draws the copy where the draft stands it.
  - A refused Apply keeps the editor open. Today `ToolKind::commits` is `false` for `Mate`, so the tool closes before its op lands.
- **The tree's `remove`.**
  - Value rows lose it: a freedom is always set, and its only edit is its number.
  - On a mate row it no longer commits. It opens the editor with that mate struck out, so the freed values and the picture are seen before Apply.
- **Rejected:** keeping the one-mate tool and accumulating mates into a list committed at once. It leaves a committed placement with no editor, has no way to pick a frame side, and decides values at commit, out of sight.

**Build notes.**
- Sequential after `the-mate-tool-offers-two-of-the-five-mate-kinds`; reuse its per-kind pose reading as the seats' reads.
- The unverified piece is the generalised snap. `session/snap.rs`'s `snapped` reads only the first mate, and its `Hold` has no case for a point held by a frame or a ball. Tier: FULL review at least; decide dual at dispatch.
- Riders: `the-mate-panel-names-no-slide-a-plane-leaves` (subsumed by the derived value rows) and `the-viewers-mate-remove-takes-the-structural-door`.
- Filed for DOCTAIL: `removing-a-value-constraint-is-a-second-spelling-of-setting-it`.
