---
id: error-design-e3-calls-a-measure-a-sink
kind: issue
title: ERROR-DESIGN E3 describes a Measure as a recipe sink that poisons no one, a consuming-model holdover
status: closed
opened: 2026-10-07
priority: P2
cost: E
refs: [d10-one-way-to-say-intent-is-unbuilt, a-measured-part-is-not-a-product-root]
closed: 2026-10-08
---

`docs/ERROR-DESIGN.md` E3 (heading at `:143`) calls a measurement "ONE dimension-generic recipe **sink** node", one that poisons no descendant, "F2 verbatim; sinks have none" (`:156`). It also accepts "DAG pollution — dozens of measurement sinks". That describes the consuming model D10 retired ("nothing consumes anything"). Two things are wrong with it:
- Under D10, a `Measure` is an operation that reads bodies and defines a scalar, so it is not a sink.
- Its reads do not remove what it measures from the product. That removal is today's bug, `a-measured-part-is-not-a-product-root`.

"Sinks have none" is also false today, because an `Assertion` reads its `Measure` (`node.rs`, the `Node::Assertion` arm). Stage 2's unit D (`measure-is-an-operation`, spec #4216) fixes the mechanism but leaves E3's text alone. So E3 is reworded to the operation it becomes: its failure poisons its readers (an assertion, or a definition reading it), per FORK-5's ruling (#4218), and nothing else. That lands with unit D.

Found by the consuming-holdover audit (`audit/intent-consuming-holdovers` H5). It was ruled under Ev's "the redesign governs what it replaced" (#4220): the text changes to fit D10.

## Stage 5 slicing (2026-10-08)

This lands with stage 2 D as planned. If D merges without it, stage 5's
PR A (`an-assertion-relates-by-equality`) carries it, since A rewrites
the neighbouring E10 line anyway (`docs/INTENT-STAGE5-SPEC.md` §2, §9).
