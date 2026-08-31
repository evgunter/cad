# MATE-3 — issue 941 items 1–2: declared cusps (the #131 ruling's kernel half)

**Binding at dispatch** (S-MATE program, `docs/S-MATE-PLAN.md`;
difficulty pre-logged in the plan's opening commit: **L**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
The primary specification is the RULING — DESIGN.md's D1 tier-3
material-wedge text (the #131 ruling, ratified 2026-08-23) — with
issue 941's inventory as the implementation map. This unit is items
1–2 ONLY; items 3–5 are handoffs (below), never absorbed.

## Situation

The ratified invariant has no enforcement: today's dihedral pass is
unsigned — wedge 0, 2π, and the legal π all classify `Smooth`
(`crates/geom-brep/src/dihedral.rs`, `DihedralClass`), and
`crates/topo/src/validate.rs` carries the named deferral ("what
tier 3 does NOT yet check"). On the authoring side there is no door:
`PathError::JunctionCusp`'s text says so, pinned by
`crates/profile/tests/path_property.rs::turn_pi_refuses_as_cusp_naming_the_absent_declaration_door`,
while the profile DATA gate already accepts declared carrier-tangent
cusp joints (`judge_joints` is direction-agnostic;
`crates/profile/tests/declared_tangency.rs`).

## Deliverables

1. **The material-side wedge check** (issue 941 item 1). Build the
   edge-local material pairing and implement the FULL ratified
   verdict table in the tier-3 pass:
   - transverse legal at the θ = ε/r margin;
   - π legal (smooth seam);
   - 0 and 2π legal iff DECLARED (the C7 `Tangent` contact
     vocabulary — never inferred from values) AND jet-determinate
     (quadratic transverse separation, κ_rel bounded away from
     zero at `TangentIntersection`'s own margin);
   - in-band κ_rel (osculation) ESCALATES;
   - undeclared cusps REFUSE, always;
   - lamina (conformal contact over a patch) REFUSES — the curve-
     locus condition; zero-volume bodies stay geometric defects.
   **Revert symmetry is a test obligation**: `revert` maps a valid
   declared-cusp body to a valid declared-slit body (wedge 0 ↔ 2π)
   bit-faithfully, and the two are legal together or not at all.
   Every arm of the table gets a red-first or refusing row; the
   escalation arm demonstrates its three-outcome band honesty.
2. **The authoring door** (item 2, PATHS): the cusp analogue of
   `.tangent()` — author the reverse-tangent junction exactly and
   emit the declaration. Retire `PathError::JunctionCusp`'s
   "there is no declaration door for cusps" text in favor of naming
   the verb; the pinned `turn_pi_refuses…` row flips to pin the new
   honest message (say so in the PR). Only the path ALGEBRA needs
   the verb — the data gate already accepts.
3. **Reachability honesty**: if items 1–2 together make a declared-
   cusp SOLID buildable through public doors (a cusp profile
   extruded), every consumer your own tests actually reach must
   refuse TYPED rather than proceed silently; anything you do not
   reach is item 5's sweep, handed off, not silently deferred —
   list what you built, what it touched, and what refused.
4. **The handoffs, recorded not absorbed** (PR body, one line
   each): item 3 (boolean routing — definite-tangent to the
   declaration ladder; VERBS' curved-crossing ground), item 4
   (M9-3 join-lane emission arm), item 5 (the consumer sweep:
   fillet/chamfer, offset/shell, mesh, sector classification,
   export). The orchestrator files or routes them at merge.
5. **Class sweep** (discipline §5): the genus is "an unsigned
   classification collapsing distinct material configurations" —
   sweep the dihedral/wedge consumers for sites reading `Smooth`
   as "π" specifically; hit list with per-hit disposition, blind
   spots stated.

## Acceptance

- Every verdict-table arm exercised; the revert-symmetry rows
  green both directions; the flipped profile pin green; existing
  topo/geom-brep/profile suites green.
- Refusals minted or re-typed classified against the D2 addendum
  in the PR body.
- ε posture (issue-1356): the wedge margins are band-sensitive
  (θ = ε/r, κ_rel margins) — argue the band story explicitly and
  consider pinning a lane with a `CI-Config:` trailer on the head
  commit; say in the PR which point gated, drawn or asked.

## Hard rules

- NO `Co-Authored-By` trailer and no model names in lane commits.
- Keyword hygiene: write "issue 941" spelled out; never a closing
  keyword before a `#`-reference. The unit does NOT close issue
  941 (items 3–5 remain); say so in the PR.
- Scope fence: `crates/geom-brep/src/dihedral.rs` (a seam note:
  geom-brep is PCURVE-adjacent but dihedral.rs is not among its
  named files), `crates/topo/src/validate.rs` (+ its immediate
  helpers), `crates/profile/src/` (the door), and the three
  crates' TEST files. Nothing else — no `boolean/` (item 3 is
  VERBS'), no `census.rs`, no `crates/sweep` source, no mesh, no
  `editor-core`, no `docs/MODEL-AB-LOG.md`, no `docs/S-MATE-*.md`.
- A sibling implementer lane is running concurrently on disjoint
  files; builds may be slow, and you MERGE MAIN before opening the
  PR and whenever it moves.
- Commit and push after every coherent unit of work (branch
  `mate/3-declared-cusps`).
