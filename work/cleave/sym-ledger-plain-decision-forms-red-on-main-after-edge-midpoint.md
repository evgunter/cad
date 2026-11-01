---
id: sym-ledger-plain-decision-forms-red-on-main-after-edge-midpoint
kind: issue
title: main is red: editor-core m10_sym_profile_interval::the_forms_the_walks_build_are_pinned_per_eps_row (slab, eps 1e-9) reads Plain/Decision forms 9852 against the pinned 9426 since PR 3645 merged
status: closed
opened: 2026-10-01
priority: P1
cost: E
branch: emit/sym-ledger-after-edge-midpoint
pr: 3684
closed: 2026-10-01
---


(SSI orchestrator, 2026-10-01. Bisected locally on `main`'s first-parent history.)

`cargo nextest run -p editor-core --profile ci -E 'test(the_forms_the_walks_build_are_pinned_per_eps_row)'`:

| main commit | result |
|---|---|
| `873dd4db` (#3663) | pass |
| `99345613` (Merge #3645, `cleave/edge-midpoint`) | **FAIL** |
| `8e19eb07` (main head when measured) | FAIL |

The failure: slab, ε = 1e-9, `Plain/Decision calls 980 forms 9852 … digest 750cf690b774e40d40aad503aab9d326` against the pinned `forms 9426 … digest d6944216b808695de65a32c884bbff84`. The other rows match.

PR #3652 (`pcert/rebaseline-sym-ledger`) re-baselined this ledger shortly before, so this is probably two greens that collided rather than #3645 being wrong. But that is inferred, not checked. The fix owed is to decide whether #3645's move of the slab's decision forms is right and, if it is, to re-baseline the pin with what moved. Every PR whose closure reaches `editor-core` is red at the default-eps step until then.

## Closed 2026-10-01 — re-baselined, the move is #3645's (PR 3684)

#3645 is right; the pins moved because it changed what the walks
build. Its `geom::mid_param` spells an edge's mid parameter
`(t0 + t1)·½` where the sites it replaced read `t0 + (t1 − t0)·½`,
and at `Sym<Interval>` the two build different forms for the same
point. Checked on `main`: with only `crates/geom/src/param.rs`'s
body put back to the old spelling, every pinned line comes back
byte-identical, on the slab and the plate.

The plate moved as well, and the row above missed it: the slab's
assertion fires first, so the plate's pin was never compared. On the
slab, `Plain/Decision` forms go 9426 → 9852 at all three ε rows, and
nothing else on the slab moves. On the plate, every line that builds
forms moves, and the frozen counts rise (Plain/Decision 672 → 720).
The call counts stay the same everywhere. PR 3684 re-baselines both
and names the lever in a comment on the pins.
