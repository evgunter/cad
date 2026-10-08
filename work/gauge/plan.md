# GAUGE — the plan

The boolean and split gates' coverage: arms no row reaches, guards that repeat, and readers that see a chord.

Opened 2026-10-08 by REACH's closing cut on its priority seam
(`work/README.md`, Track size), when REACH measured 139 budget points
against 30 with its six charter rows closed. Nothing dispatched.

## The slate

**25.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `a-nurbs-edges-sector-departure-is-its-chord` | E | both sector walks take a NURBS edge's chord as its departure direction at an ON vertex |
| P3 | `an-approx-face-on-line-edges-has-no-finished-fixture` | M | The operand gate's Approx × Plane germ-pair refusal is reached by no finished body through the public door |
| P3 | `boundary-crossing-cuts-cannot-see-an-over-tight-face-box` | E | Only crest cuts and direct box rows can see a face box tighter than its face; boundary-crossing cuts cannot |
| P3 | `cut-in-refusals-no-probe-reaches` | M | Four of the cut-in's refusals have no row: a pole inside the circle, a hole inside it, ends on two loops, uncertified roots |
| P3 | `no-row-asserts-the-operand-gate-and-the-sweep-agree` | E | No row asserts that the operand gate and the sweep's curved arm agree on the pairs the narrow phase parts |
| P3 | `section-area-skips-a-spiric-or-nurbs-section-edge` | E | certify_section_area reads a spiric or NURBS section edge as its chord |
| P3 | `split-gate-approx-arm-has-no-whole-split-row` | M | The split gate's Approx arm is read at the gate only: no public door builds a body with an Approx face to split whole |
| P3 | `split-gate-spiric-edge-arm-has-no-row` | M | The split gate's spiric-edge arm has no row: no public door builds a split operand with a spiric edge |
| P3 | `split-gate-torus-ring-fallback-has-no-door` | E | The split gate's torus fallback for R <= r has no public door: revolve refuses the spindle torus |
| P3 | `split-section-area-spells-the-planar-winding-sum-a-third-time` | M | the planar-region signed-area sum (Newell / shoelace + conic correction, 2A/P) has several spellings outside crate::loop_winding — split_section_area, chart_region, and geom-brep's loop_vector_area among them |
| P3 | `split-section-boundary-curved-arm-untested-past-the-edge-gate` | E | The section-boundary describer's Spiric/Nurbs arm has no test, because the split operand gate refuses those edge kinds first |
| P3 | `the-boolean-operand-gate-re-runs-tiers-1-and-2-on-finished-operands` | M | The boolean's operand gate re-runs tiers 1 and 2 on operands the door's type already guarantees finished |
| P3 | `the-ruled-section-tables-carry-their-rulings-as-any-curve` | M | The plane×cylinder and plane×cone tables carry their rulings as Curve3, so every reader of a two-ruling section re-checks that it is a line |
| P3 | `the-sphere-recut-recharts-operand-clones-outside-the-kept-verdict` | E | The no-crossings sphere re-cut re-charts operand clones inside the door, which the operands' kept verdict never saw: unprobed |
| P3 | `torus-window-perp-room-charge-is-unpinned-and-box-test-oracles-cancel` | E | The torus-window perp_room charge has no row of its own, and the boxes test oracles keep the cancelling sqrt(1 - a^2) spelling |

## Order

The eight `E` rows are cheap and independent: batch them two or
three to a lane. The coverage rows each end in a row that reaches the
arm or a stated reason it is unreachable (a refusal no input reaches
is dead code to delete).

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
