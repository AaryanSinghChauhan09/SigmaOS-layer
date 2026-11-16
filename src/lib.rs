// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-layer: layer component model.
//!
//! Real implementation: layer_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayerItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayerError {
    EmptyLayerItemName,
    DuplicateLayerItem(String),
    EmptyEventSubject,
}

impl fmt::Display for LayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLayerItemName => write!(f, "layer_item name must not be empty"),
            Self::DuplicateLayerItem(id) => write!(f, "duplicate layer_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LayerError {}

pub struct Layer {
    pub name: String,
    layer_items: BTreeMap<String, LayerItem>,
    events: BTreeMap<String, Event>,
}

impl Layer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            layer_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_layer_item(&mut self, item: LayerItem) -> Result<(), LayerError> {
        if item.name.trim().is_empty() {
            return Err(LayerError::EmptyLayerItemName);
        }
        let id = item.id.clone();
        if self.layer_items.contains_key(&id) {
            return Err(LayerError::DuplicateLayerItem(id.clone()));
        }
        self.layer_items.insert(id, item);
        Ok(())
    }

    pub fn layer_item_count(&self) -> usize {
        self.layer_items.len()
    }

    pub fn get_layer_item(&self, id: &str) -> Option<&LayerItem> {
        self.layer_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LayerError> {
        if event.subject.trim().is_empty() {
            return Err(LayerError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_layer_item(id: &str, name: &str) -> LayerItem {
        LayerItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Layer::new("Layer");
        assert_eq!(m.name, "Layer");
        assert_eq!(m.layer_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_layer_item() {
        let mut m = Layer::new("Layer");
        m.add_layer_item(sample_layer_item("a1", "Alpha")).unwrap();
        assert_eq!(m.layer_item_count(), 1);
        assert!(m.get_layer_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Layer::new("Layer");
        assert!(m.add_layer_item(sample_layer_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Layer::new("Layer");
        m.add_layer_item(sample_layer_item("x", "One")).unwrap();
        assert!(m.add_layer_item(sample_layer_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Layer::new("Layer");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
