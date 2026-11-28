# Review probes, PR 7825 at c088350de7

Applied on the head: PR 7808's review mutants (`review-minted-door-7c2e5a`), PR 7799's r2
mutants (NOPRES, CONTRAANY, LOOSEINC), and this review's `fate_of` probes, each an env var:

- `PROBE_NOFIX`: the fix reverted (base semantics; the rest of the tree is the head's).
- `PROBE_FOLDLOG`: log each minted face the fix drops, and whether the pieces hold it.
- `PROBE_FOLDPANIC`: panic where a dropped minted face is not among the pieces (the fix changes the fate).
- `PROBE_FOLDANY`: panic on any dropped minted face.

`editor-core` `union_fold_face_face_touch::probe_a_flush_block_off_the_plates_height_still_holds_the_line`
is a flush block of the target row's footprint at four heights. `tall`, `both` and `short` in the
order rod, flush, plate refuse `CarryRefused Found Unread` under `PROBE_NOFIX` and build on the head.

Mutant counts, `--cargo-profile release --profile ci -p topo -p sweep` (7254 tests), head and
`PROBE_NOFIX` identical per test where both ran:
none 0, NOFIX 0, NOSECTV 0, NOSECT 12, IGNREM 34, JOINTBAD 30, ANYREM 3, ANYINC 1, NOPRES 1,
CONTRAANY 1, LOOSEINC 1, GAP face/edge/recut with PANIC 3/10/53, without 2/6/9.

`.config/nextest.toml` gains capped profiles (`cap`, `cap90`, `cap600`) for this container.
