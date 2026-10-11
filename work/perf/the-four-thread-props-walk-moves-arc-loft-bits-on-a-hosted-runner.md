---
id: the-four-thread-props-walk-moves-arc-loft-bits-on-a-hosted-runner
kind: issue
title: The four-thread props walk read arc_loft_1e9eps's volume bits off the serial golden on one hosted runner
status: open
opened: 2026-10-11
---


## What

`mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads`
(`crates/sweep/tests/mass_props_are_thread_count_invariant.rs`, function
`check_against_golden`) failed on PR 4649's hosted `test` job (run
38115743488, the slow-set step at the default ε = 1e-9). PR 4649 adds
only a test module and its slow-set entry and changes no kernel code.
Exactly one line of the digest moved, and none of its verdicts:

```
got  arc_loft_1e9eps v=4022487ed6a9d899 a=403995a3b53c3c57 vpad=3eb0783423f55555 apad=3fb5fb0bfc12a5e0 | verdicts n=15 h=fb22889cccf6acbb
want arc_loft_1e9eps v=4022487ed6a9da05 a=403995a3b53c3c54 vpad=3eb0783c2ee00000 apad=3fb5fb0bfc379cc0 | verdicts n=15 h=fb22889cccf6acbb
```

The volume, area and both pads differ in their low bits. On the same
head, the same test passed 3 of 3 times in a 4-vCPU container
(120 s each). The golden was last re-cut by PR 4557 (`flux/ellipse-ring`).

So the four-thread walk's reduction order, or something host-dependent
in it (CPU features, libm), is not bit-stable for this fixture on every
runner. The test's own claim is bit-determinism at any thread count
(`work/perf/plan.md`). Nobody has root-caused it. The lane that saw
this could not re-run the job (the integration gets a 403).

**The drift is the host's, not random.** The re-run on head `79c53486f`
(job 114407755680) failed with exactly the same bits. So did
`flux/sphere-arm`'s run 38117163664. Locally, the test also passes with
CI's build settings (`CARGO_PROFILE_{DEV,TEST}_OPT_LEVEL=1`,
line-tables-only debug). So the four-thread result is deterministic per
host. It differs between the hosted runner and the container the
golden was cut in. The likeliest cause is libm transcendentals (the arc
loft evaluates sin/cos), but that has not been measured. Until it is
settled, every PR whose diff touches `sweep` fails `gate ok` on this
test.
