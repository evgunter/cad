---
id: main-red-the-conic-centre-census-reads-eleven-in-injective-against-a-pin-of-ten
kind: issue
title: main red: the lint job's conic-centre census reads 11 production reads in topo's injective.rs against a pin of 10
status: open
opened: 2026-11-24
priority: P1
cost: E
---


Found by BOXFINISH's iso-image lane (PR 7393) on 2026-11-24, on two
merges of main, and not reproduced on main's own tree: the lane's whole
change to `crates/topo/src/injective.rs` is an eleven-line
`From<ChartAxis> for Axis` impl that reads no conic door, and it did not
touch the pin.

**What is red.** The lint job's step `clippy (--all-features)` ends in
`scripts/conic_centre_census.py --check` (ci.yml's second clippy
invocation, `--force-warn clippy::disallowed_methods`, piped as JSON).
It is not among `scripts/run-gates.sh`'s gates, so a lane's local "gates
ok" does not cover it.

- On PR 7393's head `94ad9510` (main merged at about 05:06 UTC; hosted
  run 45331550537): `crates/topo/src/injective.rs: 10 production read(s)
  of a conic's centre doors, pinned at 11`.
- On its head `8c05627864e` (main merged at about 06:20 UTC; ci.yml's
  command run locally by the lane): `crates/topo/src/injective.rs: 11
  production read(s) …, pinned at 10`.

`scripts/gates/conic-centre-reads.counts` holds `crates/topo/src/injective.rs 10`
since `01c5c8f2592` (05:27 UTC, "injective.rs's pin falls to its 10
reads (main red after the conic-symmetry merge)"); EMBED's merges have
rewritten `injective.rs` on both sides of that commit. So the pin and
the file have disagreed on main in both directions within two hours:
the pin fell to 10, and a later landing reads 11.

**Owed:** the count on main's tip taken once on main's own tree, then
either the eleventh read re-posed (the census's rule is that a pin only
falls) or the pin's owner's ruling; and the two lanes landing on this
file told of each other. The census is ONEROUND's; the file's recent
landings are EMBED's. (BOXFINISH orchestrator)
