---
id: from-f64-is-split-by-where-the-number-comes-from
kind: unit
title: from_f64 is split by where the number comes from: a compile-time-constant door, counts, bands and authored values
status: dispatched
opened: 2026-11-20
priority: P1
cost: H
parent: from-computed-claims-a-zero-tangent-at-a-dual
---

The rule is `crates/geom-core/README.md`, "Reading a bracket where nothing certifies" (DL5.2, DL5.3), ratified by Ev on PR 7155. The site counts below are two designers' readings (`work/scalar/log.md`, 2026-11-20), mostly not traced to consumers: re-measure before building on one.

`scripts/gates/from-f64-allowlist.sh` lists every non-literal `from_f64` site with a class a reviewer assigns (202 rows, 257 sites: 230 `constant`, 20 `fixture`, 7 `launders`). It cannot see whether a constant is one, a helper that passes an `f64` through, a `let f = T::from_f64;` alias, or a crate outside its nine.

The final state has no door that takes a bare runtime `f64`: a literal goes through a door that accepts only a compile-time constant, a count enters as an integer, a band as the tolerance type, an authored value through the document's parameter door. Not yet specified: nobody has compiled the constant door or checked the 191 `constant` rows against the four sources. Spec it after `a-bound-to-decide-against-never-becomes-a-t` lands, which removes the lever scales from the list.

## Dispatch, 2026-11-24

Its predecessor (`a-bound-to-decide-against-never-becomes-a-t`) and the solved-value door (PR 7651, whose gate is the model for this one's) have merged. Letter **H**; tier **DUAL**: four doors every constant of the model enters by, replacing a bare door hundreds of sites use, with a claim at `Sym` per door that later code is written against. Measure-first: every listed site is classed by its true source before it moves, and a site that is none of the four stays listed against its row.
