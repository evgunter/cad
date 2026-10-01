---
id: pattern-spacing-is-a-signed-size-beside-a-direction
kind: issue
title: A linear pattern's signed spacing and a circular pattern's signed step state the direction a second time (a named follow-on of Ev's #3551 rule)
status: open
opened: 2026-10-01
priority: P2
cost: M
design: true
---


A named follow-on of Ev's ruling on #3551 (2026-10-01), which adopted the rule "**a size an operation covers is positive; its direction has one home**" and applied it to the extrude first (`work/edit/extrude-distance-is-a-depth-and-a-side`).

A linear pattern's spacing is signed beside its direction vector, and a circular pattern's step is signed beside its axis. The #3551 designers rated this "unsure" (not measured). Weigh it under the rule before building: whether each becomes a positive size with the direction in its vector or axis, and what each refusal's recourse says (Ev asked that the extrude's refusal show how to write the other direction; the same applies here). Record the answer here.

Filed by the AUTHOR orchestrator on Ev's ruling. Ground: `Node::Pattern` (`crates/editor-core/src/node.rs`, EDIT).
