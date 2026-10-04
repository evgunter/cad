# Delta review 2 of PR #3977 after fix pass 2, frozen head 831dcd7dc

Lane `reach-delta2-3977`. **Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 4 · NOTE 7.

Before starting I read the brief, `CLAUDE.md`, both prompt files, delta review 1 (`analysis/reach-delta/3977:review.md`) and its probes, the fix-pass-2 rulings (`briefs/fix-3977-2.md`), and the PR body (`get`). I read no PR comments, reviews or check runs.

Method:
- Worktrees:
  - the frozen head;
  - main at the PR's merge base `a8b205dc6`, which is what the head merged;
  - a mutant copy of the head, its switches read from `DMUT`.
- Each tree had its own target directory outside the checkout.
- Oracles are never the kernel:
  - the sign comes from the construction (upright +, reverted −);
  - the magnitudes are closed forms: π r² h, Pappus, the spherical-shell sector, n-gon area × h, and π r² h / cos θ for the cut disc;
  - the settled-residue bodies are re-measured in exact rationals of their stored f64 data (`residue_oracle.py`).
- `probes/delta2-3977/` holds everything: the probes, the mutant diff, the oracle scripts and every log.

## Delta-1 findings

| finding | status | evidence (execution unless marked) |
|---|---|---|
| **MAJOR 1**: curved walks kept the world-origin centre, so inside-out discs passed | **CLOSED** | Delta 1's `probe_d_disc` re-mounted: identical to main (modulo f64 digits) at 1e-9 and 1e-12, 0 inside-out passes. My families, 1–20 km, 3 directions (including negative coordinates), axis-aligned and rotated 0.7 rad, at 1e-9 and 1e-12:<br>• 4- and 64-arc discs: 384 / 272 verdicts;<br>• washers (planes + cylinders), spherical-shell sectors (spheres + cones), thin tori: 288 / 196;<br>• planar n-gons (n = 3, 17, 256), slivers and holed plates: 432 / 386;<br>• cylinders cut obliquely (a quadrature wall): 96 / 72.<br>**0 inside-out passes at head**, and every built inside-out body is refused. Under `oldenc` the same probes pass 207 (1e-9) and 91 (1e-12) inside-out bodies, so the fix carries the closure. Caveats:<br>• on the quadrature family the closure rests on `VolumeSignUnresolved`, and no row guards it (MINOR 2);<br>• the ruling-3 curved row cannot see the old enclosure (MINOR 4). |
| **MINOR 2**: valid far thin curved bodies newly refused (`classify_shells`, `point_in_solid`, box − disc) | **CLOSED for the cylinder-disc family; the class is OPEN** (MINOR 1) | `probe_d_disc3` and `probe_d_void` at 1e-9 and 1e-12: output identical to main, line for line. `classify_shells` reads Outer/Void, `point_in_solid` reads In/Out correctly, and box − disc at 5 km builds 2 shells and passes tier 3. But a valid disc whose wall goes through the quadrature lane is newly refused (MINOR 1). |
| **MINOR 3**: no row sees a curved walk; ruling 3 has one guard at one ε | **PARTLY** | `oldenc` turns sweep `far_thin_discs_are_read_by_their_exact_volume` red at 1e-9 and 1e-12. That meets ruling 1's bar. The tier3 curved row and `a_box_less_a_far_thin_disc` stay green under it (MINOR 4). `inflane` now turns `a_sign_is_read_off_the_exact_volume_not_its_rounding` red at 1e-9 (and contact9 `a_vertex_pair…` at 1e-12), so ruling 2's 1e-9 pin holds. |
| **MINOR 4**: PR body over-claims; backstop `padded` | **CLOSED for the backstop; new over-claims** | `padded: bool` and the hand-built `Exact` are gone (`ops.rs` `Posture::read` takes a `SignReading`; `bound_holds` sums `MassProperties::reading()`). By inspection. New over-claims are in MINOR 1 and MINOR 4. |
| **MINOR 5**: `_structural` doc rotted | **CLOSED** | `validate.rs` `validate_geometric_structural`'s doc now states that it reads the sums and diverges from `validate_geometric`, and that check 10 refuses `ShellRoleUndecided`. By inspection. |
| NOTE 6: r 1 cm × 1 µm at 20 km, 1e-12, `VolumeUncomputable` | now passes | `probe_d_disc` at 1e-12: upright passes, inverted is refused `NegativeVolume`. |
| NOTE 7: enclosure ≠ the sums' target; "which value" | CLOSED (doc), with NOTE 2 below | `certify_role` and `rederive` now say which value. That value depends on the fan's anchor (MINOR 3). |
| NOTE 8, 9 | unchanged / re-run | The topo suite is green at all three ε, 2246/2246. Delta 1's single red at 1e-12 (`rigid_map_near_eps_plane_nurbs`) no longer reproduces. |
| NOTE 11: k-lint | not re-run | Ruling 6 accepted it; the PR records it. |
| Style: `RoleNames` | kept, as allowed by ruling 7 | — |
| Style: check-7 / backstop positivity duplicate | OPEN, disclosed | Left for the door unit. |

## MAJOR

None. No inside-out body passed at head, curved or planar, across the families above, at 1–20 km and ε 1e-9 and 1e-12.

## MINOR

**1. A valid body whose wall goes through the quadrature lane is newly refused `VolumeSignUnresolved`, in domain. Main passes it.** sure.
- `probe_d2_sweep.rs::d2_quadrature_kind`, head vs main. The body is a 4-arc cylinder (r 1 mm) intersected with a slab 10 µm thick, tilted 0.3 or 0.8 rad. Its wall is trimmed by two ellipses, so it takes the certified quadrature. Exact V = π r² h / cos θ.
- **ε 1e-9: 10 of 48 upright placements are refused** at 5 and 20 km, along the skew directions:
  - `validate_geometric` gives `Err([VolumeSignUnresolved])`;
  - `classify_shells` gives `Err(Escalated)`.

  **Main passes all 48** and reads `Outer`.
- At 1e-12, 2 of 36 are refused (5 km, rotated).
- These bodies are nowhere near the band: V/A is 500 × `band.escalate()` at 1e-9. The f64 sum equals the exact volume to 4 digits (3.288e-11 against 3.288e-11). ulp(2e4) = 3.6e-12 ≪ 1e-9, so this is inside ruling 2's domain.
- Their inside-out twins are refused at head, `VolumeSignUnresolved` in place of `NegativeVolume`. That is honest, but it has lost the specific error.
- Cause (likely, not instrumented): `rederive_about` keeps a quadrature face's enclosure at the width it was measured with about the world origin (`closed_form` with `quadrature: Some(..)` → "less c·A⃗", not recentred). For a wall 5–20 km out, that width exceeds a volume of 3e-11 m³ at every round the walk reaches. Main decides on its f64 bracket.
- This breaks ruling 2, "no valid body main accepts may be newly refused inside the representable domain". The PR body's R2 row tested only the cylinder-disc family. The PR body lists `VolumeSignUnresolved` as a behaviour, but not that it refuses valid in-domain bodies 500× past the band.
- Fail-loud, so MINOR, by delta 1's convention.

**2. `VolumeSignUnresolved`, the arm that now keeps inside-out quadrature bodies from passing, is guarded by no row.** sure.
- I ran the `exempt` mutant, which reads `Certified::Unresolved` as in band, i.e. pass (`validate.rs` `plus_v_by_sign`).
- `topo` is 2246/2246 **green** at 1e-9 and at 1e-12, and so is every sweep row.
- Yet **10 inside-out bodies pass check 7** at 1e-9 (2 at 1e-12): the MINOR 1 family reverted, with V/A = −500 × escalate. Main refuses all of them `NegativeVolume`.
- No test constructs `VolumeSignUnresolved`. `rg` finds it only in the sample roster (`test_support_samples.rs`), `assembly.rs`, `tags.rs` and `validate.rs`.
- So MAJOR 1's closure on any walk with a quadrature face rests on an arm that can be deleted with every row still green. This is the Q3 shape: no row can go red.

**3. The re-baselined `door_backstop_settled_residue` verdicts follow the fan's anchor, not the geometry. Re-anchoring one loop flips every refusal in the row.** sure.
- **Oracle.** I dumped the result bodies at all three ε, with the backstop switched off by the mutant `nobackstop` so the refused rows still return their bodies. I re-measured them in exact rationals (`residue_oracle.py`, `anchor_spread.py`).
- **Where the noise comes from.** The glued block-top face's ring (the tool's corners) stands δ = 1.0e-10 (θ = ±1.2ε) to 1.7e-10 (±2ε) off its carrier at ε 1e-9. Its fan value `(anchor − c)·A⃗` changes with the anchor, because `A⃗` has an off-normal part of order δ × size. Over the outer loop's four corners (which lie exactly on the plane), it takes the values 0, +1.57e-10, +1.57e-10 and 0 (in V, i.e. /3), against crossings of ±8.0e-11 and a gap of 4.6e-12. The other anchors (the ring's corners) give −3.90e-10 and +8.02e-11.
- **By execution.** The mutant `fansecond` anchors the fan at the loop's second point: the same geometry, the same carrier, the same c. Under it:
  - standing ∪ (+1.2ε) builds at 1e-9 and 1e-6;
  - sunk A∖B (−1.2ε, −2ε) builds at every ε;
  - so the row as pinned goes **red at every ε**.
- **The moves, classified.**

| row | main | head | intended geometry | classification |
|---|---|---|---|---|
| sunk ∩, θ −1.2ε and −2ε | refuses `vol(A∩B) ≤ vol(B)` at all ε | builds at all ε | crosses vol(B) by ≤ gap | **coin-flip at the noise floor.** The fan model reads margin 0 exactly: the result's vertices are the tool's, so V_fan(A∩B) = V_fan(B). The stored-plane model reads ∓6.97e-12. The built body is right: it is within the row's own gap oracle. Main's refusal rested on reading the glued carrier over the vertices. Acceptable either way. |
| sunk A∖B, θ −1.2ε and −2ε, at 1e-12 | builds | refuses `vol(A∖B) ≥ vol(A) − vol(B)` | crosses by ≤ gap | **coin-flip at the noise floor.** A refusal here is in the design's safe direction (the intended geometry does cross). But the certified crossing (8.0e-14 at 1e-12, 17× the gap) is anchor noise: other anchors read +7.7e-14 (holds), and the stored-plane model reads +7e-15 (holds). |

- **What the lane disclosed.** It says which crossing confirms "follows the standoff's sign, not the bound's", and that "any exact-band bound across a settled declared pair is read under the representation's own ambiguity". The substance is disclosed. Two things are not:
  - that the verdict also depends on which loop point the fan starts from;
  - that the row therefore pins a re-anchoring golden. A refactor that rotates a loop's `first` (as `Body::revert` does) turns it red with no change in behaviour.
- **The doc that is now false.** `props.rs` `corner_of`'s doc says "so two bodies that store one boundary re-derive one value about it". That holds for c, not for the fan: two bodies storing one boundary with different loop anchors re-derive values that differ by up to Σ δ·|A⃗|. On this fixture that is 1.4e-9 m³ on the result.
- No body that builds is wrong, and every refusal is in the safe direction, so this stays MINOR. The row pins noise.

**4. Two guard claims in the PR body do not reproduce under faithful mutants. The ruling-3 curved row cannot see the old enclosure.** sure.
- **`oldenc`** (every walk about the world origin, planes through `closed_form_of`, and a straddle read as before, i.e. exempt):
  - goes red: `far_thin_discs_are_read_by_their_exact_volume` (1e-9 and 1e-12), the three planar tier3 rows, `door_backstop_settled_residue`, and contact9 ×2 at 1e-12;
  - stays **green**: `tier3_tests::a_far_thin_curved_body_is_read_by_its_exact_volume` and `a_box_less_a_far_thin_disc_is_a_valid_hollow`, which the PR body lists as turned red.
  - The lane's `oldenc` reports the walk unrecentred, so check 7 escalates `VolumeSignUnresolved`. That turns the rows red through the *upright* body being refused, not through the inside-out one passing.
  - So the one curved row in `tier3_tests` (ruling 3) can tell the new code from a stricter one, but not from the old one. Ruling 1's bar is met only by the sweep row.
- **`fanorigin`** (the plane's fan anchored at its carrier origin):
  - red: the settled-residue row and the three planar tier3 rows at 1e-9; at 1e-12, those plus contact9 ×2;
  - **no curved row** goes red. The PR body says "a carrier-origin planar flux turns the settled-residue row and the curved rows red". My mutant moves the anchor only; the lane's may differ. likely.

## NOTE

**1. The fan formula is sound for what it claims, by derivation, and no case I built contradicts it.** sure.

*Derivation.*
- Each loop is fanned from the anchor a, a point of the loop: the cone {a + s(γ(t) − a)}. Along each ruling, x − a lies in the cone's tangent plane, so ∫(x − c)·n dA = (a − c)·∫n dA = (a − c)·½∮(γ − a)×dγ. That is exact for **any** edge curve. `loop_vector_area` has closed forms for lines, circles, ellipses and non-rational NURBS, and refuses spirics and rational NURBS typed.
- Fans of neighbouring faces share their edge curves, so a closed body's planar fans plus its curved patches bound a closed surface. The sum is that surface's volume.
- Against any other model of the stored geometry (the stored-plane projection, or a different anchor), it differs by at most Σ_f δ_f·|A⃗_f|. So V/A moves by at most max δ.
- For check 7 that is within the band: δ ≤ zero cannot flip a decided sign. (At δ up to K·ε it could flip only inside the escalate zone.)
- For the backstop's exact posture it is not within the band, which is MINOR 3.
- The interval width is (size of the body) × |A⃗| × u, as small as c is near the face. `rederive` arranges that with the least corner.

*Attacks, by execution* (`d2_planar_fans`, `d2_disc_family`, `d2_revolved_kinds`, 1e-9 and 1e-12, head = main):
- n-gons up to 256 points;
- 8-arc holed plates (rings, plus an arc bore);
- slivers with a 1e-3 rad apex;
- rotations off-axis, and 1–20 km along three directions.

All upright bodies pass and all inside-out bodies are refused. The one exception is the sliver fixture, which both trees refuse `LaminaWedge` (a fixture limit, not check 7).

*Every curved kind check 7 sees:*
- cylinder, cone, sphere and torus are translated (`translated_surface`);
- circles, ellipses, spirics and NURBS curves are translated (`translated_curve`; `map_points` is right because the net is stored Euclidean, `nurbs.rs` `map_points` doc);
- NURBS and Approx surfaces, and quadrature faces, are not recentred and fall to `Unresolved` (MINOR 1 and 2).

**2. Two planar flux formulas now coexist.** The walk's f64 sums read the plane about its carrier origin (`geom_brep::props::planar_face`, through `closed_form_of`). The re-derivation reads the fan (`quad_lane::planar_face_about`). So the sums and the interval measure different quantities by up to Σ δ·A. The PR files the carrier-origin cause on FLUX (`work/flux/a-planar-face-sums-its-area-about-a-far-carrier-origin`). Disclosed and scheduled.

**3. No regression on valid input outside MINOR 1.**
- Delta 1's `probe_d_disc3` and `probe_d_void` give output identical to main at 1e-9 and 1e-12.
- Box − disc at 5 km builds 2 shells and passes, at r 1 mm and 1 cm, h 1 µm and 10 µm.
- My disc, revolved and planar families give the same verdicts on head and main.

**4. `validate_geometric` cost.** Release, the median of 200 calls per body, three runs per tree on an otherwise idle 4-core box (`probe_d2_cost.rs`, `logs/cost-*.log`). Medians in ms, head / main:

| body | head | main | ratio |
|---|---|---|---|
| brick (unit cube) | 0.22–0.24 | 0.16–0.19 | ~1.3× |
| block with an 8-arc bore | 0.82–0.86 | 0.35–0.36 | ~2.3× |
| 4-arc disc at 5 km | 0.42–0.43 | 0.22–0.26 | ~1.8× |
| ball (two sphere faces) | 0.23–0.30 | 0.09–0.11 | ~2.5× |

- The PR's "+40 %" was pass 1's planar figure. On curved bodies pass 2's recentring, and the curved re-derivation, cost about 2–2.5×.
- That is still the price of certifying every round. It is not a defect, but the PR body should carry the curved figure.
- These are numbers from one box, with no register (Style, Q6).

**5. ε runs.** - **`topo`, whole crate:** 2246/2246 at 1e-9, 1e-6 and 1e-12. This includes `tier3_tests`, `contact9_side_codes`, `review_cleave_farplane`, `door_backstop_settled_residue` and the sample roster, plus my residue probe, which asserts nothing.
- **`sweep` + `editor-core` at 1e-9:** 4753/4754. The one red, `every_suite_file_is_aggregated`, was **mine**: an unmounted probe file I had copied in. With it removed, the row passes. No PR row is red.
- **The PR's sweep, editor-core and pncad-py rows at 1e-9, 1e-6 and 1e-12:** 167/167 at each ε. These are `far_thin_disc_sign` (both), `shell_census_is_thread_count_invariant`, `dsc_checks`, `edit_refusal_recourse` and the whole of `pncad-py`'s cargo rows.
- No red to check against origin/main.
- **Not exercised:** Python `unittest` and the binding census; `sweep` and `editor-core` whole at 1e-6 and 1e-12; k-lint.

**6. Territory.** `work.py territory --branch reach/check7-interval` names the same 20 paths the PR lists: 19 crossings and the `ops.rs` double claim.

*Seven change another program's behaviour:*
- `topo/src/validate.rs` (RESTFRONT): check 7, both signs, certified; `VolumeSignUnresolved`; check 10 refuses `ShellRoleUndecided`.
- `topo/src/boolean/solid_contain.rs` (CLEAVE, CONTACT, HONE): `at_infinity_side` certifies through the interval, so `point_in_solid` and every boolean's shell classification answer differently.
- `topo/src/census.rs` (CONTACT): `sweep_cross_solid_backstop` flattens the new `Result` with `.ok()`. That is behaviour-neutral by inspection (still no lane, so the sums decide as before). But the public `census_traces` and `census_traces_planted` gain an `AtRestPolicy` bound, a public signature change.
- `topo/src/boolean/finish.rs`, `join.rs` and `shell_witness.rs` (CLEAVE, HONE, JOIN): bounds only. Their behaviour moves through `solid_contain`.
- `editor-core/src/assembly.rs` (RECIPE): attributes the two new variants.
- `pncad-py/src/tags.rs` (LIB): two new tag strings visible to Python, `volume_sign_unresolved` and `shell_role_undecided`.

*Tests only:* `tier3_tests.rs` (RESTFRONT); `pncad-py/src/tests.rs` and the binding census (LIB); `dsc_checks.rs`, `edit_refusal_recourse.rs`, `sweep/tests/all.rs`, `far_thin_disc_sign.rs`, `shell_census_is_thread_count_invariant.rs`, `contact9_side_codes.rs`, `door_backstop_settled_residue.rs` and `review_cleave_farplane.rs` (TCOST, TINT).

`topo/src/pieces.rs` changes behaviour (it now reads certified roles) and is not flagged, because it is unclaimed.

**7. Mutants, summary** (`topo` full suite, plus the sweep disc rows and my probes; "io" is the count of inside-out passes in my probes).

| mutant | ε 1e-9 red | ε 1e-12 red |
|---|---|---|
| `oldenc` (world origin everywhere, old plane formula, straddle exempt) | tier3 `a_sign_is_read…`, `an_inside_out_slab…`, `check_10_reads…`; `door_backstop_settled_residue`; sweep `far_thin_discs…` (io 207) | the same, plus contact9 `a_pierce…` and `a_vertex_pair…` (io 91) |
| `fanorigin` (fan anchored at the carrier origin) | the three tier3 rows and `door_backstop_settled_residue` (io 0) | the same, plus contact9 ×2 (io 0) |
| `exempt` (`VolumeSignUnresolved` read as pass) | **none** (io 10) | **none** (io 2) |
| `inflane` (`at_infinity_side` with no certifying lane) | tier3 `a_sign_is_read…` | the same, plus contact9 `a_vertex_pair…` |
| `fansecond` (fan anchored at the loop's second point; extra, MINOR 3) | `door_backstop_settled_residue` | the same (also at 1e-6) |

## Style (exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7; not Q8, since `validate.rs` is ~11k lines and I read the touched regions and all of `quad_lane.rs`'s new code)

- **Q1** `props.rs` `closed_form_of` → `geom_brep::props::planar_face` against `quad_lane.rs` `planar_face_about`. Two homes for a planar face's flux, with different targets. The sums use one and the certification the other. Disclosed and filed (NOTE 2). likely.
- **Q2/Q5** `props.rs` `corner_of` doc: "two bodies that store one boundary re-derive one value about it". False once the plane is fanned from its first loop point (MINOR 3). The fix-mints-the-defect check (§1): fix pass 1 chose an order-independent c precisely so the backstop could not certify a spurious violation, and the fan reintroduces an order-dependent term of the same kind, keyed on the loop's anchor rather than the face order. sure.
- **Q3** No row constructs `VolumeSignUnresolved`. The `exempt` mutant is green everywhere (MINOR 2). sure.
- **Q3** `tier3_tests::a_far_thin_curved_body_is_read_by_its_exact_volume` cannot go red against the old exempt-on-straddle enclosure (MINOR 4). Its doc says "the inside-out body passed" about the world-origin form, and under that form here it is refused. sure.
- **Q3** `door_backstop_settled_residue` pins the fan anchor's noise (MINOR 3). It goes red under a change that alters no behaviour. sure.
- **Q2** `door_backstop_settled_residue.rs` header: "which crossing confirms follows the standoff's sign, not the bound's". It follows the standoff × the anchor's lever. likely.
- **Q4** `ValidationError::VolumeSignUnresolved`'s Display gives the recourse "model the part nearer the origin, or tighten the tolerance". On MINOR 1's bodies the f64 sum is right to 4 digits and V/A is 500× the band, so the recourse names the user's model as the problem when it is the enclosure's width. unsure.
- **Q7** `validate.rs` `plus_v_by_sign` carries the last round's `Unresolved` through a `Cell` side channel into `plus_v_at_target`, while `shell_role` and `at_infinity_side` each keep their own `Cell<Option<RoleUnread>>`. That makes three hand-rolled copies of "remember the last round's reading", which `sign_walk`'s `last_word` could receive. unsure.
- **Q6** The ~40 % cost was a pass-1 measurement, not re-taken (the PR says so). I re-took it (NOTE 4). It still has no register. unsure.
- **Q5** PR body, "What now holds": "on every body measured it holds the exact closed form, with a width orders of magnitude below the f64 error". That does not hold for quadrature walks (MINOR 1). sure.

## Probes (`probes/delta2-3977/`, mounted temporarily, not shipped as code)

- `probe_d2_sweep.rs`: the disc, revolved, planar-fan and quadrature families (sweep `tests/all.rs`).
- `probe_d2_residue.rs`: the settled-residue dump (topo `tests/all.rs`); `residue_oracle.py` and `anchor_spread.py` re-measure it exactly.
- `probe_d2_cost.rs`: the release timing (`#[ignore]`d).
- `mutants.diff`: `oldenc`, `fanorigin`, `fansecond`, `exempt`, `inflane` and `nobackstop`, all switched by `DMUT`.
- `logs/`: head, main and mutant outputs; `res-*.txt` are the dumps.
- Delta 1's `probe_d_disc`, `probe_d_disc3` and `probe_d_void` were re-mounted unchanged.
