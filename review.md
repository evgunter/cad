# Review of PR #3984, frozen head 8abb6e7931

Lane `reach-dual3984-r1`. Wall clock 18:51 – 19:07 UTC, 2026-10-03.
**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 3 · NOTE 3. No wrong body or wrong verdict found.
The gate was kept, so no body that main refused now builds. CI run 37141807328 is `success` on 8abb6e79 (head_sha checked).
The PR branch has since moved on to `af3e825` (run 37146244407, queued). That head is not reviewed here.

## Method, and a disclosed deviation
The brief says "the gate bypassed". I first patched an env-var bypass into `gate_operand_edges` (local only, never committed). The session's
permission classifier denied the next action as security-weakening, so I reverted that patch and did not pursue it. All
execution is therefore through `#[cfg(test)]` drivers of `sweep_direction` / `sweep_and_settle`, the way the PR's harness does it
(`probes/r1_probes.rs`, included locally as a child module of `planar_lane_carrier_rows.rs`). The full pipeline (join, ring,
zip, backstop) past the gate was **not executed**. Claims about those sites below are by inspection. Oracles are closed
forms: the spiric `P(v) = (off, √(ρ²−off²), r sin v)` and the PR's Bézier `y = x(2−x)`.
Glimpse: none. I read no other `analysis/reach-dual/*` branch and no PR comments or reviews.

## Claims
1. **Planar arm and curved arm: holds (DEMONSTRATED).** P1: a spiric cap (R=2, r=1, x=½) dips through a brick face
   `y=2.5` with same-side ends. Result: `CrossingCarrierUnsupported{A, arc}` at scales ×1e-3, ×1 and ×1e3, and `{B, arc}` with the
   operands swapped through `sweep_and_settle`. P6 is the same case re-posed (rotated 0.7 rad about an off-origin axis) and refuses the same way.
   P2: the spiric against a cylinder wall ρ=2.5 gives the same variant. The reverse direction, which the PR's rows never run:
   - P3: brick edges piercing the spiric cap give exactly the oracle's VF count at every scale and in mixed in/out cases (4/2/0/2 corners, ×2 for the two-sided sheet).
   - P4: the same against the NURBS sheet refuses `ArcLoopContainmentUnsupported{Uncrossable, Spline}`, typed, in both orders.
   **The brief's premise is partly wrong (style Q-dispatch).** It lists "the join's on-edge germ frame and the section ring
   lane" among the sites the PR converts. The PR does not touch them:
   - `boolean/join.rs:1036` answers `JoinDesync` ("the operand gates refuse the kinds");
   - `boolean/join.rs:1902` and `chord_join.rs:2430` answer `SectionInvariant` with the same reason.
   None reads a curve as a line: all three refuse. But none is typed as a carrier refusal, and none names the edge.
2. **Gate kept: no new body is built.** The gate's stated reason is not re-stated, and it is not measured (MINOR-2).
3. **No caller can read `Unlaned` as a line: holds by inspection.** Each of the 4 callers of `plane_crossing_lane` matches exhaustively, and
   `boundary_meets_circle_only_at` maps `Unlaned` → not clear.
4. **Mutants (`probes/mutants.sh`):**

   | mutant | killed by |
   |---|---|
   | M1: the planar arm treats `Unlaned` as a line | 2 PR rows |
   | M2: the curved arm goes back to `frontier()` | the PR's cylinder row |
   | M3: the lane maps spiric/NURBS to `Line` | 3 PR rows |
   | M4: the lane maps only the spiric to `Line` | `each_carrier_kind…` only |
   | **M5: the curved arm sends only the spiric to `frontier()`** | **no PR row survives it**; only my P2 kills it (MINOR-1) |

ε runs: the PR's rows, `splitting::classify::*`, `offer_rows` and refusal rows (63 tests) plus my probes pass at 1e-9, 1e-6 and 1e-12.

## Findings
- **MINOR-1** `crates/topo/src/boolean/planar_lane_carrier_rows.rs:253`. All three sweep rows use a NURBS carrier, so the curved arm's spiric
  half (`reduce.rs:2181`) is unpinned: M5 survives. DEMONSTRATED (mutant). P1/P2 are ready-made rows for it.
- **MINOR-2** `crates/topo/src/boolean/reduce.rs:416-419, 479`. The gate is kept, but its docs still give the reason the PR's own drive-by
  calls false ("rung-3 edges being what the zip MINTS"), and "no … pierce arm reads a spiric", which is now untrue.
  What the gate actually still protects goes unstated: the ring lane, the germ frame, `chord_join` run edges, the NURBS chord in `sectors.rs`,
  and the section area. By inspection.
- **MINOR-3** `crates/topo/src/boolean/join.rs:1036, 1902`, `crates/topo/src/chord_join.rs:2430`. These still say "invariant / the operand gates refuse the
  kinds". P5 shows that a spiric whose box clears the other operand passes both sweep directions silently (`Ok`, no contacts).
  So once the gate narrows, a far spiric edge on a cut face's run would reach the ring lane as a mislabelled kernel invariant, not a typed carrier refusal
  naming its pair. The germ frame is unreachable: a coincident edge's box always overlaps, so the sweep refuses first. The sweep
  table in the PR body does not list these sites. Partly DEMONSTRATED (P5); the rest by inspection.
- **NOTE-1** `crates/topo/src/splitting/classify.rs:942`. The row's doc says "Every arm is read against a plane the curve crosses". The spiric over
  `v∈[0,1]` has `y ≥ √(ρ²−¼) ≈ 2.9`, so it never meets `y=½`. The row is kind-keyed, so it still bites (M3/M4), but the doc's premise is false.
- **NOTE-2** `crates/topo/src/boolean/reduce.rs:1225`. A line no longer computes `touch_at_end`, so the cover-side escalation it used to raise for a line is
  now raised only by the line lane's own sides (`:1403`). The rows look identical; the change in behaviour is undisclosed. Unsure.
- **NOTE-3** The over-refusal is box-level. Every NURBS edge refuses at the first face of any partner (poison box), which is consistent with the PR body.

## Style (exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q8 partial; not Q7)
- Q1, `boolean/mod.rs:1601-1625`, sure. "This carrier kind has no lane" now has four spellings: `CurvedEdgeUnsupported` (gate),
  `CrossingCarrierUnsupported` (sweep), `SplitReduceError::CurvedEdgeUnsupported` (split) and `SectionInvariant`/`JoinDesync` (join).
  The new variant's doc says it is "the same fact" as the gate's. The recourse sentence is duplicated verbatim (`mod.rs:2847`).
- Q4, `boolean/boxes.rs:1774-1778, 1825`, likely. These still say the spiric box is "reachable only from its own rows… because the gate
  refuses". Past the gate, the sweep's soundness now rests on that box pruning (P5): a stale premise that something else now relies on. The same class
  appears at `contain.rs:248` and `chord_join.rs:2430`. Look also at every "the operand gates refuse" sentence.
- Q3, likely. The PR's "every row red without the fix" is true, but only on the NURBS axis (MINOR-1).
- Q6, sure. The residue filed as `reach/a-nurbs-edges-sector-departure…` and `section-area…` is scheduled. The ring and germ sites (MINOR-3) are not filed anywhere.
- Q2, unsure. The comment on `reduce.rs:479` predates this PR and justifies the gate by a fact the PR changed.

## Probe sources
`probes/r1_probes.rs` (P1–P6) and `probes/mutants.sh` (M1–M5). To run: add
`#[cfg(test)] #[path = "<abs>/probes/r1_probes.rs"] mod r1_probes;` at the end of `boolean/planar_lane_carrier_rows.rs`.
