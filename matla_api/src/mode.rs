//! Aggregates matla's run-modes.

pub mod apalache;
pub mod clean;
pub mod init;
pub mod run;
pub mod setup;
pub mod testing;
pub mod tlc;
pub mod uninstall;
pub mod update;

#[cfg(feature = "with_clap")]
pub use self::requires_clap::*;

/// Aggregate all clap-related things in a private module.
#[cfg(feature = "with_clap")]
mod requires_clap {
    prelude!();

    /// Generates all the subcommands for all the modes.
    pub fn all_subcommands<'a>() -> [clap::Command<'static>; 9] {
        [
            super::clean::cla::subcommand(),
            super::init::cla::subcommand(),
            super::run::cla::subcommand(),
            super::setup::cla::subcommand(),
            super::testing::cla::subcommand(),
            super::tlc::cla::subcommand(),
            super::apalache::cla::subcommand(),
            super::uninstall::cla::subcommand(),
            super::update::cla::subcommand(),
        ]
    }

    /// Wraps the result of `action()` in a `Some(_)`.
    macro_rules! wrap_try {
        ($e:expr) => {
            match $e {
                Ok(res) => res,
                Err(e) => return Some(Err(e.into())),
            }
        };
    }

    /// Runs the mode specified by `matches`, if any.
    ///
    /// Only considers modes that must run **before** user configuration loading.
    pub fn try_pre_user_load(matches: &clap::ArgMatches) -> Option<Res<Option<i32>>> {
        if let Some(setup) = mode::setup::cla::check_matches(&matches) {
            let setup = wrap_try! {
                setup.context("setup mode initialization failed")
            };
            wrap_try! {
                setup.launch().context("mode setup failed")
            }
            Some(Ok(None))
        } else if let Some(uninstall) = mode::uninstall::cla::check_matches(&matches) {
            let uninstall = wrap_try! {
                uninstall.context("uninstall mod initialization failed")
            };
            wrap_try! {
                uninstall.launch().context("mode uninstall failed")
            }
            Some(Ok(None))
        } else {
            None
        }
    }

    /// Runs the mode specified by `matches`, if any.
    ///
    /// Only considers modes that must run **before** user configuration loading.
    pub fn try_pre_project_load(matches: &clap::ArgMatches) -> Option<Res<Option<i32>>> {
        if let Some(init) = mode::init::cla::check_matches(&matches) {
            let init = wrap_try! {
                init.context("init mode initialization failed")
            };
            wrap_try! {
                init.launch().context("mode init failed")
            }
            Some(Ok(None))
        } else if let Some(update) = mode::update::cla::check_matches(&matches) {
            let update = wrap_try! {
                update.context("update mode initialization failed")
            };
            wrap_try! {
                update.launch().context("mode update failed")
            }
            Some(Ok(None))
        } else if let Some(tlc) = mode::tlc::cla::check_matches(&matches) {
            let tlc = wrap_try! {
                tlc.context("tlc mode initialization failed")
            };
            let exit_code = wrap_try! {
                wrap_try!(tlc.launch().context("mode tlc failed"))
                    .code()
                    .ok_or_else(|| anyhow!("failed to retrieve exit code of TLC process"))
            };
            Some(Ok(Some(exit_code)))
        } else if let Some(apalache) = mode::apalache::cla::check_matches(&matches) {
            let apalache = wrap_try! {
                apalache.context("apalache mode initialization failed")
            };
            let exit_code = wrap_try! {
                wrap_try!(apalache.launch().context("mode apalache failed"))
                    .code()
                    .ok_or_else(|| anyhow!("failed to retrieve exit code of Apalache process"))
            };
            Some(Ok(Some(exit_code)))
        } else {
            None
        }
    }

    /// Runs the mode specified by `matches`, if any.
    ///
    /// Only considers modes that must run **after** user configuration loading.
    pub fn try_post_loading(matches: &clap::ArgMatches) -> Option<Res<Option<i32>>> {
        if let Some(run) = mode::run::cla::check_matches(&matches) {
            let run = wrap_try! {
                run.context("run mode initialization failed")
            };
            let exit_code = wrap_try! {
                run.launch().context("mode run failed")
            };
            Some(Ok(Some(exit_code)))
        } else if let Some(test) = mode::testing::cla::check_matches(&matches) {
            let test = wrap_try! {
                test.context("test mode initialization failed")
            };
            wrap_try! {
                test.launch().context("mode test failed")
            }
            Some(Ok(None))
        } else if let Some(clean) = mode::clean::cla::check_matches(&matches) {
            let clean = wrap_try! {
                clean.context("clean mode initialization failed")
            };
            wrap_try! {
                clean.launch().context("mode clean failed")
            }
            Some(Ok(None))
        } else {
            None
        }
    }
}
