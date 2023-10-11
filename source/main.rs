mod lib;

use log::*;
use lib::SimplestLogger;
use lib::INDENT;

fn main()
{
	SimplestLogger::initalize(LevelFilter::Info);
	// SimplestLogger::initalize(log::LevelFilter::Trace);

	trace!("Traced");
	debug!("Bugs removed");
	info!("Informative Newspaper");
	warn!("Were gouna have a problem");
	error!("We have problems");

	SimplestLogger::set_level(LevelFilter::Trace);
	println!("---");

	trace!("Traced");
	debug!("Bugs removed");
	info!("Informative Newspaper");
	warn!("Were gouna have a problem");
	error!("We have problems");

	SimplestLogger::set_level(LevelFilter::Error);
	println!("---");
	test();

	trace!("Traced");
	debug!("Bugs removed");
	info!("Informative Newspaper");
	warn!("Were gouna have a problem");
	error!("We have problems");

	println!("-----");
	info!("Meow and meow and meow
{INDENT}and meow");
}

fn test()
{
	static mut CAT: &str = "meow";

	unsafe { CAT = "purr"; }

	SimplestLogger::set_level(LevelFilter::Info);
	trace!("Traced");
	debug!("Bugs removed");
	info!("Informative Newspaper");
	warn!("Were gouna have a problem");
	error!("We have problems");
}