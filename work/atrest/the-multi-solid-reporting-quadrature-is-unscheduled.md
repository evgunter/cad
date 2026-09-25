---
id: the-multi-solid-reporting-quadrature-is-unscheduled
kind: issue
title: the multi-solid body certificate pays a second arena-wide quadrature nobody scheduled
status: closed
opened: 2026-09-22
priority: P1
cost: D
rides_with: tier3-prime-still-couples-plus-v-to-the-reporting-target
refs: [2977]
parent: ATREST-3
closed: 2026-09-25
pr: 3191
---


ATREST-1 moved check 7's subject from the body to the SOLID. On the
reporting path — `tier3_local_checks_marked` in
`crates/topo/src/validate.rs` — the battery's per-solid loop can no
longer hand its own derivation back as the body certificate once the
body holds several solids, so the `subjects.len() != 1` arm takes a
FURTHER read over `crate::query::all_faces(body)` for the returned
value. That is a narrowing of the pass's old claim ("one certified
quadrature per gate") and an added cost, and neither was scheduled:
it was disclosed at one comment inside the function and nowhere else.

**Who pays it.** Every door that reaches the battery in one pass and
wants a value: `validate_pseudomanifold_certificate` and
`contact_marks*`. `crates/step-import/src/lib.rs`'s `gate3` is the
production consumer — the aggregate STEP subject is multi-solid by
definition, so a STEP import of an assembly pays N per-solid reporting
reads plus one arena-wide one where it used to pay one.

`validate_geometric_certificate` does NOT pay it: that path certifies
at SIGN level and assembles the parts
(`crate::SignCertificate::assembled`), so its cost is still one read of
each face.

**What is measured now** (PR #2977's fix pass):

- `crates/topo/src/tier3_tests.rs`'s
  `a_multi_solid_certificate_is_the_whole_body_measurement` reds if the
  arena-wide reporting read ever stops being bitwise
  `topo::mass_properties` — the only promise that read carries.
- `crates/topo/src/props.rs`'s `face_list_door_tests` row
  `the_assembled_per_solid_certificate_is_bitwise_the_whole_body_read`
  pins the sign-level composition over the two- and three-solid grafts.

**What is NOT measured**: the COST. Nothing reds if the multi-solid
reporting path grows another whole-arena walk, and the census goldens
record arena counts rather than quadrature verdicts or counts. The work
here is to decide whether the extra read should exist at all — the
per-solid `MassProperties` could be folded instead, at the price of a
different summation order and therefore different bits from
`mass_properties` — and, if it stays, to give the quadrature count per
gate a pin that a second walk would break.

## Banded and priced, 2026-09-22 (ATREST orchestrator)

**P1, cost D, riding with
`tier3-prime-still-couples-plus-v-to-the-reporting-target`.**

**P1 rather than P4**, because this is not "the suite costs less" —
it is what a door PROMISES to return and at what derivation level,
which is architecture that entrenches as consumers are built on it.
`gate3` is already a shipped consumer on the production import path.

**Riding with tier 3'** because the two rows are one question asked
twice. Tier 3' pays a full certified quadrature where a SIGN would do;
this pays a second arena-wide reporting read for a VALUE no one solid's
read can supply. Both are *what does this tier-3 door promise, and at
what level does it owe the answer* — and both live in
`tier3_local_checks_marked`, `PlusVCheck` and the certificate types
that ATREST-1 just restructured. Specced apart they would restructure
the same hook twice; that is the mistake ATREST-1 and the per-solid
pair were deliberately merged to avoid.

The carrier's own row already says the shape is *a `SignCertificate`
whose hook is the LANE's rather than the certified one*. The question
this row adds to it: when the subject is multi-solid and the caller
wants a VALUE rather than a verdict, what is that value and who is
entitled to assume it was the check's own object.
