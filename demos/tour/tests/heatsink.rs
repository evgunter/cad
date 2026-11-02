//! `demo_tour::heatsink`'s tests.

mod name_column {
    //! The budget sweep's `name` column, pinned where a real document
    //! mints the names: this scene is one of the six the tour can name
    //! at all, and the token it writes has to survive a CSV and mean
    //! one face.
    //!
    //! **What these rows can and cannot say.** `demo_tour::face_names` is a
    //! crate-root item with six callers — `heatsink5`, `heatsink7`,
    //! `heatsink9` here and `diefillet`, `diepips`, `diecomposed` in
    //! `diefillet.rs` — and it is exercised from exactly one of them,
    //! this one. What that misses is a scene whose faces the node's
    //! table does not name, which would panic at the door rather than
    //! render badly; the door is the same door for all six and the
    //! panic names the node and the face, so the failure is loud
    //! wherever it happens, but it is not under test at five of them.
    //!
    //! **Non-emptiness and comma-freedom are NOT asserted here**, and
    //! deliberately: `demo_tour::face_names` puts every token through
    //! `tess_meter::FaceName::new` and `.expect()`s exactly those two
    //! properties, so no input can reach an assertion in this module in
    //! a failing state — the door panics first. A row asserting them
    //! after the door would be a comment wearing an `assert!`. The
    //! refusal itself is `tess-meter`'s, tested there; what is left for
    //! this scene is what the door does not check, which is that the
    //! rendering means ONE face and is reversible.

    use demo_tour::heatsink::scene_doc;
    use pncad::document::{CancelToken, EvalOptions, evaluate};
    use pncad::geom_core::Tol;
    use pncad::prelude::StableName;

    /// The scene's own body and the names of its faces, as
    /// `demo_tour::face_names` renders them for the CSV.
    fn tokens() -> Vec<(String, StableName)> {
        let tol = Tol::witness();
        let r = scene_doc(tol);
        let ev = evaluate::<f64>(
            &r.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        let body = demo_tour::heatsink::solidify(&r, &ev, 5, tol);
        let rendered = demo_tour::face_names(&ev, r.solid, &body);
        let rows = body
            .faces()
            .map(|(key, _)| {
                let token = rendered
                    .get(&key)
                    .expect("face_names covers every face of the body")
                    .as_str()
                    .to_string();
                let name = pncad::select::face_name(&ev, r.solid, 0, key)
                    .expect("the node names its own face")
                    .clone();
                (token, name)
            })
            .collect::<Vec<_>>();
        assert!(
            rows.len() > 1,
            "the fixture has faces to name: {} rows",
            rows.len()
        );
        rows
    }

    /// **The token means ONE face.** A rendering that collapsed two
    /// derivation paths would hand `tools/tess-lint` a join key no
    /// better than the ordinal it already has.
    #[test]
    fn distinct_faces_render_distinct_tokens() {
        let rows = tokens();
        let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
        for (token, name) in &rows {
            assert!(seen.insert(token), "{name} shares its token: {token:?}");
        }
        assert_eq!(seen.len(), rows.len());
    }

    /// **The `,` → `;` swap is reversible**, which is what makes the
    /// flattening injective rather than merely comma-free: a
    /// `StableName` contains no string payload, so no `;` of its own
    /// can be confused with one the swap wrote. Asserted by putting
    /// the commas back and reading the name out again.
    #[test]
    fn the_swap_round_trips_to_the_same_name() {
        for (token, name) in tokens() {
            let json = token.replace(';', ",");
            let back: StableName =
                serde_json::from_str(&json).expect("the swapped token is the name's own JSON");
            assert_eq!(back, name, "round trip through {token:?}");
        }
    }
}

mod scene {
    //! The PR gate runs the tour's tests, not the tour: this is where the
    //! scene's own assertions — the recompute and reuse counts, the
    //! surviving names, the volume at every count, and the flush fins'
    //! build — run on every PR that touches the tour.
    #[test]
    fn the_scene_holds() {
        demo_tour::heatsink::stops(pncad::geom_core::Tol::witness());
    }
}
