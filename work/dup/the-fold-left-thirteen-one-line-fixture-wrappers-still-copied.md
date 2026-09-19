---
id: the-fold-left-thirteen-one-line-fixture-wrappers-still-copied
kind: issue
title: Six box fixtures are still declared once per suite after the fold, at fifteen sites, and one whole corpus is copied
status: open
opened: 2026-09-19
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
