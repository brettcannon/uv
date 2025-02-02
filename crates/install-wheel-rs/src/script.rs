use regex::Regex;
use rustc_hash::FxHashSet;
use serde::Serialize;

use crate::Error;

/// A script defining the name of the runnable entrypoint and the module and function that should be
/// run.
#[cfg(feature = "python_bindings")]
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[pyo3::pyclass(dict)]
pub struct Script {
    #[pyo3(get)]
    pub script_name: String,
    #[pyo3(get)]
    pub module: String,
    #[pyo3(get)]
    pub function: String,
}

/// A script defining the name of the runnable entrypoint and the module and function that should be
/// run.
#[cfg(not(feature = "python_bindings"))]
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Script {
    pub script_name: String,
    pub module: String,
    pub function: String,
}

impl Script {
    /// Parses a script definition like `foo.bar:main` or `foomod:main_bar [bar,baz]`
    ///
    /// <https://packaging.python.org/en/latest/specifications/entry-points/>
    ///
    /// Extras are supposed to be ignored, which happens if you pass None for extras
    pub fn from_value(
        script_name: &str,
        value: &str,
        extras: Option<&[String]>,
    ) -> Result<Option<Script>, Error> {
        let script_regex = Regex::new(r"^(?P<module>[\w\d_\-.]+):(?P<function>[\w\d_\-.]+)(?:\s+\[(?P<extras>(?:[^,]+,?\s*)+)\])?$").unwrap();

        let captures = script_regex
            .captures(value)
            .ok_or_else(|| Error::InvalidWheel(format!("invalid console script: '{value}'")))?;
        if let Some(script_extras) = captures.name("extras") {
            if let Some(extras) = extras {
                let script_extras = script_extras
                    .as_str()
                    .split(',')
                    .map(|extra| extra.trim().to_string())
                    .collect::<FxHashSet<String>>();
                if !script_extras.is_subset(&extras.iter().cloned().collect()) {
                    return Ok(None);
                }
            }
        }

        Ok(Some(Script {
            script_name: script_name.to_string(),
            module: captures.name("module").unwrap().as_str().to_string(),
            function: captures.name("function").unwrap().as_str().to_string(),
        }))
    }
}
