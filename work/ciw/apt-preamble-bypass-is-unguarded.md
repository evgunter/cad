---
id: apt-preamble-bypass-is-unguarded
kind: issue
title: nothing stops a new workflow step spelling its own apt preamble again
status: open
opened: 2026-09-09
---


Disclosed by PR 2277, which routed all five apt call sites in
`.github/workflows/` through `scripts/apt-install.sh` and left this open.

## What is unguarded

The five sites are consistent **today**, and nothing keeps them that way.
A step added tomorrow that writes `sudo apt-get update && sudo apt-get
install -y foo` inline is exactly the state PR 2277 removed, and every
check in the tree passes it:

- `scripts/check-ci-mirror-parity.py` reads `.github/workflows/*.yml` for
  `scripts/` and `demos/` INVOCATIONS. A step that invokes no script is
  outside claims 1–4 by construction, and the file says so at its own
  header ("a hosted row that runs work inline … is outside them").
- Claim 9 is about jobs, not steps, so a new step inside an already-cited
  job adds nothing for it to see.
- There is no shell linter in the hosted gate at all: `shellcheck`
  appears nowhere in `.github/workflows/`.

So the class this program just closed can reopen one step at a time, and
the symptom when it does is the original one — a red gate on every branch
during a third-party mirror's bad hour, indistinguishable from a real red.

## Shapes

The cheap one is a recogniser in `scripts/check-ci-mirror-parity.py`:
inside a `run:` block, `apt-get` may appear only in a comment or in
`scripts/apt-install.sh` itself. That checker already refuses to parse a
workflow it does not understand, so the population is exactly the one it
reads, and the arm is a few lines beside its existing invocation scan.
The hazard is that its subject is paths and invocations, and this is a
literal in an opaque block scalar — the header lists that opacity as
deliberate ("the body of a block scalar … is scanned for invocations; it
is not parsed"), so the arm would widen what a `run:` body means to it.

The alternative is a `scripts/gates/*` member, which owns literal-shaped
prohibitions over a file population — but `scripts/gates/` is code-quality
Track K's territory and `.github/workflows/` is not among the trees those
gates read.

Not taken in PR 2277: that unit's subject was the four red steps and the
one preamble, and a new recogniser arm in the parity checker is its own
review with its own selftest.
