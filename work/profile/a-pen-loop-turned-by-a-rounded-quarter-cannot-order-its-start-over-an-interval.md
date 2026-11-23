---
id: a-pen-loop-turned-by-a-rounded-quarter-cannot-order-its-start-over-an-interval
kind: issue
title: Unguided validation at an interval scalar cannot order a boxed loop's start
status: open
opened: 2026-11-20
priority: P3
cost: M
---

Found by PROFILE's pen-rectangle lane (2026-11-20). Re-scoped by that
PR's review (PR 7264): the first version of this row blamed a rounded
quarter turn, and that is too narrow.

**What is true.** Unguided validation (`ConstructedProfile::validate`,
`Profile::validate`) at an interval scalar orders each loop's vertices by
`validate::lex_min_index` → `lex_less`. That comparison runs under
`prove`'s exact-order band, an ulp wide, so it reads two enclosures of
nearly equal coordinates as undecided and refuses `Escalated { site:
Loop { .. }, gate: CanonicalOrderX }`. `validate.rs` already documents it:
the guided pass's doc says the order is "indeterminate at an interval
scalar on essentially every input, because two enclosures of
nearly-equal coordinates overlap". That is why guided validation takes
the representative, the start and the reversal from the `f64` record
rather than deciding them again.

Measured:
- **A pen rectangle**: heading 0, `line(w)`, `turn(fl(π/2))`, `line(1)`,
  `turn(fl(π/2))`, `line(w)`, `line_to(Start)`, with `w = 1 ± h` at
  `Sym<Interval>`. It refuses this way at every h, a point box
  included: the margin is `[-8.9e-16, 2.0e-15]` at h = 0 and
  `[-0.1, 0.1]` at h = 0.05. Its fourth vertex's x is
  `cos θ·(1 + 2w·cos θ)`, about `6e-17` over the reals, with `w` on
  two terms. This is pinned as today's truth in
  `crates/profile/tests/a_boxed_rectangle_validates.rs`.
- **The review's corpus** (PR 7264) at plain `Interval`, from ±1e-12 up:
  - a rectangle with all corners boxed refuses the same way;
  - so does a triangle with all vertices boxed;
  - so does a rectangle far from the origin.
- **Not specific to a rounded quarter:** the pen turned by `Real::pi() / 2`
  validates only because the symbolic tier proves the x difference
  identically zero, and the y comparison then decides.

**Who is owed it.** No production caller has been shown to validate
unguided at a lane scalar. The review found none, and this row has not
re-swept for one. The document path replays guided, and the driver
certifies the pen rectangle over ±0.05 in both spellings
(`crates/editor-core/tests/a_boxed_rectangle_certifies_over_its_box.rs`).

So this is a property of the design, recorded so that a future caller of
unguided interval validation finds it. Close the row, or raise it, when
such a caller exists. Repairing it would mean letting the order settle
for a vertex that is minimal within the band, which changes
`lex_min_index`'s contract.
