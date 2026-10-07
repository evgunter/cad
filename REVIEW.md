IN PROGRESS

# Review: PR #4240 at 3708477019e5dd56487d3770cbceb2d7ce1d7dd7

Draft findings (mutation sweep and the instrumented battery still running):

- MINOR (executed): the corner arm skips sides by EDGE key, so a closed arc
  at a pinch hides a crossing. Probe
  `validate::tests::review_probes::a_crossed_pinch_through_a_closed_arc_passes_check_9_at_head`:
  the crossed host ring passes all of tier 3 (radius-0.2 disc).
- Wedge classes hold (`wedge_holds_reads_every_class`, executed).
- Stale "four words" filters: `validate.rs` `check_9_words` doc, the
  `topo_ring_nesting.rs` copy, and two inline filters in `validate.rs` tests.
