---
id: join-desync-on-the-star-fixture
kind: issue
title: JoinDesync (every chord arc separates a loose scaffolding pair) on the star fixture in six member orders
status: open
opened: 2026-09-07
refs: [2073, 2073]
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
