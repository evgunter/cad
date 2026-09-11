---
id: comment-symbol-names-outside-rustdocs-reach-have-no-gate
kind: issue
title: a symbol named in a comment rustdoc cannot read is checked by nothing, and 26 of them sit in viewer alone
status: open
opened: 2026-09-10
---


Filed by `doc-comments-name-symbols-that-do-not-exist` as it closed.
That item took every unbracketed `<own-mod>::<name>` code span in a
**doc** comment under `crates/viewer/src` — 64 spans over 47 names —
and bracketed the 44 that a rustdoc pass running at `-D warnings` can
actually check, so rustdoc and `scripts/doc-gate.sh` hold those. The
other 20 are named rather than linked because no such pass renders
them (`rustdoc-posture-test-names-one-axis-of-three`), which makes
them unchecked for the same reason this item's subject is, one axis
over. This file is what that pass provably cannot reach: the residue
named in that item's blind spots 1–3.

## Why bracketing cannot close it

A bracket is only worth anything where rustdoc reads the text.
Rustdoc reads `///` and `//!` and **nothing else**, so a name written
in a plain `//` comment is unreachable however it is spelled — the
bracket would not be checked, it would just be punctuation. The
closing PR's own two fixes are the demonstration: the same dead name
sat at `session/op.rs:753` in a `///` comment and at `app.rs:950` in a
`//` comment, and only the first could become a link. The second had
to be corrected by hand and is held by nothing afterwards.

## The population, and the rule that produces it

Rule: every line under `crates/viewer/src` whose first non-space
characters are `//` but **not** `///` or `//!`, scanned for backtick
code spans whose whole content matches `<mod>::<path>` with an
optional trailing `()`, where `<mod>` is one of this crate's own
modules (`lib.rs`'s `mod` list). Each name is then resolved by hand.

**Read 2026-09-10 at `5ace0e9eb` plus the closing PR's diff: 26 spans,
and every one of them names a symbol that exists today.** So this is a
missing gate rather than a live defect — the population is clean and
nothing is holding it that way.

| File | Spans |
|---|---|
| `app.rs` | 12 |
| `pane/viewport.rs` | 7 |
| `gpu.rs` | 2 |
| `forms.rs`, `pane/create.rs`, `pane/features.rs`, `pane/properties.rs`, `session/probe.rs` | 1 each |

(An earlier draft of this paragraph said six in `pane/viewport.rs`,
named a `pane/probe.rs` that does not exist — it is `session/probe.rs`
— and listed `tree.rs`, which has **none**: its only candidate,
`tree.rs:337`'s `SolvedPoses::placement`, is type-qualified and outside
the rule. The total 26 was right. A row whose subject is names that do
not exist is the last place a path that does not exist belongs.)

One resolved by hand and worth naming, because a mechanised version
will trip on it the same way: `app.rs:763`'s
`pickcache::IndexLanding::Stale` is an enum **variant**, so a
declaration regex over `fn|struct|enum|type|const|static|trait|mod|
union` reads it as undefined. It is matched twelve lines below at
`app.rs:775`. A checker for this class needs the compiler or rustdoc's
JSON, not a regex — the parent item's blind spot 6, inherited whole.

## The other two blind spots, measured

- **A span split across two `///` lines** (blind spot 3). The parent
  scan is per-line and cannot see one. Joining every doc line that
  ends mid-span with the one after it and re-running the match:
  **zero** own-module names today. Nine such split spans exist under
  `crates/viewer/src`; none of them is a `<mod>::<path>`. So this
  blind spot is real and currently empty, which is the reason to
  encode it now rather than after it fills.
- **Prose with no code span at all** (blind spot 2) is not measurable
  by any rule of this shape and is not claimed here. A sentence saying
  *"the chrome renders this through supersession\_notice"* in bare
  words matches nothing, and the honest position is that no gate
  proposed for this item covers it.

## What a gate would have to do

This is the symbol-shaped sibling of the `<file>.rs:<line>` gate
`stale-file-citations-after-the-split` asks for, and the two want the
same machinery pointed at different subjects: a name of the shape
`<own-mod>::<ident>` appearing in any comment in the crate resolves,
or the gate fails. **Not built by the closing unit deliberately** —
that unit's fence was the doc-comment class, and a new gate is its own
unit with its own selftest, its own roster question and its own
`--pr`/`--nightly` siting question in `scripts/doc-gate.sh`.

Two things a builder should settle first, because both decide the
gate's cost:

1. **Where the resolver comes from, and it must resolve PATHS.** A
   regex re-inherits blind spot 6 in both directions. It **over**-reports
   on variants, fields, associated items and macro-generated names —
   annoying but visible. It **under**-reports on a span whose leaf is
   declared and whose path is dead, which is the direction that hides a
   dead name and the one a gate exists to catch: the closing pass's
   third finding, `pane::viewport::viewport_ui`, passed a leaf regex
   and names nothing, because `viewport_ui` is an inherent method on
   `ViewerBehavior` merely written in that file. **A leaf-matching gate
   would have shipped it.** `cargo doc --output-format json` resolves
   paths, and `scripts/doc-gate.sh` already runs the doc build such a
   gate would piggyback on.
2. **Whether it is worth it at 26 clean sites.** The argument for
   building it anyway is that the class it catches is exactly the one
   #1957 created: that commit deleted three `frame.rs` items and
   updated their call sites without updating the prose naming them,
   and every stale name this program has chased since traces to it.
   A bracketed link would have reded #1957 at the gate on its own
   branch. A `//` comment would not have, and did not.
