---
id: the-span-identity-is-not-a-theorem-of-the-floats
kind: issue
title: the arc span identity is a theorem of the reals and not of the floats: an adversarial torus at minor radius 1e18 contradicts it at the witness
status: open
opened: 2026-09-06
---

**Found by M10-9's fix pass, by an assertion it added and then removed
— which is the assertion doing its job.**

## What happened

Both reviews of M10-9 found the registrants DROPPING the door's typed
answer (`Real::register_equal` was not `#[must_use]`, and neither
registrant read it). The fix made the method `#[must_use]` and had each
registrant handle the answer, loudly:

```rust
let stated = built.register_equal(held);
debug_assert!(stated != SymRegistration::Contradicted, "…the stored
              bulge is not tan(theta/4) for this carrier");
```

That assertion fires, in the `f64` lane, on
`sweep::verbs_tubewall_r2_probes::r2_stored_inner_and_outer_radii_are_always_distinct`
— an ADVERSARIAL probe that sweeps a hollow torus's minor radius from
1 to 10¹⁸ with wall widths as small as a quarter of an ULP, through
`revolve::chain::build_chain` → `swept::placed_segment_spec`
(`swept.rs:505`).

## What it means, and what it does not

The span identity's proof is exact and stands: the stored bulge is
`tan(θ/4)` BY DEFINITION, so `param_end = 4·atan|b|` is the arc's
turned angle and rotating the first radius vector by it lands on the
second. **That is a theorem of the REALS.** At `f64`, at a minor radius
of 10¹⁸ with a wall below one ULP, the bulge has lost most of its
significand, `atan` and the rotation each round, and the two sides
separate by more than the witness's relative slack. The door then
REFUSES the registration — which is exactly right: nothing is recorded
and every decision that would have rested on it stays numeric.

So the finding is about the ASSERTION, not the door:

- **A registrant may not assert that its theorem survives the floats.**
  It can state the identity and handle the refusal; it cannot promise
  that every configuration a door admits is one where the arithmetic
  agrees. The assertion turned a door that correctly refused into a
  panic.
- **The refusal is not silent.** Inside a session it is counted
  (`SymCounts::registrations_refused`) and reported by the drive's
  receipt and `render()`. At a bare `f64` there is no session and
  nothing is recorded either way, so there is nothing to be unsound
  about.

Both registrants now bind the answer without asserting on it, and say
why at the site.

## What is owed

- A decision on whether the door should distinguish "refused because
  the claim is false" from "refused because the arithmetic could not
  tell at this scale". Today it cannot: `Contradicted` covers both, and
  the second is not a defect. A registrant that wanted to be loud about
  the first would need something the door does not have.
- If a loud channel is ever wanted, the honest place is the drive's
  receipt (which already carries the count) plus a document-scale row
  that asserts the count is ZERO on the M10 fixtures — a claim about
  the documents the kernel is measured on, not about every
  configuration the doors admit.
- Note for whoever takes it: the same argument applies to the RIM
  identity's assertion, which was removed with it. Neither has been
  observed to contradict on a non-adversarial document; the M10-9 pins
  assert `registrations_contradicted == 0` on the fixtures they drive.
