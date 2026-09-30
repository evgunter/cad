---
id: all-census-idiom-forces-the-visit-not-the-update
kind: issue
title: The ALL-census idiom forces a visit to the row, not an update to the list, at every site it has
status: open
opened: 2026-09-11
priority: P3
cost: E
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
variant>`), so the sharpening is the class's, not one site's. It also
rules out two of the three options above rather than merely
strengthening them — see the next section, which measures the
discriminant walk failing the same way.

## A hole-free instrument, worked and running (2026-09-12, the kind-mirror lane)

The two sites this lane was going to add as adoptions are instead the
first two sites of a candidate answer, because the sharpening above
defeats the shape they were first written in. It is running in
`crates/topo/src/query.rs`'s test module as a `macro_rules! census!`,
invoked twice.

**The discriminant walk this file calls "the most promising of the
three" does not close the hole.** Measured, not argued: give
`SurfaceKind` a `const fn index`, add `Probe => 7`, assert every index
in `0..ALL.len()` is yielded by `ALL`. With `Probe` absent from the
list, the list still yields exactly `0..7`, and `index(Probe)` is never
evaluated, so the `7` is never seen by anything. **GREEN.** The walk
inherits the class's defect for the same reason every other shape does:
a test can only feed the match the variants the LIST hands it, so the
list can never testify to its own omissions.

**What closes it is refusing to let a test do the counting at all.**
The compiler is the only party that knows the variant set, so the
assertion has to be one it evaluates itself:

```rust
macro_rules! census {
    ($ty:ident, $list:expr, [$($variant:ident),+ $(,)?]) => {
        const _: () = {
            #[allow(dead_code)]
            fn roster_covers_the_enum(kind: $ty) {
                match kind { $($ty::$variant => (),)+ }
            }
            let mut seat = 0;
            $(
                assert!(matches!($list[seat], $ty::$variant), "…");
                seat += 1;
            )+
            assert!(seat == $list.len(), "…");
        };
    };
}
```

Two halves that close on each other. The match is exhaustive over the
enum with one arm per ROSTER entry, so a new variant reds with `E0004`
and the only cure is to add it to the roster — and adding it to the
roster is what emits `assert!(matches!($list[7], …))`, which a
seven-entry list cannot evaluate. **The edit the compiler demands is
the same edit that reds against a list of the old length.** Nothing is
deferred to a test, so nothing depends on which arm runs.

Walked end to end on the real tree, with an eighth `SurfaceKind::Probe`
and every other exhaustive match in the workspace given its honest arm,
so that the census is the only thing still speaking:

| state | what the compiler says |
| --- | --- |
| `Probe` added to the enum | `E0004: non-exhaustive patterns: SurfaceKind::Probe not covered` |
| `Probe` added to the roster, list left at 7 | `E0080: index out of bounds: the length is 7 but the index is 7` |
| list grown to 8 | green, 659 tests pass |

That middle row is the case this file exists for, and it is the case
every shape in the tree today passes.

The list-side mistakes red the same way, all at compile time — an entry
dropped (`E0080`, out of bounds), the list reordered (`E0080`,
`the list has drifted from the enum`), `CurveKind::ALL` short one
(`E0080`, out of bounds). There is no runtime row left to fail.

### What it costs, and what it still does not hold

The roster is a second hand-written list of variant names — the thing
this file is about. The difference is that it is a list the compiler
checks on both sides: exhaustively against the enum, and seat-by-seat
against `ALL`. It cannot drift from either without a compile error,
which is not true of any of the lists at the sites above.

Two edits still defeat it, and both must state something false rather
than copy something stale: deleting an arm's assertion, or reordering
the roster and the list together. Neither is the mistake this class is
about — that mistake is a variant added and a list forgotten, and the
compiler now walks the author from the variant to the list.

Not claimed: that this is the right instrument for all nine-plus sites.
It is `macro_rules!`, so it follows `crates/viewer/src/vocab.rs`'s
precedent rather than adding a proc macro, but it lives in one crate's
test module and would need a home the other sites can reach — and it
does not yet **print its own population**, which this file asks of
whatever lands. Both are the adoption unit's work, not this lane's.

## Re-homed to CENSUS, 2026-09-20

(DOOR orchestrator) Ev, in chat, 2026-09-20: *"can you kick all the design decisions back to
the track they actually belong to, leaving fix design-free?"* — asked of
FIX and applied to DOOR in the same sitting. **DOOR claims no paths**, so
unlike FIX it can never be the owning track for any decision: there is no
row here whose surface this program owns. A row needing a decision
therefore always leaves. That also retires the charter clause admitting
*"a small design call (where a shared helper's home goes, what a door
looks like)"* — DOOR's own rule already said *"a row that grows a design
question stops being this program's"*, and the two clauses contradicted
each other.

**The decision this row is blocked on:** which mechanism replaces the idiom — the row lists four (sum the arms, the
nightly-only `variant_count`, a discriminant walk, a proc macro) and says
plainly *"none of them decided here"*.

**Why CENSUS.** CENSUS is the program for *a vocabulary spelled by hand in more than one
place, and the census or instrument that cannot see one of the spellings*
(`work/census/program.md`) — and this row is that sentence about a guard
rather than a renderer: three copies of one pin, and the thing the pin claims
to catch is exactly what it cannot see. Its slate already carries the same
shape in `inert-deny-unknown-fields-on-unit-enums` (a guard that does not
guard) and `the-field-brace-fingerprint-is-spelled-at-eight-sites-in-six-crates`.
No single path owns the row: the three sites are WIRE's
(`crates/verbs/src/verb.rs`), REACH's (`crates/topo/src/boolean/mod.rs`) and
unowned (`crates/topo/src/param_source.rs` is in no open program's `paths`).

**The false claim is MEASURED, not argued** — the row compiled the
counterexample standalone: a four-variant enum whose `ALL` holds three
entries passes `ALL.len() == ops` green, because the exhaustive match forces
the VISIT and nothing forces the arm's NUMBER, and the arm an author writes
is the arm they copied. The no-repeats half is sound and does what it claims;
only the growth-alarm half is false.

Whichever mechanism CENSUS picks reaches WIRE's and REACH's files, so it
wants their assent rather than an announcement. The row's own reading is that
the discriminant walk is the most promising — a `const fn index(self)` (three
of the sites have one already), then assert every index in `0..ALL.len()` is
hit — because it closes the hole without a macro.

Nothing about the finding is changed by the move: same id, same
evidence, still `open`, and no part of its question is answered for you.

## A sibling idiom with a weaker guarantee, measured (S-FIX, 2026-09-21)

Found while adjudicating FIX's `recourse-chain-stops-at-the-second-hop-carriers`
(PR 2948) from the diff. Recorded here rather than as a second file
because it is this row's subject — a pin whose doc claims a growth
alarm it does not have — arriving through a different instrument. Split
it out if you read it as its own class.

## The shape

An enforcement row builds its own roster of arms as an array literal
and then asserts the count:

```rust
let arms = [PatchBoundError::DegreeZero, /* … */ PatchBoundError::DerivedKnots];
assert_eq!(arms.len(), 7, "an arm was added without a row here");
```

**`arms.len()` is a compile-time constant.** The assertion cannot fail,
and the message claims something it cannot check: adding a variant to
the enum leaves the array at seven and every row green. This is the
shape `docs/prompts/implementer-discipline.md` §2 names —
*"a predicate over things fixed at compile time … it is documentation,
and deleting it is the repair"*.

**It is weaker than the three sites this row already names.** Those at
least write an exhaustive `match`, so a new variant fails to compile
and the VISIT is forced; only the NUMBER is unforced. Here there is no
match over the enum at all, so neither is.

## The population, measured on `d8da18a`

`git grep -l 'an arm was added without a row here' -- 'crates/**/*.rs'`
— **eleven assertions across ten files in four crates**:

`topo/src/chart_region.rs`, `topo/src/props.rs`, `topo/src/pcurves.rs`,
`geom-core/src/predicate.rs`, `geom-core/src/spline/knots.rs`,
`geom-brep/src/patch_bound.rs`, `geom-brep/src/offset_fit.rs`,
`geom-brep/src/offset_meters.rs`, `geom-brep/src/props/mod.rs`,
`geom/src/curves/fit.rs`.

**Blind spot of that grep**: it keys on one message string, so a roster
assertion phrased any other way is invisible to it. The real population
is at least this.

**Most of these predate PR 2948**, which added five more — so this is a
house pattern propagating, not one lane's invention, and the fix is a
convention rather than a bug report. The mechanism question is this
row's: whichever answer it picks (the discriminant walk reads best) is
what these eleven should be converted to.
