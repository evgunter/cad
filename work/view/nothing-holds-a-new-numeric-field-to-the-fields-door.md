---
id: nothing-holds-a-new-numeric-field-to-the-fields-door
kind: issue
title: nothing holds a new numeric field to the door that gives it a text, so an eleventh DragValue arrives spelling its own precision
status: closed
opened: 2026-09-12
closed: 2026-09-13
---



(VIEW) Residue of
`a-drag-field-renders-a-length-at-a-precision-its-drag-speed-sets`,
disclosed at the moment it was created rather than left in that row's
prose.

## The finding

That unit routed all eleven `egui::DragValue::new` sites in this crate
through one constructor, `crate::widgets::number_field`, which is the
only place `crate::widgets::number_text` is attached. **Nothing holds
the twelfth site to it.** A lane that writes `egui::DragValue::new(v)`
directly gets egui's own precision rule back — the one that renders
40 nm as `0.000` and commits that number when the user clicks into the
field and away again — and nothing in the build says so.

The receipt for today's eleven is a grep
(`grep -rn 'DragValue::new' crates/viewer/src`), and the register's own
rule about that is the reason this row exists: a grep is a claim about
a moment, and the property is *a numeric field in this chrome*, which
a new helper wrapping the widget would not match either.

## Three shapes, none free

- **`clippy::disallowed_methods`** on `egui::DragValue::new`, with the
  door carrying the one `#[allow]`. Cheapest, and the lint already runs
  on every code tier. The cost is that `clippy.toml` is at the
  WORKSPACE root and no program owns it: every other crate's clippy run
  would then carry a rule about a `pub(crate)` function in `viewer`,
  and clippy does not read a per-crate config without shadowing the
  root one (which carries `ignore-interior-mutability`, and `viewer`
  depends on the naming layer that needs it).
- **A source-text guard**, which this repo already has a home for —
  but a new one owes a line in `crates/test-utils/tests/reader_census.rs`
  and must read through `test_utils::source` rather than roll its own
  reader. That is the census working as designed; it is still another
  crate's ledger for a viewer-local rule.
- **`egui::Style::number_formatter`**, set once on the context
  (`egui-0.36.1/src/style.rs:1434` is the default it would replace).
  This is the only shape that also catches an `egui::Slider`, and the
  only one a new site cannot miss. Two costs: it must go through
  `all_styles_mut` rather than `style_mut` or a theme switch drops it
  (`crate::app`'s `apply_polarity` sets a theme PREFERENCE for exactly
  the reason that snapshotting style is wrong), and the crate's widget
  tests build a bare `egui::Context`, so the rule would become
  invisible to every test that does not go through app startup — which
  is all of them today.

The unit took none of them: it took the door, whose whole argument is
that it is visible at the call site and testable without app startup.
Whoever takes this row is choosing between that visibility and a
guarantee, and should say which.

## Settled: the third shape, sited so a test can see it

`crate::widgets::install_number_formatter` writes `number_text` onto
`egui::Style::number_formatter` through `all_styles_mut`, and
`ViewerApp::new` calls it beside `apply_polarity`.

**Why that one.** The other two are claims about TEXT — they detect a
token, and the token they detect is `DragValue::new`. The property is
*a numeric field in this chrome*, and the gap between them is where
this row said the helper case lives. Setting the context default is
not a detection at all: a site that does not deliberately spell its
own `custom_formatter` is already right, so there is no arrival to
notice. That also settles the two costs this row could not:

- **The fork this row framed is false.** Taking the formatter does not
  cost the door: `number_field` keeps `.custom_formatter(number_text)`
  and its whole doc, and every existing test over it is untouched. The
  context default is the FLOOR under a site that misses the door, not
  a replacement for it.
- **The test-invisibility is a siting cost, not the shape's.** It
  follows only from installing the formatter inside app startup. A
  `pub(crate)` function in `widgets.rs` is callable from a bare
  `egui::Context`, so `field_tests` drives an `egui::DragValue::new`
  that has never seen the door and asserts the round trip — a WIDER
  test than any this crate had, since every existing row goes through
  `number_field`. Its control holds the same widget on an
  un-installed context and asserts it still commits 40 nm as zero
  (`a_bare_field_without_the_rule_destroys_the_value`).

**And two things this row got wrong about the tree**, both checked
rather than inherited:

- The second shape's cost is overstated. `reader_census.rs`'s own
  header cedes `scripts/`: its gates read Rust through
  `scripts/gates/lib.sh`'s `gate_rust_code`, *"a second home, in a
  second language, which this row cannot see and does not claim to"*.
  A `scripts/gates/*.sh` guard — the family `viewer-module-kinds.sh`
  and `viewer-vocab-declared-once.sh` already belong to — owes no line
  in another crate's ledger. It was still not taken: a text gate
  cannot see an aliased import, a `Slider`, or a helper that overrides
  the formatter, and it owes `ci.yml` two lines and a planted fixture.
- The third shape catches an `egui::Slider` *in mechanism* — a
  `Slider` renders its value through a `DragValue` of its own
  (`egui-0.36.1/src/widgets/slider.rs:925`, falling through to
  `drag_value.rs:534`) — but there is **no `Slider` anywhere in this
  repository today**. It is a claim about the next one, not about a
  site the other two shapes miss now.

**What is still not held**, said rather than papered over: one line in
`ViewerApp::new`. `ViewerApp::new` takes an `eframe::CreationContext`
and no test in this crate constructs one, which is why nothing tests
`apply_polarity` either. What a test now covers is the RULE — that a
bare field round-trips, and that it survives a theme switch; what it
does not cover is that startup performs the install. A gate for one
call line is a whole `scripts/gates/` member with a `ci.yml` pair and
a fixture, and it would guard a line that sits two lines under the
`apply_polarity` call with the same exposure and no such guard.

