---
id: viewer-doc-link-crosses-the-app-feature-gate
kind: issue
title: session/op.rs links crate::widgets::drag_gesture_ops across the app feature gate — the rustdoc gate is RED on main at the code tier
status: open
opened: 2026-09-11
---


Filed by the FIX orchestrator, from the
`compile-fail-blocks-without-error-codes` lane (PR 2335), whose own
diff reaches only `crates/quantity/`. Reported rather than absorbed:
the lane that caused it owes the fix, and FIX does not take another
program's debt in passing.

## The defect

`crates/viewer/src/session/op.rs:773` documents a cancel door with

```rust
/// opened it ([`crate::widgets::drag_gesture_ops`]'s `drag_stopped`
```

`pub mod session` is **ungated** (`crates/viewer/src/lib.rs:75`);
`pub mod widgets` is `#[cfg(feature = "app")]` (`lib.rs:93`). At
DEFAULT features — which is the configuration the doc gate's viewer
pass takes — the link target does not exist, so rustdoc raises
`unresolved link` and, under `-D rustdoc::broken-intra-doc-links`,
`could not document 'viewer'`. That reddens
`rustfmt + rustdoc (gate) + wasm32` and, through it, `gate ok`.

Reproduced on `main`'s own tree, not on the PR's: both the link and
the `cfg` are on `main` today at `9eed52dd`, and nothing in PR 2335's
diff is in `crates/viewer`.

## Why it survived a week

It arrived in `c1ea73a2f` ("view: a cancel door for both gesture ops",
PR #2320) and **every push to `main` since has classified below the
code tier**, so the job that reads the link has been `skipped` on each
of them — runs `34560796333` (`bc4cd8dd0`), `34561272106`
(`81a807c23`) and `34561757424` (`e222b2ada`). PR 2335's run is the
first code-tier run over it.

That is the silent-coverage class again, in the face that costs the
most: not a green job over a skipped step, but a **sequence of green
`main` pushes none of which ran the step at all**. A red that only a
code-tier change can surface is invisible for exactly as long as the
repo happens to be landing docs.

## Scope of the cost

`gate ok` is red for **every code-tier PR in the repo** until this
lands — not only VIEW's. Under Ev's 2026-08-31 ruling an inherited red
does not block a merge once it is annotated with its issue, so other
programs can keep landing; but each of them pays a red run and an
annotation, and a genuine failure now has a red to hide behind.

## Shape of the fix (VIEW's call, not FIX's)

Three obvious spellings, and the choice is a real one because
`drag_gesture_ops` is `pub(crate)`, so even under `--features app` a
public doc comment linking it is a `private_intra_doc_links` question
rather than a clean reference:

1. gate the sentence — `#[cfg_attr(not(feature = "app"), doc = "…")]`
   or drop the link to plain text at default features;
2. name it without linking (backticks, not brackets), which is what
   the `pub(crate)` visibility already argues for;
3. move the shared drag vocabulary out from behind the `app` gate.

Whoever takes it owes the other direction too: a sweep for the class —
an ungated module's doc linking a gated one — since nothing mechanical
reads feature gates against intra-doc links today.
