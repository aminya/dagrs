//! Task definition of type Yaml.
//!
//! # The task type corresponding to the configuration file: [`YamlTask`]
//!
//! [`YamlTask`] implements the [`Task`] trait, which represents the tasks in the yaml
//! configuration file, and a yaml configuration file will be parsed into a series of [`YamlTask`].
//! It is different from `DefaultTask`, in addition to the four mandatory attributes of the
//! task type, he has several additional attributes.

use std::str::FromStr;

use crate::{alloc_id, Action, Task};

/// Task struct for yaml file.
pub struct YamlTask<Name: FromStr> {
    /// `yid` is the unique identifier defined in yaml, and `id` is the id assigned by the global id assigner.
    yid: Name,
    id: usize,
    name: Name,
    /// Precursor identifier defined in yaml.
    precursors: Vec<Name>,
    precursors_id: Vec<usize>,
    action: Action,
}

impl<Name: Clone> YamlTask<Name> {
    pub fn new(yaml_id: &str, precursors: Vec<Name>, name: Name, action: Action) -> Self {
        Self {
            yid: Name::from_ref(yaml_id),
            id: alloc_id(),
            name,
            precursors,
            precursors_id: Vec::new(),
            action,
        }
    }
    /// After the configuration file is parsed, the id of each task has been assigned.
    /// At this time, the `precursors_id` of this task will be initialized according to
    /// the id of the predecessor task of each task.
    pub fn init_precursors(&mut self, pres_id: Vec<usize>) {
        self.precursors_id = pres_id;
    }

    /// Get the precursor identifier defined in yaml.
    pub fn str_precursors(&self) -> Vec<Name> {
        self.precursors.clone()
    }
    /// Get the unique ID of the task defined in yaml.
    pub fn str_id(&self) -> &Name {
        &self.yid
    }
}

impl<Name: ToString + Send + Sync + ToOwned> Task<Name> for YamlTask<Name> {
    fn action(&self) -> Action {
        self.action.clone()
    }
    fn precursors(&self) -> &[usize] {
        &self.precursors_id
    }
    fn id(&self) -> usize {
        self.id
    }
    fn name(&self) -> &Name {
        &self.name
    }
}
