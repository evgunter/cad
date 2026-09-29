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

**Registered; the judgement is still owed.** ENCL's
`encl/fix-main-census-tables` registers the site in the census table and
in `manifest.rs`'s header list, so the row is green again. It re-asked
`sym12_the_copysign_census_at_the_nominal` on the seven measured
documents with rule F on (`shipped`) and shut (`no_f`). The counts are
unchanged. `tangent.rs`'s jet atoms are the only `copysign` atoms that
stand: 7 of 14 `tangent_normal_parallel` residuals on the bracket and 14
of 28 on the link, and none on the other five. A temporary probe at
`classify`'s cylinder-pair arm and at the saddle branch printed nothing
on any of the seven replays. So no measured document reaches this site,
and its reach is unmeasured, which is how the manifest now states it.

What the landing can argue: the branch runs only after
`|δ| − |r₁ − r₂|` is decided positive (`signs` refuses `Zero` and
undecided). So `δ` is certified away from zero there, and it cannot
straddle at `Interval`, which is the failure the 2026-09-26 class note
guards against. What it cannot settle is germ's:
- whether the side of cylinder 1's axis should be a frame decision
  rather than a `copysign`;
- whether the witness, which is placed against both faces, carries an
  opaque `copysign(1, δ)` atom into those decisions under the symbolic
  tier;
- which document or row would exercise the branch.
