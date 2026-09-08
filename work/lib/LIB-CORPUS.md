---
id: LIB-CORPUS
kind: unit
title: the bench corpus is authored, not committed
status: closed
opened: 2026-09-08
branch: lib/corpus
refs: [bench-corpus-staleness-hole]
pr: 2182
closed: 2026-09-08
---

Ev's ruling (E) on `bench-corpus-staleness-hole`, executed.

`crates/pncad-py/tests/corpus/bench/` and its `MANIFEST` are deleted.
`crates/pncad-py/tests/bench_scene.py` is the one Python definition of
the tour's bench — its constants, its two part shapes, its flat-pack
layout and its mated stand — and `test_assembly_author.py` and
`test_assembly_eval.py` both build from it. The eval test writes the
four documents into a temp `Workspace` and resolves each one back out,
so the LOAD path runs on every call over documents nothing keeps on
disk.

The constant-reading guard stays, re-pointed at that module and
widened: the six base constants, the three derived seats BY FORMULA,
the flat-pack's placement literals and the stand's gauge offset and
mate seats are all read out of `demos/tour/src/assembly.rs`. What it
still cannot see is named in `test_assembly_eval.py`'s header.

`demo-tour asm-corpus` is retired: its only consumer was the committed
corpus.
