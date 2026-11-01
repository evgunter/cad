---
id: censussubject-eq-answers-false-for-a-new-variant-against-itself
kind: issue
title: CensusSubject's hand-written PartialEq has a catch-all arm, so a new variant compares unequal to itself and breaks the Eq it also implements
status: closed
opened: 2026-09-15
priority: P3
cost: E
closed: 2026-10-26
---


Found by CENSUS-DEBUG (`work/census/hand-listed-debug-censuses-in-geom-core-geom-and-topo.md`)
while sweeping hand-listed `Debug`/`PartialEq` walks. Filed rather than
fixed: it is outside that unit's fence (the types whose `Debug` it
touched), and the repair is a one-line decision topo owns.

## The defect

`impl PartialEq for CensusSubject` (`crates/topo/src/validate.rs`,
`:380`) matches `(self, other)` over two named pairs and closes with
`_ => false`. `impl Eq for CensusSubject` follows it.

A variant added to `CensusSubject` therefore **compares unequal to
itself**, silently, with no arm anywhere going red — and `Eq`'s
reflexivity is a promise the type would then be breaking. A `BTreeMap`
or `HashSet` keyed on a subject stops finding what it stored.

## The sibling that already argues the fix

`crates/editor-core/src/meta/mod.rs`'s `impl PartialEq for MetaValue`
has the identical shape and closes the hole in prose at the site: its
mismatch arm is *"spelled over the whole vocabulary rather than swept
up by a catch-all, so a value kind added to `MetaValue` must be given
its own arm above instead of silently comparing unequal to itself,
which would break the reflexivity `Eq` below"*. That is this row, in
another crate, already decided.

The repair is the same: replace `_ => false` with the mismatched pairs
spelled out, so a new variant is a non-exhaustive-match error (E0004)
rather than a wrong answer.

## What does NOT see it

`crates/test-utils/tests/hand_written_impl_census.rs` reds on a walk
that reads a field by name with no exhaustive `Self` pattern. A
catch-all arm reads no field, so this site classifies as clean there and
the census's module docs name it as a stated blind spot. Nothing else
asks the question.

## Closed (2026-10-26, tracker sweep)

Fixed by this sweep (the repair the row wrote down). `impl PartialEq for CensusSubject` (`crates/topo/src/validate.rs`, ~`:555`) now spells the mismatches out, `(Self::Entity(_), Self::FacePair(..)) | (Self::FacePair(..), Self::Entity(_)) => false`, with a comment giving the reason (the `MetaValue` precedent's), in place of `_ => false`. A new variant is now E0004 rather than unequal to itself. The census doc that named this site as the live instance of its catch-all blind spot (`crates/test-utils/tests/hand_written_impl_census.rs:88-92`) now says it was the instance until the `eq` spelled its mismatches out. Needs the orchestrator's build to confirm.
