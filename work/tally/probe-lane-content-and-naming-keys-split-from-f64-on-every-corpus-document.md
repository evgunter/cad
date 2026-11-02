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
`probe_agrees_with_f64_bit_for_bit_save_the_pinned_key_splits` is ignored, and is
run by `scripts/k_probe_sweep.sh` at ε 1e-6, 1e-9 and 1e-12. It asserts that a
node's content key and naming key are the same word in the `f64` and
`Probe` lanes, because `ContentBits for Probe` feeds the wrapped f64's
bits.

On main `fa2a942f45`, at 1e-9 and 1e-12 (and at 1e-6, run by PR 5966's
reviewer), the two keys split on 26 corpus
documents, starting at node ordinal 5 to 26 of each. Every node that
splits one key splits the other. Every other comparison the row makes
holds on those nodes: arm, refusal, name tables, verdicts, escalations,
contacts and findings. So the bits a key reads differ between the lanes
while the lanes decide and name alike.

The row is now pinned at main's split set (`PINNED_KEY_SPLITS`: per
document, the mint ordinals of the 333 nodes that split), and renamed
`probe_agrees_with_f64_bit_for_bit_save_the_pinned_key_splits`, so the
sweep runs past it. The pin goes red the day the set moves.

**Un-pinning owes a re-measure.** With the pin, `scripts/k_probe_sweep.sh`
runs past this row, where it stopped before. The sweep's later rows and
the K-report dump now run with nothing having recorded their state, so
whoever closes this row re-measures what ran past the old stop.

Unbisected. The split was already present on main `d39fbfc` (PR 5552's
review, 2026-10-21, per the GREEN row).

Reproduce: `CAD_TOLERANCE_EPS=1e-9 cargo test -p editor-core --features
probe --test all -- --ignored
m4_pr8_k_probe::probe_agrees_with_f64_bit_for_bit_save_the_pinned_key_splits`, with
`PINNED_KEY_SPLITS` emptied.

**2026-10-29, nightly triage** (run 40823227622, `k-lint (dev-probe)`, main `5648e577eb`): `m4_pr8_k_probe::probe_agrees_with_f64_bit_for_bit_save_the_pinned_key_splits` (`crates/editor-core/tests/m4_pr8_k_probe.rs:646`) reds with "the content keys split between the lanes on other nodes than the pinned ones". Five documents' node ordinals moved off `PINNED_KEY_SPLITS`: `heat_sink`, `kitchen_sink`, `heat_sink_fins`, `die_tool` and `part_select`. These are the corpus documents PR 5954 (INTENT G, patterns as index variables) rewrote (`tests/corpus/{heatsink,sink,die_tool,part_select}.rs`). The probe row runs only under `--features probe`, so G's gate did not see it. The split itself is unchanged; the pin needs re-taking at G's ordinals.

**Re-pinned 2026-10-29 (GREEN, after PR 5954).** PR 5954 (INTENT stage 3
G, patterns are index variables) rewrote the `heat_sink`,
`heat_sink_fins`, `kitchen_sink`, `die_tool` and `part_select` corpus
documents. It was the only merge to touch `crates/editor-core/tests/corpus`
after PR 5966 wrote the pin, and it merged without re-pinning. Their
split nodes renumber with the new mint order, and the other 21 documents
read the old pin exactly. `PINNED_KEY_SPLITS` now holds main's set,
identical at 1e-6, 1e-9 and 1e-12 and identical to nightly 76's message.
The split itself is unchanged in kind: 26 documents, and every node
splits both keys.

INTENT stage 6 A then added four corpus documents (`axis_projection`,
`blocks_on_one_floor`, `pin_in_bore`, `revolves_on_one_axis`). They
split the same way and are pinned too: 30 documents, with the other
26 entries unchanged.
