---
id: gate-section-scans-end-on-any-column-zero-hash
kind: issue
title: the two README section scans end at any column-zero # so a Rust attribute or shebang inside a fenced block truncates the section, and the diagnosis blames the anchor
status: open
opened: 2026-09-08
---

Found by the style review of #2172, which hit it while attacking the
new anchor scan with real markdown. **Pre-existing** — it is shared by
both README readers and predates that unit — but the anchor gives it a
new way to bite, which is why it is filed rather than absorbed.

## The mechanism

`scripts/gates/viewer-vocab-declared-once.sh:501` (`readme_kinds`) and
`:536` (`readme_table`) both end the section on `insec && /^#/` — any
line starting `#` at column zero, on the assumption that such a line is
a markdown heading.

Inside a fenced code block it is not. `#[derive(Debug)]`, `#!/bin/sh`
and `# a comment` all start with `#` at column zero and all are
ordinary content. A fence carrying one **truncates the section at that
line**, and everything after it — including the announcing paragraph
and the ratified bullets — becomes invisible to the scan.

## Why the anchor makes it worse

Before #2172 a truncated section produced a kind count that was wrong
in a way the author could puzzle out. After it, a fence landing *above*
the anchor makes the gate red claiming **"the anchor paragraph is
gone"** — a statement about a sentence that is right there in the file,
two lines below the fence. The author is told to restore something they
never removed.

The review reproduced this: a `#[derive(Debug)]` at column zero inside
a fence, placed before the anchor, reds with the missing-anchor
diagnosis.

## Why it is live rather than theoretical

The scanned region is 149 lines of prose about how a Rust macro
projects a vocabulary. A fenced Rust example carrying an attribute is
exactly the thing someone would add to it, and `crates/viewer/README.md`
already carries fenced Rust elsewhere.

## The fix, and the shape it has to have

Track fence state — a line of three or more backticks or tildes toggles
it — and only treat `^#` as a heading outside a fence. Both readers
need it, so it wants to be one helper rather than two copies, which is
the same argument this file's own `readme_table`/`readme_kinds` split
already makes about the `@` sentinel.

**It owes a self-test case**: a fence containing a column-zero `#`
planted inside the section, expecting the gate to stay GREEN. And per
the negative-control practice #2106 established, that case owes proof
it fails against the unfixed reader.

## Confidence

`sure` on the mechanism and on the reproduction. `likely` that fence
tracking is the right repair rather than, say, requiring a heading to
be followed by a space.

