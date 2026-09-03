---
id: rustdoc-gate-private-intra-doc-links
kind: issue
title: Revisit — should the rustdoc gate reinstate private_intra_doc_links?
status: open
opened: 2026-08-15
github: 519
refs: [465]
---

## From GitHub issue 519

Opened 2026-08-15; 0 comments.

Banked from #465's chunk 0. The rustdoc gate (`scripts/doc-gate.sh`) runs
with:

```
RUSTDOCFLAGS="-D warnings -A rustdoc::private_intra_doc_links"
```

This issue is only about that `-A`. The gate itself, and the per-crate
cleanup ratchet, live in #465.

## What the lint says, and why it fires here

`private_intra_doc_links` fires when a **public** doc comment links to a
**private** item:

```
warning: public documentation for `write_ascii` links to private item `NAME`
  = note: this link resolves only because you passed `--document-private-items`,
          but will break without
```

The gate does always pass `--document-private-items`, deliberately —
much of the load-bearing prose in this repo sits on private functions
(`span_offset`, `span_indices`, `frame_from_unit_aim`), and without the
flag those are neither rendered nor checked, which was half of #465's
argument. So in the docs this repo actually builds, every one of these
links resolves. The lint is warning about a doc set we do not produce.

## The measurement

Workspace-wide, `cargo doc --no-deps --document-private-items`:

| class | count |
|---|---|
| `private_intra_doc_links` | 82 |
| `broken_intra_doc_links` (unresolved) | 58 |
| `X is both a function and a module` (ambiguous) | 11 |
| redundant explicit link target | 6 |
| **total** | **157** |

Allowing the one lint takes the real backlog from 157 to 75 — and the 75
are all genuine breakage, which is what makes the per-crate ratchet in
#465 a tractable size.

## The question to settle later

Three options, roughly:

1. **Keep it allowed** (status quo). One doc set, private items
   rendered, links resolve. Cost: a consumer reading docs.rs-style
   public-only output would see 82 dead links — but nothing publishes
   such a set today, and the crate is private.
2. **Reinstate it and fix the 82.** Means rewriting public prose so it
   stops linking to the private helpers it is explaining, or promoting
   those helpers. Both look like losses for prose whose whole value is
   the invariant argument.
3. **Reinstate it and render two doc sets** — public-only (lint on) and
   private-inclusive (lint off). Honest, and the only option that gets
   both properties, at the cost of a second rustdoc invocation in CI and
   a decision about which set is canonical.

Trigger to revisit: the day anything publishes a public-only doc set, or
Q9 lands and the crate becomes public-facing — whichever comes first.
Until then option 1 is the measured choice, recorded here rather than
left implicit in a flag.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

## Home

`work/issues/`: `scripts/doc-gate.sh` is S-QA's territory and S-QA is closed; its exit walk names the doc-gate's remaining axes as standing residue rather than another program's ground.
