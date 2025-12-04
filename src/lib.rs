use nom::{error::Error, Err as NomErr};

mod day01;
mod day02;
mod day03;
mod day04;

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

aoc_lib! { year = 2025 }

pub mod grid;

pub type IResult<I, O, E = nom::error::Error<I>> = Result<(I, O), nom::Err<E>>;

pub fn convert_iresult_to_owned<O>(res: IResult<&str, O>) -> Result<O, NomErr<Error<String>>> {
    match res {
        Ok((_, o)) => Ok(o),
        Err(e) => Err(convert_error_to_owned(e)),
    }
}

fn convert_error_to_owned(e: NomErr<Error<&str>>) -> NomErr<Error<String>> {
    match e {
        NomErr::Incomplete(needed) => NomErr::Incomplete(needed),
        NomErr::Error(err) => NomErr::Error(Error {
            input: err.input.to_owned(),
            code: err.code,
        }),
        NomErr::Failure(err) => NomErr::Failure(Error {
            input: err.input.to_owned(),
            code: err.code,
        }),
    }
}
