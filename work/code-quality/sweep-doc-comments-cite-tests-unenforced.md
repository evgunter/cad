---
id: sweep-doc-comments-cite-tests-unenforced
kind: issue
title: Sixteen doc comments in sweep name a test file as their evidence, and nothing enforces any of them
status: open
opened: 2026-09-04
---


## Finding

A doc comment that names a test file as the check its claim rests on is a
citation nothing resolves: the file can be renamed, the row deleted, or the
assertion narrowed, and the sentence stays exactly as true-looking as it was.
`crates/sweep/src` carries about sixteen — `skin.rs:159`, `:593`;
`blend/mod.rs:401,416,432,453,485,534,561,1195`; `blend/surgery.rs:1762`; and
four in `blend/naming.rs`.

**This is not hypothetical: the class produced a live disagreement inside one
file on the day it was found.** `blend/naming.rs` cited the same coverage
three times at three different scopes — one fixture at `:32`, two in the
paragraph `D324` rewrote, and "all three surgery shapes" at `Retired`. The
narrowest was the untouched one, which is `D324`'s own recorded shape (*"the
diff rewrote the paragraphs immediately above and below and left this one"*).
Across the fence the same rule holds asymmetrically:
`crates/editor-core/src/names/emit_blend.rs:266-267` names one fixture where
the kernel names two.

The module also has two spellings of the citation itself — `blend/mod.rs:416`,
`:453` and `:485` cite `file.rs::test_row_name`, which at least points at the
assertion and survives a file gaining unrelated rows; others name a bare path.

**What would make it mechanical is the open question**, and it is why this is
a finding rather than a fix: a lint that resolves `sweep/tests/<file>.rs` and
a row name is cheap and would catch a rename or a deletion, but nothing can
check that the row still asserts what the sentence says it asserts. Whoever
takes this decides whether the enforceable half is worth an instrument, or
whether the honest remedy is to stop citing rows in prose and let the test
names carry it.

Track T's fence (`crates/sweep/`) covers the census; the `editor-core` site is
Track V's.

## Was

`unrowed` — raised by the T-2 style review (2026-09-04) as the class behind
its `naming.rs:32` finding.
