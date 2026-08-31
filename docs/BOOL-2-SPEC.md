# BOOL-2 — issue 1011, the cone arm: point_in_solid learns ray×cone

**Binding at dispatch** (S-BOOL program, `docs/S-BOOL-PLAN.md`;
difficulty logged pre-draw: **M/L**, recorded numeric M). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 1011 is the primary specification — the CONE half only; the
torus half is BOOL-3's, its own unit.

## Situation

`point_in_solid` (`crates/topo/src/boolean/solid_contain.rs`, ray-
crossing parity over every face) refuses `Cone` as
`PointInSolidError::KindUnsupported`. PR 1001's operand gate is
pair-scoped, so the gate admits operations that every containment
door then refuses — the join's `resolve_roles_geometric`, `ops`'
shell classification, and `finish`. The issue names this "the
scheduled capability" and its shape: real new geometry, the sphere
closed-group's discipline as the pattern.

## Deliverables

1. **The ray×cone arm**: quadratic intersection + nappe test
   (`(p − apex)·axis` sign) + the axial trim window (the cylinder
   arm's azimuth+height analogue — read that arm first; it is the
   in-file precedent for exact chart trim). Grazing/tangential
   crossings **escalate rather than answer** (the issue's stated
   posture; margined trilean discipline per D4/Q1). The apex is a
   degenerate crossing — decide its posture explicitly and state it
   at the site.
2. **The refusal retires as a D2-row-2 capability landing**,
   classified in the PR body. `KindUnsupported` for `Cone` must be
   grep-gone from the reachable containment paths for cone-bearing
   solids; the `Torus`/`Nurbs` refusals stay, byte-untouched.
3. **Own rows, not borrowed pins**: interior / exterior / near-apex /
   grazing-escalates / axial-window (a point beyond the trimmed
   extent), on solids built through public doors (a real cone body
   through revolve or the primitive door, not hand-assembled faces
   where a door exists). At least one row exercises a downstream
   consumer unlocking (a boolean whose containment question needed
   the cone arm), red on main by `KindUnsupported`.
4. **The two `verbs_gate_r1_probes` pins are TORUS-shaped and stay
   red and untouched** — they flip in BOOL-3. Verify your change
   does not disturb them (they are `#[ignore]`-free red-on-landing
   pins? read their current state and report it — do not "fix"
   them).
5. **ε posture** (issue-1356 discipline): the quadratic's
   discriminant and the escalation margins are toleranced — state
   the bands, make new rows ε-three-outcome honest, and decide
   whether the gate needs a `CI-Config` trailer on your final head
   or the rows are band-relative; say which point gated.
6. **Class sweep** (discipline §5): other `KindUnsupported` raise
   sites whose kind set this arm changes; per-hit disposition.

## Acceptance

- The consumer-unlocking row red on main (`KindUnsupported`), green
  under the arm; the five posture rows green; grazing demonstrably
  escalates (a row pinning the escalation, not just absence of an
  answer).
- Existing boolean/containment suites green; the torus pins in
  whatever state they had on main, verbatim.
- Hosted CI green on the final head, per-item gate record in the PR.

## Hard rules

- NO `Co-Authored-By` trailer and no model names in lane commits.
- Keyword hygiene: "issue 1011" spelled out; the issue stays OPEN at
  merge (the torus half remains) — say so in the PR body.
- Scope fence: `crates/topo/src/boolean/solid_contain.rs` (the cone
  arm + its chart-trim helper), topo's containment/boolean test
  suites, and the new rows. NOT: the torus arm, `splitting/`,
  `census.rs`, `geom-brep`, `sweep/tests/verbs_gate_r1_probes.rs`,
  `docs/MODEL-AB-LOG.md`, `docs/S-BOOL-*.md`, no SMELL table edits.
- VERBS is live on `geom-brep/src/intersect.rs` — you touch nothing
  there; re-merge main before opening the PR.
