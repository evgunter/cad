---
id: renderer-free-cross-crate-links-are-ungated-off-the-seed-set
kind: issue
title: a cross-crate link from viewer's renderer-free half is gated only while its target crate is a toolkit seed, and the nightly re-take that is supposed to backstop it cannot red
status: open
opened: 2026-09-11
refs: [2332]
---


Found by #2332's style pass over its own README amendment, which is the
point: the bullet was a universal whose stated reason did not produce
its population, in the section that ratifies *a universal in prose owes
the sweep rule that produces its population*.

## The hole

`crates/viewer/README.md`'s Rustdoc posture ruling now says the link
lint is inert on the DEFAULT-features viewer pass. A broken link in the
renderer-free half is therefore caught only by the `--all-features`
pass, and **which pass a run takes is keyed on the SEEDS, while what it
covers is keyed on the CLOSURE**: `ci.yml:1833-1836` selects
`--skip-viewer-toolkit` off `run_viewer_toolkit`, and
`scripts/ci-filter.py:2244` sets that from
`seeds & VIEWER_TOOLKIT_SEEDS`, while `cargo_scope` is the dependent
closure and contains `viewer` whenever anything below it changed.

So a branch that changes a crate `viewer` depends on, without seeding
the toolkit, runs the default-features pass over `viewer` **with the
link lint inert** — and if that branch renamed or deleted an item the
renderer-free half links to, nothing anywhere reports it.

**Writing a link is not the only way to break one.** The README's first
draft of this bullet argued the case was impossible because writing a
link means diffing `crates/viewer`. True, and beside the point: the
property is a link being BROKEN, and a target moves on someone else's
branch.

## Why it is empty today, and the rule that says so

Sweep rule: every intra-doc link in a `///` or `//!` line under
`crates/viewer/src`, outside the `app`-gated modules, **whose first path
segment is an external crate**, checked against
`VIEWER_TOOLKIT_SEEDS`. Read 2026-09-11 at `6891829ee` plus this
branch: **twelve sites, all into `pncad`.**

| Site | Target |
|---|---|
| `blend.rs:425` | `pncad::select::all_edges` |
| `display.rs:262` | `pncad::document::member_of` |
| `docio.rs:85` | `pncad::workspace::WorkspaceError::Io` |
| `marks.rs:297` | `pncad::select::attribute` |
| `matetool.rs:33`, `:153`, `:220` | `pncad::document::member_of` |
| `matetool.rs:54` | `pncad::document::CLASS_DEFERRAL` |
| `parts.rs:11` | `pncad::workspace::Workspace` |
| `props.rs:652` | `pncad::document::Axis3::ALL` |
| `sketch.rs:939` | `pncad::profile::ProfileVertex` |
| `tree.rs:143` | `pncad::document::ClassAdmission` |

`VIEWER_TOOLKIT_SEEDS` is `{"viewer", "pncad", "bvh"}`
(`scripts/ci-filter.py:1428`). `pncad` is in it, so every branch that
can move one of these twelve targets seeds the toolkit and takes the
all-features pass. **The bullet is true, by that and by nothing else.**

**What the rule cannot match**, stated because a sweep whose blind spot
is unstated is not a negative result: a cross-crate name in a doc
comment that is not a link at all (a bare code span is unchecked
already, in both passes, and is the subject of two other rows); and a
target reached through a `use` alias, since the link text carries the
alias and not the defining crate. **`sketch.rs:939` is the reason the
rule says "intra-doc link" and not "bracket"** — it is the reference
form, `` [`ProfileVertex`](pncad::profile::ProfileVertex) ``, and a
bracket-shaped sweep misses it. It is the only one in the crate.

## The backstop that is not one

`ci.yml:1821-1825` says the ruling's second clause travels with the
skip: *"what this pass stops documenting is re-taken ungated, once a
day, by nightly.yml's `rustdoc (viewer, all features)` row"*. That row
is `cargo doc -p viewer --all-features --no-deps`
(`nightly.yml:291-293`), and **there is no `RUSTDOCFLAGS` anywhere in
`nightly.yml`** — so it runs at rustdoc's default lint levels.

Measured rather than argued, by planting
`` [`pncad::document::NoSuchItemAnywhere`] `` in `tree.rs` and running
that exact command: **one `warning: unresolved link`, and exit 0.** The
step re-takes the RENDER — that the crate still documents at
`--all-features` — and cannot re-take the LINT. For the feature-axis
coverage the skip gives up that is enough, because a page failing to
build does fail the command; for this class it is not.

## What a fix looks like, and whose file it is

Three shapes, cheapest first:

1. **A guard on the sweep rule above** — every cross-crate link target
   in the renderer-free half is in `VIEWER_TOOLKIT_SEEDS`, or the gate
   fails. Sited with the other viewer gates, so it runs on every tier
   that reads this crate. Closes the case at its actual cause and costs
   nothing at runtime.
2. **`RUSTDOCFLAGS='-D warnings'` on the nightly row**, which would make
   the named backstop a real one — a day late, and unattended, which
   `docs/prompts/implementer-discipline.md` is explicit about.
3. **Add the target crate to the seed set**, which buys the toolkit for
   every branch touching it and is the most expensive.

**(2) and (3) are in `.github/workflows/nightly.yml` and
`scripts/ci-filter.py` — CIW's and S-TCOST's respectively, not VIEW's.**
Whichever is taken, the crossing is announced rather than assumed; only
(1) is wholly inside this program's fence. Filing this row on VIEW's
slate because the SUBJECT is this crate's link coverage and the sweep
rule is over this crate's doc comments.
