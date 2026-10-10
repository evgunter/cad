# ORBIT — the plan

The curved boolean's P1 reach: torus touches, sphere meetings, and verdicts that depend on how a body is turned.

Opened 2026-10-08 by REACH's closing cut on its priority seam
(`work/README.md`, Track size), when REACH measured 139 budget points
against 30 with its six charter rows closed. Nothing dispatched.

## The slate

**25 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P1 | `a-torus-near-a-tilted-cut-stops-at-the-extent-scan` | H | A torus near or across a tilted cut's ellipse rim stops at the extent scan or the join's germ frame: its oblique plane and wall pairs have no section arm |
| P1 | `a-torus-touch-along-a-whole-parallel-refuses-r-tan` | M +design | A torus touching a coaxial or axis-normal partner along a whole parallel refuses R-tan |
| P1 | `a-torus-touch-at-a-definite-non-elliptic-point-refuses-r-tan` | M +design | A torus touch that is isolated but not at an elliptic point refuses R-tan |
| P1 | `closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut` | H | A closed sphere group's escape is re-charted by rotating the group while a trimmed face's is cut along its meridian: two answers to one escape |
| P1 | `delete-the-boolean-operand-edge-gate` | M | delete gate_operand_edges once every site behind the sweep refuses a spline or spiric edge typed |
| P1 | `separation-grants-disjointness-only-along-world-axes` | M | topo::separation grants two placements or solids disjoint only by world-axis box non-overlap, so whether it certifies depends on how they are turned |
| P1 | `sphere-pair-meeting-inside-both-faces-refuses-spheres-meet` | M | Two sphere faces meeting in a circle inside both, with no edge crossing, refuse SpheresMeet: the meridian cut serves only the plane arm |

## Order

The world-axis row first (`separation-grants-disjointness-only-along-world-axes`;
its census twin is now CONTACTHOLD's
`census-backstop-separates-curved-pairs-only-along-world-axes`): PR 4122
built the separating-direction test it needs (`boolean::separating`),
so it is a port of a door that already exists. Then the sphere pair
(`sphere-pair-meeting-inside-both-faces-refuses-spheres-meet`) and the
escape row beside it, which share the meridian cut. The two torus
touch rows carry `design: true`: designers first
(`memories/orchestration-model.md`). The edge-gate deletion goes last: the
sites it still protects are ROOTS'
`join-and-continuation-sites-blame-the-edge-gate-for-a-spline-edge` and
GAUGE's `a-nurbs-edges-sector-departure-is-its-chord`, and PR 3984's
backed-out attempt (c4f3840bd) is the prior art.

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
