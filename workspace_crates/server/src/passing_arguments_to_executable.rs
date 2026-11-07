#[derive(Debug, Default)]
pub struct Args {
    pub version: bool,
}

pub fn parse_args() -> Result<Args, String> {
    let mut args = Args::default();

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-v" | "--version" => args.version = true,
            other => {
                return Err(format!(
                    "Unknown argument: {other}\nUsage: app [-v | --version]"
                ));
            }
        }
    }

    Ok(args)
}

pub fn will_show_version_then_would_like_to_terminate_the_program(app_executable_version: &str) -> bool {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            tracing::error!("Could not pass args: {e}");
            return false;
        }
    };

    if args.version {
        println!("App version: {app_executable_version}", );
        return true;
    }
    false
}
