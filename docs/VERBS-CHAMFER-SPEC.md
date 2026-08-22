# VERBS-CHAMFER — the fillet's ruled sibling

Unit 2 of `docs/VERBS-PLAN.md` Wave 1. Branch `verbs/chamfer`, PR to
main. Difficulty logged pre-dispatch: **M**. The register calls this
the cheapest verb row: the rolling-ball band becomes a ruled strip
over infrastructure the fillet already built (M5 PR 12 battery +
M6-1 surgery + the analytic blend table).

## The verb

`chamfer_edges(body, edges, distance, band)` — a sibling of
`fillet_edges` in `crates/sweep`: replace each requested edge's
neighborhood with a flat strip at equal setback `d` along both
supports. **Scope v1: plane–plane support pairs only**, refusing
typed on anything else (the same honest table shape as
`classify_arm`; the curved-support chamfer rides VERBS-ARMS'
machinery later). For a straight edge with plane supports both
trimlines are lines parallel to the edge, so the strip is an exact
`Surface::Plane` — the analytic case the register promises. The
symmetric-distance form is v1's whole parameter surface; no
distance–distance or distance–angle parameters yet (add later as a
widening, not a refusal).

## Reuse map (read these before writing anything)

- **Battery** (`crates/sweep/src/fillet/battery.rs`): the predicate
  suite applies with the ball radius replaced by the setback —
  convexity-sign constancy (a chamfer needs a side exactly as a
  fillet does; flips refuse), chain G1, corner configuration, and
  face consumption (the `face_clearance` shape with the chamfer's
  setbacks). Spine regularity and radius-vs-curvature headroom are
  ball facts and do NOT transfer — do not meter vacuous predicates.
  Note the lever arm is now the CHAIN_SAMPLES pairwise-chord
  functional (post-#910); consume `Link::arm_len`, never re-derive.
- **Blend table** (`fillet/blend.rs`): `plane_plane_blend` is the
  template — the chamfer's analog derives the two trimlines and the
  strip plane in closed form. Mint exact intent-derived geometry
  (the `EdgeBlend`-shape struct with the strip surface + two
  trimlines).
- **Surgery** (`fillet/surgery.rs`): the graft executor is the
  reuse target — decide-everything-first, clone, Euler-ops-only,
  attach surfaces → intrinsic edge descriptions → whole-body
  `mint_pcurves`, validate once at the end. A chamfer plan is a
  fillet plan with a different band surface; if the executor needs
  parameterizing rather than copying, parameterize it (a reviewer
  has already flagged near-parallel copies as this codebase's
  standing failure class) — but do NOT rewrite surgery beyond what
  the parameterization needs.
- **Naming** (`fillet/naming.rs`): birth records at mint time,
  survivors need no rows, `output = (source − dead) ⊎ minted`.
  Chamfer strips and corner patches get their own role vocabulary
  only if the existing fillet roles genuinely cannot say them —
  prefer reuse; a new `RoleSeg` is a naming-design touch and must be
  called out in the PR body if taken.

## Corners

Three chamfered edges meeting at a trihedral corner truncate it with
a **planar corner patch** (the sphere-octant's flat analog — the
plane through the three trimline endpoints). Constraints:

- **#644 is live in this code**: `corner_ball` is
  convexity-parametric in name only (literal `true` at both call
  sites; feet and octant axis are convex-only). Do NOT copy that
  pattern into the chamfer corner, and do NOT "fix" corner_ball's
  four convex-only arguments piecemeal (deriving one alone makes it
  more incoherent — the issue's own warning). The chamfer corner is
  new code: make ITS convexity handling coherent from birth, and
  refuse typed on corner configurations v1 does not cover (the OQ6
  vocabulary: `FilletCornerUnsupported`-shaped payloads with the
  corner tags — reuse the existing refusal types where they fit).
- Everything else at corners (valence ≠ 3, mixed
  chamfered/unchamfered incidence beyond the run-out policies)
  refuses typed per OQ6's ratified two-policy vocabulary.

## Fences

- Plane–plane only; no curved-support arms (VERBS-ARMS), no new
  metered predicate names where an existing one is honest, no
  enclosing-tangency spellings (#827 is open — invent no vocabulary
  for it), no `corner_ball` refactor (#644 stays), no signature
  tightening (#883 parked).
- If the chamfer needs a naming emitter, copy `emit_topo`'s TieRows
  deferral shape (#708: `emit_fillet` does not propagate an
  upstream tie; do not replicate that known hazard).
- No tessellation-gate acceptance rows (#746/#782 make the join
  suspect); pin geometry via census/Euler, mass properties, and
  validate instead.

## Acceptance

- **The chamfered cube**: all 12 edges of a box at equal setback →
  26 faces (6 squares, 12 strips, 8 corner triangles), tier-3
  valid; census/Euler pinned; mass properties against the closed
  form (volume = a³ − corner/edge deficits, computable exactly);
  STEP export round-trips.
- **Partial requests**: a single edge chamfered end-to-end between
  two corners refuses or runs out per the OQ6 policy actually
  implemented — the refusal payloads pinned (the box's single-edge
  case is what a consumer tries first; give it an honest typed
  answer, as `fillet_edges` does with `UnsupportedRunOut`).
- **Convexity**: a concave plane–plane edge (an L-bracket's inner
  edge — author the fixture through the public API) chamfers or
  refuses HONESTLY — whichever v1 ships, the verdict must be
  decided, correct, and pinned; if concave strips are deferred, the
  refusal names the deferral and the PR body schedules it (v5).
- Existing fillet suites untouched and green (the surgery
  parameterization, if any, must be behavior-preserving for
  fillets — bit-identical fillet outputs are the cheap proof).
- Demo: extend the tour minimally (a chamfered die or bracket
  scene) per the demo-purpose rule — natural authoring, findings
  recorded not dodged.

## Lane obligations

Read `docs/prompts/implementer-discipline.md` in full. Merge
`origin/main` before opening the PR; re-merge if main moves. No
Co-Authored-By trailer in lane commits (blinding). PR body drafts at
`~/.local/share/cad-work/verbs-chamfer-*.md`. Push after every
coherent unit. Hosted CI is the record.
