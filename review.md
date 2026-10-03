# Review of PR #3964, frozen head 7d2cb52253

**Verdict: APPROVE-WITH-FIXES** — MAJOR 0 · MINOR 1 · NOTE 5. Wall clock 15:34–15:50 UTC, 2026-10-03.

The new row is a strict improvement on the old row at every ε where the old row could be green. I ran 11 kernel mutants under both rows at
1e-12/1e-9/1e-6. Every mutant that main's row caught at 1e-9 or 1e-6, the new row also catches. The new row also catches three that main's
row passed. One part of claim 3 is overstated: at 1e-12, limb 1 has a resolution floor that the PR does not disclose.

## Method (all DEMONSTRATED BY EXECUTION unless marked)
- Built at the frozen head with `CARGO_INCREMENTAL=0`. Ran the row at 1e-12, 1e-9 and 1e-6: green, and the PR's numbers reproduce exactly (limb 1 1.47/1.80/1.80 ulps;
  limb 2 0.306/0.385/0.316 floors; floor 3.2418e-14 at every ε). Main's row (`origin/main` copy) is red at 1e-12 on the stated assert, as claimed.
- `probes/mutants.py` holds env-selected mutants of `ssi::certify::nurbs_limbs` (limb 1) and `compose::tensor::coefficient_norm_bound` (limb 2).
  `probes/run_mutants.sh` runs the new row and main's row under each one. Raw output is in `probes/mutant_results.txt`. Kernel reverted.
- `probes/probe_rigid_scale.rs` builds the fixture at scale s ∈ {1e-3, 1, 1e3}, with translations 0..1e3·s and fields f·s for f ∈ {1e-12, 1e-9, 1e-6, 3.3e-11}
  (`probes/scale_results.txt`).

| mutant | 1e-12 new / old | 1e-9 new / old | 1e-6 new / old |
|---|---|---|---|
| limb 1 L1 norm of offset | red / red | red / red | red / red |
| limb 1 drops z (`√(x²+y²)`) | **red 4.47 vs 4** / red | red 23 / red | red / red |
| limb 1 + 1e-3·\|r.x\| (frame leak 0.1%) | **green 3.44** / red* | red / red | red / red |
| limb 1 + 1e-6·\|r.x\| | green / red* | green 3.48 / green | red / **green** |
| limb 1 ×(1+1e-3), frame-free | green 3.98 / red* | red / **green** | red / **green** |
| limb 2 per-coordinate fold (cell) | green / red* | red 30× / red ×1.0025 | red / red |
| limb 2 L1 norm per coefficient | red 1.49 / red* | red / red | red / red |
| limb 2 L∞·√3 | red (FLOOR pin) / red* | red / red | red / red |
| limb 2 + 1e-3·\|z\| | green / red* | red 2.2 / **green** | red / **green** |
| limb 2 + 1e-2·\|z\| | green 0.43 / red* | red / red | red / red |

\* Main's row is red at 1e-12 with no mutant, so its reds there carry no signal. CI `test` and lint are green on the head (run 37133402596).

## Findings
**MINOR-1 — claim 3 is overstated for limb 1 at ε 1e-12.** `crates/topo/tests/rigid_map_near_eps_plane_nurbs.rs:46,309`. DEMONSTRATED (table rows 2–3).
The PR says the row "goes red at every ε on a limb-1 frame-reading defect", and that holds only for the gross M1. A 0.1% frame-dependent leak into
limb 1 stays green at 1e-12 (3.44 ulps against the pin of 4). Dropping the z component is red only by 4.47 against 4, a 1.12× margin.
At 1e-12, 4 ulps of R is 1.5e-15 m, about 0.15% of the field. So limb 1 has the same sub-floor blindness the PR concedes for limb 2,
but the PR does not disclose it, and the filed SSI item covers limb 2 only. This is not a weakening against main: main's row was
unconditionally red there, and at 1e-9 both rows pass a 1e-6 leak. The fix is wording: correct claim 3, and record limb 1's 1e-12
resolution beside `ROUNDING_ULPS` or in the filed item.

**NOTE-2 — the limb-2 tolerance reads the floor it guards.** `:319` divides by the runtime `floor`, not by the pinned `FLOOR`.
A defect that raises the floor therefore widens its own drift tolerance, which is monotone in the wrong direction (Q3). DEMONSTRATED:
under the L1 mutant the floor rose 3.24e-14 → 4.06e-14 (×1.25, still under the pin) and the tolerance rose with it. The row was red anyway.
The pin caps the effect at ×1.3. `likely` worth noting, not fixing.

**NOTE-3 — the filed item's body undercounts what is invisible.** `work/ssi/limb-2-frame-drift-…-at-eps-1e-12.md:36` says a defect "smaller than
about a third of the floor" is invisible. The honest drift is 0.306 of the floor and the ceiling is 1.0, so up to about 0.7 of the floor passes.
DEMONSTRATED: the +1e-2·|z| mutant reads 0.425 of the floor at 1e-12, green. The title's "~2% of the bound" is right; the body sentence is not. `sure`.

**NOTE-4 — the "4 ulps of R" claim and the "drift ≤ floor" claim are both origin-scale facts.** DEMONSTRATED (`probes/scale_results.txt`).
Limb 1 stays ≤1.74 ulps of the *image's* R at every scale and translation tried, so the oracle is honest.
The row takes `ulp` from the seated wall (`:263`). Under a translation of 10·s that reads 8–11 ulps, and under 1e3·s it reads 700–870 ulps.
Limb 2's drift reaches 5.6–8.2 floors under a 1 km translation. The row has no translations, so it holds as written, though a translation is
half of a rigid map. The limb-2 measurement is evidence for the SSI item's "unmeasured source": the drift tracks coordinate magnitude, so it is rounding.

**NOTE-5 — off-target: scale behaviour.** At s = 1e-3 the δ = 0 floor is 5.0e-14 m, not the 3e-17 that scaling would give. The bound has an absolute
component that does not scale with the part (`unsure` where it lives). At s = 1e3 the seated pair refuses at every placement with
"foot-point projection did not converge". This is `likely` the class of `work/flux/project-eps-point-is-absolute-so-a-km-model-refuses-off-geometry.md`.

**NOTE-6 — claims 1, 2 and 4 hold.**
- Claim 1: the drift is absolute, a few ulps, and ε-independent. It reproduces.
- Claim 2: the oracle `fl(1+δ)−1` is independent of the kernel. A frame-free limb-1 error passes main's row and fails the new one (row 5).
- Claim 4: the measured floor ×1.3 is 4.214e-14, pinned at 4.2e-14 (×1.296). `rg DRIFT crates/` has no other reader of this constant
  (the `validate.rs`/`surgery.rs` hits are unrelated). M2 at 1e-12 is unresolvable on this fixture, as the PR says (row 6).

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q8; Q7 below)
- `:111` — the new doc line is 126 columns, unwrapped in the middle of a paragraph that is otherwise wrapped at 72. `sure`.
- Q3 — the floor self-reference: see NOTE-2. `likely`.
- Q1 — the coordinate scale R is derived by hand from the wall's control net (`:263-268`). That gives √3, although the evaluated points
  have |p| ≤ √2. Taking the control net's maximum is an unstated choice; it makes the pin about 1.2× looser than the points' own scale would.
  No second copy was found (`rg 'f64::EPSILON \*' crates/topo/tests`). `unsure`.
- Q5 — the test doc (`:245-251`) says limb 2 "moves by no more than its own floor". That is true for origin-centred rotations at unit scale
  and false under translation (NOTE-4). The module doc (`:1-18`) still matches the code. `likely`.
- Q6 — `ROUNDING_ULPS` ("measured within 1.8") is guarded by its own assert, so it is fine. The limb-1 1e-12 resolution (MINOR-1) is a measured
  limit with no schedule. `sure`.
- Q4 — the TINT item's "shape of a fix" cites "the way the limb-2 drift is already stated against `DRIFT`", and this PR retires `DRIFT`.
  The item closes with this PR, so the citation is harmless. `sure`.
- Q8 — I read the whole touched file. `rotations()` (`:210`) contains rotations about the origin only; "rigid map" in the names promises more
  (NOTE-4). `likely`.
- Q7 — I would have stated limb 2 against `FLOOR` (the pin) rather than the runtime floor, which closes NOTE-2. `unsure` that it matters.
