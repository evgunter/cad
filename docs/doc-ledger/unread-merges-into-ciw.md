# UNREAD merges into CIW — 2026-11-02

UNREAD (claims, references and surfaces the tree states that no gate
reads) was cut out of GUARD on 2026-10-26, and UNSEEN folded into it the
same day (`unseen-leaves-the-tracker.md`). On 2026-11-02 the track
budget came to count only the dispatchable P0, P1 and unbanded rows
(`work/README.md`, Track size). UNREAD then measured 6 points and CIW
(hosted CI, workflows and scripts) 9, both against 30. Ev asked for
under-budget tracks to be merged where they form one track (in chat,
2026-11-02). UNREAD's question, "what goes red if this stops being
true?", is the one CIW already asks of the hosted gate, and two pairs of
rows are one finding filed from each side. So UNREAD folded into CIW
with nothing dispatched.

- CIW's title, charter, `paths` and `keep_out` now cover both halves.
- CIW keeps UNREAD's fences: a new gate announces on GUARD's log and is
  built on its harness, and a `scripts/work.py lint` arm announces on
  META's.
- `work/ciw/plan.md` carries UNREAD's slate, order and review posture.

All fifteen of its rows were open and moved to `work/ciw/` by `git mv`,
ids and bodies unchanged:

- `feature-gated-lib-unit-tests-are-compiled-and-never-run` (unbanded);
- `ledger-notes-have-no-guard-that-their-recovery-command-resolves` (unbanded);
- `measurements-have-no-mechanical-guard` (P3);
- `viewer-disabled-control-population-has-no-gate` (P3);
- `raw-door-compile-proof-needs-a-gate` (P3);
- `comment-symbol-names-outside-rustdocs-reach-have-no-gate` (P3);
- `door-table-reason-strings-are-unguarded-prose` (P3);
- `vocabulary-macro-bodies-are-outside-rustfmt` (P3);
- `doc-citations-no-gate-checks-rot-silently` (P3);
- `no-instrument-guards-a-public-method` (P3);
- `authored-content-folded-into-a-merge-commit` (P3);
- `inserting-an-item-above-another-steals-its-docs` (P4);
- `S115` (P4);
- `tracker-paths-in-crate-doc-comments-rot-on-a-program-re-cut` (P4);
- `perf-plan-citations-name-a-path-that-moved` (P4).

It had no closed rows. Its band 15700–15799 in `docs/MODEL-AB-LOG.md`
closes unspent. Its plan set no `## Exit criteria`, so it closes
without an exit walk.

The program files (`program.md`, `plan.md`, `log.md`) are recoverable
at 82080f7c76a1e08644f69622cb64aae2a2a9cea6, the commit before the
merge:

    git show 82080f7c76a1e08644f69622cb64aae2a2a9cea6:work/unread/<FILE>
