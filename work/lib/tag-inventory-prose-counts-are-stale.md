---
id: tag-inventory-prose-counts-are-stale
kind: issue
title: the tag-table prose counts drifted: thirty-seven functions / 361 literals stand at 42 / 402
status: closed
opened: 2026-09-08
closed: 2026-09-08
---


Found by LIB-POLYGON while adding one literal to `path_error_tag`.

`crates/pncad-py/src/tests.rs`, the doc comment on
`the_whole_tag_table_matches_its_committed_inventory` (around
`crates/pncad-py/src/tests.rs:2668`), states three aggregate counts as prose:

> Eighteen of the thirty-seven functions carry at least one … The other
> nineteen … have none, and between them hold 199 of the table's 361
> literals

Measured on this tree: `TAG_INVENTORY` holds **42** rows and `src/tags.rs`
declares **42** `pub fn …_tag` (`grep -c`), and the rows carry **402** literal
values. The named pinned list has eighteen entries, so the unpinned group is
**twenty-four** functions holding **285** literals, not nineteen holding 199.

The comment's own argument is what indicts it: *"a prose count is checked only
when someone happens to look"* is the reason the `.pyi` census refuses to write
one down, and this page writes down three. Nothing red when the table grew past
them.

Two ways out, and the choice is a judgement this issue does not make:

1. Delete the counts and keep the QUALITATIVE claim, which is the load-bearing
   half — that construction pins are sampled, that the inventory is the only
   guard over the unpinned functions, and that "the tag table is verified" is
   not a claim this page makes. The two enumerated lists of function names stay;
   they are checkable by reading and do not go stale silently.
2. Derive them at test time from `TAG_INVENTORY` and the pinned-function list,
   and assert rather than narrate — the shape
   `test_the_census_is_not_vacuous` uses.

Not fixed by LIB-POLYGON: that unit corrected only the count its own change
moves (`path_error_tag`'s construction pins, three of thirty -> four of thirty,
the second figure having been one short before it). Re-writing the aggregates
without deciding between (1) and (2) would just restart the same clock.

## Closed (2026-09-08, LIB-ARMS)

Taken by LIB-ARMS, which added twenty-two tag functions and 206
literals and so had to touch the counts. **Option (1)**: the three
aggregates and both enumerated rosters are deleted, and the
qualitative claim — construction pins are sampled, the inventory is
the only guard over the unpinned maps, "the tag table is verified" is
not a claim this page makes — stands unchanged.

The choice was made by a measurement this file did not have. Option (1)
kept the two rosters on the reading that they "are checkable by
reading and do not go stale silently"; they had gone stale silently.
`node_error_tag` stood in the never-pinned roster while
`shell_refusal_tags_are_stable` had been constructing three of its arms
and asserting their words. A roster that is wrong about which maps are
pinned is worse than no roster, because it is read as a survey.

What replaces the numbers is what was already carrying the weight: the
two FLOORS in the test body, which are assertions rather than prose and
so may carry numbers, re-set to 60 functions and 500 literals — well
under the table as it now stands, which is the property the comment
beside them states.
