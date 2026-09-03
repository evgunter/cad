# LIB-TUBE — Node::Tube + Node::HollowTube (recipe doors for the tube pair; G2's tube half)

**Status: spec under ratified `docs/RECIPE-DOORS-DESIGN.md` D1 + D4
AS REVISED by the #1205 ruling (Ev, issue comment + in-chat
sign-off, 2026-08-29). Binding at dispatch. Full model-A/B protocol
unit — a schema break is not mechanical. RECIPE-DOORS unit 2 of 3.**

## Deliverables

1. **Two node kinds, per revised D4**: `Node::Tube { center/axis/
   u_ref anchoring per `Node::Revolve`'s payload precedent (measure
   that precedent and follow it — datums by reference, not raw
   points, wherever Revolve does), major_radius: Expr, window,
   minor_radius: Expr }` and `Node::HollowTube { …the same…, wall:
   Expr }` — wall REQUIRED, no `Option` anywhere in the recipe
   vocabulary. Every compiler-enumerated site for BOTH kinds
   (node.rs's exhaustive matches, eval dispatch with the next TWO
   free content-key tags — read the roster, append, no reuse — the
   payload-hash match, `NodeErrorKind`), canonicalizing construction
   doors, `SlotId` entries: shared kinds for the common parameters
   (major/minor radius, window angles per the Revolve slot
   precedent), a wall slot ONLY on the hollow kind.
2. **Emitters**: `Node::Tube`'s eval arm calls `tube_along_arc`,
   `Node::HollowTube`'s calls `tube_along_arc_hollow` — each its own
   public door; nothing recipe-side re-derives the wall arms'
   verdicts (wall-vs-minor validation stays kernel-side, refusals
   cross typed). Both doors return `Revolved<T>`, so the revolve
   emitter path is the naming template (D4): measure whether it
   applies wholesale or needs a tube-specific translation, and
   REPORT the answer rather than forcing either; if translation is
   needed, no new `RoleSeg` variants without a spec-deviation
   disclosure and an argument.
3. **Schema bump per the dispatch-time-seam discipline**: read
   main's `SCHEMA_VERSION` by eye at branch time (v16 at spec) and
   at EVERY re-merge; claim the next free number; ONE bump covers
   both node kinds (D4); one ledger prose entry, one meaning;
   old-file refusal typed with the standing regenerate recourse;
   regenerate invalidated fixture families by their own documented
   recipes (header-line-only diffs, verified); a red-capable
   vN→vN+1 demonstration row is REQUIRED in this unit (the class R1
   found missing at G16 — don't make a reviewer write it again).
4. **`Node.tube` and `Node.hollow_tube` in pncad-py** — no optional
   wall on `tube`; window crossing per the Revolve angle-binding
   precedent; stubs, ty fixtures, tags; the two kinds' refusal
   families reachable and pinned (the three wall arms only via
   `hollow_tube`).
5. **Census + audit re-cuts, honest**: rows 24 (`tube_along_arc`),
   26 (`hollowelbow`), 27 (`hollowtorus`) flip only as far as each
   row's own claim. Row 26/27's subject is the STORAGE contract —
   both outer half-walls hold the caller's `minor_radius` bit for
   bit, both inner ones hold `minor_radius − wall`, one IEEE
   subtraction (the audit's G2 prose states it) — so the rows'
   oracles measure stored bits, not just volumes. Row 24's "one
   node kind, not two" prose is CORRECTED to the #1205 ruling ("two
   node kinds, one bump") and the G2 gap entry re-counted (the
   tube half discharges; the sweep half stays banked on U4/LQ3,
   rows 20–23 unmoved; row 13 `lily` and row 28 `teapot` STAY NO —
   placement/sweep and shell respectively — and their rows say so).
   Tallies re-derived from the sheet.
6. **Oracles from the scenes' own statements**: `hollowring`'s
   closed forms (V = 2π²R(rₒ² − rᵢ²)) against the full-window
   hollow node; the elbows' Pappus closed forms; a
   solid-minus-hollow differential (the cavity's own closed form)
   discriminating the two kinds within one document — the
   D3-discrimination shape G16's fix pass proved out. Corpus:
   register a tube document and a hollow-tube document in the
   Band-4 registry (small, `die_chamfer`-sized — not tour
   transcriptions), extending the name-table digest gate and
   re-blessing the m10-p fence by the roster procedure (removal-
   alone restores the prior constants; execute the probe row).
7. Python tests with the same closed forms; both kinds' name tables
   pinned; a `select_where`→`hollow_tube` flow row if the selection
   surface reaches it naturally (measure; if it does not, say so
   rather than contriving one).

## Fences

No kernel geometry changes: `tube_along_arc`/`tube_along_arc_hollow`
and the private `build` are UNTOUCHED (the ruling blesses the shared
private implementation as-is). No `wire_sweep`/U4/LQ3 motion — rows
20–23 stay NO. No `Node::Shell` (its own unit, held on #1202). No
new `RoleSeg` variants without disclosure (deliverable 2). No #917
motion. No tour-scene re-authoring beyond what the audit rows'
honesty demands (die_tool-style re-authoring checks are separate
mechanical units).

## Protocol

Full A/B: implementer arm = block LIB-12 slot 4, read back from the
redacted record via git history — the block's LAST slot: when this
unit's reviews conclude, the block OPENS per the per-block opening
rule (PCURVE-1 precedent). The standing LIB-12 contamination flag
rides the dual. Pre-draw fields at this spec: **M-L / STRUCTURAL**.
v6 dual at review (next LIB ordinal claimed at dispatch); blinding
fences as G16's (no trailers in lane commits, no model talk,
lane-private paths).
