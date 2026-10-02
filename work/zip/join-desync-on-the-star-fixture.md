---
id: join-desync-on-the-star-fixture
kind: issue
title: JoinDesync (every chord arc separates a loose scaffolding pair) on the star fixture in six member orders
status: open
opened: 2026-09-07
refs: [2073, 2073]
priority: P0
cost: H
---

## What

R2's star fixture `r2_p2` (`docm/8-review-r2`: `a` = x∈(0,1),
`c` = x∈(0.5,1.5), `d` = x∈(1.2,2.2), all y∈(0,1); `f` = x∈(0.5,1.5),
y∈(0.5,1.5); all z∈(0,1); the `a`–`c`–`d` chain declared on its four
flush families and `c`–`f` on the caps) refuses
`Boolean(JoinDesync { what: "every chord arc separates a loose
scaffolding pair" })` in six of 24 orders — every order in which `a`,
`c` and `d` are folded before `f`: `[a, c, d, f]`, `[a, d, c, f]`,
`[c, a, d, f]`, `[c, d, a, f]`, `[d, a, c, f]`, `[d, c, a, f]`. The
refusal is raised at the last step, when `f` joins the fused
`a`–`c`–`d` bar whose caps are one merged face each: `f`'s caps
overlap the merged cap in area, and the join's chord scaffolding does
not close.

## Why

`JoinDesync` is `topo`'s (`boolean/join.rs`); the union's routing
fed `(c.cap, f.cap)` at the right step through the flat merged row
(the look-through), and the refusal is the kernel's answer to the
geometry. Not touched by DOCM-8.

## Where it stands

Open, for `topo`'s placement. Reproducer: the fixture above in any of
the six orders.


## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/bool/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): the defect is the boolean's (`crates/topo/src/boolean/*` is S-BOOL's), reached from a declared union. Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

(At DOCM's exit sweep, `refs` names the PRs `DOCM-8` stood for: `DOCM-8` = #2073 — the unit rows left the tracker with `work/docm/`; `docs/DOC-LEDGER.md` sweep 14.)

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to CURVED (its charter names S-BOOL's ceded ground and inherits at S-BOOL's exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Measured (2026-10-02, `join/star-desync`)

Reproduced in `topo` directly: each member order folded left with
`flush_declarations` at every step. With every flush pair declared,
18 of 24 orders refused `JoinDesync { "every chord arc separates a
loose scaffolding pair" }`, not only the six above. The smallest is
`(c ∪ d) ∪ f`.

The join is not where the state first goes wrong. `c ∪ d` merges its
caps and walls and keeps every boundary vertex, so its y = 1 rims
carry valence-2 vertices at x = 1.2 and 1.5. `f`'s bottom cap covers
`(1.2, 1, 0)`, coplanar with the bar's cap. At that vertex,
`classify_vertex_on_face` (`boolean/vtxfac.rs`) reads both real edges
Out (lumped with the coplanar cap) and only the wall sector's bisector
In. So the Out run holds every real edge of the orbit. The fan end
`he2 = next(mate(last.he))` comes back to `first.he`, and `mev_null`
builds a strut in the wall face, as `MevSite::Fan { he1 == he2 }`
does. The record was labelled and faced as a fan split: `dangling:
false`, `he_plus` facing the start germ (−x). So the half next to the
edge arriving from +x faced −x. The join's first chord, from the
x = 1.5 site to the x = 1.2 site, then walled off the −x half from its
partner at x = 0.5 whichever way round it went: the dump of the y = 1
wall's loop at the refusal has the loose half at loop index 1, its
partner at 4, and the chord ends at 19 and 2.

Recording the strut as dangling (side Below, `he_minus` facing the
start germ, as the pierce path's own dangling strut does) folds all
24 orders to the 10-face prism. Each certifies at tiers 2, 3 and 3′,
with volume 2.7.
