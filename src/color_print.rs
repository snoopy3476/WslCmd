/// Print as colored text, which is wrapper of termcolor crate
///
/// # Examples
///
/// ```
/// #[macro_use]
/// mod color_print;
///
/// // print only literal
/// cprint!(Color::Red, "label");
///
/// // print with formatting
/// let var = "test";
/// cprint!(Color::Red, "label {}", var);
/// ```
///
macro_rules! cprint {

    ($color:expr, $label:literal) => {{
        use std::io::Write;

        // colored output writer
        let buf_writer = termcolor::BufferWriter::stdout(termcolor::ColorChoice::Auto);
        let mut buf = buf_writer.buffer();

        __cprint_raw!($color, buf_writer, &mut buf, write!(buf, $label));
        std::io::stdout().flush().ok();
    }};

    ($color:expr, $label:literal, $($args:expr), +) => {{
        use std::io::Write;

        // colored output writer
        let buf_writer = termcolor::BufferWriter::stdout(termcolor::ColorChoice::Auto);
        let mut buf = buf_writer.buffer();

        __cprint_raw!($color, buf_writer, &mut buf, write!(buf, $label, $($args), +));
        std::io::stdout().flush().ok();
    }};
}

/// Print as colored text with a trailing newline, which is wrapper of termcolor crate
///
/// # Examples
///
/// ```
/// #[macro_use]
/// mod color_print;
///
/// // print only literal
/// cprintln!(Color::Red, "label");
///
/// // print with formatting
/// let var = "test";
/// cprintln!(Color::Red, "label {}", var);
/// ```
///
macro_rules! cprintln {

    ($color:expr, $label:literal) => {{

        // colored output writer
        let buf_writer = termcolor::BufferWriter::stdout(termcolor::ColorChoice::Auto);
        let mut buf = buf_writer.buffer();

        __cprint_raw!($color, buf_writer, &mut buf, writeln!(buf, $label));
    }};

    ($color:expr, $label:literal, $($args:expr), +) => {{
        // colored output writer
        let buf_writer = termcolor::BufferWriter::stdout(termcolor::ColorChoice::Auto);
        let mut buf = buf_writer.buffer();

        __cprint_raw!($color, buf_writer, &mut buf, writeln!(buf, $label, $($args), +));
    }};
}

/// Raw print func for other cprint macros
macro_rules! __cprint_raw {
    ($color:expr, $buf_writer:expr, $buf:expr, $print_call:expr) => {{
        use std::io::Write;
        use termcolor::{Color, WriteColor};

        $buf.set_color(termcolor::ColorSpec::new().set_fg(Some($color)))
            .and_then(|_| $print_call)
            .and_then(|_| $buf.set_color(termcolor::ColorSpec::new().set_reset(true)))
            .and_then(|_| $buf_writer.print($buf))
            .and_then(|_| Ok($buf.clear()))
            .ok()
    }};
}
