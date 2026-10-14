---
id: mint-pcurves-refusal-leaves-the-cache-partly-cleared
kind: issue
title: mint_pcurves and mint_pcurves_of clear rows before a fallible mint, so a refusal leaves the caller's pcurve cache partly cleared
status: closed
opened: 2026-10-04
closed: 2026-10-05
pr: 4033
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

## Closed

PR 4033 fixed this. In both entries the rows are now derived before
anything is cleared or written:
- `mint_pcurves` (`crates/topo/src/pcurves.rs`, ~:2472) runs
  `mint_rows(body, &faces, band)?` first, and only then calls
  `body.pcurves.clear()` / `body.joints.clear()` and `write_row`.
- `mint_pcurves_of` (~:2540) runs `mint_rows(..)?` first, then
  `drop_rows` and `write_row`.

A typed refusal therefore returns before the caller's cache is touched.

These pin it:
- `crates/topo/tests/pcurve_door_refusals.rs`
  `a_null_edges_half_has_no_carrier_and_the_mint_leaves_the_body_as_found`
  asserts that the rows are unchanged after a refused whole-body mint;
- `review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise` mints
  a minted fixture under a tear, with both entries, and requires the
  clone deep-unchanged unless the mint answered `Ok`. Hoisting the clear
  above `mint_rows` turns it red (PR 4033, `## Review fixes`, finding 1).

Closed by the TOPO orchestrator on reading the code; no new PR.

