---
id: run-side-arc-rule-reads-only-the-run-at-each-end
kind: issue
title: The run-side arc rule reads only the run's tangent at each chord end, so two reflex run ends could agree on the wrong arc
status: open
opened: 2026-10-02
priority: P3
cost: M
---


## What

Found by the `reach/tilted-sphere-pair` lane, which wrote the rule.
`chord_join::select_arc_by_run_side` decides, at each run end, which
candidate arc leaves on the run's left (`split_arc_run_side`) and
requires both ends to agree (`ArcSideCase::EndsDisagree` otherwise).
Leaving on the run's left is exact where the run end is a smooth point
or a convex corner of the divided face — the face's sector there is at
most a half-turn and the two candidates leave in opposite directions.
At a REFLEX corner the divided face's corner between run and chord can
exceed a half-turn, and the wrong candidate can leave on the run's
left. One reflex end disagrees with a smooth one and refuses; two
reflex ends misleading the same way would select the wrong arc.

Every chord end the shipped poses produce is a pierce in an edge's
interior (a smooth boundary point), so no measured pose reaches it; it
is a premise stated in the rule's doc, not a gate.

## What a fix owes

Read the divided face's full sector at each end — the run's tangent and
the neighbouring boundary edge's — and require the candidate inside it;
or gate the run ends smooth-or-convex and refuse otherwise. A fixture
with a reflex corner at a pierce site (a sphere face notched by a prior
cut, pierced at the notch's corner) is the row either owes.
