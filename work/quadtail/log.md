# QUADTAIL — the log

## 2026-10-21 — opened

Cut out of QUAD on its priority seam by QUAD's orchestrator, when QUAD
measured 72.5 budget points against 30.
QUAD kept its P0 and P1 rows. Every row here moved by `git mv` with its
id and body unchanged; the cut banded and priced the rows that had no
band or cost. No A/B band is claimed: the model A/B experiment is
suspended. — (QUAD orchestrator)

- 2026-10-23 — **Seam (QUAD):** PR 5589 (`quad/measure-a-lanes`, measure door unit A) holds `props/quad.rs` too (a double claim). Unit A takes the narrowest round and leaves `a-face-runs-rounds-it-never-intersects` (the intersection) open for you. The seam is `quad::narrower` and `FaceRun::advance`. (QUAD orchestrator's lane)

- 2026-10-23 — **Seam (QUAD):** PR 5628 (`quad/measure-b-door`, measure door unit B) touches `props/quad.rs`: a `test-support` rounds counter (`rounds_run_by`) in `Run::stop`, and docs. No lane changes. (QUAD unit B lane)
- 2026-10-23 — **Seam (ISO):** `a-fitted-face-trimmed-by-a-section-has-no-volume-rule` moved to `work/iso/` to ride with `trimmed-quadrature-composite-rounds` (P1). ISO now dispatches that row on `iso/trimmed-composite-rounds`, and building it closes yours (your row's own last paragraph). `props/quad.rs` is shared ground, and the lane announces its seam in its PR. (ISO orchestrator)
- 2026-10-26 — (tracker sweep) Every live row re-read against main `bb25fa4f`. All 15 rows are still real, and none closed.
  - `purchasable-area-tightness-valve` set `deferred`: its own ratification, S-CERT Q1 (Ev, 2026-08-29), says not to build it until a consumer asks. It is re-priced `H` from legacy `D`.
  - `q9-refinement-invariance-does-not-classify-the-rule-order` re-priced `M` with `design: true`, from legacy `D`.
  - The plan notes that `patch-face-area-gauge-panics-on-random-bicubic-faces` shares the A2 tripwire with QUAD's open P2 `an-integral-patch-with-knots-hugging-its-edges-trips-the-area-gauge`.
  - Not merged back into QUAD: QUAD is active and at 56/45.
- 2026-10-26 — **Seam (QUAD):** PR 5741 (`quad/measure-c-no-default`, measure door unit C) closes `quad-round-capped-loops-do-not-read-the-shared-stop-rule` into the unit and removes `PropsError::QuadratureBudget`: every open QUADTAIL row whose premise is a budget refusal (`a-micron-thin-cylinder-wall-…`, `a-uniformly-scaled-rational-patch-…`, `cylinder-quadrature-integrates-loops-…`, `purchasable-area-tightness-valve`) now reads a bracket with `stop` `Floor`/`Wall` where it read a refusal. `props/quad.rs` changes only in its tests (`QUAD_TARGET_LEN_FACTOR` is test-only). (QUAD unit C lane)
