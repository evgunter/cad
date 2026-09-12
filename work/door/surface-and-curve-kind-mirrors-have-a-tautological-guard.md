---
id: surface-and-curve-kind-mirrors-have-a-tautological-guard
kind: issue
title: The SurfaceKind/CurveKind completeness guard asserts a tautology, and two doc comments claim it holds
status: closed
opened: 2026-09-11
closed: 2026-09-12
---



Filed by the `BooleanOp::ALL` unit's sweep of the eight unswept
`const ALL` neighbours named in
`boolean-op-has-a-third-hand-written-complete-list`. Two of the eight
are not stale, but the thing that is supposed to keep them that way
does not do it.

## The finding

`crates/topo/src/query.rs` publishes two complete variant lists:
`CurveKind::ALL` (`:130`, four kinds) and `ALL_SURFACE_KINDS` (`:216`,
seven kinds). Both are complete against their enums today.

Their claimed guard is
`crates/editor-core/tests/lib_sel1_geoselect.rs:487`,
`the_surface_kind_mirror_is_complete`, and its central assertion is a
tautology:

    let all = SurfaceKindSet::of(ALL_SURFACE_KINDS);
    assert_eq!(all.iter().count(), ALL_SURFACE_KINDS.len(),
               "a kind added to SurfaceKind must be added to ALL_SURFACE_KINDS");

`SurfaceKindSet::iter` (`query.rs:262`) enumerates `ALL_SURFACE_KINDS`
and filters by membership, so BOTH sides of that equality are derived
from the list under test. A kind missing from the list is missing from
both sides and the row stays green; what the assertion actually
measures is that the list holds no duplicates. Its message names a
failure it cannot see. The `CurveKind` half two lines below
(`:499-500`) is the same shape with the same hole.

Two doc comments state the guard as real and are wrong with it:

- `crates/topo/src/query.rs:199-201` — "`ALL_SURFACE_KINDS` below is
  pinned against this function by a unit test". `surface_bit`'s
  exhaustive match does red on a new variant, which forces a VISIT to
  the file; nothing then forces the list.
- the test's own doc at `lib_sel1_geoselect.rs:482-485` — "This pins
  the pair".

## The fix is written elsewhere in the tree

The census idiom this repo already uses holds exactly this: a match
over the enum whose every arm names the same total, so a new variant
fails to compile until it is visited and the count then reds until the
list has grown. `crates/topo/src/param_source.rs:318`
(`all_is_the_whole_field_declaration`), `crates/verbs/src/verb.rs:436`
(`all_is_the_whole_vocabulary`) and, as of the unit that filed this,
`topo::boolean`'s `all_is_every_operation` are three instances. The
guard also belongs in `topo`, beside the lists, rather than in an
`editor-core` suite two crates downstream.

## Scope

Two lists, one test row to replace (or two), two doc sentences to
correct. `E` on this program's scale.

## Closed

Both halves of the premise were checked by execution before anything
changed, and both held.

**`SurfaceKind`**: a scratch eighth variant `SurfaceKind::Probe` was
added to `crates/geom-brep/src/intersect.rs`. Three exhaustive matches
red immediately (`SurfaceKind::name`, `route`, and
`topo::splitting::classify`'s kind refusal) plus `surface_bit`, which is
the VISIT the row describes. With each of those given an arm — including
`surface_bit`'s `SurfaceKind::Probe => 7` — and `ALL_SURFACE_KINDS` left
at seven entries, `the_surface_kind_mirror_is_complete` passed:
`test lib_sel1_geoselect::the_surface_kind_mirror_is_complete ... ok`.

**`CurveKind`**: the same, with a `CurveKind::Probe` added to
`topo::query`. One match reds (`CurveKind::bit`) plus one message arm in
`query.rs`; given arms and left off `CurveKind::ALL`, the row passed
again. Both probes reverted; `git diff` clean of them.

So the row was right on every count, including that the guard's message
names a failure it cannot see.

## What replaced it

The census idiom, sited beside the lists as the row asked — in
`crates/topo/src/query.rs`'s test module rather than an `editor-core`
suite two crates downstream:

- `all_surface_kinds_is_the_whole_enum` — exhaustive match over
  `SurfaceKind`, every arm naming the same total, then `len` against it
  plus the no-repeats half.
- `curve_kind_all_is_the_whole_enum` — the edge-side twin.
- `kind_bits_are_distinct` — a third row the fix bought cheaply: the
  exhaustive `surface_bit` / `CurveKind::bit` match forces an arm to
  EXIST but cannot see that its value collides with another kind's, and
  a collision makes two kinds indistinguishable inside a set. Pinned as
  `just(k).iter().next() == Some(k)`. The `SurfaceKind` half of the old
  row asserted the weaker `.count() == 1`, which a collision with an
  earlier-listed kind satisfies.

The `editor-core` row survives as
`kind_sets_carry_exactly_their_members`, stripped of both tautological
count assertions and re-documented to say what it actually pins
(membership, the empty set, singletons and canonicality, through the
document layer's re-export) and where the census now lives.

Both doc comments the row named were verified against the tree and
corrected: `surface_bit`'s header no longer claims a unit test pins the
list against it, and the test's own doc no longer claims to pin the
pair.

## What the fix does NOT buy, measured

The census forces a visit and a re-decision, not the edit — and one
notch worse than that: only the SCRUTINEE's arm is ever read, so an
author who writes the honest new total in the arm the compiler pointed
at and leaves the others alone still gets green. Measured on this
lane's own new row and filed onto
`work/door/all-census-idiom-forces-the-visit-not-the-update`, which
already owns the class and is where the instrument that closes it
belongs. The new rows' docs say this at the site rather than claiming
more than they hold.
