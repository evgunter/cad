---
id: citation-repoint-shifted-a-number-the-lane-knew-was-wrong
kind: issue
title: the citation fix pass shifted a number it had just declared wrong, and undercounts the already-wrong rows by one in three places
status: closed
opened: 2026-09-06
refs: [2083]
closed: 2026-09-06
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

## Closed — all five accepted, and the lane committed the class it caught (2026-09-06, #2083's fix pass)

**Every one of the five is right, and the first is the one that
matters.** This PR opens by naming #2079 for publishing line numbers
its own header rewrites invalidated in the same commit, and then does
the same thing in the same section: `:928` was declared wrong in the
PR's own prose and then written as `:910`, which is `:928 − 18`. The
delta was real; the number it was applied to was not. Nothing was
re-read, whatever the PR claimed — a re-read of `:910` returns
`mesh: part.mesh(),`, which is neither of the two subjects the citation
names.

**1.** `ui-thread-work-after-the-index-seam:31` re-derived by subject
rather than adjusted, and split so each subject carries its own file:
`PickIndex::scene_focused` is `crates/viewer/src/pickindex.rs:894` and
`SceneMesh::build_parts_focused` is `crates/viewer/src/scene.rs:410` —
it was never in `pickindex.rs` at all, which the single parenthetical
had been hiding since before this branch.

**2.** Four, not three, were wrong at the merge base. The uncounted one
is `outstanding-and-progress:49`, which is now `pickcache.rs:376`
(`PickCache::indexing`) — moved again by this fix pass's own header
edit, and re-derived rather than shifted.

**3.** Four, not three, of the out-of-fence rows were already wrong.
The PR body is corrected to agree with the item, which was right.

**4.** The row is renamed:
`renamed-module-leaves-citations-in-two-other-programs`. Every
reference to the old id is re-pointed except the log entry that
introduced it, which is append-only and says so; the new log entry
names both ids.

**5.** `new-document-owes-the-reframe-open-gets:54` now carries the
disclosure IN the item — the file, then *"range unverified"* with what
`pickcache.rs:103-121` actually resolves to and an instruction to
re-find the subject. A sweeper reading the row now learns what a reader
of the PR body would have.

## What the pattern has to be, since the grep is not one

The item is right that no grep finds this. What this pass did instead,
and what a successor should do: enumerate every `file:line` in every
row the branch touches, `sed -n Np` each one, and read whether the
subject is there. That is mechanical, it is cheap — twenty-odd
citations here — and it is the only thing that catches a number that
was shifted by a correct delta from a wrong origin.

Doing it turned up a sixth instance nobody had named: **this fix pass's
own header rewrites moved five of the REVIEW's fresh citations** in
`cursor-projection-landed-in-marks-for-want-of-a-home`,
`highlight-and-edge-overlay-disagree-on-hover-equals-selected`,
`the-point3-to-gpu-corner-cast-is-at-three-sites` and
`a-module-named-for-its-spine-type-is-unfalsifiable`. All were
re-derived by subject and verified line by line, which is the only
reason they are not the next member of this class.
