---
id: a-value-chart-jumps-at-the-antipode-under-a-downward-face
kind: issue
title: A value charted on the target space jumps across the antipode: a body hung under a flat downward face moves 20 units for a 1e-9 tilt of its target
status: closed
opened: 2026-10-23
closed: 2026-10-24
priority: P1
cost: M
design: true
refs: [5613]
---


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

## 2026-10-24 — redirected: the swing retires

Ev, on PR 5621: "stop doing the swing thing and just make people specify the connection between the original coordinates in a better way". Two fresh designers (rounds 2–4, `docs/DESIGN-FORK-LOG.md` row 192) converged on the core, which Ev approved on the PR ("that plan makes sense!"):
- beside a direction-fixing mate the placement writes an orientation, one of the 24 axis correspondences (default the identity; the gesture writes the nearest);
- the spin is the angle about the mated direction left after the least turn from the orientation's prediction;
- the one pole is the orientation written backwards, which refuses in its band, naming the orientation;
- `Flip` stays, and nothing refuses short of the pole band.

They crossed on slides, then both leaned to target coordinates that the mate solves. Carried axes are the alternative. That choice is Ev's on the PR, and `[placement.values]` states the lean. Retiring under either: `reference_about`, `mate_chart_reference_antipode`, the `copy_x` swing, the half-turn at −z. Also absorbed: `a-lone-slide-refuses-the-chart-it-never-reads`, and the viewer's `AddMate` re-reading kept values on the swung chart (`viewer/src/session.rs`).

## 2026-10-24 — Ev decided: F

Ev, on PR 5621: "F makes sense if it's reasonable". So slides run along the target's axes, carried by the least turn. On an axis-aligned face that is a coordinate on the target's own axes. `[placement.values]` states it, with the agreed core. Both designers held F coherent (row 192), so it is reasonable. Its costs as they stated them:
- a slide beside a tilted mate is measured along the face;
- where the source's mating face is off its own axes, the written slides are admitted by a span check;
- the viewer's mate tool rewrites the slides it keeps so a freshly mated copy moves only by the snap.

The implementation is `the-value-chart-reads-a-written-orientation`.

## Closed (2026-10-24)

Closed by `the-value-chart-reads-a-written-orientation`, which builds F. The swing is gone: a mate carries an orientation, and the least turn carries where it puts the copy's mated direction onto the mated one. The plate's tilt moves `d` near where the identity puts the boss's top, so the least turn is near the identity and is smooth there. The boss stays within `O(tilt)` of `x = +10`, continuously through zero, for tilts from 1e-12 to 0.1 about either axis, either sign. The row is `intent_value_chart_orientation::a_boss_hung_under_a_tilted_plate_stays_within_its_tilt_of_its_place`. The one pole left is the orientation written backwards, which refuses `MateFault::Backwards` in its band, naming the orientation to write (`the_pole_band_refuses_naming_the_orientation_to_write`).
