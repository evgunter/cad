---
id: ledger-notes-have-no-guard-that-their-recovery-command-resolves
kind: issue
title: Fifteen doc-ledger notes recover their file from a commit that does not hold it
status: closed
opened: 2026-09-22
priority: P4
cost: E
closed: 2026-11-23
---

`docs/doc-ledger/auth-1-spec.md` through `auth-15-spec.md` each say
"Recover with `git show 29b8874a1:docs/AUTH-<n>-SPEC.md`". Commit
`29b8874a10` (2026-10-02, "the last commit with work/author/ in the tree")
holds no `docs/AUTH-*-SPEC.md`: `git ls-tree --name-only 29b8874a1 docs/`
lists none, and neither does its parent. The habit is the one the three
TINT notes had in September (the SHA of the commit that deleted the file, or
a later one), so it recurred after PR 3063 repaired those three.

Measured 2026-11-22 over `origin/main`: of the 401 distinct literal
`git show <sha>:<path>` strings in `docs/doc-ledger/*.md`, 240 resolve, 146
are template lines with a `<FILE>`/`<file>`/`<NAME>` placeholder, and these
15 are the only real failures. The audit is one loop of
`git cat-file -e "<sha>:<path>"` (with `git cat-file -e "<sha>^{commit}"`
first, so a commit a shallow clone lacks is reported apart from a path
that is gone).

Owed, and no gate: repoint the fifteen notes at a commit that holds each
spec (on a full clone, `git log --all --diff-filter=D -- docs/AUTH-1-SPEC.md`
gives the deleting commit; its parent holds the file), and add one sentence
to the ledger's note template saying the recovery SHA is the deleting
commit's parent, never the deleting or a later commit. A per-PR check is not
wanted: a broken note costs nothing until someone recovers from it, and the
loop above, run by whoever sweeps the ledger, finds the class in a second.

What would close it: all fifteen `auth-*-spec.md` recovery commands resolve,
and the template carries the parent-of-deletion rule.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): No latency effect, and a per-PR check would add cost for a defect that only bites at recovery time. But the class recurred. Measured today: of the 401 literal `git show <sha>:<path>` strings in `docs/doc-ledger/`, 240 resolve, 146 are placeholders, and 15 are broken. All 15 are `auth-*-spec.md` → `29b8874a1:docs/AUTH-<n>-SPEC.md`; commit 29b8874a10 (2026-10-02) holds no AUTH spec. Fix those notes, plus a template sentence. `reform/ledger-notes-have-no-guard-that-their-recovery-command-resolves.md`.

## Closed

2026-11-23, PR 7419 (CIW drive-by bundle). The 15 `auth-*-spec.md` notes now recover from `84cc49f1f`, the parent of `bb8a91c59a`, the commit that deleted the specs. All 15 commands resolve. No template sentence is owed: `memories/docs-ledger.md` already says to record "a commit hash from before the deletion", and these notes broke that rule.
