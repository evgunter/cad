---
id: band-k-is-spelled-escalate-over-zero-at-every-site
kind: issue
title: A band's K has no accessor and is spelled escalate()/zero() by hand at each site that reads it
status: open
opened: 2026-10-20
priority: P4
cost: E
---

Found by review 2 of `restread/ef-pierce-charge` (PR 5421).

## What

`geom_core::Band` (`crates/geom-core/src/predicate.rs`) holds `zero`
(ε) and `escalate` (Kε) and offers no `k()`. Every site that needs the
ratio spells it by hand, `band.escalate() / band.zero()`:

- production: `crates/topo/src/validate.rs` `resolved`
  (`T::from_f64(band.escalate() / band.zero())`, the lever of a
  rounding read `K` times over);
- tests: `crates/topo/src/census.rs`
  `a_transverse_declared_pair_holds_no_crossing_its_edges_hug` and
  `point_touch`'s `a_circle_nearly_parallel_to_a_plane_is_read_at_its_touch_point`;
  `crates/topo/src/boolean/join.rs`
  `a_radius_guard_escalates_as_the_radius_decision`;
  `crates/geom-core/src/predicate.rs`
  `no_offer_quotes_a_tolerance_at_or_below_the_floor` (×2);
  `crates/geom-brep/src/dihedral.rs` `just_below_the_offer`;
  `crates/geom-brep/src/recourse.rs`
  `no_tighten_offer_is_unvalued_and_a_negative_verdict_offers_none`;
  `crates/geom-brep/src/props/closure.rs`
  `a_loop_off_a_whole_turn_refuses_and_in_the_band_escalates`;
  and more under `crates/*/tests/` (`sweep`'s `recourse_roster`,
  `point_touch_records`, `verbs_sphsph_opening`,
  `blend_recourse_followability`; `step-import`'s `tier_gate`).

The census's span clearance (`crates/topo/src/census.rs` `span_clear`)
reads a derived band `Band::new(escalate − zero, escalate)`, `[(K − 1)ε,
Kε]`, another hand spelling of the same structure.

## Direction

Give `Band` a `k()` (and, if a second derived band appears, a door for
"the band's escalation less its zero") and fold the sites above onto
it. The pattern `escalate() / zero()` matches only the quotient; a
second pass for `escalate() - zero()` and `zero() / (escalate()` found
the census site above and no other.
