---
id: python-coincidences-replays-the-document-per-node
kind: issue
title: Python's Evaluation.coincidences(node) replays the whole document at Sym on every call
status: open
opened: 2026-10-11
priority: P3
cost: E
---

Found in review of PR 4670 (`the-symbolic-door-proves-at-one-strength-at-rest`). `crates/pncad-py/src/py/value.rs` `Evaluation.coincidences(node)` calls `coincide::prove` on that node's rows alone. Each call replays the whole document at `Sym` and takes no cancel token, so a Python loop over N nodes pays N replays (die_composed_tour's replay is 9.7 s in release). `run_checks` replays once for every node.

Owed: answer every node from one replay (memoize the proofs on the evaluation, as the product is memoized in `product_memo.rs`), or take a token.
