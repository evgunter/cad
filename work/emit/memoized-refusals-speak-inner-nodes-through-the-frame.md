---
id: memoized-refusals-speak-inner-nodes-through-the-frame
kind: unit
title: A memoized failure's inner node ids (NodeErrorKind, PartFault, MateFault) speak through the frame that hands it out
status: open
opened: 2026-10-01
priority: P2
cost: H
parent: node-labels-are-document-data
---


Split from `refusal-values-speak-the-node-with-its-label`. That PR speaks a memoized `NodeError`'s own node through the frame that owns the document (`NodeError::spoken`, `CarriedLevel::line_in`). It does not speak the node ids a failure carries INSIDE it. Those stay bare tags (`node 3fa9c1d2a0b1`), because they live in values the evaluation memo reuses, and a label captured there would go stale on a rename (DESIGN.md Band 1, "Node labels").

## What still says a bare tag

- `NodeErrorKind`'s `Display` (`eval/mod.rs`): the arms naming a section, a split half, a pattern input, a revolve axis, a declared site, a name's minting node, and so on.
- `PartFault`'s `Display` (`eval/parts.rs`), e.g. "the part's node … failed … Recourse: open the part and repair node …".
- `MateFault`'s `Display` (`mate.rs`, `mate/solve.rs`), e.g. `PlacerRefused`'s "repair node …". It is memoized inside `NodeErrorKind::Mate`, and the edit door carries it whole (`EditError::MateRefused`, `MaintenanceRefused`).
- `SelectionRefusal::NoSuchBody`'s `Display` (`clearance.rs`), "node …'s value carries no body at index …". It rides in `ClearanceRefusal::Selection`, which `NodeErrorKind::MeasureClearanceRefused` carries, so it is memoized. Ruled here by `analysis-door-refusals-speak-the-node`.
- A doubled noun where an outer spoken node meets the inner text. A mate's failed row reads "Mate 3fa9… failed: the mate solve refused: mate 3fa9…'s a reference has no derived pose …" (`viewer/tests/msolve3_placer_refused.rs`, `the_mate_row_names_the_direction_and_not_a_dangling_head`). The outer node now says its kind, and the inner `MateFault` text names the same mate again by its own noun. `EditError::MateRefused` has the same seam ("Mate … is refused by the solve on its own datum: mate …"). When the inner ids are spoken, that sentence should name the mate once.
- A carried level in a part document (`CarriedLevel::line_in` speaks it by tag): the frame that draws it does not hold the part. The pinned part's labels are in its pin, so a frame holding the resolved part could speak them without staleness.
- The Python `MateFault` value and `mate_err` (`pncad-py/src/py/mate.rs`) carry no document, so their carried causes say tags.

## The shape of the fix

These values need a rendering that takes the speaker, the document of the frame handing the value out, rather than a `Display` that cannot see one. For example, a `Display` adapter over (value, `&Doc`) that each inner id is spoken through. It is design work, because `NodeErrorKind`'s `Display` is the D2 "kernel refusals unaltered" surface.
