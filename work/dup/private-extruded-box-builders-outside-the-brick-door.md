---
id: private-extruded-box-builders-outside-the-brick-door
kind: issue
title: Seven private builders still extrude a rectangle into a box, one per suite, now that the box door is topo's
status: review
branch: dup/private-box-builders
opened: 2026-09-19
pr: 2891
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

## Re-taken 2026-09-19 (branch `dup/private-box-builders`, merge base `5b4979ef2`): seven was a floor and the floor was 34

The row's own needle reproduces its seven exactly at the merge base,
and the row is right that it is a floor. Three further instruments were
run over **every tracked file, no path argument**:

- **the row's needle** — `Extrusion::Distance(z.1 - z.0)` and its
  `real(...)` spelling: 7 sites plus `docs/GUIDE.md` (7) and
  `docs/guide/fail-loud.md` (1). Reproduced.
  *Blind: a literal height, `Extrusion::Vector`, a differently-named
  local, a profile assembled away from the call.*
- **rectangle-literal**: every 4-pair coordinate list in every tracked
  `.rs`/`.md`, filtered to axis-aligned rectangles — 709 lists, **529**
  rectangles.
  *Blind: a rectangle spelled as four `p2(..)`/`Point2::new(..)` calls
  or a `.line_to` chain, which is how `benches` and the three
  `verbs_cylcyl*` slabs write theirs.*
- **function-scoped union (the one that settled it)**: for every `fn`
  in every tracked `.rs`, a body containing an extrusion atom AND, in
  it, any four CONSECUTIVE two-coordinate expressions forming an
  axis-aligned rectangle. 165 functions at the merge base, of which
  **66** return a `Body`.
  *Blind: it matches four rectangle corners inside a LARGER polygon, so
  letterform prisms and notched profiles are hits without being boxes;
  and it cannot see a box whose corners are not consecutive in the
  source, or a profile assembled in a loop.*
- **mutation** (a sweep instrument, method item 4): three plants in
  `topo::test_support::brick` — `z.1 + 0.001`, x extent halved, box
  translated `+10` in x. It found the one folded site that nothing
  asserts on.

Reading the 66 by hand gives **34 private builders that build an
axis-aligned box by extruding a rectangle**; the rest are holed,
bulged, arced or lofted and are not members. So the class is
**34, not 7** — and a further **100** sites in 59 files write the same
construction INLINE inside a test body, which is
`the-box-extrusion-written-inline-inside-test-bodies`.

### Two of the row's own readings were wrong

- **`s16_box_soundness` and `n3r1_prune` do not have the extrusion as
  their subject.** The row guessed they did. Their headers say
  otherwise — the census's instance-containment arm and the
  pruning-delta corpus — and the boxes are operands. Both folded.
- **The choice is not "fold to `brick` or leave alone".** There is a
  third door and it is the right one wherever the extrusion IS the
  subject: `sweep::test_support::prism` / `prism_at` is
  *rectangle profile → extrude z0..z1* through the shared door, so the
  private re-spelling goes and the construction stays. That is what
  `reporting_door_bit_digest::box_extrusion` took, and its committed
  per-eps digest did not move.

### Of the row's seven

| site | disposition |
| --- | --- |
| `benches/benches/kernel.rs` | **no.** `benches/Cargo.toml` depends on `pncad` alone, deliberately — *"the benchmarks are measurements of the PUBLIC surface"* — so a test-support door is not reachable and would not be right if it were |
| `crates/pncad/tests/all.rs` | **no.** The row is *"the end-to-end proof that the Boolean vocabulary is prelude-complete"*; building its operand any other way removes the proof |
| `crates/sweep/tests/n3r1_prune.rs` | **folded**, and so is `small_box` beside it, which the row's needle could not see |
| `crates/sweep/tests/s16_box_soundness.rs` | **folded**, same pair |
| `crates/sweep/tests/verbs_cylcyl_probe.rs` | **folded**; its header claimed every body was authored through the public extrude door, which is now true of the cylinders only, and says so |
| `crates/sweep/tests/verbs_cylcyl_r1_review_probes.rs` | **folded** |
| `crates/sweep/tests/verbs_cylcylb_r1_blinded_probes.rs` | **folded** |

### Three more said no

- `crates/sweep/tests/m3_pr5_extrude_booleans.rs` — its header is
  *"operands built through the REAL profile → `extrude` path — whose
  edges carry `Intersection { s1, s2 }` descriptions"*. The extrude
  path is every row's premise.
- `crates/sweep/tests/pcurve_p1b_r2_probes.rs` — its R2-S1 roster lists
  the body as `("extrude slab", …)`; the construction is what that row
  covers.
- `crates/step-import/tests/verbs_chamfer_roundtrip.rs` — the fixture
  says it is *"built through the public doors a consumer would use"*,
  and `test_support` is not one.

