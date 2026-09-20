---
id: viewer-tests-spell-one-millimetre-extrude-literal-six-times
kind: issue
title: One literal_with_unit extrude distance written out six times in crates/viewer/tests
status: open
opened: 2026-09-20
---


## Finding

- **Where**: `Expr::literal_with_unit(0.008, Dimension::Length,
  MM.def())` — one value with one notation, **six** byte-identical
  sites in two files: `crates/viewer/tests/panel_display.rs` at five
  sites and `crates/viewer/tests/valid_range.rs` at one. Every one is
  an extrude `distance:`.
- **Why the door beside it does not serve them**: `common::len` lowers
  canonically and carries no notation, which is exactly what these
  rows are about — the unit a literal REMEMBERS. So the home is a
  second door (`common::mm(0.008)`, or a `len_in(value, unit)`), not
  `len`, and deciding which is the work. `panel_display` also holds two
  further unit-carrying literals that are not this value
  (`literal_with_unit` at seven sites in that file, of which five are
  this one).
- **What makes it a duplicate and the `Expr::literal` sites folded
  beside it not**: those were a call spelled out where a door existed;
  these are a call AND a constant AND a notation choice agreeing
  three ways across two files, with no door at all.
- **Importance**: low-medium. The value decides what the panel renders,
  so a row reading a rendered string is watching it; six copies
  agreeing today is six places to change.
- **Instrument, and its blind spot**: `git grep -n 'literal_with_unit'`
  over every tracked file, no path argument, bucketed by argument
  triple. It is literal and line-shaped: it misses a call whose
  arguments wrap (measured: one does, `panel_display.rs:359`, and it is
  a DIFFERENT value, so it is not a member), the same value written
  `8e-3` or `0.0080`, and any unit door reached by another name. The
  wider class certainly runs past this crate — the same grep names
  `crates/editor-core/tests/switch_display_units.rs` at seven sites —
  and that is a different crate's home question, not censused here.
- **Raised by**: the S-DUP lane closing the four viewer-suite door rows,
  2026-09-20, measured at `cd9fdfd6b`. It is the `Expr::literal` row's
  own stated blind spot (`Expr::literal_with_unit`), run as a second
  instrument rather than published as a caveat.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner to
file it with, and the subject — one construction spelled more than once
— is S-DUP's charter. Any of the five may claim it by `git mv`.

