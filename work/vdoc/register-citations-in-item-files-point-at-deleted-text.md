---
id: register-citations-in-item-files-point-at-deleted-text
kind: issue
title: A dozen item files cite a rule of the deleted lane register by path, and the path now holds a different file
status: open
opened: 2026-09-21
---


## Finding

`work/view/plan.md`'s rule register was deleted on 2026-09-21 (Ev's
ruling; `work/view/the-lane-register-has-no-home-after-views-directory-goes`).
The file still exists and still holds VIEW's plan, so a citation to it
does not 404 — it reads as valid and lands on text that no longer says
what the citing row claims.

The structural references were re-pointed in the deleting PR: four
`plan.md` §The register sections and four `program.md` pointers. What
was not is every item file that cites the register for a **rule**, by
path:

    work/edit/authored-step-to-canonical-segment-map-has-no-home.md
    work/edit/focus-marking-is-per-node-not-per-segment.md
    work/vdoc/stale-file-citations-after-the-split.md
    work/vgeom/program-md-cites-a-plan-section-the-re-scope-deleted.md
    work/vgeom/vgeom-plan-has-no-register-section-and-no-charter.md
    work/view/render-mm-overflows-to-inf-for-a-delta-the-door-accepts.md
    work/view/tier-rule-says-twenty-one-jobs-and-a-docs-run-shows-twenty-two.md
    work/vnews/hand-maintained-counts-in-frame-rs-prose-have-no-guard.md
    work/vnews/rank-one-discards-the-frames-other-news.md
    work/vnews/tone-doc-argues-from-a-site-that-now-reads-the-value.md

Re-derive that list rather than trusting it —
`grep -rln 'work/view/plan\.md' work/` minus the four `plan.md` files
(whose mention is the deletion note itself), the two `log.md` files
(append-only history, correct as written) and `work/STATUS.md`
(generated).

## What a taker owes, and what is NOT owed

**Not a rewrite of ten rows.** Several are closed, and a closed row is
a record of what was decided on; editing its citation edits the record.
The deletion note in `work/view/plan.md` names the recovery SHA
`66d7357417`, which is the `docs/DOC-LEDGER.md` convention for deleted
text, so every citation is resolvable at a tree.

What is owed is a decision, stated once and applied: **does a citation
to deleted text get the SHA appended, or does it get left as history?**
The answer probably differs for open rows (whose argument a reader is
expected to follow) and closed ones (whose citation is a record). Say
which, apply it to the open ones only, and say so in the PR.

`work/edit/`'s two rows are EDIT's ground and get a report, not an
edit — `work/vdoc/program.md`'s `keep_out` is explicit that other
programs' item files are never edited from here.

## Fence

`work/` tracker prose only. No `crates/` file, no behaviour.
