---
id: pncad-py-tests-rs-is-six-thousand-lines-and-carries-two-unremeasured-floors
kind: issue
title: pncad-py tests.rs is 7795 lines and its two vacuity floors are numbers nothing re-measures
status: open
opened: 2026-09-15
---


Found by the style review of CENSUS-TAG-REACH (2026-09-15) and filed
by that unit's fix pass, which added to the file rather than splitting
it.

## What is in one file

`crates/pncad-py/src/tests.rs` is **7795 lines**, re-derivable by
`git show <sha>:crates/pncad-py/src/tests.rs | wc -l` and measured at
the commit that writes this sentence (6317 when this row was written
at `census/tag-reach`, 6064 before that unit's fix pass added the
reader's shape rosters, 6798 at the merge base `cc6fb870c`). It holds the error-class
taxonomy pin, **three** of this crate's vocabulary censuses
(`TAG_INVENTORY`, `NODE_KIND_ROSTER` and now `ERRORS_MINTING_ITEMS`),
**two** Rust source recognisers with their fixture guards — the
~200-line tag-table one (`read_tag_table`, `Cursor`, `ArmShape`,
`TopForm`) and the ~350-line literal-attribution one
(`read_minting_items`, `impl_spans`, `declaration_heads`) — some sixty
construction pins, and a `#[cfg(test)] mod` of gather-memoization tests
at the end.

**CENSUS-ERRORS-ARRIVAL grew it by 997 lines, a 14.7% growth, and is
why this paragraph has been re-measured.** That unit's whole subject
was that nothing reads a tracker row at the moment a lane writes code;
it then added a second recogniser to the file whose row says the file
is too big, and left this paragraph saying 6317. A style review caught
it, and no instrument did — the same shape, one level up, as the fifth
map this program's `errors-rs-holds-…` row predicted and did not stop.

**And the first re-measurement was itself short**: the fix pass wrote
7779 and +981, numbers taken from a `wc -l` run before its last two
edits to that file and committed after them, so the corrected count was
born stale in the very paragraph whose subject is a count that went
unre-measured. That is standing finding 11 (*a stale count can be born
stale*) at one more level up, and it is the argument for what follows.

**No instrument holds this number, and one that pinned it would be
wrong.** A test comparing a committed line count to `wc -l` reds on
every line added to a file that is expected to change, and its repair
is to edit a number by hand — this row's own complaint, at
per-commit frequency instead of per-unit. What is cheap and is done
instead is to make the number SELF-CHECKING rather than held: it is
stated with the command that re-derives it and is measured at the
commit that writes it, so a reader can date it in one line rather than
trusting it. The floors below are the opposite case — they are
assertions, they already run, and deriving them from the committed
inventory in the same file costs nothing and is what this row asks
for.

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

Whoever owns the file decides whether the recognisers and their
fixtures belong beside the pins they serve or in their own module. The
floors are cheaper and are what this row asks for at minimum.

**The argument now has a second side and a precedent on each.** This
program's other two instruments —
`crates/test-utils/tests/hand_written_impl_census.rs` and
`deny_unknown_fields_census.rs` — are their own files, and on that
precedent a third recogniser belongs in one too. The precedent for
staying is `TAG_INVENTORY`: a census whose subject is a sibling module
of the crate it lives in, reading that module's source through
`crate_dir`, and whose `held_by` column names tests in this same file.
CENSUS-ERRORS-ARRIVAL sited its census here on the second and weighed
the two at `ERRORS_MINTING_ITEMS`; it is recorded rather than settled,
because the cost it pays is this row.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.
