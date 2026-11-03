---
id: the-cone-fuzz-draws-a-cone-bounding-no-one-material-at-a-seed
kind: issue
title: cone_fuzz draws a cone that bounds no one material for the split at seed 0xb33c662ce7b49b24, failing at ε 1e-9 on main
status: open
opened: 2026-11-02
priority: P2
cost: C
---

Found by SPLITDOOR on `splitdoor/section-line-components` (PR 6310),
from a changed-path run. Replayed on a clean `origin/main` worktree at
`0b4711875`: red there too, identically, so it is main's and not 6310's.

## What

`topo` `link::cone_fuzz::the_consumers_paths_never_contradict_the_exact_oracle`,
replayed with
`CAD_TOLERANCE_EPS=1e-9 CAD_FUZZ_SEED=0xb33c662ce7b49b24 CAD_FUZZ_EFFORT=1`:

```
ε 1e-9, cone 21, split: the drawn cone bounds no one material (a fixture defect)
```

The fuzzer's own guard (`cone_fuzz.rs`, the `one_material` check
before the split's probes) reports that the cone it drew for this seed
is no fixture: its faces do not bound one material. That is a defect
of the draw, not a contradiction of the oracle, but it fails the row.

A second seed, `0xc104fc1eeda44d90` (`CAD_FUZZ_EFFORT=1`, any
`CAD_TOLERANCE_EPS`), fails the same way at its ε-1e-6 draw:
`ε 1e-6, cone 25, split: the drawn cone bounds no one material`.
Found on `splitdoor/cone-apex-vertex-pairs`, and red identically at
that branch's earlier head `656e9bd920`. The failing check is fixture code only
(`drawn`, `cone_oracle`, `one_material`). A third,
`0xbdef491a30087e7a`, fails it at its ε-1e-9 draw (`cone 30, split`).

## Done when

The draw cannot produce such a cone (or skips it loudly, by
`loud_skip_marker!`, rather than failing), and this seed is pinned as
an ordinary row beside the fix.
