---
id: a-constructors-whole-wedge-read-bisects-interval-drives-at-the-corner-sign
kind: issue
title: The constructors' whole-wedge read escalates a corner's sign and wedge over wide interval boxes, so a drive bisects where the one-witness class read did not, and the witness reads the tier proved are numeric at the stations
status: open
opened: 2026-11-12
---


Filed by ONEDOOR (wedge fold stage 2, `onedoor/wedge-fold-2`), from the interval ledgers that branch re-pins.

**What changed.** The extrude's struts and cap rims and the revolve's upgrades (`sweep::extrude`'s strut loop and `upgrade_rim`, `sweep::revolve::upgrade::upgrade_intersection`) now describe each edge they write by its whole wedge, `topo::edge_wedge_description`: `geom_brep::edge_wedge_over` at the edge's 7 interior stations, each reading the arm, the wedge and, at a corner, its sign (`dihedral_sign`). They used to read `dihedral_class` at one witness, the strut's chord midpoint or the carrier's middle.

**What it costs a drive.** On `m10_3_driver_interval`'s two-parameter plate at ±0.1 the drive bisects 17 times where it bisected 3 (`Receipt { certified: 0, refused: 18, splits: 17 }`, was `{ 0, 4, 3 }`). A temporary print in `editor_core::drive`'s `indeterminate` showed every bisection's escalation named `dihedral_sign` (34) or `dihedral_wedge` (6). Every leaf still refuses on `pcurve_cylinder_chart_axial`, the pinned main defect (`work/pcert/the-cylinder-rim-axial-decision-refuses-every-leaf-of-the-two-parameter-drive.md`), so no certified leaf is lost on this fixture. On a fixture that certifies, the same escalations would split boxes the drive certified whole. The plate's corners are near right angles, so the signed margin `((n₊ × n₋)·τ̂)·arm` is about the arm in size; an enclosure of it that straddles zero over a box is the interval's overestimate, not a corner that turns.

**What it costs the tier.** Symbolic falls where the witness read was a theorem and no station's read is: the link 991 → 987, the bracket 1599 → 1597, the pad 1697 → 1689 (`m10_9_pins_interval`), the slab's plain drive 2898 → 2835 (`m10_8_pins_interval`), and the pad's ladder retries 34 → 32 (`sym_9_retry_interval`). Numeric rises by the stations' reads, about +40% on the plate (824 → 1204).

**The question.** Whether a station's corner sign, and the wedge, can be read as the tier reads the witness, or the drive should not bisect on a sign whose corner is decided at every station of the nominal. Neither is ONEDOOR's ground. `sym11_exact_channel_rows` is red on main already; this branch moves its numeric column too and does not re-pin it.
