---
id: the-lilys-stem-and-arch-walls-have-no-certificate-arm
kind: issue
title: The lily's stem glue builds through the join and refuses at the interior-loop guard: two tori with parallel axes apart have no section-certificate arm
status: open
opened: 2026-10-12
priority: P1
cost: M
---


Found by GERM's U-T3 (`germ/plane-torus-chord-circles`), measuring the
doors behind the plane × torus chord, 2026-10-12.

## Measured

The lily's stem glue (`crates/sweep/tests/mate7a_r1_probes.rs` `stem`
and `arch`), undeclared, `stem ∪ arch` and `arch ∪ stem`: the join now
completes (its chord is an arc of the arch's meridian circle in the
stem's weld cap), and the op refuses
`CurvedPairUnsupported { site: InteriorLoopGuard, kind: Torus, other_kind: Torus }`
on the stem's tube wall against the arch's. Pinned by
`germ_plane_torus_frame::the_lilys_stem_glue_passes_the_chord_to_the_interior_loop_guard`,
`germ_circle_torus::the_lily_stem_glue_is_past_the_circle_torus_pairs`
and `mate7a_r1_probes::p1_wall1_passes_the_gate_and_the_crossing_layer_and_stops_at_the_join`.

The two walls are tori whose axes are parallel (both `−y`) and apart
(rings of radius 5 m and 1.1 m about different centres), tubes
`0.060` and `0.052`. They share only the weld plane and never meet, but
their boxes overlap at the fork, so `boolean/ops.rs`
`interior_loop_verdict` asks the section certificate for the pair, and
`boolean/section_cert.rs` `torus_pair`'s torus arm reads
`Pose::Parallel { .. } | Pose::Other => Section::Intractable`.

`stem ∩ arch` and both differences refuse earlier, on the no-crossings
path (`section_extent_pass`, `FallbackExtentUnsupported` on the same
pair), which the join never reaches; unchanged by U-T3.

## What the taker owes

A certificate arm for two tori with parallel axes apart that decides a
pair like this one clear (each tube is far from the other's wherever
the faces' windows reach), or a narrower guard, so the lily's wall 1
builds. The tour's `lily` wall 1 pins the same refusal.
