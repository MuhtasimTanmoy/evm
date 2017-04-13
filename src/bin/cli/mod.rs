#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate sputnikvm;

use sputnikvm::Gas;
use sputnikvm::vm::{Machine, FakeVectorMachine};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use log::LogLevel;

pub fn read_hex(s: &str) -> Option<Vec<u8>> {
    let mut res = Vec::<u8>::new();

    let mut cur = 0;
    let mut len = 0;
    for c in s.chars() {
        len += 1;
        let v_option = c.to_digit(16);
        if v_option.is_none() {
            return None;
        }
        let v = v_option.unwrap();
        if len == 1 {
            cur += v * 16;
        } else { // len == 2
            cur += v;
        }
        if len == 2 {
            res.push(cur as u8);
            cur = 0;
            len = 0;
        }
    }

    return Some(res);
}

fn main() {
    let matches = clap_app!(svm =>
        (version: "0.1.0")
        (author: "SputnikVM Contributors")
        (about: "SputnikVM - Ethereum Classic Virtual Machine")
        (@arg GAS: -g --gas +takes_value +required "Sets the gas amount")
        (@arg DATA: -d --data +takes_value +required "Sets the data needed")
        (@arg CODE: -c --code +takes_value +required "Sets the path to a file which contains the vm byte code")
        (@arg STATS: -s --stats "Return statistics on the execution")
        (@arg debug: -D --debug ... "Sets the level of debugging information")
    ).get_matches();

    let code_hex = read_hex(match matches.value_of("CODE") {
        Some(c) => c,
        None => "",
    });

    let code = code_hex.expect("code must be provided");

    let initial_gas = (value_t!(matches, "GAS", isize).unwrap_or(0xff)).into();
    let data = match matches.value_of("DATA") {
        Some(d) => d.as_bytes().into(),
        None => "".as_bytes().into(),
    };

    let mut machine = FakeVectorMachine::new(code.as_slice(), data, initial_gas);
    machine.fire();
    if log_enabled!(LogLevel::Info) {
        info!("gas used: {:?}", machine.used_gas());
    }
}