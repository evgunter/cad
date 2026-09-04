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

`scripts/gates/viewer-module-kinds.sh`, sited in `ci.yml`'s `mirror`
job and in `ci-local.sh`'s `tier_blind_rows` — the halves that carry no
tier condition — and named in `scripts/check-ci-mirror-parity.py`'s
`TIER_BLIND`, so the siting is enforced rather than remembered. Two of
its checks read `crates/viewer/README.md`'s tables and one reads that
crate's `Cargo.toml`; a change set of only the README is `TIER=docs`
and `RUN_BUILD=false`, so under `discipline` the arms that exist for a
table edit could not fire on a table edit (*a gate must be sited where
it can fire on its own inputs*, Ev 2026-08-20, `ci.yml:804-812`).

**Where a module's kind is declared: its own doc header.** One line per
module, on all 41 modules under `crates/viewer/src`. The subject of the
rule is a module's `use` block, so the declaration sits beside it; an
author changing a module's role meets the contradiction in the file
they are editing, and a new module cannot land without answering the
question.

**What is derived rather than restated.** The driver roster is the
README's `### The drivers` table; the vocabulary roster is its two
vocabulary tables; the forbidden crates are every `dep:` in
`Cargo.toml`'s `app` feature — the right population rather than a
curated "toolkit" list, because every entry there is optional and
reached only through `app`, so a vocabulary (compiled in a
default-feature build) naming one is naming something that is not
there. The forbidden driver-module paths are the driver table's
top-level names minus any that host a tabulated vocabulary, which is
how `crate::session::SessionOp` stays green while `crate::app::…` reds.
Only two things are hand-kept: the two driver type names, held against
the README's own rule text by a check, and the exceptions below.

**The exceptions are site-granular.** An entry is `FILE|NEEDLE|COUNT`.
The needle means an exempted file that gains `use eframe::egui;` still
reds; the count means a sixth `&DocSession` reds, and that fixing a
site without lowering the count reds too, so the entry cannot outlive
its reason. A file-granular entry would have ratified every line later
added to those two files — the class `work/code-quality/D103.md`
records against the bounds allowlist and leaves unruled; D103 names "a
count pinned per file" as one of the three shapes a taker should weigh,
and this is that shape applied inside D103's own fence.

**Three path spellings are matched**, because the README's slogan names
the `use` block and a `use` block has three shapes: `use crate::app;`,
`app::x`, and `use crate::{app, camera::Camera};` — the last hides the
driver as a bare leaf inside braces and is written in 22 places
elsewhere in this workspace. The third is read from a 12-line window
over the code-only view.

What it cannot catch, stated in the gate's own header: a module's ROLE
(it reads what a module NAMES); a driver type reached through a
re-export, a generic, a trait object or a macro; a use tree wider than
12 lines; anything outside `crates/viewer/src`; and a crate reached
through a re-export of a non-`app` dependency — `pollster` is the live
near-miss and is correctly absent from the derived set, being an
unconditional dependency present in the default build.

**Every forbidden name has an isolating fixture, and the fixture list
is derived from the same documents the matcher is** — one case per
driver type, one per `dep:` in the `app` feature, and five per driver
module path, one isolating each of the three path spellings. Deleting
any single arm from the matcher turns `--selftest` red; that was
checked by seven weakening probes, all of which now go red.

The first pass found the rule already false at five sites across
`pick.rs` and `parts.rs`, recorded in the README's `### Two
vocabularies that read the session`, in those two modules' own headers,
and scheduled at
`work/view/pick-and-parts-name-the-session-driver.md`.
