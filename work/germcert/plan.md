# GERMCERT — the plan

GERM's rounding and certificate tail.

Opened 2026-10-20 by GERM's closing cut (`work/README.md`, Track size): GERM measured about 48 budget points against 30. Below its P1 and P2 rows the slate is almost all P3, so it was cut by finding class: this track has the margins, and GERMTAIL has the refusals. Nothing has been dispatched.

## The slate

**17 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `conic-doors-walk-their-taylor-terms-through-the-harmonics-noise` | M | The ellipse x torus and conic x quadric doors walk their Taylor terms through the harmonics' noise, which grows like the carrier's size to the residual's degree |
| P3 | `in-sector-escalates-on-an-in-band-bound-before-a-definite-one-reads-outside` |  | in_sector escalates on an in-band bound reading before a later reading decides the direction outside |
| P3 | `plane-cone-trileans-lever-a-span-from-the-apex-and-a-face-at-its-own-reach` | M | plane × cone levers its trileans from the apex over a span, but at the consumed region's own reach over a face |
| P3 | `root-slack-meter-reads-an-interval-root-at-its-whole-width` | M | The subdivision's root-slack meter reads an Interval root over its whole enclosure, so the dependency problem refuses about a quarter of the Interval lane's correct shallow-crossing answers |
| P3 | `section-carriers-read-off-acos-near-a-degenerate-pose-carry-unread-rounding` | M | a section arm's carrier read off acos or atan2 near a degenerate pose carries rounding its margin does not read |
| P3 | `the-tube-and-radius-guards-decide-on-the-band-where-check-1-reads-lo` | D | The section arms decide a torus tube, a cylinder radius and a cone aperture on the tolerance band; tier-3 check 1 compares the same datums' lower bound with zero |
| P4 | `the-whole-period-spiric-speed-bound-reads-sin-and-f-apart` | M | The whole-period spiric speed bound reads sup|sin| and inf f apart, so it grows as 1/f_min at the pinch (from CURVED, 2026-10-26) |
|  | `spiric-oval-turn-centre-and-ball-belong-on-the-curve` |  | A spiric oval's turn centre and closed-form ball are derived in topo's join from Curve3::Spiric's fields (from CURVED, 2026-10-26) |
| P4 | `a-narrow-cones-ruling-direction-datum-widens-its-interval-near-the-apex` | E | A narrow cone's ruling direction datum p·cos α − h·sin α widens its Interval enclosure near the apex; 48 rulings at α 0.1, scale 1e3, ε 1e-12 escalate that certify on main |
| P4 | `generic-pose-window-is-tighter-than-the-biquadratic-arm-bound` | E | the ray-torus generic-pose sweep holds roots to an absolute 1e-6 while the biquadratic arm documents a K*eps error, 1e-5 at eps 1e-6 |

## Order

`conic-doors-walk-their-taylor-terms-through-the-harmonics-noise` and `root-slack-meter-reads-an-interval-root-at-its-whole-width` first, because both refuse correct answers on the circle × torus walk that PR 5394 just moved. `section-carriers-read-off-acos-near-a-degenerate-pose-carry-unread-rounding` should be read alongside PR 5156's normalised ellipse frame, which closes part of that class. The P4 rows go in one lane.

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
