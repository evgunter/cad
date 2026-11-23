INTERIM — reviewer lane, PR 7517 @ a87d047b5a (full report follows).

Verdict so far: NOT-MERGEABLE-AS-IS, on one MAJOR (DEMONSTRATED):
- `scripts/gates/bit-identity-consumer.sh` goes RED on this head: `crates/mesh/src/trimmed.rs:3236` (`CollapsedSides::of_point`) calls `geom_core::bit_identity::one_stored_point`, and mesh/trimmed.rs is not one of the gate's NON_CONSUMER_HOMES. ci.yml's `lint` job runs every gate in that directory, so lint will fail once the queue reaches it (no job past "change filter" has run yet). This is more than a missing list line. The gate's header and `geom-core/src/bit_identity.rs`' module docs say the channel is retired from production coincidence (N6/D10), that the listed collapsed-side readers "decide no coincidence between two cells", and "add nothing without a DESIGN.md revision". The new reader decides exactly such a coincidence: a body VERTEX against a face net's side point. The orchestrator's exact-identity ruling therefore needs a DESIGN `[extensibility]` revision (Ev), or the mesh has to take the decision from somewhere else (for example the lift's own Sides decision), which also removes the claim-5 disagreement.

Everything else so far holds:
- Claim 1 is reproduced on base: polygon diagonal (0.0333,1)→(0,0), bound 0.016494 at 1e-2.
- Bytes are identical across ε 1e-9/1e-6/1e-12 on 14 bodies × 4 δ.
- Budget deviation over 14 bodies × 4 δ: max worst_ratio 0.50, no certificate over δ.
- Mesh-suite and sweep-meshing digests: no face moved outside the new rows.
- All 9 mutants are caught.
- The ulps-off vertex gives a typed CertificateExceeded and no panic.
