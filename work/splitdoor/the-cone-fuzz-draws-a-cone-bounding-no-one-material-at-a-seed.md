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

## Done when

The draw cannot produce such a cone (or skips it loudly, by
`loud_skip_marker!`, rather than failing), and this seed is pinned as
an ordinary row beside the fix.

**A second seed** (RESTREAD, `restread/self-touch-second-order`, whose
diff does not reach the fuzz's draw or its oracle): seed
`0xa194f9d0b0758284` at the default ε, effort 1, reports
`ε 1e-6, cone 18, split` and `ε 1e-6, cone 18, conic`, the same
message.
