---
id: facade-has-no-public-nurbs-faced-fixture-door
kind: issue
title: The façade has no public door to a NURBS-faced body, so an end-to-end instrument has to reach past it into sweep's test support
status: open
opened: 2026-09-21
priority: P3
cost: D
---

## What

Writing the smallest honest end-to-end program against `pncad` —
build a body with at least one NURBS wall, `validate_geometric`,
`mass_properties`, `tessellate`, read the budget meter — cannot be
done through the façade's public surface. It needs
`sweep::test_support::loft_prism`, reached as
`pncad::sweep::test_support::…` with `--features sweep/test-support`,
plus `--features budget` for the meter.

The instrument that found this is
`crates/pncad/examples/ring2_r2_e2e.rs` (RING-2's review), which
therefore carries
`required-features = ["budget", "sweep/test-support"]` in
`crates/pncad/Cargo.toml` — and is consequently built by no CI row at
all, since every row runs at default features.

## Why it is a finding and not a wart

`memories/demo-purpose.md`: an example that reaches past the API stops
being evidence about the library. Here the reach is unavoidable, which
is the finding — **the authoring façade has no way to produce the one
body shape the kernel's certification work is about**. Every
closed-form primitive the façade offers is planar or analytic; the
NURBS-walled bodies live behind a test-support module.

Two consequences worth separating:

1. **For a user**: there is no worked path from the façade to a
   curved-wall solid, so the first thing anyone doing curved work has
   to learn is which kernel crate to depend on directly — which is the
   one thing LIB-U1 exists to prevent.
2. **For the tree**: an example with `required-features` is compiled
   by nothing. It is the same shape as a row demoted to the nightly
   (`docs/prompts/implementer-discipline.md` §2) — it reports green by
   not running.

## Candidate repairs

* A public fixture door on the façade (`pncad::fixtures::loft_prism`
  or similar), behind an opt-in feature the CI clippy row turns on, so
  examples and demos can reach a NURBS-faced body the way a user
  would.
* Or a tour scene that builds one through the public authoring verbs,
  if the verbs can already reach it — in which case the finding is
  that nothing shows how.

Either repair also lets the RING-2 example drop its
`required-features` and be compiled by the ordinary gate.

## Disposition

LIB's: `crates/pncad/` is this program's ground. Filed by RING-2
(SCALAR) from its R2 review lane's ergonomics note.
