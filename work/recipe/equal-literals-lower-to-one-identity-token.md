---
id: equal-literals-lower-to-one-identity-token
kind: issue
title: Two separately typed equal literals lower to one identity token (ParamSource encode, the sym tier's Lit atom), reading equal values as intent
status: open
opened: 2026-10-03
priority: P0
cost: E
refs: [one-way-to-say-dependency-and-intent]
---


`crates/editor-core/src/param_source.rs` `encode` lowers an `Expr`
literal by its f64 bits, so two separately typed `5 mm` radii lower to
equal tokens and `topo::field_source_evidence`
(`crates/topo/src/param_source.rs`) answers `RadiusEvidence::Declared`
for them. The symbolic tier's `Lit(f64 bits)` atom
(`crates/geom-core/src/sym.rs`) does the same: `5mm − 5mm` from two
independent slots normalises to zero. Both read equal values as
intent, against the banked "coincidence is structural …, never
inferred from values".

Under the ruling `one-way-to-say-dependency-and-intent` (D10) no
dimensioned literal stands in a slot, so both arms disappear with the
build. Found by the ruling's designer pair.
