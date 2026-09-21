# ORIGIN — the plan

the D5/N6 identity channel: what a description's source says, and what the Live guard proves

Opened 2026-09-20 by TOPO's priority-seam cut (`work/README.md`, Track
size). Nothing dispatched yet.

## The slate

Carrying **22.5 budget points** of dispatchable work against a ceiling of
30 — about one sitting, which is what the cut was for.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `a-chart-spans-solids-after-move-shells-to-new-solid` | D | move_shells_to_new_solid re-homes a shell without re-minting its surfaces, so a chart can span two solids |
| P0 | `graft-copies-provenance-keys-verbatim` | H | topo: a graft copies every Provenance record verbatim, so key-carrying variants point into the SOURCE arena after boolean::combine |
| P0 | `live-guard-proves-ordering-not-identity` | D | the Live guard compares the spelling of the key looked up, not the key |
| P0 | `two-provenance-free-keys-holding-one-surface-read-as-two-charts` | H | the loop-re-parenting doors read two keys holding one surface as two charts when no GeomSource ties them, and drop rows that were correct |
| P1 | `axis-per-component-source-beside-geom-source` | H | The axis channel's per-component source beside GeomSource (step 3 of WIRE's ratified axis-channel cut) |
| P1 | `kernel-direct-origin-does-not-separate-hand-built-from-derived` | D | GeomOrigin::KernelDirect holds two of the four origins: nothing inside the kernel can tell a hand-built description from a derived one |

## Order

A live wrong answer first, cheapest evidence first within that: `live-guard-proves-ordering-not-identity` and `kernel-direct-origin-does-not-separate-hand-built-from-derived` are the two that can be shown with a unit test before anything is designed; `graft-copies-provenance-keys-verbatim` and `two-provenance-free-keys-holding-one-surface-read-as-two-charts` are the two that need a door. `a-chart-spans-solids-after-move-shells-to-new-solid` carries `needs_ev` and is on Ev's queue since 2026-09-08 — ask it again at the first sitting rather than planning around it.

## Review posture

OPEN, for this program's first dispatch. TOPO ran the full v6 dual on
kernel units; Ev took S-TCOST off the protocol entirely on 2026-09-12
and protocol v7 (`docs/MODEL-AB-LOG.md`) runs the dual on triaged-in
units only. Nobody has re-asked the question for this ground, so the
first orchestrator answers it here rather than inheriting an answer.
