---
id: decoration-seam-header-names-no-pin-for-enclose
kind: issue
title: decoration_seam.rs's header says the ssi::enclose crossing is pinned by no row; enclose.rs's own decoration_seam module pins it
status: open
opened: 2026-09-04
track: W
refs: [D289]
---


## Finding

`crates/geom-core/tests/decoration_seam.rs`'s header enumerates the four
`crates/*/src` sites that reach `RingInterval::from_certified` and cannot
be called from that suite, naming for each the row elsewhere that pins it.
The fourth bullet — `geom_brep::ssi::enclose`'s, which **no row named here
pins** — is stale in substance: `crates/geom-brep/src/ssi/enclose.rs`'s
own `tests::decoration_seam` module pins that crossing with three rows,
`every_ring_crossing_refuses_exactly_where_the_decoration_degrades`,
`no_crossing_may_be_rebounded_to_the_bracket_door` and
`a_violated_radius_poisons_the_pad`, all three green under
`--features interval` (measured 2026-09-04). The bullet should name them
as the other three bullets name theirs.

The sentence is not wrong about what THESE rows pin, which is why the
lane that added the pin could leave it standing; what it costs is the
roster's whole purpose — a reader checking whether the crossing is
covered reads the header and stops.

## Was

Disclosed by `D289`'s landing (Track Q), which found the pin already in
the tree and could not correct the header: `crates/*/tests/` is Track W's
fence, and the file is `tcost`'s territory by glob.
