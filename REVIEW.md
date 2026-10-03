IN PROGRESS

# Review of #4004 (frozen head a4181ca0)

Probe battery `crates/sweep/tests/fan_end_review_probes.rs` (29 087 lines): main → head moves
exactly 5 628 lines, all on vtxfac's unvoted bare arm. On main all of them refused `JoinDesync`.
On head 2 814 are SOUND, and the rest are typed refusals: the filed two-run case. No BAD line moved.
Mutants and mint_directed comparison still running.
