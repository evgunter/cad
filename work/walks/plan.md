# WALKS — the plan

one topological relation, implemented n times across topo/src

Opened 2026-09-20 by TOPO's priority-seam cut (`work/README.md`, Track
size). Nothing dispatched yet.

## The slate

Carrying **18 budget points** of dispatchable work against a ceiling of
30 — about one sitting, which is what the cut was for.

| pri | item | cost | title |
|---|---|---|---|
| P1 | `a-faces-loops-are-walked-by-hand-in-thirty-four-places` | D | once(face.outer).chain(face.rings) is written out at every face-loop walk in topo/src — 33 copies at last count, each re-deciding what a non-Cycle boundary means |
| P1 | `edge-carrier-walks-and-tag-matches-outside-the-one-door` | E | Nine hand-written edge to carrier walks and six tag-only carrier comparisons still read past readback::edge_carrier_ref |
| P1 | `euler-characteristic-has-three-carriers` | D | The Euler characteristic has three carriers: readback::EulerCounts, validate's ComponentCounts and review_m1_pr4's ShellComponent |
| P1 | `placeholder-question-has-two-spellings-across-topo` | E | The placeholder question is asked through two doors across topo: NetState::Placeholder and is_placeholder() |
| P1 | `producer-closing-mint-is-a-convention-with-thirteen-copies` | D | every producer's closing pcurve mint is a prose convention spelled thirteen times, unenforced, and it launders a stale operand row |
| P1 | `shell-glue-relation-has-three-implementations` | D | the per-shell component glue relation is implemented three times, and the test-support copy is the one that drifts |
| P1 | `three-answers-to-is-this-loop-inside-that-one` | D | three answers to "is this loop inside that one" with three boundary postures (shell::encloses, chord_join::rehome_rings, validate::ring_nesting), plus a fourth hand read of Surface::Plane's chart normal |
| P1 | `three-refusal-variants-nest-a-certification-error` | E | three EulerOpError variants and one BlendError nest a certification error with four spellings of the same refusal |
| P1 | `void-birth-marking-at-insert-void` | D | voids - structural void-birth marking at insert_void (planned, unscheduled, the eventual outer/void rung) |

## Order

Largest population first, because the collapse door each one needs is the same shape and the widest instance designs it best: `a-faces-loops-are-walked-by-hand-in-thirty-four-places` (33 sites), then `producer-closing-mint-is-a-convention-with-thirteen-copies` (13), then the three-implementation rows. Every unit announces its fence in its own PR — this program claims no paths.

## Review posture

OPEN, for this program's first dispatch. TOPO ran the full v6 dual on
kernel units; Ev took S-TCOST off the protocol entirely on 2026-09-12
and protocol v7 (`docs/MODEL-AB-LOG.md`) runs the dual on triaged-in
units only. Nobody has re-asked the question for this ground, so the
first orchestrator answers it here rather than inheriting an answer.
