---
id: surface-and-curve-kind-mirrors-have-a-tautological-guard
kind: issue
title: The SurfaceKind/CurveKind completeness guard asserts a tautology, and two doc comments claim it holds
status: open
opened: 2026-09-11
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
