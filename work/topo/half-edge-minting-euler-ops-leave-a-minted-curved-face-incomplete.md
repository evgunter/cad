---
id: half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete
kind: issue
title: mev, mef and mekr mint half-edges into a cached curved face and no pcurve row with them, leaving the face half-minted
status: open
opened: 2026-09-13
refs: [validate-pcurves-never-recertifies-a-face-it-finds-incomplete]
---



Found by this program's `split-edge-children-lack-pcurve-rows-on-curved-charts`
lane as the class sweep of its own defect, and measured.

Every operator that mints half-edges into an EXISTING loop has the
shape the split-edge finding named: two fresh half-edge keys join a
face that may already carry pcurve rows, no row is minted with them,
and the face is left half-minted — tier-3 invalid with one
`Pcurve MissingCache` per new half until a caller re-mints.
The minting sites are `Body::mint_halves`'s callers
(`crates/topo/src/euler.rs`: `mev_fan_execute`, `mev_lone_execute`,
`mef_chords`, `mef_lone`; `crates/topo/src/euler_ring.rs`:
`mekr_mint`, the shared mint of `mekr`'s four entries;
`crates/topo/src/split.rs`: `split_edge`, now closed).

Measured on the minted cylinder wall of
`crates/topo/tests/split_edge_pcurve_rows.rs`: `mev_line` at a `Fan`
site on the wall face's own loop returns `Ok`, and
`validate_pcurves` then reports two `MissingCache` findings for the
half-edges it minted.

**Why `split_edge`'s closing does not reach them, which is the whole of
this row.** A split had the material to do better: a `Pcurve` is a
function of the carrier parameter and holds no interval of its own, so
each child's chart image IS the parent's restricted, and the
restriction re-certifies through `PcurveCache::certify`, which sits in
`geom_brep`'s `impl<T: Decide>` block. `mev`/`mef`/`mekr` mint a
BRAND-NEW edge with no parent row to restrict; its chart image has to
be DERIVED, and every derivation door in `topo::pcurves` (`pcurve_of`,
`nurbs_iso_derive`, `mint_pcurves_of`) carries `T: PcurveFittedLane` —
the bound ripple `mint_faces`'s own comment banks. So closing this
either widens those operators' bound from `Decide` (and every
generic caller's with it) or keeps the declared primitive posture and
says so at each op.

The posture is declared today —
`pcurves::staleness_posture::DECLARED` carries each of these as
`Neither`, "Euler operator" — so this is not an undisclosed defect. It
is open because the declaration's safety argument leans on a backstop
that does not fire in this exact state (`work/trim/`'s
`validate-pcurves-never-recertifies-a-face-it-finds-incomplete`), and
because a caller reading `mev`'s own entry is not told that a curved
cached face will be tier-3 invalid on return.
