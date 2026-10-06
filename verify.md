# Verify PR #4044: frozen head c6ab1232032d761032faa95858bc761029e6a4be

Lane `reach-verify/4044`, 2026-10-06. The branch head was re-fetched at the end and has not moved. 4042 (`21e46cff`) and 4046 (`fa6ab613`) are both ancestors of the head.
No code was changed on the PR branch, nothing was merged, and nothing was posted on GitHub.

**Verdict: VERIFIED.** No wrong body in about 20,000 op runs across the three ε. Every lane claim holds. Four non-blocking observations are listed at the end.

## Mutants (ε 1e-9; each applied by hand to the head, run, then reverted)

| mutant (smallest edit) | row(s) | result |
|---|---|---|
| F2: drop `.abs()` in `span = w·(u×to_hi).abs().atan2(..)` | `a_pole_to_pole_cut_of_a_ball_wedge_builds` | **red**: `Euler(..IntervalNotForward { margin −3.141592653589793 })`; every other snowman row green |
| F1a: read only the scanned key (`if let [only, ..] = candidates[..]`) | `two_cut_ins_on_one_face_build_in_either_order`, `a_circle_across_an_earlier_cut_takes_none_of_its_own` | **red, both**: "the cut's meridian leaves the sphere face through no boundary" and "…roots are not certified" |
| F1b: refuse instead of skipping (`None if across_a_cut => return Err(..)`) | `a_circle_across_an_earlier_cut_takes_none_of_its_own` | **red** (only that row) |
| F3: farther hit below (`above(b.s, hit.s)`) | `a_cut_ends_at_the_nearest_of_several_boundary_hits`, `a_slab_cutting_a_cap_off_a_pole_strut_carve_builds` | **red, both** |
| F3: farther hit above (`above(hit.s, o.s)`) | same two | **red, both** |
| F3: both | same two | **red, both** (`Containment(VolumeUncertified)`) |
| F4: any verdict cuts (`Some((holder, _)) => Ok(Some(holder))`) | `cut_holder_rows::only_the_loop_verdict_cuts_its_face` | **red** (no build row moves) |

Under the single-sided F3 mutants, the strut `∪` builds a body that passes tiers 1–3 with volume 36.5644 against 36.3026. Only the closed-form volume catches it. That is mutant-only, but it shows the volume oracle is load-bearing for the cut.

## ε

- PR rows (all of `snowman::`, `cut_holder_rows`, `torn_hop_rows`; 55 tests) at 1e-9, 1e-6 and 1e-12: **55/55** green each.
- nextest `-p topo -p sweep` at 1e-9: **4539/4539** green. There were no reds, so there was nothing to check against main.
- Reviewers' probes, mounted per their headers and re-run on the head:
  - 1e-9: **17/17** green, 0 WRONG, 0 PANIC. r1's `probe_two_cuts_one_lens_face` (130/50, 50/130) now builds every op with `point_in_solid` 240/240. r1's `probe_wedge_pole_to_pole` and r2's `probe_two_caps_one_meridian` are green.
  - 1e-6: `probe_lens_caps` (r1 and r2) is red on ×1e-3 samples 4–7e-7 from the boundary that read `OnBoundary`. That is inside the 1e-6 band, the oracle limit r2 already recorded, and volumes match.
  - 1e-12: `probe_lens_caps` panics building the ×1e3 *fixture* ball (`not a finished body`) before any op runs, as r2 recorded.

## Random poses (probe `verify4044.rs`, local only)

The oracle is closed forms from radii. Each cap is placed so its circle and solid lie inside one trimmed face, clear by a margin. Every op runs in both orders: tiers 1–3, `volume_pad == 0`, and volume to 1e-9 relative. Each family has 30 poses per run.
The families are lens top, lens bottom (r 0.8), banded ball, pole strut, and wedges θ 2.5 / 4 / 5.5.

| run | ops per ε | wrong | refused (1e-9 / 1e-6 / 1e-12) |
|---|---|---|---|
| one cap, random spin | 1260 | **0** | 0.5% / 1% / 0.5% |
| 2–3 caps on one face, random | 1260 | **0** | 79% / 80% / 80% |
| 2–3 caps, unspun bricks | 1260 | **0** | 70% / 70% / 70% |
| 2–3 caps on one meridian (exercises the skip) | 1260 | **0** | 65% / 66% / 65% |
| 2–3 caps within ±3° of one meridian | 1260 | **0** | 61% / 68% / 62% |
| rim-grazing caps (F4 attempt), δ 1e-2 … 0 | 216 | **0** | 61% / 83% / 28% |

- **Later cuts and the skip.** Instrumented locally (`eprintln!`, not pushed), a later cut chose among pieces 264–570 times per run. The across-a-cut skip fired 534–540 times per ε in the one-meridian run. Of the ops that took the skip, 396 / 394 / 390 (1e-9 / 1e-6 / 1e-12) built to the closed form, and 2 refused at 1e-6; none was wrong.
- **What the multi-cap refusals are.** They are dominated by the plane arm's pre-existing AABB pre-check (`ops.rs:3188-3205`, "runs near the plane face's boundary"), which fires before the cut. Next comes the boolean's own `InteriorLoopGuard`. Single caps refuse only on that pre-check, or `Escalated{Sphere(CutIn)}` with its lever at 1e-6.

## Claim checks

- **F2: true.** Both ends lie on the closed half-meridian `h` with the low one below, so the turn about `w` is in (0, π]. The abs only fixes the sign of a rounded zero. Mutant as claimed.
- **F1: true.**
  - The pieces are tracked as `(operand, scanned face, piece)`. The holder is read with `sphere_region::contains`, where `None` means "decided on the boundary".
  - Skip soundness: both crossings lie on the circle's own half-meridian, and the circle is certified inside the original face. A crossing therefore reads as boundary only on an earlier cut's arc along that same half-meridian, and the circle crosses its own meridian plane at right angles there (a symmetry plane), so the crossing layer meets it transversally.
  - Empirically, every skip-taking op that built (390–396 per ε) matched the closed form; 2 refused at 1e-6, none was wrong.
  - Mixed readings refuse (`unread`), and so do two holders (`corrupt`).
  - Minor: the PR body says the scanned-key mutant turns "the two-cut row red". It turns both F1 rows red.
- **F3: true.** All three mutants turn the nearest-hits row and the strut row red.
- **F4: true.** The unit row pins every verdict.
  - I could not construct a non-R-loop pose. Rim-grazing caps at δ ≤ 1e-8, inside the band, refuse typed before the cut: `Join(UnpairedLooseEnds)`, plus `EdgeOnPlane` escalations at the band edge.
  - No section-certificate `what()` text appears in any run, so no non-Loop verdict was reached.
- **Strut flip: true.**
  - The row asserts all four forms in both directions at 1e-9 relative, and passes at all three ε.
  - Closed forms by hand: v_carve = V(0.5) − (V(0.5) − cap(0.5, 0.25))/2 = 0.302705…, and cap(0.5, 0.0075) = 8.79155e-5. That gives ∪ 36.3026176, ∩ 8.79155e-5, strut ∖ slab 0.3026176, slab ∖ strut 35.9999121.
- **Docs: true.**
  - The `CUT_LEVER` text is 30 words. It covers turning the plane, and `every_escalation_renders_within_the_viewers_word_budget` passes in the full run.
  - `EXTENT_LEVER`'s doc is back on it.
  - The PR body limits "same topology" to the operand and cites r1 NOTE 1.
  - Three open items are filed: `apply-cut-ins-walks-its-loops-twice-…`, `circle-plane-first-harmonic-has-three-hand-built-copies`, `cut-in-refusals-no-probe-reaches`.
- **Merge: true.** In `torn_hop_rows`, main's `.is_empty()` became `rechart.is_empty() && cut_in.is_empty()`, and `.len()` became the sum of the two. Each asserts what main asserted, over both lists.

## Observations (not blocking; no wrong body)

1. Multi-cap poses refuse 61–80%, almost all upstream of the cut in the plane arm's AABB pre-check. It is pre-existing and conservative, but it caps how often this PR's lane is reached.
2. When a later cap's circle passes within about 0.05° of the lens pole vertex beside an earlier cut, the finish check refuses `ResultInvalid{VolumeUncomputable{props_sphere_loop_area escalated}}`. A typed, fail-loud refusal, but it does not carry the cut's lever.
3. A cap circle within the band of a sphere face's rim refuses `Join(UnpairedLooseEnds)`. Typed.
4. Under a single-sided F3 mutant, tiers 1–3 accept a wrongly cut strut body. The kernel has no self-check for "the cut lies on the right arc"; the closed-form rows are what guard it.
