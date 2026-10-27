---
id: census-link-fuzz-prism-fixture-escalates-at-1e-6
kind: issue
title: census link_fuzz's prism fixture escalates its own description at eps 1e-6 on some seeds
status: open
opened: 2026-10-27
priority: P3
cost: E
---


`census::link_fuzz::the_link_test_agrees_with_the_prisms_own_inequalities` failed once at `CAD_TOLERANCE_EPS=1e-6`, with `CAD_FUZZ_SEED=0x13dddc011d98976b`. Seen on `atrest/gap-reads-a-cell-against-a-curved-face`; the failure is upstream of anything that branch changes. It is not a census verdict. The fixture's own build panics: `link_fuzz.rs` `build` calls `test_support_fixtures::describe_as_intersections`, and its `unwrap` takes `Certification { Escalated { check: Transversality, predicate: "dihedral_wedge", margin ≈ 8.66e-6 } }`, in band at 1e-6. The random prism has two faces too nearly coplanar for the 1e-6 band to certify the edge between them as a transversal intersection.

The fix: the fuzz draws prisms whose dihedral is clear of the band at every eps row it runs under, or skips a draw whose description escalates. A skip must count and report the skipped draws (`test_utils::loud_skip_marker!`), so it never passes silently. The fuzz's shape is a counterexample search, so a skipped draw costs detection power, never correctness.

Seen again at 1e-6 with seed `0x0936e76f7ad5e957`, this time as a `LeverEscalation` on `dihedral_wedge` (margin ≈ 5.2e-6) in the same fixture (`test_support_fixtures.rs`, `describe_as_intersections`). It fails in about one run in two at that row.
