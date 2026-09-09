---
id: the-role-name-builders-reach-only-the-outer-profile-loop
kind: issue
title: the role-name builders reach only the outer profile loop, so a hole's band is still hand-spelled
status: open
opened: 2026-09-09
refs: [no-facade-door-mints-a-revolves-role-names]
needs_ev: true
---


`band`, `band_pi`, `band_rim` and `meridian_vertex`
(`crates/editor-core/src/names/role.rs`, re-exported at
`pncad::select`) take a bare `u32` and fix `loop_index: 0` — the
signature Ev ruled, and the one every consumer that converted was
already spelling. A profile with HOLES has loops `1..n` and its bands
are named the same way, so a caller wanting one still writes the
`StableName` field by field:

- `crates/editor-core/tests/ring_r1_names_probe.rs:96` —
  `RoleSeg::Band(pe(1, s))`, an annulus revolved.
- `crates/editor-core/tests/m4_pr3_names.rs:425` — the same shape
  under a loop over `l`.

Both reach it through that file's own `name1`/`pe` helpers, so
neither is a five-field spelling today; what is missing is a door at
the vocabulary. Three shapes would close it and each costs something:
a second parameter on all four (`band(node, loop, seg)`, which reads
as two anonymous integers at every call site), a locator argument
(`band(node, ProfileEdgeRef { .. })`, which un-shortens the outer-loop
call the builders exist for), or `impl Into<ProfileEdgeRef>` with a
`From<u32>` that means "the outer loop" (one door, at the cost of an
implicit conversion that invents a field).

Not urgent: no consumer outside those two test files names a hole's
band, and the vocabulary itself stays reachable for the ones that do.

## Question for Ev (2026-09-09, LIB orchestrator; `[ev]` PR)

The four builders you ruled take a bare `u32` and fix `loop_index: 0`
(`crates/editor-core/src/names/role.rs:686`), and every shipped
consumer converts at that signature — LIB-NAMES converted thirteen
sites and LIB-PYNAMES bound the same five doors in Python. A profile
with holes names its inner loops' bands the same way, at
`loop_index: 1..n`, and the only callers that want one today are two
kernel test files that already spell the `StableName` through their
own helpers. Four shapes:

- **(A) A loop parameter on all four**: `band(node, loop, seg)` —
  two anonymous integers at every call site, and thirteen converted
  sites plus the five Python doors move again.
- **(B) A locator argument**: `band(node, ProfileEdgeRef { loop_index,
  segment })` — un-shortens the outer-loop call the builders exist
  for, at every site.
- **(C) `impl Into<ProfileEdgeRef>` with `From<u32>` meaning the outer
  loop** — one door, no site moves, at the cost of an implicit
  conversion that invents `loop_index: 0` where nothing says so; the
  Python doors would take `int | tuple[int, int]` to mirror it.
- **(D) Leave it and park the item** until a consumer outside a test
  file names a hole's band: the vocabulary stays reachable (the
  `StableName` struct spells it in Rust today; Python has no
  struct-spelling door and would gain the need with the consumer), no
  signature moves, nothing is invented. Recommended: it is the
  least machinery and the ruled signature was chosen for exactly the
  calls that exist. If a door is wanted now, (C) is the pick — the
  only one that keeps every existing call as it is.

### Why the builders fix `loop_index: 0`, added 2026-09-09 after Ev asked

Scope, not design. The `[ev]` question on 2231 offered the builders in
the shape the hand-spelling sites had — two arguments — because every
one of the thirteen sites spelled `loop_index: 0`: the tour, the
corpus and the tests all name the outer loop's faces, and the builders
were the common factor of the calls that existed. Ev ruled (A) at that
spelling and LIB-NAMES kept it rather than widen a ruled signature on
its own, filing this residue. Nothing about the outer loop earns the
shortcut on its merits: `ProfileEdgeRef` is `{ loop_index, segment }`,
loop 0 the outer loop and holes in description order, and a hole's
band is the same `RoleSeg::Band` with its loop's index. So the four
builders are "the outer-loop convenience"; (D) parks that until a
consumer wants the other loops, (C) keeps it while opening the rest,
and (A) is the symmetric signature at the cost of the thirteen sites
and the five Python doors moving to three arguments with `0` in the
middle at every current call.
