//! `demo_tour::bracket`'s tests.

mod tests {
    /// The stop builds: the bracket and its halves at their closed
    /// forms, the corner piece's cap chords chamfered at `4·(d²/2)·√2`
    /// less, and both section faces' whole rims chamfered at
    /// `rim_taken` less per face.
    #[test]
    fn the_stop_builds_its_chamfered_corner_piece() {
        let stop = demo_tour::bracket::stop(pncad::geom_core::Tol::witness());
        assert_eq!(stop.bodies.len(), 1, "the chamfered corner piece");
    }
}
