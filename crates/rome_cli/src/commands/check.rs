use crate::configuration::load_configuration;
use crate::parse_arguments::{
    apply_files_settings_from_cli, apply_format_settings_from_cli, apply_vcs_settings_from_cli,
};
use crate::vcs::store_path_to_ignore_from_vcs;
use crate::{execute_mode, CliDiagnostic, CliSession, Execution, TraversalMode};
use rome_console::{markup, ConsoleExt};
use rome_diagnostics::{DiagnosticExt, PrintDiagnostic, Severity};
use rome_service::workspace::{FixFileMode, UpdateSettingsParams};

/// Handler for the "check" command of the Rome CLI
pub(crate) fn check(mut session: CliSession) -> Result<(), CliDiagnostic> {
    let (mut configuration, diagnostics, configuration_path) =
        load_configuration(&mut session)?.consume();
    if !diagnostics.is_empty() {
        let console = &mut session.app.console;
        console.log(markup!{
           <Warn>"Found errors in the configuration file, Rome will use its defaults for the sections that are incorrect."</Warn>
        });
        for diagnostic in diagnostics {
            let diagnostic = diagnostic.with_severity(Severity::Warning);
            console.log(markup! {
                {PrintDiagnostic::verbose(&diagnostic)}
            })
        }
    }
    apply_files_settings_from_cli(&mut session, &mut configuration)?;
    apply_format_settings_from_cli(&mut session, &mut configuration)?;
    apply_vcs_settings_from_cli(&mut session, &mut configuration)?;

    // check if support of git ignore files is enabled
    let vcs_base_path = configuration_path.or(session.app.fs.working_directory());
    store_path_to_ignore_from_vcs(&mut session, &mut configuration, vcs_base_path)?;

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams { configuration })?;

    let apply = session.args.contains("--apply");
    let apply_suggested = session.args.contains("--apply-unsafe");

    let fix_file_mode = if apply && apply_suggested {
        return Err(CliDiagnostic::incompatible_arguments(
            "--apply",
            "--apply-unsafe",
        ));
    } else if !apply && !apply_suggested {
        None
    } else if apply && !apply_suggested {
        Some(FixFileMode::SafeFixes)
    } else {
        Some(FixFileMode::SafeAndUnsafeFixes)
    };

    execute_mode(
        Execution::new(TraversalMode::Check { fix_file_mode }),
        session,
    )
}
