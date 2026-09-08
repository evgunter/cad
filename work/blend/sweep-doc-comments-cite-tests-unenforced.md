---
id: sweep-doc-comments-cite-tests-unenforced
kind: unit
title: Sixteen doc comments in sweep name a test file as their evidence, and nothing enforces any of them
status: dispatched
opened: 2026-09-04
branch: blend/5-doc-citations
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

Every doc- or code-comment citation of a `sweep` test in
`crates/sweep/src/**` now carries ONE spelling, the one a `cargo test`
filter resolves:

```
crates/sweep/tests/<file>.rs::<row_name>
```

The corpus that spelling defines, and the grep that finds all of it:

```
rg -n 'crates/sweep/tests/[a-z0-9_]+\.rs::[a-z0-9_]+' crates/sweep/src
```

29 hits at the landing SHA, across eight files (`skin.rs` 2,
`blend/mod.rs` 13, `blend/surgery.rs` 5, `blend/open/ruled.rs` 3,
`blend/naming.rs` 2, `extrude.rs` 2, `blend/battery.rs` 1,
`test_support.rs` 1). The complement — a citation naming a test file in
any OTHER shape — is empty, and stays checkable with:

```
rg -n 'tests/[a-z0-9_]+\.rs' crates/sweep/src | grep -v 'crates/sweep/tests/[a-z0-9_]*\.rs::'
```

What the spelling does NOT buy is the instrument: nothing resolves the
row name to a function, so a rename or a deletion still leaves the
sentence exactly as true-looking as it was. The unit found one of each
already rotted (`review_fillet_e2_probes.rs`'s line-ring row renamed;
`review_pr12_probes.rs` cited for a claim only a printing probe
reaches). That gate is GATES' ground — `scripts/gates/*`, resolving the
citation to an existing `#[test] fn` — and is filed, not built here. It
catches renames and deletions and cannot catch a narrowed assertion.
`work/code-quality/doc-line-citations-rot-silently.md` is the sibling
class for `file:line` citations.
