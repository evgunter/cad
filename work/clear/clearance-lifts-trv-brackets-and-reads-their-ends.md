---
id: clearance-lifts-trv-brackets-and-reads-their-ends
kind: issue
title: The clearance engine lifts brackets through from_certified and never refuses a Trv one; 35 of injective.rs's and 11 of clearance.rs's endpoint reads decide on brackets that may not certify
status: closed
opened: 2026-10-29
priority: P1
cost: M
closed: 2026-11-01
---

Found by SPLICE's census lane (`splice/census-reds`, PR 6040) while
dispositioning `geom-core` `certified_endpoint_census`
(`crates/geom-core/tests/certified_endpoint_census.rs`, `ROSTER`). That
census counts `crates/topo/src/injective.rs` at (52, 1) and
`crates/topo/src/clearance.rs` at (23, 6) on main `0011f58a`, and both
entries are pinned at main's counts with a note naming this row. The
census checks counts, not soundness. Closing this row means making each
read marked unsound below sound, re-pinning the counts, and replacing
the two notes with dispositions. Line numbers are at `0011f58a`.

## The shared cause: an ungated lift

Three sites lift values through `Interval::from_certified`:
`clearance.rs` `boundary_of` (`:2967`, `:2997`, `:3028`), `window_of`
(`:2746`) and `RegionMemo::ask`. The values are vertices, span ends,
pcurve points, iso values, harmonic coefficients and carriers. At
`T = Interval`, `from_certified` keeps a refused input's ordinary
endpoints under a `Trv` decoration (`crates/geom-core/src/interval.rs`
`from_certified`, `:317`), and nothing downstream refuses it.
`T = Interval` is a production path: `lane.min_separation` in
`editor-core`'s `wire.rs`, the `m10_6_clearance_assertion_interval` row. The
same code asks the question in some places and not in others:
`extent_of` (`clearance.rs:3083`) asks `is_certified()` of the lifted
pcurve ends, and `iso_bounded_domain` asks it of knots and joins.

**R1, the minimal fix:** refuse at the lift. In `boundary_of` and
`window_of`, return `None` / `Unsupported` for any lifted value that
does not certify (`certified_bracket()`, or `from_certified` then
`is_certified()`).

## clearance.rs: 17 reads outside an asking function

**Unsound (11):**
- `min_separation` `:2271`, `:2305`, `:2306` (3) and `Bracket::pair`
  `:2455`, `:2466`, `:2485` (3). The poison check is
  `sep.lo().is_finite() && sep.hi().is_finite()`, not `is_certified()`,
  so a `Trv` separation reaches the reported bracket's
  `floor`/`lo`/`hi`. Fix: R1, plus `|| !sep.is_certified()` →
  `PoisonEnclosure`.
- `Bracket::witness` `:2422` (1). `stations.hi()` becomes the incumbent
  upper bound without being asked. Fix: skip a station separation that
  does not certify.
- `Bracket::overflow` `:2499` (1). An abandoned task's `sep` is never
  checked, and `floor.min(NaN)` drops a NaI lower end, so a poisoned
  pair falls out of the lower bound. Fix: poison, or floor at 0, on a
  refused `sep`.
- `window_of` `:2737`, `:2762` (2). The `along` superset reads a lifted
  origin and direction (R1). Once R1 holds, the cone `slant` read is
  covered by the window's own finiteness refusal.
- `boundary_of` `:3033` (1). The lifted span's ends feed `carrier.eval`
  and `Aabb::from_points`, which reads `Bounds` and refuses nothing
  (R1).

**Sound (6):** `split_axis` `:3329`, `:3342`, `:3343` (3) chooses an
axis. `coincidence` `:1737` (1) and `meeting_stations` `:1846`, `:1852`
(2) seed searches whose results are verified.

## injective.rs: 51 reads outside an asking function

| category | reads |
|---|---|
| sound as it stands | 16 |
| sound once R1 holds | 23 |
| needs R1 and a local ask | 10 |
| unsound whatever the lift | 2 |

**Sound as it stands (16):**
- seeds and selections: `mid` `:573` (1); `graph`'s `inside` `:2532`
  and `lever` `:2541` (2);
- reads behind a funnel: `present_plane` `:1854`, `:1855` (after
  SPAN), `:1904`, `:1930`, `:1931` (after CORNER) (5); `Pieces::of`'s
  `wind` `:2456`, behind `exactly` (1);
- `from_bounds` of finite parameters: `monotone_run` `:2671`, `:2707`,
  `:2710`, `:2723` (4); `narrowed` `:2785` (1); `Curved::psi`'s shift
  `:2165`, `:2166` (2).

**Sound once R1 holds (23):**
- `overlap` `:668` (1), `covered` `:1000`, `:1007` (2), `vacuous`
  `:1223`, `:1224` (2), `present_curved` `:2377` (1);
- `graph`'s `rb` `:2559` (1), `runs` `:2604`, `:2623` (2);
- `Ends::before` `:2762`, `:2764` (2) and `Ends::after` `:2772`, `:2774`
  (2), `apart` `:2824` (1), `edge_piece` `:3281` (1);
- `present_chart`'s touch/reach `:1398`, `:1399`, `:1409`, rectangle
  sides `:1460`–`:1463` and apex `:1499` (8).

**Needs R1 and a local ask (10).** Each of these rebuilds an interval
from `lo()`/`hi()` with `from_bounds`, which mints a certified interval
from a refused one, or decides on one end without asking:
- `forward` `:960` (1): `delta.lo() > 0` picks the face side. Fix:
  `delta.is_certified() &&`.
- `along` `:985`, `:986` (2): `from_bounds(min lo, max hi)` feeds
  `covered`'s coverage claim.
- `present_chart`'s merge hull `:1437`, `:1439`, `:1444`, `:1445` (4).
- `Trig::psi`'s `over` `:2085`, `:2087` (2): `from_bounds(-m.hi(),
  m.hi())` goes to `is_zero`.
- `Curved::psi`'s `bound = (...).hi()` `:2183` (1).

**Unsound whatever the lift (2):**
- `present_plane` `:1834`: the `[e]` arm returns `Vacuous` on
  `y.hi() <= 0.0` before the SPAN funnel, and `y = (d·bv)/len2`, with
  `len2` possibly reaching zero, is a quotient that may not certify.
  Fix: `y.is_certified() && …`.
- `present_chart`'s sliver `:1569`: `at(lever.norm().hi())` mints a
  certified width from a lever that `rect_jet` may have refused, then
  decides `SLIVER == Zero` → `Vacuous`. Fix: keep `lever.norm()` as the
  interval.
- Also, not counted: the merge folds near `:1444` use `f64::max`/`min`,
  which at `T = f64` skip a NaN clip line instead of refusing it.

The per-read reading is the lane's own, from the read sites. Check each
claim before fixing; none has a failing test yet.
