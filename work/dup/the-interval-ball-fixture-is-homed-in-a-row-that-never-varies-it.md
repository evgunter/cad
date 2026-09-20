---
id: the-interval-ball-fixture-is-homed-in-a-row-that-never-varies-it
kind: issue
title: m5_s12's ball(r) is parameterised only for another file, and sweep::test_support already hosts the revolve family
status: open
opened: 2026-09-19
---

## Finding

- **Where**: `crates/sweep/tests/m5_s12_curved_ops_interval.rs`,
  `certified::ball` (and `certified::recut_ball` beside it), against
  `crates/sweep/src/test_support.rs`'s revolve family.
- **Importance**: low-medium
- **Confidence**: sure about both facts below; the candidate home is a
  proposal and is not measured
- **Raised by**: the `dup/private-box-builders` lane, 2026-09-19, in
  its own fourth-pass review — this is residue of that PR's own repair,
  not something it found elsewhere

That PR folded a ball fixture that was written out verbatim in two
files. The fold is right and the duplication is gone. Where it landed
is the question this row holds, and two facts say it is the wrong home:

1. **The parameter exists only for the other file.** `ball`'s own
   rustdoc concedes it: *"It takes `r` because that suite's E1 row
   varies it; this file only ever wants 1."* A knob never turned in its
   own home is a fixture living where it does not belong.
2. **`m5_s12_curved_ops_interval::certified` is now a fixture home for
   a file that is not it.** `review_arceval_r1_probes` imports `ball`,
   `plate`, `recut_ball` and a constant from it. That is a shipped
   row's module serving as a vocabulary module for a reviewer probe,
   which is the shape this program exists to notice.

**A candidate home the fold did not weigh.** `sweep::test_support`
already hosts the revolve family — `revolved_about_y_at`, `dome`,
`ball_poled_z_at` — and a y-poled unit ball is the same kind of member
as those. It is generic in the `Decide` scalar, so the `Interval` lane
is served without a second copy, and both consumers already name that
module for `brick`.

**Why not in that PR.** The move is cross-crate (`crates/sweep/tests/`
to `crates/sweep/src/test_support.rs`), it has to answer the routing
question that module's own header sets — *"A fixture only earns a place
here once a consumer OUTSIDE this crate needs it or a second suite
inside it does"* — and the PR carrying it was four review passes deep.
It is a unit, not a fix.

The constant (`RECUT_MAPPED_ENCLOSURE_HI`) does **not** move with the
ball and must not: it is a measurement of one row's chain, and its home
is the row that owns the claim.

