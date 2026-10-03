---
id: embedded-python-readers-are-copied-between-gates-and-unlinted
kind: issue
title: Three gates embed a python reader in a heredoc: copied rather than shared, and outside ruff.toml's population
status: open
opened: 2026-09-15
priority: P3
cost: D
---


Filed by PORT (2026-09-15) from `msrv-floor-is-declared-and-never-compiled`'s
style review. Two findings over one population, filed together because the
population is the same three files and the fix for one is the fix for both.

## The population

Three gates run a python reader from a `<<'PY'` heredoc — the tool for the
job, and the reason is good: `kernel-serde-free.sh`'s header argues at
length that a TOML value has more spellings than a regex has patience, and
a parser reads the document the build reads.

| gate | heredoc |
| --- | --- |
| `scripts/gates/kernel-serde-free.sh` | ~50 lines |
| `scripts/gates/test-features-dev-only.sh` | 119 lines |
| `scripts/gates/msrv-floor-equals-channel.sh` | 128 lines |

## Finding 1 — the plumbing is copied, and `lib.sh` says that is its job

The `tomllib` import guard, the "does not parse" death, and the nested-key
dig are the same code in all three, differing in the gate name inside the
message. PORT's is the SECOND copy of `kernel-serde-free.sh`'s TOML
preamble character-for-character apart from that name, and its header cites
the copy source in prose — which is a self-declared copy, the shape this
repo keeps finding and then sharing.

`lib.sh`'s own header says the shared plumbing — the Rust reader, the
test-only module resolver, the self-test harness — lives there so that no
gate maintains a second copy. A TOML reader with a guarded import, one
death sentence and one refusal sentence is the same kind of thing, and
three instances is past the point where the shared thing gets a home.

**PORT did not lift it**, and the reason is scope rather than disagreement:
moving the preamble into `lib.sh` changes the two existing gates' behaviour
on their reader-death paths, both of which have `--selftest` cases aimed at
the exact message text (`gate_selftest_without_tool python3 "…"`), so the
lift is a three-gate change with three self-tests to re-aim. That is a unit,
not a line in a unit about MSRV.

## Finding 2 — none of these 297 lines is linted, and two of the three fail

`ruff.toml`'s header makes an explicit promise: *"a file added anywhere in
the repo is linted the moment it is committed"*, held by
`scripts/check-python-lint.py` reconciling ruff's visited set against
`git ls-files '*.py' '*.pyi'`. **Python inside a `.sh` heredoc is in
neither set**, so the promise is true of the population it describes and
silent about this one.

Extracted and run under the repo's own `ruff.toml`:

```
ksf.py:1:1:   E401   Multiple imports on one line
ksf.py:40:12: BLE001 Do not catch blind exception: `Exception`
tfd.py:1:1:   E401   Multiple imports on one line
tfd.py:55:28: RUF005 Consider `(*path, key)` instead of concatenation
tfd.py:57:42: RUF005 (same)
tfd.py:59:42: RUF005 (same)
tfd.py:88:12: BLE001 Do not catch blind exception: `Exception`
```

`BLE001` is the one that is not cosmetic here: a blind `except Exception`
around `tomllib.load` swallows an `OSError` — an unreadable or absent file —
into a message that says *"does not parse"*. PORT hit exactly that in its
own gate, where a deleted subject was reported as a parse failure and so
kept a `gate_require_file` guard invisible; catching `OSError` and
`TOMLDecodeError` separately is what made the missing-subject case provable.
The same conflation is live in both files above.

**PORT's own block passes ruff clean** under this config, which is how the
extraction was checked rather than assumed.

## The fix, whichever way GUARD takes it

If the plumbing is lifted into `lib.sh` (finding 1), the linting question
mostly dissolves: shared python belongs in a `scripts/` `.py` file that
`check-python-lint.py` already covers, and the heredocs shrink to a call.
If it is not lifted, the cheap half is still worth having — separate the
imports, catch the two exception types apart, and fix the three `RUF005`.

A third option, and the one that keeps the promise rather than satisfying
it case by case: extend `scripts/check-python-lint.py` to extract `<<'PY'`
heredocs and lint them too. That is the only version that stops the next
gate landing unlinted.

## Scope — what this sweep could not match

`grep -ln "python3 - " scripts/gates/*.sh` and `grep -c "<<'PY'"` over the
same directory, which agree on the three files above. Blind spots:

- **Only `scripts/gates/` was swept.** The style review says six other files
  in the repo embed python the same way; those were not enumerated here and
  are not in the table. A repo-wide sweep is
  `grep -rln "python3 -" --include='*.sh' .`, and it is the right scope for
  the `check-python-lint.py` option above.
- The heredoc marker was assumed to be `PY`. A block opened with any other
  word matches neither pattern.
- Python passed as `python3 -c '…'` rather than as a heredoc matches
  neither.
