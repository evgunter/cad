---
id: lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late
kind: issue
title: a lofted Circle section is unmeshable, and the refusal arrives three steps downstream
status: open
opened: 2026-09-10
---


Measured while giving the teapot a lofted spout (`demos/tour/src/teapot.rs`).

A `Node::Loft` whose sections are `LoopProgram::Circle` builds, validates
at tiers 1–3, and reports its mass properties — and then REFUSES to
tessellate:

```
UnsupportedNurbsFace {
    face: FaceKey(3v3),
    note: "NURBS direction with a C⁰ crease (interior multiplicity =
           degree) — the interpolation Taylor bound needs C¹; split the
           face at the crease",
}
```

**The mechanism, and it is not a defect in the refusal.** A `Circle`
loop is TWO segments, so each lateral wall of the loft spans a
SEMICIRCLE — and a semicircle is not one rational Bézier. The kernel
joins two quarter arcs at an interior knot of multiplicity = degree,
which is exactly the C⁰ crease the note names. The tessellator is
right to refuse it and its payload carries the remedy.

**What the finding IS: nothing says so until three steps downstream.**
The loop is authored at `Node::Profile`. The wall is minted at
`Node::Loft`. The refusal arrives at `mesh`, after the body has been
built, validated and measured — and it names a `FaceKey`, which is the
one identifier a scene cannot map back to the loop it wrote. A scene
that authored a perfectly ordinary round section learns that the
section shape was the problem by reading a Taylor-bound message about
knot multiplicity.

**The workaround is a door, not a hack**, which is why this is an issue
about ERGONOMICS rather than about capability:
`LoopProgram::CircleSplit { n: 4 }` — the grammar's own
declared-subdivision closed carrier — gives each lateral wall ONE
quarter arc and no interior knot, and the same loft meshes. The teapot
ships that way and says so at the site.

**Why no earlier scene met it.** `twisted_tube` lofts SQUARE sections,
whose sides are separate segments already, so every wall's u direction
is a single straight span. Every round body in the corpus until now
came from a `revolve` or the tube door, which mint exact analytic
surfaces rather than a fitted skin. The lofted round section is the
first construction in the corpus that reaches this door at all.

**What would close it**, in rising order of cost:

* the loft's own emitter could split a section segment whose carrier
  carries an interior knot of multiplicity = degree, so the wall it
  mints is C¹ by construction and `Circle` and `CircleSplit(4)` build
  the same body;
* failing that, `Node::Loft` could REFUSE such a section at
  evaluation, naming the loop and the segment — which is where a scene
  can act on it — rather than letting the body reach `mesh`;
* failing both, `LoopProgram::Circle`'s doc could say that a lofted
  circle is unmeshable and point at `CircleSplit`. That is the cheapest
  and it is still better than a `FaceKey`.


**Update: the teapot no longer ships `CircleSplit` here, and the
reason is a SECOND door one step further on.** The paragraph above
said "the teapot ships that way and says so at the site"; it does not
any more. `CircleSplit(4)` did clear this door exactly as recorded —
the loft meshes — and the scene then met the QUADRATURE lane at
`mass_properties`. A circle is rational however it is subdivided, so
every lateral wall is a rational patch, a rational patch is a
quadrature face whose certified enclosure is chased to a width derived
from ε, and at ε = 1e-12 the chase ran out of budget and the reading
refused typed. Measured against a target of 1.024e-9, the last round's
provable width went 2.53e-8 at four arcs, 3.97e-9 at eight, 1.39e-9 at
sixteen: four times the arcs bought eighteen times the resolution and
was still short — by 36%, with decelerating but real returns. That
is NOT a demonstration that no arc count works: a further doubling
would probably have cleared the target, and an earlier draft of this
paragraph overstated the table by saying otherwise. The circle was
ruled out on COST — 32 arcs is 64 lateral walls before the caps, on
one montage cell — and because the certificate would stay a chase
whose margin is whatever round it stops at.

The spout's sections are regular OCTAGONS now. A polygon's sides are
straight, so its lofted walls are POLYNOMIAL and take the integral
lane, which has the exact per-span shortcut the rational lane has
none of — the same reason `twisted_tube`'s square sections were never
in trouble at either door.

**What that changes about THIS issue: nothing, and that is worth
saying rather than closing it.** The finding stands exactly as
written — a lofted `LoopProgram::Circle` is unmeshable and says so
three steps late, naming a `FaceKey` — and all three remedies still
apply. What has changed is only that the corpus no longer has a scene
standing on `CircleSplit` as the workaround, so nothing in the tree
exercises this door on every pass any more. A scene that wants a
lofted round section will meet it again, and will then also meet the
quadrature door behind it; the two are worth reading together, because
the cheapest remedy above (a doc line on `LoopProgram::Circle`) would
now be wrong if it stopped at "point at `CircleSplit`".
