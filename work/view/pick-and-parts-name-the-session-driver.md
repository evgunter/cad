---
id: pick-and-parts-name-the-session-driver
kind: issue
title: pick and parts are vocabularies that name DocSession, so the boundary rule is false at two sites
status: open
opened: 2026-09-04
refs: [1848]
needs_ev: true
---



## Finding

`crates/viewer/README.md`'s `## Module boundaries` rule is that **no
vocabulary may name a driver**, and it names `DocSession` as the
session driver's type. Two vocabulary modules name it:

| site | shape |
| --- | --- |
| `crates/viewer/src/pick.rs:64` | `use crate::session::{DocSession, …}` |
| `crates/viewer/src/pick.rs:2266` | `PickIndex::sync(&mut self, session: &DocSession, …)` |
| `crates/viewer/src/parts.rs:40` | `use crate::session::{DocSession, Refusal}` |
| `crates/viewer/src/parts.rs:140` | `PartChooser::opened(session: &DocSession)` |
| `crates/viewer/src/parts.rs:149` | `PartChooser::rescan(&mut self, session: &DocSession)` |

Both take the session as a **read-only argument** — neither mutates it
and neither dispatches — so neither is a driver under the README's own
definition. The rule as ratified is simply false of the tree here, and
was false before the check that found it existed.

The whole hit list is these two files. It was derived by scanning the
code-only view of every module under `crates/viewer/src` for
`DocSession`, `ViewerApp`, a toolkit crate (`egui`, `eframe`, `wgpu`,
`winit`, `egui_tiles`, `egui_wgpu`, `egui_dock`) and the paths
`crate::{app,pane,widgets,gpu}` — the same scan
`scripts/gates/viewer-module-kinds.sh` now runs on every CI pass. What
that scan cannot see is a driver type reached through a re-export under
another name, a generic parameter, a trait object, or a macro
expansion; and it reads `crates/viewer/src` only, so `tests/` is out of
scope by design.

## Why this is open rather than fixed

Fixing it is a design choice with two answers and no obvious winner,
which is why the unit that built the gate recorded the sites instead of
picking one:

1. **Hoist the read.** Have the session hand out a value — the parts
   census, the pick-cache inputs — and let the two vocabularies take
   that instead. Keeps the rule intact and unqualified; costs a new
   value per reader and moves the derivation into the driver.
2. **Widen the rule** to permit a `&DocSession` as a read-only
   argument. Cheap, and arguably honest about what these two are — but
   it makes "no vocabulary may name a driver" a rule with a clause,
   and the clause is exactly the kind a later unit widens again.

## What holds the line meanwhile

`scripts/gates/viewer-module-kinds.sh` carries the two files as its
only `VOCAB_EXCEPTIONS`, and the exemption is **site-granular**: an
entry is `FILE|NEEDLE|COUNT`, so a **sixth** site reds, a different
forbidden name in the same file reds (the exemption covers the reason
it was granted and nothing else), and fixing a site without lowering
the count reds too — the entry cannot outlive its reason.

A file-granular entry would have ratified every line later added to
these two files, which is exactly `work/code-quality/D103.md` (open,
unruled, track K, fenced to `scripts/gates/`): *"the allowlist is
file-granular while its justifications are per-seam, so later bounds
inherit ratification"*. D103 lists "a count pinned per file" as one of
three shapes a taker should weigh; this is that shape, applied inside
D103's fence, and it is offered as evidence for that ruling rather
than as a substitute for one. `interval-square-allowlist.sh:125-133`
makes the same argument about its own retired entries.

`crates/viewer/README.md`'s `### Two vocabularies that read the
session` records the state for a reader, and both modules say so in
their own doc headers — the gate requires it, because a header reading
*"it names no driver type"* nine lines above naming one is false and
rustdoc publishes it.

## Put to Ev (VIEW orchestrator, 2026-09-04)

**This one is here because the ratified text is yours, not because the
answer is hard.** `crates/viewer/README.md`'s `## Module boundaries`
rule — *no vocabulary may name a driver* — was ratified at #1801, and
the gate built at #1848 to enforce it found the rule is **false of the
tree at five sites across two files**, and was false before the check
existed. Verified on today's tree: `pick.rs:67` and `:2269`,
`parts.rs:43`, `:143` and `:152`.

Both files take `&DocSession` as a **read-only argument**; neither
mutates it and neither dispatches, so neither is a driver under the
README's own definition. The two answers are in the body above (hoist
the read into a value; or widen the rule to permit a read-only
`&DocSession`). This program has no preference strong enough to
self-certify a change to a rule you ratified a day earlier, which is
the whole reason it is on this PR.

**What holds the line meanwhile, so nothing is urgent.**
`scripts/gates/viewer-module-kinds.sh:156-159` carries the two files
as its only `VOCAB_EXCEPTIONS`, and the entries are **site-granular**
(`FILE|NEEDLE|COUNT`): a sixth site reds, a different forbidden name in
the same file reds, and fixing a site without lowering the count reds
too. The exemption cannot outlive its reason.

That granularity is itself offered as evidence for an open
code-quality ruling — `work/code-quality/D103.md`, *"the allowlist is
file-granular while its justifications are per-seam, so later bounds
inherit ratification"*. D103 lists "a count pinned per file" as one of
three shapes; this is that shape, built inside D103's fence. It is
evidence for that ruling, not a substitute for it.
