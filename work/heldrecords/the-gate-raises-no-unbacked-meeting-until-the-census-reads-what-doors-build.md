---
id: the-gate-raises-no-unbacked-meeting-until-the-census-reads-what-doors-build
kind: issue
title: Flip the gate: finish raises a meeting no record backs (plan step 5); measured early, 55 fast-set rows refuse and they are valid shapes
status: open
opened: 2026-11-18
priority: P1
cost: M
---


`plan.md`'s step 5, as a row, with the measurement `embed/records`
took when it built the type
(`a-finished-body-holds-its-contact-records-at-every-door`, 2026-11-18).

## Where the code stands

A finished body holds its records and `finish` is its one constructor.
The gate certifies every record it is handed: a stale or contradicted
one refuses at that door (`deciding_door_census`,
`crates/topo/src/validate.rs`). It raises no meeting that no record
backs, and with no record it runs no census. `AtRestBody::bar` names
which half was paid (`Bar::RecordsConfirmed`; a `⊔`'s is
`Bar::ArrivalsDecided`).

## Measured: the gate flipped early

One box, CI's build, the `ci` profile at default ε, 17,185 tests.

- Every finish running the whole of tier 3′: 170 rows refuse. 35 are
  valid disjoint curved unions at the Boolean's gate
  (`CensusUndecidable { MayOverlap }`), 41 are fixtures that finish a
  hand-built multi-solid body bare.
- Every finish confirming its records and refusing an unbacked meeting
  inside one solid (between two result solids unasked): 55 rows
  refuse, +12% wall, +14% test time, the census 6,835 → 59,672 calls.
  By cause:
  - plain extrudes and a loft the census has no lane to read
    (`contact_edge_must_carry`'s fixture profile and five probes,
    `CensusUnsupported … MeetUnread(Unsettled(Meter
    "curve_pair_pose_basin"))`; `a_lofts_chart_ends_read_as_placed`;
    `review_blend_k_rk_probes` ×2, `CensusEscalated` in band;
    `bool5r1`'s lune);
  - a fillet its own recourse says builds
    (`blend_recourse_followability::the_turn_recourse_builds_at_each_end_it_names`,
    `contact_edge_must_carry::the_contact_recourse_is_followable_at_each_site_kind`);
  - the split's own unrecorded touches (`split_pinch_keeps` ×5,
    `a_keel_wrapped_at_both_ends_splits_as_a_ring`,
    `a_vertex_on_a_grazing_plane` in band);
  - Boolean rows: `three_solids_on_one_line::the_plate_joins_an_edge_lying_in_a_face_as_one_body`,
    `face_face_touch` ×2 (the bridged rods' unrecorded tangent line),
    `sliver_shell_role` (its typed `Escalated` becomes
    `ResultInvalid[CensusEscalated]`), `near_tangent_pierce_poses`
    (2 of 6 unions), `join_pierce_runs_sweep`;
  - four hand-built fixtures that touch themselves, finished bare;
  - the census's decisions entering every node's verdict log: six
    editor-core rows diagnose `PredicateFlip pm_census_ef_residual`
    where they diagnosed `GroupResized` or `Upstream`
    (`resolve_piece_ladder` ×2, `resolve_upstream_scope` ×2,
    `edit_ruled_carve`, `edit_refusal_recourse`), and the census's
    predicates have no words in the recourse table.
- The slow set and the 1e-6 and 1e-12 rows were not measured.

## Owed before the flip

The census's lanes (`work/atrest/`'s curved edge–edge count,
`work/reachhold/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`,
this program's rows on the split's own touches), words for the census's
predicates where a node's verdict log reads them, and a re-measurement
at three ε with the slow set.

The flip itself is the filter in `deciding_door_census` and the early
return on no records.
