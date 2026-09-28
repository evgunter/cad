---
id: product-gate-refuses-a-declared-cusp-sweep-the-verb-now-declares
kind: issue
title: the recipe layer drops a sweep's carried cusp declarations and the product gate reads none, so a document with a .cusp() extrude refuses as UndeclaredCusp
status: review
opened: 2026-09-25
priority: P1
cost: H
branch: gather/derive-cusp-legality
---


Found by BAND's `sweep-emits-no-contact-record-for-declared-cusps`
unit, following the carried declarations downstream. Measured:
`crates/editor-core/tests/m10_2_measure.rs`'s
`a_cusp_extrude_document_refuses_at_the_product_gate` evaluates a
document whose one root extrudes a `ProgramStep::Cusp` lune and pins
the `RootInvalid` / `UndeclaredCusp` refusal (it read `ProductInvalid`
before PR 3323 moved the product gate) — this item's red-first row,
which the fix flips to gathering.

## The mechanic

`sweep::extrude`, `sweep::revolve` and `sweep::loft_body` now carry the
`Tangent` contacts a profile's declared cusp joints imply
(`Extruded::declared_contacts`, `Revolved::declared_contacts`,
`Lofted::declared_contacts`, built by `cusp_contacts` in
`crates/sweep/src/swept.rs`).
The body such a verb returns refuses `topo::validate_geometric` as
`ValidationError::UndeclaredCusp` and passes
`topo::validate_geometric_declared` with that record in hand.

The recipe layer can author the joint — `ProgramStep::Cusp` lowers to
`Step::Cusp` (`crates/editor-core/src/program.rs`) — but two places
downstream of the verb lose it:

1. **The verb reader drops it.** `read_extrude` and `read_revolve`
   (`crates/editor-core/src/verbs/sweep.rs`) destructure the record with
   `..`, keeping `body`, `side_faces`/`walls` and the name table;
   `SweptOut` has no field for the declarations, so the node value's
   `contacts` channel (`ContactRecords`, which `product::sources_of`
   reads) is empty for a swept node. The loft node does the same
   (`crates/editor-core/src/eval/wire.rs`, the `sweep::loft_body` call
   that names the result and keeps `built.body`).
2. **The product gate reads no declarations.** `product_recorded`
   (`crates/editor-core/src/product.rs`) gates each source
   (`T::gate_at_rest(body, tol)`, pass 2) and the aggregate
   (`T::gate_at_rest(&aggregate, tol)`, after pass 3) through
   `AtRestPolicy::gate_at_rest`, which is `topo::validate_geometric` —
   the empty declaration slice. The contacts it carries across the graft
   (`carry_contacts`) are read only later, by the assembly's
   `gate_at_rest_declared` (`crates/editor-core/src/assembly.rs`,
   `assemble_gathered`).

So a document whose one root is an extrude of a `.cusp()` profile
evaluates, and then refuses at `product` as `RootInvalid` carrying
`UndeclaredCusp` (measured, above) — the refusal the kernel verb no
longer asks its own callers to cure by hand.

## What a fix has to decide

- **The record's shape on the node value.** `ContactRecords` holds
  census records, not `DeclaredContact` face pairs; the at-rest door
  that reads it (`topo::validate_pseudomanifold` via
  `gate_at_rest_declared`) maps `ContactRecords::curves` to `Tangent`
  face-pair declarations. A swept cusp would enter as a `CurveContact`
  whose `witness` is the strut or rim edge (`Extruded::strut_edges`,
  `Revolved::rims`, `Lofted::seam_edges` at the joint) — and whether the
  census certifies a curve contact between two faces of ONE body is
  unmeasured.
- **Which gate reads it.** The product's two `gate_at_rest` calls would
  move to the declared door for a source that carries records, which
  also runs the census; that is this program's policy
  (`product-gate-says-verbatim-then-states-the-difference` is the row
  about where that trigger lives).

The red-first row is `a_cusp_extrude_document_refuses_at_the_product_gate`
(above): flip its `expect_err` to the gathered product.

## Decided (Ev, PR 3317, 2026-09-28)

The row's framing (carry the sweep's declaration to the product gate)
is retired. A wedge-0/2π edge is legal at rest iff its tangency is
jet-determinate, derived from the body exactly as the π seam is; intent
is declared only where the tangency is created (the profile's `.cusp()`,
a boolean's C7 `Tangent` operand pair). What the unit owes:

- check 4's material arm: wedge ends legal iff jet-determinate;
  `UndeclaredCusp` retires; `LaminaWedge` and the in-band escalation stay;
- retire `validate_geometric_declared*`, `contact_marks_declared*`,
  `declares_tangent_contact`, `MaterialWedge::is_declared_arm`, the three
  sweep results' `declared_contacts` and `swept::cusp_contacts`, the
  importer's cusp use of `declared_contacts` (its `Rest` records stay),
  tier 3′ reading curve records as `Tangent` declarations, and the
  editor's `UndeclaredCusp` mapping; the profile's `cusp_joints` stays;
- the red-first row flips: a `.cusp()` extrude, revolve (and loft,
  unjudged on NURBS walls) gathers, and so does one through a boolean
  that leaves the strut, a pattern, a split and part selection;
- prose that restates the declared arm moves with it (PATHS §4's "the
  declaration is what the kernel's material-wedge invariant asks for at
  rest"; check 4's NURBS-exemption comment on "DECLARED corner
  structure"; the sweep verbs' `body` docs).

The audit that replaces tier 3's backstop is its own row:
`every-op-that-can-mint-a-wedge-end-refuses-an-undeclared-one`.
