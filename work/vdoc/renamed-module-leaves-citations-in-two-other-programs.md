---
id: renamed-module-leaves-citations-in-two-other-programs
kind: issue
title: the pick to pickcache rename leaves four open rows on two other programs' slates citing crates/viewer/src/pick.rs
status: open
opened: 2026-09-06
refs: [2083, stale-file-citations-after-the-split]
---


Filed by the lane that renamed the module
(`pick-and-pickindex-are-named-against-their-contents`). **VIEW's act,
another program's slate**, so it is disclosed here rather than fixed
there: `docs/prompts/implementer-discipline.md` §6 forbids a unit
branch from editing another program's tracker directory, and
`work/README.md`'s one-file-one-item rule is why. This row exists so
the disclosure is a scheduled thing rather than a sentence in a merged
PR body.

## What is stale

`crates/viewer/src/pick.rs` no longer exists. Four OPEN rows outside
VIEW's directory name it, and one names the module path:

- `work/chrome/edge-cost-claims-name-a-search-that-is-gone.md:13,19` —
  `crates/viewer/src/pick.rs:1979-1983` and `pick.rs:600-607`. **Both
  were already wrong before this rename**: the row's subject is
  `edge_segments`, which #2079 moved to `pickindex.rs` and this unit's
  first commit moved again to `crates/viewer/src/marks.rs:248`. The
  path is now wrong twice over;
- `work/chrome/mispaired-ids-exempts-the-empty-window.md:32` — *"(`pick.rs`'s
  unit tests)"*. Those tests are `crates/viewer/src/pickindex.rs`'s
  `mod tests` and have been since #2079; `pickcache.rs` carries no
  `mod tests` at all;
- `work/chrome/viewer-first-light-on-real-hardware.md:110` —
  `pick::PickIndex`, which is `pickindex::PickIndex` and has been
  since #2079;
- `work/code-quality/run-on-whitespace-in-message-literals.md:9,23` —
  *"reading `crates/viewer/src/pick.rs` end to end"* and
  `crates/viewer/src/pick.rs:1771`. The cited line is
  `pickindex.rs`'s since #2079 (`stale-file-citations-after-the-split`
  recorded that one and it was not fixed).

`work/fix/error-types-with-no-display-class.md:73,87` names the path
too and is CLOSED, so it is history and stays.

`docs/WORK-TRACKS-2026-09.md:341` names `pick.rs` in VIEW's own
territory sentence (*"Same files as CHROME (`session.rs`, `app.rs`,
`pick.rs`, …)"*). That is a shared doc rather than a program's slate,
and the sentence is a graduation-time description of a track that has
since split the file three ways; a taker should decide whether it is
worth re-wording at all.

## This is the THIRD split feeding one class

`stale-file-citations-after-the-split` is the open class row, and it
already carries two members: the 1c split of `app.rs`/`session.rs`, and
#2079's split of `pick.rs`. This rename plus the marks move is the
third, and its members are the four above. The class row's durable
statement — *"a behaviour-preserving move cannot falsify a sentence
about behaviour, and reliably falsifies one about STRUCTURE"* — is what
predicts every one of them, and a third member is evidence that naming
the class has not yet stopped it happening.

## What this row is for

Routing. The orchestrator has the whole board and can see whether
CHROME and code-quality already know; a unit branch cannot, which is
the second reason §6 gives. Every citation above is a path or a
`module::symbol`, so the fix is a re-point and no claim in any of the
four rows changes.

## Confidence

`sure` on every citation and on which file each subject is in now.
