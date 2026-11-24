# CONIC — the plan

No orchestrator holds the track. Live state is `work/conic/log.md`'s tail and the item files beside this plan.

Branch prefix **`conic/`**; away-channel tag `(CONIC orchestrator)`.

## Charter

Every reading of where a conic is, is read from the point the conic stores (`crates/geom-core/README.md`, "Float rounding", the conic clauses; Ev on PRs 7258 and 7523). The track builds what PR 7523 decided and then removes the doors that let a reader take a conic's centre or frame as a position.

## What was decided, and where it is written

PR 7523's body holds the three designers' final texts (A, B, C; they agree, and differ only in two details named below). The fork's seven rounds, the first reports and what Ev was sent are on `analysis/design-fork/conic-reader-interface`; `docs/DESIGN-FORK-LOG.md` row 305 is the summary. In one paragraph:

A conic states where it is as one opaque position about the point it stores. One polynomial along a conic, in `geom`, is built from that position; `geom` states numbers (values with a rounding bound read at the point, ranges, coefficients about an anchor the caller names, and at degree one the extremes and roots posed by the placement's own rule) and `geom-brep` makes every decision. A point is read against a conic three ways (its parameter, its height over the conic's plane, its level). Two conics are compared through three doors. The centre is a wrapped datum for reporters and for mints of a new entity's datum; asking for it never panics and no decision reads it. A conic made from a conic is a map that keeps its placement. `Conic::frame`, `along`, `semi_axis_vectors` and `unit_in_plane` leave, and `u_ref` and `v_ref` leave the trait in the end state. Verdicts read from centre-built conics may move, each judged against an oracle when its unit lands.

**Where the three texts differ, to be measured by the unit that meets it:** who forms the quartic ladder's coefficients from the polynomial's jets (the polynomial on request, or `geom-brep`); the first-degree root form of an arc-held conic on `Interval`.

## Posture

State on 2026-11-23, at the split. ONEROUND still holds two conic units in flight, and this track's rows build on them:

- `oneround/conic-readers` (PR 7510): directions, cover and extent read through the conic, and the gate that pins every remaining read of a conic's centre or frame (`scripts/gates/conic-centre-reads.counts`, an `#[expect]` per function). In its fix pass after a dual review.
- `oneround/conic-pair-readers` (stacked on it): every reading `curve_pair` makes of where a conic is, re-posed from the stored point, after nine wrong answers were measured at the curve-level doors for an ellipse held on its arc (`work/oneround/an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre.md`). It adds an opaque height to `Conic` that becomes the polynomial's first degree.

**A recommended order.** A unit may stack on those two branches rather than wait for them.

1. **The polynomial** (`the-certified-lanes-harmonics-of-a-conic-are-formed-about-its-centre`, P1, H): the position, the polynomial in `geom`, and the certified lane's readers moved onto it. Everything of degree two and above waits on it. A scratch polynomial and four measurements are on `oneround/conic-harmonic-probe`.
2. **Beside it:** `a-conic-made-from-a-conic-is-rebuilt-from-its-centre-and-frame` (P1, M: the maps) and `two-conics-are-compared-through-three-doors` (P1, M).
3. **After the polynomial:** `the-cone-sections-chart-image-and-its-kin-are-stated-from-the-conics-centre` (P1, H; parked on it).
4. **The anchor at a general point** (`a-section-conic-is-anchored-at-a-general-point-of-its-arc`, P1, H), which also owes `u_ref` and `v_ref` leaving the trait; then **the four mints** (`a-construction-whose-centre-runs-off-still-stores-a-centre`, P1, H; parked), then **the circle's datum reads** onto `Centre` (`a-circles-centre-is-read-as-a-datum-under-the-conic-gate`, P2; parked on the mints).
5. **Last:** the gated doors leave and the gate retires with them.

`the-windows-level-jet-is-read-exact-where-its-rounding-grows-with-the-aspect` (P1, M) is parked on `oneround/conic-pair-readers`, which re-poses the level it is about.

## One constraint on every unit

**The remaining readers are converted before the edge certificate's size limit is lifted for a conic held on its arc.** That limit (`KernelLimit`, about ε/6e-16 of semi-axis: 1.7e6 m at ε 1e-9) is what keeps every wrong answer measured so far out of any built body: the largest section a door builds is two decades below where the readers go wrong (`analysis/oneround/conic-suspects/task3.md` on `oneround/conic-suspects-probe`). A unit that loosens it, or mints an arc-held conic past it, exposes the readers the sweep marks suspect and nobody could run (`sweep2-readers.md`, `task1.md`, `task2.md`).

## Exit criteria

- No function outside `conic.rs` reads a conic's centre or frame to find a position: the gate's count is zero and the gate, with `Conic::frame`, `along`, `semi_axis_vectors`, `unit_in_plane`, `u_ref` and `v_ref`, is gone.
- The seven reproductions (`analysis/oneround/conic-suspects/src/suspects_repro.rs`) and the suspects' ladder (semi-axes 1e2 to 1e9, both ε) answer rightly or refuse at every size, as rows of the suite.
- Every verdict a unit moved on a centre-built conic was judged against an oracle, in that unit's PR.

## Tail

`a-conics-points-are-restated-in-centre-form-outside-its-evaluator` (P2), `an-arc-held-ellipse-is-charged-a-centre-held-ones-resolution-and-hulls-its-phase-across-an-interval-seam-turn` (P2), and at P3 the rows about a far-centred conic born with its centre (an import, an authored arc, every circle mint), the arc cover, the plane door's uncertified root, the test oracle, and `a-quadric-surface-is-stored-by-a-datum-that-may-lie-far-from-its-face` (`design`: the same defect one dimension up).
