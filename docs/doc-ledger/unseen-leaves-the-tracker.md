# UNSEEN leaves the tracker — 2026-10-26

UNSEEN (instruments for hand-written references and surfaces no gate
reads) was cut from META on 2026-10-26, when META carried 38 budget
points against 30. The same day GUARD's split opened UNREAD for the
claims the tree states that no gate reads, and the two are one kind of
work, so UNSEEN folded into UNREAD the same day with nothing dispatched
(Ev, in chat, 2026-10-26). UNREAD's title, charter and `keep_out` now
cover both halves; it keeps both parents' fences (a new gate announces
on GUARD's log, a `scripts/work.py lint` arm on META's). UNSEEN claimed
no paths.

All six of its rows were open and moved to `work/unread/` by `git mv`,
ids and bodies unchanged:

- `doc-citations-no-gate-checks-rot-silently` (P3);
- `no-instrument-guards-a-public-method` (P3);
- `authored-content-folded-into-a-merge-commit` (P3);
- `ledger-notes-have-no-guard-that-their-recovery-command-resolves` (unbanded);
- `tracker-paths-in-crate-doc-comments-rot-on-a-program-re-cut` (P4);
- `perf-plan-citations-name-a-path-that-moved` (P4).

It had no closed rows. Its band 15000–15099 in `docs/MODEL-AB-LOG.md`
closes unspent. Its plan set no `## Exit criteria`, so it closes
without an exit walk.

Recover the program files from the commit that deletes `work/unseen/`
(its parent holds them):

    git log --diff-filter=D --format=%H -1 -- work/unseen/program.md
    git show <that sha>^:work/unseen/<FILE>
