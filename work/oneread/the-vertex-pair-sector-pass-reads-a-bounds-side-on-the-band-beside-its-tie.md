---
id: the-vertex-pair-sector-pass-reads-a-bounds-side-on-the-band-beside-its-tie
kind: issue
title: pair_codes reads each bound's side of the other vertex's sector face on the band alone, about a partner its tie off the point
status: open
opened: 2026-11-02
priority: P3
cost: E
refs: [vtxfac-partner-side-reads-a-partners-bounds-on-the-band-beside-its-tie, the-cone-sites-read-every-bounds-side-through-bound-read]
---

Found by `the-cone-sites-read-every-bounds-side-through-bound-read`'s sweep (§5: a side read on the band alone where the reading is about a point a tie off), the sibling of `vtxfac-partner-side-reads-a-partners-bounds-on-the-band-beside-its-tie` outside a cone.

`sectors::pair_codes` (the vertex-pair sector pass, `pair_search`, and recl's edge-sector germ, `resolve_edge_sector`'s `pair_codes(&a_sectors[a], &b_sectors[b], band)`) reads each sector's bounds against the other vertex's sector face through `sector_code` → `datum_code`, which is `bound_read` on `Reading::plain(band)`. Each bound departs from its own vertex, and the other vertex's face passes through that vertex's point, which stands off it by up to the pair's tie (within the zero band). As with `partner_side`, a bound decided off the face at first order past `K·zero` is not exposed; the exposure is a **descended** bound, tied at first order, whose parting is decided past `K·zero` on its own while the first-order residue and the offset beside it go uncounted.

Owed: read the pair's codes on `link::metric::Reading::tied` over the pair's offset (as `partner_side` now does, `vtxfac::tests::a_partners_descended_bound_is_read_over_its_offset`), or show the offset cannot reach a decided code; with its outcome diff, since these codes key Table I's records (the edge-sector site's own `flank_read` stays on the band, item 2 of `the-line-cone-reads-a-flanks-side-beside-the-one-bound-side-reading`'s `## Decided`, so a change here is a design question about whether the pair pass's codes are a cone site's).
