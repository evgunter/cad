# Review of PR #3844, frozen head 7d72667dd1

Lane `reach-dual3844-r2`. **Verdict: NOT-MERGEABLE-AS-IS.** MAJOR 1 · MINOR 2 · NOTE 4. Wall clock 13:59–14:43 UTC 2026-10-02. No glimpse: I read only my own branch, the PR body (`get`) and the check runs. CI run 37016144797 is green on this head, and I re-ran topo and sweep (`ci` profile) at ε 1e-9, 1e-6 and 1e-12: all green except my own probe's profile builder at 1e-12 with s = 1e3.

## MAJOR

**M1. The declared-pair allowance lets a wrong component out that main refuses: MAJ-1's own wrong body, at every ε** (`crates/topo/src/boolean/ops.rs:1662-1680`, used in every `bound_holds` margin). Confidence: sure. DEMONSTRATED BY EXECUTION (`probes/reach3844_probes.rs` p1, `probes/reach3844_r2_probe.rs` e1).
- **The MAJ-1 fixture.** This is `volume_backstop_refuses_a_wrong_component_hidden_by_a_large_area`: a 2×2×0.1 plate whose result wrongly keeps a 3 mm cube (ΔV 2.7e-8 m³). With the plate tops declared one plane (true for plate ∩ plate), the backstop **passes** it under ∩ and ∖ at ε 1e-9 and 1e-6. With nothing declared it refuses. The pass/refuse edge is exactly ΔV = escalate × 4 m²: 0.99× passes and 1.01× refuses at all three ε. So the largest kept cube that passes has an edge of 3.4 mm at 1e-9, 3.4 cm at 1e-6 and 0.34 mm at 1e-12.
- **The PR's own real fixture.** In the rounded stack (sweep), the flush detector's declarations (8–9 pairs, side walls and fillets included) let a planted ∩ result through when it is B thickened by δ:
  - largest δ that passes: 1.4e-8 m (sunk pose: 4.0e-9 m);
  - so ΔV = 3.3e-7 m³ passes at 1e-9 (a **6.9 mm cube**), and 3.3e-4 m³ at 1e-6 (a 6.9 cm cube);
  - at scale ×1e3 and ε 1e-9, ΔV = 0.32 m³ passes;
  - with no declarations, δ/2 refuses in every case.
- **Why main refuses all of these.** Declarations never reach main's backstop, and main's arm 1 refuses any certified negative. On these dyadic and closed-form margins the interval re-derivation certifies (measured width 0 on the plates, checked exactly against rational arithmetic), so "nothing declared" is main's verdict.
- **What this costs.** The allowance is a sound bound on what a *correct* result may move. But it is a volume, so it forgives a defect of that volume anywhere in the body. With declarations present, arm 1's threshold falls to escalate × Σ(declared area), the same order as arm 2's escalate × lever, which is the magnitude-only posture MAJ-1 exists to reject.
- **How much headroom.** Its only live witness (the +1.2ε wedge) needs 1.74e-12 m³ against an allowance of 8.7e-10, which is 500× headroom.
- **The PR's argument against this** ("With nothing declared it is zero, so the MAJ-1 row still refuses") tests only the undeclared case, and that case is not the row's subject.

## MINOR

**m1. The same pair declared twice multiplies the allowance** (`boolean/mod.rs:3712-3715` refuses a repeat only under a *different* class; `ops.rs:1662` sums every entry). Confidence: sure. DEMONSTRATED (p1b): at ε 1e-6, the pair declared 1000 times passes a result 10 % too thick (ΔV 0.04 m³); at 1e-9, 10⁶ copies do the same.

**m2. The interval re-derivation is pinned by no row.**
- The mutant that skips it (refuse on the f64 sign, `probes/mutate.py` M3) leaves every row green: `boolean::ops::tests` `volume_backstop*`, `door_backstop_settled_residue`, `reach_continuation::declared_rounded…`.
- The flush-top intersect that the PR body and `crates/sweep/tests/reach_continuation.rs:706-710` credit to the re-derivation is actually rescued by the allowance: its 9 declarations cover the 2-ulp tie. Only M1+M3 together turns it red, with main's exact text (`got 11.892699081698725, bound …723`).
- So the new `QuadLane` field, `PastTarget::interval_volume` and the allowlist pin buy behaviour that no test can see degrade (style Q3). Confidence: sure, DEMONSTRATED by mutant.

## NOTE

- **N1. Mutants.**
  - M1 (allowance ×0) turns the `allows_what…` row and the residue row red.
  - M2 (×100), M4 (`encloses_material` dropped) and M5 (∪ arm dropped) each turn exactly one row red, the new unit rows, and nothing end to end.
  - M3: see m2.
- **N2. The tables, against my closed form.**
  - The sweep table reproduces at s ∈ {1e-3, 1, 1e3} and rotations {0, 0.3 rad}: ∩ and A∖B build to ≤ 4e-16 relative; B∖A and ∪ refuse `FallbackExtentUnsupported`.
  - On sampled grids of 315 points per result, `point_in_solid` disagrees with the oracle 0 times.
  - The residue table reproduces: ∪ at ±1.2ε is ±1.74e-12, B∖A matches to 3e-17, and sunk ±2ε refuses `JoinDesync`.
  - The tilt sweep also found what the PR does not list: sunk at ±0.5–1.2ε, ∩ and A∖B *build* and cross ∩ ≤ B and ∖ ≥ A−B by up to 1.74e-12 at −1.2ε. These are live witnesses of the allowance on two more arms. Standing at |tilt| ≥ 1.5ε, ∪ refuses `RestZipUnsupported`.
- **N3. The new arms never refused a correct body.**
  - 597 exact tight bodies at non-dyadic sizes (∪ = A+B, ∖ = A−B, ∩ = B): 0 refusals.
  - Stacked rounded unions at 3 scales × 3 rotations build to ≤ 7e-16.
  - The contact9 sliver rows are green at all three ε.
  - Each arm refuses a planted violation of 1e-15 to 1e-6 with nothing declared.
- **N4. The design stop is honest** (`ops.rs:2524-2526`). I reproduced the tier-3 table with temporary instrumentation on this head (topo, `ci` profile):
  - 710 results (677 f64 + 33 interval), 55 of which fail tier 3; the PR reports 687 and 55;
  - all 55 pass the backstop, where the PR reports 52 shipped. That gap is drift from `cd49025f`;
  - 3 have both operands passing tier 3, all `contact9_side_codes` (first error `ScaffoldAtRest`).
  - The allowlist gate passes at 15. The pin's argument holds by inspection: `closed_form` is reachable only through `QuadLane::certified`, which needs `CertifiedBounds`, and the duals' `gate_volume_backstop` is a no-op.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q6, Q7; Q5 lightly; Q8 not exercised: I read `ops.rs` 1415–1960 and the tests, not the whole 3.7k-line file)

- **Q1** `props.rs:1805` `closed_form_inputs` restates `face_flux`'s face/surface/loop resolution (`props.rs:1839-1864`, the same two `Corrupt` strings) instead of `face_flux` using it. `quad_lane.rs:141`'s plane/curved dispatch mirrors `face_flux`'s, so a fix that closes a rounding gap mints a second copy. Confidence: likely.
- **Q1** `ops.rs:1911` `encloses_material` is a third copy of the arm-1 → interval → open → refine ladder over `"volume_backstop_violation"`, beside `bound_holds`. The two `interval_volume`s (`ops.rs` free fn, `props.rs:666`) are two spellings of one name. Confidence: likely / unsure.
- **Q2** `ops.rs:1484-1510`, "the door's own error bound and the only one", is longer than the code it defends and asserts something nothing enforces. It also does not say the bound is a volume that forgives any defect of that size (M1). Confidence: likely.
- **Q3** `ops.rs:3555` declares `(top, top)` on identical cubes with the defect *on* the declared face. It cannot exercise the failing mode, a large declared face with a defect elsewhere. Confidence: sure.
- **Q4** The MAJ-1 row's doc (`ops.rs:3613-3623`: "must refuse however much boundary area it is smeared over"; "delete arm 1 and this test goes red") now holds only when nothing is declared, and the text was not touched. This is a code drift from an intended invariant, not doc rot. Confidence: likely.
- **Q7** `bound_holds` encodes its sides as `±1.0` floats and patches the refusal text with a post-hoc `bound = -bound` when the result sits on the large side. It reads awkwardly; an enum would make the side a type. Confidence: unsure.

## Probes (`probes/`; wiring in `probes/wiring.diff`)

- `reach3844_probes.rs` (in-crate, `boolean/`): p1 allowance hole, p1b duplicates, p3 far-origin interval width, p4 tight and planted bounds, p5 wedge tilt sweep.
- `reach3844_r2_probe.rs` (sweep `tests/all.rs`): e1 rounded stack (scales, rotations, `point_in_solid`, planted δ bisection), e2 tight unions.
- `mutate.py`: mutants M1–M5 of `ops.rs`.
