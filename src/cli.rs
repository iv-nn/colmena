//! Global CLI Setup.

use clap::{App, AppSettings, Arg};
use crate::command;

macro_rules! register_command {
    ($module:ident, $app:ident) => {
        $app = $app.subcommand(command::$module::subcommand());
    };
}

macro_rules! handle_command {
    ($module:ident, $matches:ident) => {
        if let Some(sub_matches) = $matches.subcommand_matches(stringify!($module)) {
            command::$module::run(&$matches, &sub_matches).await;
            return;
        }
    };
    ($name:expr, $module:ident, $matches:ident) => {
        if let Some(sub_matches) = $matches.subcommand_matches($name) {
            command::$module::run(&$matches, &sub_matches).await;
            return;
        }
    };
}

pub fn build_cli() -> App<'static, 'static> {
    let mut app = App::new("Colmena")
        .version("0.1.0")
        .author("Zhaofeng Li <hello@zhaofeng.li>")
        .about("NixOS deployment tool")
        .global_setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("config")
            .short("f")
            .long("config")
            .value_name("CONFIG")
            .help("Path to a Hive expression")

            // The default value is a lie (sort of)!
            //
            // The default behavior is to search upwards from the
            // current working directory for a file named "hive.nix".
            // This behavior is disabled if --config/-f is explicitly
            // supplied by the user (occurrences_of > 0).
            .default_value("hive.nix")
            .long_help(r#"If this argument is not specified, Colmena will search upwards from the current working directory for a file named "hive.nix". This behavior is disabled if --config/-f is given explicitly.

For a sample configuration, see <https://github.com/zhaofengli/colmena>.
"#)
            .global(true))
        .arg(Arg::with_name("show-trace")
            .long("show-trace")
            .help("Show debug information for Nix commands")
            .long_help("Passes --show-trace to Nix commands")
            .global(true)
            .takes_value(false));

    register_command!(apply, app);
    register_command!(apply_local, app);
    register_command!(build, app);
    register_command!(introspect, app);
    register_command!(upload_keys, app);
    register_command!(exec, app);

    app
}

pub async fn run() {
    let mut app = build_cli();
    let matches = app.clone().get_matches();

    handle_command!(apply, matches);
    handle_command!("apply-local", apply_local, matches);
    handle_command!(build, matches);
    handle_command!(introspect, matches);
    handle_command!("upload-keys", upload_keys, matches);
    handle_command!(exec, matches);

    app.print_long_help().unwrap();
    println!();
}