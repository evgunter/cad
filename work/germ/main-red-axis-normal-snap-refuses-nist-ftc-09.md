---
id: main-red-axis-normal-snap-refuses-nist-ftc-09
kind: issue
title: main-red: since PR 7171's fix pass made the snap refuse an in-band pair of every kind, nist_ftc_09 refuses at its in-band tangent pair #5130 (step_snap_relation) and three slow-set rows go red
status: closed
opened: 2026-11-20
closed: 2026-11-23
priority: P1
cost: E
---


Found by EXCH's unit-A lane running the full step-import slow set (`cargo nextest run -p step-import --profile ci --ignore-default-filter --no-fail-fast`). The rows are reproduced on a clean `origin/main` at `46e89240`. They are slow-set rows, so PR 7171's per-PR gate never ran them; the nightly will.

Red rows, each refusing `nist_ftc_09_asme1_rd.stp` at entity #5130 with `step_snap_relation` in the band:
- `snap::tests::nist09s_five_cylinders_snap_and_its_printed_tangent_edges_are_read_off_them`
- `wild::wild_refusals_are_typed_and_name_their_class`
- `tier_gate::every_corpus_import_passes_the_shared_gate` (fails only at that file)

The likely cause is commit `c25c174105` ("step-import: an axis-normal relation in the file's band refuses at the snap"), which merged with GERM's PR 7171 (`germ/section-class-lane-choices`) and changes `crates/step-import/src/snap.rs`. That file has no B-spline surface, so EXCH's promotion units cannot touch it.

Per `memories/agent-lane-operations.md` the lane that caused a red owes the fix, so the row is on GERM's slate. The snap catalogue is IMPORTGATE's ground (`work/importgate/log.md`, PR 7135), so the fix should agree with IMPORTGATE's census, which found no axis-normal relation Zero at the file band in the corpus.

Signed: (EXCH orchestrator)

## 2026-11-22 (GREEN): the render lane is red on it too

The nightly's `render lanes / scene inputs + uv sheet + wild montage` job panics on the same import ("nist/nist_ftc_09_asme1_rd.stp: refusal drifted off its pinned class \"(step_edge_on_description), in the band\"", entity #5130, `step_snap_relation`) on 11-21 (runs 44870651337 and 44927630239), and skips the three render lanes after it. No render re-baselines while this stands.

## Fixed

Reproduced on `origin/main` at `43295779ee`: the three rows red, each at
`#5130`, `Ambiguous { step_snap_relation, gap 5.6844521718398856e-5 m }`.

**Neither premise is wrong, and the relation is not axis-normal.** The
refusing pair is a **tangent** one: plane `#5116` (origin z
0.10908707180743679 m, normal −z) and cylinder `#5130` (axis along y at
z 0.11359853590371839 m, r 0.004568308617999999 m), along edge `#5118`.
Its gap is |0.1135985 − 0.1090871| − r = −5.6845e-5 m: the cylinder `#5130`,
between parallel planes `#5116` and `#5144`, bites `#5116` by 56.8 µm.
The pair `#5130` × `#5144` (edge `#5132`) reads in band too; `#5118` is
first only by edge-id order. At ε_in 3.3816e-5 m (k 10) the band is
(33.8 µm, 338 µm), so the reading is `InBand`, at every ε (the band's
floor is ε_in). The axis-normal candidate of that pair reads apart (its
axis lies in the plane). So IMPORTGATE's census, which counted no
axis-normal relation Zero at the file band, holds, and `c25c174105`'s
axis-normal refusal never fires on this file. The red is from PR 7171's
fix pass, `2cc97f1ce7` (review MINOR 2): the snap refuses an in-band pair
of **every** kind on its own reading. Before it, the snap passed the pair
over and the build refused the file first at `#4950` (a printed line
180 µm off a snapped tangency), which the rows pinned.

**The refusal is right; the pins were wrong.** With the snap's in-band
refusal removed, the build reads the same pair at `#5118` as
`Ambiguous { step_edge_on_description, gap 5.6844521718398856e-5 }`,
the same number. Both stages read one fact, that the file does not decide
whether `#5130` is tangent. D7 (`docs/DESIGN.md` `[import]`): "Data
ambiguous at ε_in scale fails with a typed ambiguity error rather than a
guess". Step 3 snaps only relations that read Zero. `snap.rs`'s module
docs say a pair read in band refuses at the snap, and
`a_relation_in_the_files_band_refuses_at_the_snap` pins that. The rows
had pinned which of the file's defects is reached first. The snap runs
before the build, so it is now `#5130`.

This is a refusal-class change: `nist_ftc_09` moves from `MissInBand`
(`step_edge_on_description`, `#4950`) to `Ambiguous`
(`step_snap_relation`, `#5130`) in `tests/wild.rs`, in `tests/tier_gate.rs`
and in `demos/wild`'s `WILD_REFUSALS` (the render lane's pin). The snap
unit row is re-aimed at the in-band pair. Its old content, the five
snaps and the build's readings of the printed crossing lines, is no longer
reachable on this file, so `build::choose_edge`, the old row's only
caller, is gone. The printed-crossing-line reading has a synthetic
witness instead:
`a_tangent_edge_printed_as_a_crossing_line_is_read_off_its_ruling`
(`tests/a_relation_the_file_band_reads_zero_is_snapped.rs`). Mutants: the axis-normal-only condition reds the snap row
and `wild_refusals`; refusing on the earlier record reds the snap row
(5116 for 5130).

