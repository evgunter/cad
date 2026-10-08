# TRIM leaves the tracker — 2026-10-01

TRIM — the NURBS trim frontier — opened 2026-09-03 as PCURVE's P-2
residue and closed 2026-10-01 **on its exit walk** (`docs/TRIM-EXIT-WALK.md`),
which Ev ratified by merging PR 3620. The walk checked the plan's exit
shape clause by clause; every clause was met and every row was closed.

**What it delivered**, each under its own spec, ledgered at its merge:
the de Boor collapse extractor for interior iso-curves (TRIM-1, #2095),
which let the P-2 body mint and validate at rest; the trimmed-region
quadrature and tessellation of a face carrying a `General` pcurve
(TRIM-2, #2564 and #2863); the chart-boundary description and the
clearance window that reads it (TRIM-3, #1911 and #2554); and
`boundary_iso_u/_v`, `interior_iso_u` refusing a corrupt net instead
of panicking (#3525). `docs/PCURVE-P2-SPEC.md`, its charter input, left
`docs/` as superseded (`pcurve-p2-spec.md` in this directory).

**The residue went before the sweep.** On 2026-09-20 (Ev, in-chat: cut
the program to what it would finish itself) CHART opened with 31 of
TRIM's rows on the chart side; two more `pcurves.rs` rows followed;
the chord-count arithmetic class went to TESS; the last unit's sweep
filed one row on PCERT. Nothing open was left in `work/trim/`.

**Eight `refs:` entries in other programs' rows named TRIM ids and
were stripped** — `chart` ×7 (`clearance-window-tightening-needs-chart-boundary`),
`iso` ×1 (`interior-iso-curve-de-boor-extractor`) — because `work.py
lint` validates `refs` against live items. Prose citations of
`work/trim/…` resolve at the SHA below.

**A/B record.** Blocks TRIM-B1 (concluded) and TRIM-B2 (slots 0–1
concluded, slot 2 unspent at the 2026-09-23 suspension) are folded in
`docs/MODEL-AB-LOG.md`; band 2500–2599 stays claimed with 2504 its
last ordinal.

Sweep SHA `ca84a8fc8461287716d172ffe6785c896fc3960d`.

    git show ca84a8fc8461287716d172ffe6785c896fc3960d:work/trim/<FILE>
    git show ca84a8fc8461287716d172ffe6785c896fc3960d:docs/TRIM-EXIT-WALK.md
