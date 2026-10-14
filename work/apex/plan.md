# APEX — the plan

The curved lanes' numeric frontier: where the cone, torus and touch lanes refuse short of what they could answer.

Opened 2026-10-08 by REACH's closing cut on its priority seam
(`work/README.md`, Track size), when REACH measured 139 budget points
against 30 with its six charter rows closed. Nothing dispatched.

## The slate

**29.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `a-conic-touching-a-torus-off-its-face-passes-the-pierce-and-refuses-at-the-section-pass` | M | A circle or ellipse edge touching a torus off its face passes the pierce, then refuses FallbackExtentUnsupported at the section pass |
| P3 | `a-straddling-span-whose-roots-do-not-settle-keeps-its-door-off-the-face` | E | A span whose ends straddle a carrier and whose roots do not settle keeps the pierce door though every touch is certified off the face |
| P3 | `a-touch-beside-a-spline-or-spiric-edge-is-not-certified-clear-of-it` | M | A carrier touch whose ball a spline or spiric boundary edge's box reaches is never certified clear of that edge, so it keeps the pierce door |
| P3 | `f64-cannot-place-a-shallow-crossing-within-the-finest-band` | H +design | At eps 1e-12 an f64 first-order bound cannot place a circle x sphere crossing of slope below ~1e-3: the near-tangent snowman stops at delta 1e-7 |
| P3 | `face-reach-misses-an-interior-far-point-on-a-sphere-torus-or-cone-face` | M | face_extent reads only a face's boundary, which misses a sphere's or torus's far side or a cone's apex inside the face |
| P3 | `the-cone-apex-refusal-zone-is-a-frontier` | H | The cone root lanes refuse a wide zone about the apex, growing with scale: a frontier VERBS-CONE consumers will meet |
| P3 | `the-far-nappe-tell-off-has-no-row-where-it-decides` | M | No row reaches the cone crossing lane's far-nappe tell-off where it decides: every cone face the rows build has a trim that already places a mirror-nappe root outside |
| P3 | `the-line-cone-depth-rung-is-levered-by-the-carriers-reach` | M | The line × cone depth rung divides by the carrier's reach R, so near the apex a clean transversal pair refuses inside δ ≈ √(2εR) |
| P3 | `the-line-cone-lead-rung-is-levered-by-the-segments-length` | M | The line × cone lead rung is levered by the segment's length, so a short edge reads as generator-parallel about ±15° off one |
| P3 | `the-off-face-touch-decisions-have-uncalibrated-margins` | E | The four off-face touch decisions have uncalibrated margins and no K samples |
| P3 | `vertex-vertex-side-codes-take-no-curvature-charge-on-curved-sector-faces` | M | The vertex-vertex lane reads a bound's side against a curved sector face's tangent plane with no curvature charge, where the pierce lane charges it |

## Order

The frontier rows want measurement before code: each states its
zone, and a lane should first chart it at the three ε rows. The
far-nappe row is a coverage gap a verifier found surviving every
mutant (PR 4135's `analysis/reach-verify/4135`), and is the cheapest
start. `f64-cannot-place-a-shallow-crossing-within-the-finest-band`
carries `design: true`: designers first.

## Ratified ground (cited, not re-litigated)

- `docs/DESIGN.md` D1–D9 and D10 as ratified; the DEV-1 set; the C6
  interference-fit era; the merge door's `RecordsASkip` doctrine (#2105).
- Shared ground: crates/topo/src/boolean/* and crates/topo/src/splitting/* are shared ground with REACH's sibling successors (ORBIT ROOTS TALLY APEX GAUGE REACHTAIL REACHHOLD) and with CLEAVE HONE GERM CONTACT ZIP BOXES PIN TANG and JOIN. run scripts/work.py territory on your branch and announce the seam in the PR.

## Review posture

REACH's, inherited: the review tiers of `memories/orchestration-model.md`
(style, full single, or a dual under `docs/DUAL-REVIEW-PROTOCOL.md`,
logged in `docs/DUAL-REVIEW-LOG.md`), with an independent verifier on
any unit a review passes with fixes. A lane measures its row's repro on
current main before it builds: the ground has moved since most of
these rows were filed.

Two practices REACH's last sitting learned the hard way:
- every worktree builds in its own target directory (a shared
  `CARGO_TARGET_DIR` serves one tree's crate to another's build);
- the ε knob is `CAD_TOLERANCE_EPS`.

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`,
so the program closes without an exit walk.
