---
id: certification-value-hygiene-has-no-gate
kind: issue
title: nothing mechanical keeps certification values to certification spellings once Real is in scope, and the one type still speaks three dialects of refusal, hull and ring
status: open
priority: P3
cost: D
opened: 2026-09-24
refs: [ring-3-ring-dissolves-into-interval, ring-3-residue-outside-its-fence, H5]
---

## Finding

RING-3 made the certification substrate the evaluation scalar
`Interval` (C9, `crates/geom-brep/README.md` C9). What the retired type
enforced by being a separate non-`Real` type is now convention. Both
reviews of PR 3153 found the same class from several sides; the
orchestrator ruled it filed, not fixed in that unit. Each item below
cites a name, with the line at `scalar/ring-3`'s fix-pass head.

1. **The refusal and the transcendental-free rule are conventions.**
   At a certification site with `Real` in scope, `x.is_poison()`
   compiles and resolves to `Real::is_poison` (NaI or empty), which
   answers `false` on a `Trv` bracket with real endpoints — the
   certifying branch on a value that does not certify. R2's compile
   probe: `topo/src/props.rs` `quad_lane`'s exact imports make
   `x.is_poison()` on an `Interval` E0599 (the compiler is that pin,
   `props.rs:2759`); adding `Real` to them compiles, and returns `false`
   on `[1,2]/[0,1]`. Three certification files import `Real` today
   (`topo/src/props.rs:43`, `geom/src/curves/nurbs.rs:161`,
   `geom-brep/src/ssi/certify.rs:113`),
   and `topo/src/props.rs` `bracket_seam_tests` (`:3360`) does too.
   The same holds for C9's transcendental-free rule: certification code
   with `Real` in scope can call `sqrt`/`asin` on a certification
   `Interval` and mint `Trv` or empty; the old type had no such method
   (C9 now says the rule is about what certification does, not what the
   type can). `Interval::point`/`zero`/`one`/`powi`/`sqr` are restated
   inherently (`interval.rs` near `:279`) "so a certification site
   needs no `Real` in scope" — a comment doing a type system's job.
   Source: R1 N6/S3/S1, R2 NOTE-3/Q1.
2. **Three hull semantics on one type.** `Interval::hull`
   (`interval.rs:370`: anything not certified becomes NaI),
   `SpanLocate::enclosure_hull` (`:838`) and the free `tangent_hull`
   (`:938`) (NaI stays NaI, empty stays empty, `Trv` flows through at
   the minimum decoration), and the backend's `DInterval::hull`, which
   absorbs the empty set. A caller holding an `Interval` gets a
   different refusal depending on which name it reaches. Source: R2 Q1.
3. **"Poison" means three things.** `Interval::poison()` (`:290`) is
   NaI; `Real::is_poison` (`:621`) is NaI or empty; the certification
   refusal is `!is_certified()` (`:247`). Rows and prose still say
   "must poison" for a certification refusal (`interval.rs`
   `certification_door_tests`, e.g. `:2070` "divisor touching zero must
   poison"; `spline/hull.rs`'s module doc). Source: R1 S2.
4. **Public names still spell the retired type.** `CurveRingData`
   (`spline/compose.rs:126`), `SurfaceRingData`
   (`spline/compose/tensor.rs:142`), `ring_coords`
   (`geom/src/curves/nurbs.rs:1638`, `:1647`;
   `geom/src/surfaces/nurbs.rs:1373`), `apply_ring`
   (`spline/algebra.rs:261`). RING-3 kept "ring" as the algebra
   adjective ("ring quotient"); these name the retired type's role.
   Renaming them is a public-API change across `geom`, `geom-brep`,
   `mesh`, `step-import` and `topo`. Source: R1 S5, R2 Q7.
5. **The backend differential cannot red on a door change.**
   `crates/geom-core/tests/interval_backend_differential.rs` compares
   `Interval`'s forwarded operators (`Self(self.0 op rhs.0)`) against
   the same `DInterval` operators, so its agreements hold by
   construction and it exercises none of the certification doors — the
   only surface RING-3 moved. It is a forwarding regression guard, not
   evidence about the doors; nothing differential stands over them.
   Source: R1 N3/S8.
6. **The census's CI gate names fewer crates than it walks.**
   `crates/geom-core/tests/certified_endpoint_census.rs`'s `gated_to!`
   names `geom-core`, `geom-brep`, `geom`, `mesh`, `topo` and one
   `step-import` file, while `population` walks every `crates/*/src`:
   a door call added in `sweep`, `editor-core` or `profile` reds only at
   the nightly. Source: R1 S9.
7. **The door list is written three times by hand.** `interval.rs`'s
   module doc (near `:40-56`), `is_certified`'s doc (near `:230-247`)
   and the doors block's heading doc (near `:252`) each enumerate the
   doors, and nothing keeps them in step. Source: R1 S7, R2 Q2.

## Fix

Items 1-3 want one design answer, and it is the substantive one: a
certification value that cannot reach `Real`'s refusal or
transcendentals — a lint over the census population refusing
`.is_poison()` and the transcendental methods on an `Interval` receiver
(text-level, with the census's blind spots), or a certification view
type again (the thing RING-3 dissolved, so that is Ev's call, via H5).
Item 4 is a rename. Item 5 wants a door-level differential or its
honest re-description. Items 6-7 are mechanical: widen `gated_to!` to
every crate the walk reads, and state the door list once and link it.
