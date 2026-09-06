---
id: citation-repoint-shifted-a-number-the-lane-knew-was-wrong
kind: issue
title: the citation fix pass shifted a number it had just declared wrong, and undercounts the already-wrong rows by one in three places
status: open
opened: 2026-09-06
refs: [2083]
---


Found by the style review of #2083. The PR's sweep section is the part
of it that reports on itself, and four separate numbers in it are
wrong. Every one is checkable against the lane's own merge base
`499a17b`.

## 1. A number declared wrong, then shifted rather than corrected

`work/view/ui-thread-work-after-the-index-seam.md:31` cites
`PickIndex::scene_focused → SceneMesh::build_parts_focused` at
`crates/viewer/src/pickindex.rs:910`.

The PR body names this row as one whose number *"was already wrong at
this lane's merge base — `:928` against a real value of `:912`"*. At
the merge base, `pickindex.rs:912` is `pub fn scene_focused(` and
`:928` is `mesh: part.mesh(),`. The move shifted the header by −18, so
the correct number at head is `:894` — which is where
`pub fn scene_focused(` is. The row was written `:910`, which is
`:928 − 18`: the WRONG number, shifted. Neither of the two subjects the
citation names is at `:910`; it is still `mesh: part.mesh(),`.

The PR says *"the numbers here were all re-read after the last edit of
this branch"*. A re-read of `:910` would have shown this.

## 2. "Three already wrong" is four

The PR reports three merge-base-wrong tracker citations
(`:405`/`:442`, `:756`, `:928`). All three check out. A fourth is not
counted: `outstanding-and-progress-are-two-three-state-enums-one-hop-apart:49`
cited `pick.rs:359` for `PickCache::indexing()`, and at the merge base
`pick.rs:359` is `}` while `PickCache::indexing` is at `pick.rs:375`.
The rename shifted `pickcache.rs` by −2, and the row was correctly
rewritten to `:373` — so this one WAS fixed, it just was not counted.
Four rows were wrong at the merge base, not three.

## 3. "Three of the four" out-of-fence rows is four

`work/view/renamed-module-leaves-citations-in-three-other-programs.md`
enumerates four open out-of-fence rows and asserts, bullet by bullet,
that all four were already citing the wrong file before this rename
(*"and have been since #2079"* three times, plus the `edge_segments`
one). The PR body says *"three of the four were already citing the
wrong file before this rename"*. The item and the PR disagree, and the
item is right.

## 4. The row's own id names three programs and lists two

`renamed-module-leaves-citations-in-three-other-programs` — the four
rows are on `work/chrome/` (three) and `work/code-quality/` (one). Two
programs. `work/README.md` makes ids stable for life, so the count is
baked into the filename and every future citation of the row.

## 5. A knowingly-wrong range left in a live row with no note in it

`work/view/new-document-owes-the-reframe-open-gets.md:54` now reads
`crates/viewer/src/pickcache.rs:103-121`. The PR discloses that the
range was off-target at the merge base and that it was corrected on
path only. Nothing in the item says so: `pickcache.rs:103-121` is the
tail of `IndexInputs::of` and the head of `PickCache`'s doc comment,
and a sweeper following the row will read it as a citation. The
disclosure lives in a PR body, and `work/README.md` is explicit that a
PR body is not a slate — *"Disclosing a residue is therefore not
scheduling it"*. A half-clause in the item (*"range unverified"*) would
have cost nothing.

## The class

All five are the same defect the PR opens by naming in #2079: a
citation pass that trusts a number's OFFSET rather than re-finding its
SUBJECT. The pattern to sweep with is not a grep — it is re-resolving
each `file:line` in every row this branch touched and reading what is
actually there. Where else to look: every row in `work/view/` that
cites `pickindex.rs:NNNN`, and `stale-file-citations-after-the-split`,
which is the open class row for exactly this.

## Confidence

`sure` on 1, 2, 3 and 4 (all mechanical against `499a17b`). `sure` on
the fact in 5; `likely` on the judgement that the disclosure is
insufficient.
