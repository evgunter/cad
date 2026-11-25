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

## Measured (gate lane, 2026-11-21 to 22): the splits by reason

`origin/embed/gate-measure` at `abf225214e` (every split's reason
recorded as class, site and shared cell, and inherited by its
children; no verdict changed; population and rows in `measure/gate/`).
The finding above does not survive it as written.

**The 13 dearest refusing asks (929,000 splits).**
- **905,000 (97%) are NON-touching cell pairs**, split because their
  separation's enclosure straddles the band's edge
  (`lo < K·ε ≤ hi`); 900,176 repeat their parent's reason exactly.
- Touching pairs where a local certificate answered `Split`: 24,000.
  One part presented and the other `Local` at an edge no presentation
  reads, about 19,000; a `Structural` part, about 4,000; both
  presented with the departure sign straddling zero, 354; `Local` in
  `zero_of` 281, in `present_chart` 220. No margin wholly in band.
- Per refusing face pair (166): a refusal that is provably safe to
  make early (a `Structural` part, or a margin or separation wholly in
  band) exists in 9 pairs, first at depth 11, and saves 6.9% of the
  time. Refusing a pair whose every split below some depth repeats its
  parent's reason would save 87.8% (72 pairs, from depth 1), and is not
  sound as stated: a straddling separation can still narrow.

**A sample of 40 asks whose first refusal comes at the depth floor
(34,000 splits).** Distance undecided, straddling: 53%. Touching:
`present_plane`'s catch-all (a cell holding three or more of the plane
face's edges) 9,746; `present_plane` at a corner (a lean or span sign
undecided) 4,739; both presented with the departure margin WHOLLY IN
BAND 1,282; no separator found 299. Units are in flight on each of the
three (`embed/strip`, `embed/straight-corner`, `embed/remeter`).

**The 38 asks that crossed after a refusal:** 5 crossings were found at
depth 2 to 5 at a fresh subtree root, 33 at the depth floor. A walk
that after its first refusal split only non-touching pairs would still
find 32 of the 38, and on the dear asks that is nearly the whole walk.

**What is not yet known, and decides what this unit is:** whether the
straddling distance pairs are the neighbourhood of a touching pair
whose certificate failed (near a shared edge the distance goes to zero,
so cells beside the shared cell straddle at every depth; then they are
the local failure one level removed, and the unit is what a failed
touching pair owes its neighbourhood), or faces that truly come within
the band of each other off their shared cells (near-tangent
placements; then the unit is about a separation that never leaves the
band's edge, which is a reading of the band and not of a split's
ground). Asked of the gate lane on 2026-11-21.

## Measured (2026-11-25): where a refusing ask's splits and seconds go on main

**Instrument.** `origin/embedwalk/split-ground-instrument` at `46dac1094`
(not for merge): `origin/main` at `c85571202` plus
`crates/topo/src/split_ground.rs`. The gate's ask is ported, not merged:
a log-and-pass hook in `AtRestBody::validate` asks
`clearance::embedded` of each solid against itself at the gate's
budget (depth 40, 65,536 cell pairs) and drops the verdict. Every
split the walk makes is recorded with its reason, its parent, its
depth and the seconds spent classifying its pair. A touching pair's
reason is the departure exit and each face's presentation state, with
the `Unpresented::Local` site by line. A distance pair's reason is its
separation's class and whether its faces share a cell. No verdict
changes. Scripts, filters, logs and the summary are in
`measure/split-ground/` on that branch.

**Population.** The gate lane's own population
(`measure/gate/walk-population.json` on `origin/embed/gate-measure`):
its 4 dearest tests (`dear`) and its 39 depth-floor tests
(`m5_floor`). They were run on main with the hook on, and every one
passed. That gave 2,107 gate asks: 252 refusing, 41 crossing and 1,814
embedded. Every refusing ask comes from the hook. The 17 refusing asks
of the dear tests are the bodies of the 13 dearest asks measured on
2026-11-21. The 235 refusing asks of the floor tests come from 17 of
those tests.

**The dear asks are gone.** Each dear test's gate asks, in all, and its
test time: two donuts in general pose 6.3 s (test 88 s; 1,930 s
before); a ball against a donut 4.8 s (35 s; 999 s); a ball and a rod
against a donut 3.4 s (24 s; 448 s); a ball round the donut's axis
2.1 s (20 s; 311 s). Every refusing dear ask refuses at depth 0 or 1,
after 0 to 3 splits, on `Uncertified { arm: Departure }` from
`stuck-both`. Both parts are `Structural` (sphere or cylinder against
torus), which is a reason no smaller cell changes, and the engine
already refuses on it at once. The cost on 2026-11-21 came after the
first refusal, and "an ask stops at its first finding" removed it.

| group | refusal | asks | secs | splits | median refusal depth | least share of the ask's time before its refusal |
|---|---|---|---|---|---|---|
| dear | `Uncertified`, stuck-both (Structural + Structural) | 17 | 7.80 | 14 | 0 | 0.999 |
| floor | `Uncertified` at the depth floor, both parts `Local` | 170 | 4.37 | 66,224 | 40 | 0.987 |
| floor | `Budget(Depth)`, a distance pair straddling (faces share a cell) | 53 | 2.25 | 36,895 | 40 | 0.990 |
| floor | `Uncertified` at the floor, `Local` + `P-side` / `P-point` | 10 | 0.62 | 4,058 | 40 | 0.994 |
| floor | `Uncertified`, stuck-both | 2 | 0.00 | 30 | 0 | 0.988 |

Every refusing ask has exactly one refusing face pair, since the walk
ends at its first finding. At least 98.7% of each ask's time precedes
its refusal, the rest being the report. The question "seconds after the
first refusal" therefore has no content on main.

**Splits of the 252 refusing asks by reason (107,221).**

| reason | splits | child as its parent |
|---|---|---|
| touching, both parts `Local` | 61,380 (57.2%) | exact repeat 22,166; both still `Local` at another site 30,653; changed 80; root 8,481 |
| distance, `lo ≤ 0`, faces share a cell | 37,231 (34.7%) | exact repeat 34,391; changed 2,840 |
| distance, `lo ≤ 0`, faces share nothing | 4,271 (4.0%) | exact repeat 3,938; root 333 |
| touching, `Local` + presented (`P-point`, `P-side`, `P-corner`, `P-curve`) | 3,585 (3.3%) | mostly exact repeats |
| touching, both presented, departure margin straddling | 602 (0.6%) | exact repeat 564 |
| distance, `0 < lo` in band | 150 | — |

None of these splits has a separation or margin wholly in band. The
`Local` sites, on either side at depth ≥ 30 on the refusing pair, are
all `present_plane`:
- 6,404 the two-line corner's convexity undecided;
- 889 the same, decided reflex;
- 1,188 three edges with no sliver end;
- 320 two edges that do not meet;
- 288 the catch-all, a cell holding 4 edges and 2 to 3 vertices.

At the floor itself, 142 of 170 refusals are both cells failing the
corner's convexity undecided.

**Per refusing face pair: a reason no smaller cell changes.** Read off
the code: `present_plane`'s corner convexity (`lean(n1, bv)`,
`lean(n2, a)`) is read from the two edges' whole segments, never from
the cell. So a cell that holds the corner fails it the same way at
every depth, whether the lean is decided reflex or undecided, and every
smaller cell that holds the vertex holds the same two edges. Count that
failure with a `Structural` part and a margin wholly in band or decided
zero.
- **Floor asks:** 233 of the 235 carry one, first at depth 2 to 5
  (on the refusing pair, depth 2 to 7). Of the 233 floor refusals,
  210 carry it in their own lineage, 188 of them from depth 2 to 6.
- **What comes after:** 77,034 of those asks' 106,900 splits (72%)
  and 5.2 of 7.0 walk seconds (75%) follow that depth.
- **Dear asks:** the reason is the refusal itself, at depth 0, with
  nothing after it.
- **Refusing at the parent's exact reason:** no reason repeats exactly
  below a stable depth. The median depth below which every split of
  the pair repeats its parent's exact reason is 37, and what follows it
  is 5,176 splits and 0.31 s. The site moves from cell to cell while
  the failure stays `Local`.

**The open question: the straddling pairs are the neighbourhood of a
touching pair whose certificate failed.**
- 37,381 straddling distance splits on refusing asks are on faces that
  share a cell, and every one has a touching ancestor: 28,192 on the
  refusing pair and 9,189 on another pair. Most (28,557) are more than
  3 generations below their nearest touching split, and only 2,013
  have a sibling that splits touching. They are cells beside the
  shared cell that never hold it, so `Near::of` reads them not
  touching, and their enclosures overlap at every depth.
- Their separation never narrows toward a positive value in band. `lo`
  stays at or below 0 to depth 39, and the median `hi` shrinks with the
  cells, from 2.1 at depth 0 to 4 down to 7.9e-3 at depth 35 to 39.
  No split anywhere has a separation wholly in band.
- The 53 `Budget(Depth)` refusals are such pairs reaching the floor,
  and 43 of them carry the corner failure in their lineage.
- Faces that share nothing straddle in 4,271 splits, all off the
  refusing pair. All of them are decided by depth 24; none reaches the
  floor, and none refuses.
- No face pair in this population comes within the band of another off
  its shared cells.

So on main the unit is what a failed touching pair owes its
neighbourhood: a touching failure no smaller cell changes refuses its
cell typed at once (here the corner, first seen at depth 2 to 5). Cells
beside a shared cell, which no distance test parts, then inherit that
refusal rather than being halved to the floor. On this population the
saving is 72% of the floor asks' splits. The whole of those asks is
7.2 s, against the dear asks' 7.8 s refused at once, and the dear asks'
cost is now the pair's own certificate at the root. That last point is
read off the timing, not traced: 0.3 to 0.5 s each on 0 to 3 splits.

**Demonstrated:**
- the refusal classes, depths and times above;
- that the first refusal ends every refusing ask;
- the splits by reason and their child-versus-parent relation;
- the depth at which a reason this section counts as fixed first
  appears, and what follows it;
- that every straddling pair on faces sharing a cell descends from a
  touching split, with `lo ≤ 0` and a shrinking `hi`;
- that no share-nothing pair reaches the floor.

**Read off the code, not demonstrated:**
- that the corner's convexity is cell-independent (the lean's
  inputs), and so that refusing at it is safe for the cell holding the
  vertex;
- that the straddling cells sit beside the shared cell rather than on
  it (from `Near::of` reading them not touching while the faces share
  a cell);
- where a dear ask's root seconds go.

The corner failure is the dispatched row
`work/embed/a-plane-faces-corner-that-is-not-strictly-convex-is-read-at-no-depth.md`,
which already states that every smaller cell holding the vertex fails
it. This run is its census.

**Reproduce:**

```sh
git fetch origin embedwalk/split-ground-instrument && git checkout FETCH_HEAD
S=~/.local/share/cad-work/<lane>-scratch measure/split-ground/run.sh topo mesh sweep editor-core
python3 measure/split-ground/sg-summ.py $S/log     # reasons, lineage, sites, fixed depth
python3 measure/split-ground/sg-tables.py $S/log   # the refusal-class table
```
