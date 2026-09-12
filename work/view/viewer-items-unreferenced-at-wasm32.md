---
id: viewer-items-unreferenced-at-wasm32
kind: issue
title: Two viewer items are dead code at wasm32, and they are why the new wasm row cannot deny warnings
status: closed
opened: 2026-09-09
closed: 2026-09-09
branch: view/wasm-dead-items
pr: 2272
---

Found by CIW unit 4 (PR 2263) while adding the first CI row that
compiles this crate for `wasm32-unknown-unknown`. Reported rather than
fixed: `crates/viewer/src/*` is VIEW's and CHROME's, not CIW's.

## What

At `--target wasm32-unknown-unknown --features app`, two items in
`crates/viewer/src/app.rs` are unreferenced and warn under the default
`dead_code` lint:

- `WINDOW_TITLE` (`app.rs:121`). Its only use is `eframe::run_native`'s
  first argument in `run` (`app.rs:1887`), and `run` is
  `#[cfg(not(target_family = "wasm"))]`. The browser build's title is
  the page's, so there is nothing for the constant to do there.
- `ViewerApp::deliver_status` (`app.rs:1097`). Its two call sites
  (`:1250`, `:1267`) are both inside `#[cfg(not(target_family =
  "wasm"))]` blocks — the Open…/Save… arms, which are compiled out on
  wasm because the browser build links no file dialog (#1125's posture:
  the control still shows, disabled, with its reason). It arrived with
  `status-line-writers-bypass-the-ranking` (PR 2026) as the `&mut self`
  shorthand beside `apply_status`.

Neither is a bug at any target this repo ships today. What they are is a
warning that a `-D warnings` row would red on.

## Why it is filed rather than left alone

`.github/workflows/ci.yml`'s `wasm32 check (viewer app feature - the
browser entry point)` is `cargo check` and **not** `-- -D warnings`,
precisely because of these two. Its comment says so at the row. So this
crate's wasm arms are now compiled by CI but are the only viewer rows in
the workflow that cannot fail on a warning — every other viewer row
(`clippy (viewer app feature - eframe + wgpu)`, the app-feature test
row, `clippy (viewer, all features)` in the nightly) denies them.

**The flip is the close condition.** When both items are resolved, that
row can become `cargo clippy -p viewer --features app --target
wasm32-unknown-unknown -- -D warnings` (or keep `check` with
`RUSTFLAGS=-Dwarnings`), and CIW will take that edit on request. Without
this file nothing schedules it, and the asymmetry becomes permanent by
default.

## Two shapes, and this file does not choose between them

1. **`#[cfg(not(target_family = "wasm"))]` on the items themselves**,
   matching the `cfg` already on every one of their users. Cheapest,
   and it says what is true.
2. **A wasm user that was lost.** `deliver_status` is the ranked-status
   shorthand; if the browser build ought to be delivering status for a
   refusal it currently swallows, the missing call is the finding and
   the warning is the symptom. That is a question about
   `status-line-writers-bypass-the-ranking`'s model, which is why this
   is filed on VIEW's slate rather than CHROME's.

`WINDOW_TITLE` is almost certainly (1). `deliver_status` is worth the
look.

## Home

VIEW and CHROME declare identical `paths`
(`crates/viewer/src/*, crates/viewer/tests/*, crates/viewer/README.md`),
so the directory is a judgement and not a derivation: it is here because
shape (2) is a question about a VIEW unit's own model. If CHROME is the
right owner, re-home by moving the file (`work/README.md`).


## Closed (2026-09-09)

**Shape (1) for both, and the argument for `deliver_status` is not the
argument for `WINDOW_TITLE`.** Both items now carry
`#[cfg(not(target_family = "wasm"))]`, matching the `cfg` on every one
of their users. Every citation above is re-derived on the closing tree.

**Reproduced before it was touched.** `cargo check -p viewer --features
app --target wasm32-unknown-unknown` at `1d29a8eeb` gave exactly the
two warnings this file names, at `app.rs:114:7` and `app.rs:1066:8`.

**`WINDOW_TITLE`: the sole-use claim, checked by compiling.** `dead_code`
firing at wasm is a whole-crate reachability verdict, so it settles that
there are zero uses at that target; the `cfg` compiling clean on the host
settles that every host use is inside a `cfg(not(wasm))` region. The one
use is `run`'s `eframe::run_native` argument, and `eframe::WebOptions`
has no title field — the page's `<title>` is what a browser reader sees.

**`deliver_status`: why the missing wasm caller is not a lost one.**
Shape (2) asks whether the browser build swallows a refusal the desktop
build says. Three things answer it, and all three had to hold:

1. **The event does not occur.** Both call sites take
   `frame::dialog_status`'s verdict on a dialog that has just returned.
   The browser links no dialog — `pick_open`/`pick_save` are absent and
   `rfd`'s wasm backend offers only the async form — so there is no
   empty-handed return to report. This door answers a question wasm
   never asks.
2. **The refusal wasm DOES raise is raised earlier and is already
   said.** `frame::chooser_backend()` answers
   `ChooserBackend::Absent` on wasm, `add_enabled(chooser.usable(), …)`
   disables Open…/Save As…, and `.on_disabled_hover_text(
   frame::NO_CHOOSER_BACKEND)` carries the reason — the SAME const
   string `dialog_status`'s `Show` arm would put on the status line.
   #1125's posture is met on the hover route, not the status route, and
   that route is target-blind: a desktop with no zenity and no portal
   gets the identical treatment.
3. **The status-line sentence is unreachable on every target**, which
   this door's own doc already says and this lane re-checked: the same
   `self.chooser` copy gates the button and feeds `dialog_status`, so a
   click implies `usable()` and every reachable verdict at both sites is
   `Keep`. The sentence being "lost" on wasm has never been shown
   anywhere.

**What would have shown shape (2), and what it turned up.** The sweep
rule was: every `cfg(target_family = "wasm")` / `cfg(not(…))` site under
`crates/viewer/src` — counted as lines matching
`#[cfg(…target_family = "wasm"…)]` or `cfg!(target_family = "wasm")`,
which is **35**: 23 in `app.rs` (two of them this PR's), 7 in
`bin/viewer.rs`, 2 in `evalseam.rs`, and one each in `lib.rs`,
`prefs.rs` and `frame.rs` — asked of each: does the browser take a
different arm, is that difference a refusal, and does anything say so.
The rule's blind spot is a platform difference spelled some other way
— a runtime probe, a feature flag, a dependency whose wasm build
differs — which this sweep does not see.

- **eval/index seams.** Inline rather than threaded; nothing is spawned,
  so nothing can refuse to spawn, and `StartupError::Worker` is `cfg`-ed
  away with its producer. No refusal.
- **`run_web` and `WebStartupError`.** Every browser startup refusal is
  typed and printed into the page. Said.
- **The preferences store.** `prefs::Absent`, and this one **is** a
  refusal that goes unsaid: `remember_theme` returns early on
  `!store.usable()`, so the theme picker is offered, applies, and its
  persistence is silently ineffective — against two doc claims in this
  crate that say it is disabled with a reason. Filed as
  `wasm-theme-choice-is-offered-and-silently-not-kept`. It does **not**
  give this door a wasm caller: `remember_theme`'s reporting route is a
  direct `notices.push`, and the fix shape is chrome, not a status
  write. So shape (2) is real in this crate and it is not here.

**The flip, proved but not made.** `.github/workflows/ci.yml` is CIW's
and is untouched. Both candidate spellings were run at the wasm target
on the closing tree and both are clean:

```
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' \
  cargo clippy -p viewer --features app --target wasm32-unknown-unknown -- -D warnings
RUSTFLAGS='--cfg getrandom_backend="wasm_js" -Dwarnings' \
  cargo check -p viewer --features app --target wasm32-unknown-unknown
```

The negative control is what makes that evidence rather than a claim:
the same clippy command on the unfixed tree exits 101 with those two
warnings as errors and nothing else. **Do not add `--all-targets`** to
the row — `crates/viewer/tests/` reaches `ThreadEvaluator`, which is
host-only, so the test targets do not compile at wasm and never have.

**The class, and the rule that produces it.** The population is *every
item under `crates/viewer/src` that the flip's own lint set reports as
unreferenced at `--target wasm32-unknown-unknown --features app`*, and
it is **exactly two** — read off the negative-control run above, not off
a grep, because `dead_code` is a reachability analysis over the compiled
configuration and a grep is not. Two blind spots, both in the rule: it
cannot see a `pub` item with no wasm reader (the crate root keeps it
alive), and it is lint-set-relative, so it moves when the lint set does.

**Residue.** `viewer-docs-do-not-build-at-wasm32` — a rustdoc pass at
the browser target was already red with nine unresolved intra-doc links
at `1d29a8eeb`, and this PR makes it ten, deliberately and with the
number stated.
