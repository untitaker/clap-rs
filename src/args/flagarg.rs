/// `FlagArg` represents a flag argument for command line applications. Flag arguments 
/// take no additional values, and are always preceded by either a `-` (single character)
/// or `--` (single word, no spaces). `FlagArg` isn't directly used by the end application
/// writer, only internally to the `clap` library.
///
/// # Example
///
/// ```sh
/// $ myprog -a --some
/// ```
pub struct FlagArg {
	/// The unique name of the argument, required
    pub name: &'static str,
    /// The short version (i.e. single character)
    /// of the argument, no preceding `-`
    pub short: Option<char>,
    /// The long version of the flag (i.e. word)
    /// without the preceding `--`
    pub long: Option<&'static str>,
    /// The string of text that will displayed to 
    /// the user when the application's `help` 
    /// text is displayed
    pub help: Option<&'static str>,
    /// Determines if multiple instances of the same
    /// flag are allowed
    /// I.e. `-v -v -v` or `-vvv`
    pub multiple: bool,
    /// How many occurences of this flag have been
    /// found when parsing
    pub occurrences: u8,
    /// A list of names for other arguments that
    /// *may not* be used with this flag
    pub blacklist: Option<Vec<&'static str>>,
    /// A list of names of other arguments that 
    /// are *required* to be used when this
    /// flag is used
    pub requires: Option<Vec<&'static str>>
}