---
id: the-registration-contradiction-check-misses-a-door-zero-the-read-gated
kind: issue
title: "The contradiction path cross-checks none of the zeros the decision path's read-shut walks find where the read-on form is not zero: S8, S13 and a door form the read leaves non-zero"
status: open
opened: 2026-10-06
priority: P3
cost: M
refs: [DECIDE-10]
---

Found by DECIDE-10's sweep and widened by its FULL review (MINOR-1).
The id is kept from the first, narrower filing, which named
`door_zero` alone.

**What the cross-check is.** A decision the numeric channel answers
with a DEFINITE sign still asks the tier whether its form is zero. A
plain or early zero there is a contradiction between two channels that
read no axioms (`contradicts`, via `discharge`), and a door zero is a
registration false over the box (`door_zero`,
`SymCounts::registrations_contradicted`). Both are in
`crates/geom-core/src/sym.rs`.

**What it no longer sees.** Since DECIDE-10 the decision path
(`discharge_retried` → `rungs`, `decision = true`) walks a rung again
with every value read shut wherever a read may have moved its form. A
zero that walk finds where the read-on form was NOT zero is a decision
the contradiction path never cross-checks: it runs the read-on rungs
alone. The class, each shape a row in `sym_root_rows`:
- **S8**, `atan(max(x+Z, 3) − max(x, 3) + x) − atan(x)`: the read-on
  form is gated and non-zero, because an atom's key carries its
  argument's gate (`an-atom-over-a-gated-argument-is-another-indeterminate`).
- **S13**, `max(x+Z, P)·Q − max(x, P)·Q` at a 4-term budget: both
  products freeze, and a frozen form is ungated and not zero.
- **A door form the read leaves non-zero**: the review's
  `atan((max(x·x, 3) − max(x, 3)) + x·x) − atan(x)` with `x·x`
  registered `= x` (`decide/10-review`, `sym_root_rows::decide10_review::review_door_through_an_atom`).
  It is `registered` with the read on and with it shut.
- **`door_zero`** answers `d.is_zero() && !d.gated`, so a false
  registration whose door zero the read gated is never counted. This
  was the first filing.

A zero the read-on rung already reached, gated, IS cross-checked:
`contradicts` counts a gated zero as a zero. So the gap is exactly the
zeros above.

**Why not fixed in DECIDE-10.** The contradiction path runs on every
definite margin. A shut walk behind every gated form there cost R2's
bracket replay 70–95 % in dev (DECIDE-10 Phase 1, candidate 1c), and
the shut walk is the expensive part on the tilted derived-frame family
(DECIDE-10's fix pass). A fix walks the shut rungs on the contradiction
path where the read-on form is gated and NON-zero, or after a gated
freeze. It needs a row first: a margin numerically definite where a
shut walk finds a zero, which can only be a tier defect, so the row is
a planted one.
