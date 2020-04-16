use anyhow::Result;
use clap::{App, ArgMatches};
use rhq::Workspace;

#[derive(Debug)]
pub struct RefreshCommand {
    verbose: bool,
}

impl RefreshCommand {
    pub fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Scan repository list and drop if it is not existed or matches exclude pattern.")
            .arg_from_usage("-v, --verbose 'Use verbose output'")
    }

    pub fn from_matches(m: &ArgMatches) -> RefreshCommand {
        RefreshCommand {
            verbose: m.is_present("verbose"),
        }
    }

    pub fn run(self) -> Result<()> {
        let mut workspace = Workspace::new()?.verbose_output(self.verbose);
        workspace.drop_invalid_repositories();
        workspace.save_cache()?;
        Ok(())
    }
}
