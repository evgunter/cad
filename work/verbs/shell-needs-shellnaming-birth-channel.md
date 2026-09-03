---
id: shell-needs-shellnaming-birth-channel
kind: issue
title: kernel ask from RECIPE-DOORS D5 - shell/shell_open need a ShellNaming birth channel before Node::Shell can exist
status: open
opened: 2026-08-29
github: 1202
refs: [LIB-G17]
---

## From GitHub issue 1202

Opened 2026-08-29; 0 comments.

(LIB orchestrator) Filed per ratified `docs/RECIPE-DOORS-DESIGN.md` D5 (Ev, in-chat, 2026-08-29).

`topo::shell` / `shell_open` (`crates/topo/src/shell.rs:472/:494`) return a bare `Body<T>` — no birth record of any kind (measured: no naming machinery anywhere in the file), where fillet (`FilletNaming`, `sweep/src/fillet/naming.rs:85`), split (`SplitNaming`, `topo/src/splitting/finish.rs:101`) and boolean (`BooleanNaming`, `topo/src/boolean/ops.rs:188`) each write one. A `Node::Shell` without an emitter mints no `StableName`s, which reproduces G16's exact defect one verb over — so the recipe door (G17, LIB-owned) is HELD until this lands.

**The concrete ask** — a `ShellNaming` record in the `FilletNaming` shape, written by the doors themselves:
- per surviving source face: the outer wall face it became (`FromTarget`-analogous rows);
- per source face: its inner offset twin;
- per OPENED face (`shell_open`): the annular rim face minted in its place, keyed to the source `FaceKey` the caller designated;
- the rim's trim edges/feet keyed to the source face's boundary edges, so a selector can name "the rim of the mouth" after a rebuild.

The record types carry arena keys only (attribution to stable names is the document layer's job, per the standing division `MintedDeclaration`'s doc comment states). Exact row set is the kernel's to refine — the list above is what LIB's emitter needs to translate, offered so the ask is concrete rather than a shape the kernel has to guess.

Node payload decided already (D5) so this is the only blocker: `Node::Shell { target, thickness: Expr, open: Vec<StableName> }`, open faces resolved through the N5 ladder to the `FaceKey`s `shell_open` takes — replacing the teapot's by-description plane scan (`demos/tour/src/teapot.rs:417-431`), whose scene note is the recorded friction.

## Home

The ask is kernel-side in `crates/topo/src/shell.rs`, VERBS' `paths:` territory and its shell-verb ground; the recipe door it unblocks is LIB's unit LIB-G17.
