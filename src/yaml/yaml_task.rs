//! Task definition of type Yaml.
//!
//! # The task type corresponding to the configuration file: [`YamlTask`]
//!
//! [`YamlTask`] implements the [`Task`] trait, which represents the tasks in the yaml
//! configuration file, and a yaml configuration file will be parsed into a series of [`YamlTask`].
//! It is different from `DefaultTask`, in addition to the four mandatory attributes of the
//! task type, he has several additional attributes.

use kstring::KString;

use crate::{alloc_id, Action, Task};

/// Task struct for yaml file.
pub struct YamlTask {
    /// `yid` is the unique identifier defined in yaml, and `id` is the id assigned by the global id assigner.
    yid: KString,
    id: usize,
    name: KString,
    /// Precursor identifier defined in yaml.
    precursors: Vec<KString>,
    precursors_id: Vec<usize>,
    action: Action,
}

impl YamlTask {
    pub fn new(yaml_id: &str, precursors: Vec<KString>, name: KString, action: Action) -> Self {
        Self {
            yid: KString::from_ref(yaml_id),
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
    pub fn str_precursors(&self) -> Vec<KString> {
        self.precursors.clone()
    }
    /// Get the unique ID of the task defined in yaml.
    pub fn str_id(&self) -> &str {
        &self.yid
    }
}

impl Task for YamlTask {
    fn action(&self) -> Action {
        self.action.clone()
    }
    fn precursors(&self) -> &[usize] {
        &self.precursors_id
    }
    fn id(&self) -> usize {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
}
