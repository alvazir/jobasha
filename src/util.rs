use crate::{Cfg, ListCounts};
use anyhow::{anyhow, Context, Result};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::{
    fs::{create_dir_all, File},
    io::{self, BufWriter, Write},
    path::Path,
    time::Instant,
};

pub(crate) enum MsgTone {
    Neutral,
    Warm,
    Good,
    Bad,
    Ugly,
}

macro_rules! msg {
    ($text:ident, $tone:ident, $verbose:ident, $cfg:ident) => {
        if !($cfg.quiet || $verbose > $cfg.verbose) {
            let text = $text.as_ref();
            if $cfg.no_color {
                eprintln!("{text}");
            } else {
                match $tone {
                    MsgTone::Neutral => {
                        eprintln!("{text}");
                    }
                    MsgTone::Warm => {
                        eprintln!("{}", $cfg.guts.color_suggestion.apply_to(text));
                    }
                    MsgTone::Good => {
                        eprintln!("{}", $cfg.guts.color_success.apply_to(text));
                    }
                    MsgTone::Bad => {
                        eprintln!("{}", $cfg.guts.color_warning.apply_to(text));
                    }
                    MsgTone::Ugly => {
                        eprintln!("{}", $cfg.guts.color_ignored_error.apply_to(text));
                    }
                }
            }
        }
    };
}

pub(crate) fn msg<S: AsRef<str>>(
    text: S,
    tone: MsgTone,
    verbose: u8,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    if !cfg.no_log {
        log.write(&text)
            .with_context(|| "Failed to write to log file buffer")?;
    }
    msg!(text, tone, verbose, cfg);
    Ok(())
}

fn msg_thread_safe<S: AsRef<str>>(text: S, tone: MsgTone, verbose: u8, cfg: &Cfg) -> Result<()> {
    msg!(text, tone, verbose, cfg);
    Ok(())
}

pub(crate) fn err_or_ignore<S: AsRef<str>>(text: S, cfg: &Cfg, log: &mut Log) -> Result<()> {
    if cfg.ignore_errors {
        msg(
            format!("{}{}", cfg.guts.prefix_ignored_error_message, text.as_ref()),
            MsgTone::Ugly,
            0,
            cfg,
            log,
        )
    } else {
        Err(anyhow!(format!(
            "{}{}",
            text.as_ref(),
            cfg.guts.suffix_add_ignore_errors_suggestion
        )))
    }
}

pub(crate) fn err_or_ignore_thread_safe<S: AsRef<str>>(text: S, cfg: &Cfg) -> Result<()> {
    if cfg.ignore_errors {
        msg_thread_safe(
            format!("{}{}", cfg.guts.prefix_ignored_error_message, text.as_ref()),
            MsgTone::Ugly,
            0,
            cfg,
        )
    } else {
        Err(anyhow!(format!(
            "{}{}",
            text.as_ref(),
            cfg.guts.suffix_add_ignore_errors_suggestion
        )))
    }
}

pub(crate) struct Progress {
    pub(crate) off: bool,
    period_ms: f64,
    bar: ProgressBar,
    timer: Instant,
    next_tick_ms: f64,
}

impl Progress {
    pub(crate) fn new(plugins_num: usize, cfg: &Cfg) -> Progress {
        let off = cfg.quiet || cfg.no_progress;
        let period_ms = 1000.0 / cfg.guts.progress_frequency as f64;
        Progress {
            off,
            period_ms,
            bar: get_progress_bar(off, plugins_num, cfg),
            timer: Instant::now(),
            next_tick_ms: period_ms,
        }
    }

    pub(crate) fn tick(&mut self, count: u64) {
        let elapsed = self.timer.elapsed().as_millis() as f64;
        if elapsed > self.next_tick_ms {
            self.bar.set_position(count);
            self.next_tick_ms = elapsed + self.period_ms;
        }
    }

    pub(crate) fn finish(&self) -> f64 {
        self.bar.finish();
        self.timer.elapsed().as_secs_f64()
    }
}

fn get_progress_bar(off: bool, plugins_num: usize, cfg: &Cfg) -> ProgressBar {
    if off {
        ProgressBar::with_draw_target(None, ProgressDrawTarget::hidden())
    } else {
        let target = ProgressDrawTarget::stderr_with_hz(cfg.guts.progress_frequency);
        let template = match cfg.no_progress_bar {
            true => &cfg.guts.progress_template,
            false => &cfg.guts.progress_bar_template,
        };
        let style = ProgressStyle::with_template(template)
            .unwrap()
            .progress_chars(&cfg.guts.progress_bar_chars);
        let bar = ProgressBar::with_draw_target(Some(plugins_num as u64), target)
            .with_style(style)
            .with_prefix(cfg.guts.progress_prefix.to_owned());
        bar.set_position(1);
        bar
    }
}

pub(crate) struct Log {
    pub(crate) buffer: Option<BufWriter<File>>,
}

impl Log {
    pub(crate) fn new(cfg: &Cfg) -> Result<Log> {
        if !cfg.no_log {
            let log = match &cfg.log {
                None => return Err(anyhow!("Failed to get log file name")),
                Some(log) => log,
            };
            create_dir_early(log, "log")?;
            let buffer = Some(BufWriter::new(File::create(log).with_context(|| {
                format!("Failed to create/open log file \"{}\"", log.display())
            })?));
            Ok(Log { buffer })
        } else {
            Ok(Log { buffer: None })
        }
    }

    pub(crate) fn write<S: AsRef<str>>(&mut self, text: S) -> io::Result<()> {
        match &mut self.buffer {
            None => Ok(()),
            Some(buffer) => {
                writeln!(buffer, "{}", text.as_ref())
            }
        }
    }
}

pub(crate) fn show_output_plugin_suggestion(
    counts: &ListCounts,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    if !cfg.dry_run && counts.placed != 0 {
        let text = format!(
            "\nPlace \"{}\" last in load order and activate\n",
            cfg.output.name
        );
        msg(text, MsgTone::Warm, 0, cfg, log)?;
    }
    Ok(())
}

pub(crate) fn show_log_path(cfg: &Cfg, log: &mut Log) -> Result<()> {
    if cfg.no_log {
        Ok(())
    } else {
        let log_path = match &cfg.log {
            None => return Err(anyhow!("Failed to show log path because it's empty")),
            Some(log_path) => log_path,
        };
        msg(
            format!("Log is being written into \"{}\"", log_path.display()),
            MsgTone::Good,
            0,
            cfg,
            log,
        )
    }
}

pub(crate) fn show_settings_written(cfg: &Cfg, log: &mut Log) -> Result<()> {
    msg(
        format!(
            "Wrote default program settings into \"{}\"",
            cfg.settings.display()
        ),
        MsgTone::Good,
        0,
        cfg,
        log,
    )
}

pub(crate) fn create_dir_early(path: &Path, name: &str) -> Result<()> {
    match path.parent() {
        None => {}
        Some(dir) => {
            if dir != Path::new("") && !dir.exists() {
                create_dir_all(dir).with_context(|| {
                    format!("Failed to create {} directory \"{}\"", dir.display(), name)
                })?;
                eprintln!(
                    "{} directory \"{}\" was created",
                    name[0..1].to_uppercase() + &name[1..],
                    dir.display()
                )
            }
        }
    }
    Ok(())
}
