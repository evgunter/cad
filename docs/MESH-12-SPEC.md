# MESH-12 — issue 1601 and issue 1588: the saturated span's fold, and the rim-continuation witness

**Binding at dispatch** (S-MESH program, `docs/S-MESH-PLAN.md`;
difficulty logged pre-draw: **S/M, recorded S**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issues 1601 and 1588 are the primary specification. Both were filed
from this program's own reviews (MESH-11's R2; MESH-8's dual) and both
are S-MESH ground: `props/curved.rs`'s sphere fold is Track R
(props/curved.rs is R fence ground on this program's leave; the area
lanes, `quad.rs` and `patch_bound.rs` are S-CERT's and are NOT
touched), and the coherence examination is `topo::coherence`, this
program's own (MESH-8). Issue 1598 is NOT this unit's — it is handed
to S-CERT (the closed form's extent premise, issue 723's class).

## Situation

**Issue 1601.** `sphere_meridian_pole_margins` (MESH-11's one home for
the pole-membership test, serving CERT-1's fold and the branch door)
clamps `c_edge = cos(min(dt/2, π))`. For a span `dt > 2π` that leaves
the sign `f = ⟨P, M⟩ + 1`, whose zero set is NOT empty: `P = −M` is an
interior point, δ past `t0` for a span `2π + 2δ`, and the chord copied
onto that sign is ≈ δ·R. Measured (MESH-11 R2, on head and merge
base): a `2π + 2δ` span folds SHORT by `(1 − cos δ)/2`, up to −3.5%
area at δ = 0.375, on 36 of 400 spans, at three ε rows and on the
interval lane. The branch door is unaffected (the other pole sits at
`M`, a definite `Positive`). Reach: hand-built or uncertified spans
only — certification bounds `0 < Δt ≤ τ` per edge, the import door
normalises into `(0, τ]`, and MESH-10's fold refuses a reconstructed
torus span past the winding bound. CERT-1's row
`a_multi_wrap_span_covers_both_poles` passes only because its `f`
rounds to `+0.0`. The two doc sites that claimed "≥ 2π covers the
whole circle, both poles fold" were already corrected by MESH-11 to
state the actual behaviour and cite the issue.

**Issue 1588.** MESH-8 measured that the rim-continuation condition
(`CoherenceCondition::RimContinuation`) is unreachable through
`tessellate` on any natively constructible body: a v-gap between two
edges of one rim row forces one carrier `sqrt(εR)` off the surface,
and MESH-7's shape door refuses the face before the walk. The
relocated condition fires on a synthetic (two circles at 1024 ε) and is
quiet at c = 0, but no committed body reaches it end to end, so the
corpus row cannot include a rim-continuation positive and the
condition's live reach is asserted, not demonstrated.

## FIRST, before the build — two measurements, reported

1. **The saturated span's admission set.** Which callers can hand
   `sphere_meridian_pole_margins` a span past `2π` today: enumerate
   the doors into `sphere_boundary` / `curved_face` (the flux lane's
   `mass_properties`, `boundary_material_sign`, MESH-7's
   `require_iso_rectangle`, MESH-11's `require_one_chart_branch`) and
   for each say whether a `dt > 2π` can arrive (certified: no, by the
   per-edge winding bound; hand-built `LoopEdge`s: yes; MESH-10's
   folded torus meridian: refused past τ — and the SPHERE has no fold
   of its own, say so). Report the set with the row that pins each
   answer.
2. **The rim-continuation route.** Attempt the witness the issue
   asks for through the import door: a STEP file whose rim is stated
   as two circle arcs at slightly different levels, within props'
   band (`props_rim_level` admits) but over the coherence band
   (`RimContinuation` reports). Report whether the shape door admits
   such a face at all, at which ε rows, and whether `tessellate`
   reaches the walk on it. If no ε row admits it, that is the
   finding: the condition is dead through every public door, and
   deliverable 3 records that instead of a fixture.

## Deliverables

1. **The saturated span refuses at the parse** (issue 1601): a span
   whose stored `dt` exceeds the per-edge winding bound τ is not a
   meridian arc the closed form can fold — refuse typed at
   `sphere_boundary` (a named decide on the span against τ, props'
   band, the sphere's radius as the lever — `certify.rs`'s
   `WindingExceeded` is the invariant re-decided here, as MESH-10's
   `props_meridian_pieces_winding` re-decided it for the torus). Do
   NOT make the clamp "genuinely fold both poles" — a span past 2π
   is not a datum the certified world produces, and answering it
   would be a closed form over an uncertified premise. State at the
   helper that its `min(dt/2, π)` clamp is therefore never saturated
   on an admitted span, and delete or re-aim the clamp accordingly
   (keep it only if a row shows an admitted span reaches it).
   Red-first: MESH-11 R2's two fold rows (`r2_the_saturated_span_
   sign_is_a_rounding_residual`, `r2_a_saturated_span_with_the_pole_
   antipodal_to_its_midpoint`, pinned as the limitation with
   direction "short, never long") FLIP to the typed refusal; CERT-1's
   `a_multi_wrap_span_covers_both_poles` — decide with the row's own
   history: it exists to pin a 3π span folding both poles; under
   this deliverable a 3π span REFUSES. That is a change to a CERT-1
   row and must be stated in the PR with the reason (the row pinned
   a behaviour the certified world cannot produce and that was
   numerically false for 36 of 400 neighbours); if S-CERT's
   orchestrator objects on the away channel, the orchestrator holds
   the merge.
2. **Every consumer flips together**: the flux lane, the material
   sign, both doors — the refusal is the parse's, so one row per
   consumer showing the same name; D9: no certified body changes (the
   two-build digest MESH-4 established; the tour byte-stable).
3. **The rim-continuation witness** (issue 1588): per measurement 2,
   EITHER a committed fixture (STEP under `crates/step-import/tests/
   fixtures/`, or an Euler-door body in `mesh/tests/common/
   witness_bodies.rs` if one can pass the shape door) that reaches
   `RimContinuation` through `import_step` → `examine_chart_coherence`
   and through `tessellate`, with the corpus row gaining its
   rim-continuation positive; OR the measured statement that no public
   door admits such a face at any ε row, recorded at the condition's
   doc and in the corpus row's blind-spot paragraph, with the synthetic
   row named as the only witness — and issue 1588 closes on that
   record, saying so.
4. **ε posture** (issue 1356): the new span decide's key, band and
   lever stated per band; `docs/predicate-dimension-audit.md` gains
   its row; three-ε battery; the trailer decision argued (the fold
   decides at the band on both lanes — ask for the interval lane and
   the ε row where the 36/400 spans were measured).
5. **Class sweep** (discipline §5): every other saturating clamp or
   `min(·, π)` in `props/curved.rs` and `topo/chart*.rs` —
   enumerate, disposition (the torus has none after MESH-10; the cone
   arm's nappe test; `unwrap_near`'s half-period).
6. **Issues 1601 and 1588 close at this merge** (say so; keyword
   hygiene — the orchestrator closes). Issue 1598 is cited as handed
   to S-CERT, not touched.

## Acceptance

- A span past τ refuses typed at the parse with every consumer
  agreeing; the fold rows flipped; the CERT-1 row's change stated;
  the rim-continuation condition either witnessed end to end or
  measured dead through every public door and recorded as such; D9
  identical; hosted CI green; gate record per head.

## Hard rules

- NO `Co-Authored-By`, no model names. "issue 1601" / "issue 1588"
  spelled out; no closing keywords (the orchestrator closes).
- Foreground builds only — never arm background waiters for your own
  builds; push after every item (the container restarts).
- Scope fence: `crates/geom-brep/src/props/curved.rs` (the sphere
  parse, the pole-membership helper, the audit row), the geom-brep
  props suites incl. CERT-1's one row as stated; `crates/topo/src/
  coherence.rs` (doc + the corpus row), `crates/topo/tests/`,
  `crates/step-import/tests/` (a fixture and its row), `crates/mesh/
  tests/common/witness_bodies.rs` if the Euler-door route is taken.
  NOT: `props/quad.rs`, `patch_bound.rs`, the area lanes (S-CERT's),
  the flux derivations beyond the refusal's insertion, `certify.rs`,
  the walk, MESH-11's branch door (cite it), `docs/MODEL-AB-LOG.md` /
  `docs/S-MESH-*.md` / SMELL edits. `crates/geom-brep/src/props` is
  Track R fence ground on this program's leave — disclose any Track R
  row's file reached.
- Re-merge main before opening the PR.
