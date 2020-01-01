use crate::errors::*;
use crate::ruleset::*;

use std::path::Path;

struct File {
    rules: RuleSet
}

/// Given a single specific gitignore style file, allow matching against
/// the rules within that file.
impl File {
    fn new<P: AsRef<Path>>(_path: P) -> Result<File> {
        unimplemented!();
    }
}

//
