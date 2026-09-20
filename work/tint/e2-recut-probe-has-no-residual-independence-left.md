---
id: e2-recut-probe-has-no-residual-independence-left
kind: issue
title: review_arceval's E2 now re-executes the shipped row's fixture, constant and predicate, and can no longer disagree with it
status: open
opened: 2026-09-19
---

## Finding

- **Where**: `crates/sweep/tests/review_arceval_r1_probes.rs`,
  `e2_recut_escalation_hi_is_pinned_to_the_measured_constant`, against
  `crates/sweep/tests/m5_s12_curved_ops_interval.rs`'s
  `interval_sphere_subtract_decides_definitely_after_the_recut`.
- **Importance**: low-medium — a probe that cannot disagree with the
  row it witnesses
- **Confidence**: sure; it is a property of the two rows' text after
  `dup/private-box-builders` landed
- **Raised by**: that lane, 2026-09-19, disclosing the consequence of
  its own fix

E2 was opened as a **staleness pin**: the shipped row bounded its
escalation's `hi` from above only, so a partial tightening of the arc
chain could leave `RECUT_MAPPED_ENCLOSURE_HI` stale in silence, and E2
re-ran the fixture and pinned `hi` from both sides.

Two things have happened to that argument, neither of them E2's doing:

1. **The shipped row was tightened to the same both-sides form.** It
   asserts `hi == RECUT_MAPPED_ENCLOSURE_HI`. E2's reason for existing
   was absorbed. (The file's header still described the old ceiling-only
   form until 2026-09-19.)
2. **The fixture and the constant are now single-sourced.** E2 builds
   its operands from `m5_s12_curved_ops_interval::certified::{plate,
   recut_ball}` and reads that module's constant. That was done to
   repair a real defect — the fold had left a prose sentence as the only
   thing holding two hand-copied constants together — but it also
   removes the last axis on which E2 could differ.

**So E2 now runs the same subtract on the same two bodies and asserts
the same predicate against the same constant.** It cannot red while the
shipped row's arm is green, and it cannot detect the constant going
stale, because it does not restate it. What remains is a second
execution of one row, in a second module, at the one configuration
either runs at: (interval, 1e-12).

The question is S-TINT's, not S-DUP's: a probe that cannot disagree
with its subject is a coverage question. Three answers are open and the
lane that filed this has no view on which is right — restore an
independent axis (its own fixture, deliberately, with the divergence
risk stated and guarded); keep it as a cheap second execution and say
so at the row; or retire it, which is a reviewer-probe file's own
owner's call.

**Do not resolve it by re-duplicating the constant or the fixture.**
That is what was there before and it is what the header of that file
now explains at length.

