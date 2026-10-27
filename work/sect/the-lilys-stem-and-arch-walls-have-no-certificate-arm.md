---
id: the-lilys-stem-and-arch-walls-have-no-certificate-arm
kind: issue
title: The lily's stem glue builds through the join and refuses at the interior-loop guard: two tori with parallel axes apart have no section-certificate arm
status: closed
opened: 2026-10-12
closed: 2026-10-27
priority: P0
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

## Re-banded to P0 (on request in chat, 2026-10-25)

Lily wall 1. Two tube arcs glued end to end on a shared disc is an ordinary join on ordinary geometry, so this is a normal verb broken on normal geometry.

## Moved to SECT (2026-10-25)

From ORBIT. Moved by `git mv` in the chat-approved refold (2026-10-25) that put the torus pairs with the section-first join they are built on. Lily wall 1's torus × torus pair is a section-certificate arm for a torus pair, beside SECT's torus germ frames (teapot wall 2); the two are one torus-pair thread on the section object.

## Closed

**Correcting "Measured" above:** the two tube walls do meet. Their
spine circles are internally tangent at the fork, and either side of it
the arch's tube leaves the stem's through its inner wall: two loops,
mirror images across the weld plane, each outside one face's window
(the arch's side lies past the stem's 22°; the stem's side lies before
the arch's start). What never meets is the faces.

Built the general arm: `boolean/section_cert.rs` `torus_parallel_torus`,
two tori on parallel axes at any offset and height. It lists every
component of the carriers' section with a witness; the faces' windows
clear the lily's pair (each of its two loops, mirror images across the
weld plane, stands outside one face), not the carriers. Lily wall 1
builds: `stem ∪ arch` and `arch ∪ stem` are one shell of the two tubes'
volume, `stem ∩ arch` is empty and each difference gives back its first
operand (`mate7a_r1_probes::p1_wall1_glues_the_stem_into_one_body`).
Rows: `section_cert_parallel_tori_rows`. Filed: level tubes' node
(`work/secttail/level-tubes-of-parallel-tori-refuse-at-the-node.md`), the cone
sibling and ball separation for tilted poses (both `work/secttail/`), and the
tour's two-piece stem
(`work/issues/the-tour-lily-ships-its-stem-as-two-pieces.md`).
