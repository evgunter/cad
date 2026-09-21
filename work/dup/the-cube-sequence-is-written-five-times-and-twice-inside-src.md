---
id: the-cube-sequence-is-written-five-times-and-twice-inside-src
kind: issue
title: The §9.4.2 cube sequence is written five times in topo; after link 3 every copy is in src/ and they fold onto each other
status: closed
opened: 2026-09-18
refs: [brick-has-two-constructions-and-two-homes]
branch: dup/fold-the-cube-sequence
pr: 2843
closed: 2026-09-19
---


## Finding

- **Where**: `crates/topo/src/fixtures.rs` (`ops_cube`, `:765`),
  `crates/topo/src/review_m1_pr3.rs` (`build_box`),
  `crates/topo/src/cert_m3r1_probes.rs` (its verbatim copy of
  `geometric_cube`, folded by link 3),
  `crates/topo/src/splitting/reassembly.rs` (`quad_prism`),
  `crates/topo/src/test_support_fixtures.rs`
  (`geometric_cube`/`cube_into`, and `prism_ops` which subsumes them;
  was `crates/topo/tests/common/mod.rs` before link 3), against each
  other.
- **Importance**: medium
- **Confidence**: sure for the `ops_cube`/`build_box`/`geometric_cube`
  triple, which was measured by dumping; the fifth copy
  (`cert_m3r1_probes.rs`) is carried from an earlier row and was NOT
  dumped here
- **Raised by**: the S-DUP orchestrator, 2026-09-18, out of the link-3
  home measurement on
  `work/dup/brick-has-two-constructions-and-two-homes.md` — the lane
  measured it while settling whether `review_m1_pr3::build_box` was a
  member of the family, and disclosed it in that row's prose. Prose is
  not scheduling (`work/README.md`), so it gets a file.

## The measurement

**`fixtures::ops_cube` is `geometric_cube` with the face geometry
declined.** The two derived `Body` dumps are byte-identical in
`points`, **`curves` (all 1199 lines)**, `half_edges`, `loops`,
`edges`, `vertices`, all seven provenance maps, `curve_origins` and
`surgery`. They differ in exactly three sections — `faces` (which
surface key each face carries), `surfaces` (one shared NURBS
placeholder against six Newell planes) and `surface_origins`. Reading
confirms the dumps: operator for operator, including the
`f_bottom.he_plus` strut anchor and the `f_front.he_plus` close.

**`review_m1_pr3::build_box` is `ops_cube` at a uniform 2× scale.**
Every section of both dumps is identical except `points` (`1.0` →
`2.0`) and the `curves` that carry those coordinates.

So the §9.4.2 sequence stands written in `topo` at least five times:
`prism_ops` (the shared builder link 1b landed), `cube_ops` (folded
into it by link 1b), `cert_m3r1_probes.rs`'s copy, `fixtures::ops_cube`
and `review_m1_pr3::build_box`. **Two of those five are in `src/`, are
the same function as each other, and are the same function as the
`tests/` builder up to one axis (face geometry) and one scale.**

`crates/topo/src/review_m1_pr2/cube_independent.rs` is a sixth
spelling and is **exempt**, not a defect: its header states *"independent
derivations — do not 'simplify' them to match shipped fixtures …
Promoted per Ev's request (PR #17 thread)"*, and it genuinely differs in
addressing (seeds at the top, closes the TOP face first, struts
downward, the seed face ends as the bottom). Anyone working this row
reads that header before touching it. `review_m1_pr2/atomicity.rs`
builds a digon pillow and is not a member either — the census flagged it
on `mvfs`/`mev_line` call sites, which is the predicted
undercount-by-shape running in the other direction.

## What this row is NOT

It is **not** "delete four of the five". Two of the three measured
copies decline something on purpose, and what they decline is the point
of the suite they serve:

- `ops_cube` declines the face surfaces because its consumers are
  operator-count and atomicity tests that must not depend on geometry.
- `build_box`'s 2× scale may be load-bearing for whatever
  `review_m1_pr3` asserts, or may be arbitrary — **unmeasured**.

The honest shape is a builder with the declined axes as parameters,
which is what `prism_ops` already is. Whether the `src/` copies can
reach it is a **consequence of link 3**, not independent of it: today
nothing in `src/` can name `tests/common`, which is the same namability
wall that holds `cert_m3r1_probes.rs`'s copy in place.

## Sequencing

**Unparked 2026-09-18**: link 3 landed and
`brick-has-two-constructions-and-two-homes` is closed. The family is at
`topo::test_support` (`crates/topo/src/test_support_fixtures.rs`), so
every in-`src` copy can name the shared builder and they fold in one
unit rather than behind three separate walls. The re-census at the
bottom of this file is what that unit starts from.

## What is unmeasured

- `cert_m3r1_probes.rs`'s copy was not dumped against the other four;
  its membership is carried from
  `work/dup/topo-src-cert-m3r1-probes-holds-an-in-src-copy-of-the-cube-family.md`,
  which established it by reading.
- Whether `build_box`'s 2× scale is read by any assertion in
  `review_m1_pr3`.
- `f64` only, default features, one eps row.
- No consumer of `ops_cube` was checked for a dependency on the
  placeholder surfaces *being* placeholders (as opposed to merely not
  being read).

## Re-census at link 3's merge base (2026-09-18, `dup/move-the-fixture-family`)

Link 3 has landed the family in `crates/topo/src/test_support_fixtures.rs`,
so the wall this row parks behind is down. Two things changed under it.

**One of the five is gone.** `cert_m3r1_probes.rs`'s copy was folded
onto the shared family in the same unit — it now calls
`test_support_fixtures::{geometric_cube, describe_as_intersections,
face_surface_of_he}` and builds nothing box-shaped.

**One that was never on the list is, and it cancels the other's
subtraction. The count is five:** `prism_ops`,
`splitting::reassembly::quad_prism`, `fixtures::ops_cube`,
`review_m1_pr3::build_box`, and `cube_independent.rs`'s exempt
independent derivation.

**And all five are now under `crates/topo/src/`** — the shared builder
included, since link 3 moved it there. The id and the original title's
*"twice inside src"* are the count at the moment this row was opened;
the title is corrected, the id is not (`work/README.md`: ids are
stable). What that changes for the unit is that the fold is entirely
`src`-local: no crate boundary, no feature edge, no `tests/` consumer
to re-point.

**The census, re-run over every tracked file with no path argument**, as
`plan.md`'s method item 3 asks. The pattern was the SHAPE, not a name: a
file holding a `.mvfs(` call with at least 5 `mef`/`mef_chord` and at
least 7 `mev`/`mev_line` call sites — the cube sequence's own arity.
Twenty files matched; sixteen are dispositions this row or the brick row
already recorded, or are not boxes. One is new:

| hit | disposition |
| --- | --- |
| `crates/mesh/tests/r2_mesh6_probes.rs` | **NEW candidate**: an inline 1 `mvfs` + 7 `mev_line` + 5 `mef` unit cube with a scaffold strut planted on it, in a crate above `topo` that can now name `topo::test_support::geometric_cube`. Not folded — the strut is the subject and the fold needs a `features = ["test-support"]` on `mesh`'s dev edge. Belongs in this row's unit |
| `crates/mesh/tests/common/witness_bodies.rs` | not a box: a cylinder-walled body over an L profile |
| `crates/topo/src/iso.rs`, `crates/topo/src/merge_faces.rs` | not boxes: digon pillows |
| `crates/topo/src/review_d18.rs`, `crates/topo/src/boolean/boxes.rs` | not boxes: a mint/kill slot recycler and a conic sector |
| `crates/topo/src/lib.rs` | the crate doc example, deliberately hand-written through the public door |
| `euler.rs`, `euler_kill.rs`, `euler_ring.rs`, `review_m1_pr4.rs`, `review_m1_pr2/*`, `review_m1_pr3.rs`, `validate.rs`, `fixtures.rs`, and four `topo/tests/` suites | already dispositioned by this row or the brick row |

**What the pattern could not match.** It counts CALL SITES, so it
undercounts every builder that loops — which is exactly how it misses
`prism_ops` itself (3 `mev` sites, 2 `mef` sites, N-general). A future
copy written as a loop is invisible to it, and no name sweep replaces
that: a loop-written copy under a new name is found only by execution.
It also cannot see a copy assembled through the raw builder rather than
the operators, which is how `fixtures.rs`'s raw family would look.

## `quad_prism`, and why both of link 3's censuses missed it
### (added 2026-09-19 by the PR 2842 fix pass, out of the review)

`crates/topo/src/splitting/reassembly.rs`'s `quad_prism` is a copy of
`prism_ops` in `src/`, and it said so itself: its doc read *"the
tests/common builder's minimal in-crate copy"* at
`d928c332a:crates/topo/src/splitting/reassembly.rs:42`. It is
`#[cfg(test)] pub(crate) mod reassembly` (`splitting/mod.rs`), so the
namability wall this row is about never held it: `test_support_fixtures`
is nameable from it today, and `tests/common` was not nameable from it
before link 3 either — the copy predates and outlives that wall.

**Established by READING both functions side by side**, not by dumping.
Operator for operator it is `prism_ops` specialised to `f64` and
`n = 4`: the same `mvfs(bot[0])` seed, the same `MevSite::Lone` first
rim edge then `MevSite::Fan { he1: at, he2: at }` chain over `2..n`,
the same `find_half_edge(seed.face, …)` / reversed-profile `rev` bottom
close through `MefSite::Chords`, the same strut loop, the same
`(i + 1) % n` side loop with `first_side_he_plus` closing the last
wall, and the same closing `set_face_surface` on the seed face. It
differs in exactly three ways: it is monomorphic at `f64`, it returns
the `Body` rather than a key bundle, and it takes `height` where
`prism_ops` takes `z: (f64, f64)`. The description step is absent from
both — in `prism_ops` that is deliberate and documented as the
caller's.

**It has already drifted**, which is the cost this row exists to
name. Its strut loop carries a first arm `if i == 0 { chain[0].he_plus }`
that `prism_ops` folds into `i < n - 1` (`chain[0].he_plus` is what the
general arm yields at `i = 0`). Dead in the sense that removing it
changes nothing, live in the sense that a reader now has two different
pictures of the same anchor rule.

**Call sites**: 19 in `topo/src`, across `boolean/ops.rs` (6),
`props.rs` (3), `boolean/solid_contain.rs`, `census.rs`,
`offset_together.rs`, `shell10_r2_probes.rs` (2 each),
`splitting/reassembly.rs` and `surgery.rs` (1 each). Nothing in
`tests/` can reach it (`review_m3_pr3_consumer.rs`'s `add_quad_prism`
is a different, locally-defined function). So the fold is `src`-local
and the blast radius is those 19 lines.

### Why the two instruments missed it, and which third one finds it

- **The shape census counts CALL SITES.** `quad_prism` is loop-written,
  so it scores 1 `mvfs` / 3 `mev` / 2 `mef` and falls under the
  ≥7 `mev` / ≥5 `mef` threshold. This row's own "what the pattern could
  not match" paragraph **names that blind spot exactly** — *"it
  undercounts every builder that loops — which is exactly how it misses
  `prism_ops` itself"* — and then does not compensate for it. A
  disclosed blind spot that nothing is run through is not a negative
  result; the sentence should have been followed by a second pattern,
  and this row is the evidence for why.
- **The name census keys on family-member names.** `quad_prism` is a
  new name for a family member, which is the one thing a name census
  structurally cannot see.
- **A census shaped on the GEOMETRY finds it in one line.** Files
  holding both `mvfs(` and `newell_plane` — the two ends of "Euler-op
  construction with certified planar faces", neither of them an arity —
  return ten files across the tree, and `reassembly.rs` sits directly
  beside `test_support_fixtures.rs` in the list. Verified 2026-09-19:
  `sweep/src/{extrude,loft,revolve/partial}.rs`,
  `topo/src/{euler,splitting/reassembly,test_support_fixtures,validate}.rs`,
  `topo/tests/{m3_pr3_split,review_m2_pr3,review_m3_pr3_rings}.rs`.
  `git grep 'in-crate copy'` also returned it, as a single hit, at
  link 3's merge base — a self-declared copy is findable by its own
  prose, and no census in either link was run over prose.

### What landed in PR 2842, and what did not

**Not folded.** The fold is this row's unit and is new scope for a PR
about moving the family. What landed is the disclosure: `quad_prism`'s
doc now says it is a copy of `test_support_fixtures::prism_ops` and
points here, and `cert_m3r1_probes.rs`'s header no longer claims the
moved family is *"the one Euler-op fixture family"*.

## What the fold measured (2026-09-19, `dup/fold-the-cube-sequence`)

**The placeholder surfaces ARE depended on as placeholders.** The row
recorded this as unmeasured. Instrument: hand `fixtures::ops_cube` real
Newell planes (one character at the fold's call site) and count what
reddens. **Ten rows**, all in `topo`'s lib suite — nine in
`merge_faces::tests`
(`the_placeholder_cube_forms_no_group_and_its_faces_are_named`,
`a_placeholder_run_has_no_regime_and_is_set_aside`,
`a_source_stamp_joining_a_placeholder_to_a_plane_refuses_typed`,
`an_ok_carries_a_recorded_skip_beside_the_placeholder_census`,
`a_described_face_beside_placeholders_is_untouched_and_they_are_named`,
`every_contradicted_fact_escapes_the_recording_regime`,
`every_contradicted_fact_refuses_the_refusing_regime`,
`the_door_records_same_face_as_a_skip`,
`the_planar_fixtures_take_the_two_regimes`) and one in `revert::tests`
(`revert_flips_sense_on_non_plane_faces_instead_of_refusing`). So the
weaker reading — "not read" — is false, and the declined axis is a
parameter, not a default to be folded away.

**`build_box`'s 2x scale is NOT read by any assertion.** Instrument:
normalise it to the unit square and run the whole `topo` suite. 1291
of 1291 pass. It is still kept, for a reason that is not an assertion:
the three hole recipes its callers plant sit at x, y in (0.5, 1.5) and
z up to 1.5, which is inside a 2x2x2 box and outside a unit cube.
These are tier-1/2 suites, so nothing would go red — the fixture would
just become geometrically incoherent silently. The scale is therefore
`prism_ops`'s extent argument at the call site, stated, rather than a
`2.0` multiplier hidden in a map.

**The fold preserves every body key-for-key.** `fixtures::deep_snapshot`
(all ten arenas in slot order, full payloads, D5 provenance) over
`ops_cube`, `ops_holed_box`, `ops_genus2`, `build_box` and
`quad_prism`, before and after: byte-identical, all five. The `mesh`
probe's printed output is identical down to `FaceKey(3v1)`.

**Two more members the row's censuses missed**:
`crates/topo/tests/review_m3_pr1.rs`'s `ops_cube_public` (the declined
cube, twenty-eight operator calls) and
`crates/topo/tests/interval_body.rs`'s
`interval_cube_builds_and_validates_at_both_tiers` (the same at
`T = Interval`, in a file whose OTHER rows already take
`common::geometric_cube::<Interval>`). Both folded. The same
re-measurement shows `crates/topo/tests/cube_by_hand.rs` and
`review_m1_pr2/cube_independent.rs` score 1 `mvfs` / 4 `mev` / 2 `mef`
— **neither is matched by the arity census at all**, so this row's
"`review_m1_pr2/*` already dispositioned" line was matching that
directory's other two files, not the exempt one.

### How `review_m3_pr1.rs` was actually lost — a bucket, not a threshold
### (corrected 2026-09-19 by the PR 2843 fix pass)

The paragraph above first said both were *"loop- or closure-written and
so under the arity census's threshold"*. **That is false of
`review_m3_pr1.rs`, and the instrument says so.** Re-run at this row's
merge base (`63d6c9ea8`), it scores **20 `mev` / 14 `mef`** — nearly
three times the ≥7 / ≥5 threshold. It was **matched**, and then
bucketed away under this row's unnamed *"four `topo/tests/` suites
already dispositioned"* line.

**A bucket disposition is where a census loses things.** The four files
that line covered, re-derived at the same merge base, are
`m3_pr1_surgery.rs` (15/12), `review_m1_pr5.rs` (25/12),
`review_m2_pr3.rs` (10/14) and `review_m3_pr1.rs` (20/14). **Two of the
four were mis-dispositioned**, and PR 2843 found one of them and
re-buried the other: `m3_pr1_surgery.rs`'s `cube_with_inner_box` grows
its outer cube from a token-for-token copy of the same sequence, and its
own comment at the site called it *"the ops cube from the crate
example"*. Folded in the fix pass. The other two are correct
dispositions and are named here so the bucket does not have to be
re-opened a third time: `review_m1_pr5.rs` builds digon pillows and
pillow tori, and `review_m2_pr3.rs`'s `triangle_prism` carries
`PlacedSegment` rims and `ExtrudedPoint` struts rather than chord lines
— a different body, and the carriers are its subject.

The lesson this row now carries is not about thresholds. An arity
census under-counts loops, which this row already said; what it did NOT
say is that **a hit dismissed in a group of four is dismissed without
evidence**, and the group is where the count that survives to the next
lane is written. One line per hit, named, is the cost of not paying
this twice.

## The fix pass (2026-09-19, PR 2843)

**`fixtures::ops_cube` and `OpsCube` are deleted, not disclosed.** The
fold had left them as a name for `declined_cube::<f64>` and a name for
`GeoCube<f64>`, disclosed in the PR body as a minted instance of this
very class and filed nowhere. They are gone: 74 call sites across 20
files now name `test_support_fixtures::declined_cube` directly, and the
two `OpsCube` destructuring sites name `CubeOps`. The argument that
settles it is that `fixtures.rs`'s header says its bodies are built
through the raw builder with index-derived placeholders while
`test_support_fixtures.rs` says a body from one is not a substitute for
a body from the other — and `fixtures::ops_cube` had become a body from
the other. Both module headers are reconciled to what is now true.

**Two more members, both found by an instrument none of the four the PR
ran could see**: `git grep -n 'find_half_edge(seed.face'`, a STRUCTURAL
needle rather than an arity, a name, a geometry import or a
self-disclosure. 26 hits, dispositioned one line each in PR 2843's body.

- `crates/topo/tests/m3_pr1_surgery.rs`'s `cube_with_inner_box` — folded
  (above).
- `crates/topo/src/boolean/ops.rs`'s `far_cube` — the deleted
  `ops_cube` body character for character under an x-shift, inside the
  same test function that called `ops_cube`. Folded to one `prism_ops`
  call with a translating map; it is the member the new `FaceGeometry`
  parameter made foldable for the first time.

Both proved body-identical by `fixtures::deep_snapshot` — 79 lines
each, every arena in slot order with D5 provenance, hand-written against
folded — before either edit was made.

**`build_box`'s 2× extent now has a guard at the claim site.** The PR
measured that no assertion reads it (1291/1291 with the box normalised)
and kept it on a written argument; nothing enforced that argument, and
`FaceGeometry::Declined` means no geometric tier could. The extent is a
named constant (`review_m1_pr3::BOX_EXTENT`) that `build_box`'s profile
and z are spelled from, and `carve_hole` asserts every planted rim and
drop point against it — so normalising the box reds at the recipes
rather than mis-siting them silently. The comment's evidence is
corrected too: the recipes' **face coordinates 0.0 and 2.0** are the
sharp half (flatly off a unit cube's faces), not the section coordinates
0.5/1.5 the PR's comment led with.

## Closed (2026-09-19, PR #2843)

The §9.4.2 class is closed: eight spellings fold onto `prism_ops` with
the declined face geometry as a `FaceGeometry` parameter, every body
proved **byte-identical before and after** by `deep_snapshot` over all
ten arenas, and the two late members proved identical *before* the edit
by a scratch probe rather than after. `cube_independent.rs` stays, on
the surviving clause of
`memories/review-and-dependency-policy.md` — *its row's claim needs its
own derivation* — rather than on the protected-class rule Ev withdrew
(`the-withdrawn-never-simplify-rule-still-stands-in-seventeen-files`).

**The count went 5 → 8 while the unit was being worked**, with the
census re-run at the head each time. A duplication row's count is a
lower bound with a date on it, never a total.

Residue filed rather than disclosed:
`the-9-3-holed-box-sequence-is-written-out-four-times`,
`the-quad-sheet-helper-is-written-three-times-across-two-chart-region-files`,
`the-cylindrical-patch-rim-builder-is-written-nine-times`.
