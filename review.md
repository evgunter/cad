# Review of PR #3987, frozen head b8eb4dd0eb

Lane `reach-dual3987-r2`. **Verdict: NOT-MERGEABLE-AS-IS.** Counts: MAJOR 2 · MINOR 5 · NOTE 4. Glimpses: none (I read only my brief, the PR body via `get`, PR 3977's body via `get` to resolve the brief's "PR 3977", and the CI run list and job logs of this branch). Wall clock 18:51–19:40 UTC, 2026-10-03.

**CI on the frozen head:** there are no runs on b8eb4dd0e (the PR is `dirty`, so no `pull_request` run). The last run, 37143652417 on 290921b4c, is red: python `test_binding_census` (`AtRestBody` unlisted, fixed in the head commit by inspection) and the eps 1e-6/1e-12 step. Locally on the head I ran `ci` nextest for topo + sweep + verbs at 1e-9 (4218/4218 pass), 1e-6 (4218/4218) and 1e-12 (4217/4218; the one red is `contact9_side_codes::a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex`). I did not run editor-core, pncad or python, so any verdict on those is conditional on a green run that does not exist yet.

## MAJOR

1. **At `Dual<T>`, an inside-out operand ships its complement. This regresses from main.** Demonstrated by execution: `probes/dual-3987-r2/r2_dual3987_probes.rs::r2_inside_out_wedge_at_dual` on the head, against `r2_dual3987_main_probe.rs` on merge base 3cb7bc807.
   - Setup: brick (0,1)²×(0.5,1.5) and a clockwise wedge prism, `subtract_with` / `intersect_with` with flush declarations.
   - Main refuses both ops with `InsideOutOperand { operand: B }`.
   - The head's `Dual64::gate_at_rest_kept` (`crates/topo/src/props.rs:3330`) mints `NotRunAtThisScalar`. The door then answers ∖ = **0.03521** and ∩ = **0.96479**.
   - My own polygon clip gives the true values ∩ = 0.035214095513947, ∖ = 0.964785904486053. So these are wrong bodies, each with the other's volume, which is exactly the defect CLEAVE fixed.
   - Cause: the PR retires `gate_operand`'s check-7 read, which ran at every scalar on main, on the premise that the type carries it. A dual's `AtRestBody` carries no verdict (`ops.rs:533`/`:540`, where `one_solid` no longer gates).
   - The PR's own text is false here: `work/cleave/an-inside-out-operand-passes-the-boolean-gates-as-its-complement.md:66` says "unreachable from the door at any scalar".
   - Class: the stranded operand (claim 2) presumably reaches the classification invariant again at duals too. I did not test that.
2. **The merge order breaks the ratified sequencing, and the result is a wrong verdict on a CI row.** Demonstrated by execution: the 1e-12 row is red locally.
   - The unit's §Sequencing 1 says check 7's interval re-derivation lands *before or with* the gate.
   - Without it, the gate refuses a valid body with a **definite** `NegativeVolume`. My closed form is |det|·dip²/(6s²) = 14·(5e-10)²/6 = 5.833e-19 m³, derived independently of the row's `sliver()`.
   - The PR body names "the parallel reach/check7-interval lane". The brief names PR 3977. The tree records the dependency nowhere: no in-code note at `crates/topo/tests/contact9_side_codes.rs:280`, and the contact item is still `open`.
   - It resolves if #3977 merges first. As is, merging turns main red at ε 1e-12 (claim 5 holds: the row is not suppressed).

## MINOR

1. **The new gate's own subject has no row that can go red** (Q3, test gap). Demonstrated by execution with two mutants of `ops::gate` (`crates/topo/src/boolean/ops.rs:2702`):
   - M1 (tiers 1–2 only) reddens 4 rows: `the_result_gate_refuses_a_scaffold_at_rest`, 2 contact9 seam rows, and `verbs_germarms`.
   - M2 (main's gate exactly: tiers 1–2 plus the scaffold fence) reddens **only** `verbs_germarms`, at 1e-9, 1e-6 and 1e-12. Its payload under M2 is the backstop's `VolumeUnmeasured(RingOnCurvedFace)`: the same body, refused one step later.
   - So no door row pins a result that tier 3 refuses and main shipped. The farplane rows accept either outcome. No gate-level unit feeds `gate()` a lamina, a negative-volume body or a planar-residual body. Claim 6 holds only for M1.
2. **The backstop's positivity arm is now dead behind the gate, and nobody owns its retirement** (`ops.rs:1714`, `:1735`).
   - Demonstrated by execution: mutant M3 (both `≥ 0` arms removed) reddens only the two backstop unit rows that call `volume_backstop` directly.
   - The unit item says the arm retires; PR 3977 says "the door unit retires it". This PR's Built section is silent and files nothing. Claim 3's "nothing is gated twice" fails here, at the editor seat (MINOR 3) and at `gate_operand`'s tiers 1–2 (filed).
3. **The editor seat re-gates every boolean operand, including a body the previous boolean just gated** (`crates/editor-core/src/eval/wire.rs:855`). By inspection.
   - The body defers this to "the later step where the evaluator carries kept bodies". No `work/` item names that step (grep of `work/` at the head), so the deviation is unscheduled (Q6).
   - Its cost is excluded from the PR's cost tables by construction ("runs outside the door"), so claim 4's op-time figures leave out the editor's real added cost.
4. **The verdict is conditional on CI that has not run on this head** (see above). The last run is red.
5. **The PR knowingly turns the nightly `dev-probe` k-lint row red** (108 flags). It is filed (`k-lint-reads-the-boolean-doors-tier-3-at-probe`), but merging a known red is an orchestrator ruling the PR asks for and does not have. By inspection of the body.

## NOTE

1. **Correction to the brief's claim 1.** The door does *not* run `gate_at_rest_declared`. The census half is stopped, disclosed and parked (`work/reach/boolean-door-runs-the-census-over-its-result.md`). So "never below tier 3′" is not claimed, and results that fail the census still ship, as on main.
   - Probe `r2_edge_touch_union_against_tier_3_prime`: the edge-touch and vertex-touch brick unions ship and *pass* tier 3′ over their own vv records.
   - The flush-offset union refuses `UndeclaredCoincidence` up front.
2. **E2E (claims 1 and 3).** Demonstrated by execution in `r2_rotated_boxes_every_op_both_orders_reused`.
   - Setup: random rotated boxes at ×1e-3, ×1 and ×1e3; ∪/∩/∖ in both orders; each result reused as an operand (−C and ∪C).
   - Each result was checked against an analytic inverse-map oracle with 30 In and 30 Out `point_in_solid` samples, plus a 200k-sample Monte Carlo volume.
   - Results: 308 / 192 (+36 typed in-band refusals at 1e-6, ×1e-3 scale) / 300 results at ε 1e-9 / 1e-6 / 1e-12, **0 wrong**.
3. **The type fence holds (claim 3).** Demonstrated by compile failure, in the `compile_fail` module of the probe file:
   - `&mut *at_rest` and `describe_as_intersections(&mut clone)`: E0596;
   - a struct literal: E0451;
   - `AtRestBody::not_run`: E0624.
4. **Claim 2 holds at f64.** The stranded split top refuses at `validate` with findings on face 7 and its own vertices 5, 6, 8 and edges 9, 12, 13, at offsets 20ε, 1e3ε and 1e5ε (`offer_rows_stranded_probe.rs`). The unit says "49 fixtures" and the PR measures 38 tests and 151 calls; the two counts are not reconciled.

## Style (questions exercised: Q1, Q3, Q4, Q5, Q6; Q2 and Q7 lightly; Q8 not done)

- `crates/topo/src/boolean/offer_rows.rs:1592`: the stranded row asserts finding *kinds* only, never that the named face and edges are the stranded half's. The claim it carries is "naming its own entities". *likely*
- `crates/topo/src/boolean/ops.rs:179` against `docs/DESIGN.md:287`: DESIGN calls an `AtRestBody` tier-3′ ("passes tier 3′ against its own declared contacts"). `BooleanBody.body` holds one that is tier 3 only, so the one type name now covers two bars, and nothing at a reader's site can tell which. *likely*
- Class sweep for MAJOR 1: every `AtRestPolicy` gate absent at duals now guards something main guarded at every scalar. Look at shell's adoption and at `gate_operand_pairs` (`reduce.rs:375`). *likely*
- Q1: there are four spellings of "finish a body" — `AtRestBody::validate`, `T::gate_at_rest_kept`, `test_support::finished` (which panics) and the editor's `finished_operand`. At certifying scalars the first two are identical. *unsure*
- Inside the door, the sphere re-cut re-charts operand clones that the kept verdict never saw, so the API fence does not cover the door's own internals. Not exercised. *unsure*
- Q8: `ops.rs` (about 4000 lines) and `validate.rs` were not read end to end. Only the touched regions and the type's docs were read.

Probe sources are in `probes/dual-3987-r2/`.
