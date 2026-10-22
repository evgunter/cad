# Designer A — first report (quad rounds dial), as delivered

## For Ev

**Recommendation (likely): stop treating "the bracket is wider than 1024·ε" as a refusal.** The reporting measurement takes the resolution it is asked for as a per-call parameter, the way tessellation takes δ. It refines until that resolution is met or the work wall is reached, and returns the certified bracket it reached either way. The round caps stay, as the one thing the kernel decides: a bound on work. These all go away:
- `PropsError::QuadratureBudget`;
- the after-round-0 "budget exit";
- `TargetUnreached` (a refusal that carries the value in its payload);
- the Python exception with `volume_lo/volume_hi` attached.

The "dial decision" then dissolves: turning a cap moves no refusal set, only how narrow a bracket the wall buys and what it costs.

**Premise corrections.**
- **dm1 is stale.** The row's "one more round admits dm1" no longer holds: dm1's only remaining refusal is the arc-rim `MapResidual` at the pcurve mint. The dial does nothing for dm1 (sure).
- **The widths are unchanged since August**, as a fixed schedule must leave them. Re-measured on the branch (release), against 1.024e-6:
  - quarter torus: 1.1457e-6 m;
  - two-span half cylinder: 1.3035e-6 m.
- **One more round is cheap on one arm and expensive on the other.** With the cap raised locally, rounds 8 and 9 give:
  - torus: 2.88e-7, then 7.79e-8;
  - half cylinder: 3.26e-7, then 8.15e-8;
  - ratio 3.98–4.0 per round, no floor.

  Cost per round (rounds 7, 8, 9):
  - torus (composite arm): 11.9 s, 41 s, 156 s;
  - half cylinder (v-exact arm): 1.1 s, 1.4 s, 2.0 s.

  With the cap at 8 the torus becomes a 54 s number where it is a 2 s refusal today (sure).
- **"When the quadrature stops and refuses" is two decisions, not one.** PR 5568 already separated them for the sign caller: check 7 refines to the wall, and refuses only with the sign still open. The conflation that remains is in the *number* door. There, the wall (a work bound) and the reporting target (a precision policy) end in one refusal, and the `rounds: 0/1/N+1` payload is the only tell.

**What the issue is evidence of.**
- **The target makes a part-size frontier.** `1024·ε` is an absolute length chosen inside the kernel. It is applied to a rule whose achievable width at a fixed wall scales with part size; `quad.rs`'s own table says "metre-scale parts sit ON the frontier". So every fixed cap at a fixed absolute target is a part-size frontier, and the dial only moves it, by 4× in size per round.
- **The recourse is the wrong shape.** The refusal's recourse says "loosen the tolerance to 1.273e-9 m", i.e. change every classification in the model in order to print a volume. D4 `[tolerance.recourse]` tolerates that shape only for a genuine kernel limit, and this is a self-imposed reporting policy.
- **The validity decisions already do it right.** Check 7, `boolean::ops::volume_backstop` and the shell-role reads decide a SIGN or a bound at the end of the schedule, metering what is still open as a boundary displacement against the model's band.
- **The number door is the one place a width is a verdict.** `MassProperties` already carries `volume_pad`, so the width is already in the value.

**Final state B (recommended).**
- **A requested resolution.** `mass_properties` (and `SignCertificate::measure`) take a requested resolution: a length, the mean boundary displacement the flux width corresponds to, which is the meter already used. They stop at the first round that meets it, else at the wall.
- **The convergence test is a plain comparison** on an f64 width: a stop criterion, not a verdict. So `props_quad_converged`, `props_quad_last_round` and the last-round in-band escalation (the new P2 row) go: no funnel decision is made about a request.
- **The return is always the bracket** when every face has an enclosure. Refusals remain only where there is none: an unsupported chart, a degenerate face, a poisoned bracket, an escalated geometric decision. `MassProperties` should make the bracket primary (volume as an enclosure, the midpoint a method), so a wide answer cannot be read as a number by accident.
- **The budget exit (`last_round_width_lo`) is deleted**: its one consumer was the number door. A request the schedule cannot meet runs to the wall and returns the narrowest bracket the wall buys. Today's exit hands back the round-0 bracket, so the lane that stops early is the lane that answers worst:
  - on the quarter torus, 1.9e-2 m where round 7 is 1.15e-6;
  - the lily at ε = 1e-12 reads a bracket 100× looser than at 1e-9.
- **The default request is a fixed length**, 1.024e-6 m (today's value at the default ε), not ε-derived. The mesh layer never reads ε for sizing, and a request that tightens with ε makes every 1e-12 run pay the wall for a precision nobody asked for. ε is read only by the soundness pads and the sign gates. Every face that converges at the default today returns bit-identical numbers (same rounds, same arithmetic).
- **The caps are stated once**, as a work bound per face (cells the last round evaluates), with each lane's round count derived from it. `LAST_ROUND_EVERY_LANE_RUNS` and its compile-time assert go, since the sign walk already reads each lane's `rounds_remain`.
  - I would not raise the wall now. Under B it moves no admission set at the number door, and at the sign gates only a body whose sign is still open after ~10⁶ cells would gain; none is known.
  - A wall that ends a sign walk is the one refusal left, row 1 with an honest recourse.
- **Reversible:** yes. It deletes code, and a parameter is easy to add back a policy over.

**Alternative A (the row's framing): turn the dial 7→8.** Reject: it is a patch, and it owes the row-by-row argument the issue asks for.
- It admits the quarter torus (54 s) and the half cylinder (1.9 s), not dm1.
- Every composite-arm face that reaches round 8 costs 4×.
- The frontier stays a size frontier: a 4 m two-span cylinder still refuses.
- The recourse still says "loosen ε", and the P2 escalation row and the budget-conflation row stand.

**Alternative C: keep the refusal, and make the target relative to part extent.** Reject. It removes the size frontier, but it keeps a kernel-chosen precision policy and privileges a length scale (which extent: face, body?), the thing D4 refuses for angles.

**Ratified text to change.**
- `crates/geom-brep/README.md` clause (7), "exhaustion is `QuadratureBudget`, never a silent Gaussian". It was written in the implementation commit (b54f9af64d, 2026-08-01) and was never Ev's words. Under B it reads: the wall ends in the bracket it reached, with its width in the value; an uncertified rule is never trusted.
- One sentence for DESIGN.md beside `[tolerance.chordal]`: a measurement's requested resolution is a per-call parameter of δ's kind, participating in no validity decision; the certified promise is the enclosure.
- Nothing else ratified is in the way.

The quadtail row's question (should the loops read `descent`'s stall rule) is answered no: these rules quarter a priori and have no stall to detect. What they share with `descent` is the work bound only.

**Cost, stated separately.**
- Faces that today refuse after one round (1.6 s on the torus) would run to the wall at the default request: ≈25 s on a composite-arm torus face, ≈2 s on a v-exact face. Ill-conditioned extreme-weight patches pay the wall for a bracket that stays useless.
- With the default request fixed rather than ε-derived, a 1e-12 run costs what a 1e-9 run does.
- If that is too much for CI, the one-line mitigation is a lower wall for the number door than for the sign gates. A reachable looser request is the caller's own lever.

**Confidence:**
- the recommendation: likely;
- the stale-dm1 and re-measured-floor claims: sure;
- "cost 4×/round on the composite arm": sure (measured);
- "no body's sign stays open at the wall": unsure (not swept).
