---
id: solid-of-face-has-eleven-hand-written-walks-outside-it
kind: issue
title: Body::solid_of_face has eleven hand-written face to shell to solid walks outside it
status: open
opened: 2026-09-19
---


## Finding

- **Where**: `crates/topo/src/shell10_r2_probes.rs` (`faces_of`, ~:39)
  and `crates/topo/src/offset_together.rs` (`faces_of`, ~:981) —
  **byte-identical helper bodies under one name in two files**;
  `crates/topo/src/props.rs` (~:3293), `crates/topo/src/seqgen.rs`
  (~:725, ~:733, ~:1594, ~:1602), `crates/topo/src/euler_ring.rs`
  (~:2755), `crates/topo/tests/bool4r1_probes.rs` (~:199),
  `crates/sweep/tests/revolve_ring.rs` (~:58),
  `crates/sweep/tests/shell5_r1_probes.rs` (~:533),
  `crates/sweep/tests/verbs_tubewall.rs` (~:165).
- **Importance**: medium
- **Confidence**: sure for the `topo/src` sites, which were read; the
  three `sweep/tests` sites are structural-regex hits not yet read.
- **Raised by**: PR #2857's fix pass, 2026-09-19.

`Body::solid_of_face(face) -> Option<SolidKey>` — face → its shell's
back-pointer → `Shell::solid` — carries the doc claim *"the one spelling
of face → shell → solid the census, the point-in-solid door and their
suites read."* That sentence is scoped and true of what it names. What
it does not say is that **eleven hand-written spellings of the same walk
sit outside it**, two of them byte-identical `faces_of` helpers in two
files of the same crate.

The shapes:

| shape | sites |
| --- | --- |
| `faces_of(body, solid)` — byte-identical helper | `shell10_r2_probes.rs` (~:39), `offset_together.rs` (~:981) |
| `solid_of_face` inlined as `get_face(k).and_then(get_shell).is_some_and(...)` | `props.rs` (~:3293) |
| `get_shell(face.shell).expect(...).solid`, the half-edge data already in hand | `seqgen.rs` ×4 |
| the walk split over two `let`s | `euler_ring.rs` (~:2755) |
| test-lane `unwrap` chains | `bool4r1_probes.rs`, three `sweep/tests` files |

## The instrument — sibling-door re-census

**Take the door a change cites as its PRECEDENT, and re-census that
door's own walk.** PR #2857 sited `Body::face_of_half_edge` on
`solid_of_face` as the model for "the one spelling of this walk". A door
held up as a model is a door somebody once folded onto — and the fold is
as old as its claim, so the claim is exactly as stale as the tree has
moved since. Running the census on the model rather than on the new door
found the same defect one door over, at eleven sites.

**Blind spots.** It needs a cited precedent, so it says nothing about a
door with no model, and it inherits whatever blind spot the census used
on the precedent has. The structural regex used here
(`get_shell(... .shell ...) ... .solid`) has a bounded-nesting limit and
cannot see a walk split across a helper boundary, so eleven is a floor.
`git grep -n '\.shell'` read by hand is what confirmed the `topo/src`
nine.
