Round 3 reply: fork-definite-margin

## For Ev

**Recommendation now (likely): hold back. I move to the other reply's rule, with one addition.** Terms:
- F: the form's enclosure over the box.
- V: the value's enclosure.
- P: the same expression enclosed op by op, outward-rounded, at one member chosen from the box. Its width is that member's rounding.

The answer band(F ∩ V) stands only where band(P) gives the same answer; otherwise band(V) stands. The addition: where F ∩ V lies wholly within P's width of the band edge it was read against, the indeterminate is marked terminal. A margin finer than its own rounding stays undecided at every width, so the driver refuses it at the root, typed, and does not walk it to `Budget` or report it as a flip. P enters the same atoms F does (a hull by its children at the point), or every hull-carrying ask is held back.

1. **The witness-match argument is incomplete; I concede it.**
   - When the witness and the real sign agree, the leaf certifies. An `f64` build at another member, whose rounding reads the other sign, then takes a branch the leaf did not.
   - Today a definite V encloses every member's rounded value as well as the real one, because outward rounding is monotone. So E6's "the leaf takes the nominal build's decisions" has in practice covered every member's build. The unheld read would take that away.
   - When they disagree, the leaf is a "flip" at every width, a box of no width included. The document certifies nowhere and the refusal names the wrong cause. A margin that is zero to rounding is not a topology change; the honest report is "decided by rounding", once, at the root.
2. **The hold-back's own costs.**
   - It is a function of the box alone: P's member is a deterministic function of each parameter's symbol and bracket (`point_of` in PR 7712), so it is the same on any thread and at any split. A node served from the store is served only where its brackets are unchanged, so its member is the same too (likely; not traced through the store).
   - It is a sample, not a bound: a child box samples another member and may decide where its parent was held back. That is still each leaf's own function, and I say "the sampled member" in the claim.
   - Paid on every decided ask: P costs 0.2 to 1 ms warm, against the 28 ms ladder those asks pay today. It is the dominant new cost, and still well under today's cost.
   - A clearance of 1e-17 by construction is held back. That is right: its `f64` build is decided by rounding, so the geometry is below the kernel's resolution, and DESIGN `[simple.conditioned]` assumes rounding does not decide. A refusal is the true report.
3. **The twin: one rule with one principled difference.** A proven symbolic `Zero` is not held back. It is a theorem that holds whatever the literals' rounding, so where a member's `f64` reads a rounding-size sign, the member is the one that is wrong. E12 already ratified that ("asked where it rounds to 0 and not where it rounds to 2.8e-17").
   A form-read sign of a rounding-size non-zero margin (6.58e-19) is a fact about how the inputs were rounded: the fixture's hand-set normal. It is the same class as a member's noise. So the hold-back applies to signs and in-band zeros read from F, never to a zero proven from the form.
4. **What an exact comparison is for.** The least-band callers are:
   - `prove`: a generator's inequalities, and `census::undecided_signs`, which narrows the readings of a residual its band read left undecided;
   - `param_order`: the curve pair's walk order and same-foot tests, and the join's crease test;
   - `choose`, and the compound readers `Period::near` and `Azimuth::near`.

   Each survives a held-back sign:
   - `choose` takes its caller's tie rule, and either candidate is sound;
   - `undecided_signs` runs all three readings, which is today's behaviour;
   - `param_order` escalates, which is right for two parameters equal only to rounding.

   So the fix is at the reading. A caller that asks an exact sign of a quantity zero by construction is a construction defect: the fixture's plane is already filed, and on the Boolean-built body the residual is the zero form. That is fixed at the construction, not by a caller's rule.
5. **The rule, and the sentence for E6.**
   - Rule: at an exact witness, a decision the value leaves undecided answers band(F ∩ V) where band(P) agrees and band(V) elsewhere. It is terminal where F ∩ V lies within P's width of the band edge. A zero proven from the form answers `Zero` regardless.
   - E6: "Every decision of a certified leaf holds of its margin's real value at every member of the leaf, and of the rounded computation at the leaf's sampled member; a decision finer than that rounding is refused at the root, never certified."
   - The two replies no longer differ. What would reopen it: the extended count showing held-back terminals as a material share of the tour's leaf refusals. The next step would then be the constructions behind them, not removing the hold-back.

**E12 Decision text** (the other reply's, which I adopt): "Every other margin decides on the intersection of its value's enclosure and its normal form's enclosure over the box's brackets, each atom enclosed from its own arguments; where the form cannot be enclosed, or the margin read at one member with its rounding answers differently, on the value's."

**Confidence.** The hold-back: likely. That the twin's exemption is principled: likely. The determinism of P's member under the store: likely; not traced.

## For the orchestrator

- **The terminal marking is new to this round.** It needs the extended count to report "held back, terminal" as its own column, next to "held back because P declined".
- **M16's members** were the expression evaluated at `f64`, not `f64` member builds. The other reply's suggestion stands: compare decision logs from member replays before closing.
