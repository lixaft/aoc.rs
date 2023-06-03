use std::fs;
use std::io;

pub struct Solver {
    file: String,
    callback: fn(String) -> u64,
}

pub struct Solvers {
    pub part_one: Solver,
    pub part_two: Solver,
}

impl Solver {
    pub fn new<S: Into<String>>(file: S, callback: fn(String) -> u64) -> Solver {
        Solver {
            file: file.into(),
            callback,
        }
    }

    pub fn call(&self) -> Result<u64, io::Error> {
        let content = fs::read_to_string(&self.file)?;
        Ok((self.callback)(content))
    }
}

impl Solvers {
    pub fn new(part_one: Solver, part_two: Solver) -> Solvers {
        Solvers { part_one, part_two }
    }
}
