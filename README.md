# SimplestLogger

Inspired by SimpleLogger, this is a crate for ultra-simple logging. I created it because there were no crates that focused on being ultra lightweight and no-effort to use.

## Roadmap

- [x] `impl Log for SimplestLogger` so it works with the log macros.
- [ ] Unit testing
	- [ ] Errors
	- [ ] Panics (only one!)
	- [ ] Benchmarks
- [ ] Integration testing (with [`tests/`](https://doc.rust-lang.org/stable/rust-by-example/testing/integration_testing.html))
	- [ ] Errors
	- [ ] Panics (only one!)
	- [ ] Benchmarks
- [ ] Attribute feature to change log level per-function `#[log(Level)]`. (Maybe not sicne this may hurt performance)

## Performance

TODO (tl;dr: super fast!)