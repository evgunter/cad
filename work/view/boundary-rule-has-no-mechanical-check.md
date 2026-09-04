---
id: boundary-rule-has-no-mechanical-check
kind: issue
title: The vocabulary/driver rule is called mechanically checkable and nothing checks it
status: open
opened: 2026-09-04
---


`crates/viewer/README.md:247` ratifies the crate's module boundary
rule and sells it on being checkable by machine:

> The rule is mechanically checkable — read the `use` block — which is
> the property that makes it survive contact with the next unit.

**Nothing reads a `use` block.** Not `scripts/*.py`, not `scripts/*.sh`
(the gates in `scripts/gates/` included), not
`.github/workflows/ci.yml`. The two things that come closest are
`scripts/doc-gate.sh`, which fails on rustdoc's broken intra-doc links
and says nothing about imports, and `clippy`, which has no lint for
"this module names a type from that module". The only enforcement the
rule has is a reader who happens to look.

## Why it matters here rather than in general

The 1c split just took thirteen new modules under this rule, in a
crate that had **four prose claims outrun its tree in a single day**:
the `forms` row's "each a hand-maintained mirror", the dead
`gesture_safe` symbol, the four-`Refusal`-arms framing, and the
`widgets` row's classification (`work/view/log.md`, the 1c entries).
Every one was caught by a reader with the tree open and none by a
gate. A rule whose selling point is mechanical checkability, enforced
by exactly the mechanism that has already failed four times this day,
is the weakest form of the obligation it just imposed.

The specific failure it does not catch: a `use crate::session::DocSession;`
added to a vocabulary module during a later unit compiles, passes
clippy, passes the doc gate, and silently makes the README's
`### The session's vocabularies` table false.

## Reviewer-brief Q6

A claim resting on a mechanism owes one of three things: a guard, a
scheduled re-measure, or a written reason it can have neither. The
README owes one and states none.

## Candidate shape (not a design)

A gate in the shape `scripts/gates/`'s allowlist gates already
take:
each module in `crates/viewer/src/` declares its kind, and the gate
reads that module's `use` block against the kind — a vocabulary may
not name `DocSession`, `ViewerApp` or `egui`; a driver may name
anything. Where the kind is declared (a header convention, a table in
the README, an attribute) and how a driver split across modules
(`app`, `pane::*`, `widgets`) is spelled are the design, and belong to
whoever takes this. The alternative disposition is equally
answerable: write in the README that the rule is enforced by review
and delete the word "mechanically".
