# GERMTAIL — the plan

GERM's refusal and structure tail.

Opened 2026-10-20 by GERM's closing cut (`work/README.md`, Track size): GERM measured about 48 budget points against 30. Below its P1 and P2 rows the slate is almost all P3, so it was cut by finding class: this track has the refusals and structure, and GERMCERT has the margins. Nothing has been dispatched.

## The slate

**24.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P2 | `ring-path-spells-its-circle-and-conic-arcs-by-hand` | M | ring_path keeps its own circle, conic and oval evaluators beside Curve3 |
| P3 | `a-full-revolves-return-half-rim-misses-its-start-by-two-rounded-half-turns` | M | a full revolve's return-half rim starts at fl(π) and runs fl(π) more, so it ends r·2·(π − fl(π)) short of its start vertex, past the band at ε = 1e-12 from r ≈ 4100 |
| P3 | `edge-pose-reads-a-near-circular-ellipses-u-ref-off-its-semi-axes` | E | edge_pose answers a near-circular ellipse's u_ref from major_frame, which rounding decides |
| P3 | `interior-loop-cut-in` | H | A face pair with a certified interior loop (the section certificate's R-loop) refuses where the loop could be cut into both faces and answered |
| P3 | `ring-path-reads-curve-meetings-and-one-curve-outside-meet` | M | ring_path's misses_in_plane and segment_misses decide curve-pair meetings, and split_ring_path_same_circle decides one curve, outside geom_brep::curve_pair |
| P3 | `square-wall-crossing-at-a-tubes-top-refuses-r-order` | M | The square-wall arm refuses R-order where the wall crosses the tube at its top or bottom parallel, though the section is regular there |
| P4 | `a-ring-path-along-a-ruling-or-crest-arcs-carrier-reads-nothing` | M | A ring path along a cone ruling arc's line, or the torus's crest cylinder, reads nothing even off the arc |
| P4 | `section-wrong-lane-ends-in-a-tag` | E | geom-brep: SectionError::WrongLane ends in '(a kernel bug)', not the shared kernel-defect ending |

## Order

`a-full-revolves-return-half-rim-misses-its-start-by-two-rounded-half-turns` and `square-wall-crossing-at-a-tubes-top-refuses-r-order` first: each is a regular body refused today. The three ring-path rows go in one lane: they edit one file, and the P2 one is its own carriers' evaluators. `interior-loop-cut-in` is H and wants a design read before dispatch (`nurbs-plane-section-has-no-component-arm` moved to SPLICE, which closed it). The P4 rows go in one lane.

## Ratified ground (cited, not re-litigated)

- `docs/DESIGN.md` D1–D9 as ratified, and D10 **as held**: a row that
  needs a touching pose (F0) waits on D10 and is not dispatched round it.
- Ev's PR 5156 decision: a near-square plane × cylinder/cone section is
  the exact Ellipse, decided once on the pose; no reader re-decides its
  kind from the semi-axes.
- Shared ground: see the program's keep_out. Run `scripts/work.py
  territory` on your branch and announce the seam in the PR.

## Review posture

GERM's, inherited: the tiers of `memories/orchestration-model.md` and
`docs/DUAL-REVIEW-PROTOCOL.md` (H → concurrent pair, M → the arm byte),
rows in `docs/dual-review-log/rows/`. A lane measures its row's repro on
current main before it builds; most rows were filed before PRs 5152–5394
moved the ground. Every worktree builds in its own target directory;
the ε knob is `CAD_TOLERANCE_EPS`; test filters are `test(/module/)`,
since sweep and geom-brep tests compile into one `all` binary.

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`,
so the program closes without an exit walk.
