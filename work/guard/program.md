---
id: guard
kind: program
title: GUARD — the gate harness: the shared readers and the selftests
status: ready
opened: 2026-09-11
group: infra/ci
prefix: guard/
tag: (GUARD orchestrator)
ab_band: 3600-3699
paths: [scripts/gates/*]
keep_out: [crates/geom-core/tests/bounds_census.rs and crates/test-utils/src/source.rs are S-TINT's and S-TCOST's ground - the bound-list row lands its shell side here and announces the Rust side on the seam, tools/* is INSTR's, .github/workflows/* and local-scripts/* and scripts/check-*.py and ruff.toml's checker are CIW's - wiring the gates' selftests into a job lands by announced seam, BLIND (work/blind/) shares six gate scripts here and owns what their matchers see while this program owns the reader and harness they run on, UNREAD (work/unread/) adds new gates under this directory on this program's harness and announces each one here]
---

`scripts/gates/lib.sh` and the plumbing every gate shares: the Rust
reader (`gate_rust_code`), the test-only skip, the selftest harness, the
guards on a reader's pipeline stages, and the readers gates copy instead
of share. Every row is a way the harness lets a gate read nothing, read
wrongly, or prove itself on the wrong evidence, while the gate prints
OK. Since the 2026-10-26 split the matcher rows are BLIND's
(`work/blind/`) and the ungated-population rows UNREAD's
(`work/unread/`). Charter and unit order: `work/guard/plan.md`;
narrative in `work/guard/log.md`.
