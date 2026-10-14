---
id: copysign-census-red-since-5152-intersect-mint-site
kind: issue
title: main is red on sym_rule_f_rows' copysign census since PR 5152 added an intersect.rs mint site
status: open
opened: 2026-10-14
---


(Found by the NURBS program's build on PR 5123, whose CI went red on a row its diff does not touch.)

## What

`geom-core` `sym_rule_f_rows::the_copysign_mint_sites_the_tree_holds_are_these` is red on main since PR 5152. It is still red at `64c0b376`, where the red was reproduced locally.

Cause: PR 5152 (germ, commit `54f627a7`, "plane x cone's tangent generator is the extremum, not a residue root") added a `.copysign(` mint site in `crates/geom-brep/src/intersect.rs`. It did not add the site to the row's table or to `sym/manifest.rs`'s header list. The row's own text asks for the census (`sym12_the_copysign_census_at_the_nominal`) to be re-run before the table moves, so the update is the owner's.

Every PR's CI is red on this row until it moves. Of #5123's run, 13,532 of 13,533 rows passed; this was the one red.

## What is open

Re-run the census, move the table and the manifest header list, and merge. Germ's adjudication note on `copysign` sites (`log.md`, "A class, recorded at adjudication") also applies to the new site.
