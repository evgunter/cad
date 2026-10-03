---
id: a-doc-comment-names-a-test-row-and-nothing-checks-it-exists
kind: issue
title: a bare backtick name in a doc comment is checked by nothing; 35 spans over 29 names in viewer
status: open
opened: 2026-09-16
priority: P4
cost: E
---



Filed by `a-supersession-outlives-its-own-frame` as it closed, because
that unit's whole deliverable is a lifetime claim made traceable by
naming the five rows that hold it — five new members of this class.

## The class, and why the two existing items do not cover it

`doc-comments-name-symbols-that-do-not-exist` (closed) and
`comment-symbol-names-outside-rustdocs-reach-have-no-gate` (open) both
range over spans of the shape `<mod>::<path>`. This class is a span
with no `::` in it at all: a bare `snake_case` identifier in backticks
inside a `///` or `//!` comment, naming a **test row** — the row that
holds the sentence the comment is making.

The closed item calls prose-with-no-code-span its blind spot 2 and
says honestly that no gate proposed there covers it. That is true of
prose; it is not true of this shape. A bare name in backticks IS a
code span, it is mechanically recognisable, and the population below
was produced by a rule.

It is unreachable by rustdoc for a different reason from either
sibling: bracketing would not help. A `#[cfg(test)]` row and a row in
`crates/viewer/tests/` are both invisible to every rustdoc pass this
repo runs, so `[`a_hover_only_batch_leaves_the_status_line_alone`]`
would be a broken intra-doc link rather than a checked one. The bare
backtick is the correct spelling today and is checked by nothing.

## The population, and the rule that produces it

Rule: every line under `crates/viewer/src` whose first non-space
characters are `///` or `//!`, scanned for backtick code spans whose
whole content matches `[a-z][a-z0-9]*(_[a-z0-9]+){3,}` — a lower-snake
identifier of at least four segments, which is what this crate's test
names look like and what its function names mostly do not. Each
distinct name is then resolved against `fn <name>` anywhere under
`crates/viewer`.

**Read on `view/supersession-lifetime` at `d92846f5b1`: 35 spans over
29 distinct names. Twenty-six resolve to an `fn` under
`crates/viewer`; the three that do not are foreign APIs the rule
admits and should not** — `copy_texture_to_buffer` (wgpu),
`on_disabled_hover_text` (egui) and `len_without_is_empty` (a clippy
lint). So the class is a MISSING GATE and not a live defect: every
name that is this crate's own exists today, and nothing is holding it
that way. The three foreign hits are also the measure of what a gate
would cost to site — it needs a way to say "not ours" that is not
"undefined", which is the same decidability question
`doc-comments-name-symbols-that-do-not-exist` answers by restricting
`<mod>` to the crate's own module list. A bare name carries no module,
so that answer is not available here and a gate would need an
allowlist or a per-crate resolver.

Not every one of the 26 is a test row: `permitted_during_free_move`,
`permitted_during_value_gesture` and `clear_for_new_document` are
production methods named the same way. That does not narrow the class,
it widens it — the defect is a name in a doc comment that no pass
reads, and the resolver answers for a method exactly as it does for a
row.

Spans per file, which is the population of record for this row:
`frame.rs` 10; `readout.rs` 4; `session.rs` 3; `session/op.rs` 3;
`pane/viewport.rs` 2; `scene.rs` 2; `widgets.rs` 2; and one each in
`blend.rs`, `display.rs`, `gpu.rs`, `history.rs`, `pane/view.rs`,
`platform.rs`, `prefs.rs`, `seats.rs`, `vocab.rs`. Five of `frame.rs`'s
ten arrived with this row's parent unit.

## What the four-segment threshold cannot match

A name of **three** segments or fewer. Three of the resolving names sit
exactly on the boundary at four — `clear_for_new_document`,
`delta_not_a_number`, `fits_and_reads_back` — so the threshold is a
convenience rather than a property, and one shorter name would be
silently outside the population. Lowering it admits ordinary method
names (`permitted_during_free_move`, already in the list, is a
production `fn`), which is why the resolver has to run over the whole
crate rather than over the test targets: the rule cannot tell a row
from a method, and does not need to, because both are `fn`s that
either exist or do not.

It also cannot match a name spelled without backticks, or one split
across two `///` lines. The split case is the parent item's blind spot
3 and is empty for `<mod>::<path>` spans; it was not re-measured for
this shape.

## What a gate would cost

Less than either sibling's. The resolver is `fn <name>` over
`crates/viewer`, with no path resolution and no rustdoc JSON, because
the names have no path in them. The open question is the same one:
where it is sited in `scripts/doc-gate.sh`, and whether the crate or
the workspace is its subject.
