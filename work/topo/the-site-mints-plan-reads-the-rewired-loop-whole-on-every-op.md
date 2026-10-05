---
id: the-site-mints-plan-reads-the-rewired-loop-whole-on-every-op
kind: issue
title: The site mint's plan still reads the rewired loop whole on every op (presence, chart box, element), so N ops on one face cost O(N²) reads
status: open
opened: 2026-10-04
priority: P3
cost: M
design: true
refs: [euler-site-mint-re-walks-the-rewired-loop-on-every-op, split-edge-re-reads-the-faces-window-on-every-split, site-row-plans-re-mint-a-face-whose-walk-closed-short-of-a-member]
---


Found closing `euler-site-mint-re-walks-the-rewired-loop-on-every-op`.
Since the elements change (R, PR #4037), `site_rows` in
`crates/topo/src/pcurves.rs` derives and certifies only what a door
creates. The plan still reads every half-edge of each rewired loop on
every operator:
- the loop walk `Body::plan_site_rows` builds in `crates/topo/src/euler.rs`;
- `stored_rows`' presence reads and chart boxes, through `site_rows_from`;
- the chart box of every image, for the certification window;
- every element, for the winding.

So N operators on one minted face whose loop grows with each still cost
O(N²) reads. Measured on `cyl_wall_sheet` with N struts after N rim
splits (debug build, shared box): 0.008 / 0.025 / 0.086 / 0.351 s at
N = 25 / 50 / 100 / 200. That is roughly ×4 per doubling, at a third of
the cost before the change.

It keeps the P3 of the row it replaces: the reads are cheaper, but the
asymptotics that row's title named are unchanged.

A fix would keep a face's window and a loop's winding as data that an
operator updates by its delta rather than re-reads. The window is the
harder of the two, because it enters certification verdicts.

## The question this row holds

Every one of the four reads answers a question about the WHOLE face
or loop, so a plan that reads only its delta needs that answer carried
from the previous operator:

- **Whether the face re-mints** (`StoredRows::remints`): stores a
  row, and every gap is on a loop a null edge holds open. That is a
  count of gaps and of null-held loops per face.
- **The window** (`site_rows`' `boxes`, through `certify_walked`): the
  hull of every image the face holds after the surgery. It enters
  verdicts, not bits: `chart_arms_at` in
  `crates/geom-brep/src/pcurve_cache.rs` levers a cone's azimuth
  margins by the window's `v` reach, and trim containment decides the
  gap between the window and each box. So it must be the re-read hull
  exactly, not a superset.
- **The winding** (`walk_cycle`): an integer sum of the loop's elements.
- **Which loops a null edge holds open** (`held_open`, over every
  rewired half in `site_walks`).

The winding is invertible (a sum), so an operator can update it by its
delta. The window is not: a hull only grows by insertion. `mev` and
`mekr` only add images, but `split_edge` replaces a parent's box by two
children's boxes that may be smaller, a kill removes images, and `mef`
splits one face's images between two faces. Keeping the window exact
under those doors needs either a removable hull (per face, the four
extremes as ordered multisets, O(log n) per update) or a re-read of the
part that changed (O(part) for `mef`, which re-parents that part
anyway).

So every design that reaches O(delta) keeps new state across operator
calls. Three shapes:

- **(A) A derived row summary on `Body`**: per face the gap and
  null-held counts and a removable hull; per loop the winding. Every
  writer of `pcurves` or `joints`, and every door that moves halves
  between loops or faces, maintains it or invalidates it. This is new
  persistent state on a value D1 calls "a plain value: cheaply
  cloneable, serializable, diffable", and a stored sum beside C4's
  "a loop's lift is derived by summing elements". The guard the
  dispatch asks for, a debug assert comparing the cache to a re-read,
  costs the re-read: the workspace's `[profile.release]` sets
  `debug-assertions = true`, so it would restore O(face) per op in
  every build CI runs. It has to sit behind a feature instead, as
  `per-op-postcondition` does.
- **(B) The same summary, held only while a surgery scope is open**
  (`crate::surgery`, the ruling on
  `work/perf/d1-per-op-tier1-sweep-price`). A body at rest carries
  none. It is built on the first site mint of a face inside a scope and
  dropped at the close, and a clone starts without one, as
  `SurgeryDepth` does. Doors outside the delta-maintaining set
  invalidate it. The at-rest value is unchanged. This is the producer
  shape the quadratic needs: many operators on one face. But it is
  still a `Body` field, and it is as invasive as (A) inside a scope.
- **(C) Keep the re-read.** The one production caller that runs
  operators on minted faces is the fillet surgery
  (`crates/sweep/src/blend/surgery.rs`), with a handful per face. The
  quadratic binds only for a producer that grows one minted face by
  hundreds of operators, and none exists. The harness
  (`crates/topo/tests/site_mint_scaling.rs`, `#[ignore]`d) keeps the
  measurement one command away.

Two rows hang on the same answer. The split door's window read
(`split-edge-re-reads-the-faces-window-on-every-split`) is the same
read on a door that shrinks the hull. The membership proof that
`site-row-plans-re-mint-a-face-whose-walk-closed-short-of-a-member`
would add to these plans is a whole-loop read, which runs the other
way from this row.

**Measured** with that harness: `cyl_wall_sheet`, `n` rim splits then a
strut up the ruling at each split vertex, all inside one surgery scope,
so the per-door tier-1 sweep does not mask the shape. Shared box, so
read the shape only:

| n | splits, debug | struts, debug | splits, release | struts, release |
|---|---|---|---|---|
| 25 | 0.0031 s | 0.0043 s | 0.0007 s | 0.0009 s |
| 50 | 0.0069 s | 0.0120 s | 0.0013 s | 0.0022 s |
| 100 | 0.0163 s | 0.0499 s | 0.0032 s | 0.0072 s |
| 200 | 0.0532 s | 0.1491 s | 0.0084 s | 0.0246 s |
| 400 | 0.1322 s | 0.5926 s | 0.0258 s | 0.0692 s |

Outside a scope, each operator is its own door and pays the tier-1
sweep (O(body)). That sweep is about half of a release run's
instructions under callgrind (`Body::assert_euler_postcondition`), so a
producer that skips the scope stays quadratic whatever the site mint
does. Across that whole run, `Pcurve::chart_box` (a `sin_cos` per
harmonic image, per image re-read, per operator) is about a quarter of
the instructions, and `stored_rows` about a fifth.


## Adjudication (TOPO orchestrator, 2026-10-05)

(C) for now: the re-read stays, and the row is not dispatched. No
producer grows one minted face by more than a handful of operators
(the fillet surgery is the only one that runs operators on minted
faces), so the O(N²) shape costs nothing a user reaches today, and the
harness above keeps the measurement one command away. (A) and (B) both
add state to the body that D1 describes as a plain value, so they are a
design fork. It goes to the designers (and to Ev if it is Ev's) when a
producer that grows one face by many operators is built or planned. A
dispatch before then reopens this question rather than building (A) or
(B). `split-edge-re-reads-the-faces-window-on-every-split` is decided
with this row.
