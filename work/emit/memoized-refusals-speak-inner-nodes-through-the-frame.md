---
id: memoized-refusals-speak-inner-nodes-through-the-frame
kind: unit
title: A memoized failure's inner node ids (NodeErrorKind, PartFault, MateFault) speak through the frame that hands it out
status: review
opened: 2026-10-01
priority: P2
cost: H
parent: node-labels-are-document-data
pr: 3782
---


Split from `refusal-values-speak-the-node-with-its-label`. That PR speaks a memoized `NodeError`'s own node through the frame that owns the document (`NodeError::spoken`, `CarriedLevel::line_in`). It does not speak the node ids a failure carries INSIDE it. Those stay bare tags (`node 3fa9c1d2a0b1`), because they live in values the evaluation memo reuses, and a label captured there would go stale on a rename (DESIGN.md Band 1, "Node labels").

## What still says a bare tag

- `NodeErrorKind`'s `Display` (`eval/mod.rs`): the arms naming a section, a split half, a pattern input, a revolve axis, a declared site, a name's minting node, and so on.
- `PartFault`'s `Display` (`eval/parts.rs`), e.g. "the part's node … failed … Recourse: open the part and repair node …".
- `MateFault`'s `Display` (`mate.rs`, `mate/solve.rs`), e.g. `PlacerRefused`'s "repair node …". It is memoized inside `NodeErrorKind::Mate`, and the edit door carries it whole (`EditError::MateRefused`, `MaintenanceRefused`).
- `SelectionRefusal::NoSuchBody`'s `Display` (`clearance.rs`), "node …'s value carries no body at index …". It rides in `ClearanceRefusal::Selection`, which `NodeErrorKind::MeasureClearanceRefused` carries, so it is memoized. Ruled here by `analysis-door-refusals-speak-the-node`. `ClearanceReport::serialize` (`clearance.rs`, its `verdict refused` line through `ClearanceRefusal::payload`) prints that `Display`, so a machine form carries the 12-digit tag rather than the full id.
- A doubled noun where an outer spoken node meets the inner text. A mate's failed row reads "Mate 3fa9… failed: the mate solve refused: mate 3fa9…'s a reference has no derived pose …" (`viewer/tests/msolve3_placer_refused.rs`, `the_mate_row_names_the_direction_and_not_a_dangling_head`). The outer node now says its kind, and the inner `MateFault` text names the same mate again by its own noun. `EditError::MateRefused` has the same seam ("Mate … is refused by the solve on its own datum: mate …"). When the inner ids are spoken, that sentence should name the mate once.
- A carried level in a part document (`CarriedLevel::line_in` speaks it by tag): the frame that draws it does not hold the part. The pinned part's labels are in its pin, so a frame holding the resolved part could speak them without staleness.
- The Python `MateFault` value and `mate_err` (`pncad-py/src/py/mate.rs`) carry no document, so their carried causes say tags.

## The shape of the fix

These values need a rendering that takes the speaker, the document of the frame handing the value out, rather than a `Display` that cannot see one. For example, a `Display` adapter over (value, `&Doc`) that each inner id is spoken through. It is design work, because `NodeErrorKind`'s `Display` is the D2 "kernel refusals unaltered" surface.

## Ruled here by `selection-door-refusals-speak-the-node`

- `NamingError` (`names/emit.rs`): raised only inside evaluation (the verbs, `names/defer.rs`, `names/discriminate.rs`, `eval/anchor.rs`) and carried whole by `NodeErrorKind::Naming`, so it is memoized and keeps the tag. Its three node sentences (the `EMISSION_FRAMING` "upstream node", the `UNRULED_FRAMING` "operand node" and "member node") and its names are this row's.
- `ResolveError` is carried by five `NodeErrorKind` arms (`eval/mod.rs`). It now writes its sentence once over `spoken::Speaker` and has `spoken(doc)`. A `NodeErrorKind` rendering that takes a speaker can forward `Said(error, by)` rather than its tag `Display`.
- `NodeStanding` rides inside `ProductError::Root` (so `PartFault`) and `ClearanceRefusal`. It has `spoken(doc)` too.
- `MintRefusal` is memoized with a part's evaluation (`eval/parts.rs`, `PartValue::unminted`). It has `spoken(doc)`, which `AssemblyError::spoken` uses for this document's own rows. Carried rows keep their tags.

`spoken::Speaker`, `Say` and `Said` (`spoken.rs`) are the shape this row's "Display adapter over (value, `&Doc`)" names. They are public (pncad's `ExportError` is said by them too), and nothing in `NodeErrorKind` uses them yet.

## Built

Each value keeps its bare ids and writes its sentence once over `spoken::Speaker` (`Say`); its `Display` is that sentence said by tag.

- **`NodeErrorKind`** says every node of its own document through the speaker, and forwards `ResolveError`, `NamingError`, `MateFault` and `SelectionRefusal` as `Said(value, by)`. `NodeError::spoken`, `NodeRefusal::line_at` and `CarriedLevel::line_in` hand it the frame's speaker.
- **The failing node is named once.** `Speaker::about(node)` marks the node the enclosing line already names, and `Speaker::node_as` then says it as `this <noun>`. A mate's failed row reads "Mate … failed: the mate solve refused: this mate's a reference …", and `EditError::MateRefused` reads "Mate … is refused by the solve on its own datum: this mate …".
- **`MateFault`**, with `LeverRefusal`, `FaceRefusal`, `OffsetCheck` and `PoseRefusal`, says the mate, its instances, its head, its placer and its part node from the mate's document. A part's face and reach refusal are numbered in the part and keep their own `Display`.
- **`PartFault`** is numbered in the part. A frame that holds only this document says it by tag. `PartFault::spoken(doc_ref, part)` and `CarriedLevel::line_in_part(part)` speak it from the resolved part, and each panics on another document.
- **`NamingError`** says its upstream, operand and member nodes and its names' minting nodes.
- **`SelectionRefusal`** says its node, its group and its `NodeStanding`. `ClearanceRefusal::payload`, the goldening form `ClearanceReport::serialize` prints, now holds a machine form with full ids (`no_such_body node=<16 hex> index=…`). `ClearanceReport::render` holds no document and says the tag.
- **The edit door** keeps the nodes a refused mate's fault names, spoken from the document the mate would stand in (`EditError::MateRefused::held`, `spoken::held_by`, `HeldNodes`). That refusal is not memoized.
- **Python**: `SolvedPoses` keeps the document it solved. Its `MateFault` values, `MateError`, and the placement door's `EvaluationError` speak from that document. An edit refusal's `fault` speaks from the nodes the door kept. Every node getter still crosses the full id.

`Unplaced::DeadGauge` names a deleted gauge, which no document holds, so its tag is what a speaker would say anyway. `pncad-py`'s `prose_census` now reads `Say` impls as well as `Display` impls.
