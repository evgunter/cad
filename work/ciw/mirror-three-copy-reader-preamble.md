---
id: mirror-three-copy-reader-preamble
kind: issue
title: claim 10's three readers open with the same five-step preamble, written three times
status: open
opened: 2026-09-11
---


## What

`cargo_flags`, `env_prefixes` and `work_dirs` in
`scripts/check-ci-mirror-parity.py` each begin with the same five steps:

1. `_join_continuations(lines)`
2. `GH_EXPR_RE.sub("$GHEXPR", line)`
3. a cheap substring or regex guard that skips lines the arm cannot care about
4. `_command_spans` / `_simple_commands`
5. `shlex.split(chunk)` inside a `try`, whose `Bail` differs from the other
   two only in its last sentence

Three copies of one walk. The file's own rule — stated at `_command_word`,
which this unit extracted for exactly this reason — is that two copies of a
walk are two answers, and the anchor bug `_command_word` exists to fix was one
copy drifting from what the other did.

## Why it is filed and not fixed here

The directory unit extracted the INNER walk (`_command_word`) and deliberately
left the outer preamble tripled, because all three arms consume it and a change
to it moves every claim's answer at once. That is a differential someone has to
take over the whole checker, not a tidy-up to ride along on a
working-directory item.

It is also where `mirror-readers-blind-through-bash-c` belongs: the right shape
is one preamble that hands a `bash -c` string back to the splitter once, for
all three arms, rather than the directory arm's private `SHELL_C` branch plus
two arms that read nothing.

## Provenance

The style review of CIW's third slate, unit 4
(`mirror-pairs-context-beyond-env` clause 1).
