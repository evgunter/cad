---
id: vertex-section-quotient-has-no-room-precondition-and-four-offset-spellings
kind: issue
title: The vertex section reading divides by a sine its bound can exceed, charges the wall normal by assertion, and spells offset and side three ways
status: open
opened: 2026-10-14
priority: P1
cost: M
---

Found by the third review of PR 4858 (`cleave/vertex-against-section`,
head efa3830bc7, verdict MERGE). None of these blocked the merge; this
row schedules them.

## What

1. **No `room > 0` precondition on `Rounded::quotient`** (`geom_core::running`;
   likely mechanism, reachability unsure). Its old caller in
   `section_cert/ruling.rs` guaranteed room first; `vertex_section.rs`
   does not. If the sine reads below its own bound, the quotient's bound
   goes negative, `inward` then moves the reading away from zero, and
   the result is a definite `vertex_section_clear` off where nothing is
   resolved. Cylinder poses at ψ ≤ 1e-15 never reach it, since the plane
   is tangent and enters no face. A torus saddle at the Boolean door is
   the likely route. Fail loud: give the precondition a type or a
   `unreachable!` with values (D9), and pin it.
2. **The wall-normal charge in `section_sine` is asserted, not derived**
   (likely). It charges 4u per component, but `implicit_gradient` goes
   through `axial_radial(p − chart origin)`, whose error scales with
   |p − origin|/r. A tall cylinder or a far chart origin under-charges
   it. Derive it through the same running bound.
3. **One evaluation home is claimed, but there are five sites.** The
   `far` closure in `vertex_face_side` (`boolean/reduce.rs` ~798)
   re-evaluates offsets with plain `plane_offset` under the door's own
   name. At site 1 it reads the outward `PlaneDesc` while the door reads
   the chart normal. The split reuses its cached `offsets`.
4. **Sign → PlaneSide is spelled three times**: `splitting/classify.rs`
   ~269, `neighborhood.rs` ~280, `reduce.rs` ~811. `entered_faces`
   spells its strict-opposite test twice, inline and in `opposes`.
5. **Pins.** `an_unresolved_section_reading_keeps_the_offsets_verdict`
   catches the "unadmitted → refuse" mutant only at the default band.
   `offers_the_section_lever` matches only the split's `SliverVertex`.
   No pin covers an unresolved reading at the Boolean door.
6. **One refusal, two subjects.** The Boolean's in-band refusal states
   `Coincide::VertexOnFace`'s subject ("lies on a face of the other")
   while its margin is δ/sin ψ, under `VERTEX_SECTION_SUBJECT`.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P1: its items 3 and 4 are five evaluation sites and three Sign→PlaneSide spellings: two implementations of one underlying logic, P1 by `work/README.md` Priority (items 1, 2, 5 and 6 are P2 and travel with them).
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
