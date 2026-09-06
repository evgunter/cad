---
id: facade-guard-file-keeps-two-line-local-readers
kind: issue
title: Three readers in the façade's guard file still read a statement through a line
status: open
opened: 2026-09-06
---


Swept out of `lb13-guards-are-line-local`'s class — "every future
scanner in this file that reads a statement instead of a line" — while
LIB-MECH2 converted that issue's two named guards. Three readers in
`crates/pncad/tests/all.rs` are still line-local, and none was
converted because none of the three conversions is mechanical the way
the guards' was.

The sweep that found them: every `.lines()` in the file (nine sites),
each read for whether the thing it is reading is a Rust STATEMENT. The
blind spot of that pattern is a reader that splits on `\n` by hand or
walks bytes counting newlines — there is none in this file today, and
nothing stops one arriving.

(The id says "two" and the item names three: the third was found on a
re-sweep after the file was filed, and ids are stable.)

## 1. The U1 guard's check 1 reads the crate root off the `use` line

`this_file_reaches_the_kernel_only_through_pncad`, check 1: it trims
each line, takes the text after `use `, and reads the path root from
that same line. A `use` whose root is on a continuation line —

    use
        serde::Deserialize;

— is not seen at all. Its blind spot is NARROW rather than absent:
check 2 of the same test scans the whole comment-stripped file for
each of the twelve kernel crate names as a path root, so any KERNEL
path reaches a violation however it is wrapped. What escapes both is a
non-kernel, non-`std` root written on a continuation line.

**Why not converted here.** The statement accumulation LIB-MECH2 added
(`pub_use_statements`) searches the comment-stripped text and does not
know a string literal from code. That is safe for `pub use`, which
does not appear as a literal in this file, and unsafe for `use `,
which now does: the reader selftest
`the_boundary_readers_read_across_line_breaks` builds a synthetic
`pub use …` snippet as a string, and a text search for `use ` finds it
inside that literal and reads a root of `{layer}`. Converting check 1
therefore needs a reader that knows literals — `test_utils::source`'s
`code_only` view is exactly that, and this file is already carried in
`crates/test-utils/tests/reader_census.rs` as
`Unconverted("Track E, issue #763")`, so the honest fix here is the
conversion that entry already owes rather than a second hand-rolled
special case.

## 2. `root_declared_pub_names` reads the declaration off the `pub` line

It requires `pub <keyword> <name>` to open a line at column 0, so a
declaration whose name wrapped (`pub struct` on one line, the name on
the next) is not counted. Column 0 is the function's whole scope rule
— that is what tells a root item from one nested in a `mod` block — so
this cannot be converted by accumulating to a terminator without
re-deciding how the scope test is made. No live instance: rustfmt does
not write that shape, and both roots this reader is applied to are
plain.

## 3. `code_without_cfg_gated` detects the attribute on one line

It treats a line whose trimmed start is `#[cfg(` as the opening of a
gated item, so a `#[cfg(…)]` whose own parenthesis list wrapped is not
seen as a gate at all and the item it guards survives into the view.
Unlike 1 and 2 this limit is DECLARED at the site — its doc says
"Line-based and deliberately shallow … exactly the shape both roots
use" — so it is recorded here for completeness of the class rather
than as a hidden blind spot. rustfmt does not wrap the two short
`#[cfg(feature = "…")]` attributes the two roots actually carry.

## Disposition

None has a live vehicle today and all three are negative-claim
readers, so they are false-GREEN risks rather than false-alarm ones —
which is why they are written down rather than left to a sweep. Item 1
is best taken with the Track E conversion of this file; item 2 is
worth re-deciding only if the column-0 rule is ever relaxed; item 3
would follow item 1's reader for free and is not worth its own pass.
