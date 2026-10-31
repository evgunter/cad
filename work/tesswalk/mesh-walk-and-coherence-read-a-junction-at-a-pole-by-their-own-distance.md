---
id: mesh-walk-and-coherence-read-a-junction-at-a-pole-by-their-own-distance
kind: issue
title: The mesh walk's pole_index and coherence's pole exemption each decide a vertex at a pole by their own distance, kept equal by hand, apart from the kernel's one incidence reading
status: open
opened: 2026-10-31
---


The kernel reads "this vertex lies on its chart's singular point" once:
`geom_brep::chart_singularity::vertex_at_singular_point`
(`chart_singularity_at_vertex`, the vertex's 3-D distance to a pole or an
apex, at the caller's band). The loop walk's reset joint
(`topo::pcurves::singular_at`), tier 3's at-vertex placement (`locate`)
and the wrap-tip rule all read it (PR 6097). Two readers outside topo's
validator decide the same fact by their own distance:

- `mesh::walk::loop_polygon`'s `pole_index` (`crates/mesh/src/walk.rs`,
  near line 1002). A junction is identified with `Chart::poles()`'s point
  where `Eps::coincident(|p − pole|)` holds. That is the mesh's own
  inclusive threshold, not the band's three-way decision.
- `topo::coherence::loop_findings`'s pole exemption
  (`crates/topo/src/coherence.rs`, near line 605):
  `(p − pp).norm() <= eps`. Its comment says it is inclusive *because*
  the walk is, so the two have to identify the same junctions. They are
  kept equal by hand.

Inside the band these can disagree with the reset. The mint and tier 3
can read a tip or a pole vertex as on its pole while the mesh and
coherence read it as off, or the reverse. A wrap tip makes this live:
the tip vertex is exactly where the walk resets.

**What it needs:** both read the one reading, either by vertex identity
(the vertex the joint resets at) or through `vertex_at_singular_point`
at the run's band, with the in-band arm refused rather than folded. They
move together: coherence follows the walk.

The RESTCURVE wrap-tip unit's fix pass (PR 6097) filed this rather than
fixing it: `walk.rs` is TESS's and TESSWALK's ground, and the walk's
`Eps` is not a band, so the change is not local.

Signed: (RESTCURVE implementer)
