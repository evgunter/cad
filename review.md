# Review of PR #3977, frozen head 1d4f235512

Lane `reach-dual3977-r1`. **Verdict: NOT-MERGEABLE-AS-IS.** MAJOR 2 · MINOR 3 · NOTE 4. Wall clock 17:10–17:50 UTC 2026-10-03. No glimpse: I read only my brief, the PR body (`get`), and the PR's check runs (13; `test` and `lint` green on 1d4f2355).
Oracles are closed forms (box `dx·dy·dz`; the sliver `det·dip²/(6·s²)`, ∪/∖ from it), never the kernel. Each differential ran the same
probe source on main (623004b9, a worktree) and on head. Probes are in `probes/`. `mutant_rederive_and_mounts.diff` holds the
mutant and the mounts.

## MAJOR
1. **Check 7 now passes genuinely inside-out bodies that main refused correctly.** `validate.rs:4430` (straddle → `None`) and
   `:4436` (undecided at the target → pass). A straddling interval is not negative, but it is not positive either, and the
   f64 reading it overrides was right. DEMONSTRATED BY EXECUTION:
   - `probe_r1_booleans`, contact9's corner pose, the ∩ sliver reverted (`Body::revert`, exact −5.83e-19 m³, f64 −3.2e-19):
     main `NegativeVolume`, head **Ok** at ε 1e-12 (s = 1 mm, both operand orders).
   - Same pose ×1e3 at ε 1e-9: the reverted ∩ (exact −5.83e-10, f64 −4.04e-10 / −2.71e-10) passes at head and was refused on main.
   - `probe_r1_slab` (box, 1–10 mm × 0.1–1 µm, planes re-anchored 1–10 km): **29 reverted bodies per ε (1e-9 and 1e-12)**
     go from refused to passed. Example: 10×10 mm × 100 nm at 3 km, exact −1e-11, f64 −9.54e-12 (within 5 %).
   - `probe_r1_interval_width`: the interval is sound (it holds every exact value) but 3–20× wider than the f64 error. At
     1 mm × 100 nm, 1 km it is [−1.6e-13, 3.8e-13] around an f64 1.109e-13 against an exact 1e-13.
   So claim 1 is falsified as worded: the enclosure is sound, and a body it straddles is let through where main refused it.
   The posture is the backstop's. Inheriting it moves the gate from "refuse what f64 says is negative" to "refuse only what
   is certified negative", and the PR body never states that a wrong body now passes. sure.
2. **Check 10 now silently skips a whole multi-shell solid when one shell's role straddles.** The role is `None`
   (`validate.rs:4775`, `:4777`), and `shell_winding_errors` then `continue`s at `:4599`. DEMONSTRATED BY EXECUTION
   (`probe_r1_check10_tiny_second_shell`, ε 1e-9 and 1e-12): a 1 m cube with a 1 mm × 100 nm slab grafted under the
   **same solid**, its planes anchored 1 km or 3 km away. Upright, that is a second Outer, two pieces: main
   `SolidOuterShells{outer:2}`, head **Ok**. Reverted, it is a Void outside every Outer: main `ShellWinding`, head **Ok**. At
   anchor 0 both trees refuse. Main read these roles correctly. A straddling role now reads as "no evidence", and the solid
   is passed unchecked rather than escalated. sure.

## MINOR
3. **Claim 5, doubled half: no row anywhere goes red when the enclosure degrades.** DEMONSTRATED BY EXECUTION: a mutant in
   `props.rs` `rederive` (`:753`), driven by `R1_MUT`.
   - zeroed width: both new rows go red (contact9 `:79` and `tier3_tests.rs:2710`, `Some(Outer)`), at 1e-9 and 1e-12.
   - doubled width: **the whole `topo` suite stays green**, 2182/2182 at 1e-9, new rows included.
   - ±1e30: about 40 rows go red.

   The new rows assert "undecided / passes", which only gets easier as the interval widens (Q3's wrong-direction monotone).
   No row pins that an inside-out body near the band is still refused, which is the guard MAJOR 1 lacks. sure.
4. **`classify_shells` now refuses valid bodies that main classified correctly.** Every upright slab that main read `Outer`
   (correct) is now `Err(Escalated)`, ~30 per ε (1e-9, 1e-12), at anchors 1–10 km; through `props.rs:2469` →
   `shell_role_at`. DEMONSTRATED BY EXECUTION (`probe_r1_slab`). It fails loud, not wrong. But it reaches the shell verb and
   editor checks, and the PR's "no body is newly refused" covers tier 3 only. likely.
5. **Stale filing path.** `tests/contact9_side_codes.rs:132` cites `work/geom/a-planar-face-sums-…`. The item is
   `work/flux/…`; there is no `work/geom/` file. The PR body says the doc "points at the FLUX item". By inspection (ls). sure.

## NOTE
6. Claim 2 holds. The corner sliver's ∩/∪/A∖B/B∖A all pass tier 3 at 1e-6, 1e-9 and 1e-12, at ×1e-3 and ×1, in both operand
   orders, and `point_in_solid(q)` is In/In/Out/Out correctly. The volume is still wrong at 1e-12, as filed: s = 1 mm reads
   3.21e-19 / 1.95e-19 (by operand order) against 5.83e-19; s = 1 m reads −2.96e-16.
   New and good: at ×1e3, ε 1e-9, the valid ∩ (f64 −4.6e-7 against an exact +5.8e-10) was `NegativeVolume` on main and is Ok
   at head. EXECUTION. sure.
7. The pass side is uncertified, and that is pre-existing and disclosed. The reverted 1 m-edge sliver (f64 +2.96e-16, exact
   −5.8e-19) passes tier 3 on **both** trees. EXECUTION. Filed by the PR in `lane-free-volume-sign-reads…`. sure.
8. The sliver reused as an operand gives no body on both trees: `(A∩B) ∪ B` and `B − (A∩B)`, at every scale and ε. It is
   pre-existing and I did not find the cause. EXECUTION. unsure.
9. The ×1e3 fixture panics in `mapped_cube` at 1e-12 (`carrier_matches_mapped_source` escalates), and every probe stands down
   at 1e-6. These are probe limits.
   Suites: topo is green at 1e-9 (2182). At 1e-12 the only non-probe red is `rigid_map_near_eps_plane_nurbs`, main's.
   `voided_rods_verdicts` is green at all three ε.

## Claim 3 (the other sign readers)
The backstop reads the same `rederive` with the same straddle → pass, so it is consistent, and MAJOR 1 is equally its own.
`props`/`mass_properties` decides nothing. Check 7 certifies refusals only, while check 10 and classify certify both
signs. The PR states the asymmetry and files it, and MAJOR 2/MINOR 4 are its cost. I did not exercise the assembly gate
(`editor-core`); it only maps `NegativeVolume`, by inspection.

## Style (exercised: Q1, Q2, Q3, Q4, Q6, Q7; not Q5, Q8. validate.rs is ~11k lines and was not read end to end)
- Q1: two role readers off one enclosure, `validate::plus_v_read` and `props::shell_role_at` (`props.rs:2503`), with
  different predicate names and different undecided handling. The backstop's positivity arm is a third copy of check 7 (the
  PR says so, and names the door unit as the schedule). likely.
- Q7: `VolumeEnclosure{volume_lo: v, volume_hi: v}`, with `v` an interval (`props.rs:729`, `:2469`), encodes "one interval"
  in a two-ended type. The `padded: bool` knob (`:2522`) exists to keep that encoding from double-metering. likely.
- Q2/Q6: `validate.rs:4370`'s pass-side argument ("a positive f64 sum cannot sit beside a definitely negative enclosure";
  "45 km at 1e-12") asserts an invariant that nothing enforces and a measured number with no guard at the claim site.
  It assumes the f64 and interval closed forms take the same branches, which the `rederive` doc itself says they may not.
  unsure.
- Q4: `shell_role`'s doc ("None where the walk refuses or the sign is still undecided") now also covers a re-derivation
  refusal folded into `None` (`:4777`), where check 7 maps the same refusal to `VolumeUncomputable`. likely.
- Q3: `tier3_tests.rs:2686` stands down at 1e-6, so at that ε the PR's only built witness is vacuous. The contact9 row is
  the only 1e-6 coverage. unsure.
