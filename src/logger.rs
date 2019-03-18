use fern::colors::ColoredLevelConfig;
use fern::Dispatch as Logger;
use log::{debug, Level};
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
/// use log::{Level, info, warn};
/// use vdot::logger;
///
/// logger::init(Level::Info);
///
/// info!("Hello world!");
/// warn!("This is a warn message.");
/// ```
///
/// # Panics
///
/// This function will panic in the unlikely case that is is unable to build a logger instance.
pub fn init(level: Level) {
    let colors = ColoredLevelConfig::default();
    let level = level.to_level_filter();

    Logger::new()
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
                .level(level)
                .filter(|metadata| metadata.level() <= Level::Warn && metadata.target() == "vdot")
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
                .level(level)
                .filter(|metadata| metadata.level() >= Level::Info && metadata.target() == "vdot")
                .chain(io::stdout()),
        )
        .apply()
        .expect("failed to initialise logging");

    debug!("initialised to log {} messages", level);
}
