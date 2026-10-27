---
id: the-escalation-tolerating-rows-can-assert-their-answers-now
kind: issue
title: Four rows tolerate a convergence escalation that no longer happens below the last round, and can assert their answers
status: open
opened: 2026-10-12
priority: P3
cost: E
---



## What

Since PR 4839, a convergence margin in the band refines at every round
that has a successor (`geom_brep::props::quad::Schedule::converged`),
and escalates only at the schedule's last round. Four rows were written
when an in-band margin escalated at any round. They tolerate that
escalation rather than require it, so they stayed green, but each
asserts less than it now could:

- `sweep/tests/band_annulus_host_boundary.rs`: the `y = 3` cut that
  removes the whole cap (`Cut::HostFromAbove`'s docs) is steered
  around, at `y = 2`.
- `sweep/tests/a_split_through_a_wrap_edges_vertex.rs`
  (`a_boolean_through_a_strut_vertex_keeps_one_face_where_the_piece_goes_round`):
  lets a `VolumeUnmeasured` stand on some poses.
- `sweep/tests/reach_split_gate_azimuth.rs`
  (`every_cut_clear_of_a_partial_turn_sphere_face_splits`): lets a
  half's convergence escalation stand under a floor.
- `sweep/tests/review_cleave_wrongarc.rs`: prints `props_quad_converged`
  refusals rather than failing them.

## The shape of a fix

Per row, measure which poses still refuse at all three ε. Assert the
answer on the rest, and pin what remains by type, with its own cause,
at the ε where it was measured.

## The volume backstop left the operation path (2026-10-27)

PR 5842, under `[ev]` PR 5780 (the boolean's volume relations are a test oracle, `topo::test_support::volume_relations`), deleted the backstop: `BooleanError::{ResultVolumeImplausible, VolumeUnmeasured, VolumeCorrupt, VolumeUndecided}`, `BooleanDecision::VolumeBackstop`, `AtRestPolicy::gate_volume_backstop` and the `volume_backstop*` predicates are gone. `a_split_through_a_wrap_edges_vertex.rs`'s `VolumeUnmeasured` arm is gone: the variant no longer exists, and those poses build and are checked. Three rows remain.
