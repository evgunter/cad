---
id: viewer-docs-do-not-build-at-wasm32
kind: issue
title: a wasm32 rustdoc pass over viewer is red by construction on host-only links, and nothing runs it
status: closed
opened: 2026-09-09
closed: 2026-09-10
pr: 2288
branch: view/wasm-docs
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

**Neither shape whole, and the shape that wins was already ruled one
axis over.** `scripts/doc-gate.sh:243-262` records the isomorphic
problem on the FEATURE axis: with F off, every link into F-gated code is
unresolvable BY CONSTRUCTION, and the answer is to allow
`rustdoc::broken_intra_doc_links` for that pass only
(`RUSTDOC_LINTS_INERT`, `:559`), with the cost stated at the site as the
cost of the widening, the population enumerated complete, and line
numbers deliberately not carried. The target axis is the same claim with
`target_family` for `feature`, so it gets the same answer:
`crates/viewer/README.md`'s **Rustdoc posture** ruling. Shape (2)'s
instinct was right and its ARGUMENT was wrong — the point is not that
the browser docs have no reader, it is that the lint cannot tell *this
link is broken* from *this link's target is in the other half*.

**The seven are a cost, not a defect, and the gate is the host pass.**
It runs at `-D warnings` and holds every page it renders, which also
covers the case no by-construction argument reaches: a link resolving at
NEITHER target reds on a host page.

**Shape (1) for two, and not as a concession.** `app.rs`'s two `run`
links sat in `run_web`'s doc. Measured: the host build with
`--features app` renders `app/fn.run.html` and **no**
`app/fn.run_web.html`; at `wasm32-unknown-unknown` it is the reverse. So
that link resolved on no page any pass renders — a plain dead link, not
a target-posture question. Repaired by following the crate's own
convention in the other direction, `WINDOW_TITLE`'s
*"absent from this configuration, so named rather than linked"*. Both
edits are within the line, so `app.rs` is 1,956 lines before and after
and no citation shifted.

**The classifier this row's first close shipped was a proxy, and it is
replaced.** It said to read the `cfg` on the item the doc comment is
attached to. `WebStartupError` is `cfg(target_family = "wasm")` and its
five variant doc comments carry **no `cfg` of their own**, so that test
reads them as unconditional and calls their brackets permitted when they
are the defect class — #2278's shape again, a rule ranging over a proxy
where the claim ranges over a fact. The test is now page existence in
rustdoc's own output, which is the same reason this row already gave for
ranging over the pass's output rather than over attributes, applied to
both halves of the rule instead of one. It also dissolves the missing
third case: `app.rs:41` sits on `pub mod app`, `cfg(feature = "app")` at
`lib.rs:82`, which is neither of the two cases the old rule named and is
decided correctly by asking whether the host pass renders its page.

**Where the population lives now.** The README owns it, dated, by
identifier and file, with **no line numbers**, per doc-gate's own stated
reason. The table above is the finding-time reading at `ac4a69dd5` and is
kept as that rather than as a second live copy — the same way doc-gate
records the reading it superseded. The row's title carries no count,
because `work/STATUS.md` renders titles and a re-measure should not have
to edit one.

**Verified, before and after.** Nine errors at `ac4a69dd5`, whose nine
`-->` lines matched the filed table cell for cell and whose every
third-column citation resolved; seven after. `scripts/doc-gate.sh:555`
and `:638` still say what this row cites them for.

**And the `--bins --examples` claim in `## What` above is wrong, twice.**
That half of doc-gate's pass shape does not merely add no link site — at
`wasm32-unknown-unknown` it does not COMPILE: `error[E0432]: unresolved
import viewer::evalseam::ThreadEvaluator` at
`crates/viewer/examples/r1_e2e.rs:19:37`, in one of two files under
`crates/viewer/examples/` that this row never mentions. With the link
lint denied it aborts at four sites, not seven, because the private-`fn`
sites are not reached; with the lint allowed the lib-only pass is clean
and this half is the only red. The conclusion that it adds no site
survives; the reason given for it — `bin/viewer.rs` has no intra-doc
links — was an argument about the wrong half, and "cannot" was too
strong.

**Disclosed rather than fixed, and scheduled**:
`work/view/wasm-only-doc-comments-are-checked-by-nothing.md`, which now
carries the priced intermediate as its recommended shape.

**What this row got wrong, for the record.** Its framing offered two
shapes over one population when the population is not homogeneous. More
than that, both shapes were reasoned from scratch: the repo had already
answered this question on the feature axis, and neither the row nor its
first close cited it. Everything measurable in the row was right at the
SHA it named except the `--bins --examples` sentence above.
