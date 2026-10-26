---
id: a-split-lineage-lists-a-joined-edge-over-chords-and-an-operand-rim-as-minted
kind: issue
title: SplitResult::lineage lists an edge the merge's join made over section chords and an operand rim as minted in the cut face's division, though it is also the image of that rim
status: open
opened: 2026-10-19
priority: P3
cost: M
---


## Finding

From the review of PR 5334 (WIRE, 2026-10-19). `SplitResult::lineage`
(`crates/topo/src/splitting/finish.rs`) lists, as `minted` in each
divided operand face's `Division`, every live edge between a piece of
that face and a face the cut drew. Since PR 5334 that test reads
`SplitNaming::holds_section`, which also counts an operand face the
coplanar merge kept for a group holding a section face: the notched
floor of `crates/editor-core/tests/split_pinch_keeps.rs` (`NOTCHED`,
split at `y = 1`), which absorbs the three sections in the lower half.

The shape that reaches it is that fixture's lower half: each cap's
edge along `y = 1` is one edge the merge's join made over the cap's
section chords and the floor's own cap rim (`RimEdge(cap, floor)`).
It lies between the merged top (kept face: the floor) and the cap's
lower piece, so it is listed as minted in the cap's division, and it
is also the image of the floor's rim through the joins (`Kept`, read
through `Lineage::joined`). Its chord stretches are minted in the
cap's interior and its rim stretch is not: the listing is right for
part of the edge.

Whether that misleads a carry (`boolean::records::landings` reads a
minted cell's `OneCut` backing) was not measured: no contact-record
fixture splits on a plane holding an operand face. The narrower
listing (only an edge whose cover is chords alone) would drop the
chord stretches from the cap's exhaustive division; the reading this
wants is a minted cell that is part-image, which `Division` cannot
state.

Signed (wire-splitmax lane).

## Slate triage (2026-10-26, CLEAVE)

- Banded P3: a lineage listing right for part of an edge, its effect on a carry unmeasured: latent unsoundness, unreached by any known input, P3 by `work/README.md` Priority.
- Priced `M`, the effort the body describes.
- Moved to WIRE (`work/wire/`) by `git mv`, id and body unchanged: whose lane filed it: the rows a node publishes and the lineage they are carried by. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
