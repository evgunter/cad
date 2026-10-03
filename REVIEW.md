# Review — PR 3967 (frozen head `e0be3b56`, merge-base `82b9ceb2`)

**Verdict: APPROVE.** MAJOR 0 · MINOR 0 · NOTE 6.

Probes (`#[ignore]`d, release, default ε): `topo/tests/review3967_probe.rs`,
`sweep/tests/review3967_battery.rs`, `editor-core/tests/review3967_docm2.rs`.

## Claims

**1. Abandoning an in-band arm is sound — HOLDS.**
- *Argument.* The arm is `sin(r_k, n) · extent` (`containment.rs:478`). It depends on q only
  through `extent = max|p − q|`. The three axes are schedule members, so some member has
  `sin ≥ √(2/3)`. An in-band arm on every member therefore needs `extent ≲ 1.3e-8`, which
  puts every vertex within the band of q, and the boundary pre-pass (`containment.rs:400`)
  answers that case first. Abandoning only removes candidate rays. Every verdict is still
  read off a member whose arm is definitely positive and whose ray readings are all definite.
- *Arc walk, by reading the code.* Its answers cannot change: main already skipped the
  in-band arm (`ArmBand::Retry`). Only its exhaustion refusal changes (NOTE-3).
- *Executed.* `prism_ops` under a rotation putting a cap or wall normal φ ∈ {0, 1e-12 … 1e-3}
  off each of the 16 members; square, 1e-4 sliver, 1 × 1e-4 thin, concave L with 1e-4 arms;
  q at edge midpoints, vertices and 3e-8 along edges, offset δ ∈ {0, 1e-11 … 1e-5}; oracle =
  signed distance in the local frame.
- *Result.* 2 488 320 queries per lane. **0 wrong on head, f64 and Interval.** On main, 3 692
  answers are `Esc(point_in_loop_arm)`; on head every one of them is `In` (1 372) or `Out`
  (2 320), and all are oracle-correct. They come from the in-band arm configurations: square
  and thin at φ = 2e-9 and 5e-9, sliver and L at 5.2e-5, L at 1e-6. The nearest changed answer
  is 1.2e-8 from the boundary; pre-pass outcomes are identical on both trees.

**2. No consumer's answer changes wrongly — HOLDS.**
- *Containment.* As in claim 1: 3 692 changes, all refusal → correct answer.
- *Tier 3′ through `outcome`.* My battery covers 12 profiles × 11 near-flush turns (1e-4°,
  −3e-4°, 1e-3°, −2e-3°, 6e-3°, 0.02°, 45.0005°, 44.998°, 90.001°, −5e-4°, 135.003°) × 48
  shears × 4 ops, which is 25 344 booleans. **0 lines differ** between main and head, and
  `t3p=false` occurs 0 times on both.
- *Caveat.* No pose here reaches an in-band arm: it shows no regression, not a gain.

**3. The docm2 ε/16 re-baseline is a true gain — HOLDS.**
- *Independent check.* The certified Interval union at ε/16 has the f64 evaluation's counts
  (f6 e16 v12). Every f64 vertex lies inside an Interval vertex enclosure; the widest is
  2.5e-10. The volume enclosure [3.999999998, 4.000000002] contains the f64 volume, 4.
- Main's refusal was an arm enclosed in `[0, 1.12e-9]`: a member parallel to the normal,
  widened — about the member, not the part. ε/64 gives the same picture.

**4. The tests can go red — HOLDS for revert; FALSIFIED for "accept".**
- *M1: abandon reverted to escalate in `walk_schedule`.* All three go red:
  `an_in_band_schedule_arm_takes_the_next_member` (`review_m3_pr3_pil.rs:445`), the census row
  (`join_rc_probes.rs:164`), and the docm2 ladder at ε/16 (`docm2_part_interval.rs:206`).
- *M2: the in-band member's ray is read, not abandoned.* **Nothing goes red**: all topo rows,
  both sweep rows, the docm2 ladder (ε/16 is green) and my probe, which finds 0 wrong. See
  NOTE-1.

**5. The sweep is complete — HOLDS for this PR's class, with one misreport.**
- `order::in_plane_frame` (`order.rs:96`) already skips in-band members.
  `chart_region::point_in_polygon` (`chart_region.rs:2920`) escalates on ray-level side and
  advance; filed as `chart-region-polygon-walk-refuses-on-a-ray-level-margin`.
- *Misreport.* The PR body calls `boolean::solid_contain`'s 3-D sweep "OK". It is not. A
  cylinder wall's `bool_wall_trim` at a ray's hit reaches `cast_ray` as `RayFault::Fatal`,
  through a bare `?` (`solid_contain.rs:4987`). That is the plane arm's `RayFault::at_hit`
  shape (`:4924`), read the other way. It is already filed as
  `point-in-solid-curved-arms-read-the-band-before-the-face` (P3); not new (NOTE-2).
- No walk abandons a q-level reading; both pre-passes still escalate.

## Findings

- **NOTE-1 — nothing tells abandoning from accepting** (`containment.rs:484`). M2 survives
  every row and my 2.5M-query probe. In f64, normalising a projection of in-band length still
  gives an in-plane direction good to about 1e-7. In Interval, the poisoned enclosures abandon
  the ray downstream anyway. So the in-band arm appears to guard conditioning, not soundness.
  The PR's premise is unaffected, but no row pins the choice it makes.
- **NOTE-2 — the PR body's sweep calls `solid_contain` "OK"** (`solid_contain.rs:4987`). Its
  wall-trim readings at a hit are Fatal, and are already filed (claim 5). The body should name
  that row rather than clear it.
- **NOTE-3 — the arc walk's refusal category moved, unpinned** (`containment.rs:1827`). An exhausted arc walk with an in-band arm now refuses `Escalated(point_in_arc_loop_arm)`,
  not `RayExhausted`; through `contain.rs:71` the census's `CensusUnsupported` becomes
  `CensusEscalated` (`census.rs:1446`). Disclosed in the body. No test names
  `point_in_arc_loop_arm`, so dropping the arc walk's `skipped` would go unseen.
- **NOTE-4 — the new rows bite at the default ε only** (`review_m3_pr3_pil.rs:424`,
  `join_rc_probes.rs:127`). By arithmetic, not run: at ε = 1e-6 the 5e-9 arm is below
  `zero`, at 1e-12 definite, so there the topo row cannot see M1. The default row does.
- **NOTE-5 — the coarse-ε allowance accepts any `Escalated`** (`join_rc_probes.rs:139`). Any
  predicate is admitted, including a regression that refuses on the arm inside the boolean's
  own classification.
- **NOTE-6 — older defects the battery saw (not this PR; identical on main), for the
  orchestrator to file.**
  - Three intersections at 1e-4° ship a body that is not a legal operand (`operand=false`,
    v ≈ 0): `eBot 0.0001 0 0.05 I`, `eLeft 0.0001 0.05 0 I`, `eRight 0.0001 -0.05 0 I`.
  - 1 541 kernel-invariant refusals (`JoinDesync` 860, `FanStartMismatch` 617, `PairingMismatch`
    44, `ClassificationInvariant` 20), clustered at eLeft/eBot near-flush turns
    (`review3967_near_flush_battery`).

## Style

Exercised: Q1–Q4, Q6, Q7. Q8 partial (`containment.rs`, 2 327 lines: structure, module doc and
both walks, not end to end). Q5 not exercised.

- **likely** — `order.rs:96-112` is a third copy of the schedule-arm loop. It uses the same
  projection and the same `Margin::levered(d.norm()/r.norm(), arm)` as `walk_schedule`
  (`containment.rs:469-489`), but keeps the **last** diagnostic, where `Abandoned` keeps the
  **first**. That is the same rule spelled two ways. The PR's sweep looked at it and called
  it OK.
- **likely** — `walk_schedule` takes `skipped: &mut Abandoned` as an out-parameter only so that
  each caller writes the same two-level fold,
  `abandoned.refusal(|| skipped.refusal(..))` (`containment.rs:446`, `:1828`). The fold is
  hand-written twice, and its precedence between the two holders is a convention.
- **likely** — `Abandoned`'s doc (`ray_parity.rs:80`) now calls "whether its schedule member
  projects into the plane" a fact about one ray. The margin is levered by `extent`, which
  depends on q. The argument for why that is still safe (claim 1) is written nowhere.
- **unsure** — the module doc (`containment.rs:37-40`) and `polygon_walk`'s comment (`:417`)
  still say that a collapsed loop "ends in `RayExhausted`". On the Interval lane, an arm
  enclosure that straddles the band now ends in `Escalated(arm)`.
- **unsure** — `containment.rs:61` lists `point_in_loop_arm` as the "skip gate". The sentence
  at `:71`, "Both are facts about one ray", still counts only side and advance.
- **likely** — the reflowed doc at `docm2_part_interval.rs:168-170` leaves a one-word line
  ("narrower rung certify. Each").
- **likely** (from the PR's own table) — three of the census row's four poses are green on
  main since PR 3866; only `sqQ1 -0.25 -0.75 U` is red-able here, which the row's doc omits.

REVIEW COMPLETE
