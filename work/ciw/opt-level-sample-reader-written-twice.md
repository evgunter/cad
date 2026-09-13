---
id: opt-level-sample-reader-written-twice
kind: issue
title: nightly.yml reads the written opt-level sample path twice, with the explanation only at the first
status: open
opened: 2026-09-11
refs: [2327]
---

`.github/workflows/nightly.yml:1289` and `:1305` are the same line in two
steps:

    new="$(git status --porcelain -- docs/perf-data/opt-level | awk '{print $2}' | head -1)"

The first carries the whole argument — *"`record` declines to write one when
fewer than two arms survived … The commit step below asks this one"* — and
sets `wrote=true/false`. The second, in the commit step gated on that output,
re-derives the same path with no sentence saying it is the same read or why it
is taken again.

**Two separable things, and the second is the smaller.**

1. **A constant written twice with the explanation at one copy.** This is the
   class PR 2327's sweep was for, with the digits taken out: nothing compares
   the copies, and an edit to the porcelain parsing at one site leaves the
   other reading the old way. The verdict there — that `head -1` is
   one-candidate-**by construction** here, since `record` writes at most one
   sample — holds for the *value*; it says nothing about the *duplication*.

2. **`awk '{print $2}'` is not a porcelain parser.** Two shapes break it, and
   neither is exotic: a rename is `R  old -> new` (field 2 is the OLD path),
   and any path containing a space is split across fields. Both are unlikely
   under `docs/perf-data/opt-level/<epoch>-<sha>.json`, which is why this is an
   issue and not a defect; `git status --porcelain=v1 -z` with a NUL split, or
   `git diff --name-only`, is what reads the same fact without the guessing.

**The cheap fix is one step output, not two reads**: the recording step already
knows the path it wrote, so it can emit `sample=<path>` alongside
`wrote=true`, and the commit step consumes that instead of re-deriving it.
That also deletes the parsing question at the second site rather than fixing it
twice.
