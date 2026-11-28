# Review probes, PR 7832 at f9a6cc7487

On the head, with PR 7825's review probes ported (`review-editor-fold-red-5f04e9`), each an env var:

- `PROBE_OLDSECT`: the section rule without twins (main's rule). With `PROBE_OLDFILTER`, also main's `fate_of` face filter.
- `PROBE_TWINPANIC`: panic where the twin rule excludes a section cell the old rule kept (TWINX log).
- `PROBE_TWIN1PANIC`: panic where a section cell with no twin is kept interior (TWIN1 log).
- `PROBE_MREADPANIC` / `PROBE_FACEPANIC`: panic where `minted_read` drops a cell in `fate_of` / a face is left among minted.
- `face_pieces` logs `TWINROOT SAME|OTHER` per excluding twin face: whether it descends from the divided face; `PROBE_SAMEPANIC` panics on SAME.
