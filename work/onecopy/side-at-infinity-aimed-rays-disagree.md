---
id: side-at-infinity-aimed-rays-disagree
kind: issue
title: side_at_infinity's aimed rays read opposite sides of a selection that is not one embedded shell (invalid bodies): production takes the first ray's, so which finding an invalid body gets depends on face order
status: open
opened: 2026-11-06
priority: P2
cost: M
refs: [6592, 6595]
---


Found by `a-ladder-that-checks-its-answers-agree-has-no-home`
(`onecopy/refusal-standing`). Its first form, commit `a8e175f9` (on
the lane's local branch, not pushed), wired a debug check of the
`Answering::Any` premise into the `Any` ladders: read every rung after
the answer, and assert each agrees. That wiring was held for the `Any`
design fork, `[ev]` PR 6592, so the check is not on main.

## What

`side_reader::cast_aims` (`crates/topo/src/side_reader.rs:606`) climbs
`Answering::Any` over a selection's faces, one aimed ray per face. Its
premise: every ray from outside reads the one side, so the first ray's
side is the answer. That is a theorem on one embedded closed shell
(Jordan–Brouwer). **On the selections below it is false**, and
**production takes the first answer**, so the finding an invalid body
gets depends on the order of its faces.

**Narrowed (2026-11-09).** The first filing listed 24 tests. PR 6595
fixed the larger family: a dropped shared root in
`solid_contain::ray_crossings`, which misread a twin root and made valid
bodies' rays disagree (the other 17 of the 24). What remains is the selections that
are not one embedded shell, all invalid bodies: inside-out or misnested
shells in one solid, a self-crossing shell, and a raw whole body. Their
remedy is `[ev]` PR 6592's: a shell's role is read one shell at a time
behind the embedding check, and a multi-shell selection carries its
side. Priority falls to P2, since no valid body reaches it.

## The disagreement shape (re-run on main after 6595)

With `cast_aims` swapped to `ladder::climb_agreeing` and a panic on
`Err(Disagree)`, topo `ci` gives **14 disagreements: `Ok(Out)` then
`Ok(In)` 10 times, `Ok(In)` then `Ok(Out)` 4 times.** Before 6595, one
probe showed two faces aimed at one shared point reading `Out` and `In`.
That was the twin-root family. An inside-out shell's face reading `In`
past outer faces reading `Out` is this one.

## The failing tests (7)

- bool4r2_probes::the_per_solid_door_reads_the_container_s_own_sign_at_infinity
- contact5_gate_and_beam::a_solid_whose_cavity_crosses_its_wall_blocks_its_pair
- shell_winding::a_void_outside_every_outer_refuses
- split_operand_gate::an_inside_out_shell_under_one_solid_does_not_finish_at_f64
- the_fin_crown_winds_as_a_solid::the_fin_crown_is_a_solid_where_the_old_crown_crossed_itself
- tier3_tests::check_10_reads_every_shell_s_role_or_refuses_the_solid
- tier3_tests::the_census_gate_finds_a_far_slab_nested_in_another_solid

To reproduce: swap the site's `climb(Answering::Any, &geos, ..)` for
`geom_brep::ladder::climb_agreeing(&geos, .., same)`, where `same` holds
for two `Ok` answers that are equal, have its `Err(Disagree { .. })`
panic, and run `cargo nextest run -p topo --profile ci`.

## Done when

`[ev]` PR 6592's answer lands. Then the reader reads one shell at a
time, a multi-shell selection carries its side, and the site's premise
is that of an `Alike` ladder whose source type establishes it. The
every-rung audit (probe-gated) then passes on every fixture.
