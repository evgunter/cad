---
id: a-direction-and-its-length-are-spelled-as-a-pair-at-23-sites
kind: issue
title: 23 sites spell (v / |v|, |v|) by hand beside Vec3::normalize: give the pair one home (Vec3::direction_and_length through the same prescale)
<<<<<<< HEAD
status: dispatched
=======
status: closed
>>>>>>> origin/main
opened: 2026-11-06
priority: P2
cost: M
refs: [6572]
<<<<<<< HEAD
=======
closed: 2026-11-06
pr: 6575
branch: onecopy/direction-and-length
>>>>>>> origin/main
---


From PR 6572's FULL review.

## What

PR 6572 kept 23 sites dividing by hand because the length feeds other arithmetic, for example `carrier_touch.rs:165` (`n − radius`, `radius/n`), `implicit.rs:272` and `ring_path.rs:309`. Most of them spell the same pair, `(v / |v|, |v|)`.

- The normalise gate cannot see them.
- They lack the out-of-range totality `normalize` gained through its prescale.
- Sites with a guard on `let n = v.norm()` now compute the norm twice.

## What closing it takes

1. Give `Vec3` a `direction_and_length()` that returns both through the same prescale.
2. Port the pair sites onto it, bit-identical in range.
3. Keep the sites whose zero behaviour differs (for example `ruled_region.rs:483`), each with its reason.
4. Extend the gate. Its header should also name its two other blind spots: nested parentheses in the second pass, and a call with arguments in the chain.
<<<<<<< HEAD
=======

## Closed

PR 6575 gave the pair its home, `Vec2`/`Vec3::direction_and_length` through `normalize`'s prescale, with `normalize` as its first half. It ported 44 sites. `scripts/gates/normalize-by-hand.sh` now refuses `let n = v.norm(); … v / n`, and its header names the blind spots, including the two named above.

`ruled_region.rs`'s guarded `dir` and `UnitVec3::new`'s unscaled-norm refusals stay as they are, for the reasons given in the PR body.

The review fix pass (same PR) gave "what an out-of-range direction does" one home, `geom_core::length_out_of_format`. It is `UnitVec3::new`'s refusal of a plain norm that overflowed or underflowed, extracted, and every door that refuses a direction reads it. The total readings, `normalize` and `direction_and_length`, answer past either end.
>>>>>>> origin/main
