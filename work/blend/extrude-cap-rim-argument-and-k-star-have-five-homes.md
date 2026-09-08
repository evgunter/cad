---
id: extrude-cap-rim-argument-and-k-star-have-five-homes
kind: issue
title: The K-conditional cap-rim argument is written out four times and K* ≈ 1.272 is spelled in seven places
status: open
opened: 2026-09-08
---


Found by the BLEND unit 2 style review (PR 2122), which read the extrude
bundle's prose end to end. Two entangled duplications, filed as one because
any fix has to move the same paragraphs.

## 1. The K-conditional cap-rim argument is written out four times

The argument — the two direction gates bound an admitted extrusion vector's
tilt, so the cap–wall angle obeys `sin θ ≥ K/√(K² + 1)` while `Smooth` needs
`sin θ ≤ 1/K`, and the two close exactly when `K⁴ > K² + 1`, so the
`SmoothCapRim` arm is reachable only below `K*` — is stated in full, with its
own derivation, in four places:

- `crates/sweep/src/extrude.rs:47-60` — the module docs' rim-upgrade step.
- `crates/sweep/src/extrude.rs:240-259` — the `ExtrudeError::SmoothCapRim`
  variant doc.
- `crates/sweep/src/extrude.rs:1237-1279` — the `Smooth` arm of `upgrade_rim`,
  the longest and most specific of the four (it is the only one that also
  carries the arc-leg-cannot-reach-this argument and the second-order aside).
- `crates/sweep/src/lib.rs:88-99` — the crate docs' cap–wall rim bullet.

Two of the four already name a third as the home: `extrude.rs:55-57` says the
argument "is written at the arm in `upgrade_rim`", and `lib.rs:99` says "the
argument is at `extrude::upgrade_rim`'s arm". They then write it out anyway.
The same K-and-ε reasoning therefore has to be kept consistent across four
independently maintained copies; the crate's own convention (`lib.rs:24`,
"stated once — owned here") is the rule it violates.

## 2. `K* ≈ 1.272` is spelled in seven places, one of them user-facing

The constant is the positive root of `K⁴ = K² + 1`, i.e. `√φ = 1.27201965…`.
It appears as prose in five places, as a literal inside a user-facing message
in a sixth, and is computed from its definition in a seventh:

| site | form |
| --- | --- |
| `crates/sweep/src/extrude.rs:55` | prose, `` `K* ≈ 1.272` `` |
| `crates/sweep/src/extrude.rs:249-250` | prose, the defining equation + `` `K* ≈ 1.272` `` |
| `crates/sweep/src/extrude.rs:1237` | prose, `` `K* ≈ 1.272` `` |
| `crates/sweep/src/extrude.rs:1255` | prose, `` `K > K* ≈ 1.272` `` |
| `crates/sweep/src/lib.rs:96` | prose, `` `K* ≈ 1.272` `` |
| `crates/sweep/src/extrude.rs:353` | **a literal in `Display`** — `"…above K* = 1.272…"`, text a user reads out of `ExtrudeError::SmoothCapRim` |
| `crates/sweep/tests/review_fillet_h6_r2_probes.rs:118` | computed — `((1 + √5)/2).sqrt()`, asserted into `1.2..1.3` |

The `Display` site is the one that actually costs something: it is a rounded
decimal in a message the user sees, with no named constant behind it and no
test pinning the string against the root, so a re-derivation that moved the
number would leave the refusal message stating the old one.

## Why this unit did not fix it

Both halves reach outside BLEND unit 2's fence:

- The four-fold argument is H6 prose (`fillet_h6_cap_rim`,
  `review_fillet_h6_r{1,2}_probes` are its evidence), and collapsing it to one
  home is a decision about where the H6 argument lives.
- The `[ev]` question on K — what the shipped multiplier should be, and
  whether `K*` is a threshold the kernel names — is open on PR 2119. Naming a
  `K_STAR` constant or rewording the refusal message pre-empts it.

## What a fix looks like

One home for the argument (`upgrade_rim`'s arm, which the other three already
point at), the other three cut to a sentence and a link; and `K*` named once
as a constant with `Display` formatting it rather than spelling it, with the
`review_fillet_h6_r2_probes.rs:118` computation asserting against that
constant instead of a `1.2..1.3` window. Both wait on the K question.
