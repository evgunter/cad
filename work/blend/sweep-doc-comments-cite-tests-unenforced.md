---
id: sweep-doc-comments-cite-tests-unenforced
kind: unit
title: Sixteen doc comments in sweep name a test file as their evidence, and nothing enforces any of them
status: review
opened: 2026-09-04
branch: blend/5-doc-citations
pr: 2155
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

## Claimed by BLEND (2026-09-06)

Moved from `work/code-quality/` to `work/blend/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, `track:` letter and body unchanged. Track T residue on `crates/sweep/src/*`; the `emit_blend.rs` site is EVAL's and announced.


## Landed (BLEND unit 5)

Every doc- or code-comment citation of a `sweep` test row in
`crates/sweep/src/**` now carries ONE spelling — the one
`cargo test -p sweep --test all -- <filter>` actually resolves, because
`tests/all.rs` mounts each suite as a module of one binary:

```
<module>::<row_name>
```

Measured, on this tree: the path spelling
`crates/sweep/tests/m7_skin_integral.rs::the_uniform_loft_is_bitwise_unchanged`
selects **0** tests; `m7_skin_integral::the_uniform_loft_is_bitwise_unchanged`
selects **1**. A path-shaped citation is grep-resolvable and
cargo-inert, so it is not the spelling.

**The instrument is a row, not a gate.**
`crates/sweep/tests/review_blend5_r5_probes.rs::every_test_citation_in_the_sweep_docs_resolves_to_a_test_row`
reads every `<module>::<row>` in the prose of `crates/sweep/src/**`
(through `test_utils::source::comments_only`), and requires that
`crates/sweep/tests/<module>.rs` declare `<row>` as a `#[test]`. A
rename or a deletion of a cited row turns it red; a helper name in a
citation turns it red too (it caught one when it landed,
`review_blend6_r1_probes::seeds`). Its floor asserts the corpus is
non-empty: 45 citations at the landing SHA. Nothing is filed for GATES.

What it does NOT buy, stated: a row that still exists and asserts LESS
than the sentence says resolves green. That half needs a reader, and
this unit found four instances of it by reading. The corpus is
`crates/sweep/src/**` citing `crates/sweep/tests/**` only; a citation to
another crate's suite (`admit.rs` naming a `test-utils` row) is outside
it and the row says so. `work/code-quality/doc-line-citations-rot-silently.md`
is the sibling class for `file:line` citations.
