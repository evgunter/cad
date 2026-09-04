---
id: boundary-rule-has-no-mechanical-check
kind: issue
title: The vocabulary/driver rule is called mechanically checkable and nothing checks it
status: review
opened: 2026-09-04
branch: view/module-kind-gate
pr: 1848
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
take: each module in `crates/viewer/src/` declares its kind, and the gate
reads that module's `use` block against the kind — a vocabulary may
not name `DocSession`, `ViewerApp` or `egui`; a driver may name
anything. Where the kind is declared (a header convention, a table in
the README, an attribute) and how a driver split across modules
(`app`, `pane::*`, `widgets`) is spelled are the design, and belong to
whoever takes this. The alternative disposition is equally
answerable: write in the README that the rule is enforced by review
and delete the word "mechanically".

## What landed

`scripts/gates/viewer-module-kinds.sh`, wired into both halves of CI
(the hosted half by a named step in `.github/workflows/ci.yml`, the
local half by the loop that already runs this directory), under
`scripts/gates/lib.sh`'s contract: `--root DIR`, and a `--selftest`
that passes a clean fixture and fires on thirteen planted ones.

**Where a module's kind is declared: its own doc header.** One line per
module, `//! Module kind: **vocabulary**` or `//! Module kind:
**driver**`, on all 41 modules under `crates/viewer/src`. The subject
of the rule is a module's `use` block, so the declaration sits beside
it; a README table would have put the two in different files with the
gate as their only tie, and would have made Markdown table syntax an
unversioned interface for a bash gate. The drift that choice admits —
module and README disagreeing — is closed by check 5, which reads the
first column of the README's two vocabulary tables and requires each
named module to declare `vocabulary`.

What the gate checks: every module declares exactly one kind; a
`driver` declaration is on the ratified roster (`session`, `app`,
`pane` and its bodies, `widgets`, `gpu`), and every roster entry still
exists and still declares `driver`; no vocabulary names `DocSession`,
`ViewerApp`, a toolkit crate, or a `crate::{app,pane,widgets,gpu}`
path anywhere in its **code** (not only its `use` lines — a
fully-qualified name evades an import check); the README's vocabulary
tables agree with the modules they name, and have not lost their rows.

What it cannot catch, stated in the gate's own header: a module's ROLE
(it reads what a module NAMES, so a module that owns state and
dispatches while importing nothing forbidden passes); a driver-roster
entry the README has retired; vocabularies the README does not
tabulate; a driver type reached through a re-export, a generic, a trait
object or a macro; anything outside `crates/viewer/src`.

The first pass found the rule already false at two sites — `pick` and
`parts` take a `&DocSession` — recorded as the gate's only exceptions,
in `crates/viewer/README.md`'s `### Two vocabularies that read the
session`, and scheduled at
`work/view/pick-and-parts-name-the-session-driver.md`. The exceptions
retire themselves: an entry that stops hitting the matcher fires the
gate.

`crates/viewer/README.md` also gained `gpu` in the driver split (it
names `eframe::wgpu` and was in no classification) and lost a false
claim that `src/frame.rs` sits behind the `app` feature.
