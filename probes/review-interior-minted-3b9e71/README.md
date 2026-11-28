# Review probes, PR 7832 at f9a6cc7487

On the head, with PR 7825's review probes ported (`review-editor-fold-red-5f04e9`), each an env var:

- `PROBE_OLDSECT`: the section rule without twins (main's rule). With `PROBE_OLDFILTER`, also main's `fate_of` face filter.
- `PROBE_TWINPANIC`: panic where the twin rule excludes a section cell the old rule kept (TWINX log).
- `PROBE_TWIN1PANIC`: panic where a section cell with no twin is kept interior (TWIN1 log).
- `PROBE_MREADPANIC` / `PROBE_FACEPANIC`: panic where `minted_read` drops a cell in `fate_of` / a face is left among minted.
- `face_pieces` logs `TWINROOT SAME|OTHER` per excluding twin face: whether it descends from the divided face; `PROBE_SAMEPANIC` panics on SAME.
- `records.rs`: `PROBE_PATHLOG` logs each record's exit from `landings` (landed, parted, and the whole-removal fall-through, which PR 7825's `probe_consume` did not hook; it now does, as `fallthrough`).

Counts, `--cargo-profile release` with debug assertions, `--profile ci -p topo -p sweep` (7273 tests); "old" = `PROBE_OLDSECT PROBE_OLDFILTER`:
none 0; TWINPANIC 128; TWIN1PANIC 0; MREADPANIC 10; FACEPANIC 0; old 1 (the new pin); OLDSECT alone 4 (flush rows + pin).
Mutants head/old: NOSECT 13/13, NOSECTV 0/1, IGNREM 18/35, JOINTBAD 14/31, ANYREM 3/4, ANYINC, NOPRES, CONTRAANY, LOOSEINC 1/2
(old's +1 is the pin everywhere; IGNREM/JOINTBAD's other 16 are faces whose every fragment is discarded: Divided-with-no-pieces on old, Removed on head).
GAP with PANIC, fall-through hooked: face 3/4, edge 12/13, recut 56/56 (same set); without PANIC recut 16/12.

Slow set: 1039 `sweep::all` tests outside ci's default filter (`reach_cone_root_lane` excluded), profile `cap90`, release with debug assertions,
head with `PROBE_SAMEPANIC PROBE_TWIN1PANIC PROBE_FACEPANIC`: 997 pass, 3 fail (the seam-split census, germ_coplanar_conic, the spline-capped box; the PR's main-and-head three), 39 time out;
no probe or assertion fired; 16520 TWINROOT lines, all OTHER. 138 tests reach a twin exclusion; under "old" those 138 give 134 pass / 1 fail / 3 timeout, the same tests as head.
