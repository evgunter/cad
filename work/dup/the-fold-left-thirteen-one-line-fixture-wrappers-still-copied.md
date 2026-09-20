---
id: the-fold-left-thirteen-one-line-fixture-wrappers-still-copied
kind: issue
title: Six box fixtures are still declared once per suite after the fold, at fifteen sites, and one whole corpus is copied
status: review
opened: 2026-09-19
branch: dup/one-line-fixture-wrappers
pr: 2899
---

**The id says thirteen and the object is fifteen.** Ids are stable
(`work/README.md`), so it keeps the number it was filed under; the
title and the table below are the count, and the paragraph after them
says what it counts.

## Finding

- **Where**: `crates/sweep/tests/` — six fixtures, **fifteen
  declaration sites**, enumerated below.
- **Importance**: low-medium
- **Confidence**: sure; measured mechanically on the folding branch's
  own head, not read
- **Raised by**: the `dup/private-box-builders` lane, 2026-09-19,
  reading its own diff for X4 before handing back, and re-measured in
  that PR's fix pass

`private-extruded-box-builders-outside-the-brick-door` folded 32
private box builders onto `sweep::test_support::{brick, block, cube}`.
Each folded builder's BODY is now one line. What the fold did not
remove is the **naming**: where two suites wanted the same box, they
still each declare a wrapper for it, and the wrappers are identical
one-liners rather than identical ten-liners.

## The measurement, and what the number counts

Every `fn` in `crates/sweep/tests/` whose entire body is one call to
`brick`, `block` or `cube`, normalised by dropping the door prefix and
the trailing tolerance argument, grouped by that body:

| the body | declared as |
| --- | --- |
| `block(4.0, 4.0, 1.0)` | `m5_pr9_boss_union::plate`, `m9_3_wall_door::plate`, `m5_s13_pips::slab`, `m5_s13_pips_interval::slab` |
| `brick((0.0, 6.0), (0.0, 4.0), (z0, z0 + 1.0))` | `curved_mergedoor::plate6`, `r1_probes_m9_3::plate6`, `m9_3_zip::plate` |
| `brick((0.9, 1.1), (1.25, 1.35), (0.3, 0.7))` | `m5_s10_face_sense::pellet`, `m5_s11_concave_sense_interval::pellet` |
| `brick((cx - h, cx + h), (-h, h), (z0, z0 + 0.4))` | `n3r1_prune::small_box`, `s16_box_soundness::small_box` |
| `brick((-0.9, x_max), (-0.15, 0.15), (-0.1, 0.1))` | `n3r1_prune::rim_plate`, `s16_box_soundness::rim_plate` |
| `brick((-0.15, 0.15), (y_min, 0.9), (0.9, 1.1))` | `n3r1_prune::top_rim_plate`, `s16_box_soundness::top_rim_plate` |

**Six fixtures, fifteen declaration sites.** Twelve further one-line
wrappers in that tree are singletons and are not in this row.

**Sixteen, if the count is of the FIXTURE rather than of the body.**
`m5_s13_review_probes::slab` is `brick((0.0, 4.0), (0.0, 4.0), (0.0,
1.0))` — geometrically the first row's box, spelled **by bounds** where
the other four spell it **by extent**, so a by-body census reads it as
a different fixture and a reader does not. That is the two-vocabularies
shape `block`'s own rustdoc argues about, one layer out: the door
offers both views deliberately, and the cost is that a census over
spellings under-counts a fixture by one every time a suite picks the
other view.

## The rule this row and its PR both apply

A wrapper survives when removing it would delete information:

- **a fixture name the suite's rows read by** — `pellet` is *the pellet
  strictly inside the notch*, `plate6` is *the 6x4 plate*; inlining
  `brick((0.9, 1.1), …)` at each call site costs the suite its
  vocabulary, which is `sweep::test_support::block` and `cube`'s own
  precedent; or
- **a computation performed once** — `review_m6_5_pr2_sweep_probes::box_at(x0, l)`
  is the `x0 + l` arithmetic, and inlining it hand-evaluates that
  arithmetic at every placement.

A wrapper whose body is the door call with the caller's own arguments
re-spelled — `boxx(x0, x1, y0, y1, z0, z1)`, `slab(x, y, z)`,
`plate(x, y, z)`, `bar(…)` — is a second name for the door and nothing
else. Those went in the PR, and their 47 call sites name `brick`.
Nothing survives the rule that this row would delete, and nothing this
row keeps would survive it.

## The corpus copy, which is bigger than three of the rows above

`crates/sweep/tests/n3r1_prune.rs` and
`crates/sweep/tests/s16_box_soundness.rs` do not share three fixtures,
they share **seven**, byte for byte: `cylinder_at`, `cylinder`,
`small_box`, `nested_box`, `plate`, `rim_plate` and `top_rim_plate`.
`n3r1_prune.rs`'s own header says why — it was *"adopted from a
reviewer probe (the CERT-N3 dual review)"* — so the copy is the
adoption, recorded and then left. The box builders of that set are now
one line of `brick` in both files (which is why three of them appear in
the table); the three-arc cylinder and the four plate placements are
still written out twice.

## Why none of it was fixed in the fold

The remedy is a shared home, and
`crates/sweep/src/test_support.rs`'s header sets the rule that decides
which one: *"A fixture only earns a place here once a consumer OUTSIDE
this crate needs it or a second suite inside it does; the narrower
homes, and the rule that routes between them, are stated in `sweep`'s
own `tests/common` module."* Every fixture above is eligible under the
second clause, and each needs the routing question answered rather than
assumed — `tests/common` or `src/test_support` — across nine suites.
The `n3r1`/`s16` pair needs one answer before that: whether the two
subjects (the census's containment arm, the pruning delta) want one
corpus at all, or share it only because one was adopted from the
other's review.

## Re-taken 2026-09-20 (branch `dup/one-line-fixture-wrappers`, merge base `b29fe8bd1`)

**The figure is 35 one-line box wrappers in the tree, of which 16 are
this row's — six fixtures at sixteen DECLARATION sites.** What that
counts: a `fn` in any tracked `.rs` whose whole body, comments
stripped, is a single call to `brick`, `block` or `cube` with no
semicolon, grouped by the GEOMETRIC box each call names rather than by
its spelling. Not bodies (two of the sixteen spell the same box two
ways) and not call sites (those are counted per fixture below).

The instrument is a source walk over `git ls-files` with **no path
argument** — every tracked `.rs`, every root, every cfg. Its blind
spots: a wrapper written over two or more statements, a wrapper built
in a loop or a closure, a door reached under an alias the regex does
not know, and a box whose extents are not literal or locally bound.
Closing the row's own disclosed blind spot — the by-bounds/by-extent
split — was done by normalising each hit to its `(x, y, z)` bounds
before grouping, which is what turns fifteen into sixteen and is the
only difference between this figure and the row's.

**The row's table reproduces exactly at the merge base**, and the
sixteenth (`m5_s13_review_probes::slab`) is the one it predicted.

### Two of this row's own claims were wrong

- **"Those went in the PR, and their 47 call sites name `brick`"** is
  false of one member. `census_containment_cause::boxx(x0, x1, y0, y1,
  z0, z1)` is still the door call with the caller's own arguments
  re-spelled, at one call site. Inlined in this unit.
- **"they share SEVEN, byte for byte: `cylinder_at`, `cylinder`,
  `small_box`, `nested_box`, `plate`, `rim_plate`, `top_rim_plate`"**
  is wrong in three ways. `plate` is declared in NEITHER file.
  `cylinder_at` is `n3r1_prune`'s alone, and the two `cylinder`s are
  two parameterisations, not one text
  (`conic-corpus-cylinder-has-two-parameterisations`). What IS
  byte-identical is **five fixtures** — `small_box`, `nested_box`,
  `rim_plate`, `top_rim_plate` and `rounded_plate`, which the row did
  not list and which is the largest of them at twenty-two lines —
  plus `p2`, which is every suite's and not this class.

### The per-site disposition, with the rule's limb named

The rule: a wrapper survives when deleting it deletes information —
**(1)** a fixture name the suite's rows read by, or **(2)** a
computation performed once. Sixteen sites, and the rule selects limb
(1) at every one of them: each is a zero-or-few-argument wrapper
carrying CONSTANTS, so inlining it would spell those constants at
every call site instead of once. **Nothing in this row was deleted by
the rule.** What the rule does not settle is where the one body lives,
and that is what this unit moved: the NAME stays at the suite, as an
import; the BODY goes to one home.

| site | limb | disposition |
| --- | --- | --- |
| `m5_pr9_boss_union::plate` | (1) | body to `common::operands::slab`; name kept as `use … as plate` |
| `m9_3_wall_door::plate` | (1) | same |
| `m5_s13_pips::slab` | (1) | body to `common::operands::slab`; name imported |
| `m5_s13_pips_interval::slab` | (1) | same, at the `Interval` scalar — the door is generic |
| `m5_s13_review_probes::slab` | (1) | same; its by-bounds spelling and the other four's by-extent one are the SAME door call, so the fold is textual, not a change of construction |
| `curved_mergedoor::plate6` | (1) | body to `common::operands::plate6`; name imported |
| `r1_probes_m9_3::plate6` | (1) | same |
| `m9_3_zip::plate` | (1) | same; name kept as `use … as plate` |
| `m5_s10_face_sense::pellet` | (1) | body to `common::operands::pellet`; the fixture's own invariant (it is strictly outside the notch) travels with it |
| `m5_s11_concave_sense_interval::pellet` | (1) | same, at the `Interval` scalar |
| `n3r1_prune::small_box` | (1) | body to `common::operands`; reached there only through `nested_box` |
| `s16_box_soundness::small_box` | (1) | same; two direct call sites besides |
| `n3r1_prune::rim_plate` | (1) | body to `common::operands` |
| `s16_box_soundness::rim_plate` | (1) | same |
| `n3r1_prune::top_rim_plate` | (1) | body to `common::operands` |
| `s16_box_soundness::top_rim_plate` | (1) | same |

Outside the six-fixture table, in the same fence and dispositioned:

| site | limb | disposition |
| --- | --- | --- |
| `n3r1_prune::nested_box`, `s16_box_soundness::nested_box` | (2) | `small_box(cx, h, 0.3)` is the one placement both corpora read; shared |
| `n3r1_prune::rounded_plate`, `s16_box_soundness::rounded_plate` | (1) | byte-identical twenty-two lines, not a box wrapper but the same corpus's operand; shared |
| `census_containment_cause::boxx` | **neither** | the door call with the caller's own arguments re-spelled, one call site. Deleted; the site names `brick` |
| `n3r1_prune::cylinder_at` / `s16_box_soundness::cylinder` | — | NOT one spelling; declined and filed as `conic-corpus-cylinder-has-two-parameterisations` |
| `s16_box_soundness::top_rim_x_plate` | (1) | one suite's own, so it stays there (`tests/common`'s routing rule) even though its two siblings moved |

The thirty-five hits account for themselves: sixteen are this row's,
three are the doors themselves in `sweep::test_support` (`brick`,
`block`, `cube` — the two views its own rustdoc argues for), one is
`boxx` above, and the remaining **fifteen are singletons** — one
declaration, one suite. Two of those already sit in a shared tree
(`common::approx::unit_box`, `common::cavity::brick`); the other
thirteen are seated where the routing rule puts a helper one suite
uses, and the duplication rule has nothing to say about them —
`s16_box_soundness::top_rim_x_plate`, dispositioned above, and:
`m5_s12_curved_ops_interval::plate`, `review_arceval_r1_probes::block`,
`review_m6_5_pr2_sweep_probes::box_at`,
`s49_census_jurisdiction::brick`, `shell5_r1_probes::boxy_at`,
`verbs_1031b_arcwind::cutter`, `verbs_f7_r2_probes::brick_operand`,
`mesh`'s `r2_bool_door::slab`, `step-export`'s `common::cube`,
`editor-core`'s `resolve::pick`'s `unit_prism`, `topo`'s
`geom_origin_rows::unit_brick` and `review_f7_pole_r1_probes::distant_brick`.
Four of those thirteen select limb (2) rather than (1) — `box_at`,
`boxy_at`, `s49`'s `brick` and `review_arceval`'s `block` each perform
the caller's corner arithmetic once — which is why the rule and the
routing rule agree about them from both sides.

### The routing question, answered: `tests/common`, and it costs nothing

`crates/sweep/src/test_support.rs`'s header seats a fixture there only
*"once a consumer OUTSIDE this crate needs it or a second suite inside
it does"*, and routes the rest by `sweep`'s `tests/common` module
rule: *"an item lives at the narrowest one all of its consumers can
reach"*. **Every consumer of all sixteen sites is a `crates/sweep/tests`
suite**, so the narrowest home is `tests/common` and the `src` arm is
not reached. New module `crates/sweep/tests/common/operands.rs`, body
authoring, routed beside `common::cavity` for the reason that module
gives.

The cost is one `pub mod` line. `crates/sweep/tests/all.rs` is the
crate's only test root (`autotests = false`) and already declares
`mod common;` once for the whole binary, so a suite reaches a fixture
with `use crate::common::operands::…;` — no manifest edit, no feature,
no new cargo target, one parse and one codegen of the module per
binary.

**What that means for the gate question, measured rather than
asserted.** `scripts/gates/test-features-dev-only.sh` refuses a
`features = […]` on a `[dependencies]` line and permits the same on
`[dev-dependencies]`. This unit adds no manifest line of either kind
— `git diff --stat` touches no `Cargo.toml` — so the gate has nothing
to read that it did not read before; run on the branch head it passes.
The `src/test_support` arm, which is the one that would have needed a
`test-support` forward, was not taken.

The available alternative was `use crate::<other_suite>::<fixture>`,
which the aggregated binary makes legal and which
`review_arceval_r1_probes` already does against
`m5_s12_curved_ops_interval::certified`. Declined for the reason
`src/test_support`'s own header gives about hosting fixtures inside a
consuming module: it *"keeps neither module the owner of the other's
fixture"*, and a cross-suite import makes one suite the owner of the
other's.

### The probe, planted (merge base `b29fe8bd1`)

`cargo test -p sweep --test all` at the merge base: **1489 passed, 0
failed, 8 ignored**. At head, unplanted: **1489 passed, 0 failed, 8
ignored**; the `interval` lane's filtered baseline is 5 passed on the
two interval members, out of 1606.

Every plant is a one-line edit to the shared fixture in
`common/operands.rs` (or, for the last row, to the inlined call site),
run filtered to the consuming suites:

| plant | reds |
| --- | --- |
| `slab` 4x4x1 -> 4x4x1.1, `plate6` x 6.0 -> 6.5, `pellet` x 1.1 -> 1.15, one run | **15 rows** across `m5_pr9_boss_union` (2), `m5_s10_face_sense` (1), `m5_s13_pips` (5), `m5_s13_review_probes` (2), `m9_3_wall_door` (3), `m9_3_zip` (1), `r1_probes_m9_3` (1) |
| `plate6` height `z0+1.0` -> `z0+0.9` | 8 rows, `curved_mergedoor` (1), `m9_3_zip` (1), `r1_probes_m9_3` (6) |
| `slab` + `pellet`, `--features interval` | 3 rows: `m5_s11_concave_sense_interval::interval_union_keeps_the_pellet`, `m5_s13_pips_interval::certified::{interval_pip_pair_is_bracketed_and_additive, interval_finding_union_is_bracketed}` |
| `small_box` z-depth 0.4 -> 0.45 | 1 row in `s16_box_soundness` |
| `nested_box` ignores `cx` | 1 row in `s16_box_soundness`, 1 in `n3r1_prune` |
| `rim_plate` ignores `x_max` | 2 rows in `s16_box_soundness`, 1 in `n3r1_prune` |
| `top_rim_plate` pinned into the rim | 1 row in `s16_box_soundness`, 1 in `n3r1_prune` |
| `rounded_plate` doubled in `x` | 1 row in `n3r1_prune` |
| the inlined `census_containment_cause` box, z 1.0 -> 1.05 | both rows of that suite |

**Live at all sixteen sites, and at the three extra ones, with two
qualifications stated rather than buried:**

- `curved_mergedoor`'s `plate6` site is live on the plate's HEIGHT and
  dead on its WIDTH: its rows assert additivity and validity of a pair
  built from two copies of the same fixture, so a symmetric change of
  extent cannot reach them; the height is pinned because a declared
  coincident face is looked up at `z = 1.0`. The first plant left that
  suite green and only the second reddens it — the first plant alone
  would have been a green diff reported as proof.
- `rounded_plate` has **no live probe in `s16_box_soundness` at all**,
  and its bulges — the feature it exists for — have none in either
  suite. Filed:
  `work/tint/rounded-plate-bulges-are-asserted-by-nothing.md`.

### Residue filed

- `work/tint/rounded-plate-bulges-are-asserted-by-nothing.md` — a
  coverage defect, S-TINT's charter.
- `work/dup/conic-corpus-cylinder-has-two-parameterisations.md` — the
  one corpus member this fold declined.
