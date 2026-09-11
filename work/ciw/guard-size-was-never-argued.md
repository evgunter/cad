---
id: guard-size-was-never-argued
kind: issue
title: the apt guard is 1770 lines against an item that estimated a few, and only its siting was argued
status: open
opened: 2026-09-11
refs: [apt-preamble-bypass-is-unguarded]
---


Found by the correctness review of PR 2345, and filed by the lane that wrote
the file.

`work/ciw/apt-preamble-bypass-is-unguarded` estimated its own cheap shape as
*"a few lines beside its existing invocation scan"*. What landed is
`scripts/check-install-wrappers.py` at roughly 1770 lines — a shell
tokenizer, a head-word resolver, a population reader and a selftest of fifty
mutants across five routes.

**The deviation from that estimate was argued at length in the wrong
dimension.** PR 2345 argues SITING — why not an arm in
`check-ci-mirror-parity.py`, why not a `scripts/gates/*` member, why a
separate script like `check-status-capture.py` — and that argument is sound
and was accepted. It says nothing about SIZE, which is the whole cost of the
choice: the item's estimate was for a literal-match arm, and what replaced it
is a reader.

The case for the size is real and should be written down rather than assumed:
a literal match on `apt-get` reds `ci-local.sh`'s own prereq sentence
(`admesh not installed (apt admesh, …)`), so command position is needed;
command position needs words; words need quoting; and a guard that a comment
can trip gets routed around. Each step is forced by the one before it. But
*forced* is a claim, and nobody has tested it against the cheaper thing —
for instance a recogniser that reads only the first word of each LINE, with
a refusal for every line it cannot reduce to one.

## What to decide

- Whether the file is the right size for what it buys, measured against a
  deliberately cheaper alternative rather than against nothing.
- If it is, that argument belongs in the file's header beside the siting
  argument, which is where the next reader will look for it.
- If it is not, the cheaper reader is a rewrite with the existing mutant
  table already written — which is the one thing that makes the question
  answerable at all.
