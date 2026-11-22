//! The controls guide and the input mapper read one table.
//!
//! [`InputMap::bindings`] is both what the mapper honours and what the
//! guide lists. These rows hold the two readings together from
//! outside, over every registered preset: each row the guide shows is
//! a gesture the mapper answers with that row's action, and each
//! gesture the mapper answers is a row the guide shows.

use viewer::camera::{Camera, CameraOp};
use viewer::input::{
    Binding, Held, InputMap, Navigation, PRESETS, PickAction, PointerButton, ViewportEvent,
    ViewportSize,
};

use crate::common;

fn viewport() -> ViewportSize {
    ViewportSize {
        width_px: 1024.0,
        height_px: 768.0,
    }
}

fn camera() -> Camera {
    common::framed(viewport().aspect().expect("a positive aspect"))
}

const AT: [f64; 2] = [10.0, 20.0];

/// What the mapper made of one event, in the guide's terms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Did {
    Orbit,
    Pan,
    Zoom,
    Select,
    Highlight,
}

/// Both mappings' answers to `event`. Leaving the viewport is not
/// asked here: it is no gesture of its own in the guide.
fn did(map: &InputMap, event: &ViewportEvent) -> Vec<Did> {
    let moved = map
        .map(event, viewport(), &camera())
        .map(|op| match op {
            CameraOp::Orbit { .. } => Did::Orbit,
            CameraOp::Pan { .. } => Did::Pan,
            CameraOp::Dolly { .. } => Did::Zoom,
            CameraOp::Frame { .. } => panic!("{event:?} framed the model"),
        });
    let picked = map.pick(event).map(|action| match action {
        PickAction::Select(_) => Did::Select,
        PickAction::Hover(_) => Did::Highlight,
        PickAction::ClearHover => panic!("{event:?} cleared the hover"),
    });
    moved.into_iter().chain(picked).collect()
}

/// The event that is exactly `binding`'s gesture, and what its row
/// says it does.
fn performed(binding: &Binding) -> (ViewportEvent, Did) {
    match *binding {
        Binding::Drag {
            button,
            held,
            moves,
        } => (
            ViewportEvent::Drag {
                button,
                shift: held.shift,
                alt: held.alt,
                delta_px: [7.0, -3.0],
            },
            match moves {
                Navigation::Orbit => Did::Orbit,
                Navigation::Pan => Did::Pan,
            },
        ),
        Binding::Scroll => (ViewportEvent::Scroll { units: 1.0 }, Did::Zoom),
        Binding::Pinch => (ViewportEvent::Pinch { factor: 1.25 }, Did::Zoom),
        Binding::Click { button } => (ViewportEvent::Click { button, pos_px: AT }, Did::Select),
        Binding::Hover => (ViewportEvent::Hover { pos_px: AT }, Did::Highlight),
    }
}

/// Whether `binding` is a row that names `event`'s gesture.
fn names(binding: &Binding, event: &ViewportEvent) -> bool {
    match (*binding, *event) {
        (
            Binding::Drag { button, held, .. },
            ViewportEvent::Drag {
                button: dragged,
                shift,
                alt,
                ..
            },
        ) => button == dragged && (!held.shift || shift) && (!held.alt || alt),
        (Binding::Scroll, ViewportEvent::Scroll { .. })
        | (Binding::Pinch, ViewportEvent::Pinch { .. })
        | (Binding::Hover, ViewportEvent::Hover { .. }) => true,
        (Binding::Click { button }, ViewportEvent::Click { button: clicked, .. }) => {
            button == clicked
        }
        _ => false,
    }
}

/// Every event of the vocabulary that could denote something: each
/// button dragged under each modifier state and clicked, the wheel, a
/// pinch, a hover.
fn every_gesture() -> Vec<ViewportEvent> {
    let mut events = vec![
        ViewportEvent::Scroll { units: 1.0 },
        ViewportEvent::Scroll { units: -2.0 },
        ViewportEvent::Pinch { factor: 1.25 },
        ViewportEvent::Hover { pos_px: AT },
    ];
    for button in PointerButton::ALL {
        events.push(ViewportEvent::Click { button, pos_px: AT });
        for shift in [false, true] {
            for alt in [false, true] {
                events.push(ViewportEvent::Drag {
                    button,
                    shift,
                    alt,
                    delta_px: [7.0, -3.0],
                });
            }
        }
    }
    events
}

/// The registered presets, and one table no preset holds: the
/// default's rows reversed with the default's modifiers swapped, so a
/// mapper arm that read a fixed button or modifier instead of the
/// table disagrees with it.
fn tables() -> Vec<(String, InputMap)> {
    let swapped: Vec<Binding> = InputMap::DEFAULT
        .bindings
        .iter()
        .rev()
        .map(|binding| match *binding {
            Binding::Drag {
                button,
                held,
                moves,
            } => Binding::Drag {
                button,
                held: Held {
                    shift: held.alt,
                    alt: held.shift,
                },
                moves,
            },
            Binding::Click { .. } => Binding::Click {
                button: PointerButton::Middle,
            },
            other => other,
        })
        .collect();
    PRESETS
        .iter()
        .map(|(name, map)| ((*name).to_owned(), *map))
        .chain([(
            "the swapped table".to_owned(),
            InputMap {
                bindings: Vec::leak(swapped),
                ..InputMap::DEFAULT
            },
        )])
        .collect()
}

/// **Every row the guide shows is honoured**: performing exactly the
/// row's gesture does exactly the row's action. A row shadowed by
/// another, or one the mapper has no arm for, is red here.
#[test]
fn every_guide_row_is_a_gesture_the_mapper_answers_with_that_action() {
    for (name, map) in tables() {
        assert!(!map.bindings.is_empty(), "{name}: a table with no rows");
        for binding in map.bindings {
            let (event, says) = performed(binding);
            assert_eq!(
                did(&map, &event),
                vec![says],
                "{name}: the guide says `{}` does `{}`",
                binding.gesture(),
                binding.action(),
            );
        }
    }
}

/// **Everything the mapper honours is a row the guide shows**: over
/// the whole event vocabulary, an event that denotes something is
/// named by a row whose action is what it did. A binding hard-coded in
/// the mapper beside the table is red here.
#[test]
fn every_gesture_the_mapper_answers_is_a_guide_row() {
    for (name, map) in tables() {
        for event in every_gesture() {
            for done in did(&map, &event) {
                assert!(
                    map.bindings
                        .iter()
                        .any(|binding| names(binding, &event) && performed(binding).1 == done),
                    "{name}: {event:?} did {done:?} and no guide row says so"
                );
            }
        }
    }
}

/// **A map with no rows honours nothing** — leaving the viewport
/// included, which rides on the hover row.
#[test]
fn an_empty_table_honours_no_gesture() {
    let map = InputMap {
        bindings: &[],
        ..InputMap::DEFAULT
    };
    for event in every_gesture() {
        assert_eq!(did(&map, &event), vec![], "{event:?} with no bindings");
    }
    assert_eq!(map.pick(&ViewportEvent::Leave), None, "leave, no hover row");
    assert_eq!(
        InputMap::DEFAULT.pick(&ViewportEvent::Leave),
        Some(PickAction::ClearHover),
        "leave, under the hover row"
    );
}

/// **No two rows of a guide read alike**: a guide with two rows for
/// one gesture cannot say which of them holds.
#[test]
fn a_guide_names_each_gesture_once() {
    for (name, map) in tables() {
        let gestures: Vec<String> = map.bindings.iter().map(Binding::gesture).collect();
        for (at, gesture) in gestures.iter().enumerate() {
            assert!(
                !gestures[..at].contains(gesture),
                "{name}: `{gesture}` is listed twice in {gestures:?}"
            );
        }
    }
}

/// The README's bindings table, as the guide writes it.
fn readme_table(map: &InputMap) -> String {
    let rows: String = map
        .bindings
        .iter()
        .map(|binding| format!("| {} | {} |\n", binding.gesture(), binding.action()))
        .collect();
    format!("| Gesture | Action |\n| --- | --- |\n{rows}")
}

/// **`README.md`'s "Mouse bindings" table is the default guide, row
/// for row.** The page is read before the viewer is run, so it keeps a
/// table; this is what stops that table being a second copy.
#[test]
fn the_readme_bindings_table_is_the_default_guide() {
    let readme = include_str!("../README.md");
    let section = readme
        .split_once("\n## Mouse bindings\n")
        .expect("the README has a `## Mouse bindings` section")
        .1;
    let table: String = section
        .lines()
        .skip_while(|line| !line.starts_with('|'))
        .take_while(|line| line.starts_with('|'))
        .map(|line| format!("{line}\n"))
        .collect();
    assert_eq!(
        table,
        readme_table(&InputMap::DEFAULT),
        "crates/viewer/README.md's bindings table against `InputMap::DEFAULT.bindings`"
    );
}
