---
id: engine-leaves-does-not-count-self-arms-in-an-impl-of-an-engine-type-outside-it
kind: issue
title: engine-leaves.sh does not count an engine's items spelled Self:: inside an impl of an engine type written outside the engine
status: open
opened: 2026-11-03
priority: P2
cost: E
refs: [6451]
---

Found in PR 6451, where a lane briefly held `census.rs`'s `names-boolean` pin by doing exactly this, and the review caught it.

`scripts/gates/engine_leaves.py` counts every occurrence of an engine item by its own name, and its header refuses second names outright ("by name is sound only while every item is spelled by its own name"). One second name is not refused, though, and not counted either.

An inherent `impl crate::boolean::FaceUnread { ... }` block written in `crates/topo/src/census.rs` names the engine type once, in its header. Every arm inside it is spelled `Self::Trim`, `Self::Placement(..)` and so on, and the counter skips any name under a `Self` head ("it counts unless H is `Self`"). So code outside an engine can name any number of the engine's variants and associated items at a cost of one site. The docstring's "WHAT IT CANNOT SEE" lists the reverse case (a method an engine declares on a type outside it), not this one.

Fix: inside an `impl` whose self type is an engine item, written in a file outside that engine, count each `Self::` occurrence as a site of that item, or refuse such an impl the way an alias is refused. Add a selftest plant of the shape.

