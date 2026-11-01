---
id: a-rescued-adjacent-pair-whose-joint-decides-tangent
kind: issue
title: The shared-end rescue's joint-coupling fuzzer finds a pair whose joint decides Tangent
status: open
opened: 2026-10-23
---


## What

`crates/profile/src/seg.rs`, `pair_contact_tests::every_pair_the_shared_end_touch_rescues_escalates_at_its_joint`
(a shape-1 fuzzer, `test_utils::fuzz`), went red on hosted CI at the
default ε on PR 5635 (CI run 40023948365, job 120655712172). That PR
does not touch `crates/profile`. The row's claim is that every pair
`touches_only_near_the_shared_end` rescues also escalates at
`joint_tangency`. This draw breaks it: the pair was rescued, and the
joint DECIDED `Tangent` on `path_junction_turn`.

```text
prev (529.9224040052325, -386.85094177922434) → (-4.670652184157155, -9.718533077548313)
next (-4.670652184157155, -9.718533077548313) → (-344353.06262695126, 242913.2618424182)
CAD_FUZZ_SEED=0x092282367a715284 CAD_FUZZ_EFFORT=1
```

Here the short leg arrives (the reversed-traversal arm) and the long
leg is about 4·10⁵ m. Per the row's docs, a joint that decides where
the pair pass escalated lets a loop through that the pair pass
escalated. That is either a soundness question for the rescue rule or
a gap in the row's own model of the joint's lever. Undiagnosed. The
first step is to replay the seed and pin it as a deterministic row
beside the fix (discipline §8).
