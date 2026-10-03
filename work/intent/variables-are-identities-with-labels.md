---
id: variables-are-identities-with-labels
kind: unit
title: D10 stage 1, the representation: what a variable is (identity, label, free or defined, scope), what constants remain, how an Expr reads a variable, how it persists
status: open
opened: 2026-10-03
priority: P0
cost: H
design: true
parent: d10-one-way-to-say-intent-is-unbuilt
---


## The question

D10 (`docs/DESIGN.md`) says every slot that admits more than one value
holds a variable whose type suits the slot; a variable is free (value,
written unit, optional distribution) or defined (an `Expr` over other
variables, or an output of an operation); no dimensioned literal stands
in a slot or inside a formula; typing a value in the GUI mints a free
variable and offers an existing one of equal value, and declining the
offer is what makes the two distinct. Structural coincidence is "the
same construction of the same variables", so a variable's IDENTITY is
load-bearing: it decides what glues silently and what the
`unproven-coincidence` lint reports.

Stage 1 builds the scalar half. Before any lane builds it, the
representation must be settled: what a variable is, how it is
identified and shown, and how everything that holds a number today
holds a variable instead.

## What exists

- `Expr` (`crates/editor-core/src/expr.rs`): private `ExprKind` with
  `Literal(Lit)` (f64 bits plus a display-unit code), `CountLiteral`,
  `Param(ParamName)`, arithmetic, trig, min/max. Dimensions are
  Length, Angle, Count, Scalar, checked by the smart constructors.
- The parameter table (`Doc.params: BTreeMap<ParamName, DocParam>`,
  `crates/editor-core/src/doc.rs`): `DocParam::Continuous { dim, value,
  display_unit, distribution }` or `Count { value }`. The name is both
  the identity and what a person reads. There is no derived parameter,
  no rename and no delete door; `SetDocParam` is create-or-replace.
- Slots: every node's continuous fields are `Expr`s, evaluated through
  `eval/slots.rs`; profile programs carry `Expr` step arguments
  (`crates/editor-core/src/program.rs`), resolved at f64 (PP1).
- Parameter identity reaches the kernel through `ParamSource`
  (`crates/editor-core/src/param_source.rs`; VERB-SEAT P1–P3), which
  lowers an `Expr` to a token by its structure — literals by bits
  (`equal-literals-lower-to-one-identity-token`, which this stage
  closes). `ParamScope::Root(DocumentId)` / `Part(DocRef)` scope it.
- Node identity moved to digests with a separate human label that is
  never identity (DESIGN.md Band 1, "Node labels"); that is the nearest
  precedent for "identity versus what a person reads".
- About 140 call sites build literals (`Expr::literal*`,
  `length_in`, `angle_in`, `written_*`) across editor-core, the
  viewer, `demos/tour`, `pncad` and `pncad-py`. The façade is
  f64-first by ratified design (`docs/LIBRARY-DESIGN.md` U1).
- The error-propagation lane seeds one dual per document parameter and
  reads distributions per parameter (`crates/editor-core/src/stackup.rs`,
  `analysis.rs`).
- Later stages make variables typed beyond scalars (`Point`,
  `Direction`, `Axis`, `Plane`, `Frame`, `Face`, `Edge`, `Body`) and
  make nodes operations that define variables; stage 1 builds none of
  that, but its identity scheme is the one they extend.

## What must be decided

What a variable's identity is and how a person names, renames and
reads it; whether a variable typed inline in a slot is the same kind of
thing as one declared in the table; where variables live (document
scope, a part's scope, per-instance arguments); which constants remain
and how they are spelled (D10: dimensionless rationals and rational
fractions of a turn); how an `Expr` reads a variable; how the façade's
f64-first doors and the bindings meet "no literal"; what `ParamSource`
becomes; what persists and how a file this build cannot read refuses;
and anything else the representation forces.
