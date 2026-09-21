---
id: renderer-free-link-to-editor-core-opens-the-ungated-hole
kind: issue
title: the renderer-free half now links into editor-core, which is not a toolkit seed, so the ungated-link hole is open rather than theoretical
status: open
opened: 2026-09-21
---



## Finding

Found by VDOC's `vdoc/readme-attributions` lane while re-deriving
`crates/viewer/README.md`'s cross-crate-link census by subject. Filed
here rather than added to
`renderer-free-cross-crate-links-are-ungated-off-the-seed-set` because
`work/vdoc/program.md`'s `keep_out` forbids this program editing
another program's item files; that row is the parent and this is the
instance it predicted.

**That row says the hole is empty today.** It is not, as of this tree.
Its own sweep rule, run over `crates/viewer/src` on `main`:

    rg -n -g '!{app,drafts,forms,gpu,widgets,pane}.rs' -g '!pane/**' -g '!bin/**' -e '(\[`|\]\()(pncad|bvh|editor_core|toml)::' crates/viewer/src

gives **15** sites, not twelve. Fourteen target `pncad`. The fifteenth
is `crates/viewer/src/session/refuse.rs:151`, in `Refusal::NoSuchParam`'s
doc comment:

    /// renders — [`editor_core::edit::UNDECLARED_PARAM_RECOURSE`], its

The target is `crates/editor-core/src/edit.rs:545`.

## Why that opens the hole

`VIEWER_TOOLKIT_SEEDS` is `{"viewer", "pncad", "bvh"}`
(`scripts/ci-filter.py:1436`) and `RUN_VIEWER_TOOLKIT` is
`"true" if seeds & VIEWER_TOOLKIT_SEEDS else "false"`
(`scripts/ci-filter.py:2495`). `editor-core` is in none of it. So a
branch that renames or deletes `UNDECLARED_PARAM_RECOURSE`:

- seeds `editor-core`, so `RUN_VIEWER_TOOLKIT` is `false`;
- still carries `viewer` in `cargo_scope`, since `viewer` is in the
  dependent closure;
- takes `scripts/doc-gate.sh --skip-viewer-toolkit` at `ci.yml`'s
  `rustdoc (gate)` step (`ci.yml:1890-1894`), where the link lint is
  inert for this crate;
- and is backstopped by nothing, because `nightly.yml:291-293`'s
  `rustdoc (viewer, all features)` sets no `RUSTDOCFLAGS` and exits 0
  over an unresolved link — measured by the parent row, by planting one.

The break would reach `main` unreported and would then be invisible
until some later branch happened to seed the toolkit.

## What moved in the population

Beside the new member, two things the parent row's table no longer
describes:

- `sketch.rs:939`'s reference-form link
  `` [`ProfileVertex`](pncad::profile::ProfileVertex) `` is gone; the
  site is now the shortcut `` [`ProfileVertex`] `` at
  `crates/viewer/src/sketch.rs:1315`, which resolves into `pncad`
  through a `use`. That is the parent row's own stated blind spot, and
  it means the population has one member no path-shaped sweep can see
  where it used to have one it could.
- three sites arrived that the twelve-row table does not have:
  `frame.rs:1752`, `idpass.rs:157` and `session/op.rs:1219`.

## Disposition

The three fix shapes the parent row costs are unchanged and none of
them is VDOC's; shape (1), a gate on the sweep rule, is the one that
closes this at its cause. What VDOC did on its own ground was correct
the README, which now states the rule as the command above and says
the hole is open — `crates/viewer/README.md`, the **Nowhere** bullet
of the rustdoc-posture section.

## Confidence

`sure` that the link exists and that `editor-core` is not a seed, both
read off the tree. `likely` on the CI consequence, which is read from
`ci.yml` and `ci-filter.py` rather than measured by pushing a branch
that renames the constant.
