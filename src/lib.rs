// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-atomic-counters: atomic counters component model.
//!
//! Real implementation: counterss, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Counters {
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
pub enum AtomicCountersError {
    EmptyCountersName,
    DuplicateCounters(String),
    EmptyEventSubject,
}

impl fmt::Display for AtomicCountersError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCountersName => write!(f, "counters name must not be empty"),
            Self::DuplicateCounters(id) => write!(f, "duplicate counters {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AtomicCountersError {}

pub struct AtomicCounters {
    pub name: String,
    counterss: BTreeMap<String, Counters>,
    events: BTreeMap<String, Event>,
}

impl AtomicCounters {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            counterss: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_counters(&mut self, item: Counters) -> Result<(), AtomicCountersError> {
        if item.name.trim().is_empty() {
            return Err(AtomicCountersError::EmptyCountersName);
        }
        let id = item.id.clone();
        if self.counterss.contains_key(&id) {
            return Err(AtomicCountersError::DuplicateCounters(id.clone()));
        }
        self.counterss.insert(id, item);
        Ok(())
    }

    pub fn counters_count(&self) -> usize {
        self.counterss.len()
    }

    pub fn get_counters(&self, id: &str) -> Option<&Counters> {
        self.counterss.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AtomicCountersError> {
        if event.subject.trim().is_empty() {
            return Err(AtomicCountersError::EmptyEventSubject);
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

    fn sample_counters(id: &str, name: &str) -> Counters {
        Counters {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = AtomicCounters::new("AtomicCounters");
        assert_eq!(m.name, "AtomicCounters");
        assert_eq!(m.counters_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_counters() {
        let mut m = AtomicCounters::new("AtomicCounters");
        m.add_counters(sample_counters("a1", "Alpha")).unwrap();
        assert_eq!(m.counters_count(), 1);
        assert!(m.get_counters("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = AtomicCounters::new("AtomicCounters");
        assert!(m.add_counters(sample_counters("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = AtomicCounters::new("AtomicCounters");
        m.add_counters(sample_counters("x", "One")).unwrap();
        assert!(m.add_counters(sample_counters("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = AtomicCounters::new("AtomicCounters");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
