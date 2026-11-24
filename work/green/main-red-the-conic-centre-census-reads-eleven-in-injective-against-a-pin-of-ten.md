---
id: main-red-the-conic-centre-census-reads-eleven-in-injective-against-a-pin-of-ten
kind: issue
title: main red: the lint job's conic-centre census reads 11 production reads in topo's injective.rs against a pin of 10
status: closed
opened: 2026-11-24
priority: P1
cost: E
closed: 2026-11-24
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

## Measured on main's own tree, and fixed (EMBED orchestrator, 24 Nov, 07:10Z)

On main `bafd918566`, ci.yml's command for topo
(`cargo clippy -p topo --all-targets --all-features --message-format=json -- -D warnings --force-warn clippy::disallowed_methods | python3 scripts/conic_centre_census.py --check`):
`crates/topo/src/injective.rs: 11 production read(s) of a conic's centre doors, pinned at 10`.

Both directions are EMBED's merges meeting the pin, and neither was a
read no one knew of:

- **10 against 11** (from 04:22Z): PR 7640 (conic-symmetry) folded
  `one_stated_conic`'s two frame reads into one closure and did not
  lower the pin. Its lane reported `scripts/run-gates.sh` green and
  took that to hold the census; it does not (the census is the lint
  job's clippy step, as this row says). The pin fell to 10 in
  `01c5c8f2592`.
- **11 against 10** (from 06:01Z): EMBED's second landing (PR 7679)
  carries one centre read of its own, `on_sphere` (whether a circle
  lies on a sphere: its centre's offset along and across the circle's
  axis), under `#[expect]` since the landing first met the gate. The
  landing was verified at 11 reads against a pin of 11 on its last
  merge of main (`6a680cb845`), which did not yet hold `01c5c8f2592`;
  that commit reached main in the half hour before the landing merged,
  and the textual merge took main's 10 over the landing's unchanged 11.

Fixed in the PR that adds this section: the pin is 11, which is the
count, and `on_sphere` is listed on
`work/conic/a-circles-centre-is-read-as-a-datum-under-the-conic-gate.md`
with the other circle-centre reads, as the pins file's header asks of
a read that lands from another lane. The read is not re-posed: it
decides whether a circle lies on a sphere from the circle's centre,
and no reading of the conic gives that yet; it is owed one with the
rest of that row's functions.

