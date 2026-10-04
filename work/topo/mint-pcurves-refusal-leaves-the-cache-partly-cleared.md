---
id: mint-pcurves-refusal-leaves-the-cache-partly-cleared
kind: issue
title: mint_pcurves and mint_pcurves_of clear rows before a fallible mint, so a refusal leaves the caller's pcurve cache partly cleared
status: open
opened: 2026-10-04
priority: P2
cost: E
---


(Found by the §5 receipt of `work/topo/graft-stages-into-a-fresh-body-and-commits-on-success.md`.)

## What

Both are public `&mut Body` doors in `pub mod pcurves`, and both write the
caller's body before a step that can refuse:

- `pcurves::mint_pcurves` (`crates/topo/src/pcurves.rs`, ~:2283) takes the
  whole cache (`core::mem::take(&mut body.pcurves)`) and then calls
  `mint_faces(..)?`. That call inserts each face's rows in turn. A later face
  can refuse through `carry_rows(..)?` (Certify / RowInterval / Corrupt), or
  through its `return Err(e)`.
- `pcurves::mint_pcurves_of` (~:2345) clears the named faces' rows
  (`clear_face_caches`) and then calls the same `mint_faces(..)?`.

On `Err`, the body has lost its original rows and holds new rows only for the
faces minted before the refusal. A certification refusal is reachable on a
tier-1-valid body. Tier 1 survives, since the cache is not topology, so
`review_m1_pr5_internal`'s `ALLOWED` lists both doors. The at-rest pcurve state
the caller had does not survive. The docs promise that the mint "never drops a
certificate it cannot re-derive", and that holds only on `Ok`.

Every in-crate caller passes a stage (`transform.rs` `&mut out`, `shell.rs`
`&mut out`, `offset_axial.rs` `&mut work`, `splitting`'s result part). So the
exposure is to callers outside the crate.

## Build

Make the refusal leave the cache as it was. One way is to mint into a scratch
map and swap it in on success. Another is to restore `found` into
`body.pcurves` on every `Err`. Add a row that drives a certification refusal
partway through the faces and compares the cache before and after.
