# M9-D1 — the revolve pole resolution (spec)

Orchestrator work order for the D1 candidate (M9-PLAN, "sequence
EARLY"). Substrate: fresh exploration 2026-08-15. The defect: the
revolve name emitter resolves vertices by seeded elimination, a
loop whose every vertex is ON-AXIS yields zero seeds
(emit_sweep.rs:341-345 — `rims[v] == None` IS the on-axis marker),
so the poles stay UNRESOLVED and `check_total` kills the whole
table: no sphere reaches `Node::Revolve`, and every recipe wanting
a ball pays the split-at-equator detour.

## Substrate corrections to the M8-LOG D1 entry (recorded here;
the closed log stays as written)

1. The natural sphere carries TWO band faces (the π-band pair,
   pinned V2 E2 F2 R0 in revolve_ball.rs), the workaround FOUR —
   the cost is a per-π-band doubling plus an equator rim/vertex
   pair, not two-vs-one.
2. The diepips/diecomposed STL/STEP frame-axis drift came from the
   ball's AUTHORING-FRAME change (deviation (a)-shaped), not the
   equator split — deleting the workaround will NOT restore the
   raw build's azimuth, and this spec does not promise it.
3. The refusal is NOT full-revolve-only: the partial arm refuses
   identically (emit_sweep.rs:233/239/242) — the fix covers BOTH
   arms.

## The fix (binding): kernel-exported pole keys

The emitter's contract is "exact wiring facts, never geometric
matching" — so the fix is a CONSTRUCTION RECORD, not emitter
cleverness and not a margined verdict:

1. `sweep::Revolved` grows a canonical-indexed pole export
   (`poles`: profile-vertex → VertexKey where on-axis) — full.rs's
   build_wire knows both tips by construction (the mvfs seed and
   the last mev's vertex; the swept→canonical map already exists
   at assembly), and partial.rs's assembly (231-250) exports the
   same.
2. `name_revolve`'s pole resolution becomes a LOOKUP inserting the
   EXISTING `RoleSeg::Pole(pv)` — no new RoleSeg, no serde, no
   schema, no hash/selector/resolve changes (Pole is already a
   leaf in all three).
3. The `UNRESOLVED` all-on-axis refusal text retires (or narrows
   to whatever genuinely remains unresolvable — say which, with
   the construction argument).

Rejected shapes, recorded: half-edge-direction inference (the
θ>0 reversed-chain caveat makes it an undocumented invariant),
recipe-side axial-order tie-breaks (spreads the fact across a
seam), and any margined discriminator (contradicts the module
contract and would mint a flagged K row).

## Acceptance

1. **The natural meridian works end-to-end**: a bulge-1 semicircle
   closed by its on-axis diameter revolves to the named ball —
   both poles named `Pole(pv(0,0))`/`Pole(pv(0,1))`, table total.
   Same for a PARTIAL revolve of an all-on-axis loop (the wedge).
2. **Both workarounds DELETE, plus the twin**: demos/tour/
   diefillet.rs's tan(π/8) half_disc (the natural three-step
   program replaces it), corpus/die_pips.rs deviation (b), AND
   corpus/die_composed.rs's copy of the same program. Counts /
   validity / volumes unchanged where geometry is unchanged; the
   pip lane's F4→F2 band collapse and equator-entity removal are
   the EXPECTED census deltas, stated per fixture; byte identity
   per the demo doctrine's soft rule — NOT promised, drift
   explained if CI's render/STEP lanes move (through the hosted
   pipeline only).
3. **The red row exists**: no test pins today's refusal — add the
   all-on-axis row RED on main first (the refusal), then GREEN
   with the fix (the naming assertions); its removal is the
   natural mutant. m4_pr3_names.rs's wire row keeps its cylinder
   case (the off-axis anchor path must not regress).
4. Name stability: the die_composed authored fourteen and
   excluded_meridians survive verbatim (canonical segment 0 stays
   the lower arc — assert it); no surviving StableName changes;
   nothing N5-dangles.
5. K census: the profile loses a segment so sample COUNTS move —
   re-run the k-lint census; if the gate fires, re-derive per the
   runbook (dimension unchanged, counts only), never retune
   geometry.
6. Coordination: PR #516 (open, LIB) touches names/emit.rs — if a
   NamingError variant is added/removed, merge-main and add the
   Display arm; #516's pins do not cover the UNRESOLVED text.

## Process

Unit protocol: implementer = block M8-15 position 3 (OPUS — the
block's last slot; the next block opens as M9-16). Difficulty
pre-dispatch: **S-M**; task-class: STRUCTURAL (a construction
record + lookup; no numeric decision). One blinded reviewer + fix
pass; review ordinal claimed from the ledger ON MAIN at dispatch.
Standard brief lines (foreground discipline, no trailers,
invariant comments, lane-private publish paths, ε honesty,
merge-main + union).
