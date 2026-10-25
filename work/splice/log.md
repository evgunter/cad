# SPLICE — the log

## 2026-10-25 — opened

Opened by a refold approved in chat, before any orchestrator started on
ORBIT or SECT. Twelve rows moved by `git mv`, ids and bodies unchanged,
each with a dated Moved note: the gate deletion from ORBIT; the sites it
waits on from ROOTS and GAUGE; the crossing rung from REACHHOLD and the
spline join from HONE, both un-parked (their parks waited for a first
producer of spline edges between faces, and every loft and sweep is
one); two touch and walk rows from CLEAVE (P3, taken on request); one
touch row from APEX; two flush rows from SEAMRANK; two GAUGE test rows
that go live with the deletion. Every row is P0 or P1: the work here is
what blocks the klein, lily and teapot joins. 29 budget points.

## 2026-10-25 — orchestrator picks up the track

The SPLICE orchestrator (`session_016P4GehXjm4s8N46wtAg2FB`, a cloud
session) holds the track from here.

**Overlap with INTENT / D10.** SPLICE's work is kernel-side (operand
edges, crossings, the join, the flush readings) and does not touch
dependency, placement or intent, so the D10 refactor hold
(`d10-one-way-to-say-intent-is-unbuilt`, DESIGN.md "State of the
build") does not hold any row here. Two seams to watch:
- D′3 (PR 5636, `intent/dprime3-assemble-product-slot`, unmerged)
  edits `crates/editor-core/src/names/emit_union.rs`, on this track's
  ground. Its change is additive (`repeated_member_view`) and does not
  touch `Flush`, so it does not collide with the two flush rows; the
  flush unit, dispatched last, takes whatever main carries then.
- Stage 4 E and F are built: booleans glue on a Zero verdict and the
  boolean declare surface is retired. So no lane here declares a
  contact on a boolean. Where a row's prose says "a declared `Rest`
  contact would have to accept" (the klein rims, in
  `delete-the-boolean-operand-edge-gate`), it is read as a coincidence
  decided from values under `[coincidence]`.

**Dispatch.** Lanes run as their own cloud sessions: this container is
4 vCPU and builds one heavy cargo job at a time, and every unit here
seeds topo. Three run in parallel:
- `splice/gate`: the sites typed, both sector walks' NURBS
  departure, and the gate deleted, in one PR (the sites are only
  reachable, and their rows only red, once the gate is gone; PR 3984's
  attempt took the same shape). Tier: **dual**, because it is new
  incidence under every boolean (plan, "Review posture"). The two
  split-side P1 rows (`section-area-skips-…`,
  `split-section-boundary-…`) sit behind the SPLIT gate
  (`splitting::classify::gate_operand`), not this one; the lane
  measures whether the boolean reaches them and takes them only if it
  does.
- `splice/touch-doors`: `the-touch-and-cross-doors-…` and
  `a-touch-beside-a-spline-…`, both in `carrier_touch` /
  `carrier_cross`. Tier: single, full (certified distance bounds).
- `splice/carrier-walk-spline`: `carrier-walk-has-no-crossing-row-…`.
  Tier: single, full (a certified crossing count).

Held for the gate: the crossing rung, then the spline join, then the
two flush rows.

Lane sessions: `splice/gate` `session_01P78fY3Hgsq13g359bgcNLP`;
`splice/touch-doors` `session_01V5dB8mxDac8zAFBWSEmdT7`;
`splice/carrier-walk-spline` `session_019eib6yHkmVaSGLLzEVtvhK`.

## 2026-10-25 — the SECT seam gets a row

Agreed in chat: SECT gets its own orchestrator rather than SPLICE moving
rows across. SECT's ownership ruling forbids a stopgap, and the arm is
built on SECT's critical path. Filed SECT's
`a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm`, parked on
SECT step 1 and on `delete-the-boolean-operand-edge-gate`. Lily wall 8
stays this track's first demo witness.
