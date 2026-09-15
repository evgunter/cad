---
id: pncad-py-tests-rs-is-six-thousand-lines-and-carries-two-unremeasured-floors
kind: issue
title: pncad-py tests.rs is 6300 lines and its two vacuity floors are numbers nothing re-measures
status: open
opened: 2026-09-15
---


Found by the style review of CENSUS-TAG-REACH (2026-09-15) and filed
by that unit's fix pass, which added to the file rather than splitting
it.

## What is in one file

`crates/pncad-py/src/tests.rs` is **6317 lines** (measured
2026-09-15 at `census/tag-reach`, 6064 before this unit's fix pass
added the reader's shape rosters). It holds the error-class
taxonomy pin, two of this crate's four vocabulary censuses
(`TAG_INVENTORY` and `NODE_KIND_ROSTER`), a ~200-line Rust source
recogniser (`read_tag_table`, `Cursor`, `ArmShape`, `TopForm`), that
recogniser's own fixture guard, some sixty construction pins, and a
`#[cfg(test)] mod` of gather-memoization tests at the end.

## The two floors, which are the sharper half

`the_whole_tag_table_matches_its_committed_inventory` asserts

    table.functions.len() >= 60
    literals >= 500

as VACUITY floors — a reader that came back with nothing must red
rather than pass. The site argues for them and says why an assertion
may carry a number where the prose may not, which is right as far as
it goes. **What nothing does is re-measure the headroom.** If
`src/tags.rs` legitimately shrinks below either floor, the row reds
and the repair is to edit a number by hand; if it grows tenfold, the
floor stops discriminating and nothing says so. A floor derived from
the committed inventory itself — which is in the same file and is the
exact pin — would move with the table instead.

## The split, which is a judgement and not this row's to make

Whoever owns the file decides whether the recogniser and its fixture
belong beside the pins they serve or in their own module. The floors
are cheaper and are what this row asks for at minimum.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.
