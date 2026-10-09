# Review — PR 4415 (frozen head 24aa90e7), DUAL tier, sequential arm, first review

**Verdict: APPROVE-WITH-FIXES** · MAJOR 1 · MINOR 2 · NOTE 6. Claims: 1 holds, 2 holds, 3 holds, 4 is falsified (multi-shell), 5 holds, 6 holds, 7 holds, 8 is real and rightly out of scope, 9 is a NOTE.
Method: base was b1a15ad0 (the PR's merge base, which carries 8e3edbe5) against the head, in separate release targets. Everything was read through `differential::outcome`. Probes are in `review-probes/4415/`: `r_two_cavities.rs` is a sweep example, and `lump.py`/`check49.py`/`thr.py` derive V/A in exact rationals with 60-digit areas from the pose alone. No other lane's branch or PR comment was read.

## Findings

**MAJOR-1 — with two in-band shells, the offered tolerance is false and depends on operand order** (executed). `ops.rs:3732` `finished_body_refusal` quotes the *first* finding's enclosure. Probe `R_MODE=two R_D1=1e-8 R_D2=1e-8 R_S2=2` builds two cavities, the witness void at scale 1 and at scale 2, with V/A −3.289e-9 and −6.577e-9, both certified in band.
- ε = 1e-9: A∪B refuses `Escalated{ShellRole}` quoting 3.289e-9; that offer passes at 0.9×.
- B∪A quotes the *other* cavity: "tighten the tolerance below 6.577432436489206e-10 m".
- B∪A re-run at 0.9× that offer (ε = 5.9197e-10): it refuses `Escalated{ShellRole}` again on the 3.289e-9 cavity. That is the same decision, so `test_utils::offer::execute` would score it F1.
- At ε = 3.2e-10 both orders build (5 shells).

So the recourse D10 names is false on a reachable public pose, and the refusal is not symmetric under commutation. The fix is to choose the in-band finding whose enclosure's nearer end is least in magnitude. The new test (`ops.rs` `the_gate_types_only_certified_in_band_findings_escalated`) passes the same `certified` twice, so it cannot see this. Both executed offer cases are single-shell.

**MINOR-1 — the `Escalated` path drops diagnostics the caller had** (executed and inspected). Base's refusal named `SolidKey(1v1)`/`ShellKey(2v5)`, the walk's margin, and *every* in-band shell. Head's `Escalated{decision, diag}` keeps one certified `Indeterminate`. The "move the parts" lever then has nothing to say which piece is thin, and with n slivers n−1 are silently unnamed (`ops.rs:3728-3734`).

**MINOR-2 — the claim "Display, endings and tags unchanged" holds per variant, not per shell** (inspected). In `props.rs:3234-3244` the arm `(Ok, Ok, Some)` now turns a walk-read `ZeroVolume`/`Straddles` into `Escalated`. That reaches `classify_shells`, editor-core Connectedness (`checks.rs:1315`) and the Python tag (`tags.rs:3086`, "zero_volume" → "escalated") for a shell whose walk reads zero but whose certificate reads in band. The change is defensible, but it is undisclosed in the PR body and pinned only at unit level.

**NOTE-1 — claim 1 holds** (executed). I probed the band edges on `notch307 e0 a3`, base vs head:
- **Below the band (d 3.0–3.1e-9).** These refuse earlier, on Coincidence/Containment, so the role is never reached.
- **Wholly in band (d 1e-8, 3.03e-8, 3.04e-8).** These type `Escalated` (exact V/A 3.2887e-9, 9.9648e-9, 9.9977e-9). At 3.05e-8 (1.003e-8) it builds SOUND.
- **Straddling the escalate threshold.** I bisected d* = 3.0407001874e-8, where the exact V/A is 1e-8, then took 17 tilts at d* ± k·2e-16:
  - every enclosure wholly below 1e-8 gives `Escalated`;
  - the 3 that straddle 1e-8 stay `ResultInvalid{ShellRoleUndecided}`;
  - past that it builds.

  So a touching or straddling enclosure never sets `sliver` (`props.rs:871-874`; geom-core `interval.rs:897`).
- **Census.** Not one head `ShellRoleUndecided` remained across the three ε families below.
- **Unfiled residue (Q6).** A threshold-straddling shell, in band or 2e-16 from it, is still typed as a kernel defect. D10 calls only *definite* findings that. The window is measure-thin, but no row schedules it.

**NOTE-2 — claim 3 holds, with a caveat** (executed).
- **Witness.** Exact V/A at d = 1e-8 is 3.28871625245e-9, inside the enclosure [3.2887162003e-9, 3.2887163469e-9]. The walk's 3.050e-9 is 7% off, so the typing rightly reads the certificate. d = 3e-8 gives 9.86614863e-9, matching the P0 row.
- **Thin void.** CI-pinned to within 1e-6 of its closed form (dsc_checks green on run 37913362742).
- **Every escalation in my families.** I re-derived all 147: 49 at 1e-9, 50 at 1e-6, 48 at 1e-12. The 44 + 49 + 44 ∩/∖ matched a piece's exact V/A on the first pass; w60 matched on its whole convex profile.
- **The caveat.** A tight enclosure certifies the *stored, rounded result body*, not the pose's closed form. `convex e1 a13 d-2e-11 pc S` [2.8328754345e-12] vs exact 2.8328796e-12, and `asym e1 a6 cp I` falls below lo by 1.4e-18. That is ≤1.5e-6 relative, typing-relevant only within that distance of a threshold, and consistent with `rederive`'s doc. The corner recentring is sound far from the origin: widths stay ~1e-16 relative.

**NOTE-3 — claim 4 holds for single shells** (executed). At 0.9× the quoted value:
- the witness ∩ builds at d = 1e-8 (ε = 2.96e-10) and at d = 3e-8 (ε = 8.88e-10);
- the cavity ∪ builds in both orders.

The lever holds: d 3.05e-8 builds. `thickness()` is independent f64 recentred code, matched at 1e-4 (`offer_rows.rs:2073`), which would catch the walk's 7%. `quoted_margin` reading the nearer end is the conservative side (`offer_rows.rs:2061-2069`). The failure for multiple shells is MAJOR-1.

**NOTE-4 — claim 8: the P0 is real, pre-existing, and rightly rides its row** (executed). On head *and* base, `piece ∩ cube` (d = 1e-8, and 3e-8) returns `Ok`, one shell. Its tier-3′ census flags `pm_census_ee_span`, which the door does not run. The same sliver beside the main lump refuses: base as `ResultInvalid`, head as `Escalated`. So the inconsistency (refused in a two-shell result, shipped alone) predates this PR, which only retypes the refusal side. It should not block. Check 10 skipping one-shell solids is at `validate.rs:5235`.

**NOTE-5 — claims 5, 6, 7 hold** (executed and inspected).
- **Claim 5.** The only downstream `ShellClassifyError::Escalated` matches use `..` (`tags.rs:3086`, `checks.rs:1315`, `validate.rs:3237`). `BooleanDecision` is matched only via methods or the kind strum. No docs, stubs or k-lint enumerate decisions.
- **Claim 6.** My census families, base vs head, 11 520 runs each: 49 at ε 1e-9 (±2e-8, ±5e-9), 50 at 1e-6 (±2e-5, ±5e-6) and 48 at 1e-12 (±2e-11, ±5e-12) moved. All moved `ResultInvalid{ShellRoleUndecided}` → `Escalated{ShellRole}`; 0 others moved. `pinch_runs_battery` (3 025 lines) and `rc_wide` shards 5/84 and 50/84 (481 each) moved 0. Both re-pinned rows' poses (`notch307 e0 a3 d1e-11` at ε 1e-12, and the thin void) moved only by type.
- **Claim 7.** The ported files are byte-identical to 4417's merge `14f0cdf1`, and the PR carries every changed line of 4417's `ops.rs` change. A trial merge of origin/main is clean, and afterwards the ported files diff 0 against main.

**NOTE-6 — claim 9: design fit, confidence *likely*.** The in-band judgement has one home, `RoleUnread::certified_by` (`props.rs:870`), which reads the classifier's `terminal_sliver`. The door's `finding_arm` only types it, which is right: neither check 10 nor `classify_shells` should know door semantics. The risk is the P0 fix: typing check 7's `Certified::Open` will add a second in-band arm reading a different record. Route it through the same `certified_by`, or the judgement gains a second spelling.

## Style lane (Q1–Q8 exercised; Q8 partially: the gate region and the props role read, not all 7 538 lines of ops.rs)

- **S1, Q3, sure.** `ops.rs:5672-5682` "certified slivers alone" uses two *identical* certified findings, so a wrong choice among in-band findings cannot go red. That premise excludes MAJOR-1's failure mode.
- **S2, Q1, likely.** `ShellClassifyError::Escalated{source, sliver}` (`props.rs:2808-2818`): in the `(Ok, Ok, Some)` arm, `source` and `*sliver` are the same value, and `sliver` restates `Indeterminate::terminal_sliver`. That is two spellings of "certified in band". `finding_arm` (`ops.rs:3746`) trusts `sliver` without re-checking `terminal_sliver`, and the field is pub on a pub enum, so the invariant is held by convention.
- **S3, Q1, sure.** The near-tangent pose is now spelled a fourth time: `offer_rows.rs:465-675` (`dot3`…`clip3`, `basis3`, `sliver_lump`), the probe, `sliver_shell_role.rs:26-70` and `pinch_cones.rs`. `basis3` drops the probe's `unit(m)`, which is a silent drift point. Class: sweep the topo/sweep test helpers for pose builders.
- **S4, Q4, sure.** These docs cite the old premise:
  - `BooleanError::ResultInvalid` (`boolean/mod.rs:2874-2885`) still says every at-rest gate failure is this variant and a kernel defect;
  - `Escalated`'s doc (`mod.rs:2167-2169`) names only reduction/classification predicates.

  Neither mentions the finished-body gate's in-band arm.
- **S5, Q5/Q2, sure.** The census probe this PR edits prints its own oracle `LUMP … v/a=3.866977e-9` beside the kernel's correct `ESCALATED … [3.2887e-9]`. The oracle sums V about the world origin in f64 (`near_tangent_census_probe.rs:156-164`) and is 18% off, the very failure the PR's logic names. A reader comparing the lines would blame the kernel.
- **S6, Q7, likely.** `certify_role`'s `(Unread(cert), Decided(_)) => cert.certified_by(&cert)` (`props.rs:1017`) reads oddly. The walk's decision is discarded unsaid, and the self-application hides that the certified read *is* the refusal there.
- **S7, Q6, likely.** The deviation "other escalated findings keep `ResultInvalid`" is scheduled (P1 row, nine witnesses). The threshold-straddling shell-role case (NOTE-1) and the multi-shell offer (MAJOR-1) are not.
- **S8, Q7, unsure.** `finished_body_refusal` runs the typing on `structural_gate`'s findings too (`ops.rs:3865-3875`), where no in-band arm can fire. A typing step that cannot vary there is noise, though harmless.

REVIEW COMPLETE
