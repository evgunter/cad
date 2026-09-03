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
