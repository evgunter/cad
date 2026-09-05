---
id: ruled-band-has-no-bit-identity-corpus-row
kind: issue
title: The ruled band has no bitdump row, so blend PRs' C1 never covers ruled_phase
status: closed
opened: 2026-09-05
track: T
closed: 2026-09-05
pr: 1943
branch: fillet/t-riders
---

## What

**Every blend PR claims "every existing carve is bit-identical to the
merge base" on a corpus that never reaches the ruled band.**
`crates/sweep/tests/bitdump.rs:1-33` names its coverage — the die (open
plane–plane chains + corners), the pipped die's LADDER pip rim, the
chamfered cube, one convex closed rim per coaxial arm family, one
CONCAVE rim per closed-rim door, the shell open-box corpus, the
extrude/revolve corpus — and `review_arms2_r1_probes.rs:449`
(`bitdump_dome_annulus`) adds the dome's one-edge annulus. None of them
builds a ruled link, so `crates/sweep/src/blend/surgery.rs`'s
`ruled_phase` (the flat milled along a rod, terminating at transverse
caps) executes in no dump row at either SHA of any differential.

`grep -rn "bitdump" crates/sweep/tests/` returns three files:
`bitdump.rs`, `shellfix1_bitdump.rs` (shells, `SHELLFIX_BITDUMP_DIR`)
and `review_arms2_r1_probes.rs`. `crates/sweep/tests/fillet_h7_transverse_cap.rs`
— the ruled band's own suite, from the unit that built it — has no
`BITDUMP_DIR` row at all.

**What that costs.** A carve change that moved bits only in
`ruled_phase` would pass the C1 differential of every blend PR, empty
`diff -r` and all. It is not hypothetical for the door FILLET-T just
touched: two of the eight `kef` sites in `surgery.rs` are
`ruled_phase`'s (`ruled crease kef` at the crease excision, `cap sliver
kef` at each end's fold-in), and their bit-identity in that PR rests on
`fillet_h7_transverse_cap.rs` passing rather than on a dump.

## Fix — landed, PR 1943

`crates/sweep/tests/bitdump.rs::bitdump_ruled_band`: the rod with a flat
milled along it (`test_support::rod_with_flat`, the module docs' own
fixture), both creases carved in one `fillet_edges` call at
`ROD_FILLET`, dumped by that file's own `dump` and armed by the same
`BITDUMP_DIR` read as every other row. The corpus comment at the head of
the file names it. `ruled_band.txt` is now one of the 14 files a blend
PR's C1 differential compares.

The same PR gave `dump` ONE home. `review_arms2_r1_probes.rs` carried a
copy that omitted the mass-properties line, so the dome-annulus row —
the only armed row on the plane–sphere annulus path — could not have
seen a volume, area or pad move at all. It now calls
`bitdump::dump`.

## Fence

Track T — `sweep/`. **Fence:** `crates/sweep/tests/bitdump.rs`.
