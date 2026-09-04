---
id: session-shims-and-test-imports
kind: issue
title: The split's pub use shims leave two spellings of every moved path, and re-pointing the 32 test files needs CHROME's glob
status: parked
opened: 2026-09-04
blocked_on: [viewer-session-god-module-split]
---


The scheduled followup the module split owes, filed at the moment the
deviation is chosen rather than after it
(`docs/prompts/reviewer-style-lane.md` Q6: a narrowing owes a named
unit, and "deferred" is not a schedule).

## The deviation

32 of the 44 files in `crates/viewer/tests/` spell
`use viewer::session::{…}` — the module path — for these items:
`SessionOp` (5 files), `Refusal` (5), `ProfileShape` (5), `DocSession`
(5), `Selection` (4), `NodeKindWanted` (4), `DatumSpec` (4),
`FaceSelection` (3), `PatternRuleSpec` (2), `Hovered` (2),
`EdgeSelection`, `AtRestBadge`, `admits`.

**Most of that list, but not all of it, is already spelled at the
crate root.** `lib.rs:147`'s `pub use session::{…}` block re-exports
eleven of the thirteen; `AtRestBadge` and `admits` appear in neither
that block nor any other crate-root `pub use`, so for those two a
re-point is not a substitution. `AtRestBadge` is reached by the module
path in four files — one `use viewer::session::{…}` line
(`story_assembly.rs:51`) and three fully-qualified
`viewer::session::AtRestBadge::…` spellings — and has no crate-root
spelling to move to; `admits` is imported by no test at all, only
named in prose (`combine_ops.rs:1327`), so it does not belong in this
list. Either the block grows first or those sites stay on the module
path. The sweep is that much less mechanical than the rest.

The split moves most of those items into `session::{select, refuse,
op, author}`. It lands `pub use` shims in `session` so **no test file
changes and no assertion moves** — the property that makes an L-size
refactor reviewable at all. The cost is that every moved item then has
two working spellings, one of which is a lie about where it lives, and
the crate's own boundary rule is about being able to read a `use`
block and know what a module is.

## Why it is not simply done in the same unit

`crates/viewer/tests/*` is **CHROME's** glob
(`work/chrome/program.md`); this program's `paths` are `src/*` and the
README. Re-pointing 32 test files from a VIEW unit branch is a
cross-program edit made by diff, which `work/README.md` makes a merge
conflict by design.

It is also the right order on its own merits. A move that changes no
import and a sweep that changes 32 of them are different reviews: the
first is checked by the compiler, the second by reading whether each
new path is the honest one.

## What it takes

Either the announce that widens this program's `paths` to the test
glob for the sweep, or CHROME takes the unit. The mechanical part is a
sweep with `viewer::session::` as the pattern; **what that pattern
cannot match** is a test reaching an item through the crate root
already (correct, and invisible to the sweep) and any `use viewer::app::`
import, which the app half of the split moves separately — five files,
six items, listed in this program's log.

## Blocked on

`viewer-session-god-module-split` — there is nothing to re-point until
the move lands.
