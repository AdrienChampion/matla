//! Apalache mode, just runs a Apalache command.

prelude!();

/// CLAP stuff.
#[cfg(feature = "with_clap")]
pub mod cla {
    use super::*;

    /// Apalache subcommand name.
    const CMD_NAME: &str = "apalache";
    /// Apalache options key.
    const ARGS_KEY: &str = "Apalache_ARGS_KEY";

    /// Apalache subcommand.
    pub fn subcommand() -> clap::Command<'static> {
        clap::Command::new(CMD_NAME)
            .alias("apa")
            .about("Calls Apalache with some arguments.")
            .args(&[
                crate::cla::top::project_path_arg(),
                clap::Arg::new(ARGS_KEY)
                    .help("Options to pass to the Apalache command *directly*")
                    .index(1)
                    .last(true)
                    .multiple_values(true)
                    .value_name("Apalache OPTIONS")
                    .takes_value(true),
                clap::Arg::new(crate::mode::run::cla::SHOW_CONFIG_KEY)
                    .help("Displays the options that matla will use to run Apalache")
                    .long("show_apalache_config"),
            ])
    }

    /// Constructs a [`Run`] if Apalache subcommand is active.
    pub fn check_matches(matches: &clap::ArgMatches) -> Option<Res<Run>> {
        matches.subcommand_matches(CMD_NAME).map(|matches| {
            let opts = matches.values_of(ARGS_KEY);
            let args = opts
                .into_iter()
                .map(|s| {
                    s.into_iter()
                        .map(|s| s.split(char::is_whitespace))
                        .flatten()
                })
                .flatten()
                .filter_map(|s| {
                    let s = s.trim();
                    if s.is_empty() {
                        None
                    } else {
                        Some(s.to_string())
                    }
                })
                .collect();
            let show_config = matches.is_present(crate::mode::run::cla::SHOW_CONFIG_KEY);
            Run::new(args, show_config)
        })
    }
}

/// Runs Apalache mode.
#[readonly]
#[derive(Debug, Clone)]
pub struct Run {
    /// Build target.
    pub target: conf::Target,
    /// Trailing arguments for Apalache.
    pub tail_args: Vec<String>,
    /// If true, show config before running.
    pub show_config: bool,
}
impl Run {
    /// Constructor.
    pub fn new(tail_args: Vec<String>, show_config: bool) -> Res<Self> {
        let target = conf::Target::new_run(conf::top_cla::project_path()?, true);
        Ok(Self {
            target,
            tail_args,
            show_config,
        })
    }

    /// Launches a plain Apalache command.
    pub fn launch(self) -> Res<io::ExitStatus> {
        // let mut cmd = conf::toolchain::tlc_cmd()?;
        // cmd.args(&self.args);

        let mut cmd = self
            .try_matla_project()?
            .expect("failed to generate Apalache command");
        cmd.args(&self.tail_args);

        let (child, mut com) = thread::ChildCmd::new(cmd);
        let handle = child.spawn();
        let styles = conf::Styles::new();

        'handle_run: loop {
            match com.next() {
                None => {
                    log::debug!("child is done");
                    break;
                }
                Some(Ok((line, is_stderr))) => {
                    if is_stderr {
                        print!("{} ", styles.fatal.paint(">"));
                    }
                    println!("{}", line);
                    continue 'handle_run;
                }
                Some(Err(e)) => {
                    report_error(e, ": in Apalache child");
                    continue 'handle_run;
                }
            }
        }

        match handle.join() {
            Ok(res) => res,
            Err(_) => bail!("failed to join Apalache process"),
        }
    }

    /// Prepares an Apalache command when inside a matla project.
    ///
    /// Involves
    /// - taking project configuration into account,
    /// - putting Apalache's trash files in `target`.
    pub fn try_matla_project(&self) -> Res<Option<io::Command>> {
        let project_path = conf::top_cla::project_path()?;
        log::info!("loading project from `{}`", project_path.display());
        let project = project::SourceProject::from_path(&project_path)?;

        log::info!("creating actual build project");
        let project = project.into_full_apalache(self.target.clone())?;

        let cmd = project.full_apalache_cmd()?;
        if self.show_config {
            print!("> {}", cmd.get_program().to_string_lossy());
            for arg in cmd.get_args() {
                print!(" \\\n    {}", arg.to_string_lossy());
            }
            println!();
            if let Some(path) = cmd.get_current_dir() {
                println!("| in `{}`", path.display());
            }
            println!()
        }
        Ok(Some(cmd))
    }
}
