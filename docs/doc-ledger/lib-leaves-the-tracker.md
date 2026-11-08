# LIB leaves the tracker — 2026-11-03

LIB ("usable as a library") opened 2026-08-06 and made the kernel usable as a library under `docs/LIBRARY-DESIGN.md`. It was cut on its priority seam on 2026-09-20, which opened BIND. It closed on 2026-11-03 (Ev, in chat: close the old parents whose P1+ work had run thin, and triage LIB). At that point it held 57 live rows behind 124 closed ones. Three of the live rows were P1, fourteen were unbanded and many were priced legacy `D`. Its plan set no `## Exit criteria`, so it closes without an exit walk.

**Where its live rows went.** Each moved by `git mv`, id and body unchanged, with a dated note. The unbanded rows were banded and the `D` rows re-priced in the move, and each row records what it was given:
- **PYDOOR** (`work/pydoor/`, opened for them; it inherits LIB's `paths` and its A/B band 300-399): 33 rows, the library doors and the façade's usability. These are the three P1s, three P2 door defects and the P3/P4 doors the north-star audit waits on.
- **BIND**: 24 rows, the bindings' housekeeping: tag words, payload rungs and projections, the prose census, stubs and guides, and Python test rows.

**LIB's residual register.** The register (2026-08-10, categories A to E, with F from the 2026-08-28 re-survey) and the ASM cross-program deposit of 2026-08-23 lived only in `work/lib/log.md`, and permanent docs cited them as live. Those citations in `docs/LIBRARY-DESIGN.md`, `docs/PATHS-DESIGN.md`, `docs/guide/north-star-audit.md`, `crates/pncad-py/tests/test_binding_census.py` and the two die-tool test headers now name this note. `work/pydoor/lib-residual-register-was-never-migrated-to-rows.md` re-files the register's open entries.

**Other citations.** Citations of live rows were rewritten to their new paths. In shipped code and docs, a citation of a closed row now names the row's id and this note. Header references (`refs`, `parent`) from other programs' rows to closed LIB rows were dropped. Logs, experiment logs and other programs' row bodies were left as written; they are history.

Recover its program, plan, log and closed rows at 0f01217666:

    git show 0f01217666:work/lib/<FILE>
