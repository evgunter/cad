---
id: main-red-the-copysign-census-since-the-apex-tangent-generator
kind: issue
title: Main red: the copysign census row since PR 5152's apex tangent generator added an unregistered site in intersect.rs
status: closed
pr: 5180
opened: 2026-10-14
closed: 2026-10-15
priority: P1
cost: E
---


Found by CLEAVE on PR 5100's hosted run (head `3b82cc090c`). It is red
only on `geom-core sym_rule_f_rows::the_copysign_mint_sites_the_tree_holds_are_these`
(`crates/geom-core/tests/sym_rule_f_rows.rs:667`). Every other test, 14,074
of them, passes at every ε. PR 5100 touches neither geom-brep nor geom-core.
`origin/main` `64c0b37662` carries the same red.

**The site.** `54f627a7bc` ("germ: plane x cone's tangent generator is the
extremum, not a residue root", PR 5152) adds, in
`crates/geom-brep/src/intersect.rs`:

    PlaneConeSection::ApexTangentLine(gen_at(phi + half_turn.copysign(c) + half_turn))

Neither the census row's expected table nor
`crates/geom-core/src/sym/manifest.rs`'s header list names this new
`.copysign(` site.

**What is owed.** The row's doc prescribes the remedy, as on the
closed `section-cert-copysign-mint-site-is-unregistered`:
- re-ask the census;
- move the table and the manifest's list together.

`work/germ/log.md`'s 2026-09-26 class note asks that a new site be read
first. Here `c`'s sign picks which generator the tangent line is, and
`c` can straddle zero at `Interval`. Whether that wants a frame decision
rather than a `copysign` is GERM's judgement. Until it lands, every PR's
`gate ok` is red on this one row.

**Also blocks NURBS** (2026-10-14): #5123 and #5163, each with every other row green.

**Proposed patch** (NURBS build A), to apply only after the census is re-asked:
- add `("crates/geom-brep/src/intersect.rs", 1)` to the expected table at `sym_rule_f_rows.rs:705-715`;
- add a line naming "`geom-brep/src/intersect.rs`'s plane×cone apex tangent generator (the side `c` leans)" to `sym/manifest.rs`'s site list (around l.71-84).

**Closed** by PR 5180 (`22b6266807`, by another track): the site is registered in the census table and the manifest, and the census re-asked finds no copysign atom at a decision. `sym_rule_f_rows` is 9/9 green on main `bc39253aa1`.
