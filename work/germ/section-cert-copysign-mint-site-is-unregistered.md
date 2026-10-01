---
id: section-cert-copysign-mint-site-is-unregistered
kind: issue
title: section_cert's null-saddle witness mints an unregistered copysign site, and the copysign census row is red on main
status: closed
branch: germ/section-cert-frame-side
pr: 3421
opened: 2026-09-28
priority: P1
cost: E
closed: 2026-09-29
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

## Resolution

The `copysign` is gone. The saddle arm of `section_cert.rs`
`cylinder_cylinder` decides `δ₀` on the band under its own predicate,
`section_cylinder_pair_side`, and picks the ruling `o₂ − r₂m` when it is
`Positive` and `o₂ + r₂m` when it is `Negative`. A `Zero` or undecided
`δ₀` cannot occur there, since `nest` has decided `|δ₀| − |r₁ − r₂|`
positive. It refuses R-tan anyway, through `signs`, as the arm's other
margins do.

- **Frame decision or `copysign`:** a frame decision. The sign chooses
  which ruling carries the witness, and that is the case the 2026-09-26
  class note sends to a decision.
- **The atom under the symbolic tier:** the witness no longer carries a
  `copysign(1, δ)` atom. The site has left the census table in
  `sym_rule_f_rows` `the_copysign_mint_sites_the_tree_holds_are_these`
  and `sym/manifest.rs`'s header list, along with the empirical
  paragraph that described it as unmeasured. The census was not re-run.
  Removing an atom that no measured replay reached (per PR 3407's probe)
  cannot move its counts.
- **Which rows exercise the branch:** a temporary probe, since reverted,
  showed that four of `sweep`'s `germ_interior_saddle` rows reach it
  (`a_cylinder_saddle_behind_a_pin_refuses_every_op`,
  `a_tilted_cylinder_saddle_behind_a_pin_refuses_every_op`,
  `the_saddle_is_a_certified_interior_loop_naming_both_walls`,
  `the_saddle_without_a_pin_refuses_at_the_fallback`), all at
  `δ₀ = +1.3`. `section_cert_rows`
  `the_saddle_witness_lies_on_cylinder_1s_side_for_either_sign` takes
  `δ₀` of both signs on both sides of the axis, with either wall the
  thinner. It asserts that the witness is on both carriers and on the
  near ruling. It goes red with the side flipped.

