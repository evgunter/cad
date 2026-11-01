---
id: section-cert-reach-parallels-extent-ball
kind: issue
title: section_cert's Reach is a parallel type to ExtentBall, adapted at every use; line_tilt is hosted in carrier_eq for two consumers
status: open
opened: 2026-10-11
priority: P1
cost: E
---


Left by PR 4657's re-review (style S1/S2).

- `crates/topo/src/boolean/section_cert.rs` `Reach` (~L320) survives
  beside `geom_brep::ExtentBall`. The new `Reach::ball()` adapter
  (~L332) is called at every use, and `ops.rs` `overlap_reach`
  (~L1242) still builds a `Reach`. That leaves two vocabularies for
  one ball, one adapting to the other. Retire `Reach` in favour of
  `ExtentBall`, or say what `Reach` carries that a ball does not.
- `carrier_eq.rs` `line_tilt` (~L589) is now `pub(crate)` and read by
  `section_cert`: a shared helper hosted inside one of its two
  consumers. Give it a neutral home.


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Mechanical.** `section_cert::Reach` is literally `{centre, radius}`: delete it for `ExtentBall`. `carrier_eq::line_tilt` moves down, out of its consumer. The real signal is in cluster C (`extent-ball-and-reach-are-two-statements-of-one-extent`). DESIGN home: `[one-way.readings]`.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: two vocabularies for one ball: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
