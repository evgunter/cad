---
id: the-witness-ladder-reads-one-interior-candidate-per-face-so-one-in-band-hides-a-definite-one
kind: issue
title: The witness ladder probes one interior candidate per planar face, so a face whose first candidate reads in band refuses though another reads off the band
status: open
opened: 2026-10-25
priority: P3
cost: E
---


Found by FLUSH's needle-cavity lane (`flush/needle-cavity-section-loop`,
2026-10-25). `crates/topo/src/stands.rs` has no owning program.

**What happens.** `stands::ladder`'s rung 3 takes, per planar face, the
first candidate `face_interior_point` certifies inside the face, and
probes that point alone. If the probe reads in band, the face offers
nothing more, though a later candidate may lie well off the band.

**Measured** on main (`28e9e447`), ε 1e-6, with the probe
`crates/topo/tests/needle_cavity_probe.rs` (`moved` pose, 30ε). The
union `(brick ∖ slab) ∪ fill` leaves a `w × w` channel, and the fill's
face toward it is `w = 30ε` wide. Its first candidate is a corner
triple's centroid, `w/3 = Kε` off the cavity floor. Rounding puts its
margin at `9.999999999996e-6`, against `escalate = 9.999999999999999e-6`, so it reads
in band. The midpoint of a floor corner and a ceiling corner is `w/2 =
15ε` off both walls and would read definite. The section-loop
read refused `SectionLoopUndecided`, and the axis-aligned twin builds.
FLUSH's branch now reads that loop off its seam germs
(`join.rs` `seam_germ_side`). The ladder's other readers do not: the
uncut-shell verdict (`boolean::shell_witness::shell_verdict`,
`ShellWitnessExhausted`) and check 10 / the result sort
(`stands::witness_insides`). No instance has been built for them.

**What is owed.** Decide whether rung 3 probes each certified
candidate in turn until one is decisive (each is a point of the
face's relative interior, so any decisive one names the side). If so,
build an uncut-shell instance to pin it.
