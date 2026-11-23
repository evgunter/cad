//! `demo_tour::chaintol`'s tests.

mod tests {
    use demo_tour::chain::{CERTIFIED_PIN_BOX, LINKS, PIN_RADIUS, study};
    use demo_tour::chaintol::*;
    use pncad::geom_core::Tol;

    /// **Every link count certifies the whole study on both lanes**, at
    /// the ambient ε (`ci.yml` runs this row at the default): the module
    /// header's table, executed outside the narration so a run at
    /// another ε row asks it too.
    #[test]
    fn both_lanes_certify_the_whole_study_at_every_link_count() {
        let tol = Tol::witness();
        for links in 1..=LINKS {
            let built = study(links, tol);
            for row in [
                interval_leaf(links, &built.doc),
                sym_leaf(links, &built.doc),
            ] {
                assert!(
                    row.certifies,
                    "{links} link(s) at ε = {}: the whole study refuses: {:?}",
                    tol.eps(),
                    row.first
                );
            }
        }
    }

    /// **The drive over the whole study splits as the header's table
    /// says, at two to four links** ([`DRIVE_AT_THE_STUDY`], asserted at
    /// the default ε inside [`drive_and_report`]). Minutes per link
    /// count, so it runs in the nightly; the cell runs the one-link
    /// drive.
    #[test]
    fn the_drive_over_the_whole_study_splits_as_pinned() {
        let tol = Tol::witness();
        for links in 2..=LINKS {
            drive_and_report(links, tol);
        }
    }

    /// **The tip's certified enclosure over the whole study is the
    /// worst-case lever sum, and past the 1 mm band**: across the chain
    /// the pins run `1 : 3 : 6 : 10` to first order, and the tip's half-width is wider
    /// than the asserted band, which is why the drive finds VIOLATED
    /// leaves from two links on (the module header's drive table).
    #[test]
    fn the_whole_studys_pin_enclosures_are_the_lever_sum() {
        let tol = Tol::witness();
        let boxes = certified_pin_boxes(LINKS, tol);
        let unit = boxes[1].1;
        for (k, (_, across)) in boxes.iter().enumerate().skip(1) {
            let lever = (k * (k + 1) / 2) as f64;
            assert!(
                // To first order: each joint's swing is a sine of its
                // angle, so the outer pins fall short of the linear sum
                // by a part in a thousand at `3σ`.
                (across / unit - lever).abs() <= 1.0e-2 * lever,
                "pin {}'s certified half-width across the chain is {across:e} m, {} of pin \
                 2's; the worst-case lever sum is {lever}",
                k + 1,
                across / unit
            );
        }
        let tip = boxes[LINKS].1;
        assert!(
            tip > demo_tour::chain::POSITION_BOUND,
            "the tip's certified half-width {tip:e} m is past the 1 mm band"
        );
        assert!(tip > PIN_RADIUS, "and wider than the pin itself");
        assert_eq!(boxes.len(), CERTIFIED_PIN_BOX.len());
    }
}
