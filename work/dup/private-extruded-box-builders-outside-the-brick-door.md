---
id: private-extruded-box-builders-outside-the-brick-door
kind: issue
title: Seven private builders still extrude a rectangle into a box, one per suite, now that the box door is topo's
status: closed
branch: dup/private-box-builders
opened: 2026-09-19
pr: 2891
closed: 2026-09-19
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

## Re-taken 2026-09-19 (branch `dup/private-box-builders`, merge base `5b4979ef2`): seven was a floor and the class is 44

The row's own needle reproduces its seven exactly at the merge base,
and the row is right that it is a floor. Four instruments were run over
**every tracked file, no path argument**:

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
  axis-aligned rectangle. **166** functions across 105 files, of which
  **66** return a `Body`.
  *Blind: it matches four rectangle corners inside a LARGER polygon, so
  letterform prisms and notched profiles are hits without being boxes;
  and it cannot see a box whose corners are not consecutive in the
  source, a profile assembled in a loop, a closure, or an inline
  construction in a `main`.*
- **mutation** (a sweep instrument, method item 4): four plants in
  `topo::test_support::brick` and one in `sweep::test_support::extruded`.
  It found the one folded site nothing asserts on.

Reading the 66 by hand gives **41** members; the instrument's four
structural blind spots account for **3** more, found by the needle and
by reading the two example binaries — `benches/benches/kernel.rs`'s
`slab` (its rectangle comes from a local `rect()` helper), the `slab`
CLOSURE in `crates/pncad/tests/all.rs`, and the blank built inline in
`crates/sweep/examples/p1b_r2_ab_interval.rs`'s `main`.

**44 members. 33 folded, 11 not, and the two sum.** A further **100**
functions (166 − 66) write the same construction inline inside a test
body; that is
`the-box-extrusion-written-inline-inside-test-bodies`, censused at the
merge base because the fold blinds this instrument over its own area.

### Two of the row's own readings were wrong

- **`s16_box_soundness` and `n3r1_prune` do not have the extrusion as
  their subject.** The row guessed they did. Their headers say
  otherwise — the census's instance-containment arm and the
  pruning-delta corpus — and the boxes are operands. Both folded.
- **The choice is not "fold to `brick` or leave alone".** There is a
  third door and it is the right one wherever the extrusion IS the
  subject: `sweep::test_support::prism` / `prism_at` is
  *rectangle profile -> extrude z0..z1* through the shared door, so the
  private re-spelling goes and the construction stays. That is what
  `reporting_door_bit_digest::box_extrusion` took, and its committed
  per-eps digest did not move.

### The 33 folded

`crates/sweep/tests/` (31): `census_containment_cause::boxx`,
`common/approx::unit_box`, `curved_mergedoor::plate6`,
`m5_pr9_boss_union::plate`, `m5_s10_face_sense::pellet`,
`m5_s11_concave_sense_interval::pellet`,
`m5_s12_curved_ops_interval::plate`, `m5_s13_pips::slab`,
`m5_s13_pips_interval::slab`, `m9_3_wall_door::plate`,
`m9_3_zip::plate`, `n3r1_prune::small_box`, `n3r1_prune::plate`,
`r1_probes_m9_3::plate6`, `reporting_door_bit_digest::box_extrusion`
(to `prism`), `review_arceval_r1_probes::block`,
`review_arceval_r1_probes`'s E2 plate (to the shared `m5_s12` fixture),
`review_m6_5_pr2_sweep_probes::box_at`, `s16_box_soundness::small_box`,
`s16_box_soundness::plate`, `verbs_1031b_arcwind::cutter`,
`verbs_cylcyl_probe::slab`, `verbs_cylcyl_r1_review_probes::slab`,
`verbs_cylcylb_r1_blinded_probes::slab`,
`verbs_f7_r2_probes::brick_operand`, `verbs_ga_r2_probes::boxx`,
`verbs_germarms::boxx`, `verbs_germarms_interval::bar`,
`verbs_germarms_r1_probes::boxx`, `verbs_pierce::boxx`,
`verbs_pierce_r2_probes::boxx`.
`crates/mesh/tests/` (1): `r2_bool_door::slab`.
`crates/editor-core/src/` (1): `resolve/pick.rs`'s `unit_prism`, in
that file's `#[cfg(test)] mod tests`.

### The 11 not folded, each with its reason

| site | why not |
| --- | --- |
| `benches/benches/kernel.rs::slab` | `benches/Cargo.toml` depends on `pncad` alone, deliberately: *"the benchmarks are measurements of the PUBLIC surface"*. The door is unreachable and would be wrong if it were |
| `crates/pncad/tests/all.rs`'s `slab` closure | its row is *"the end-to-end proof that the Boolean vocabulary is prelude-complete"* |
| `crates/mesh/tests/r1_probe_bool_route.rs::slab` | returns `Result<Body, ExtrudeError>`; the refusal is the subject and `brick` panics |
| `crates/step-import/tests/verbs_chamfer_roundtrip.rs::chamfered_cube` | the fixture states it is *"built through the public doors a consumer would use"* |
| `crates/sweep/examples/p1b_r2_ab.rs::cube` | frozen reviewer evidence: it prints and asserts nothing, so re-authoring its blank would silently stop it reproducing the numbers it was cited for |
| `crates/sweep/examples/p1b_r2_ab_interval.rs`'s inline blank | the same binary's interval twin, same reason |
| `crates/sweep/tests/m3_pr5_extrude_booleans.rs::slab` | the header is *"operands built through the REAL profile -> `extrude` path"*; that path is every row's premise |
| `crates/sweep/tests/pcurve_p1b_r2_probes.rs::slab` | its R2-S1 roster lists the body as `("extrude slab", ...)`; the construction is what the roster covers |
| `demos/tour/src/bool_bodies.rs::slab` | the demos render through the public API from an outside consumer's seat (`implementer-discipline` §3) |
| `demos/tour/src/bossplate.rs::plate` | same |
| `demos/tour/tests/verbs_teapot.rs::boxy` | same |

`docs/GUIDE.md` and `docs/guide/fail-loud.md` are not members, as the
row says. `demos/tour/src/bodies.rs::plate` is not one either: it is a
rectangle with two circular holes.

### The construction swap does not move a certification enclosure — measured

`m5_s12_curved_ops_interval`'s `interval_sphere_subtract_...` asserts
`hi == RECUT_MAPPED_ENCLOSURE_HI` bit-exactly, and its left operand is
the folded `plate()`. Run under `CAD_TOLERANCE_EPS=1e-12` with
`--features interval`:

| plate | `hi` |
| --- | --- |
| merge base, extrude-built | `1.1362773333939659e-12` |
| head, `block(3.0, 3.0, 0.8)` | `1.1362773333939659e-12` |
| **geometry control**, `block(4.0, 4.0, 0.8)` | `1.1361065349779188e-12` |
| **arena control**, the same box with its corner list rotated one place | `1.1362773333939659e-12` |

Two controls, and the second does **less** than an earlier draft of
this row claimed. Read the next paragraph before citing it.

- The **geometry** control shows `hi` is not degenerate in the plate:
  change the box and the number moves.
- The **corner-rotation** control perturbs the plate's construction
  observably — under that rotation
  `crates/topo/tests/cube_doors_agree.rs`'s
  `every_box_door_builds_one_body` and
  `every_door_builds_the_prism_its_inputs_name` both go **red** — and
  `hi` does not move.

### The corner rotation does NOT discriminate, and an earlier draft said it did

This row claimed the rotation was *"the discriminating control, because
the two constructions differ in curve-arena ordering and in the
`(s1, s2)` sense… starting the corner list at a different vertex builds
the same box with a permuted arena"*. **It reproduces neither axis.**
Dumped 2026-09-19 for the unit box — per edge, the curve's ordinal in
the curve arena's own order, and whether
`s1 == face_surface_of_he(he_plus)`:

| body | senses | edge-order → curve-order |
| --- | --- | --- |
| extrude door | `MMMMPPPPPPPP` | `[7, 8, 9, 10, 11, 4, 5, 6, 0, 1, 2, 3]` |
| extrude door, corners rotated | `MMMMPPPPPPPP` | `[7, 8, 9, 10, 11, 4, 5, 6, 0, 1, 2, 3]` |
| Euler door (`brick`) | `PPPPPPPPPPPP` | `[11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]` |
| Euler door, corners rotated | `PPPPPPPPPPPP` | `[11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]` |

The two doors differ on both axes, reproducing the 2026-09-16
measurement. **Rotating the corner list changes neither, for either
door.** Whatever `cube_doors_agree` sees under the rotation is at the
vertex/point level, not on either axis that separates the
constructions.

The sense half cannot be reached that way even in principle:
`topo::test_support::describe_as_intersections` sets
`s1 = face_surface_of_he(body, edge.he_plus)` **unconditionally**, so
any body it describes is `P` on every edge whatever the corner order.
That is a proof about the code, not a measurement.

**So what is established, exactly.** `hi` is not degenerate in the
plate (geometry control), and the two real constructions feed it
bit-identically (head against merge base). The constructions' own
difference is covered by that one row, which varies arena ordering and
`(s1, s2)` sense together; **no control isolates either axis**, and the
corner rotation is not one. A future lane wanting per-axis evidence has
to build a body that differs on one axis alone, which neither door
offers today.

### A decline that was wrong, corrected 2026-09-19

This row said `crates/editor-core/src/resolve/pick.rs::unit_prism` was
declined because *"it is in `src/`, so the door needs a LIBRARY edge —
the route `scripts/gates/test-features-dev-only.sh` refuses"*. **Both
halves are false** and the site is now folded:

- `unit_prism` is inside that file's `#[cfg(test)] mod tests` (opens
  `:2331`), so it is not library code and needs no library edge.
- `crates/editor-core/Cargo.toml`'s `[dev-dependencies]` already reads
  `sweep = { path = "../sweep", features = ["test-support"] }` (`:169`),
  which is the spelling the gate PERMITS — it refuses `features` on a
  `[dependencies]` line, which this is not. The same manifest uses that
  permitted spelling for `profile` too, and the module in question
  already said `use sweep::{Extrusion, extrude};`.

So the door was nameable there with no manifest change at all. Folded
to `sweep::test_support::cube(1.0, Tol::witness())`;
`cargo test -p editor-core --lib` is 152 passed, 0 failed.

The reason was written for a member surfaced late, and it was not
checked against that member's manifest. A wrong decline is invisible in
a green diff, which is why it is recorded here rather than quietly
fixed.

