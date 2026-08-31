//! The palette's own invariants — the rows that hold whatever colours
//! a theme happens to state.
//!
//! Every row here runs with **no `app` feature**: `viewer::theme` names
//! no toolkit, so the palette is asserted on in ordinary headless CI
//! with neither `egui` nor `wgpu` compiled. That is the whole reason
//! the module sits outside `app`.
//!
//! What is NOT here yet: the colourblind-legibility check that holds
//! [`Safety::ColorblindSafe`] to its claim. It arrives with the theme
//! that makes the claim; no theme registered today makes it, so a
//! check would currently assert over an empty set and report a
//! confidence it had not earned.

use viewer::theme::{Mark, Safety, Theme, from_linear, linear};

/// A theme's name is how `--theme` and a preferences file will reach
/// it, so two themes sharing one is a theme nobody can select.
#[test]
fn registered_names_are_unique() {
    let mut names: Vec<&str> = Theme::ALL.iter().map(|theme| theme.name).collect();
    names.sort_unstable();
    let before = names.len();
    names.dedup();
    assert_eq!(names.len(), before, "two registered themes share a name");
}

/// The registry and the lookup are one set: every theme in `ALL` is
/// reachable by its own name, and nothing else is.
#[test]
fn by_name_round_trips_every_registered_theme() {
    for theme in Theme::ALL {
        assert_eq!(
            Theme::by_name(theme.name),
            Some(*theme),
            "{} is registered but not reachable by name",
            theme.name,
        );
    }
    assert_eq!(Theme::by_name("no-such-theme"), None);
    // Not a fallback: an unrecognised `--theme` is a typo, and
    // opening the default in silence would hide it.
    assert_eq!(Theme::by_name(""), None);
}

/// The default has to be one of the themes the registry checks.
#[test]
fn default_is_registered() {
    assert!(
        Theme::ALL.contains(&Theme::DEFAULT),
        "the default theme is not in the registry, so nothing here checks it",
    );
}

/// Strengths and the ambient term are mix fractions. Checked once
/// over the registry rather than at each use: `Mark::strength` is
/// documented as `[0, 1]`, and a value outside it reaches the shader
/// as a `mix` that overshoots — a colour brighter than either input,
/// with nothing to report it.
#[test]
fn mix_fractions_are_in_range() {
    for theme in Theme::ALL {
        assert!(
            (0.0..=1.0).contains(&theme.ambient),
            "{}: ambient {} outside [0, 1]",
            theme.name,
            theme.ambient,
        );
        for (which, mark) in theme.marks() {
            assert!(
                (0.0..=1.0).contains(&mark.strength),
                "{}: {which} strength {} outside [0, 1]",
                theme.name,
                mark.strength,
            );
        }
    }
}

/// sRGB → linear → sRGB is the identity on all 256 codes.
///
/// The palette and the document both state colour in 8-bit sRGB and
/// the shader shades in linear, so this pair is the one crossing
/// every colour makes. A round trip that lost a code would move a
/// theme's colours by a step each time one was composited — and
/// [`Mark::over`] composites in linear and answers in sRGB, so the
/// loss would compound.
#[test]
fn srgb_linear_round_trip_is_exact() {
    for code in 0..=u8::MAX {
        let color = editor_core::appearance::Rgba8::opaque(code, code, code);
        assert_eq!(
            from_linear(linear(color)),
            color,
            "code {code} did not survive the round trip",
        );
    }
}

/// The mix endpoints are the two colours it mixes.
///
/// Strength 0 leaves the body untouched and strength 1 replaces it —
/// which is what the doc comment on [`Mark::strength`] promises, and
/// what makes the composited colour the honest subject of a
/// legibility check rather than the raw tint.
#[test]
fn a_mark_at_its_endpoints_is_body_or_tint() {
    for theme in Theme::ALL {
        for (which, mark) in theme.marks() {
            let none = Mark {
                tint: mark.tint,
                strength: 0.0,
            };
            assert_eq!(
                none.over(theme.body),
                theme.body,
                "{}: {which} at strength 0 moved the body colour",
                theme.name,
            );
            let full = Mark {
                tint: mark.tint,
                strength: 1.0,
            };
            assert_eq!(
                full.over(theme.body),
                mark.tint,
                "{}: {which} at strength 1 did not reach its tint",
                theme.name,
            );
        }
    }
}

/// A mark actually moves the body colour it is mixed over.
///
/// The weakest registered mark is the focus tint at 0.24, and a
/// palette whose marks composited to the body colour would draw a
/// highlight nobody can see — the failure this row exists to catch is
/// a theme edited to a tint too close to its own body.
#[test]
fn every_mark_is_visible_against_its_body() {
    for theme in Theme::ALL {
        for (which, mark) in theme.marks() {
            assert_ne!(
                mark.over(theme.body),
                theme.body,
                "{}: {which} composites to the body colour and marks nothing",
                theme.name,
            );
        }
    }
}

/// Claims are held to something. Today no registered theme claims
/// colourblind safety, and this row says so out loud: it is the
/// tripwire that fails the moment a theme makes the claim without the
/// check that enforces it having landed.
#[test]
fn no_theme_claims_safety_the_suite_cannot_yet_check() {
    for theme in Theme::ALL {
        assert_eq!(
            theme.safety,
            Safety::Unchecked,
            "{} claims {:?}, but the check that holds a theme to it is not in this suite yet",
            theme.name,
            theme.safety,
        );
    }
}
