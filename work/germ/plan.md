# GERM — the plan

The cone and torus operand lanes, and the containment arms that stop short of them.

Opened 2026-09-20 by REACH's priority-seam cut (`work/README.md`,
Track size). GERMCERT, its 2026-10-20 rounding and certificate tail,
and ROOTS and APEX, REACH's root-arithmetic and numeric-frontier
successors of 2026-10-08, merged in on 2026-11-02. Nothing is dispatched.

## The slate

**18 budget points** of dispatchable P0, P1 and unbanded work against a
ceiling of 30 (2026-11-02), after GERMCERT, ROOTS and APEX merged in.
GERM, GERMCERT, ROOTS and APEX measured 6, 3, 6 and 3 points under the budget rule of
2026-11-02 (only P0, P1 and unbanded rows count; Ev, in chat). Every row
the three held moved here by `git mv` with its id and body unchanged, each
with a dated Moved note (`docs/doc-ledger/germcert-roots-and-apex-merge-into-germ.md`).
`python3 scripts/work.py status --program germ` lists the slate. The
rows that set the order:

| pri | item | cost | strand |
|---|---|---|---|
| P1 | `main-red-the-meeting-axes-pose-refuses-its-endpoint-at-1e-12` | — | band reach |
| P1 | `a-near-right-angle-cylinder-pair-stops-at-a-first-order-sector-side` | M | frontier (from APEX) |
| — | `section-class-lane-choices-escalate-in-band` | — | band reach |
| — | `curve-surface-crossing-has-no-kind-pair-table` | — | root lanes (from ROOTS) |
| — | `the-edge-sweeps-line-wall-and-line-sphere-roots-carry-no-root-slack` | — | root lanes (from ROOTS) |
| — | `spiric-oval-turn-centre-and-ball-belong-on-the-curve` | — | margins (from GERMCERT) |
| P2 | `an-oblique-rod-clear-of-a-cone-refuses-the-extent-scan-at-1e-12` | E | band reach |
| P2 | `conic-section-just-past-its-band-fails-certification-downstream` | M | band reach |
| P2 | `a-sliver-crossings-long-bisector-ellipse-is-served-past-representability` | M | band reach |
| P2 | `circle-cylinder-square-arm-root-slack-charges-the-whole-term-bound` | M | root lanes |
| P2 | `conic-quadric-first-harmonic-arm-refuses-crossings-the-ladder-places` | M +design | root lanes |
| P2 | `circle-plane-first-harmonic-has-three-hand-built-copies` | M | root lanes |
| P3 | `conic-doors-walk-their-taylor-terms-through-the-harmonics-noise` | M | margins |
| P3 | `root-slack-meter-reads-an-interval-root-at-its-whole-width` | M | margins |

The four unbanded rows are charged as medium until they are banded and
priced; band them before the first dispatch.

## Order

1. **The P1 rows**, `main-red-the-meeting-axes-pose-refuses-its-endpoint-at-1e-12`
   and `a-near-right-angle-cylinder-pair-stops-at-a-first-order-sector-side`,
   and the unbanded rows, banded and priced first.
2. **Band reach.** `an-oblique-rod-clear-of-a-cone-refuses-the-extent-scan-at-1e-12`
   is cheap and its repro is one row. Then
   `conic-section-just-past-its-band-fails-certification-downstream`, a
   design read on how far past the band a conic section may be served;
   `a-sliver-crossings-long-bisector-ellipse-is-served-past-representability`
   is the same question on the bisector, so take the two together.
3. **The root lanes' three charge rows** (from ROOTS): they are the
   strand's spine and each is measured.
   `conic-quadric-first-harmonic-arm-refuses-crossings-the-ladder-places`
   carries `design: true`: designers first. The coverage rows ride with
   whichever lane builds the arm they cover.
4. **The margins** (from GERMCERT):
   `conic-doors-walk-their-taylor-terms-through-the-harmonics-noise` and
   `root-slack-meter-reads-an-interval-root-at-its-whole-width` first,
   because both refuse correct answers on the circle × torus walk that
   PR 5394 moved. `section-carriers-read-off-acos-near-a-degenerate-pose-carry-unread-rounding`
   is read alongside PR 5156's normalised ellipse frame, which closes
   part of that class. The P4 rows go in one lane.
5. **The frontier** (from APEX): these rows want measurement before
   code. Each states its zone, and a lane first charts it at the three ε
   rows. The far-nappe row is a coverage gap a verifier found surviving
   every mutant (PR 4135's `analysis/reach-verify/4135`), and is the
   cheapest start. `f64-cannot-place-a-shallow-crossing-within-the-finest-band`
   carries `design: true`: designers first.

## Ratified ground (cited, not re-litigated)

- `docs/DESIGN.md` D1–D10 as ratified. The D10 hold lifted when
  INTENT closed (2026-10-30, #6103), so a row that needs a touching
  pose (F0) no longer waits on it; its lane measures the pose on main
  first.
- Ev's PR 5156 decision: a near-square plane × cylinder/cone section is
  the exact Ellipse, decided once on the pose; no reader re-decides its
  kind from the semi-axes.
- Shared ground: see the program's keep_out. Run `scripts/work.py
  territory` on your branch and announce the seam in the PR.

## Review posture

The repo-wide tiers of `memories/orchestration-model.md` and
`docs/DUAL-REVIEW-PROTOCOL.md` (H → concurrent pair, M → the arm byte),
named per unit at dispatch. A lane measures its row's repro on current
main before it builds: most rows were filed before PRs 5152–5394 moved
the ground. Every worktree builds in its own target directory; the ε
knob is `CAD_TOLERANCE_EPS`; test filters are `test(/module/)`, since
sweep and geom-brep tests compile into one `all` binary.

The posture the track opened with: REACH inherited protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. This program's units are candidates where
the failure mode is a confident WRONG answer rather than a refusal —
the first orchestrator names which of its rows those are.

## From GERMTAIL (merged 2026-11-20)

GERM's refusal and structure tail.

Opened 2026-10-20 by GERM's closing cut (`work/README.md`, Track size): GERM measured about 48 budget points against 30. Below its P1 and P2 rows the slate is almost all P3, so it was cut by finding class: this track has the refusals and structure, and GERMCERT has the margins. Nothing has been dispatched.

### The slate

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

### Order

`a-full-revolves-return-half-rim-misses-its-start-by-two-rounded-half-turns` and `square-wall-crossing-at-a-tubes-top-refuses-r-order` first: each is a regular body refused today. The three ring-path rows go in one lane: they edit one file, and the P2 one is its own carriers' evaluators. `interior-loop-cut-in` is H and wants a design read before dispatch (`nurbs-plane-section-has-no-component-arm` moved to SPLICE, which closed it). The P4 rows go in one lane.

### Ratified ground (cited, not re-litigated)

- `docs/DESIGN.md` D1–D10 as ratified. The D10 hold lifted when
  INTENT closed (2026-10-30, #6103), so a row that needs a touching
  pose (F0) no longer waits on it; its lane measures the pose on main
  first.
- Ev's PR 5156 decision: a near-square plane × cylinder/cone section is
  the exact Ellipse, decided once on the pose; no reader re-decides its
  kind from the semi-axes.
- Shared ground: see the program's keep_out. Run `scripts/work.py
  territory` on your branch and announce the seam in the PR.

### Review posture

GERM's, inherited: the tiers of `memories/orchestration-model.md` and
`docs/DUAL-REVIEW-PROTOCOL.md` (H → concurrent pair, M → the arm byte),
rows in `docs/dual-review-log/rows/`. A lane measures its row's repro on
current main before it builds; most rows were filed before PRs 5152–5394
moved the ground. Every worktree builds in its own target directory;
the ε knob is `CAD_TOLERANCE_EPS`; test filters are `test(/module/)`,
since sweep and geom-brep tests compile into one `all` binary.

### Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`,
so the program closes without an exit walk.

## From HONEROOT (merged 2026-11-20)

No orchestrator yet. Live state is `docs/doc-ledger/germtail-and-honeroot-merge-into-germ.md`'s tail and the
item files beside this plan.

Branch prefix **`honeroot/`**; away-channel tag `(HONEROOT orchestrator)`.

### Charter

The boolean's root doors (`circle_roots`, `circle_torus`, the
section ruling, the line doors) and its sector and rim readers decide
every side, order and root on a margin that charges its own rounding,
in the dimension clause (iii) of `[tolerance.margins]` puts it in; no
root off the arc refuses the arc; and every certification row has a
test that fails without it.

### Lanes

- **Root doors**: the subdivision's bare residual, its off-arc
  meter, the line doors' missing meter, the ruling's seam order, the
  Villarceau `F ≡ 0` case and the width row's missing red test.
- **Levers and dimensions**: parameter spans onto the rate pair,
  `rim_wedge`'s bare cosines, `in_sector`'s uncharged sines.
- **The band's edge**: the tangent arc split `sqrt(2·eps·r)` from its
  tangency.

### The slate

**30 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `a-villarceau-circle-lying-on-a-torus-is-unsettled` | M | A Villarceau circle lying on a torus is the F≡0 case the circle×torus root door still answers Uncertain |
| P3 | `an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band` | M | "boolean: an arc tangent to the other operand's face at its end vertex is split where it leaves the zero band, sqrt(2·eps·r) from the tangency, and the sliver collapses the sector arm the tangent descent reads" |
| P3 | `circle-root-walks-decide-sides-on-the-bare-residual` | E | The circle-root subdivision doors decide a piece's side, the ON test and the bisection on the residual's bare value, with no rounding bound |
| P3 | `in-sector-levers-its-sines-without-taking-off-their-rounding` | E | sectors::in_sector levers its sines and cosine without taking off their rounding, as arc_side and apart do |
| P3 | `line-roots-carry-no-root-slack-meter` | M | Line x sphere and line x wall roots carry no root-position meter, where the circle doors refuse an unplaceable root |
| P3 | `parameter-spans-cross-through-levered-not-the-rate-pair` | M | Angular parameter spans cross to metres through Margin::levered, where clause (iii) puts every parameter span on the rate pair |
| P3 | `rim-wedge-decides-bare-cosines-on-the-metre-band` | E | rim_wedge decides three bare cosines against the metre band (seam_rim_traversal, seam_line_traversal, seam_rim_axis_sense), and the dimension audit has no row for them |
| P3 | `root-slack-meters-roots-outside-the-arc` | M | The subdivision's root-slack meter charges every root round the turn, so a root off the arc that it cannot place refuses an arc it does not touch |
| P3 | `ruling-root-order-compares-raw-parameters-across-the-seam` | M | the section ruling's root order compares raw parameters, so two roots ε apart across its ±π seam read a turn apart |
| P3 | `subdivision-width-row-has-no-row-that-fails-without-it` | E | certified_subdivision's width row can be disabled and every topo and sweep row stays green |

#### Arrived from CLEAVE (2026-10-26)

Moved by CLEAVE's priority-seam cut, ids and bodies unchanged, each with a dated `## Slate triage` note (11 points of the total above):

| pri | item | cost | title |
|---|---|---|---|
| P2 | `boolean-in-span-readings-of-grazing-roots-are-levered-by-arc-length` | M | boolean: bool_wall_root_in_span (line × cylinder or sphere) and bool_carrier_cross_in_span read a near-graze root's span by arc length, with no root-slack bound |
| P2 | `circle-torus-root-slack-divides-by-a-spread-sine-read-off-its-cosine` | E | The circle x torus root slack divides by sin_spread = sqrt(1 - c^2), whose own rounding floors at sqrt(2u) near a tangent root, where the acos linearisation it serves has already failed |
| P2 | `rim-identity-decides-centre-radius-and-axis-one-at-a-time` | M | rim_wedge's same_circle decides a rim's centre, radius and axis one at a time, not their sum |
| P2 | `section-cert-equal-aperture-cones-read-the-plane-conic-beside-tilt-and-aperture` | M | section_cert's equal-aperture cone pair serves the plane conic beside the tilt and the aperture gap one at a time |
| P2 | `section-cert-square-wall-reads-its-trace-beside-the-square-tilt` | M | section_cert's square-wall arm reads its trace and fold drift beside the square tilt one term at a time |

### Exit criteria

None set: the program closes when its slate is empty.
