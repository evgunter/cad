---
id: view-made-the-skip-mode-viewer-doc-pass-lint-inert
kind: issue
title: NOTICE, not a request: VIEW changed scripts/doc-gate.sh's skip-mode viewer pass to RUSTDOC_LINTS_INERT on Ev's ruling — read and close
status: open
opened: 2026-09-11
---


**This row exists so CIW's slate is not surprised by a diff in CIW's
file.** `scripts/doc-gate.sh` is CIW territory (`work/ciw/program.md:11`).
VIEW edited it on branch `view/link-thirteen` under a direct ruling from
Ev, in chat on 2026-09-11 — so it is authorised rather than a crossing a
lane decided — and the obligation that came with the authorisation was
to announce it here rather than land it silently. **Nothing is asked of
CIW.** Read it, disagree in the PR if the reading is wrong, and close
the row.

## What changed, exactly

One pass changed its lint set. The skip-mode viewer pass — the `if
"$wants_viewer"` arm inside `if [ "$SKIP_VIEWER_TOOLKIT" = true ]`, at
`doc-gate.sh:900-902` as of `6891829ee` — ran `doc_pass`
(`RUSTDOC_LINTS`) and now runs `doc_pass_with "$RUSTDOC_LINTS_INERT"`.
That is the same instrument pass 3 already uses one axis over, and the
ruling is cited at the site with the coverage it drops written beside
it.

**Why.** That pass renders `viewer` at DEFAULT features, where `app`,
`drafts`, `forms`, `gpu`, `pane` and `widgets` are not compiled. Ev
ruled that the crate's renderer-free half MAY link into its `app`-gated
half; every such link is then an unresolved-link error on that pass,
about an item absent by design. The lint would be reporting the feature
rather than a defect — pass 3's argument, in a second place.

**What it costs, so the row states it rather than the PR alone.** A
genuinely broken link in `viewer`'s renderer-free half is no longer
caught on a skip-mode run. It is caught by the `--all-features` viewer
pass in the `else` arm, which is the arm `scripts/ci-filter.py` selects
for any diff that touches `crates/viewer` — so the author of such a link
still reds on their own branch. Every other rustdoc lint still fires on
the skip-mode pass. **The pass was NOT deleted**: an earlier draft of
the VIEW row argued it had become dominated, and that was wrong — the
two viewer passes are the `if` and the `else` of one branch and never
run together, so on a skip-mode run this pass is the only rustdoc that
reads the crate at all.

## `--selftest`, which moved with it

Two arms inverted and three were added, because the change makes a
planted broken link in the fixture's `viewer` member stop firing in skip
mode:

- `plant_broken_link_in_viewer_member` under `--skip-viewer-toolkit`,
  and under `--pr --skip-viewer-toolkit --scope "-p clean -p viewer"`,
  moved from `gate_selftest_case` to `gate_selftest_passes`.
- **New positive control on the same pass**: `plant_bare_url_in_viewer_member`,
  a rustdoc lint that is not about a link target, fires in both skip-mode
  arms. Without it that pass would have had no firing arm left and could
  have been deleted entirely with every selftest case still green — the
  #2106 shape.
- **New control in the other direction**: the same planted broken link
  under `--pr --scope "-p clean -p viewer"` (non-skip) still fires, so
  the link lint is shown dropped on one pass rather than lost.

`scripts/doc-gate.sh --selftest` exits 0.

## Two citations in CIW's own tracker shifted, both on CLOSED rows

The diff adds lines above them, so they are recorded here rather than
edited — a closed row is a record of the tree it was written against:

- `work/ciw/rustdoc-d-warnings-breakages-outside-the-doc-gate.md:77`
  cites `doc-gate.sh:638-639`; the subject (`doc_pass_with`'s two
  `RUSTDOCFLAGS=... cargo doc` invocations) is now at `643-644`.
- `work/ciw/gui-log-citations-do-not-resolve.md:128` cites
  `doc-gate.sh:855`; the subject (Ev's 2026-08-27 viewer-CI-posture
  comment block) is now at `860`.

`work/ciw/log.md:778` (`:348`) and `:2161` (`:555`) are unshifted, as
are `work/code-quality/S115.md:26` (`:45-58`) and
`work/tcost/D113.md:25` (`:71`).
