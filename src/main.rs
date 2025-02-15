use clap::Parser;
use std::fmt::{Debug, Display};
use std::io::{BufReader, BufWriter, Read, Write};

//use base64::decode::{DecodeConfig, decode_bytes_with};
//use base64::encode::{EncodeConfig, encode_to_bytes_with};
use base64::engine::{
    Engine,
    general_purpose::{
        STANDARD,
        STANDARD_NO_PAD,
        URL_SAFE,
        URL_SAFE_NO_PAD,
    },
};

mod cli;
use cli::IOArgs;

fn main() {
    let args = cli::CliArgs::parse();

    match args.command {
        cli::EncodeOption::Encode(ioargs) => {
            let engine = select_engine(&ioargs);
            encode(engine, &ioargs);
        }
        cli::EncodeOption::Decode(ioargs) => {
            let engine = select_engine(&ioargs);
            decode(engine, &ioargs);
        }
    }
}

fn select_engine(args: &IOArgs) -> impl Engine {
    match args {
        IOArgs{ urlsafe:false, no_padding:false, .. } => STANDARD,
        IOArgs{ urlsafe:false, no_padding:true , .. } => STANDARD_NO_PAD,
        IOArgs{ urlsafe:true , no_padding:false, .. } => URL_SAFE,
        IOArgs{ urlsafe:true , no_padding:true , .. } => URL_SAFE_NO_PAD,
    }
}

fn expect<T, E: Debug>(result: Result<T, E>, msg: impl Display) -> T {
    result.unwrap_or_else(|e| {
        eprintln!("error: {}", msg);
        panic!("error: {}: {:?}", msg, e);
    })
}

fn encode(engine: impl Engine, ioargs: &IOArgs) {
    let infile = BufReader::new(ioargs.r#in.clone());
    let mut outfile = BufWriter::new(ioargs.out.clone());

    expect(outfile.write_all(
        engine.encode(
            infile.bytes().map(|b| expect(b, "could not read input")).collect::<Vec<u8>>(),
        ).as_bytes()
    ), "could not write to output");
}

fn decode(engine: impl Engine, ioargs: &IOArgs) {
    let infile = BufReader::new(ioargs.r#in.clone());
    let mut outfile = BufWriter::new(ioargs.out.clone());
    expect(outfile.write_all(
        expect(engine.decode(
            infile.bytes().map(|b| expect(b, "could not read input")).collect::<Vec<u8>>(),
        ), "Invalid base64 format").as_ref()
    ), "could not write to output");
}
