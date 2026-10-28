---
id: probe-lane-content-and-naming-keys-split-from-f64-on-every-corpus-document
kind: issue
title: The Probe lane's content and naming keys split from f64's on every corpus document (m4_pr8_k_probe's differential, which aborts scripts/k_probe_sweep.sh); pinned 2026-10-28
status: open
opened: 2026-10-28
---


Filed by GREEN (`green/slow-set-reds-pins`, 2026-10-28), from
`work/green/main-reds-editor-core-slow-set-and-the-k-probe-sweep.md`.
The code is `editor_core::eval::memo` (`ContentBits for geom_core::Probe`),
which is TALLY's and DOCTAIL's ground.

`crates/editor-core/tests/m4_pr8_k_probe.rs`'s
`probe_agrees_with_f64_bit_for_bit_over_the_corpus` is ignored, and is
run by `scripts/k_probe_sweep.sh` at ε 1e-9 and 1e-12. It asserts that a
node's content key and naming key are the same word in the `f64` and
`Probe` lanes, because `ContentBits for Probe` feeds the wrapped f64's
bits.

On main `fa2a942f45`, at both ε rows, the two keys split on 26 corpus
documents, starting at node ordinal 5 to 26 of each. Every node that
splits one key splits the other. Every other comparison the row makes
holds on those nodes: arm, refusal, name tables, verdicts, escalations,
contacts and findings. So the bits a key reads differ between the lanes
while the lanes decide and name alike.

The row is now pinned at main's split set (`PINNED_KEY_SPLITS`: per
document, the first split node and how many nodes split), so the sweep
runs past it. The pin goes red the day the set moves.

Unbisected. The split was already present on main `d39fbfc` (PR 5552's
review, 2026-10-21, per the GREEN row).

Reproduce: `CAD_TOLERANCE_EPS=1e-9 cargo test -p editor-core --features
probe --test all -- --ignored
m4_pr8_k_probe::probe_agrees_with_f64_bit_for_bit_over_the_corpus`, with
`PINNED_KEY_SPLITS` emptied.
