/*!
An ultra simple no-configuration logger with colored output and zero
dependencies.Just add it to dependencies and initalize it as soon as
possible.

```rust
use log::*;
use SimplestLogger;

fn main
{
	SimplestLogger::initalize(LevelFilter::Info);

	// your code!
}
````

SimplestLogger gives lots of line length for longer messages, and uses
easily-distingusishable but theme respecting (ANSI) terminal colors.

We also have some constants you can use to more easily line things up.
*/

use log::Record;
use log::Level;
use log::Metadata;
use log::SetLoggerError;
use log::LevelFilter;
use log::info;
use log::logger;

pub struct SimplestLogger
{
    level: Level,
}

// You probably already know this but a lot of devs don't know `static` v. `const`. Basically.
//
// - Const represents a VALUE and is INLINED whereever used. This leads to a
//   (slightly) bugger binary but faster run times.
// - Static represents a MEMORY ADDRESS and is looked for when its used.
//   this leads to a smaller binary but slower run times.
//
// Here we want values sicne we want them inlined, and so true with every case
// in this file.
//
// source: https://stackoverflow.com/questions/52751597/what-is-the-difference-between-a-constant-and-a-static-variable-and-which-should

pub const INDENT: &str = "        ";

const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

const DEBUG_COLOR: &str = "\x1b[95m";
const INFO_COLOR: &str = "\x1b[96m";
const ERROR_COLOR: &str = "\x1b[91m";
const WARNING_COLOR: &str = "\x1b[93m";

// I hate this but this is just the minimum level it will log! If you pass a higher value,
// say info, then trace and debug will still be excluded.
// const LOGGER: SimplestLogger = SimplestLogger { level: Level::Trace };

impl SimplestLogger
{
	// TODO: Make it write to a file too.

	#[inline(always)]
	pub fn initalize(level: LevelFilter) -> ()
	{
		/*!
		Initalize your logger. Note that any log macros used before this is
		called will simply be ignored, and that only one logger may be
		initalized and attempting to initalize a logger when there already
		is one will cause the program to panic.
		
		It is best practice to make this line 1 of your main function, and
		use it nowhere else so it is easy to know which cases need to be
		removed.
		
		Note that this function is always inlined as it must only be called
		once, so this micro-optimisation saves you some clock cycles when
		the rust compiler would have otherwise decided not to inline
		this function.
		*/

		match log::set_logger(&SimplestLogger { level: Level::Trace }).map(|()| log::set_max_level(level))
		{
			Ok(_) => info!("Logger Initalized"),
			Err(_) =>
			{
				println!("       {BOLD}{ERROR_COLOR}Error{RESET} Cannot initalize: there can only be one logger.");
				panic!("tried to initalize a logger where an instance already existed");
			}
		}
	}

	// The compiler will never inline functions not marked with `#[inline]`*,
	// this does not however make it inline: it lets the compiler decide based
	// on how many times it is called. If its called frequently inlining it is
	// a bad idea as it reduces cache hits, but if its called very few times
	// inlining it is a good idea as the overhead of the call may take longer
	// than the actual function itself (as tiny as that is)
	//
	// * it will if its in the main binary, but not if its a crate!

	// TODO: Performance benchmarks on this, inlining it may not help or may
	// make it slower!

	#[inline]
	pub fn set_level(level: LevelFilter) -> ()
	{
		/*!
		Clean interface to set the log level. You can also fdo this directly
		with `log::set_max_level(LevelFilter::YourLevel)`. This function has
		the inline attribute so it should cause no performance loss as opposed
		to doing it directly.
		*/
		log::set_max_level(level)
	}
}

#[doc(hidden)]
impl log::Log for SimplestLogger {
	#[cold]
	fn enabled(&self, metadata: &Metadata) -> bool
	{
		return metadata.level() <= self.level;
	}

	fn log(&self, record: &Record)
	{


		// if !self.enabled(record.metadata())
		// {
		// 	return;
		// }

		// skip function call to self.enabled(), probably saves a few clock
		// cycles since we skip an extra function call of record.metadata().
		// Old code preserved above just incase.
		if !(record.level() <= self.level)
		{
			return;
		}

		match record.level()
		{
			Level::Trace => println!("  {BOLD}Trace{RESET} {name} ({file}:{line})",
				name = record.args(),
				file = record.file().unwrap_or("unknown"),
				line = record.line().unwrap_or(0),
			),
			Level::Debug => println!("  {BOLD}{DEBUG_COLOR}Debug{RESET} {}", record.args()),
			Level::Info => println!("   {BOLD}{INFO_COLOR}Info{RESET} {}", record.args()),
			Level::Warn => println!("{BOLD}{WARNING_COLOR}Warning{RESET} {} ({file}:{line})",
				record.args(),
				file = record.file().unwrap_or("unknown"),
				line = record.line().unwrap_or(0),
			),
			Level::Error => println!("  {BOLD}{ERROR_COLOR}Error{RESET} {} ({file}:{line})",
				record.args(),
				file = record.file().unwrap_or("unknown"),
				line = record.line().unwrap_or(0),
			),
		}
	}

	fn flush(&self) {}
}


#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn change_level_with_set_level()
	{
		SimplestLogger::initalize(LevelFilter::Info);
		SimplestLogger::set_level(LevelFilter::Debug);
		todo!("Assertions!");
	}

	#[test]
	#[should_panic]
	fn panic_on_logger_exist()
	{
		SimplestLogger::initalize(LevelFilter::Info);
		SimplestLogger::initalize(LevelFilter::Debug)
	}
}