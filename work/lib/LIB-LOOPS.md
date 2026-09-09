---
id: LIB-LOOPS
kind: unit
title: the role-name builders take the loop index: the symmetric signature on both sides, with a hole scene pinned
status: review
branch: lib/loops
opened: 2026-09-09
refs: [the-role-name-builders-reach-only-the-outer-profile-loop]
pr: 2268
---


Closes `the-role-name-builders-reach-only-the-outer-profile-loop`
under its `## Ruled` (A): the loop index is an argument like the
segment, on all four builders and on both sides of the Python
boundary.

## Delivered

- **The four signatures**, `crates/editor-core/src/names/role.rs`:
  `band(node, loop_index, seg)`, `band_pi(node, loop_index, seg)`,
  `band_rim(node, loop_index, vertex)`,
  `meridian_vertex(end, node, loop_index, vertex)`. `carried` is
  unchanged.
- **The parameter is spelled `loop_index`, not `loop`**, in Rust and
  in Python alike. `loop` is a Rust keyword, so the kernel builder
  would have had to take `r#loop` while the stub declared `loop` —
  two spellings for one argument, and a keyword call that works on
  one side of the boundary and not the other. `loop_index` is what
  `ProfileEdgeRef`/`ProfileVertexRef` already call the field the
  argument becomes.
- **The rustdoc stops privileging a loop.** Each builder names the
  loop it is given; the shared paragraph on `band` says once that the
  index is `ProfileEdgeRef::loop_index` and spells what that field
  spells — 0 the outer loop, then holes in description order — and
  that `seg`/`vertex` index THAT loop's chain. The Python doors, the
  stub and the guide say the same in their own words.
- **Every Rust call site moved**, all passing `0`: 38 calls of the
  band family across `demos/tour/src/teapot.rs`,
  `demos/tour/tests/teapot_document.rs`,
  `crates/editor-core/tests/corpus/vessel.rs`,
  `lib_g17_shell_node.rs`, `lib_g17_r1_probes.rs`,
  `lib_g17_r2_probes.rs`, `blend5_r1_probes.rs`,
  `blend5_r2_probes.rs`, `blend5_rim_support.rs`,
  `seat6_param_source.rs`, plus the one `meridian_vertex` call in the
  tour.
- **The two hand-spelling sites converted at their real loop**:
  `ring_r1_names_probe.rs` (the hole at loop 1, and the wire outer's
  π-bands at loop 0) and `m4_pr3_names.rs`'s holed-ring block (the
  loop variable). `ring_r1`'s `pv` helper had no other caller and is
  gone with its `ProfileVertexRef` import; `name1`/`pe` stay in both
  files because `RoleSeg::Meridian` has no builder and those files
  spell it.
- **The `role.rs` pins are doubled**, each builder at loop 1 and again
  at loop 0: a builder that dropped its new argument and kept the
  outer loop satisfies a pin written only at 0, so the pin written
  only at 0 is not the assertion this change needs.
- **The five Python doors and the stub moved to the same arity**
  (`crates/pncad-py/src/py/select.rs`, `pncad.pyi`), and
  `test_role_names.py` gained a THIRD SCENE: the ring's square section
  with a square hole, revolved a full turn, where the emitter mints
  two loops' worth of bands, rims and seam meridian vertices and the
  doors answer those bytes at `loop_index` 1. Compared as sets over
  each loop's four segments — which corner a loop's canonical chain
  starts at is `crates/profile`'s business, not this file's — plus the
  claim that makes the loop argument load-bearing:
  `band(node, 0, 0) != band(node, 1, 0)`.
- **ty fixtures**: the legal rows move to the new arity and gain a
  hole's band at loop 1; the illegal side gains two rows — a length
  where the loop belongs, and the two-argument call the old signature
  took.
- **The guide step** (GUIDE.md §2, "Naming a role before the body
  exists") moves with them and says what `loop_index` is; the ring it
  builds has one loop, so it passes 0 and says so.

## Not this unit

`carried`'s signature; any new builder; the Python doors' seat.
`m4_pr3_names.rs`'s OTHER blocks keep their hand spelling: they probe
the emitter's whole `RoleSeg` vocabulary (`Lateral`, `RimEdge`,
`CapVertex`, `Pole`, `AxisEdge`, `Meridian`, …), most of which has no
builder, and the item named only the holed block.
