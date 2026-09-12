---
id: all-census-idiom-forces-the-visit-not-the-update
kind: issue
title: The ALL-census idiom forces a visit to the row, not an update to the list, at every site it has
status: open
opened: 2026-09-11
---



Filed by the `BooleanOp::ALL` unit, whose review measured the hole in
the very instrument that unit had just copied. The unit's own census
row carries the measurement at its site and points here rather than
patching one of three copies.

## The idiom

Three rows in the tree pin a hand-written `ALL` against its enum the
same way:

- `crates/verbs/src/verb.rs:436` `all_is_the_whole_vocabulary`
- `crates/topo/src/param_source.rs:318` `all_is_the_whole_field_declaration`
- `crates/topo/src/boolean/mod.rs` `all_is_every_operation` (added by
  the unit that filed this)

Each writes an exhaustive match over the enum whose every arm names the
same total, then asserts `ALL.len()` equals it and that `ALL` holds no
repeats. Each doc claims some version of *"every arm names the same
total, so visiting it means writing the new count, which then reds
until `ALL` has grown too."*

## The claim is false, measured

The match forces the VISIT — a new variant fails to compile until an
arm is written for it. Nothing forces the arm's NUMBER, and the arm an
author writes is the arm they copied. Compiled and run standalone:

    enum Op { Union, Intersect, Subtract, Xor }
    impl Op { const ALL: &'static [Op] = &[Op::Union, Op::Intersect, Op::Subtract]; }
    let ops = match Op::Union {
        Op::Union => 3, Op::Intersect => 3, Op::Subtract => 3, Op::Xor => 3,
    };

`ALL.len() == ops == 3` with `Xor` absent from `ALL`: **GREEN**. The
only murmur is a `variant is never constructed` warning, which is an
artifact of the fixture — a real variant is constructed somewhere and
that warning does not fire.

So what the row buys is a forced visit and a forced decision, which is
not nothing and is not what the docs say. The no-repeats half is sound
and does what it claims.

## The shape an answer has

The number has to come from somewhere the author cannot copy. Options,
none of them decided here:

- **Sum the arms instead of naming a total.** `ALL.iter().map(|v| match
  v { … => 1 }).sum()` counts what the match VISITS, not what an author
  typed — but it visits `ALL`'s entries, so an absent variant is absent
  from the sum too. It moves the hole rather than closing it.
- **`std::mem::variant_count::<T>()`** is exactly this number and is
  nightly-only (`feature(variant_count)`); this workspace is stable.
- **A discriminant walk**: give the enum a `const fn index(self) ->
  usize` (three of the sites have one already for other reasons), then
  assert every index in `0..ALL.len()` is hit by `ALL`. The index match
  is exhaustive, so a new variant must be given an index, and an index
  the list never yields reds. This closes it without a macro and is the
  most promising of the three.
- **A proc macro** projects the list from the declaration and ends the
  class outright. The workspace has none and
  `crates/viewer/src/vocab.rs`'s `vocabulary!` is the `macro_rules!`
  precedent for refusing to add one.

Whichever is taken, it is one instrument written once and adopted at
all three sites, plus the three docs corrected to claim what holds.
`crates/viewer/src/vocab.rs`'s projected `ALL` is NOT in this class:
there the list and the enum are one declaration and the question does
not arise.

## The population is not three (2026-09-11, the DOOR orchestrator)

**This item was filed carrying a hand-maintained count of the sites of a
defect about hand-maintained counts, and the count was wrong within the
hour.** PR #2391 added a fourth (`all_is_every_dimension`,
`crates/editor-core/tests/m4_pr1_dims.rs`) and did not amend this file;
the review of that PR caught it. That is this item's own subject
happening to this item, and it is the strongest argument for the
instrument it asks for.

**Measured** with a structural scan — a `match` whose every arm is
`=> <the same integer>` — over `crates/**/*.rs`:

| site | arms |
| --- | --- |
| `crates/topo/src/param_source.rs:312`, `:319` | 5 each |
| `crates/topo/src/boolean/mod.rs:2559` | 3 |
| `crates/verbs/src/verb.rs:388`, `:396` | 5 each |
| `crates/verbs/src/verb.rs:437` (one match, wrapped) | 10 → 9 |
| `crates/verbs/src/flow.rs:402` | 5 |
| `crates/editor-core/tests/m4_pr1_dims.rs:184` | 4 |
| `crates/geom-brep/src/certify.rs:2233` (one match, wrapped) | 15+ → 21 |

**That is a FLOOR, not a count, and this file will not carry one.** The
scanner reads a fourteen-line window, so a match whose arms wrap past it
is split (two rows above are one census each) or missed entirely —
`crates/verbs/src/flow.rs:445` (`FlowSource`), named by the review, does
not appear above for exactly that reason. What the measurement
establishes is only that the population is **at least nine and not
three**, which is all this item needs to justify an instrument.

Establishing the exact number is the instrument's job, not prose's.
Whatever lands here should print its own population, so the next reader
does not inherit a number somebody counted by hand — the failure this
paragraph is a record of.

**Two of the sites above are worth a second look on their own**, found
incidentally and not chased: `verb.rs:437`'s match appears to carry ten
arms against a stated total of nine, and `certify.rs:2233`'s stated 21
is larger than the arms the scan could see. Either may be the scanner's
windowing rather than the code's; neither was verified, and the
instrument will answer both.

## The hole is one notch wider than filed (2026-09-12, the kind-mirror lane)

Measured while adopting the idiom at the `SurfaceKind` / `CurveKind`
mirrors. This file says the arm an author writes is the arm they
copied. The sharper statement is that **only ONE arm is ever read at
all** — the scrutinee's — so the other arms are not a second chance to
catch the author, and they can disagree with each other indefinitely
without any row noticing.

Probe, on `all_surface_kinds_is_the_whole_enum` with a scratch eighth
`SurfaceKind::Probe` variant absent from `ALL_SURFACE_KINDS`:

    let kinds = match SurfaceKind::Plane {
        SurfaceKind::Plane => 7,   // …and five more arms at 7
        SurfaceKind::Approx => 7,
        SurfaceKind::Probe => 8,   // the new arm, honestly numbered
    };

`kinds` is the `Plane` arm's 7, `ALL_SURFACE_KINDS.len()` is 7:
**GREEN**, with the new kind absent from the list AND an arm in the
same match saying the total is 8. Writing 8 in *every* arm reds it
correctly (`it holds 7 kinds, the enum has 8`). So the row rewards the
author who re-decides the number in all arms and is silent about the
one who re-decides it in the arm they just wrote — the likelier
mistake, since that is the arm the compiler pointed at.

Every site in the table above has this shape (a `match <one literal
variant>`), so the sharpening is the class's, not one site's. It
strengthens the case for the discriminant-walk or macro answer rather
than changing which answer is right.

## Two more sites (2026-09-12)

`crates/topo/src/query.rs`'s `all_surface_kinds_is_the_whole_enum` (7
arms) and `curve_kind_all_is_the_whole_enum` (4 arms), added by the
`surface-and-curve-kind-mirrors-have-a-tautological-guard` unit, which
adopted the idiom because it is what the tree has and its residue is
this item. They are two more adoption sites for whatever lands here —
recorded because this file's own subject is a count nobody maintains,
not because the count is now eleven.
