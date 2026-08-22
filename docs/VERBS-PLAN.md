# VERBS — the modeling-verb breadth program (plan)

Executes `docs/KERNEL-VERBS.md` (the register): the missing modeling
verbs whose prerequisites are already ratified, in dependency order,
plus the register's verb-gating defect rows. Kicked off by Evan
in-chat 2026-08-21. Branch prefix **`verbs/`** (orchestrator worktree
branch `mngr/kernel-verbs` predates the prefix and is armed
alongside). Narrative record: `docs/VERBS-LOG.md`. The register stays
the reference view; rows scheduled here are marked there only by this
plan's existence — the register itself never schedules.

**Kickoff rulings (Evan, in-chat 2026-08-21):**

- Wave order as below; an issue-scan subagent surveys open GitHub
  issues first because several plausibly touch fillet machinery — its
  findings fold into Wave 1 specs before dispatch. **This plan is
  PROVISIONAL until that fold-in; unit cuts below may move.**
- VERBS owns the register's **verb-gating** defect rows: #554 (closed-
  rim lever arm), `FullRevolveHoles`, and `tube_along_arc`'s missing
  wall parameter. The other defect rows (#555 mesh sub-floor, the
  loft U-turn gate, edge-selection reach) stay with their home
  programs / the #614 routing.
- Design conversations open **as soon as the relevant info is
  available** — likely most now — rather than queuing behind their
  implementation waves. Each is a design-conversation PR per the
  standing rule (Evan sign-off before merge).
- The C7 / declared-conformal row stays M9's. Helix: the register
  names #222 (now CLOSED) as blocker — the scan verifies whether the
  frontier actually retired; helix schedules only after that verdict.

## Wave 1 — cheap, already-ratified plumbing

1. **VERBS-RIM (#554, S/M)** — the fillet battery's lever arm is the
   endpoint chord, ~0 on every closed rim, so full revolves get a
   FALSE `TangentialEdge` on 30° dihedrals. Fix the metering
   (`crates/sweep/src/fillet/battery.rs`, `extent_of` +
   `convexity_at`); closed rims then report honestly (today that
   means `SpineUnsupported` until unit 3 lands). First because it
   gates every fillet consumer on a solid of revolution.
2. **VERBS-CHAMFER (M)** — the fillet's ruled-surface sibling: swap
   the rolling-ball band for a ruled strip over the existing
   trimline/support-split infrastructure (M5 PR 12 + M6-1 surgery).
   The register's cheapest verb row.
3. **VERBS-ARMS (L)** — the C8-ratified analytic constant-radius
   fillet arms `classify_arm` never implemented: sphere×cone,
   cone×plane, cone×cone, sphere×sphere, and the cylinder pairs
   (CURVED-DESIGN C8: circular-arc spine → torus patch, cone cases →
   cone/torus; `PlaneSphereTorus` already mints the torus). Consumer:
   the calochortus bud's sphere–cone seam (#319). The coaxial arms
   ride along per the register's own note (meridian-arc authoring
   stays the better answer where it applies — no consumer claimed).
4. **VERBS-TUBEWALL (S)** — `tube_along_arc` grows a wall/inner-
   radius parameter so hollow tubes keep the door's exact-intent
   storage. No design record yet: the unit PR carries the (small)
   design elaboration; self-merges only if it stays a faithful
   elaboration of the door's existing contract.
5. **VERBS-RING (M)** — retire `FullRevolveHoles`: the per-hole seam
   surgery the revolve's own docs name as mechanical-but-unexercised
   M2 scope. One-call hollow ring becomes available (register wall 6).

## Wave 2 — curved boolean breadth

The banked germ-chord lanes (DESIGN frontier (d)); the SSI lift
already removed the storage half. Each lane its own unit:

6. **VERBS-GATE (M)** — the operand gate is per-face-KIND over the
   whole body, so one cone/torus face makes every boolean unavailable
   to the body (Klein bottle walls 3–4). Re-scope the refusal to the
   face pairs that actually meet, refusing typed only where an
   unsupported KIND pair genuinely intersects. Spec must rule what
   "genuinely intersects" costs (box-level conservatism is the
   likely shape) — dispatch after Wave 1 lands evidence.
7. **VERBS-CYLSPH (L)** — cylinder×sphere germ lane.
8. **VERBS-SPHSPH (M/L)** — sphere×sphere germ lane.
9. **VERBS-CONE (L)** — cone (and torus) operand lanes, sequenced on
   what 6–8 learn.

## Wave 3 — Q8: offset → shell → the teapot

Gated on the Q8 design conversation (below). Anticipated cut, to be
re-cut at ratification: **VERBS-OFF-A** (analytic kinds, closed under
offset — D3 payoff); **VERBS-OFF-N** (the approximating-surface
machinery: intensional `Offset(S,d)`, fit, certified residual ≤ ε,
mirroring fitted intersection curves); **VERBS-SHELL** (open-shell /
face-removal vocabulary per D1 + the verb); the **Utah teapot demo**
(the verb's designated demo, Evan 2026-08-09) with the Klein bottle's
hand-authored double-offset walls as the second consumer.

## Design conversations (Evan-paced; open as info firms up)

- **Q8 offset/shell elaboration** — first; gates Wave 3.
- **Draft** — "no design record yet — needs its own conversation"
  (register). Face-replacement surgery generalized + tapered mint.
- **Patterns/mirror (D8)** — recipe-level instancing; pattern indices
  are a ratified naming-doc requirement; mirror needs reflection
  instancing + the D9 conv. 4 equivariance frame. Blocks hole
  features.
- **Sheet bodies (D1 extension)** — the named non-manifold trigger;
  a real D1 conversation, not a feature.
- **Point-section loft tier 1** — cheap when a consumer appears; by
  ruling ("mark it down for the future") not opened until one does.

## Out of this program

C7/REST joins (M9); variable-radius fillet (frontier (f),
consumer-gated); hole features / rib / text / datums (behind patterns
or far tail); spheroid primitive (unclaimed, no consumer pressure
beyond lily wall 4); #555 and the loft U-turn gate (not verb-gating,
per kickoff ruling).

## Protocol

Implementer dispatches ride the A/B ledger (`docs/MODEL-AB-LOG.md` on
main at dispatch — block draws, ordinals, dual-review sampling, v5
review lanes). Briefs point at `docs/prompts/implementer-discipline.md`
and reviews at `docs/prompts/reviewer-style-lane.md`, by path. Unit
branches `verbs/<unit>`; lanes via `local-scripts/new-lane.sh`;
state-sync docs PRs at every pipeline seam.
