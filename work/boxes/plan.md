# BOXES — the plan

One boolean fact, decided in several places.

Opened 2026-09-20 by REACH's priority-seam cut (`work/README.md`,
Track size). Nothing dispatched yet.

## The slate

**27 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P1 | `D280` | D | census.rs reach_box re-derives FaceBoxRule's arithmetic; make the one box rule answer once |
| P1 | `D95` | D | boolean/combine.rs answers one proof two ways; dispose of the six siblings the unreachable! conversion left |
| P1 | `G9` | D | Unify the two operand-kind gates (S95) and fix chord_join's contradicted placement (S96) |
| P1 | `S173` | D | Move boolean::rest::face_carrier out of boolean/ so the curved half of the one door lives where the header argues |
| P1 | `boolean-declarations-has-no-geometric-producer` | D | API gap — BooleanDeclarations has no geometric producer, so every direct kernel caller hand-writes one |
| P1 | `chart-window-walk-written-twice` | E | One home for the chart-window walk: chord_join::run_azimuth_window and solid_contain::torus_chart_windows are the same construction written twice |
| P1 | `description-staleness-ladder-three-spellings` | D | The description staleness/adjacency-coherence ladder has three hand-kept spellings, already drifted - it wants one home |
| P1 | `signed-penetration-depth` | D | Clearance reports a coincidence, not a signed penetration depth |
| P1 | `sphere-operand-box-is-the-whole-ball` | H | A sphere face's operand box is the whole ball - the same per-kind box class the cone, cylinder and torus arms have left |
| P1 | `the-chord-dip-charge-has-two-homes` | D | The chord-dip charge f2*step^2/8 has two homes and nine spellings: the one in topo/boolean/boxes.rs cannot be depended on from geom-brep |
| P1 | `the-shell-of-a-face-is-scanned-for-where-a-back-pointer-answers` | E | boolean/ops.rs scans every shell's face list for the shell of a face, where Face::shell answers in one lookup |

## Order

Widest population first, because the door each row needs is the same
shape and the widest instance designs it best: `the-chord-dip-charge-has-two-homes`
(two homes, nine spellings), then `D280` (`census.rs`'s `reach_box`
against `FaceBoxRule`), then `description-staleness-ladder-three-spellings`,
which has already drifted and so carries its own evidence.

`sphere-operand-box-is-the-whole-ball` is the one `H` row and the one
with a measurable payoff — it is also PERF's territory by subject, so
say so in the PR.

## Review posture

OPEN, for this program's first dispatch. REACH inherited protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Most rows here are collapses rather than new
logic, which v7 triages OUT; the first orchestrator confirms that
rather than inheriting it.
