---
id: separating-axis-reads-swallow-their-unit-escalations
kind: issue
title: separating.rs drops an undecided axis with .ok() and leaves its escalation on the log
status: open
opened: 2026-10-14
priority: P2
cost: E
---

Found by PR 5140's review (style Q1, class sweep), filed by its fix pass.

Two candidate-axis reads in `crates/topo/src/boolean/separating.rs` turn
a refused unit decision into "no axis" with `.ok()`, and splice nothing:

- `planar_axis` (~:139): `UnitVec3::levered(n.vec(), PAIR_NORMAL, band,
  (hi - lo).norm()).ok()`.
- `apart` (~:193): `UnitVec3::new(q - p, PAIR_AXIS, band).ok()` for the
  axis between the two anchors.

When the band leaves the length in band, the decision records an
escalation on the open `k_stats` frame, and the read then answers
`None` and the caller tries the other axes. The repo's convention for a
read whose undecided outcome is overruled is `k_stats::detached` plus
`splice_superseded` (`boolean/rim_wedge.rs`, `boolean/carrier_eq.rs`,
`boolean/contact_verify.rs`; PR 5140 does the same in `boxes.rs`
`face_box_rule`).

**Whether the outcome rests on it — not decidable at the site.** If
another axis certifies the pair apart, the outcome is definite and the
escalation is superseded, so it should not stay on the log: the
subdivision driver would read it as the node's cause. If no axis
certifies, `apart` answers `false`, and the dropped axis is one reason
the pair was not separated, so the escalation is arguably live. A fix
probably wants `apart` to splice the undecided-axis logs superseded on a
`true` answer and normally on `false`, the same shape as `rim_wedge`'s
`same_circle` loop. `operand_axes`, which collects `planar_axis`
results, needs the same look.

## Slate triage (2026-10-26, CLEAVE)

- Banded P2: an overruled unit decision's escalation stays on the log the subdivision driver reads: interval and error propagation, P2 by `work/README.md` Priority.
- Priced `E`, the effort the body describes.
- Moved to HONEPAIR (`work/honepair/`) by `git mv`, id and body unchanged: the P2 track opened for one verdict decided on one margin at its own reach: rows that decide two margins one at a time, compose two readings, or lever a reading at the wrong extent. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
