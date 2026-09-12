---
id: gq5-recap-citation-points-at-the-readme-the-split-emptied
kind: issue
title: A DOOR row cites the GQ5 recap at crates/viewer/README.md:1500, which the GUI-DESIGN split moved to another file
status: open
opened: 2026-09-12
---


Found by VIEW's `probe-identity-stops-at-the-instance` unit (#2479,
2026-09-12) while censusing the rows its own diff shifted. Not caused
by that diff: the citation names a file the subject has left.

## What it is

`work/door/dimension-all-has-readers-outside-the-viewer.md`'s closing
paragraph reads *"Also not this row: `crates/viewer/README.md:1500`
spells `Dimension = Length | Angle | Count | Scalar` in prose. That is
the GQ5 design-question recap…"*.

`crates/viewer/README.md` is **1444 lines**, so the citation is past
the end of the file and cannot be read at all. It was **correct when
it was written** — `625722e79e` (*door: publish `Dimension::ALL` and
retire the radio row's inline mirror*, 2026-09-11 19:30) wrote it
against a 1849-line README whose line 1500 is exactly that sentence,
checked here at that sha.

What moved it is **#2462** (`e9824abf3b`, *viewer: the ratified GUI
clauses move to GUI-DESIGN.md; the README is the record*): `G1`–`G5`
and `GQ1`–`GQ7` left the README for `crates/viewer/GUI-DESIGN.md`, and
the GQ5 recap went with them. **The subject is at
`crates/viewer/GUI-DESIGN.md:155`** — located by its own words, not by
arithmetic:

```
  quantities: `Dimension = Length | Angle | Count | Scalar`, every
```

So this is not a line shift. The citation names the **wrong file**, and
no re-derivation inside `README.md` can find the subject, which is why
it is worth a row rather than a silent repoint by a lane outside the
fence.

## What is NOT claimed here

**The population.** #2462 moved a whole clause set out of a 1849-line
file, so every row citing `crates/viewer/README.md:N` for anything the
split carried is damaged the same way, and this is one confirmed
member found incidentally. A `wc -l` filter finds only the members that
landed past the new end of the file (this one); a member citing a line
that still exists but now holds different prose is invisible to that
test and needs the subject check per row. Nobody has run that census.
`work/view/stale-file-citations-after-the-split` is a different split
(`session.rs`/`app.rs`) and does not cover this one.

## Home

DOOR's: the damaged row is `work/door/dimension-all-has-readers-
outside-the-viewer.md`. The repair is one line — the file and the
number — and the population question above may belong wider.
