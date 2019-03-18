use fern::colors::ColoredLevelConfig;
use fern::Dispatch as Logger;
use log::Level;
use std::io;

/// Configure logging.
///
/// Uses the [`fern`] crate to configure a logger that prints to stdout and stderr, with colourised output.
///
/// *Note*: Logs of [`Level::Warn`] and above are written to stderr.
///
/// [`fern`]: https://crates.io/crates/fern
/// [`Level::Warn`]: ../../log/enum.Level.html#variant.Warn
///
/// # Examples
///
/// ```
/// use log::{Level, info};
/// use vdot::logger;
///
/// logger::init(Level::Info);
///
/// info!("Hello world!");
/// ```
///
/// # Panics
///
/// This function will panic in the unlikely case that is is unable to build a logger instance.
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
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{}: {}",
                        colors.color(record.level()).to_string().to_lowercase(),
                        message
                    ))
                })
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
