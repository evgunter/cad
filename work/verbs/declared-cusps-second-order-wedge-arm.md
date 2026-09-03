---
id: declared-cusps-second-order-wedge-arm
kind: issue
title: Implementation - declared cusps, the second-order wedge arm (#131 ruling)
status: open
opened: 2026-08-23
github: 941
refs: [131, 1423, 1439]
---

## From GitHub issue 941

opened 2026-08-23, 1 comment.

#131 is ruled (with Ev, 2026-08-23): the tier-3 material-wedge invariant gains a **declared second-order arm**. Wedge = 0 (a cusp) and wedge = 2π (a knife slit — the cusp's `revert` image; revert is an involution, so the two are legal together or not at all) are legal iff the tangency is **declared** (the C7 `Tangent` contact vocabulary — never inferred from values) and **jet-determinate**: quadratic transverse separation with κ_rel bounded away from zero, `TangentIntersection`'s own margin. In-band κ_rel (osculation) escalates; undeclared cusps refuse, forever. Laminae are unaffected — conformal contact over a patch fails the curve-locus condition, so zero-volume bodies stay geometric defects (the PR #15 rationale is untouched). A doubled cusp (two material wedges on one tangent line — the kissing union, a slit interior to material) is F2's coincident-distinct-edges class, each edge classifying separately under this rule. The ruling text lives in `docs/DESIGN.md` (D1 tier 3); this issue is the implementation door it names.

Inventory, roughly in dependency order:

1. **The material-side wedge check** (the named deferral in `crates/topo/src/validate.rs`, "what tier 3 does NOT yet check"). Today's dihedral pass is unsigned — wedge 0, 2π, and the legal π all classify `Smooth` (`crates/geom-brep/src/dihedral.rs`, `DihedralClass::{Transverse, Smooth}`), so the ratified invariant has no enforcement at all. Build the edge-local material pairing and implement the full verdict table: transverse with θ = ε/r margin legal; π legal (smooth seam); 0/2π legal iff declared + κ_rel margin; osculation escalates; undeclared refuses; lamina (conformal patch) refuses. Revert symmetry is a test obligation: `revert` maps a valid declared-cusp body to a valid declared-slit body, bit-faithfully.
2. **The authoring door** (PATHS): a cusp analogue of `.tangent()` that authors the reverse-tangent junction exactly and emits the declaration; retire `PathError::JunctionCusp`'s "there is no declaration door for cusps" text (pinned by `crates/profile/tests/path_property.rs::turn_pi_refuses_as_cusp_naming_the_absent_declaration_door`) in favor of naming the verb. The profile *data* gate already accepts declared carrier-tangent cusp joints (`judge_joints` is direction-agnostic; `crates/profile/tests/declared_tangency.rs`), so only the algebra needs the verb.
3. **Boolean routing**: the curved crossing layer today returns byte-identical `CurvedPierceUnsupported` errors for an exact kiss and a 25% interpenetration (`boolean/reduce.rs`, the `bool_circle_curved_clearance` arm collapses `Zero | Negative`). When that layer lands, definite-tangent must route to the declaration ladder (declared ⇒ build the cusp/slit edge from the closed-form locus; undeclared ⇒ the undeclared-tangency refusal), never to the pierce frontier. The locus side already exists: `cylinder_cylinder_section` returns the exact `TangentLine`, and `tangent_locus` (M9-2) certifies the parallel-cylinder kiss.
4. **M9-3 emission**: the join-lane spec admits `Rest` + `Tangent` declarations but only reconciles the wedge-π tube chain; a declared-`Tangent` join whose result carries material on one side of the locus emits a wedge-0/2π edge under this ruling (or the F2 doubled form). The spec needs that arm before `Tangent` joins ship.
5. **Consumer sweep**: every wedge-conditioned consumer names its wedge-0/2π answer or refuses typed at the consumer — fillet/chamfer (a knife edge is unfilletable at any radius), offset/shell, mesh (normal conditioning and sizing at the cusp band — anisotropic-sliver territory), boolean sector classification, export. Typed refusal is an acceptable first answer everywhere; silence is not.

No urgency; sequencing naturally follows M9-3's resumption (item 3/4) with items 1–2 independent of it.

## Comments

**2026-09-01** — orchestrator:

(S-MATE orchestrator) Items 1–2 LANDED (PR #1423, merged): tier 3 enforces the ratified verdict table (transverse / π / declared-0-2π with κ_rel jet-determinacy / lamina / in-band escalation; revert involution bit-checked) and PATHS has the `.cusp()` door (bit-exact ray negation, pinned). Two corpus residuals moved en route: the "lone face flip certifies GREEN" rows in step-export are now honest `LaminaWedge` catches — one genuinely zero-volume, one an orientation defect the message now distinguishes. The issue stays OPEN for the remaining inventory, routed at this merge:

- **Item 3 (boolean routing — definite-tangent to the declaration ladder)**: VERBS' curved-crossing ground; their register when the crossing layer's tangency work resumes.
- **Item 4 (M9-3 join-lane emission)**: the `m9_3_zip` tube-chain row is now a LIVE two-sided witness (undeclared refuses / hand-declared validates) — whoever resumes the join lane inherits it as the red-first for the emission arm.
- **Item 5 (consumer sweep — fillet/chamfer, offset/shell, mesh, sector, export)**: multi-program; typed-refusal-first is the ruling's own acceptable answer, and the reachability list in PR #1423's body says which consumers a built cusp solid can reach today.
- **Two additional handoffs from the unit** (PR body's numbered entries): sweep-side declaration emission (the extrude/revolve doors do not yet emit the profile joint's declaration onto the minted body — the reason the zip row hand-declares), and the two `docs/predicate-dimension-audit.md` rows for the new predicates.

Also filed from the dual review: issue 1439 (the lever-arm fold's consolidation class).

---
_Generated by [Claude Code](https://claude.ai/code)_

## Home

S-MATE's `keep_out` names this issue's remaining halves — the boolean-routing and M9-3 emission arms — as VERBS' ground, and VERBS' charter carries Wave 2's curved boolean breadth.
