// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-virt-install: virt install component model.
//!
//! Real implementation: installs, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Install {
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
pub enum VirtInstallError {
    EmptyInstallName,
    DuplicateInstall(String),
    EmptyEventSubject,
}

impl fmt::Display for VirtInstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInstallName => write!(f, "install name must not be empty"),
            Self::DuplicateInstall(id) => write!(f, "duplicate install {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for VirtInstallError {}

pub struct VirtInstall {
    pub name: String,
    installs: BTreeMap<String, Install>,
    events: BTreeMap<String, Event>,
}

impl VirtInstall {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            installs: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_install(&mut self, item: Install) -> Result<(), VirtInstallError> {
        if item.name.trim().is_empty() {
            return Err(VirtInstallError::EmptyInstallName);
        }
        let id = item.id.clone();
        if self.installs.contains_key(&id) {
            return Err(VirtInstallError::DuplicateInstall(id.clone()));
        }
        self.installs.insert(id, item);
        Ok(())
    }

    pub fn install_count(&self) -> usize {
        self.installs.len()
    }

    pub fn get_install(&self, id: &str) -> Option<&Install> {
        self.installs.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), VirtInstallError> {
        if event.subject.trim().is_empty() {
            return Err(VirtInstallError::EmptyEventSubject);
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

    fn sample_install(id: &str, name: &str) -> Install {
        Install {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = VirtInstall::new("VirtInstall");
        assert_eq!(m.name, "VirtInstall");
        assert_eq!(m.install_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_install() {
        let mut m = VirtInstall::new("VirtInstall");
        m.add_install(sample_install("a1", "Alpha")).unwrap();
        assert_eq!(m.install_count(), 1);
        assert!(m.get_install("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = VirtInstall::new("VirtInstall");
        assert!(m.add_install(sample_install("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = VirtInstall::new("VirtInstall");
        m.add_install(sample_install("x", "One")).unwrap();
        assert!(m.add_install(sample_install("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = VirtInstall::new("VirtInstall");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
