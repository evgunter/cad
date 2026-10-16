---
id: the-two-carrier-face-pair-confirm-escalates-where-its-second-face-reads-the-point-out
kind: issue
title: The census's tangent-line face-pair confirm returns on the first face's escalation, so a midpoint the second face decides out reads escalated, not stale
status: open
opened: 2026-10-13
priority: P2
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
decides out (`census::curved::in_both`, which `curved_interiors_meet`
also uses per candidate). The fix here is that helper over the two
`face_holds` readings, the `Some(None)` refusal pushed inside the
reading. Found by PR 4956's §5 sweep for conjunctions of cell readings
that short-circuit; it needs a fixture whose tangent line's midpoint is
in band of one face's boundary and out of the other.
