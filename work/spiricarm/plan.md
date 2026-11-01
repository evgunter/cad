# SPIRICARM — the plan

Opened 2026-10-27 by the re-cut of FLUX and FLUXTAIL. Together they
held 95.5 budget points against 30, and both shared
`props/curved.rs` and `props/spiric.rs`, so their orchestrators
could not run side by side. The rows were re-cut by logical thread
instead of by priority, so that the five successors can run in
parallel: each mostly owns its own files, and where two share a file
they work on different functions of it
(`docs/doc-ledger/flux-leaves-the-tracker.md`). Nothing dispatched.

## The slate

**20 budget points** of dispatchable work against a ceiling of 30, in 8 rows (2026-10-27).

| pri | item | cost | title |
|---|---|---|---|
| P2 | `a-long-plane-face-cut-by-a-torus-reads-a-wide-pad-about-a-point-off-its-plane` | M | A long plane face cut by a torus reads a pad up to 1e3× wider at 1e-12 when its shell's point lies off its plane: 70× with one torus on a bar's top face |
| P2 | `tier-3-refuses-a-major-1300-torus-tube-at-1e-12-that-the-measurement-converges-on` | M | at 1e-12 tier 3 refuses the major-1300 torus tubes, hollow and solid (VolumeUncomputable, an escalated sign read), that the measurement door converges on, so the typed door cannot measure them |
| P3 | `a-spiric-bounded-torus-band-has-no-volume-lane` | M | A spiric-bounded torus band (two loops winding the wall both ways) has no volume lane: the spiric wall refuses LoopsWindChart |
| P3 | `a-spiric-elbow-a-hundred-kilometres-out-exhausts-the-quadrature-budget` | M | The klein elbow hollow translated by 1e5 refuses QuadratureBudget at the reporting walk (width 7.9e-6 against target 1.0e-6 at round 13): the spiric lane's width grows with distance from the origin |
| P3 | `a-thin-corner-bars-spiric-cap-exhausts-the-quadrature-budget-at-1e-12` | M | The fat torus less a thin corner bar (w = 0.05) refuses VolumeUnmeasured QuadratureBudget at eps 1e-12 (width 2.13e-9 against target 1.02e-9 at round 13): a spiric-bounded face of P minus T |
| P3 | `a-torus-face-whose-chart-closes-in-both-directions-may-read-its-side-off-the-disc` | M | Probe: a one-loop torus face whose chart domain closes in both directions without a wrap edge (torus minus a disc) may be read as the disc by the Green-area side |
| P3 | `f64-spiric-wall-ends-near-tangency-can-read-a-closed-junction-open` | M | At the f64 lane a spiric wall's ends near tangency carry rounding past the vertex slack, so a closed junction can read open |
| P3 | `the-spiric-wall-quadrature-lane-reads-only-the-own-torus` | M | the spiric-bounded torus-wall quadrature refuses a wall on the carrier's mirror torus |

## Order

Re-measure first: PR 5741 (QUAD's measure door, unit C) removed the `QuadratureBudget` refusal that `a-spiric-elbow-a-hundred-kilometres-out-…` and `a-thin-corner-bars-spiric-cap-…` name. Each now reads a bracket, and its row says what to measure. The lane rows (`a-spiric-bounded-torus-band-has-no-volume-lane`, `the-spiric-wall-quadrature-lane-reads-only-the-own-torus`, `f64-spiric-wall-ends-near-tangency-…`) share `wall_windings` and the wall lane, and batch together. The two P2 width rows (`a-long-plane-face-cut-by-a-torus-…`, `tier-3-refuses-a-major-1300-torus-tube-…`) lead on priority. `a-torus-face-whose-chart-closes-in-both-directions-…` is a probe: build the body before building any fix.

QUAD holds `spiric.rs`'s rounds and `quad_lane.rs`. Read `work/quad/plan.md` before dispatching, and announce the seam.

## Review posture

FLUX's, inherited: the review tiers of `memories/orchestration-model.md`
(orchestrator's read, single style or full review, or a dual under
`docs/DUAL-REVIEW-PROTOCOL.md`, logged in `docs/DUAL-REVIEW-LOG.md`),
named with its reason at dispatch. Expect a dual on a row that changes
what the kernel will certify over a family: a wrong closed form is the
error that reads as an improvement.

What PROPS and FLUX learned, stated so that briefs carry it:

- **A spec that asserts a premise hands the unit a defect.** One
  premise about `du_of_rims` failed three times running in PROPS, the
  last time on main. A spec here requires a decide.
- **A re-baselined golden is not a re-pointed row.** Check what a row
  witnesses before moving its constants: its subject can move under a
  change while its numbers stay movable.
- **A narrower bound is the expected outcome of a width fix, and
  therefore not evidence that the fix is correct.** Containment is.
  Test against exact arithmetic, with cases chosen to break it.

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`,
so the program closes without an exit walk.
