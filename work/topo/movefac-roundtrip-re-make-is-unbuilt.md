---
id: movefac-roundtrip-re-make-is-unbuilt
kind: issue
title: "seqgen's roundtrip skips every movefac choice: its kfmrh + mfkrh re-make is unbuilt, and the catalog row's pointer to it named a row about something else"
status: closed
opened: 2026-09-30
refs: [movefac-row-skips-three-component-shells]
priority: P3
cost: M
closed: 2026-10-05
branch: topo/movefac-roundtrip-remake
pr: 4053
---

## What

Found by the receipt pass of `movefac-row-skips-three-component-shells`.

`crates/topo/src/seqgen.rs`'s `roundtrip` returns
`SkippedIrreversible` for every `OpChoice::Movefac`, and the module
docs ("The sites with no re-make") and the arm's own comment say why:
the re-make is unbuilt, not impossible. It would mirror the
`KfmrhFuse` arm run backwards: `kfmrh` fuses each minted shell back
and `mfkrh` re-promotes the face the fusion demoted. What is missing
is the site search, which must succeed before the partition runs: an
`f2` that is a ring-free face of each component that moved.

Both places pointed at `movefac-row-skips-three-component-shells` as
the row that carries it. That row was about which shells the catalog
offers, not about the re-make, and closes with the catalog row now
offering every shell of `c >= 2` components; the pointers now name
this row.

The catalog now offers shells of three and more components, so a
re-make pairs `c − 1` fusions with `c − 1` promotions, and the
canonical form's positional shell order (`crate::iso`'s honest
limits) decides which order they must run in.

## The shape to give

`roundtrip`'s `Movefac` arm searches its re-make sites before the
partition and runs the partition, the fusions and the promotions,
skipping only where the search fails; `OpChoice::may_skip_roundtrip`
narrows to that site the way the `Kev` arm's does.

## Closed

`roundtrip`'s `Movefac` arm runs the re-make: `movefac_remake_sites`
reads `f1` (the seed of the component that stays) and one ring-free
`f2` per moved component before the partition, and the arm fuses
each minted shell back into `shell` and re-promotes. It skips only
where a moved component has no ring-free face, and
`OpChoice::may_skip_roundtrip_at` admits exactly that site. Any
fusion order restores the shell list, since every fusion keeps
`shell` and retains a minted shell (all appended last) out;
`seqgen::tests::movefac_roundtrip_restores_the_partitioned_shell`
pins a three-component shell and a two-component one ahead of
another shell, the second the case a fusion that kept the minted
shell would fail on.
