---
id: private-extruded-box-builders-outside-the-brick-door
kind: issue
title: Seven private builders still extrude a rectangle into a box, one per suite, now that the box door is topo's
status: open
opened: 2026-09-19
---


## Finding

- **Where**: seven private box builders, each one suite's own, listed
  below.
- **Importance**: low-medium — they are the box class this program is
  already closing, one layer out
- **Confidence**: sure about the seven; the count is a floor, for the
  reason under *What the instrument could not see*
- **Raised by**: the `dup/sweep-brick-delegation` lane, 2026-09-19,
  by the structural needle it ran over its own diff — the deleted
  construction's shape, not its name

`sweep::test_support::brick` is now `topo::test_support::brick`, so
the tree's box door no longer extrudes anything. These seven still do,
privately, each re-spelling *rectangle profile → extrude z0..z1*:

| site | shape |
| --- | --- |
| `benches/benches/kernel.rs:146` | `fn slab(x, y, z)` |
| `crates/pncad/tests/all.rs:1415` | inline, in a helper |
| `crates/sweep/tests/n3r1_prune.rs:73` | private helper |
| `crates/sweep/tests/s16_box_soundness.rs:425` | private helper |
| `crates/sweep/tests/verbs_cylcyl_probe.rs:373` | private helper |
| `crates/sweep/tests/verbs_cylcyl_r1_review_probes.rs:491` | private helper |
| `crates/sweep/tests/verbs_cylcylb_r1_blinded_probes.rs:68` | private helper |

**Not all of them should fold, and that is the row's question.** A
suite whose subject IS the extrusion — `s16_box_soundness` and
`n3r1_prune` read like two — wants an extruded box on purpose and
would lose its subject by taking `brick`'s Euler-built one. A suite
that only needs *a box to cut with* should name the door. Whoever
takes this decides per site and says which, rather than folding all
seven; `implementer-discipline.md` §5's hit-list rule applies.

`docs/GUIDE.md` (7 lines) and `docs/guide/fail-loud.md` (1) carry the
same shape and are **not** members: the guide teaches the public
extrusion API and showing it spelled out is the point.

## What the instrument could not see

The needle was the literal `Extrusion::Distance((real()?)z.1 - z.0)`.
It cannot see a box extruded by a literal height, by
`Extrusion::Vector`, by a height bound to a differently-named local,
or built from a rectangle assembled anywhere but at the call. The
denominator it ran against is **303** textual `extrude(` in
`crates/sweep/tests` alone, so seven is a floor and the class is very
likely larger — method item 8 says that is an instruction to run a
second instrument, not a licence to publish the count, and this row
publishes a floor precisely so the next lane knows it is one.

## Why it sits here and not on the territory owner's slate

`scripts/work.py territory` puts five of the seven on S-TCOST's and
S-TINT's ground, one on S-PERF's (`benches/`) and one on S-LIB's
(`crates/pncad/tests/`). The finding is a **duplication** finding —
one box spelled seven private ways — which is S-DUP's charter and not
any of theirs, and S-DUP claims no territory by design (`plan.md`,
*"this program claims nothing and announces by seam"*). Filed here as
one row rather than four, because the four would be one sentence each
of the same finding; a program that wants its share claims it by
`git mv` per `work/README.md`.
