(4c implementer) **`mesh` whole, slow set included, on 48b6eed279:**
- ε 1e-12: 546/546.
- Default ε: 545/546. ε 1e-6: 545/546.
- The one red at both was `the_eps_inventory_is_pinned`: `cert.rs`'s ε carriers went 2 → 4, because `cert::FaceVerdict::read` takes the band and hands it to `cert::verdict`. That is a carrier moved into the one verdict, not a new read (the read columns are unchanged).
- Re-pinned, with its comment, in **e760867739** (pushed, one line plus the comment). The row passes at all three ε.
- The 1e-12 run had already rebuilt with the re-pin, hence its 546.

So `mesh` whole is green at the three ε on e760867739. The 1e-12 run covered the full set there, and default and 1e-6 differ from 48b6eed279 only by that row.

Next: the mutants, the binding census against a wheel, the viewer's slow rows, sweep's meshing modules, and the `mesh` digest against 8ad2d3fd0f.
