//! Default yaml configuration file parser.

use super::{FileContentError, FileNotFound, YamlTask, YamlTaskError};
use crate::{utils::ParseError, Action, CommandAction, Parser, Task};

use std::{collections::HashMap, fs::File, io::Read, str::FromStr, sync::Arc};
use yaml_rust::{Yaml, YamlLoader};

/// An implementation of [`Parser`]. It is the default yaml configuration file parser.
pub struct YamlParser;

impl YamlParser {
    /// Given file path, and load configuration file.
    fn load_file(&self, file: &str) -> Result<String, ParseError> {
        let mut content = String::new();
        let mut yaml = File::open(file).map_err(FileNotFound)?;
        yaml.read_to_string(&mut content).unwrap();
        Ok(content)
    }
    /// Parses an item in the configuration file into a task.
    /// An item refers to:
    ///
    /// ```yaml
    ///    name: "Task 1"
    ///    after: [b, c]
    ///    cmd: echo a
    /// ```
    fn parse_one<Name: FromStr>(
        &self,
        id: &str,
        item: &Yaml,
        specific_action: Option<Action>,
    ) -> Result<YamlTask<Name>, YamlTaskError> {
        // Get name first
        let name_str = item["name"]
            .as_str()
            .ok_or(YamlTaskError::NoNameAttr(id.to_owned()))?;
        let name: Name = Name::from_str(name_str).unwrap();

        // precursors can be empty
        let mut precursors = Vec::new();
        if let Some(after_tasks) = item["after"].as_vec() {
            after_tasks.iter().for_each(|task_id| {
                precursors.push(Name::from_str(task_id.as_str().unwrap()).unwrap())
            });
        }

        if let Some(action) = specific_action {
            Ok(YamlTask::new(id, precursors, name, action))
        } else {
            let cmd = item["cmd"]
                .as_str()
                .ok_or(YamlTaskError::NoScriptAttr(name_str.to_owned()))?;
            Ok(YamlTask::new(
                id,
                precursors,
                name,
                Action::Structure(Arc::new(CommandAction::new(cmd))),
            ))
        }
    }
}

impl<Name: ToString + Send + Sync + ToOwned> Parser<Name> for YamlParser {
    fn parse_tasks(
        &self,
        file: &str,
        mut specific_actions: HashMap<String, Action>,
    ) -> Result<Vec<Box<dyn Task<Name>>>, ParseError> {
        let content = self.load_file(file)?;
        // Parse Yaml
        let yaml_tasks =
            YamlLoader::load_from_str(&content).map_err(FileContentError::IllegalYamlContent)?;
        // empty file error
        if yaml_tasks.is_empty() {
            return Err(FileContentError::Empty(file.to_string()).into());
        }
        let yaml_tasks = yaml_tasks[0]["dagrs"]
            .as_hash()
            .ok_or(YamlTaskError::StartWordError)?;

        let mut tasks = Vec::new();
        let mut map = HashMap::new();
        // Read tasks
        for (v, w) in yaml_tasks {
            let id = v.as_str().unwrap();
            let task = if specific_actions.contains_key(id) {
                let action = specific_actions.remove(id).unwrap();
                self.parse_one(id, w, Some(action))?
            } else {
                self.parse_one(id, w, None)?
            };
            map.insert(id, task.id());
            tasks.push(task);
        }

        for task in tasks.iter_mut() {
            let mut pres = Vec::new();
            for pre in task.str_precursors() {
                if map.contains_key(&pre[..]) {
                    pres.push(map[&pre[..]]);
                } else {
                    return Err(YamlTaskError::NotFoundPrecursor(task.name().to_string()).into());
                }
            }
            task.init_precursors(pres);
        }

        Ok(tasks
            .into_iter()
            .map(|task| Box::new(task) as Box<dyn Task>)
            .collect())
    }
}
