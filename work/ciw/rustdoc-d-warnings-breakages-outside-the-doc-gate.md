---
id: rustdoc-d-warnings-breakages-outside-the-doc-gate
kind: issue
title: Pre-existing rustdoc -D warnings breakages the doc gate does not render
status: closed
opened: 2026-09-04
closed: 2026-09-10
branch: ciw/citations
refs: [rustdoc-gate-private-intra-doc-links, doc-gate-two-unread-axes]
pr: 2299
---

Found by DOCM-1 (PR #1829) running `RUSTDOCFLAGS="-D warnings" cargo doc
--no-deps` on `topo` and `editor-core` — a pass `scripts/doc-gate.sh
--pr` does NOT make (it renders `--workspace` with its own flags and is
green), so none of these gate today and none is DOCM-1's:

- `crates/topo/src/boolean/mod.rs:31` — unresolved link to
  `SweepStrategy::Idealized` (the variant is gone or renamed).
- `crates/topo/src/boolean/contain.rs:70`, `:532`, `:536`, `:555` —
  public docs link to private items (`boundary_pre_pass`,
  `super::solid_contain::point_on_wall_in_face`,
  `curved_boundary_containment`, `super::solid_contain::cylinder_chart_trim`).
- `crates/topo/src/boolean/rest.rs:492` — public docs link to the
  private `face_plane`.
- `crates/editor-core/src/eval/mod.rs:243` — `contacts` links to the
  private `crate::eval::wire::OpOut`; `:4071` — unresolved link to
  `crate::report`.
- `crates/editor-core/src/node.rs` `payload_names` — links to the
  private macro `name_free_node`.

Either the gate should render the private-item links too (the
`--document-private-items` question the doc-gate header may already
answer) or these eight sites want their links rewritten; whichever, a
reader following any of them today lands nowhere.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/ciw/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Correction to the first bullet (2026-09-06, CIW orchestrator)

`SweepStrategy::Idealized` is **not** gone or renamed. It is at
`crates/topo/src/boolean/reduce.rs:84`, carrying
`#[cfg(feature = "sweep-testing")]`, so the link resolves with that feature
on and is unresolved with it off. That makes both of its sites — the one
above and `crates/topo/src/boolean/reduce.rs:18`, which the bullet list
missed — instances of the in-half broken link that
`doc-gate-two-unread-axes` accepted permanently on 2026-09-04, not
documentation rot. The differential run that would have caught them is not
implementable, for the reason recorded there: the two runs never see the
same site.

Neither is a link-rewrite. What is left for this item is the other six
sites, which are public docs linking to private items and are a real
question about `--document-private-items` rather than a feature axis.

## Closed 2026-09-10 (CIW unit 7, branch `ciw/citations`) — no site here is live

This item asks: "either the gate should render the private-item links
too … or these eight sites want their links rewritten." **The gate
already answers it, on both halves, and every surviving site belongs to a
sibling item that holds the whole class with a decision recorded.**

Nothing is rewritten. Rewriting these links would enact option 2 of a
question recorded as settled on option 1, in `crates/topo` and
`crates/editor-core`, which are other programs' fences besides.

### The gate does not report the private links, deliberately

`scripts/doc-gate.sh:638-639` passes `--document-private-items` on every
`--no-deps` pass, and `:555` sets

```
RUSTDOC_LINTS="-D warnings -A rustdoc::private_intra_doc_links"
```

so the lint is allowed with the argument in the script's header
(`:24-31`, `:1330-1339`). DOCM-1 hit these because it ran
`RUSTDOCFLAGS="-D warnings"` **without** that `-A`. Both runs, on
`origin/main` at `c5558def5`, over the two crates this item names:

```
$ RUSTDOCFLAGS="-D warnings -A rustdoc::private_intra_doc_links" \
    cargo doc -p topo -p editor-core --no-deps --document-private-items --all-features
exit=0        # 0 "links to private item", 0 "unresolved link"

$ RUSTDOCFLAGS="-D warnings" \
    cargo doc -p topo -p editor-core --no-deps --document-private-items --all-features
exit=101      # 119 "links to private item" (topo 64 + editor-core 55), 0 unresolved
```

And the gate itself, run to completion on the same tree:
`scripts/doc-gate.sh --pr` → **exit 0, `doc-gate OK`**, zero
`unresolved link` and zero `links to private item` lines in the whole
log. `scripts/doc-gate.sh --selftest` → exit 0.

### The eight sites, re-derived

Line numbers above have drifted. Measured from the deny run's own
`-->` lines:

| this item's citation | today | class |
| --- | --- | --- |
| `topo/boolean/contain.rs:70` | `:70` `boundary_pre_pass` | private |
| `topo/boolean/contain.rs:532` | `:532` `super::solid_contain::point_on_wall_in_face` | private |
| `topo/boolean/contain.rs:536` | `:536` `curved_boundary_containment` | private |
| `topo/boolean/contain.rs:555` | `:555` `super::solid_contain::cylinder_chart_trim` | private |
| `topo/boolean/rest.rs:492` | `:492` `face_plane` | private |
| `editor-core/src/eval/mod.rs:243` | **`:250`** `crate::eval::wire::OpOut` | private |
| `editor-core/src/node.rs` `payload_names` | **`:2652`** `name_free_node` | private |
| `editor-core/src/eval/mod.rs:4071` | **`:5317`** `crate::report` | **feature axis** |

`rest.rs:493` (`face_plane_source`) and `rest.rs:540` (`face_plane`) are
two more private-link sites in the same file this item never listed.
That under-listing is the point: see the population below.

### `crate::report` is the feature axis, not rot — and it is already counted

`crates/editor-core/src/lib.rs:71-72` gates `pub mod report` on
`#[cfg(feature = "interval")]`, and `interval` is **not** a default
feature (`crates/editor-core/Cargo.toml` declares no `default`). DOCM-1
read it at default features, where the module does not exist. The gate
documents at `--all-features`, where it does — hence the 0 unresolved
links above.

That is the same shape as this item's own `## Correction` retracting
`SweepStrategy::Idealized` behind `sweep-testing`: **both** of this
item's non-private bullets were feature-gated links read at the wrong
feature setting. Both belong to the in-half hole
`doc-gate-two-unread-axes` accepted permanently on 2026-09-04, and
**neither needs re-filing there** — that item's axis-(a) re-sweep counts
`editor-core ×7`, and reproducing it exactly finds those seven to be
`crate::report` plus six `geom_core::Interval` / `crate::clearance*`
sites, `clearance` being `#[cfg(feature = "interval")]` at
`lib.rs:23-24`:

```
$ RUSTDOCFLAGS="-A rustdoc::private_intra_doc_links" \
    cargo doc -p editor-core --no-deps --document-private-items --no-default-features
warning: unresolved link to `crate::clearance::min_separation`   eval/measure.rs:32
warning: unresolved link to `crate::report`                      eval/mod.rs:5317
warning: unresolved link to `crate::clearance::FaceScope`        measure.rs:83
warning: unresolved link to `crate::clearance::min_separation`   measure.rs:97, :516
warning: unresolved link to `crate::clearance`                   measure.rs:659
warning: unresolved link to `geom_core::Interval`                measure.rs:660
```

Seven sites, all feature-gated, none dead. The count matches; the class
is already recorded.

### The seven private sites are a sample of a population held elsewhere

`work/ciw/rustdoc-gate-private-intra-doc-links.md` holds the whole class
with three options and option 1 (keep the lint allowed) as the measured
choice, with a revisit trigger. Its 82 is stale and is corrected in place
in the same PR. Re-derived on `origin/main` at `c5558def5`:

```
$ RUSTDOCFLAGS="-A rustdoc::broken_intra_doc_links" \
    cargo doc --workspace --no-deps --document-private-items
278 reported sites, 11 crates:
topo 63  editor-core 50  geom-brep 40  viewer 35  geom-core 30
sweep 27  geom 11  step-import 8  mesh 6  profile 5  bvh 3
```

`--all-features` — the selection the gate actually documents under —
gives **292 across 12 crates** (`pncad-py` joins with 1; `topo` 64,
`editor-core` 55, `viewer` 41, `sweep` 28, the rest unchanged). Both are
counts of **reported sites**, one rustdoc warning each, not distinct link
spellings.

The deny must be run as a **warn** to get a population: `-D warnings`
aborts the workspace at the first crate that fails, and the floor it
reports is not the count.

**So the program's signature defect, this time in the tracker rather than
in code: this item was written against the instance and its sibling
already holds the property.** Seven sites here; 278 in the population.
