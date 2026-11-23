---
id: a-far-origin-rod-cut-escalates-its-mapped-source-at-eps-1e-12
kind: issue
title: At ε 1e-12 the far-origin rod's cut by a brick escalates carrier_matches_mapped_source (red on main)
status: closed
opened: 2026-10-10
priority: P2
cost: M
closed: 2026-10-12
refs: [no-door-rejects-geometry-past-eps-f64-resolution]
---


`crates/sweep/tests/parallel_cylinder_join.rs`'s
`a_tipped_rod_whose_origin_is_stored_far_joins_along_its_rulings` fails
under `CAD_TOLERANCE_EPS=1e-12`. It fails on main (origin/main at
`4868268ba4`, measured 2026-10-10) and on `intent/s4-e-glue-on-zero`
alike, and passes at the default ε and at 1e-6.

The failure is in the fixture itself, not in the scene under test.
`cut_and_tipped` (`parallel_cylinder_join.rs:383`) intersects a rod
whose wall origin is stored `far` along its axis with a brick, and that
intersect refuses:

```
CrossingInsertion { operand: A, edge: EdgeKey(3v1), source: Certification { error: Escalated {
  check: MappedSource, sample: 3, cause: Indeterminate { margin: 1.2533307724993392e-12,
  band: Band { zero: 1e-12, escalate: 1e-11 }, predicate: Some("carrier_matches_mapped_source") } } } }
```

The certifier's mapped-source residual (`geom-brep` `certify.rs` /
`mapped.rs`) lands just past the zero band at 1e-12. That is the far
stored origin's lever on a residual that scales with the wall's
parameter range. CI's ε-extra job runs only for the crates a diff
seeds, so the row has not run at 1e-12 since PR 4231 added it.

Either the residual's floor has to be read against the lever `far`
puts on it, or the fixture keeps `far` within what the band
certifies at 1e-12, with that limit stated beside it.

## Folded in: TINT's `tipped-rod-join-escalates-at-1e-12`

TINT filed the same failure on 2026-10-07: the BAND lane saw it running
`CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep`, identical on
`origin/main`, with the same margin of 1.2533e-12. ENCL's nightly triage
flagged the pair, and that row is folded in here because TANG's PR 4231
added the row. Its framing of the fork: either the check's lever must
absorb the stored origin's distance, or the row is eps-sensitive and
wants a loud skip at that row.

## Closed (2026-10-12, TANG)

The check is right. The pose sits outside D4 ¶4's size range at
1e-12, and the row now stands it down there.

The escalating edge is the long rod's strut after its first split
(z ∈ [−1e5, 0.5]), not the final cut rod. At sample 3 the carrier
evaluates to `z = −62499.8125` and the description to
`−62499.81249999999`. That is one ulp at |z| ≈ 6.2e4: 7.28e-12, the
margin main reports now, and 7× ε. Past `2⁵²·ε` (about 4.5 km at
1e-12), two f64 evaluations of one point can differ by more than ε.
No stored datum, however minted, certifies there. An enclosure of the
residual with its rounding still straddles the band. Taking the floor
as an allowance would grow the tolerance per entity, which D4 ¶1
rejects. So `carrier_matches_mapped_source` escalating is the correct
verdict.

Under that ulp sits a smaller miss in the stored datum itself.
- The strut stores its split as `span = fl(0.5000025)` along
  `vec = (0, 0, 2e5)`.
- In exact rationals that datum misses the carrier by 1.228e-12 at
  sample 3 (the 1.2533e-12 first reported) and 3.28e-12 at s = 1. That
  is the parameter's quantum, `|vec|·2⁻⁵³`, at the lever `far` gives it.
- It would escalate on its own too, but it is not the floor that binds.

`a_tipped_rod_whose_origin_is_stored_far_joins_along_its_rulings` now
runs a pose only where `far·f64::EPSILON ≈ ulp(far)` is within the zero
band. It calls `test_utils::vacuity::stood_down` for any pose it drops,
and asserts that some pose runs. At 1e-12 it drops 1e5 and keeps 2 and
1000. At 1e-9 and 1e-6 all three run.

Filed:
- The unenforced range is on SCALAR's slate:
  `work/scalar/no-door-rejects-geometry-past-eps-f64-resolution.md`.
- The fact that a runtime-ε stand-down reaches no reader on the gate
  is added as evidence to CIW's
  `work/ciw/gating-nextest-jobs-discard-every-passing-tests-stdout.md`.
