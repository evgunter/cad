---
id: mirror-flag-arm-blind-through-bash-c
kind: issue
title: claim 10's flag arm reads nothing inside a bash -c command string
status: open
opened: 2026-09-11
---


## What

`cargo_flags` (`scripts/check-ci-mirror-parity.py:1651`) tokenises a chunk with
`shlex.split` and then asks `if "cargo" not in toks` (`:1678`). A `bash -c '…'` puts the
whole command string in ONE token, so `bash -c 'cd benches && cargo fmt --all
--check'` tokenises to three tokens and none of them is `cargo`: the invocation
is invisible to the flag arm entirely, not read with some flags missed.

That spelling is live on exactly one row, `local-scripts/ci-local.sh:1185`, the
local half of `fmt / rustfmt (benches — its own cargo root)`.

**Measured, 2026-09-11.** Adding `--no-default-features` — an allowlisted
`SEMANTIC_FLAGS` member — to that row's cargo command inside the quotes leaves
`python3 scripts/check-ci-mirror-parity.py` at rc=0. The flag is on one half of
a mirrored pair and the gate says nothing, which is claim 10's headline defect
on the one row of the repo that spells its command this way.

`env_prefixes` is NOT affected in the same direction, and the difference is
worth writing down: its walk breaks at the command word `bash` and its
trailing scan then sees `CAD_X=1 cargo test` as a token `ASSIGN_RE` matches, so
an allowlisted variable inside a `bash -c` string **Bails**. Loud, not silent.
The directory arm reads through the string deliberately (`_chunk_dirs`'s
`SHELL_C` branch), because that row's `cd` is the fact the pair is compared on.

## Why it is not the directory unit's to fix

The fix is in `_simple_commands` or in `cargo_flags`, which every claim in the
file shares. The directory unit (PR for `mirror-pairs-context-beyond-env`
clause 1) kept its `bash -c` reading inside `_chunk_dirs` precisely so that no
other claim's answer would move, and said so in its PR body. Widening the
shared tokenizer is a change whose blast radius is every claim here and wants
its own unit, with its own measurement of what the other claims then say.

## Provenance

Found and measured by the sweep of CIW's third slate, unit 4
(`mirror-pairs-context-beyond-env` clause 1).
