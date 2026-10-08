# REACH leaves the tracker — 2026-10-08

REACH (curved operand reach, merge-door and join reach, and the at-rest
census) opened 2026-09-20 in CURVED's priority-seam cut and closed
2026-10-08. By then its six charter rows were closed, its last four
units had merged (PRs 4122, 4123, 4135, 4159), and Ev asked in chat for
it to be split into priority-stratified successors and closed.

Its plan set no `## Exit criteria` (its "Exit shape" asked only that
the six rows close), so it closes without an exit walk
(`work/README.md`, "A closed program's directory is deleted").

## Where the rows went

REACH measured 139 budget points against 30. Its 72 live rows moved by
`git mv`, ids and bodies unchanged, into seven successors cut on the
priority seam (`work/README.md`, Track size):

| program | band | pri | rows | points | what |
|---|---|---|---|---|---|
| ORBIT | 11100–11199 | P1 | 8 | 25 | torus touches, sphere meetings, world-axis separation, the edge gate |
| ROOTS | 11200–11299 | P2 | 11 | 27.5 | the root lanes' charges and harmonics, arms short of a lane |
| TALLY | 11300–11399 | P2 | 10 | 23.5 | result-side refusals: volume bounds, the result gate, ops that disagree |
| APEX | 11400–11499 | P3 | 11 | 29.5 | the numeric frontier: the cone apex, shallow crossings, touch margins |
| GAUGE | 11500–11599 | P3 | 15 | 25.5 | gate and split coverage, repeated guards, chord readers |
| REACHTAIL | 11600–11699 | P4 | 5 | 8 | copies, a long function, a moved doc link |
| REACHHOLD | 11700–11799 | P1 | 12 | 0 | the parked rows (eight on the D10 hold) and one deferred; opens `blocked` |

Three unpriced rows from PR 4044's dual review were priced at the cut:
`circle-plane-first-harmonic-has-three-hand-built-copies` (P2, M),
`cut-in-refusals-no-probe-reaches` (P3, M) and
`apply-cut-ins-walks-its-loops-twice-and-overloads-its-predicate-names`
(P4, M). The two rows still marked `review` closed with their PRs:
`an-edge-crossing-a-cone-face-has-no-root-lane` (PR 4135) and
`torus-touch-off-the-faces-refuses-at-the-section-pass` (PR 4159).

Live rows across the tracker that cited a closed REACH row in `refs`
now cite the PR that closed it; `an-open-sign-row-reds-main-at-1e-6-with-section-loop-mixed`,
which closed without a PR of its own, is cited by PR 3716, the merge
its bisection named. Path citations in code, docs and items were
re-pointed: a moved row to its new directory, a closed row to its id
and closing PR. Logs and `docs/DESIGN-FORK-LOG.md` keep their paths,
which resolve at the SHA below.

Band **6000–6099** closes with REACH, UNSPENT: no A/B ordinal was drawn
from it (its dual reviews are logged as DR rows in
`docs/DUAL-REVIEW-LOG.md`).

Recover SHA `872b33cc178f62a93807d9c2294b5378ab073713`.

    git show 872b33cc178f62a93807d9c2294b5378ab073713:work/reach/<FILE>

## What it left

- The six first-afternoon refusals build or refuse typed at the door
  that stops them: a union with a tilted cylinder boss (PR 3611); a
  declared Rest on a full-period wall (PR 3615); a slab cut through a
  cylinder wall (PR 3627); `sphere ∪ sphere` (PR 3659); two stacked
  parts with rounded outlines over a cosurface wall (PR 3657); and the
  plane × cone elliptic section split (PR 3688).
- Root lanes for curved operand edges: line × sphere (PR 3659), every
  non-circle conic against the curved faces (PR 3805), ellipse × torus
  (PR 3973), and line and conic × cone (PR 4135). The planar crossing
  lane no longer reads a curved carrier as a line (PR 3984).
- Edges tangent to a carrier off the face are no event (PR 4128), and a
  torus touching a plane off the faces builds (PR 4159).
- The boolean door takes the finished-body type (PR 3987). The operand
  gate separates along turning directions, not just the world axes
  (PR 4122). A sphere face's box is its chart rectangle (PR 4123). A
  trimmed sphere group's escape through a plane face builds (PR 4044),
  and the arc side has one rule (PR 3985).
