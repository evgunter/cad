---
id: flush-detector-widening-to-curved-rungs
kind: issue
title: Widening the flush detector to the curved Rest rungs is a one-identifier door swap that changes the document detector's answers
status: closed
opened: 2026-09-02
closed: 2026-09-05
github: 1537
refs: [1531]
---

## From GitHub issue 1537

Opened 2026-09-02; 0 comments.

(SEAT orchestrator) The fork SEAT-3 (PR #1531) measured and deliberately did not take; filed so the SELECT-DESIGN scope paragraph has a concrete home instead of "wants its own unit".

**The measurement** (asserted live in `demos/tour/src/twopeg.rs::seat3_measurements`): `carrier_pair_relation` — the door the declared-`Rest` verify path actually runs — already carries plane/sphere/cylinder/torus rungs. On twopeg's peg/bore pair, `declared: true` → `Ok(SameOpposite)` and `declared: false` → `Err(Undeclared { predicate: "carrier_cyl_axis_parallel", relation: SameOpposite })` — the same "would verify if declared" encoding the planar detector reads as a finding. `PlaneRelation` IS `CarrierRelation` and `PlaneEqError` IS `CarrierEqError` (type aliases), so widening `topo::flush::pair_finding` from `flush_pair_relation` to `carrier_pair_relation` is literally a one-identifier swap — **no verify table moves** (the anti-twin rule survives by construction).

**Why it was not taken in SEAT-3**: the widening changes both seats at once — the document door's `find_flush_candidates` answers grow curved findings (behavior SEAT-3 pinned unchanged), and the demo tree has scenes whose walls are pinned on curved contacts refusing (the lily's stem glue, downstream of the shared `flush_declarations` helper). Taking it means re-adjudicating those pins and the twopeg/lily hand-assembled cylindrical declarations deliberately (they would collapse — twopeg's 18 cylindrical declarations become detector findings), which is content review, not a side effect.

**What the unit would do**: the identifier swap; the document-door behavior change adjudicated and its pins re-baselined; twopeg's nine-per-peg and lily's socket re-authored through the detector; the SELECT-DESIGN scope paragraph updated from "planar" to the verify ladder's true reach; a measured statement of which demo walls change and why each change is right.

## Home

`work/seat/` — `crates/topo/src/flush.rs` is a SEAT territory glob and the flush detector at the body seat is §1 of `docs/VERB-SEAT-DESIGN.md`, SEAT's charter.

## Closed

SEAT-FW (PR PRNUM, 2026-09-05). The swap is one identifier in
`topo::flush::pair_finding` and no verify table moved, exactly as
measured here; the unit was the content review it forces. Both seats'
answers grew curved findings (the twopeg parts go from 7 findings to
25 — the eighteen peg-against-bore pairs), twopeg's eighteen hand
declarations and the lily's socket now come out of the detector's own
report, and every one of the tour's 45 scenes renders byte-identical
STL and STEP: the declared sets are the same sets by another road.
The stem-glue pin STAYS — its refusal was never the detector's
blindness (the stem and arch share only their disk; their tube walls
are tori about different ring centres) — and the socket's refusal
stays too, at the reduction's curved-face arm, which is #1032 and not
this door. G19 closed at the document seat with it
(`docs/guide/north-star-audit.md` row 38).

Two residues, each with its own file:
`work/seat/flush-pair-relation-has-no-caller.md` and
`work/seat/verb-seat-design-s3-names-the-planar-verifier.md`.
