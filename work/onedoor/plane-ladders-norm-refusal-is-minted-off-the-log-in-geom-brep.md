---
id: plane-ladders-norm-refusal-is-minted-off-the-log-in-geom-brep
kind: issue
title: The plane ladder's Norm refusal mints an Indeterminate off the frame's log, which geom-brep's escalation census refuses
status: open
opened: 2026-11-13
---

Found by `carrier-eq-and-plane-eq-take-no-body` (branch `onedoor/carrier-eq-home`), which stopped here rather than decide it.

Moving `plane_eq.rs` to `geom-brep`'s root brings it under a rule `topo` never held: `geom-brep/tests/kstats_escalation_channel.rs` (`shipped_geom_brep_code_spells_no_indeterminate_literal`) refuses any `Indeterminate` literal in shipped `geom-brep` code, because an escalation not taken back from a funnel door is on no frame's log. On the branch it reds with exactly one site, `plane_eq.rs:200`: `geom_brep::plane_eq::unreadable_norm` builds `Indeterminate { margin: MarginDiag::INVALID, predicate: Some("plane_eq_parallel"), .. }` for the parallelism rung's norm decided definitely negative (`plane_ladder`'s `Sign::Negative` arm), which reaches the caller as `CarrierEqError::Escalated { rung: PlaneRung::Norm, .. }`.

The fix is not mechanical, which is why the move did not make it:
- **Through the funnel** (`geom_core::k_stats::gate_rejection(name, decided, band, passes)`, the door `dihedral.rs` and `pair_section.rs` use): the refusal would carry the decided negative margin tagged as a rejected sign, not `MarginDiag::INVALID`, and would go on the log. That changes what `PlaneRung::Norm` reports ("poisoned input, with no margin to report") and what its readers read: `topo::boolean::refusal_routes`' `an_unreadable_norm_and_the_neighbour_gate_end_where_their_door_reaches` pins the kernel-defect ending on that payload, and `BooleanError::plane_identity` renders it.
- **As a typed poison**: a `CarrierEqError` arm of its own for an unreadable norm, with no `Indeterminate` at all. That is a new arm on public API.

Related ground: `work/topo/plane-offset-rung-decided-zero-shares-invalid-with-a-poisoned-margin.md` (the same `INVALID` margin shared by two meanings).

Until this is ruled, the move cannot land green on the PR gate (the row is in the default suite).
