
Found by both reviewers of D′2 (PR 5613, frozen head `ca29c81468`), and demonstrated by execution. The probes are `review/intent-dprime2-R1` `r1_dprime2_probe.rs` and `review/intent-dprime2-R2` `r2_dprime2_probe.rs` (`r2_antipode_sensitivity`).

**What happens.** A boss rests under plate copy `B`'s bottom face, whose outward normal is −z, with `Slide(X) = 10`. `B` is turned by `Spin(Y) = tilt`.
- At tilt 0 or 1e-12, the boss lands at x = +10.
- At tilt 1e-9, it lands at x ≈ −10.
- Tilts of 1e-6, 1e-3 and 0.1 stay near −10.
- Tilts about x are continuous.

Nothing refuses. The `mate_chart_reference_antipode` decision returns Zero inside its band and flips sign just outside it. So a placement edit, or a variable sweeping a tilt through 0, moves a hung part 20 units.

**Why it is new with D′2.** The value chart is the target space's axes, swung onto the mated direction by the shortest turn (`[placement.values]`, row 191; `mate/value.rs`, `Chart::of`). On main the chart swung through the target body's own frame. There a modelled downward face's normal is exactly the body's own −z, so tilting the body never approached the singularity. D′2 moves the singularity to the space's −z, which is exactly the case of a part hung under a horizontal plate.

**The question.** The code follows the ratified "shortest turn", so the answer is a design decision, weighed by designers first and put to Ev if it is a fork. The options include:
- a typed refusal band around the antipode;
- another reference direction;
- accepting the jump and recording why.

Test 24 turns `B` only about z, so it cannot reach this case. The answer's row is the probe above.
