pub mod exec;
pub mod parse;

#[cfg(test)]
pub mod test_utils;

// TODO after parsing make sure everything has a valid state (e.g. all methods
// named, all classes named, etc instead of relying on unreachable!()s at
// runtime)
