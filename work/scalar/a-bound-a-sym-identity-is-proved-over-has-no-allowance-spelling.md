---
id: a-bound-a-sym-identity-is-proved-over-has-no-allowance-spelling
kind: issue
title: Two bounds keep a T because their Sym proof needs a shape no allowance door spells, a constant factor and a hull of two bounds
status: open
opened: 2026-11-20
---

Found building `a-bound-to-decide-against-never-becomes-a-t` (DL5.2, `crates/geom-core/README.md`). Restated after that unit's fix pass. DL5.2's letter answers both sites ("never becomes a `T`"). What is open is a door that reproduces their proof.

## What

Each margin door that takes an `f64` (`Margin::less`, `less_floored`, `levered_at`, `bound_levered`, `metered_bound`, `bound`) forms the margin at the decision with the operations of the bound's `from_computed` spelling. At `Sym` that makes the bound an unknown. A form's zero therefore survives a product with it and is lost by a sum with it, exactly as before. Two sites prove through something else, so neither door fits them:

- `editor-core/src/coincide/replay.rs` `swept_along` levers a component at `Lever::unsourced(T::from_f64(pow2))`, the power of two at or above a decided pair's reach (`from-f64-sites.tsv`, `constant`). The proof rests on that factor being a literal. Its own comment says an opaque factor keeps the product from discharging at the door's budget. `levered_at` would give the opaque factor, so the site waits on `from-f64-is-split-by-where-the-number-comes-from`, which owns constants of the model.
- `edge_nurbs::limb2_enclosure_as` (`crates/geom-brep/src/edge_nurbs.rs`) builds `T::from_computed(floor).enclosure_hull(T::from_computed(upper)).free_identity(identity)`. The margin is the hull of two bounds, with no reading in `T`, and at `Sym` its node is the residual identity over a free parameter (`scripts/gates/free-identity-allowlist.sh`). One reading plus one allowance does not express a hull of two bounds that carries an identity.

## The change

For `replay.rs`: a door for a positive constant factor of the model, once the `from_f64` split says what one is. For limb 2: a margin door for a bracket of two bounds with an identity (`Margin::bracket(floor, upper).free_identity(..)`), decided at the scalar as the hull is today.
