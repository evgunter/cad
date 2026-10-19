---
id: the-two-carrier-face-pair-confirm-escalates-where-its-second-face-reads-the-point-out
kind: issue
title: The census's tangent-line face-pair confirm returns on the first face's escalation, so a midpoint the second face decides out reads escalated, not stale
status: closed
opened: 2026-10-13
closed: 2026-10-13
priority: P2
cost: M
---


`census::confirm_face_pair` (`crates/topo/src/census.rs`), the
tangent-line lane after `boolean::tangent_pair_at_rest`, places the
overlap's midpoint `q` in each face in turn (`for face in [a, b]`,
`face_holds`) and returns at the first `None` (an escalation, pushed).
Where `a` reads `q` in band and `b` decides it `Out`, the record is
decidedly stale, yet the confirm answers escalated.

Same defect as PR 4956's review finding 2 on the point-touch lane
(`census::point::face_face` / `edge_face` closures): there it is fixed by
reading both cells and keeping the escalations only when neither
decides out (`census::in_both`, which `curved_interiors_meet`
also uses per candidate). The fix here is that helper over the two
`face_holds` readings, the `Some(None)` refusal pushed inside the
reading. Found by PR 4956's §5 sweep for conjunctions of cell readings
that short-circuit; it needs a fixture whose tangent line's midpoint is
in band of one face's boundary and out of the other.

## 2026-10-17 — parked on the D10 hold, moved from RESTREAD

RESTREAD read this row against the D10 hold (`docs/DESIGN.md`, State of
the build) and the open [ev] PR 4666 (the census records what it
decides): it stands on contact records, declared pairs or the shape of
at-rest findings, which INTENT stages 4 and 5 rebuild. Per RESTREAD's
plan (a row needing declared pairs or contact parks and moves to
CONTACTHOLD), it moved here by `git mv` with its id unchanged. Priced
`M` where it carried no cost. (RESTREAD orchestrator)

Closed by CONTACTHOLD's `contacthold/one-tangent-interiors-reading`:
the confirm no longer places one point. It reads the line through both
faces with the op's own reading, `boolean::line_through_both_faces`,
whose stations each read both faces and settle on either deciding out
(the `census::in_both` rule), so a station the second face
decides out reads out whatever the first left unread. The same rule in
the op is witnessed by `review_4831_probes::p10_the_sliver_family`,
whose fat-disc-first rows escalated on exactly that shape and now answer.
