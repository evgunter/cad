# UNSEEN — the plan

Instruments for hand-written references and surfaces no gate reads.

Opened 2026-10-26 by META's cut (`work/README.md`, Track size). Nothing
dispatched; branch prefix `unseen/`, away-channel tag
`(UNSEEN orchestrator)`, A/B band 15000–15099 claimed for bookkeeping.

## The slate

**12 budget points** of dispatchable work against a ceiling of 30 (one
row unpriced, charged 2.5).

| pri | item | cost | title |
|---|---|---|---|
| P3 | `doc-citations-no-gate-checks-rot-silently` | D | `file.rs:NNN` citations across `work/` and test names in doc comments, neither checked |
| P3 | `no-instrument-guards-a-public-method` | D | the façade censuses guard exported names and members, never a public method |
| P3 | `authored-content-folded-into-a-merge-commit` | D | authored lines inside merge commits, invisible to `git log -p` |
| — | `ledger-notes-have-no-guard-that-their-recovery-command-resolves` | — | nothing runs a doc-ledger note's `git show <sha>:<path>` |
| P4 | `tracker-paths-in-crate-doc-comments-rot-on-a-program-re-cut` | E | a `work/<program>/<id>` path in a crate comment rots when the row moves |
| P4 | `perf-plan-citations-name-a-path-that-moved` | E | five citations still name `docs/PERF-PLAN.md` |

## Order

The citation family first, because its three rows are one instrument
seen from three sites: `doc-citations-no-gate-checks-rot-silently` (two
arms) and `tracker-paths-in-crate-doc-comments-rot-on-a-program-re-cut`
(its third arm, and the one every program re-cut feeds), with
`perf-plan-citations-name-a-path-that-moved` as the instance a working
checker should find and the fix that lands with it. Then the ledger
guard, which needs a band and a cost before it is dispatched and whose
body already names the shallow-clone case it must not red on. Then the
two surface guards, each of which opens on a decision its body states:
how large an alphabet the public-method census pays for, and how strict
"resolution content" is for a merge commit.

## Review posture

META's, which it inherits: infra and prose units, no A/B row. One style
review per unit, plus a correctness reviewer where a unit adds a check
that can red CI, since a false positive there blocks every program.
