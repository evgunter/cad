---
id: viewer-docs-do-not-build-at-wasm32
kind: issue
title: a wasm32 rustdoc pass over viewer is red with seven intra-doc links, ruled host-only, and nothing runs it
status: closed
opened: 2026-09-09
closed: 2026-09-10
---


Found by `viewer-items-unreferenced-at-wasm32`'s lane, checking whether
the `#[cfg(not(target_family = "wasm"))]` that item's fix adds could
break a documentation build. It cannot make things worse than they
already are, and finding that out is what turned up this.

## What

```
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' \
RUSTDOCFLAGS='-D warnings -A rustdoc::private_intra_doc_links' \
cargo doc --no-deps --document-private-items \
  -p viewer --features app --target wasm32-unknown-unknown
```

— `scripts/doc-gate.sh`'s own lint set (`:555`) and pass shape
(`:638`), pointed at the browser target — failed at `1d29a8eeb` with
**nine** `rustdoc::broken_intra_doc_links` errors, rose to ten when
`viewer-items-unreferenced-at-wasm32` closed, and is **back to nine**:
the tenth was `apply_status`'s link to `ViewerApp::deliver_status`, and
that door is deleted (the ruling
`was-the-status-route-supposed-to-fire-for-an-absent-chooser`), so the
link went with the item it pointed at. It reached this row's close at
**seven**: the two `run` links below were de-linked by the closing
unit, for a reason that is not the reason the other seven survive (see
`## Closed`). The seven are read off the command above on the closing
tree, not counted from a diff, and each site below is the `-->` line
rustdoc printed. Every one of them is a doc comment compiled at both
targets linking an item that exists only at the host:

| site | the link | what gates the target away |
|---|---|---|
| `app.rs:41` | `crate::evalseam::ThreadEvaluator` | `evalseam.rs:453` |
| `app.rs:532` | `ThreadEvaluator` | `evalseam.rs:453` |
| `app.rs:552` | `StartupError::Worker` | `app.rs:495` |
| `app.rs:573` | `StartupError::Worker` | `app.rs:495` |
| `evalseam.rs:18` | `ThreadEvaluator` | `evalseam.rs:453` |
| `evalseam.rs:38` | `ThreadEvaluator` | `evalseam.rs:453` |
| `evalseam.rs:74` | `ThreadIndexer` | `evalseam.rs:453` |

Two further links, `app.rs:1901` and `app.rs:1906`, were in the table
when this row was filed and are **not** in it now. They were the sharp
case: they sat in `run_web`'s own doc (`#[cfg(target_family = "wasm")]`,
`app.rs:1918`, with `pub async fn run_web` at `:1919`), both naming
`run`, so they were unresolvable in the only configuration that
compiles the item they document and never rendered at all in the one
that resolves the name. The closing unit de-linked them; `## Closed`
says why that is a different act from de-linking the seven.

## Why it is filed rather than fixed

Nothing runs this pass. `.github/workflows/ci.yml`'s rustdoc gate is
host-target, and its wasm rows are `cargo check` — so this is a
configuration whose docs no row has ever built, and the count is a
ratchet with nobody holding it. The PR closing
`viewer-items-unreferenced-at-wasm32` took it from nine to ten,
deliberately and with the number stated — `apply_status`'s doc is
compiled at both targets and linked `ViewerApp::deliver_status`, which
that PR made host-only, and de-linking a working host link to hold a
count nothing reads would have made the host docs worse for no reader.
The ratchet then fell back to nine, and **that is the shape of the
problem rather than progress on it**: the tenth went because an
unrelated ruling deleted the item it linked, not because anything held
the count.

## The two shapes

1. **Fix the links** — de-link, or re-word, the nine sites so a wasm
   rustdoc pass is clean, and then a row can hold it.
2. **Rule that the viewer's docs are host-only** and say so once, in
   `crates/viewer/README.md`, so the next lane does not re-derive this.

Either way the thing to avoid is the current state, where a reader
cannot tell which it is. Note that (1) alone buys nothing durable
without a row, which is the `-D warnings` argument
`viewer-items-unreferenced-at-wasm32` makes for `check`.

## Closed (2026-09-10)

**Neither shape whole: the honest answer is a split, and the two halves
fall on opposite sides for opposite reasons.**

**Shape (2) for the seven.** `crates/viewer/README.md`'s GQ6 now rules
the viewer's rustdoc a host artifact and permits a doc comment compiled
at both targets to link an item the browser build does not have. The
argument is this row's own, about the tenth link, taken to its limit:
de-linking a working host link to hold a count nothing reads makes the
host docs worse for no reader, and that argument does not stop at the
tenth because it is about a ratio rather than about a site. On one side
is a live link on a page CI builds and a person reads; on the other is a
lint in a configuration whose docs no row has ever built. The ratio is
the same for all seven. `evalseam`'s module doc is the case that forces
it rather than merely permitting it: its whole subject is that the seam
has two arms and one is host-only, so it cannot name the arm it is about
without linking it, and a gate on this pass would forbid that
permanently — the gate making the docs worse every day rather than once.

**And the fence made (1) unavailable as a whole answer anyway.** Its
durable half is a CI row and `.github/workflows/ci.yml` is CIW's. This
lane could have shipped nine edits plus a §6 request; it is not worth
doing, and the paragraph above is why — the request would have asked
CIW to gate a browser-docs page nobody publishes, at a standing cost to
the pages that are published.

**Shape (1) for two, and NOT as a concession.** `app.rs:1901` and
`app.rs:1906` both named `run` from inside `run_web`'s doc. Measured
rather than argued: in the host build `doc/viewer/app/fn.run.html`
exists and `fn.run_web.html` does not; at `wasm32-unknown-unknown` it is
the other way round. So that link resolved in **no** configuration
whatever — the only page that renders it is the one where the target is
absent. That is not a target-posture question at all, it is a plain dead
link, and the ratio that saves the seven does not apply: there is no
reader on either side, and a bracket that no configuration checks spells
a checked claim that is not one. The repair follows a convention the
crate already had, in the other direction: `WINDOW_TITLE`'s doc
(`app.rs:115`) names `run_web` rather than linking it, *"absent from
this configuration, so named rather than linked"*. Both sites are now
bare spans; `app.rs`'s line count did not change, so no citation
anywhere shifted.

**Verified, before and after, with this row's own command.** Nine
errors at `ac4a69dd5`, whose nine `-->` lines matched the filed table
exactly and whose every third-column citation resolved; seven after,
matching the table above. `scripts/doc-gate.sh:555` and `:638` both
still say what this row cites them for. The second half of that script's
pass shape — the `--bins --examples` invocation — adds no site the
command misses, and cannot: `crates/viewer/src/bin/viewer.rs` contains
zero intra-doc links.

**Disclosed rather than fixed, and scheduled**:
`work/view/wasm-only-doc-comments-are-checked-by-nothing.md`. The
ruling's stated cost is that a doc comment on a wasm-only item is now
rendered by no pass this repo runs, so nothing resolves its links — the
two links repaired above were in exactly that class and had been dead
for their whole lives with no checker able to see them. That file also
carries the third shape this row did not offer and the tree does:
`#[cfg_attr(target_family = "wasm", allow(rustdoc::broken_intra_doc_links))]`
on the four items bearing the seven links makes the browser pass green
without weakening the host pass, at the price of a per-site tax forever.
It was costed and refused; the file records the costing so a successor
argues with it rather than re-deriving it.

**What this row got wrong, for the record.** Only its framing, and
mildly: the two shapes were offered as alternatives over one population,
and the population is not homogeneous — two of the nine were a defect on
any posture and seven were not. Everything measurable in it was right at
the SHA it named and still right at close.
