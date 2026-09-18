---
id: vocabulary-macro-bodies-are-outside-rustfmt
kind: issue
title: rustfmt does not reach inside a macro_rules! invocation, so nine viewer enums are formatted by hand
status: open
opened: 2026-09-06
---


Filed by the `const-all` unit as its own disclosed cost, at the moment
its style review made the cost measurable (PR 2046, finding S4).

## The fact

`crates/viewer/src/vocab.rs`'s `vocabulary!` declares nine enums —
`PathVerb` (17 variants), `Seat` (9), `ToolKind` (7), `ArcMode` (6),
`DatumKind` (4), `ShapeKind` (3), and three two-variant choices — and
**rustfmt formats none of their bodies**. Every variant and every
variant doc in those blocks is indented by hand.

Demonstrated, not inferred. Re-indenting `Chamfer = "chamfer"` to
column 21 inside `crates/viewer/src/blend.rs`'s `vocabulary!` block
leaves `cargo fmt --check` at exit 0. The identical mis-indent applied
to `BlendError`'s first variant, an ordinary enum ten lines below it in
the same file, is caught:

    Diff in crates/viewer/src/blend.rs:173:
    -            /// No edges are held, so there is nothing to blend.
    +    /// No edges are held, so there is nothing to blend.

## The fix that does not work, tested

The dispatching idea was that rustfmt formats a `macro_rules!`
invocation whose body PARSES as Rust, and that `pub const ALL;` is what
stops it — so moving the `ALL` declaration onto the enum as an
attribute (`#[all(pub const ALL)]`) would leave the body as a single
well-formed item.

**Refuted, two ways.** With the body rewritten to exactly that shape —
one attribute-carrying `enum` item and nothing else — a deliberate
mis-indent still leaves `cargo fmt --check` at exit 0. Delimiting the
invocation with `()` instead of `{}` does not reach it either. It is
the macro INVOCATION that rustfmt declines, not the unparseable body:
rustfmt does not format the token stream of a `macro_rules!` call in
item position, and no rearrangement of what is inside the braces
changes that.

So this is not a defect in how the macro was written. It is the price
of the construction, and the same price
`crates/profile/src/path/program.rs`'s `arc_modes!` and
`transition_table!` already pay for the arc-mode and verb
vocabularies — a fact worth checking when this is taken, since the
kernel side has been paying it longer and may have found something.

## Answers on the table

- **Accept it and say so** — what the unit did. `src/vocab.rs`'s module
  doc and `crates/viewer/README.md`'s **Closed vocabularies are
  declared once** both carry the cost, so a reader of either meets it.
  The exposure is bounded: the bodies are variant lists and doc
  comments, the shape a reviewer reads most easily by eye.
- **A gate**: a scan that reads the invocation bodies and checks
  indentation against a fixed rule. Small, and re-implements a corner
  of rustfmt badly.
- **Un-convert.** Only if the formatting loss turns out to cost more
  than the membership guarantee bought, which it has not yet.
- **Upstream.** `rustfmt` formats macro invocations in some positions;
  whether item-position `macro_rules!` calls could be included is a
  question for rustfmt, not for this repo. Recorded as the honest
  ceiling on the other three.

## Scope

Nine enums, all in `crates/viewer/src`: `forms.rs` (five), `tools.rs`,
`seats.rs`, `combine.rs`, `blend.rs`. Any future `vocabulary!` site
joins them.
