---
id: one-argument-for-outstanding-restated-in-five-places
kind: issue
title: #2055's argument for Outstanding is restated near-verbatim in five in-tree places, 68 lines of prose over 13 of code
status: open
opened: 2026-09-06
---



`session::Outstanding` is twelve lines of declaration over a
seven-line `outstanding()` and a six-line `progress()`. The argument
for it is written out five times in the tree, and two of those five
are near-verbatim of each other:

- `crates/viewer/README.md:335-350` — nineteen lines.
- `crates/viewer/src/session.rs:429-446` — the `Outstanding` doc,
  eighteen lines.
- `crates/viewer/src/session.rs:726-739` — the `outstanding()` doc,
  fourteen lines.
- `crates/viewer/src/frame.rs:1450-1454` — the `progress` doc's new
  paragraph.
- `crates/viewer/tests/frame_policy.rs:1571-1577` and
  `crates/viewer/tests/eval_seam.rs:99-102` — the two test rows'
  headers.

The README and the type doc are the same sentence:

> README:1 Two adjacent `bool`s that mean different things swap
> silently: the swap type-checks, the chrome it produces is plausible,
> and a row that covers the consumer by repeating the same positional
> convention agrees with a swapped call site rather than contradicting
> it.

> session.rs: Two adjacent `bool`s that mean different things swap
> silently: the swap type-checks, the chrome it produces is plausible
> — a spinner where a cancel belongs — and a row that covers the
> consumer by repeating the same positional convention agrees with a
> swapped call site instead of contradicting it.

The clause *"a row that names the states says nothing about which
session state produces which"* appears four times over
(`README.md:349-350`, `session.rs:732-734`, `eval_seam.rs:101-102`,
`frame_policy.rs:1573-1577`). Nothing declares any of these a copy,
so the disclosed-copy vocabulary (`verbatim`, `mirror of`,
`re-derived`) does not find them; only reading the four sites does.

Two consequences, and the second is the one that costs:

- **It is an argument about a shape the tree no longer contains.**
  `crates/viewer/src/session.rs:439-446` defends the enum against a
  pair of `bool`s that no signature takes any more. Implementer
  discipline §4: *"Comments state the invariant, not the history …
  An argument about how the code used to work belongs in the PR
  description."* The invariant here is one sentence — the two reads
  are one three-state fact and a consumer is handed the fact — and it
  is already stated at `session.rs:433-437` before the defence starts.
- **Four copies drift as one edit.** A future unit that changes what
  `Outstanding` means, or that gives the index seam a value of its
  own, has to find all five sites; the README and the type doc are
  the pair most likely to come apart, because the gate that reads
  `crates/viewer/README.md` checks its module tables and not its
  paragraphs.

`crates/viewer/src/frame.rs:1450-1454` is the narrowest instance and
the clearest: *"The two arguments are the two seams and they are
different types, so neither can be given in the other's place"* is a
sentence describing the signature printed directly beneath it.

Filed as a class, not an instance: the same question should be asked
of `Landing` and `AtRestBadge`, whose README paragraph and type docs
sit in the same two files and were written the same way.
