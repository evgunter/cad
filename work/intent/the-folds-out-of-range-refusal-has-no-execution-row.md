---
id: the-folds-out-of-range-refusal-has-no-execution-row
kind: issue
title: fold_bundle's PoseOutOfRange has no execution row since msolve12 retired, and now also asks a lone mate
status: closed
opened: 2026-10-18
closed: 2026-10-18
priority: P2
cost: E
---

Found by the review of INTENT stage 3 F (PR #5299, finding 8).

**No execution row.** `fold_bundle`'s `is_finite_length(held.representative.translation.norm())` branch, which refuses `MateFault::PoseOutOfRange` (`crates/editor-core/src/mate/solve.rs:880`), is still live. `coset-translation-finiteness-reads-the-norms-length` cites it. But no execution test reaches it now:
- `msolve12_honest_translation`'s `out_of_range_through_the_doors` and `out_of_range_at_the_coset` built it from two planes, which F's bundle rule now refuses at the door, and that file was retired.
- The only remaining reference is a Display row in `refusal_concision_chains`.

**A behaviour change the migration did not report.** The check now also runs after the first mate's fold. A lone mate whose pose has an overflowing translation norm would refuse with `held == added`. Whether any lone mate can reach that through the pose doors is unshown.

**The work.**
1. Find a reachable bundle that drives the meeting's translation past the finite range, such as a ball and a direction met far away (`coset::ball_meets`), and pin it as a row.
2. Or show the branch unreachable, and retire it in favour of the pose doors' own range refusals.

## Closed (2026-10-18): unreachable, and retired

The branch is unreachable behind the bundle's arm. `bundle_arm` takes the
largest `|o_on| + |o_to|` over the bundle's mates as `datum`, and is formed
before the solve (`eval/wire.rs` `fold_with`). `Arm::of` refuses an arm whose
sixteenfold square overflows, so `datum < 3.4e153`. A lone mate's
representative then moves by at most `datum`, and the one transverse meeting
(`coset::ball_meets`, a ball and a direction) by `p − R·p + R·t`, at most
`3·datum`. Its squared norm stays inside the format.

So `fold_bundle` no longer asks for a range, and its doc carries the argument.
`MateFault::PoseOutOfRange`, its class `MatePoseOutOfRange`, its tag
`mate_pose_out_of_range` and its binding (`held`, `added`) are retired with
it.
