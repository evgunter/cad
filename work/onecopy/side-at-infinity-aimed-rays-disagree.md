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
(`onecopy/refusal-standing`). Its first form, commit `a8e175f9` (pushed: it is in
`onecopy/refusal-standing`'s history, so the wiring can be checked out
to reproduce), wired a debug check of the
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
`Err(Disagree)`, topo `ci` gives **14 climbs that disagree** (each
counted once, at its first disagreeing pair, where the panic stops it):
`Ok(Out)` then `Ok(In)` 10 times, `Ok(In)` then `Ok(Out)` 4 times.
Counted by rung instead (every rung that disagrees with the first
answer, as PR 6606's delta review read it with `a8e175f9`'s wiring), the
7 tests give **32 disagreeing rungs: `Out` then `In` 19, `In` then `Out`
13.** Before 6595, one
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

## Other sites waiting on the same `Alike` (2026-11-09)

When 6592's `Alike` lands, these premise-only ladders flip to it along with `cast_aims`:

- **`topo::link::Cone::reads_given`'s reference climb** (`crates/topo/src/link.rs`, "every reference reads one material"). It climbs under `Answering::First` until then (PR 6637's review, F1): the premise is shown failing on `work/honeone/a-cones-parity-reference-is-off-its-sectors-plane-by-the-tie.md`, and `editor_core::drive`'s `classify_replay` acts on the escalation log a superseded splice would drop. The code comment there says "pending 6592's `Alike`".
- **`topo::chart_region::polygon_relation`'s vertex climb** (QML T4): "every vertex off the other polygon's boundary reads one relation" holds only where the boundaries do not properly cross (its own doc), so it climbs under `Answering::First` until then.

## The simplicity-and-speed answer (2026-11-16)

Ev asked on 6592 what the change does to simplicity and speed, given that the exact error an invalid surface gets matters little. The designer's measurements on main `0be17a13e` (f64 release, n-gon prisms):

- **Recommendation: parts 1 and 2 only.** Part 3, the `Alike`/`AlikePremise` token, is dropped. It would classify 44 `climb*(Answering::Any…)` sites in 22 files, and its only gain is a deterministic error on invalid input. The side reader's ladder becomes `First`, with a comment. The sites in the section above stay on `First`.
- **Simplicity:**
  - `side_at_infinity`'s production callers go from 3 to 1.
  - The missed-ray re-read is removed (`missed_side`, `MissedRead`, `sided`, `as_given`, `SolidFaces::read_as_given`, `operand_faces`), along with `inside_out_operand`.
  - The inside-out chain retires: `PointInSolidError::InsideOut`, `ValidationError::SliverInsideOut`, `BooleanError::InsideOutOperand`, and `OrientationUndecided` with its 3-arm `OrientationUnread`, which folds into `ShellRoleUndecided`. That is about 20 match arms.
  - The selection's existing `at_infinity: Option` becomes required.
- **Speed:**
  - One side read costs 0.09, 0.43, 1.8 and 10.5 ms at 6, 66, 258 and 1,026 faces.
  - The gate is unchanged for one shell, and an n-shell solid saves one read.
  - Point queries from outside run about 1.8× faster: 40 queries take 2.0→1.0, 30→16, 131→72 and 821→468 ms.
  - Booleans save under 1%.
- **Sequencing:** part 2 needs the following named. The census's instance containment (`census.rs`, `SolidFaces::of(body, outer)`) runs at the gate, before the solid's roles are known, so its selection cannot carry `Out` from a constructor. Either the census runs behind the per-shell role reads and takes the side from them, or its selection reads the side once and caches it (`work/onecopy/a-selections-aimed-side-read-is-repeated-on-every-missed-query.md`).
