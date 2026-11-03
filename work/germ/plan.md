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
