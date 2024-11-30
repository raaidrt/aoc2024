use clap::{Parser, ValueEnum};
use std::{error::Error, fmt};

#[derive(Clone, ValueEnum, Parser, Debug)]
#[clap(rename_all = "kebab_case")]
pub enum Stage {
    A,
    B,
}

#[derive(Debug)]
pub struct StageUnimplemented(pub Stage);

impl fmt::Display for StageUnimplemented {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stage not implemented: {:?}", self.0)
    }
}

impl Error for StageUnimplemented {}
