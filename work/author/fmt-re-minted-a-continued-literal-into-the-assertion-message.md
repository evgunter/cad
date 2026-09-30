---
id: fmt-re-minted-a-continued-literal-into-the-assertion-message
kind: issue
title: A backslash-continued assertion message reaches the reader with two 14-space runs in it
status: open
opened: 2026-09-21
---



Found by the `the-two-drags-name-their-gestures-in-two-shapes` lane
(VSEAM, PR #2965), whose own whitespace-run check over
`crates/viewer/{src,tests}` returns exactly this one hit and nothing
else. Filed rather than fixed: the line is AUTH-2's and the fence
holds.

## What it is

`crates/viewer/src/widgets.rs`, in
`the_second_hand_over_of_one_typed_text_changes_nothing`'s first
`assert_eq!`, the message argument. The string a failing run prints is:

> the widget hands the text over twice; if it stops, this row
> `<14 spaces>` and the guard behind it are answering a question
> nobody asks: `<14 spaces>` {emitted:?}

Two runs of **14 spaces**, at offsets 72 and 147 of the literal,
measured with a `  +` scan over the line rather than counted by eye.

## Why it happened, and why it is a class rather than a typo

The message was written as a `\`-continued literal across three source
lines. A backslash continuation eats the newline and the *leading*
whitespace of the next line, but `cargo fmt` re-indents the
continuation lines first, and the run it leaves is whatever the new
indent is. The lane register's standing rule says exactly this — *use
short one-line literals* — and the check it prescribes is
`grep '[a-z,—] \{6,\}[a-z]'` over `crates/viewer/{src,tests}`.

It was already collapsed at the commit that introduced it
(`4db2d821e5`, *AUTH-2 fix pass: the no-op guard compares text*): the
diff adds the line in its re-minted form, so nothing about the author's
own working tree would have shown it. That is the whole reason the rule
exists as a grep rather than as a habit.

## The repair

Shorten the message to one line, or break it into two `assert_eq!`
messages. Nothing about the assertion's logic moves; what changes is
what a reader of a failure sees.

## Home

`crates/viewer/src/widgets.rs` is CHROME's, VGEOM's and VIEW's by
territory, not `author`'s — `work/author/program.md`'s `paths` does not
list it. The row is filed here because the line is AUTH-2's own and
this is the program that will recognise it; re-home it if that reads
wrong.
