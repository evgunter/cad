---
id: pcurves-docs-claim-a-recycled-slot-can-read-another-half-edges-row
kind: issue
title: pcurves' stale-row consequence says a recycled slot can hand a dead row to a different half-edge; SecondaryMap's version check forecloses that, and the real residue is narrower
status: open
opened: 2026-09-14
---


Found by TOPO's `mef-and-kef-move-half-edge-runs-between-charts-and-leave-their-rows`
lane while placing `kef`'s killed halves in the posture table: the
kill leaves their two rows under dead keys, and the sentence that
names what that costs claims more than the map can do.

**The sentence.** `crates/topo/src/pcurves.rs`'s module docs, the
"stale-row consequence" paragraph under the `Neither` posture: *"a
`SecondaryMap` row outlives its key until the slot is reused, so
surgery on a body that already carries caches can leave a row attached
to a half-edge that no longer means what the cache says (or, once a
slot is recycled, to a different half-edge entirely)."* The
parenthetical is the claim at issue, and `mint_pcurves_of`'s "This is
NOT `mint_pcurves` over a subset of the map" section repeats its
premise ("invisible to tier 3 until the slot is recycled").

**What the map does.** `Body::pcurves` is a
`slotmap::SecondaryMap<HalfEdgeKey, PcurveCache<T>>` (`crates/topo/src/body.rs`,
`slotmap` 1.1.1 per `Cargo.lock`). A `SecondaryMap` slot stores the
key's VERSION beside the value, and `get`, `remove` and `insert` all
compare it (`secondary.rs`: `get` filters on
`slot.version() == kd.version.get()`; `insert` on a newer version
replaces the slot outright). A recycled arena slot mints a half-edge
key with a bumped version, so the new half-edge reads `None` from the
old row and its own first `insert` overwrites it. No half-edge can
ever read another's row through this map; the "different half-edge
entirely" arm does not exist.

**What the residue actually is**, narrower than written: a dead row is
reachable only by a holder of the DEAD key (`Body::pcurve(dead)` still
answers while the slot is unreused, because the versions still agree)
and occupies memory until the slot is recycled or a whole-body
`mint_pcurves` clears the map. Every reader that reaches rows through
faces — `validate_pcurves`, `props`, the tessellator, `chart_boundary`
— cannot reach it, which the `mint_pcurves_of` section already says.
So the consequence the `Neither` posture rests on is smaller than the
paragraph states, and the kill ops' entries (`kev`, `kef`, `kvfs`,
`kemr`) are safer than their bucket's doc implies.

**What would close it.** Re-state the parenthetical to what the map
does (a dead row is a holder-of-the-dead-key and memory residue, not a
mis-attribution), and say the same at the `mint_pcurves_of` section;
if the memory residue is worth removing, the kill ops' hygiene bullet
in `crates/topo/src/euler_kill.rs`'s module docs ("a killed entity's
arena slot AND its D5 provenance entry are removed together") is where
a `pcurves.remove` per killed half would belong, and that is TOPO's
side of the seam. TRIM's file, so filed here.
