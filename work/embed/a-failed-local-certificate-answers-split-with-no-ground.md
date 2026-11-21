---
id: a-failed-local-certificate-answers-split-with-no-ground
kind: issue
title: A local certificate that fails answers Split with no ground, so the engine learns that no cell of any size will certify only by reaching the depth floor or the budget: where a refusing ask's seconds go
status: open
opened: 2026-11-21
priority: P1
cost: H
refs: [a-refusing-ask-runs-on-to-its-whole-budget-looking-for-a-verified-crossing]
---


## Finding

From the design fork on `a-refusing-ask-runs-on-to-its-whole-budget-looking-for-a-verified-crossing`
(fork-log row 296, PR 7309) and the gate lane's measurement of where a
refusing ask's seconds go (`origin/embed/gate-measure`,
`measure/gate/walk-rows.tsv`, population in `walk-population.json`).
Both designers named it; neither traced it cell by cell, so the cause
below is read off the code and the measurement, not demonstrated.

**The measurement.** On the 13 dearest refusing asks of the suites
(torus bodies of 4 to 7 faces, 4,434 s in all) the first refusal comes
within 3 splits. Then 4 to 21 other face pairs refuse in their turn,
and 3,250 s pass before each of those reaches its own first refusal.
On the cheap refusing asks the first refusal comes at the depth floor
(median depth 40) and 71 to 93% of the ask's time precedes it. No rule
about when an ask stops reaches either figure.

**The cause, as read.** A local certificate that fails on a touching
cell pair answers `Outcome::Split { arm, shared }`
(`crates/topo/src/injective.rs`), documented "No arm held here; a
smaller pair may", with no ground for the claim. The engine cannot tell
"this cell is too big" from "no reading exists for this shape at any
size", and learns the second only by reaching the depth floor or the
budget. The certificate knows more than it says:

- `departure` returns `split` whenever either face's part is not
  presented, and `stuck` refuses only when both are;
  `a-contradicted-extent-refuses-with-no-typed-cause` records the
  one-sided case running to the depth budget;
- `Unpresented::Local` is returned from about 30 sites, among them the
  fall-through arms of `present_chart` and its neighbours, where what a
  smaller cell would change is not said;
- `interval_sign_as` maps a decided `Zero` and an `Indeterminate` both
  to no sign, and the pair splits, where a distance cell in the same
  state refuses at once (`Bound::terminal`, `Sliver`);
- `CUSP_DEPTH`, the one early floor that exists, is by its own doc "a
  trade and not a derivation".

Instances already on rows: `vacuous-reads-a-cell-at-a-corner-local-at-every-depth`,
`a-cell-whose-side-lies-a-rounding-off-a-curved-edge-reads-local-at-every-depth`,
`a-plane-faces-corner-that-is-not-strictly-convex-is-read-at-no-depth`,
and PR 7113's re-review (an off-axis sphere union that main refuses at
once takes 2,000 to 4,400 splits to refuse once a pose reads its waist
corner).

## What is needed

`[one-way.readings]`: "an unlisted arm refuses typed, naming what it
lacks; no arm answers a default value". A split is to carry its ground,
the thing a smaller cell would reduce (more boundary in the cell than
one reading takes; a margin that straddles the band's edge); a failure
with no such ground refuses that cell at once, typed. At `Interval` and
`Sym` a margin that is wide because the parameter box is wide is such a
failure. A ground typed wrongly refuses a cell a smaller one would have
certified: an over-refusal, never a wrong `Embedded`.

**Measured first** (asked of the gate lane, 2026-11-21): for every
touching cell pair that answers `Split` on the 13 dearest asks and a
sample of depth-floor asks, which part failed and how, and whether the
child failed as its parent did; per refusing pair, the depth at which a
reason no smaller cell changes first appears and the splits after it.
That is this unit's saving, and it decides whether the embedding ask
can stop only at a verified crossing (PR 7309's second decision).

Not dispatchable until that measurement is in and the engine units in
flight on `injective.rs` have landed: it touches every presentation.
