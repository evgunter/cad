Progress note, fix pass 2. Nothing pushed yet. Local head 98ec7ad7a6: items A to J are committed, and origin/main 362cd72fd2 is merged once, cleanly.

**Earlier result:** the armed tour on 650e2ce14e finished 106/106, with an empty armed log (no route difference from main's `lane_of` on any tour face).

**Fix pass status, by item:**
- A: done.
- B: done.
- C: cure kept. The door's `Ok` now means decided; filed row closed.
- D: done. A curved door asked of a plane answers `Defect`.
- E: D9 shows the four unminted witnesses, with their constants moved, and the minted ones as `*_minted` bodies.
- F: measured, and filed P3.
- G: three rows done.
- H: `Route::Trimmed(Trim::{Structural, Read(lift)})`, one `read_domain`, and `Route::ready → Ready`; no `Option`, no `unreachable!`. I kept the route-time read for faces the doors decide no rectangle: reading it at the face's turn instead moved three unminted D9 witnesses to a chord-pass refusal.
- I: filed.
- J: retitled.

**Running now, detached:** mesh at three ε, geom-brep, topo and step-import, gates, lint, doc-gate, clippy. Next come the 11 mutants, the armed mesh differential, the viewer rows, sweep's meshing modules and the tour. Then I push and send the full report.
