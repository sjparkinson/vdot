use fern::colors::ColoredLevelConfig;
use fern::Dispatch as Logger;
use log::Level;
use std::io;

pub fn init(level: Level) {
    let colors = ColoredLevelConfig::default();
    let result = Logger::new()
        .chain(
            // Handle warn and error logs.
            Logger::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{}: {}",
                        colors.color(record.level()).to_string().to_lowercase(),
                        message,
                    ))
                })
                .level_for("vdot", level.to_level_filter())
                .filter(|metadata| metadata.level() <= Level::Warn)
                .chain(io::stderr()),
        )
        .chain(
            // Handle info, debug, and trace logs.
            Logger::new()
                .format(|out, message, _| out.finish(format_args!("{}", message,)))
                .level_for("vdot", level.to_level_filter())
                .filter(|metadata| metadata.level() >= Level::Info)
                .chain(io::stdout()),
        )
        .apply();

    // Avoid an unwrap, if logging failed to setup something is really wrong.
    match result {
        Ok(()) => (),
        Err(err) => panic!(err),
    }
}
