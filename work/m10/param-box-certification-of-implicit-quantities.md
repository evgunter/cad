---
id: param-box-certification-of-implicit-quantities
kind: issue
title: certification over a parameter box for IMPLICIT quantities (the frontier M10-7's symbolic identity layer does not reach)
status: open
opened: 2026-09-03
---


(M10 orchestrator) Filed at Ev's direction, 2026-09-03 ("do add a work
item for the frontier to s cert"), from the M10-7 design conversation.

**Context.** M10's E6 driver certifies a parameter-box leaf when every
funnel predicate decides definitely over the box at `Interval`. Today
that fails at ε-scale widths because checked IDENTITIES (an endpoint on
its carrier, consecutive walls cosurface) widen as `[0, c·w]` under
dependency loss. M10-7 (`work/m10/M10-7.md`) closes that for EXPLICIT
quantities: a hash-consed expression DAG over the parameter symbols
rides beside the lane value, and a margin whose polynomial normal form
(exact rational coefficients; sqrt/trig/float literals as opaque atoms)
is identically zero decides `Zero` for every parameter value at any box
width. Non-identity margins keep the numeric path.

**The frontier this item owns.** A quantity computed by ITERATION has no
expression in the parameters — an SSI march point, a projection foot, a
root of a rung-3 meter, a Newton-polished station — so its residual
check is a genuine numeric margin that widens with the box whatever the
symbolic layer does. Sites of that shape in the funnel's identity-shaped
population (PR #1231's sweep of 57 names):
- `ssi_on_locus`, `ssi_on_locus_foot` — `crates/geom-brep/src/ssi/certify.rs:370`, `:441`
  (a marched intersection point's residual against both surfaces; the
  foot of its projection);
- `plane_nurbs_on_locus` — `crates/geom-brep/src/certify.rs:1940`;
- `offset_reanchor_on_carrier` — `crates/topo/src/replace_face.rs:1928`.
Over a box each is the statement "the implicit function x(p) stays on
the locus for all p in the box", which is a parameter-dependent
interval-Newton / Krawczyk certificate per family (the existence and
uniqueness of the root in a box around x(p₀), uniform over p), not a
residual evaluation.

**What is owed here.** (1) The census: which of the 57 identity-shaped
predicates are implicit, on the M10 corpus and the tour — a list, so the
frontier is named rather than assumed (M10-7 measures the explicit
side). (2) Per family, a box-certificate door the driver can call over a
leaf: SSI first (the plate never hits it — plane × cylinder is closed
form — but any two-cylinder fillet or cross-bore does), projections
second. (3) The driver's refusal for an implicit residual it cannot
certify over the box stays TYPED and priced (`Budget`-class today; a
named `Implicit { predicate }` reason would make the frontier visible in
every accounting).

**Keep-out note.** S-CERT's program record names `ssi*` as Track Q ground
behind PCURVE P-2; the SSI family's certificate should be executed with
PCURVE, this item being the frontier's home rather than a claim on that
code. Not blocking M10-7 (explicit quantities), which is what the
two-hole plate needs.

---

**The census landed** (M10-7, 2026-09-04). PR #1231's two sweeps re-run
on M10-7's head find 66 identity-shaped names (57 at that PR's merge
base). Buckets, with the evidence, in `geom_core::sym`'s module docs and
M10-7's PR body:

- **IMPLICIT — exactly the four this item already names**, confirmed
  rather than widened: `ssi_on_locus`, `ssi_on_locus_foot`,
  `plane_nurbs_on_locus`, `offset_reanchor_on_carrier`.
- NOT A PREDICATE — 6 (`coincide`, `coincident`, `carrier_kind`,
  `carrier_in_chain`, `rebind_identity`, `frame_coincidence`).
- EXPLICIT — 56, of which 8 are measured discharging symbolically on the
  M10 fixtures and the tour's plate.

**And one family that is EXPLICIT and this tier still misses**, which
belongs beside the frontier even though it is not implicit: an ARC rim's
endpoint pinning. A swept arc's carrier is
`Circle { center: c, radius: r, u_ref: (q − c).normalize() }`
(`crates/sweep/src/swept.rs`), so `‖carrier.eval(0) − q‖` is zero iff
`‖q − c‖ = r` — a fact about the RADIUS'S SIGN, not a rational-function
identity, so no normal form reaches it. Measured: the two-hole plate's
whole-certifying ceiling is unmoved by the tier (7.81e-7 of the real
study either way) and `carrier_endpoint_start` is the first refusal
beyond it, where the straight-walled slab's ceiling moves by ~8·10^9.
E12's reserved recourse — a provenance token, "built as the arc's far
endpoint", discharged structurally and verified at the f64 witness — is
the shape that closes it; this item is where it should be scheduled
against, since the driver's refusal for it is the same typed-and-priced
refusal the implicit rows get.
