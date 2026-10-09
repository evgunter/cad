//! **One saved document against another, up to minted ids**: the
//! comparator that says a change moved nothing but ids and the output
//! variables an operation defines (D10).
//!
//! Both are read as JSON bodies. The newer one's output variables are
//! set aside first — their rows in the variable table, their names and
//! their mint-log entries — and the two are then walked in step: every
//! string spelling an id (`3:3fa9c1d2a0b1c3d4`), as a value or as an
//! object key, must read as its image under ONE map, built as the walk
//! meets ids and held injective both ways; everything else is equal.
//! The mint chain is not compared: it is a digest over the ids.
//!
//! Where a refactoring hands over the node map it carried with, two
//! live documents are `fixture::round_trip::same_up_to_ids`'s instead:
//! this walk is for two saves no map joins.
//!
//! A map built by the walk is a bijection of the ids the two documents
//! spell, so a dropped node, an added one, a moved value or two ids
//! swapped in one place and not another all refuse, each naming the
//! path where the two parted.

use std::collections::BTreeMap;

use serde_json::Value;

/// `doc` with every output variable removed: its row, its name and its
/// mint-log entry.
pub fn without_outputs(doc: &Value) -> Value {
    let mut doc = doc.clone();
    let snapshot = &mut doc["snapshot"];
    let outputs: Vec<String> = snapshot["vars"]
        .as_object()
        .expect("a variable table")
        .iter()
        .filter(|(_, var)| var["def"].get("Output").is_some())
        .map(|(id, _)| id.clone())
        .collect();
    let vars = snapshot["vars"].as_object_mut().expect("a variable table");
    for id in &outputs {
        vars.remove(id);
    }
    if let Some(names) = snapshot.get_mut("var_names").and_then(Value::as_object_mut) {
        for id in &outputs {
            names.remove(id);
        }
    }
    snapshot["mint"]["log"]
        .as_array_mut()
        .expect("a mint log")
        .retain(|entry| {
            !entry
                .get("var")
                .and_then(Value::as_str)
                .is_some_and(|id| outputs.iter().any(|o| o == id))
        });
    doc
}

/// `doc` with each operand read in place of the input it reads: every
/// read of an output variable, in a node or an edit, spelled as the
/// operation defining it, and an authored `{"Node": id}` as the bare
/// id — the spelling a document had while an operand named its input
/// node. A read of a port past the first keeps its port, as
/// `{"Port": [id, port]}`, so two documents reading two ports of one
/// operation differ here as they do in what they build. Output
/// variables themselves are left for [`without_outputs`].
pub fn reads_as_inputs(doc: &Value) -> Value {
    reads_as_inputs_by(doc, doc)
}

/// **A pre-B part projection in the port spelling**: a part that
/// selects the lower half of a split named the split and read its
/// second port, so its `of` is spelled as that port
/// ([`reads_as_inputs`]'s `{"Port": [id, 1]}`); the upper half is the
/// first port, the bare id it already was.
pub fn split_halves_as_ports(doc: &Value) -> Value {
    let mut doc = doc.clone();
    if let Some(nodes) = doc["snapshot"]["nodes"].as_object_mut() {
        for node in nodes.values_mut() {
            if let Some(part) = node.get_mut("Part")
                && part["select"]["SplitHalf"] == "Below"
                && let Some(of) = part["of"].as_str().map(str::to_owned)
            {
                part["of"] = port(&of, 1);
            }
        }
    }
    doc
}

/// **A pre-B document in the field's one name**: a profile's and an
/// in-plane axis's `plane` field is `frame` since unit B (the kind the
/// field reads, its slot, its label and its word), so a document the
/// base saved compares with its re-blessed twin byte for byte
/// otherwise — in the snapshot's nodes and in a log's authored ones.
pub fn plane_field_as_frame(doc: &Value) -> Value {
    fn rewrite(value: &mut Value) {
        match value {
            Value::Object(object) => {
                for tag in ["Profile", "AxisInPlane"] {
                    if let Some(Value::Object(fields)) = object.get_mut(tag)
                        && let Some(frame) = fields.remove("plane")
                    {
                        fields.insert("frame".to_owned(), frame);
                    }
                }
                object.values_mut().for_each(rewrite);
            }
            Value::Array(items) => items.iter_mut().for_each(rewrite),
            Value::String(_) | Value::Null | Value::Bool(_) | Value::Number(_) => {}
        }
    }
    let mut doc = doc.clone();
    rewrite(&mut doc);
    doc
}

/// A read of `port` of `node`, as [`reads_as_inputs`] spells it.
fn port(node: &str, port: u64) -> Value {
    if port == 0 {
        Value::String(node.to_owned())
    } else {
        serde_json::json!({ "Port": [node, port] })
    }
}

/// **A pre-B edit log in the one slot door's spelling** (Q1): every
/// `SetParam`'s `expr: F` as `value: {"Formula": F}`, the field the
/// door writes now, so a log the base saved compares with its
/// re-blessed twin byte for byte otherwise.
pub fn set_param_writes_value(doc: &Value) -> Value {
    fn rewrite(value: &mut Value) {
        match value {
            Value::Object(object) => {
                if let Some(Value::Object(edit)) = object.get_mut("SetParam")
                    && let Some(expr) = edit.remove("expr")
                {
                    edit.insert("value".to_owned(), serde_json::json!({ "Formula": expr }));
                }
                object.values_mut().for_each(rewrite);
            }
            Value::Array(items) => items.iter_mut().for_each(rewrite),
            Value::String(_) | Value::Null | Value::Bool(_) | Value::Number(_) => {}
        }
    }
    let mut doc = doc.clone();
    rewrite(&mut doc);
    doc
}

/// [`reads_as_inputs`], each output variable's operation read off
/// `table`'s variable table — the replayed document, for a saved edit
/// log whose own snapshot holds none.
pub fn reads_as_inputs_by(doc: &Value, table: &Value) -> Value {
    let mut doc = doc.clone();
    let defined_by: BTreeMap<String, Value> = table["snapshot"]["vars"]
        .as_object()
        .expect("a variable table")
        .iter()
        .filter_map(|(id, var)| {
            let output = var["def"].get("Output")?;
            let node = output.get("node")?.as_str()?;
            Some((id.clone(), port(node, output.get("port")?.as_u64()?)))
        })
        .collect();
    fn rewrite(value: &mut Value, defined_by: &BTreeMap<String, Value>) {
        match value {
            Value::String(text) => {
                if let Some(read) = defined_by.get(text.as_str()) {
                    *value = read.clone();
                }
            }
            Value::Object(object) => {
                let read = match object.iter().next() {
                    Some((tag, Value::String(id)))
                        if object.len() == 1 && (tag == "Node" || tag == "Var") && is_id(id) =>
                    {
                        Some(
                            defined_by
                                .get(id)
                                .cloned()
                                .unwrap_or_else(|| Value::String(id.clone())),
                        )
                    }
                    Some((tag, Value::Object(at))) if object.len() == 1 && tag == "Output" => {
                        match (
                            at.get("node").and_then(Value::as_str),
                            at.get("port").and_then(Value::as_u64),
                        ) {
                            (Some(node), Some(p)) if is_id(node) => Some(port(node, p)),
                            _ => None,
                        }
                    }
                    _ => None,
                };
                match read {
                    Some(read) => *value = read,
                    None => object.values_mut().for_each(|v| rewrite(v, defined_by)),
                }
            }
            Value::Array(items) => items.iter_mut().for_each(|v| rewrite(v, defined_by)),
            Value::Null | Value::Bool(_) | Value::Number(_) => {}
        }
    }
    rewrite(&mut doc["snapshot"]["nodes"], &defined_by);
    if let Some(edits) = doc.get_mut("edits") {
        rewrite(edits, &defined_by);
    }
    doc
}

/// `doc` with the nodes `ids` names taken out whole: their rows, the
/// root list's entries for them, their outputs, every other variable
/// no remaining node, definition or edit spells, and those ids'
/// mint-log entries. For a
/// subgraph whose shape a change moved by design, set aside on both
/// sides so the rest compares.
pub fn without_nodes(doc: &Value, ids: &[String]) -> Value {
    let mut doc = doc.clone();
    let snapshot = &mut doc["snapshot"];
    let nodes = snapshot["nodes"].as_object_mut().expect("a node map");
    for id in ids {
        nodes.remove(id);
    }
    if let Some(roots) = snapshot.get_mut("roots").and_then(Value::as_array_mut) {
        roots.retain(|root| !root.as_str().is_some_and(|r| ids.iter().any(|id| id == r)));
    }
    // A variable stays while anything left spells it.
    let mut spelled = std::collections::BTreeSet::new();
    fn ids_in(value: &Value, out: &mut std::collections::BTreeSet<String>) {
        match value {
            Value::String(text) if is_id(text) => {
                out.insert(text.clone());
            }
            Value::Object(object) => object.values().for_each(|v| ids_in(v, out)),
            Value::Array(items) => items.iter().for_each(|v| ids_in(v, out)),
            _ => {}
        }
    }
    ids_in(&snapshot["nodes"], &mut spelled);
    ids_in(&doc["edits"], &mut spelled);
    let snapshot = &mut doc["snapshot"];
    let mut gone: Vec<String> = ids.to_vec();
    loop {
        let vars = snapshot["vars"].as_object().expect("a variable table");
        let mut defs = std::collections::BTreeSet::new();
        for var in vars.values() {
            ids_in(&var["def"], &mut defs);
        }
        let unspelled: Vec<String> = vars
            .iter()
            .filter(|(id, var)| match var["def"].get("Output") {
                // An output goes with its operation.
                Some(output) => output["node"]
                    .as_str()
                    .is_some_and(|node| ids.iter().any(|id| id == node)),
                None => !spelled.contains(*id) && !defs.contains(*id),
            })
            .map(|(id, _)| id.clone())
            .filter(|id| {
                snapshot
                    .get("var_names")
                    .and_then(Value::as_object)
                    .is_none_or(|names| !names.contains_key(id))
            })
            .collect();
        if unspelled.is_empty() {
            break;
        }
        let vars = snapshot["vars"].as_object_mut().expect("a variable table");
        for id in &unspelled {
            vars.remove(id);
        }
        gone.extend(unspelled);
    }
    snapshot["mint"]["log"]
        .as_array_mut()
        .expect("a mint log")
        .retain(|entry| {
            !entry
                .as_object()
                .and_then(|e| e.values().next())
                .and_then(Value::as_str)
                .is_some_and(|id| gone.iter().any(|g| g == id))
        });
    doc
}

/// The tube subgraph of `doc`: every `Tube` and `HollowTube` node and
/// the datum each reads its anchor from — a spine node id, or a frame
/// output read through its defining operation.
pub fn tube_subgraph(doc: &Value) -> Vec<String> {
    let snapshot = &doc["snapshot"];
    let defined_by = |id: &str| -> String {
        snapshot["vars"]
            .get(id)
            .and_then(|var| var["def"].get("Output"))
            .and_then(|out| out.get("node"))
            .and_then(Value::as_str)
            .unwrap_or(id)
            .to_owned()
    };
    let mut out = Vec::new();
    for (id, node) in snapshot["nodes"].as_object().expect("a node map") {
        for tag in ["Tube", "HollowTube"] {
            if let Some(tube) = node.get(tag) {
                out.push(id.clone());
                for anchor in ["spine", "frame"] {
                    if let Some(read) = tube.get(anchor).and_then(Value::as_str) {
                        let datum = defined_by(read);
                        if !out.contains(&datum) {
                            out.push(datum);
                        }
                    }
                }
            }
        }
    }
    out
}

/// Whether `text` spells an id: an ordinal in decimal, a colon and
/// sixteen lowercase hex digits.
fn is_id(text: &str) -> bool {
    editor_core::MintId::parse(text).is_some()
}

#[derive(Default)]
struct Bijection {
    forward: BTreeMap<String, String>,
    back: BTreeMap<String, String>,
}

impl Bijection {
    fn pair(&mut self, old: &str, new: &str, at: &str) -> Result<(), String> {
        match (self.forward.get(old), self.back.get(new)) {
            (None, None) => {
                self.forward.insert(old.to_owned(), new.to_owned());
                self.back.insert(new.to_owned(), old.to_owned());
                Ok(())
            }
            (Some(image), _) if image == new => Ok(()),
            (Some(image), _) => Err(format!(
                "{at}: {old} reads as {new}, and earlier as {image}"
            )),
            (None, Some(preimage)) => Err(format!(
                "{at}: {new} is the image of {old}, and earlier of {preimage}"
            )),
        }
    }
}

/// **`old` and `new` are one document up to minted ids**, `new`'s
/// output variables set aside. `Err` names the first path where they
/// part.
pub fn equal_up_to_ids(old: &Value, new: &Value) -> Result<(), String> {
    same_up_to_ids(old, &without_outputs(new))
}

/// **`old` and `new` are one document up to minted ids**, each as it
/// stands. `Err` names the first path where they part.
pub fn same_up_to_ids(old: &Value, new: &Value) -> Result<(), String> {
    let mut map = Bijection::default();
    walk(old, new, "$", &mut map)
}

/// An object's entries, ids in mint order and every other key after
/// them in text order.
fn in_mint_order(object: &serde_json::Map<String, Value>) -> Vec<(&String, &Value)> {
    let mut entries: Vec<_> = object.iter().collect();
    entries.sort_by_key(|(key, _)| {
        (
            editor_core::MintId::parse(key).is_none(),
            editor_core::MintId::parse(key),
            key.as_str(),
        )
    });
    entries
}

fn walk(old: &Value, new: &Value, at: &str, map: &mut Bijection) -> Result<(), String> {
    match (old, new) {
        (Value::String(a), Value::String(b)) if is_id(a) && is_id(b) => map.pair(a, b, at),
        (Value::Object(a), Value::Object(b)) => {
            if a.len() != b.len() {
                return Err(format!("{at}: {} entries against {}", a.len(), b.len()));
            }
            // Entries pair in mint order: minting more ids keeps the
            // order of the ones both documents hold, where the text of
            // an id does not.
            for ((ka, va), (kb, vb)) in in_mint_order(a).into_iter().zip(in_mint_order(b)) {
                let here = format!("{at}.{ka}");
                if ka == "chain" && at.ends_with(".mint") {
                    continue;
                }
                if is_id(ka) && is_id(kb) {
                    map.pair(ka, kb, &here)?;
                } else if ka != kb {
                    return Err(format!("{at}: key {ka} against {kb}"));
                }
                walk(va, vb, &here, map)?;
            }
            Ok(())
        }
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                return Err(format!("{at}: {} elements against {}", a.len(), b.len()));
            }
            for (i, (va, vb)) in a.iter().zip(b).enumerate() {
                walk(va, vb, &format!("{at}[{i}]"), map)?;
            }
            Ok(())
        }
        (a, b) if a == b => Ok(()),
        (a, b) => Err(format!("{at}: {a} against {b}")),
    }
}
