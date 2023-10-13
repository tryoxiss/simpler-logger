// #![feature(test)]
mod lib;

use log::*;
use lib::SimplestLogger;
use lib::INDENT;

// extern crate test;
// use test;

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

	SimplestLogger::set_level(LevelFilter::Trace);

	trace!("finds exact locations, should always be ommitted from final \n{INDENT}binaries. With out logger, automatically includes file \n{INDENT}path and line number.");
	debug!("Debug message, should never appear in a final binary. No location \n{INDENT}included, use trace for that.");
	info!("Confirmations that the program is working as expected. \n{INDENT}No location needed.");
	warn!("Something might go wrong or should be brought to your attention. \n{INDENT}Location included.");
	error!("Something went wrong, but could be reovered from. \n{INDENT}Location included.");
}

fn test()
{
	SimplestLogger::set_level(LevelFilter::Info);
	trace!("Traced");
	debug!("Bugs removed");
	info!("Informative Newspaper");
	warn!("Were gouna have a problem");
	error!("We have problems");
}