---
id: part-nesting-segfaults-before-the-depth-bound
kind: issue
title: editor-core: a chain of a few hundred nested parts kills the process with SIGSEGV before instantiation reaches MAX_DEPTH, so DepthExceeded never renders
status: open
opened: 2026-09-29
priority: P1
cost: M
---


(EDIT, found by the review of `edit/part-refusal-recourse`, PR 3492.)

## What

`PartFault::DepthExceeded` (`crates/editor-core/src/eval/parts.rs`)
is the bound `PartCache::resolve_and_evaluate` checks against
`MAX_DEPTH` (1024) before it descends into a referenced document. The
descent is ordinary recursion: each level evaluates the referenced
document on the same thread's stack. The stack runs out long before
the bound, so the process dies with SIGSEGV instead of refusing typed.
That breaks fail-loud: `DepthExceeded`'s sentence, and its recourse,
never render.

## Evidence

A chain of N documents, each instantiating the one before over a
`bench_scene.post()` leaf, stored in one `Workspace` and evaluated from
Python (`evaluate(last, resolver=Workspace(dir))`):

| build | N that evaluates | N that dies with SIGSEGV |
|---|---|---|
| dev cdylib (maturin, unoptimized) | 100, 200 | 400, 700 |
| release cdylib | 300 | 450, 600, 700, 1030 |

Reproduced at N=400 on the dev wheel of PR 3492's fix pass (exit 139).

The probe, run from `crates/pncad-py/tests` with the wheel installed:

```python
import sys, time
from pathlib import Path
import tempfile
import bench_scene
from pncad import Doc, DocRef, Node, Workspace, evaluate, content_pin

N = int(sys.argv[1])
d = Path(tempfile.mkdtemp()) / "chain"
d.mkdir()
store = Workspace(str(d))
prev = bench_scene.post()
store.create(prev)
for i in range(1, N + 1):
    doc = Doc(f"chain-{i}")
    doc.insert(Node.instantiate_part(DocRef(prev.id, content_pin(prev))))
    store.create(doc)
    prev = doc
ev = evaluate(prev, resolver=Workspace(str(d)))
```

## What would close it

A bound the stack honours: either descend without recursion (or on a
stack sized for `MAX_DEPTH`), or lower `MAX_DEPTH` to a depth measured
to fit the smallest stack a door runs on (a Python thread's, a viewer
worker's), with a row that evaluates a chain one past the bound and
reads `DepthExceeded` back typed.
