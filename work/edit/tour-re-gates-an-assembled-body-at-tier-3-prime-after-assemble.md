---
id: tour-re-gates-an-assembled-body-at-tier-3-prime-after-assemble
kind: issue
title: the tour re-runs tier 3′ over a body assemble already passed, because Assembly hands back no certificate
status: closed
closed: 2026-09-29
opened: 2026-09-28
priority: P4
cost: M
---


Found by the sweep of `gather/assemble-single-local-battery`, which
made `editor_core::assemble` pay tier 3's local battery once per
aggregate. The sweep looked for callers that gate a body and then gate
the same body again.

## The finding

The tour's assembly scenes (`demos/tour/src/assembly.rs`,
`layout_scene` and its siblings) call `assemble`, which runs tier 3′
over the product body and its minted records, and hand
`assembly.body` / `assembly.contacts` to `SceneBody::seamed` or
`SceneBody::at_rest`. `run_body` (`demos/tour/src/main.rs`) then runs
`topo::validate_pseudomanifold_certificate` over that same pair, so
every assembled scene pays the whole tier-3′ pass, battery and census,
a second time.

It does that to get a measurement: `run_body` continues the
certificate it gets back (`continued`), and `Assembly`
(`crates/editor-core/src/assembly.rs`) carries no certificate. The
gate computes one, since `gate_at_rest_declared`'s door makes check 7,
and drops it. `SignCertificate` borrows the body, so an owned
`Assembly` cannot hold one as it stands. The fix is either an
`Assembly` door that measures through the gate's walk (the shape
`validate_geometric_certificate` has) or accepting the double in a
demo. That is a design question for the owner. The cost is the
tour's alone today: no production caller measures an assembly.

## Closed by design (2026-09-29, EDIT orchestrator)

Two answers are offered: an `Assembly` door that measures through the
gate's walk, or accepting the double in a demo. The cost is the tour's
alone, since no production caller measures an assembly, and the first
answer changes kernel code for a demo's measurement. The discipline
every designer and implementer lane works under says non-test code
does not change for a demo or a test (`docs/prompts/designer.md` §4).
So the tour keeps paying the second pass.

If a production caller ever needs an assembly's certificate, that
caller is the reason to add the door. File it then, citing this row.
