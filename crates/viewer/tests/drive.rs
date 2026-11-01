//! **The scripted driver reaches the session through the chrome**:
//! a control found by its label and clicked where it was painted
//! performs what clicking it at the window performs, and a label that
//! is not on screen is refused rather than clicked at a guess.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

test_utils::loud_skip_marker!(
    feature = "app",
    row = app_lane_skipped_no_driver_coverage_here,
    absent = "coverage of the scripted driver",
);

#[cfg(feature = "app")]
mod driven {
    use crate::common;
    use viewer::app::drive::{DriveError, Driver};

    /// The startup document, driven, over a preferences file of its own.
    fn started(label: &str) -> Driver {
        Driver::start(
            common::tempdir(label).join("prefs.toml"),
            pncad::tolerance::witness(),
        )
        .expect("the app starts")
    }

    #[test]
    fn a_profile_added_through_the_chrome_lands_in_the_tree() {
        let mut driver = started("drive-profile");
        let before = driver.session().tree_rows().len();
        assert!(
            !driver.shows("Add profile"),
            "the creation forms start folded"
        );
        driver.click("Add feature", 0).expect("the section opens");
        assert!(
            !driver.shows("Add profile"),
            "the open section offers the form's door, not the form"
        );
        driver.click("Add profile…", 0).expect("the form opens");
        driver.click("rectangle", 0).expect("the shape is chosen");
        driver
            .click("Add profile", 0)
            .expect("the profile is added");
        let rows = driver.session().tree_rows();
        assert_eq!(rows.len(), before + 1, "one node per Add");
        let added = rows.last().expect("a row").spoken.to_string();
        assert!(
            added.contains("Profile 2"),
            "the new row is the profile the form named: {added}"
        );
        assert!(
            driver.dump().contains("Profile 2"),
            "and the feature tree on screen shows it"
        );
    }

    /// **The picture is the window's**: as many pixels as the window
    /// has, and inside the viewport the startup part is drawn — the
    /// pane's pixels are not all the ground it is painted on.
    #[test]
    fn a_picture_shows_the_window_with_the_part_drawn() {
        let driver = started("drive-picture");
        let (device, queue) = pollster::block_on(viewer::app::drive::picture_device())
            .expect("a device to render on");
        let picture = driver.picture(&device, &queue).expect("the window renders");
        let window = driver.window();
        assert_eq!(
            picture.size,
            [window.x as u32, window.y as u32],
            "one pixel per point at the driver's scale"
        );
        let viewport = driver.viewport().expect("the viewport was drawn");
        let pixel = |x: f32, y: f32| {
            let at = (y as usize * picture.size[0] as usize + x as usize) * 4;
            &picture.rgba[at..at + 4]
        };
        // The corner is ground: the startup view frames the part with
        // a margin around it.
        let ground = pixel(viewport.left() + 2.0, viewport.top() + 2.0);
        let (mut drawn, mut all) = (0_usize, 0_usize);
        for y in (viewport.top() as u32..viewport.bottom() as u32).step_by(4) {
            for x in (viewport.left() as u32..viewport.right() as u32).step_by(4) {
                all += 1;
                if pixel(x as f32, y as f32) != ground {
                    drawn += 1;
                }
            }
        }
        assert!(
            drawn * 10 > all,
            "the startup part covers more than a tenth of the viewport: {drawn} of {all} \
             sampled pixels are not the ground"
        );
    }

    #[test]
    fn a_label_not_on_screen_is_refused_with_what_was_meant() {
        let mut driver = started("drive-refusal");
        match driver.click("add feature", 0) {
            Err(DriveError::NotOnScreen { shown, near, .. }) => {
                assert_eq!(shown, 0, "labels match exactly, case included");
                assert!(
                    near.iter().any(|label| label == "Add feature"),
                    "the refusal names the label it was near: {near:?}"
                );
            }
            other => panic!("a mis-cased label was not refused: {other:?}"),
        }
        assert!(
            matches!(
                driver.find("Add feature", 1),
                Err(DriveError::NotOnScreen { shown: 1, .. })
            ),
            "a second painting of a label painted once is not on screen"
        );
    }
}
