---
id: section-cert-copysign-mint-site-is-unregistered
kind: issue
title: section_cert's null-saddle witness mints an unregistered copysign site, and the copysign census row is red on main
status: open
opened: 2026-09-28
priority: P1
cost: E
---


Found by CONTACT-7's landing (PR 3383), whose hosted run is red on
`geom-core::all sym_rule_f_rows::the_copysign_mint_sites_the_tree_holds_are_these`.
The red is main's and not CONTACT-7's.

**The site.** `crates/topo/src/boolean/section_cert.rs`, in the section
certificate's cylinder-pair rule, at the null saddle loop's witness:
`let q = o2 - m * (r2 * T::one().copysign(delta));`. It arrived with
`5ff0efd84` ("germ: the section certificate"). The census row
(`crates/geom-core/tests/sym_rule_f_rows.rs`) counts one `.copysign(`
there, and its expected table and `crates/geom-core/src/sym/manifest.rs`'s
header list do not name `section_cert.rs`. So the row is red on
`origin/main` itself, which CONTACT-7 merged. CONTACT-7's own
`copysign` in `census.rs` stood below that file's first `#[cfg(test)]`
line, which the census cuts at, so it was never counted. The landing
drops it anyway.

**What is owed.** The row's doc prescribes the remedy:
1. re-ask the census (`m10_10_evidence_interval`'s
   `sym12_the_copysign_census_at_the_nominal`);
2. move the table and the manifest's list together.

`work/germ/log.md`'s 2026-09-26 class note asks that a new site be read
first. Here `delta`'s sign picks WHICH side of the axis the witness
ruling lies on, which is the case the note says needs a frame decision
rather than a `copysign`, where `delta` can straddle zero at
`Interval`. That judgement is germ's.
