/// Print debug msg, only when compiled in debug mode
///
/// # Examples
///
/// ```
/// // print only label
/// __wslcmd_dbg!("label");
///
/// // print label and vars (with dbg! macro)
/// __wslcmd_dbg!("label", var1, var2, ...);
///
/// // print only vars
/// //   almost same with dbg!() macro,
/// //   except that this macro only prints on debug mode
/// __wslcmd_dbg!(var1, var2, ...);
/// ```
///
#[macro_export]
macro_rules! __wslcmd_dbg {

    // for label-only
    //   __wslcmd_dbg!("label");
    ($label:literal) => {

            // for debug mode
            #[cfg(debug_assertions)]
            {
                // print header
                crate::__wslcmd_dbg_header!($label);
            }

            // for release mode
            #[cfg(not(debug_assertions))]
            {
                // do nothing on release mode, just removing unused warnings
                ($label)
            }

    };

    // for label and vars
    //   __wslcmd_dbg!("label", var1, var2, ...);
    ($label:literal, $($args:expr), +) => {
        {

            // for debug mode
            #[cfg(debug_assertions)]
            {
                // print header
                crate::__wslcmd_dbg_header!($label);
                // print body
                crate::__wslcmd_dbg_body!($($args),+)
            }

            // for release mode
            #[cfg(not(debug_assertions))]
            {
                // do nothing on release mode, just removing unused warnings
                ($label,$($args), +)
            }

        }
    };


    // for vars only
    //   __wslcmd_dbg!(var1, var2, ...);
    ($($args:expr), +) => {
        {

            // for debug mode
            #[cfg(debug_assertions)]
            {
                // print body
                crate::__wslcmd_dbg_body!($($args),+)
            }

            // for release mode
            #[cfg(not(debug_assertions))]
            {
                // do nothing on release mode, just removing unused warnings
                ($($args), +)
            }

        }
    };
}

/// Prints debug msg header only, if str is not empty (for the macro [`__wslcmd_dbg`])
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! __wslcmd_dbg_header {
    ($label:literal) => {
        #[cfg(debug_assertions)]
        {
            use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
            let mut stderr = StandardStream::stderr(ColorChoice::Always);

            // set color
            stderr
                .set_color(
                    ColorSpec::new()
                        .set_fg(Some(Color::Black))
                        .set_bg(Some(Color::Yellow)),
                )
                .ok();
            // print header
            eprint!(" [WSLCMD_DBG] {} ", $label);
            // reset color
            stderr.reset().ok();
            eprintln!();
        }
    };
}

/// Prints debug msg body (args) only (for the macro [`__wslcmd_dbg`])
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! __wslcmd_dbg_body {
    ($($args:expr), +) => {
        #[cfg(debug_assertions)]
        {
            use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
            let mut stderr = StandardStream::stderr(ColorChoice::Always);

            // set color
            stderr.set_color(
                ColorSpec::new()
                    .set_fg(Some(Color::Yellow))
            ).ok();
            // print vars
            let ret = dbg!($($args),+);
            // reset color
            stderr.reset().ok();
            ret
        }
    };
}
