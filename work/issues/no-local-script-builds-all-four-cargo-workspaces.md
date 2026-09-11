---
id: no-local-script-builds-all-four-cargo-workspaces
kind: issue
title: The repo has four Cargo workspaces plus tools/tess-meter and no local script builds them all, so a signature change sweeps crates/ and reaches hosted CI red from demos/
status: open
opened: 2026-09-06
---


Reported by MSOLVE-3's implementer lane (PR 2081), outside its fence;
filed by the MSOLVE orchestrator. CIW's by shape.

The root workspace, `benches/`, `demos/tour/` and `demos/wild/` are
four Cargo workspaces, and `tools/tess-meter` a fifth root; hosted CI
builds them all, and no local script does. A lane changing a public
signature (MSOLVE-3's `Frame::rotate_then_translate` gaining a band
and a `Result`) swept `crates/`, went green locally, and reached hosted
CI red on `demos/tour/src/assembly.rs`. `local-scripts/ci-local.sh`
is the natural home for a "every root" build, or a `--all-roots`
mode on `test-fast.sh`; until then every signature sweep is a grep
over `crates/ benches/ demos/ tools/`, and the discipline doc could
say so.
