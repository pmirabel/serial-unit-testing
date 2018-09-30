/*
 * File: src/commnds.rs
 * Date: 30.09.2018
 * Auhtor: MarkAtk
 * 
 * MIT License
 * 
 * Copyright (c) 2018 MarkAtk
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
 * of the Software, and to permit persons to whom the Software is furnished to do
 * so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::time::Duration;

use clap::{Arg, ArgMatches};
use serialport::{self, SerialPortSettings};

use utils;

pub fn serial_arguments<'a>() -> Vec<Arg<'a, 'a>> {
    let databits = [ "5", "6", "7", "8" ];
    let parity = [ "none", "even", "odd" ];
    let stopbits = [ "1", "2" ];
    let flowcontrols = [ "none", "software", "hardware" ];

    vec![Arg::with_name("port")
            .long("port")
            .short("p")
            .help("Serial port OS specific name")
            .required(true)
            .value_name("PORT")
            .takes_value(true),
        Arg::with_name("baud")
            .long("baud")
            .short("b")
            .help("Serial port baud rate")
            .takes_value(true)
            .default_value("9600"),
        Arg::with_name("databits")
            .long("databits")
            .short("d")
            .help("Serial port number of data bits")
            .takes_value(true)
            .possible_values(&databits)
            .default_value("8"),
        Arg::with_name("parity")
            .long("parity")
            .short("P")
            .help("Serial port parity")
            .takes_value(true)
            .possible_values(&parity)
            .default_value("none"),
        Arg::with_name("stopbits")
            .long("stopbits")
            .short("s")
            .help("Serial port stop bits")
            .takes_value(true)
            .possible_values(&stopbits)
            .default_value("1"),
        Arg::with_name("flowcontrol")
            .long("flowcontrol")
            .short("f")
            .help("Serial port flow control mode")
            .possible_values(&flowcontrols)
            .default_value("none"),
        Arg::with_name("timeout")
            .long("timeout")
            .short("t")
            .help("Set serial port timeout duration")
            .takes_value(true)
            .default_value("1000"),
        Arg::with_name("hex")
            .long("hex")
            .short("H")
            .help("Set hexadecimal mode"),
        Arg::with_name("binary")
            .long("binary")
            .short("B")
            .help("Set binary mode")
            .conflicts_with("hex")
    ]
}

pub fn text_input_arguments<'a>() -> Vec<Arg<'a, 'a>> {
    vec![Arg::with_name("carriagereturn")
            .long("carriage-return")
            .short("R")
            .help("Add carriage return at the end"),
        Arg::with_name("newline")
            .long("newline")
            .short("N")
            .help("Add newline at the end"),
        Arg::with_name("escape")
            .long("escape")
            .short("E")
            .help("Enable input string escaping"),
        Arg::with_name("hexinput")
            .long("hex-in")
            .help("Set hexadecimal input mode"),
        Arg::with_name("binaryinput")
            .long("binary-in")
            .help("Set binary input mode")
            .conflicts_with("hexinput")
    ]
}

pub fn text_output_arguments<'a>() -> Vec<Arg<'a, 'a>> {
    vec![Arg::with_name("hexoutput")
            .long("hex-out")
            .help("Set hexadecimal output mode"),
        Arg::with_name("binaryoutput")
            .long("binary-out")
            .help("Set binary output mode")
            .conflicts_with("hexoutput")
    ]
}

pub fn get_serial_port_settings<'a>(matches: &'a ArgMatches) -> Result<(SerialPortSettings, &'a str), String> {
    let mut settings: SerialPortSettings = Default::default();

    let port_name = matches.value_of("port").unwrap();
    let baud_rate = matches.value_of("baud").unwrap();
    let timeout = matches.value_of("timeout").unwrap();
    let data_bits = matches.value_of("databits").unwrap();
    let parity = matches.value_of("parity").unwrap();
    let stop_bits = matches.value_of("stopbits").unwrap();
    let flow_control = matches.value_of("flowcontrol").unwrap();

    if let Ok(rate) = baud_rate.parse::<u32>() {
        settings.baud_rate = rate.into();
    } else {
        return Err(format!("Invalid baud rate '{}'", baud_rate));
    }

    if let Ok(duration) = timeout.parse::<u64>() {
        settings.timeout = Duration::from_millis(duration);
    } else {
        return Err(format!("Invalid timeout '{}'", timeout));
    }

    settings.data_bits = match data_bits {
        "5" => serialport::DataBits::Five,
        "6" => serialport::DataBits::Six,
        "7" => serialport::DataBits::Seven,
        "8" => serialport::DataBits::Eight,
        _ => {
            return Err(format!("Invalid data bits '{}'", data_bits));
        }
    };

    settings.parity = match parity {
        "none" => serialport::Parity::None,
        "even" => serialport::Parity::Even,
        "odd" => serialport::Parity::Odd,
        _ => {
            return Err(format!("Invalid parity '{}'", parity));
        }
    };

    settings.stop_bits = match stop_bits {
        "1" => serialport::StopBits::One,
        "2" => serialport::StopBits::Two,
        _ => {
            return Err(format!("Invalid stop bits '{}", stop_bits));
        }
    };

    settings.flow_control = match flow_control {
        "none" => serialport::FlowControl::None,
        "software" => serialport::FlowControl::Software,
        "hardware" => serialport::FlowControl::Hardware,
        _ => {
            return Err(format!("Invalid flow control '{}'", flow_control));
        }
    };

    Ok((settings, port_name))
}

pub fn get_text_format(matches: &ArgMatches) -> utils::TextFormat {
    if matches.is_present("binary") {
        return utils::TextFormat::Binary;
    } else if matches.is_present("octal") {
        return utils::TextFormat::Octal;
    } else if matches.is_present("decimal") {
        return utils::TextFormat::Decimal;
    } else if matches.is_present("hex") {
        return utils::TextFormat::Hex;
    }

    utils::TextFormat::Text
}

pub fn get_text_input_format(matches: &ArgMatches) -> utils::TextFormat {
    if matches.is_present("binaryinput") {
        return utils::TextFormat::Binary;
    } else if matches.is_present("octalinput") {
        return utils::TextFormat::Octal;
    } else if matches.is_present("decimalinput") {
        return utils::TextFormat::Decimal;
    } else if matches.is_present("hexinput") {
        return utils::TextFormat::Hex;
    }

    get_text_format(matches)
}

pub fn get_text_output_format(matches: &ArgMatches) -> utils::TextFormat {
    if matches.is_present("binaryoutput") {
        return utils::TextFormat::Binary;
    } else if matches.is_present("octaloutput") {
        return utils::TextFormat::Octal;
    } else if matches.is_present("decimaloutput") {
        return utils::TextFormat::Decimal;
    } else if matches.is_present("hexoutput") {
        return utils::TextFormat::Hex;
    }

    get_text_format(matches)
}