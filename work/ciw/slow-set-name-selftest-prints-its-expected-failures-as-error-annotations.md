---
id: slow-set-name-selftest-prints-its-expected-failures-as-error-annotations
kind: issue
title: check-slow-set-names.py --selftest prints its expected failures as ::error annotations on a green lint job
status: open
opened: 2026-11-20
priority: P3
cost: E
---


Found reviewing EXCH's PR 7166: its green `lint (fmt, clippy, gates)` job
carries an `::error` annotation, `slow-set entry matches no test: b: n::z`.

`scripts/check-slow-set-names.py`, `selftest`: the negative cases call
`check(...)` on fixtures that are meant to fail (a renamed test, a gone
binary, a duplicate), and `check` prints each failure as a GitHub
`::error file=.config/nextest.toml::` workflow command. So
`--selftest` emits `a::all: m::y` and `b: n::z` as error annotations on
`.config/nextest.toml` in every run that self-tests, while the job
passes — a reviewer reading the checks page sees errors that are not
there, and a real dead entry reads like one more of them.

The repair is for `check` to return (or take a sink for) its findings,
with the caller printing the workflow commands, so the self-test asserts
on the findings silently.
