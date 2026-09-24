---
id: rustdoc-count-sentences-outside-the-two-doors-are-unguarded
kind: issue
title: Doc comments across crates/*/src carry counts of the tree that nothing holds true
status: open
opened: 2026-09-24
priority: P4
cost: D
refs: [a-doors-rustdoc-carries-an-unguarded-census-sentence]
---


## Finding

- **Where**: 140 doc-comment sites across `crates/*/src`, listed below
  with the phrase each instrument matched.
- **Importance**: low — prose hygiene (P4); each sentence is a count
  of the tree, true when written, that nothing re-measures.
- **Confidence**: sure the hits exist; NOT triaged — most are likely
  a design statement (*"ONE definition, three callers"*, *"the three
  places a dialog can open"*) rather than a drifting population, and
  deciding which is the work.
- **Raised by**: `dup/topo-fixture-batch`, 2026-09-24, out of the class
  sweep for `a-doors-rustdoc-carries-an-unguarded-census-sentence`,
  which folded the hits in its own two files (`topo/src/body.rs`,
  `sweep/src/test_support.rs`) and left these.

**The rule the triage applies** is `work/dup/plan.md` item 13: a door's
rustdoc carries an invariant for a user, not a measurement for a future
lane. A hit that counts a population that can grow or shrink (callers,
copies, suites) is rewritten to say what the door is for, or to point
at a row holding the dated measurement; a hit that enumerates named
things in place (arms of one function, three sites named in full)
stays.

## Instruments, at `6db5b87f2`

Over every tracked `crates/*/src/**/*.rs` file, each file's consecutive
`///`/`//!` lines **joined into one string** (a line-shaped pass
missed 50 of 107, the row this came from among them):

1. a count of three or more, word or digit, then within two words
   *sites, copies, spellings, callers, call sites, suites, consumers,
   places, files* — 107 hits;
2. an ordinal (*third … twentieth*) before the singular of those
   nouns — 41 hits.

**Blind spots**: `//` comments (not doc), `crates/*/tests`, `demos/`,
`tools/`, and a count whose noun is not in the list (*"the four
lines"*, *"fifteen INDEPENDENTLY CHECKED facts"*). Minus the eight
folded in the originating PR, the list is below. Six of them were
read there and judged named enumerations that stay (see that row's
closing section); the rest are unread.

```
  crates/editor-core/src/doc.rs:696: four sites
  crates/editor-core/src/doc.rs:997: FOUR callers
  crates/editor-core/src/edit.rs:647: three callers
  crates/editor-core/src/eval/measure.rs:420: four call sites
  crates/editor-core/src/eval/measure.rs:437: fifth lever site
  crates/editor-core/src/eval/mod.rs:899: 150 places
  crates/editor-core/src/eval/wire.rs:4576: Three callers
  crates/editor-core/src/mc.rs:638: third copy
  crates/editor-core/src/names/defer.rs:224: third copy
  crates/editor-core/src/names/emit_union.rs:21: three consumers
  crates/editor-core/src/names/interrogate.rs:379: sixth copy
  crates/editor-core/src/node.rs:1059: three consumers
  crates/editor-core/src/node.rs:1326: three callers
  crates/editor-core/src/node.rs:1326: three copies
  crates/editor-core/src/node.rs:2839: three callers
  crates/editor-core/src/program.rs:2825: third spelling
  crates/editor-core/src/report.rs:577: Four decimal places
  crates/editor-core/src/resolve/pick.rs:1387: 43 call sites
  crates/geom-brep/src/dihedral.rs:211: three sites
  crates/geom-brep/src/intersect.rs:1670: third classification no consumer
  crates/geom-brep/src/offset_fit.rs:1063: three copies
  crates/geom-brep/src/offset_fit_lane.rs:22: five sites
  crates/geom-brep/src/offset_meters.rs:298: third consumer
  crates/geom-brep/src/pcurve_cache.rs:1902: Three consumers
  crates/geom-brep/src/props/curved.rs:784: three torus consumers
  crates/geom-brep/src/props/curved.rs:862: Three sites
  crates/geom-brep/src/props/curved.rs:1376: third caller
  crates/geom-brep/src/props/curved.rs:1680: three callers
  crates/geom-brep/src/props/quad.rs:2317: three harder places
  crates/geom-brep/src/props/quad.rs:2345: three places
  crates/geom-brep/src/props/quad.rs:4618: third spelling
  crates/geom-brep/src/ssi.rs:206: four call sites
  crates/geom-brep/src/ssi/enclose.rs:117: three sites
  crates/geom-brep/src/ssi/enclose.rs:260: third consumer
  crates/geom-brep/src/tangent.rs:103: THREE consumers
  crates/geom-core/src/k_stats.rs:94: 530 call sites
  crates/geom-core/src/k_stats.rs:94: 82 files
  crates/geom-core/src/k_stats.rs:146: Four shipped sites
  crates/geom-core/src/linalg/affine.rs:732: third spelling
  crates/geom-core/src/linalg/frame.rs:1242: four normalizing sites
  crates/geom-core/src/linalg/mat.rs:151: three spellings
  crates/geom-core/src/real.rs:1152: three files
  crates/geom-core/src/real.rs:1153: fourth file
  crates/geom-core/src/real.rs:1158: five files
  crates/geom-core/src/sym.rs:653: three callers
  crates/geom-core/src/sym/discharge_pins.rs:21: five files
  crates/geom-core/src/sym/discharge_pins.rs:96: fourth place
  crates/geom-core/src/sym/profile.rs:89: three callers
  crates/geom-core/src/sym/quotient.rs:169: fifth site
  crates/geom-core/src/tolerance.rs:649: 256 call sites
  crates/geom/src/curves/nurbs.rs:1538: third spelling
  crates/mesh/src/chords.rs:44: third caller
  crates/mesh/src/chords.rs:761: third caller
  crates/mesh/src/nurbs_cert.rs:3594: third spelling
  crates/mesh/src/sizing.rs:142: five predicate call sites
  crates/pncad-py/src/errors.rs:84: three spellings
  crates/pncad-py/src/prose_census.rs:44: three places
  crates/pncad-py/src/py/analysis.rs:78: third place
  crates/pncad-py/src/tags.rs:2373: three raise sites
  crates/pncad-py/src/tags.rs:2826: THIRD spelling
  crates/pncad/src/tolerance.rs:10: 256 call sites
  crates/profile/src/path.rs:4732: fourth caller
  crates/profile/src/path.rs:4741: third caller
  crates/profile/src/path.rs:5213: ten decimal places
  crates/step-import/src/chart.rs:9: 13 measured files
  crates/sweep/src/blend/battery.rs:1678: three sites
  crates/sweep/src/blend/mod.rs:804: nine places
  crates/sweep/src/blend/mod.rs:835: third copy
  crates/sweep/src/blend/mod.rs:1624: three sites
  crates/sweep/src/blend/open/planar.rs:83: three places
  crates/sweep/src/chamfer.rs:11: three places
  crates/sweep/src/extrude.rs:429: three copies
  crates/test-utils/src/roster.rs:11: 192 files
  crates/test-utils/src/roster.rs:35: eighteen sites
  crates/test-utils/src/roster.rs:300: three consumers
  crates/test-utils/src/source.rs:370: Three sites
  crates/test-utils/src/source.rs:1335: Fifteen copies
  crates/test-utils/src/source.rs:1339: fifteen call sites
  crates/test-utils/src/source.rs:1350: fifteen copies
  crates/test-utils/src/source.rs:1429: three suites
  crates/test-utils/src/vacuity.rs:95: third spelling
  crates/test-utils/src/vacuity.rs:288: nine copies
  crates/test-utils/src/vacuity.rs:303: ten test files
  crates/topo/src/boolean/boxes.rs:1888: 16 in three places
  crates/topo/src/boolean/boxes.rs:1888: three places
  crates/topo/src/boolean/contain.rs:560: three of its sites
  crates/topo/src/boolean/join.rs:1339: three sites
  crates/topo/src/boolean/mod.rs:2567: three places
  crates/topo/src/boolean/solid_contain.rs:1426: fourth spelling
  crates/topo/src/census.rs:93: three sites
  crates/topo/src/census.rs:869: THIRD spelling
  crates/topo/src/census.rs:873: three sites
  crates/topo/src/census.rs:1026: Three further copies
  crates/topo/src/chart_region.rs:1295: three named places
  crates/topo/src/chart_region.rs:1811: THREE places
  crates/topo/src/contact.rs:6: three consumers
  crates/topo/src/euler.rs:130: three   places
  crates/topo/src/euler_ring.rs:1106: third spelling
  crates/topo/src/merge_faces.rs:2252: three of its sites
  crates/topo/src/null.rs:71: 4 call sites
  crates/topo/src/offset_nappe.rs:23: four more places
  crates/topo/src/ray_parity.rs:2: three consumers
  crates/topo/src/review_d18.rs:78: six files
  crates/topo/src/review_d21_probes.rs:36: seventeen sites
  crates/topo/src/sector_shape.rs:683: six genuinely retired spellings
  crates/topo/src/sector_shape.rs:686: six spellings
  crates/topo/src/sector_shape.rs:695: third file
  crates/topo/src/seqgen.rs:124: 374 strut sites
  crates/topo/src/split.rs:76: 15 consumers
  crates/topo/src/splitting/rules.rs:8: three callers
  crates/topo/src/test_support_fixtures.rs:247: three axes the callers
  crates/verbs/src/verb.rs:209: third place
  crates/viewer/src/app.rs:533: three places
  crates/viewer/src/app.rs:543: three places
  crates/viewer/src/camera.rs:1119: four sites
  crates/viewer/src/frame.rs:172: four call sites
  crates/viewer/src/frame.rs:1348: twelve sites
  crates/viewer/src/frame.rs:1522: three places
  crates/viewer/src/frame.rs:1614: twelve sites
  crates/viewer/src/input.rs:154: four spellings
  crates/viewer/src/input.rs:175: fourth spelling
  crates/viewer/src/narrowing.rs:8: five sites
  crates/viewer/src/pane/properties.rs:352: third place
  crates/viewer/src/pane/view.rs:107: third spelling
  crates/viewer/src/pane/view.rs:271: third spelling
  crates/viewer/src/pickindex.rs:1529: fourth spelling
  crates/viewer/src/props.rs:160: Three call sites
  crates/viewer/src/readout.rs:168: three places
  crates/viewer/src/scene.rs:52: four production sites
  crates/viewer/src/seats.rs:361: third consumer
  crates/viewer/src/session.rs:306: three call sites
  crates/viewer/src/session/op.rs:919: three spellings
  crates/viewer/src/session/op.rs:1023: three places
  crates/viewer/src/tree.rs:721: seven suites
  crates/viewer/src/tree.rs:734: five places
  crates/viewer/src/widgets.rs:369: twelfth site
  crates/viewer/src/widgets.rs:443: twelfth site
  crates/viewer/src/widgets.rs:2877: twelfth site
  crates/viewer/src/widgets.rs:2927: twelfth site
  crates/viewer/src/widgets.rs:3165: twelfth site
```
