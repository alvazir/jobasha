use crate::{msg, Cfg, ComparePlugin, ComparePlugins, Log, MsgTone};
use anyhow::{anyhow, Context, Result};
use std::path::Path;

pub(crate) fn get_plugins_to_compare(cfg: &Cfg, log: &mut Log) -> Result<ComparePlugins> {
    let mut compare_plugins = ComparePlugins::default();
    let mut path: &Path;
    let mut comp: &mut ComparePlugin;
    let mut purpose: &str;
    if !cfg.no_compare {
        if !cfg.compare_with.is_empty() {
            (path, comp, purpose) = (
                Path::new(&cfg.compare_with),
                &mut compare_plugins.compare_with,
                "\"--compare-with\"",
            );
            get_plugin_to_compare(path, comp, purpose, false, cfg, log)
                .with_context(|| format!("Failed to load {purpose} plugin: {path:?}"))?;
        }
        if !cfg.compare_delev_with.is_empty() {
            (path, comp, purpose) = (
                Path::new(&cfg.compare_delev_with),
                &mut compare_plugins.delev_compare_with,
                "\"--delev-compare-with\"",
            );
            get_plugin_to_compare(path, comp, purpose, false, cfg, log)
                .with_context(|| format!("Failed to load {purpose} plugin: {path:?}"))?;
        }
        (path, comp, purpose) = (&cfg.output.path, &mut compare_plugins.previous, "previous output");
        get_plugin_to_compare(path, comp, purpose, true, cfg, log)?;
        if cfg.delev && cfg.delev_distinct {
            (path, comp, purpose) = (&cfg.delev_output.path, &mut compare_plugins.delev_previous, "previous delev");
            get_plugin_to_compare(path, comp, purpose, true, cfg, log)?;
        }
    }
    Ok(compare_plugins)
}

fn get_plugin_to_compare(
    path: &Path,
    result: &mut ComparePlugin,
    purpose: &str,
    fallible: bool,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    let (path_exists, path_is_file) = get_path_exists_is_file(path);
    if path_is_file {
        match result.plugin.load_path(path) {
            Ok(()) => result.loaded = true,
            Err(error) => match fallible {
                true => {
                    result.load_error = true;
                    let text =
                        format!("Failed to load {purpose} plugin: {path:?}. Continuing as if --no-compare was passed. Error: {error}");
                    msg(&text, MsgTone::Bad, 0, cfg, log)?;
                }
                false => {
                    return Err(anyhow!(error));
                }
            },
        };
    } else if path_exists {
        let error = "Path exists though it's not a file";
        match fallible {
            true => {
                result.read_error = true;
                let text =
                    format!("Failed to load {purpose} plugin: {path:?}. Continuing as if --no-compare was passed. Error: {error:?}");
                msg(&text, MsgTone::Bad, 0, cfg, log)?;
            }
            false => {
                return Err(anyhow!(error));
            }
        }
    } else {
        match fallible {
            true => {
                result.not_found = true;
                let text = format!(
                    "{} plugin not found: {path:?}. Continuing as if --no-compare was passed.",
                    uppercase_first_letter(purpose)
                );
                msg(&text, MsgTone::Neutral, 1, cfg, log)?;
            }
            false => {
                return Err(anyhow!("File not found"));
            }
        }
    };
    Ok(())
}

fn get_path_exists_is_file(path: &Path) -> (bool, bool) {
    match path.metadata() {
        Err(_) => (false, false),
        Ok(metadata) => (true, metadata.is_file()),
    }
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
