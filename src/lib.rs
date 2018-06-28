#![feature(catch_expr)]
#![feature(crate_in_paths)]
#![feature(crate_visibility_modifier)]
#![feature(proc_macro)]
#![feature(in_band_lifetimes)]
#![feature(termination_trait_test)]
#![allow(dead_code)]

extern crate datafrog;
extern crate failure;
extern crate histo;
extern crate polonius_engine;
extern crate polonius_parser;
extern crate rustc_hash;
extern crate structopt;
extern crate clap;

mod dump;
mod facts;
pub mod intern;
mod program;
pub mod tab_delim;
mod test;

pub mod cli;
