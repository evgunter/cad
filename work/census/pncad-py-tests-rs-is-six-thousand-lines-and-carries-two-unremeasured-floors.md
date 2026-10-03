---
id: pncad-py-tests-rs-is-six-thousand-lines-and-carries-two-unremeasured-floors
kind: issue
title: pncad-py tests.rs is 9162 lines and its two vacuity floors are numbers nothing re-measures
status: open
opened: 2026-09-15
priority: P3
cost: D
---


Found by the style review of CENSUS-TAG-REACH (2026-09-15) and filed
by that unit's fix pass, which added to the file rather than splitting
it.

## What is in one file

`crates/pncad-py/src/tests.rs` is **9162 lines**, re-derivable by
`git show <sha>:crates/pncad-py/src/tests.rs | wc -l` and measured at
`c9c007ef6`, the commit that writes this sentence (6317 when this row
was written at `census/tag-reach`, 6064 before that unit's fix pass
added the reader's shape rosters, 6798 at the merge base `cc6fb870c`,
7805 on `main` at `370bd6f41`). It holds the error-class
taxonomy pin, **three** of this crate's vocabulary censuses
(`TAG_INVENTORY`, `NODE_KIND_ROSTER` and `ERRORS_MINTING_ITEMS`),
**four** source recognisers with their fixture guards, some sixty
construction pins, and a `#[cfg(test)] mod` of gather-memoization tests
at the end.

The four recognisers, three over Rust and one over a Python stub:

* the ~200-line tag-table one (`read_tag_table`, `Cursor`, `ArmShape`,
  `TopForm`);
* the literal-attribution one (`read_minting_items`, `scope_spans`,
  `declaration_heads`, `SCOPE_WALK_BLIND_SPOTS` and their helpers),
  **658 lines** at `c9c007ef6` — lines 6733 to 7390, of which 262 are
  doc comments;
* `declares_test`/`attribute_run`, which reads this file's OWN source
  for a `#[test] fn` of a given name, so that the two committed lists
  that cite tests by name cite names something looks up;
* `stub_instance_attributes`, which reads `pncad.pyi` — a Python stub
  and no Rust at all — by line prefix and triple-quote parity, and is
  the second reader of a convention `tests/test_stubs.py` parses with
  `ast` (`one-stub-convention-has-two-readers-in-two-languages`).

**The row said "two recognisers" and "~350 lines" while the file held
four and 658**, which is this row's own subject at one more remove:
the count of the things in the file that make it big went
unre-measured in the row about the file being big. The second and
third arrived in CENSUS-ERRORS-ARRIVAL and its residue unit, in diffs
that touched this paragraph without re-reading it.

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

**The corrected 7795 then went stale at the MERGE, which is a way this
row had not recorded.** 7795 is right at `fa49e26b0`, the commit that
wrote it, and `17a68a0fa` — the merge that landed it — is 7805:
another lane had added an `ortho_frame_error_tag` row to
`TAG_INVENTORY` on the other parent, and the merge carried both. So
the mitigation this row adopted, *stated with the command that
re-derives it and measured at the commit that writes it*, does not
survive a concurrent change to the same file, and the stale number
reached `main`, this row's title, `plan.md`'s slate and the spec of the
next unit. What a count in a tracker row is about is the TREE, and a
merge-only repo moves the tree between the measurement and the landing.

**CENSUS-ARRIVAL-RESIDUE grew it 1357 lines, 17.4%** (9162 at
`c9c007ef6`, against 7805 on `main` at `370bd6f41` — and still 7805 at
`f8f8e648e`, which is `main` re-read on 2026-09-16, the check the
paragraph above says a number in this row now owes; **1544 added and
187 removed**, of which 637 of the additions are doc comments —
`git diff --numstat origin/main -- crates/pncad-py/src/tests.rs`, taken
after the last edit to the file). Its subject was the residue of the
instrument that grew this file last, so it is the same recogniser
growing again: a per-entry blind-spot list on the scope walk, now data
rather than prose with every probe re-derived; a scope walk over
`trait` and `fn` as well as `impl` and `mod`; a declaration walk that
finds its keywords instead of its lines; eight new tests, taking the
file's own count from 116 to 124, and six new rows in the reader's
fixture (19 to 25, with two more keys renamed by the wider scope walk);
a `held_by` column restructured from prose into holders a reader
re-derives; and a stub reader with its own fixture guard.

**Its own count arithmetic was wrong too, and in the direction this row
warns about.** The unit's first size-row edit said `867 added / 129
removed`, from `grep -c '^+'`, which counts the `+++` header line: the
figures were 866 and 128, and `git diff --numstat` says so without the
off-by-one. A count taken with a command that includes its own header
is a stale count born stale by a different route.

**No instrument holds this number, and one that pinned it would be
wrong.** A test comparing a committed line count to `wc -l` reds on
every line added to a file that is expected to change, and its repair
is to edit a number by hand — this row's own complaint, at
per-commit frequency instead of per-unit. What is cheap and is done
instead is to make the number SELF-CHECKING rather than held: it is
stated with the command that re-derives it and is measured at the
commit that writes it, so a reader can date it in one line rather than
trusting it — and, since the paragraph above shows that is not enough
on its own, with the SHA on `main` it was last true of beside it, so a
merge that moves the file leaves a number a reader can falsify rather
than one they have to believe. The floors below are the opposite case — they are
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
