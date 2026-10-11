---
id: the-full-symbolic-pass-runs-off-the-hot-path
kind: issue
title: The coincidence door's full symbolic pass runs asynchronously after an evaluation and updates the same report in place, so the resident pass's words are transient
status: open
opened: 2026-10-11
priority: P2
cost: M
---

INTENT stage 4 C+D (`carriers-compare-in-canonical-form`) gives the
coincidence door one budget parameter, `coincide::SymbolicPass`. The
`unproven-coincidence` resident runs after every evaluation at
`Resident`: 64 terms, no retry ladder. It leaves every row it does not
discharge saying "unproven at the resident budget; the full symbolic
pass decides". The full pass (256 terms and the `kept_atom` ladder) is
`ChecksConfig::symbolic_pass = Full`.

Measured: the full pass costs 26 s per evaluation on
`die_composed_tour` in release, and +45 % on the tour gallery's debug
test. That is too much for the hot path, so a viewer user sees the
resident's words where the full pass would prove the row (the disc's
supports, 21 of the tour's 42 joints).

Owed: run the full pass off the hot path, after the evaluation and
cancelable by the next one, and replace the resident's rows in the
same report when it lands. Then the resident wording is transient.
Done when the viewer's report reaches the full pass's verdicts without
an evaluation waiting on them.
