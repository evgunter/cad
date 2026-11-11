---
id: main-red-the-stated-wrong-general-rows-refuse-chart-image-unavailable-in-the-slow-set
kind: issue
title: Main red: the two stated-wrong-general rows refuse ChartImageUnavailable, and only the slow set runs them
status: closed
opened: 2026-11-11
branch: pcert/stated-wrong-general-ending
pr: 6833
closed: 2026-11-11
priority: P1
cost: E
---


Found by the RESTREAD lanes on PR 6816 (`restread/self-touch-lunes`);
filed by the RESTREAD orchestrator. Neither PR 6816 nor RESTREAD
caused it.

## What

Both rows
`a_bending_carrier_on_a_spline_wall_takes_the_general_lane::a_stated_wrong_general_row_keeps_its_term_{f64,interval}`
(`crates/topo/tests/a_bending_carrier_on_a_spline_wall_takes_the_general_lane.rs`)
fail with `ChartImageUnavailable { chart: Nurbs, carrier: Circle }`:

    cargo nextest run -p topo --test all -E 'test(/a_stated_wrong_general_row_keeps_its_term/)'

They fail run alone, in their own process. They fail identically at
`b125b84b30` and at `4d193f273d` (main), so they are red on main.

## Why the PR gate is green

`df2a4a7693` ("the crossing and stated-row rows join the slow set")
put them in the slow set. CI's gate runs `--profile ci`, whose
default-filter in `.config/nextest.toml` excludes the slow set, so
`cargo nextest list --profile ci -p topo --test all` lists neither row.
Only the default profile, and so the nightly, runs them. The rows were
written as `a-minted-refusals-provenance-is-guarded-only-by-its-doc`'s
witnesses (that row's "Rows:" line), so that guard is currently red,
not held.

## The fix

Decide whether the rows or the code is wrong: the row says a `Stated`
image keeps its term, and the certification ends instead in
`ChartImageUnavailable`. Then make both green in the default profile.

## Closed

Bisected to PR 6815's merge `dcffc387` (its first parent `cbc0d381`
is green). The rows were right about the code and wrong about where
it ends; nothing in `src/` changed.

- **At a build, `ChartImageUnavailable` is the stated row's own term.**
  Since 6815 the edge door certifies a stated image on a spline chart
  through `certify_stated_image`, the door the mint certifies it
  through, and `mint_refusal` reports that certificate's
  `FittedLimb { HullValueChart }` in the edge door's vocabulary. It is
  the refusal of the description's own image, decided, never "not
  yet", and it comes before anything is stored. Nothing was pre-empted:
  the limb refusal the rows were written to see is the same
  certificate's, read one door earlier.
- **So the mint's provenance gate is reachable only at rest.** The
  edge door and the mint certify a stated image through one door at
  one band, so a stated row the build admits, the mint admits too. A
  `Stated` image reaches the mint's relabel sites only once something
  moved under it. The rows now witness both: the build pins
  `ChartImageUnavailable` exactly, and at rest the true row is stated,
  then its wall is re-parametrized under it (`ring_split_at`: the same
  wall, the interior knot moved, so the carrier still lies on it). The
  mint ends that in check 3's own `ResidualExceeded { MapResidual }`.
- 6532's M5 (the provenance gate dropped, applied at both relabel
  sites) reds both rows with `NoGeneralLane { gap: Image }`.
- **The f64 row leaves the slow set; the Interval row stays.** On
  hosted CI (run 43022130783, default ε) they took 0.352 s and 0.467 s.
  The f64 row clears the 0.5 s bar for a named test, so it now runs on
  every PR whose closure reaches topo, and M5 reds it. The Interval row
  sits within runner noise of that bar, so it stays in the slow set.
  Locally both cost more (0.8–1.7 s at ε 1e-9 and 1e-6, up to 5.7 s at
  1e-12), but the bar is the hosted default-ε reading. The Interval
  row's gap is noted on
  `a-minted-refusals-provenance-is-guarded-only-by-its-doc`, and the
  case is evidence on green's
  `a-src-change-that-moves-a-decide-site-skips-its-crates-recourse-roster`.
