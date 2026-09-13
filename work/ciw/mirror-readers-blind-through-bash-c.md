---
id: mirror-readers-blind-through-bash-c
kind: issue
title: claim 10's flag and env arms both read nothing inside a bash -c command string
status: open
opened: 2026-09-11
---


## What

`bash -c '<string>'` puts a whole command line in ONE `shlex` token. Two of
claim 10's three arms walk tokens and neither hands that token back to a shell,
so both read the invocation inside it as absent. The directory arm does hand it
back (`_chunk_dirs`'s `SHELL_C` branch), because the one live site's whole fact
is the `cd` inside the quotes.

That site is `local-scripts/ci-local.sh:1191`, the local half of
`fmt / rustfmt (benches — its own cargo root)`, written
`run_row "rustfmt (benches)" bash -c 'cd benches && cargo fmt --all --check'`.

**The flag arm.** `cargo_flags` (`scripts/check-ci-mirror-parity.py:1742`)
asks `if "cargo" not in toks` (`:1769`); the token is
`cd benches && cargo fmt …`, so no token IS `cargo` and the invocation is
invisible. Measured: adding `--no-default-features` — an allowlisted
`SEMANTIC_FLAGS` member — to that row's cargo command inside the quotes leaves
`python3 scripts/check-ci-mirror-parity.py` at **rc=0**.

**The env arm, and this half was got wrong when the item was first written.**
`env_prefixes`' trailing scan runs `ASSIGN_RE` over the tokens after the
command word, and `ASSIGN_RE` anchors at `^`. So it Bails only when the
assignment is the FIRST thing inside the string, and returns silently
otherwise. Measured on the live row:

| local half | verdict |
| --- | --- |
| `bash -c 'RUSTFLAGS=-Awarnings cd benches && cargo fmt --all --check'` | rc=1, Bail |
| `bash -c 'cd benches && RUSTFLAGS=-Awarnings cargo fmt --all --check'` | **rc=0, silent** |

The live row begins `cd benches &&`, so the silent case is the one that
applies to it. The first version of this item claimed the env arm "Bails. Loud,
not silent" — that was read off the first row of that table and is false of the
second.

## Why it is not the directory unit's to fix

The fix is in `_simple_commands`/`_command_spans` or in each arm's tokenising
preamble, all of which every claim in the file shares. The directory unit kept
its `bash -c` reading inside `_chunk_dirs` precisely so that no other claim's
answer would move, and measured that none did. Widening the shared tokenizer
changes what every claim here reads and wants its own unit, with its own
differential over the other claims' answers.

It also overlaps `mirror-three-copy-reader-preamble`: the right fix is probably
one preamble that hands a shell string back to the splitter once, for all three
arms.

## Provenance

The flag half: the sweep of CIW's third slate, unit 4
(`mirror-pairs-context-beyond-env` clause 1). The env half: that unit's
correctness review, which re-took the measurement with the assignment in the
second position and found the item's own sentence false.
