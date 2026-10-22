---
id: the-python-edit-doors-take-a-resolver-they-never-read
kind: issue
title: The Python edit doors take a resolver= keyword they never read: Doc.apply, Doc.constrain, Doc.insert, Doc.measure and place's constraints arm
status: closed
opened: 2026-10-18
priority: P3
cost: E
refs: [the-edit-door-takes-a-reach-it-never-reads]
closed: 2026-10-22
---

Found while retiring the kernel edit door's unread reach
(`the-edit-door-takes-a-reach-it-never-reads`).

The kernel door no longer takes a reach. The Python doors still take
`resolver=`, and five of them now read it nowhere. Each one has a
`let _ = resolver;` pointing here:

- `Doc.minting` (`crates/pncad-py/src/py/doc.rs`), behind `Doc.insert`
  and every other insert that goes through `insert_node`;
- `Doc.apply`;
- `Doc.constrain`;
- `Doc.measure`;
- `Doc.place`'s constraints arm. Its `pose=` arm evaluates the copy
  and does read the seam, so `place` keeps the keyword.

The stub (`crates/pncad-py/pncad.pyi`, `Doc.apply`, `Doc.insert`,
`Doc.constrain`, `Doc.measure`) still calls `resolver` "the document
seam an edit door reads parts through", though it says each time that
the door reads none.

**The work.** Retire the keyword from the four doors that never read
it, and restate the stub and the callers. Six call sites in
`crates/pncad-py/tests/` pass it today, among them
`test_assembly_author.py:527,528,551,1923`. For `place`, either keep
the keyword for the `pose=` arm and say in the stub that the
constraints arm does not read it, or split the two arms.

This was left out of the kernel retirement because it removes public
Python keywords, which is more than restating callers.


## Closed

Closed 2026-10-22 (`intent/hygiene-batch`). `Doc.apply`, `Doc.constrain`,
`Doc.insert` and `Doc.measure` no longer take `resolver=`, and the
private `insert_node` and `minting` no longer carry one; the stub drops
the keyword and the sentences calling it a seam. The callers lose it:
13 keyword passes in `test_assembly_author.py`, `test_world_pins.py` and
`bench_scene.py`, plus `bench_scene.rest` and `bench_scene.on_floor`,
whose `resolver` parameter fed only `constrain` (one positional pass,
`test_assembly_author.py`'s `base_pinned`).

`Doc.place` keeps its keyword and is not restated here: D′1
(`a-placement-names-its-target-space`) rewrites `place`'s arms and its
stub, routing both through `placed`, which reads the resolver, so the
constraints-arm half of this row is D′1's ground.
