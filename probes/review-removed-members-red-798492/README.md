# Review probes, PR 7799 at 17237bee03

The diff to the head is in `probes.patch`, and is also applied on this branch. Every
mutant is an env var, so one build serves them all. Each row ran as
`cargo nextest run --cargo-profile release --profile ci -p <crate>`, with
`CARGO_TARGET_DIR` per crate. Counts are topo + sweep, net of the two `probe_a_*`
rows, which are red by design.

| env | mutant | red |
|---|---|---|
| (none) | | 0 |
| PROBE_ANYINC | any pair error is structure (incidence included) | 1 |
| PROBE_LOOSEINC | incidence counts where neither image is minted (new) | **0** |
| PROBE_NOSECT | section not named | 14 |
| PROBE_ANYREM | consume on any removal | 2 |
| PROBE_IGNREM | ignore removed members | 34 |
| PROBE_JOINTBAD | B's clone cells read unmapped in the joint body | 31 |
| PROBE_SETASIDEALONE | set aside even as the only landing | 1 |
| PROBE_SETASIDEVV | set aside two-vertex minted landings too (new) | 3 |
| PROBE_CONTRALEAK | contradicted is touched on kept pairs too (= (2) outside removed reads) | 1 |
| PROBE_CONTRAANY | any removed-read error is touched (new) | **0** |
| PROBE_NOCONTRA | (2) reverted: contradicted is Unread (new) | **0** in `ci` |
| PROBE_GAP=face/edge/recut + PROBE_PANIC | red / silent consume | 3/3, 12/6, 56/54 |

GAP at PR 7745's head `a27ce80982`, with R1's patch: silent consumes 3 / 6 / 54, the same
test sets. Recut moves two topo rows from the `removed` path to `structure`.

Probe rows (all on this branch):

- `vertex_on_edge_records::probe_a_removed_member_touching_nothing_consumes_by_sense_alone`:
  a removed member that touches nothing, same sense as the other face → `Ok(0)`
  (consumed). Opposed → `Stale`. The census's Door 1 (sense) contradicts before
  Door 2 (overlap) is read.
- `boolean::records::tests::probe_a_shared_glued_piece_consumes_a_record_touching_nothing`:
  glued divisions that share a piece consume a record touching nothing → `Ok(0)`.
  Before PR 7799 only a shared minted cell did this.
- `PROBE_LOGSET`, `PROBE_LOGSTRUCT` and `PROBE_FINDINGS` trace the pierce row.
  It has 12 set-asides, all `beside=true`. Its result records hold that vertex–edge
  pair through another route. Its tier 3′ findings are CensusUndecidable plus the
  pinned EdgeFaceOverlap only.
