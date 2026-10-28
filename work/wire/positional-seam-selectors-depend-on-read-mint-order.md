---
id: positional-seam-selectors-depend-on-read-mint-order
kind: issue
title: A positional SegPat over a seam's sides now depends on which read was minted first, though a seam's sides are a set
status: closed
opened: 2026-10-20
priority: P2
cost: E
closed: 2026-10-27
---


## Finding

Since `wire/pair-seam-name-order` (PR 5469) a seam's sides are in name
order, each `From { read, of }` (or a `Merged` set), so which side is
`a` follows which read sorts first: the read's mint ordinal. A
selector written `SegPat::tag(SegTag::Seam).of([p, q])` matches its
arguments positionally (prefix argument matching, `names/select.rs`),
so it holds only while the reads keep that order:

- `crates/editor-core/tests/corpus/die_composed.rs`, `selector()`'s
  `pip_rim`: `of([from(cap_top), from(face(band))])`, true because the
  cube's read is minted before the ball's;
- `crates/editor-core/tests/lib_u7_select.rs`,
  `a_seam_pattern_may_constrain_only_its_first_side`: constrains `a`
  alone, the same dependence;
- Python `SegPat.tag(SegTag.Seam).of([...])`
  (`crates/pncad-py/src/py/select.rs`), the same matcher;
- `crates/pncad-py/tests/ty_fixtures/legal.py:445`, a type-check
  fixture spelling one.

Re-authoring the document so the tool's read mints first would make
these patterns match nothing, with no name having moved in meaning.
A seam's sides are a set (N3), so a pattern over them reads as one
whether it is a set pattern (each argument matching some side) or a
documented positional one over the name order. Which, and whether
`SegPat` grows a set form, is the question.

Signed (WIRE lane `wire-seamorder`).

Closed by PR 5854 (2026-10-27). These positions now match pattern arguments as sets, by distinct-member bipartite matching:
- a seam's sides;
- `Merged` constituents;
- `EdgeCrossing` lines;
- `BandFace`, and `BandCross`/`BandSlit`'s band;
- a junction's run of seam lines.

Positional roles are unchanged. The four witnesses are red on base, and the re-authoring test asserts that the side order flips. The old positional wording was unsigned lane text.
