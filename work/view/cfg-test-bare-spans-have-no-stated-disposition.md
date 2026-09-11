---
id: cfg-test-bare-spans-have-no-stated-disposition
kind: issue
title: seven bare spans inside cfg(test) doc comments sit beside ten bracketed links in the same two files, with no rule saying which is right
status: open
opened: 2026-09-11
refs: [named-not-linked-is-a-silent-disposition-at-eleven-of-thirteen-sites, 2332]
---


Re-filed from
`named-not-linked-is-a-silent-disposition-at-eleven-of-thirteen-sites`
(closed 2026-09-11), whose Category A dissolved when Ev ruled the
thirteen feature-axis spans into links. Category B did not: the ruling
is about what rustdoc renders, and nothing renders these.

## The population

Rule: the same path-shaped rule as the closed row — every unbracketed
backtick span in a `///` or `//!` line under `crates/viewer/src` whose
whole content is `<mod>::<path>` — restricted to doc comments inside a
`#[cfg(test)]` module. Read 2026-09-11 at `6891829ee`, unchanged by
`view/link-thirteen`, which deliberately left all seven alone:

| Site | Span |
|---|---|
| `frame.rs:2043` | `pane::viewport` |
| `frame.rs:2275` | `pane::viewport` |
| `frame.rs:2279` | `app::ViewerApp::apply_status` |
| `pane/viewport.rs:525` | `frame::frame_status` |
| `pane/viewport.rs:562` | `frame::frame_status` |
| `pane/viewport.rs:563` | `frame::apply` |
| `pane/viewport.rs:568` | `frame::joined_subject` |

**`frame.rs:2043` is the trap and is worth carrying in the row**: it is
the doc comment ON the test module, so it sits *above* the
`#[cfg(test)]` at `:2045`. Line-wise it reads as production; item-wise
rustdoc renders none of it. A lane sweeping by line number will pick it
up as a fourteenth feature-axis site and be wrong.

## Why it is open rather than inert

`cargo doc` renders no page for any of these, so neither a bracket nor
a bare span can be checked or broken — the closed row called a bracket
here "inert rather than illegal", which is true and is not the same as
settled. **Ten bracketed links sit in `#[cfg(test)]` comments elsewhere
in these same two files**, so the crate does both, and
`frame.rs:2274-2275` spells both inside one comment. A reader has no
rule to apply and neither does a sweep: whichever way the next lane
guesses, it is changing a file to match nothing.

The decision is one sentence — either a `cfg(test)` doc comment links
like any other because it costs nothing, or it names because nothing
renders it — and it belongs in `crates/viewer/README.md` beside the
Rustdoc posture ruling, which is where the rest of this question now
lives. It is not worth a unit of its own; it should ride the next
change that touches either file.
