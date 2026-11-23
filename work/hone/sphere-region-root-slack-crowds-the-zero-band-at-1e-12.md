---
id: sphere-region-root-slack-crowds-the-zero-band-at-1e-12
kind: issue
title: bool_sphere_region_roots_slack's zero-classified margin on lily_walls crowds the coincidence band at eps 1e-12 (one k-lint rule-2 flag), the sibling of circle-torus root slack
status: closed
opened: 2026-10-07
priority: P3
cost: E
refs: [circle-torus-root-slack-crowds-the-zero-band-at-1e-12]
closed: 2026-10-26
---


## What

Found by CLEAVE's `cleave/lily-clearance` lane while reading k-lint on
main. `scripts/k_probe_sweep.sh` at `origin/main` `1f27ea0881`
(2026-10-07), linted by `tools/k-lint`, flags one
`bool_sphere_region_roots_slack` sample (decided in
`crates/topo/src/boolean/sphere_region.rs`: `ROOT_ROWS.root_slack`, read
by `ray_roots` through `first_harmonic_roots_in_band`) on `demo/lily_walls`, at ε 1e-12 only:

```
FLAG demo/lily_walls:bool_sphere_region_roots_slack |m|=1.5093202990315798e-14 band_zero=1e-12 — zero-classified within 10^2 of the coincidence threshold
```

The same sample reads 1.509320e-14, `zero`, at ε 1e-6 and 1e-9 too: the
margin does not move with ε, only the rule's threshold (`band_zero /
10^2` = 1e-14 at 1e-12) does. It is the one sample of the name's
lily_walls tail above 1e-14 (the next are 9.0e-15 and 6.8e-15). The
nightly of 2026-10-06 (run 37460629022, head `d9bdfaf9`) carries the
same flag, beside eight `bool_sphere_region_roots_extreme` flags
(1.03e-14 / 1.06e-14, also rule 2 at 1e-12) that main no longer
records.

This is the shape of
`work/germ/circle-torus-root-slack-crowds-the-zero-band-at-1e-12.md`:
a root slack built from geometry and machine-epsilon rounding,
classified against the ε band. Owed: confirm from the source that
nothing on the slack's path reads ε, then rule with the germ row
whether rule 2's zero arm applies to an ε-independent noise statistic
at all (a rule-4-style allow-list entry or a different comparand). Do
not change geometry to get under the lint.

## Ev's ruling reaches this row (PR 4567, 2026-10-10)

The circle × torus sibling's ruling covers this flag too: rule 2's
zero arm is a tally that fails nothing, so the 1.5093e-14 sample on
`demo/lily_walls` at 1e-12 now prints as a tallied note
(`bool_sphere_region_roots_slack: 1`) and no longer counts toward
`k-lint (dev-probe)`'s failure. That row stays red for its other
findings. Built on `germ/klint-zero-arm-tally`. Whether anything else on
this row is still wanted is HONE's call.

## Closed (2026-10-26, tracker sweep)

**Fixed by the ruling this row quotes.** Ev ruled on PR 4567, and the ruling was built on `germ/klint-zero-arm-tally`. Rule 2's zero arm is now a tally that fails nothing (`tools/k-lint/src/lib.rs:5`–`:26`, `zero_arm_usage`), so this flag no longer counts toward `k-lint (dev-probe)`. The sibling `work/germ/circle-torus-root-slack-crowds-the-zero-band-at-1e-12.md` closed on 2026-10-10 on the same build. Its owed check (whether rule 2's zero arm applies to an ε-independent statistic at all) is the question that ruling answered. Nothing else on this row asks for a change.
