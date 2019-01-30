/*
 * File: parser/error.rs
 * Date: 02.10.2018
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

use std::fmt;
use std::error::Error as StdError;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownError(u32, u32),
    ReadFileError,
    IllegalToken(String, u32, u32),
    MissingClosingParenthesis(String, u32, u32),
    MissingDirectionSeparator(u32, u32),
    MissingGroupIdentifier(u32, u32)
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnknownError(line, column) => formatter.write_fmt(format_args!("Unknown error at {}:{}", line, column)),
            Error::ReadFileError => formatter.write_str("Unable to read file"),
            Error::IllegalToken(ref value, line, column) => formatter.write_fmt(format_args!("Illegal token {} at {}:{}", value, line, column)),
            Error::MissingClosingParenthesis(ref value, line, column) => formatter.write_fmt(format_args!("Missing closing parenthesis {} at {}:{}", value, line, column)),
            Error::MissingDirectionSeparator(line, column) => formatter.write_fmt(format_args!("Missing direction separator at {}:{}", line, column)),
            Error::MissingGroupIdentifier(line, column) => formatter.write_fmt(format_args!("Missing group identifier at {}:{}", line, column))
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnknownError(_, _) => "Unknown error",
            Error::ReadFileError => "File file error",
            Error::IllegalToken(_, _, _) => "Illegal token",
            Error::MissingClosingParenthesis(_, _, _) => "Missing closing parenthesis",
            Error::MissingDirectionSeparator(_, _) => "Missing direction separator",
            Error::MissingGroupIdentifier(_, _) => "Missing group identifier"
        }
    }
}
