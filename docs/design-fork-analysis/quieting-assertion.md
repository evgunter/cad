FORK: atrest/a-gap-over-adjacent-cells-states-nothing-of-the-row (D10 last unbuilt item)
Dispatched 2026-10-30 ~09:20Z by the INTENT orchestrator.
Blinding byte: 142 (even: A = Opus, B = Fable)
Opus designer: session_01VyvYnHM3dyfg7iLpqCFVRs; Fable designer: session_01VyvYnMhPckGZCQGhjEh4zK (identical briefs, dispatched together 09:11Z)

## First report, A (09:18Z)
A (Opus, byte 142 even) first report, 09:18Z.
- **Answer:** split by what a row decides.
  - A contact (where material is) is quieted by Gap.
  - Every other coincidence (one axis, one radius, centre on an axis, a joint, a common foot, split ON) is said by construction alone; no assertion quiets it.
  - An opposed tangency is a contact: `Gap`'s curved arms ignore material, a defect to fix.
- **Retires** the not-built item; recourse is chosen by row kind.
- **Rewords** [intent.checked], [intent.statements], [tolerance.recourse], [assertions.quiet] and the state of the build.
- **Evidence:** Ev's flush ruling on PR 5555 ("a mate that additionally indicates that it is flush") already breaks [intent.checked].
- **Confidence:** likely.
- Rejected the framing: partly (the brief's list mixes three things).
- Changes to ratified text: yes.
- **Defects found:**
  - the Gap curved arms;
  - Recourse::words says "assert" for unassertable kinds;
  - SurfaceTangent carries no sense.
- Wants the DS3 knob row in the same [ev] PR.

## First report, B (09:26Z)
- **Answer:** keep "an assertion quiets when it says the same" and add vocabulary.
  1. A tangency is a gap of zero: add Gap arms (plane×cyl, plane×sphere, cyl×sphere, external cyl×cyl).
  2. A measure's reference may be a pose (point, axis, plane), so pose coincidences are said by `Distance = 0` or `Gap = 0` over poses: "whatever a mate can construct, an assertion can claim".
  3. [intent.checked] names the unsayable: the blend turn's feet and written-value equality, recourse construction only, never refusing.
- **Flush:** it would let a flush pair quiet via `Distance = 0` over the planes, reopening Ev's PR 5555 flush ruling unless the clause says why.
- **Argument:** overconstrained placements cannot take the mate, so they need a checked claim.
- **Confidence:** likely.
- Rejected the framing: partly (three problems, not one).
- Changes to ratified text: yes ([intent.checked], [assertions.quiet], [coincidence] constructing, [assertions] flush).
- **Provenance:** [intent.checked]'s sentence is 00f48c9dcc in PR 5230, agent-authored; the Gap-only rule is c0b81b07d3, Ev's, on PR 4319.

## Comparison after the first reports
- **Agree:**
  - an opposed tangency is a contact (Gap curved arms, which read material);
  - the blend turn's common foot and SameRadius are construction-only;
  - recourse is chosen by row kind;
  - the sym P1 blocks BatteryTurn;
  - a-row-over-a-pose folds in;
  - the DS3 knob follows.
- **Split (core):**
  - for pose coincidences (coaxial, centre on an axis, axes meet, split ON, joints): A says construction only, no assertion; B says an assertion over poses quiets them;
  - flush: A builds on Ev's PR 5555 ruling, B would reopen it.
- **Round 1 of reconciliation:** each is shown the other's report.

## Round 1 of reconciliation, A (09:28Z)
- **Moved to B's core:** one rule (an assertion that says the same quiets); widen it to pose references; tangency is `Gap = 0` once the curved arms read material.
- **Moved by:** the overconstrained placement, and [placement]'s "its recourse is an assertion"; a contact is also a pose equality.
- **Kept:**
  - the Gap material defect;
  - recourse by kind;
  - door-limit rows never refuse;
  - NEW: only `= 0` quiets (`≤ t` is a tolerance, not a quiet).
- **Still differs:** BatteryJoint should carry the operand's verdict rather than get an Angle arm (would accept the Angle arm as an interim).
- **Flush:** leans (i), quietable by `Distance = 0`, which reopens 5555; Ev's call.
