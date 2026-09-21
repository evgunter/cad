---
id: validate-pcurves-never-recertifies-a-face-it-finds-incomplete
kind: issue
title: validate_pcurves skips its re-certification and continuity passes on any face missing a row, so a stale row on an incomplete face is never measured
status: open
opened: 2026-09-13
refs: [S331]
priority: P0
cost: D
---



Found by TOPO's `split-edge-children-lack-pcurve-rows-on-curved-charts`
lane, by execution, on the item's own fixture.

`validate_pcurves` (`crates/topo/src/pcurves.rs`) runs three passes per
face: pass 1 collects presence and builds the face's chart window from
the stored rows, pass 2 re-certifies every stored row against that
window, pass 3 re-checks the loop's one-branch continuity. Pass 1 sets
`complete = false` on a `MissingCache`, and the guard between the
passes — `let (Some(window), true) = (window, complete) else { continue; }`
— then skips passes 2 and 3 **for the whole face**. So on an incomplete
face every row that IS stored is accepted unmeasured.

Measured (the pre-fix behaviour of `Body::split_edge`, a minted
cylinder wall of `crates/topo/tests/split_edge_pcurve_rows.rs`): after
the split, the two parent half-edges' rows still claimed the parent's
whole interval `(0.2, 1.4)` while their edge's certified curve had been
narrowed to `(0.2, 0.8)`, and the pass reported only
`MissingCache` for the two new halves. The wrong interval was never
re-certified, and would not have been however wrong it was.

**Why it matters beyond that op.** `crates/topo/src/pcurves.rs`'s
module docs rest the whole `Neither` posture on this backstop — "the
tier-3 pcurve pass catches a stale row LOUD — it re-certifies against
the current carrier/surface/window and fails". The ops that stale a row
in CONTENT are frequently the same ops that mint a rowless half-edge
into the same face (`mev`/`mef`/`mekr`, and `split_edge` before TOPO's
fix), so the incompleteness that fires `MissingCache` is exactly the
state that switches the re-certification off. The body is still
refused, so this is a wrong REASON rather than a vacuous green — but it
means the module's safety claim is not the one the code makes.

Whether the fix is to run pass 2 over the rows that ARE present (with a
window hulled from those rows, as pass 1 already builds it) or to state
the narrower claim at the entry is this program's call. Sibling row on
the neighbouring question of what an at-rest absence may claim: `S331`.
