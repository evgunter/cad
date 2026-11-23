//! The controls guide and the input mapper read one table.
//!
//! [`InputMap::bindings`] is both what the mapper honours and what the
//! guide lists. These rows hold the two readings together from
//! outside, over every registered preset: each row the guide shows is
//! a gesture the mapper answers with that row's action, and each
//! gesture the mapper answers is a row the guide shows.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use viewer::camera::{Camera, CameraOp};
use viewer::input::{
    Action, Binding, Held, InputMap, Navigation, PRESETS, PickAction, PointerButton, ViewportEvent,
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

/// Both mappings' answers to `event`, in the guide's terms. Leaving
/// the viewport is not asked here: it is no gesture of its own in the
/// guide.
fn did(map: &InputMap, event: &ViewportEvent) -> Vec<Action> {
    let moved = map.map(event, viewport(), &camera()).map(|op| match op {
        CameraOp::Orbit { .. } => Action::Orbit,
        CameraOp::Pan { .. } => Action::Pan,
        CameraOp::Dolly { .. } => Action::Zoom,
        CameraOp::Frame { .. } => panic!("{event:?} framed the model"),
    });
    let picked = map.pick(event).map(|action| match action {
        PickAction::Select(_) => Action::Select,
        PickAction::Hover(_) => Action::Highlight,
        PickAction::ClearHover => panic!("{event:?} cleared the hover"),
    });
    moved.into_iter().chain(picked).collect()
}

fn drag(button: PointerButton, shift: bool, alt: bool) -> ViewportEvent {
    ViewportEvent::Drag {
        button,
        shift,
        alt,
        delta_px: [7.0, -3.0],
    }
}

/// The event that is exactly `binding`'s gesture.
fn performed(binding: &Binding) -> ViewportEvent {
    match *binding {
        Binding::Drag { button, held, .. } => drag(button, held.shift, held.alt),
        Binding::Scroll => ViewportEvent::Scroll { units: 1.0 },
        Binding::Pinch => ViewportEvent::Pinch { factor: 1.25 },
        Binding::Click { button } => ViewportEvent::Click { button, pos_px: AT },
        Binding::Hover => ViewportEvent::Hover { pos_px: AT },
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
        ) => button == dragged && held.met_by(shift, alt),
        (Binding::Scroll, ViewportEvent::Scroll { .. })
        | (Binding::Pinch, ViewportEvent::Pinch { .. })
        | (Binding::Hover, ViewportEvent::Hover { .. }) => true,
        (
            Binding::Click { button },
            ViewportEvent::Click {
                button: clicked, ..
            },
        ) => button == clicked,
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
                events.push(drag(button, shift, alt));
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
            assert_eq!(
                did(&map, &performed(binding)),
                vec![binding.does()],
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
                        .any(|binding| names(binding, &event) && binding.does() == done),
                    "{name}: {event:?} did {done:?} and no guide row says so"
                );
            }
        }
    }
}

/// How many keys `held` names.
fn keys(held: Held) -> usize {
    usize::from(held.shift) + usize::from(held.alt)
}

/// **A key a row does not name is not asked about**: a row's drag
/// with any further keys down still does the row's action, unless
/// that state is another row's — one on the same button naming more
/// keys, or as many and listed earlier.
#[test]
fn a_drag_row_holds_under_every_key_it_does_not_name() {
    let mut held_under_more = 0;
    for (name, map) in tables() {
        for (at, row) in map.bindings.iter().enumerate() {
            let Binding::Drag { button, held, .. } = *row else {
                continue;
            };
            for (more_shift, more_alt) in [(false, true), (true, false), (true, true)] {
                let (shift, alt) = (held.shift || more_shift, held.alt || more_alt);
                let taken = map.bindings.iter().enumerate().any(|(other_at, other)| {
                    matches!(
                        *other,
                        Binding::Drag { button: bound, held: other_held, .. }
                            if other_at != at
                                && bound == button
                                && other_held.met_by(shift, alt)
                                && (keys(other_held) > keys(held)
                                    || (keys(other_held) == keys(held) && other_at < at))
                    )
                });
                if taken || (shift, alt) == (held.shift, held.alt) {
                    continue;
                }
                held_under_more += 1;
                assert_eq!(
                    did(&map, &drag(button, shift, alt)),
                    vec![row.does()],
                    "{name}: `{}` with shift {shift}, alt {alt} down",
                    row.gesture(),
                );
            }
        }
    }
    assert!(
        held_under_more >= 8,
        "the tables hold rows with a key to spare: {held_under_more} states asked"
    );
}

/// **Between two rows a drag fits that name equally many keys, the
/// earlier is the one it means** — whichever of the two that is.
#[test]
fn of_two_rows_naming_equally_many_keys_the_earlier_wins() {
    const ORBIT: Binding = Binding::Drag {
        button: PointerButton::Middle,
        held: Held::SHIFT,
        moves: Navigation::Orbit,
    };
    const PAN: Binding = Binding::Drag {
        button: PointerButton::Middle,
        held: Held::ALT,
        moves: Navigation::Pan,
    };
    let both = drag(PointerButton::Middle, true, true);
    for (bindings, first) in [(&[ORBIT, PAN], Action::Orbit), (&[PAN, ORBIT], Action::Pan)] {
        let map = InputMap {
            bindings,
            ..InputMap::DEFAULT
        };
        assert_eq!(did(&map, &both), vec![first], "{bindings:?}");
    }
}

/// The words of `text`, in lower case.
fn words(text: &str) -> Vec<String> {
    text.split(|letter: char| !letter.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect()
}

/// **A row's wording says what its fields say**: a drag's gesture
/// names its button and no other, and a key exactly when the row
/// requires it; its action is its `moves`; and no two actions, and no
/// two gestures, read alike. Every button under every key state, so
/// the rows no preset holds are worded too.
#[test]
fn a_rows_wording_names_its_button_its_keys_and_its_action() {
    let names_only = |said: &[String], button: PointerButton, row: &Binding| {
        for other in PointerButton::ALL {
            assert_eq!(
                said.iter().any(|word| word == other.name()),
                other == button,
                "{row:?} names the {} button in `{}`",
                other.name(),
                row.gesture(),
            );
        }
    };
    let mut gestures = Vec::new();
    for button in PointerButton::ALL {
        for (shift, alt) in [(false, false), (true, false), (false, true), (true, true)] {
            for (moves, does) in [
                (Navigation::Orbit, Action::Orbit),
                (Navigation::Pan, Action::Pan),
            ] {
                let row = Binding::Drag {
                    button,
                    held: Held { shift, alt },
                    moves,
                };
                let said = words(&row.gesture());
                names_only(&said, button, &row);
                for (key, required) in [("shift", shift), ("alt", alt)] {
                    assert_eq!(
                        said.iter().any(|word| word == key),
                        required,
                        "{row:?} says {key} in `{}`",
                        row.gesture()
                    );
                }
                assert!(said.iter().any(|word| word == "drag"), "{row:?}: {said:?}");
                assert_eq!(row.does(), does, "{row:?}");
                assert_eq!(row.action(), does.label(), "{row:?}");
                if moves == Navigation::Orbit {
                    gestures.push(row.gesture());
                }
            }
        }
        let row = Binding::Click { button };
        let said = words(&row.gesture());
        names_only(&said, button, &row);
        assert!(said.iter().any(|word| word == "click"), "{row:?}: {said:?}");
        gestures.push(row.gesture());
    }
    gestures.extend([Binding::Scroll, Binding::Pinch, Binding::Hover].map(|row| row.gesture()));
    for (at, gesture) in gestures.iter().enumerate() {
        assert!(
            !gestures[..at].contains(gesture),
            "two gestures read `{gesture}`: {gestures:?}"
        );
    }
    let labels = Action::ALL.map(Action::label);
    for (at, label) in labels.iter().enumerate() {
        assert!(
            !label.is_empty() && !labels[..at].contains(label),
            "two actions read `{label}`: {labels:?}"
        );
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
