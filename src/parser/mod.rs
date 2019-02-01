/*
 * File: src/parser/mod.rs
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

use std::fs;
use std::io::{BufReader, Read};
use std::time::Duration;

use tests::{TestCase, TestSuite, TestCaseSettings};
use utils::TextFormat;

mod error;
mod token;
mod string_util;
mod char_util;
mod lexer;
mod finite_state_maschine;

use self::lexer::Lexer;
use self::token::{Token, TokenType};
use self::error::Error;
use self::finite_state_maschine::FiniteStateMachine;

pub fn parse_file(file: &mut fs::File) -> Result<Vec<TestSuite>, Error> {
    let mut reader = BufReader::new(file);
    let mut content = String::new();

    if let Err(_) = reader.read_to_string(&mut content) {
        return Err(Error::ReadFileError);
    }

    let mut lexer = Lexer::new(content);
    let tokens = lexer.get_tokens();

    analyse_tokens(tokens)
}

fn analyse_tokens(tokens: Vec<Token>) -> Result<Vec<TestSuite>, Error> {
    let mut lines: Vec<Vec<Token>> = Vec::new();
    let mut line: Vec<Token> = Vec::new();

    // split token stream into lines
    for token in tokens {
        if token.token_type == TokenType::Illegal {
            return Err(Error::IllegalToken(token.value, token.line, token.column));
        }

        if token.token_type == TokenType::Newline {
            // only add line if not empty
            if line.len() > 0 {
                lines.push(line);

                line = Vec::new();
            }

            continue;
        }

        line.push(token);
    }

    // analyse each line
    let mut test_suites: Vec<TestSuite> = Vec::new();

    // [ Identifier ]
    let group_state_machine = FiniteStateMachine::new(1, vec!(4), |state, token| -> u32 {
        match state {
            1 if token.token_type == TokenType::LeftGroupParenthesis => 2,
            2 if token.token_type == TokenType::Identifier => 3,
            3 if token.token_type == TokenType::RightGroupParenthesis => 4,
            _ => 0
        }
    });

    // <> mark optional tokens
    // / mark alternative tokens
    // <( Identifier )> <b/o/d/h>" Content " : <b/o/d/h>" Content "
    let test_state_machine = FiniteStateMachine::new(1, vec!(9), |state, token| -> u32 {
        match state {
            1 if token.token_type == TokenType::LeftTestParenthesis => 2,
            1 if token.token_type == TokenType::FormatSpecifier => 5,
            1 if token.token_type == TokenType::Content => 5,
            2 if token.token_type == TokenType::Identifier => 3,
            3 if token.token_type == TokenType::RightTestParenthesis => 4,
            3 if token.token_type == TokenType::ContentSeparator => 10,
            4 if token.token_type == TokenType::FormatSpecifier => 5,
            4 if token.token_type == TokenType::Content => 6,
            5 if token.token_type == TokenType::Content => 6,
            6 if token.token_type == TokenType::DirectionSeparator => 7,
            7 if token.token_type == TokenType::FormatSpecifier => 8,
            7 if token.token_type == TokenType::Content => 9,
            8 if token.token_type == TokenType::Content => 9,
            10 if token.token_type == TokenType::Identifier => 11,
            11 if token.token_type == TokenType::OptionSeparator => 12,
            12 if token.token_type == TokenType::Identifier => 13,
            13 if token.token_type == TokenType::RightTestParenthesis => 4,
            13 if token.token_type == TokenType::ContentSeparator => 10,
            _ => 0
        }
    });

    for line in lines {
        let first_token: &Token = line.first().unwrap();

        if first_token.token_type == TokenType::LeftGroupParenthesis {
            match analyse_test_group(&line, &group_state_machine) {
                Ok(test_suite) => test_suites.push(test_suite),
                Err(err) => return Err(err)
            };

            continue;
        }

        if first_token.token_type == TokenType::LeftTestParenthesis || first_token.token_type == TokenType::FormatSpecifier || first_token.token_type == TokenType::Content {
            match analyse_test(&line, &test_state_machine) {
                Ok(test) => {
                    if test_suites.is_empty() {
                        test_suites.push(TestSuite::new(String::new()));
                    }

                    let mut test_suite: &mut TestSuite = test_suites.last_mut().unwrap();
                    test_suite.push(test);
                }
                Err(err) => return Err(err)
            };

            continue;
        }

        return Err(Error::InvalidLineStart(first_token.line, first_token.column));
    }

    Ok(test_suites)
}

fn analyse_test_group(tokens: &Vec<Token>, state_machine: &FiniteStateMachine) -> Result<TestSuite, Error> {
    let result = state_machine.run(&tokens);

    if let Err((state, token)) = result {
        return match state {
            2 => Err(Error::MissingGroupIdentifier(token.line, token.column)),
            3 => Err(Error::MissingClosingParenthesis("]".to_string(), token.line, token.column)),
            _ => Err(Error::UnknownError(token.line, token.column))
        };
    }

    let name = &tokens[1].value;

    Ok(TestSuite::new(name.to_string()))
}

fn analyse_test(tokens: &Vec<Token>, state_machine: &FiniteStateMachine) -> Result<TestCase, Error> {
    let result = state_machine.run(&tokens);

    if let Err((state, token)) = result {
        return match state {
            2 => Err(Error::MissingTestIdentifier(token.line, token.column)),
            3 => Err(Error::MissingClosingParenthesis(")".to_string(), token.line, token.column)),
            4 | 5 => Err(Error::MissingContent("input".to_string(), token.line, token.column)),
            6 => Err(Error::MissingDirectionSeparator(token.line, token.column)),
            7 | 8 => Err(Error::MissingContent("output".to_string(), token.line, token.column)),
            _ => Err(Error::UnknownError(token.line, token.column))
        };
    }

    // create test case
    let mut name = String::new();
    let input: String;
    let output: String;
    let mut settings = TestCaseSettings::default();

    let mut index = 0;

    if tokens[index].token_type == TokenType::LeftTestParenthesis {
        name = tokens[index + 1].value.clone();
        index += 2;

        while tokens[index].token_type == TokenType::ContentSeparator {
            set_test_option(&tokens[index + 1 .. index + 4], &mut settings)?;

            index += 4;
        }

        index += 1;
    }

    if tokens[index].token_type == TokenType::FormatSpecifier {
        settings.input_format = get_text_format(&tokens[index])?;
        index += 1;
    }

    input = tokens[index].value.clone();

    // skip direction separator
    index += 2;

    if tokens[index].token_type == TokenType::FormatSpecifier {
        settings.output_format = get_text_format(&tokens[index])?;
        index += 1;
    }

    output = tokens[index].value.clone();

    Ok(TestCase::new_with_settings(name, input, output, settings))
}

fn get_text_format(token: &Token) -> Result<TextFormat, Error> {
    match token.value.as_str() {
        "b" => Ok(TextFormat::Binary),
        "o" => Ok(TextFormat::Octal),
        "d" => Ok(TextFormat::Decimal),
        "h" => Ok(TextFormat::Hex),
        _ => Err(Error::UnknownError(token.line, token.column))
    }
}

fn set_test_option(tokens: &[Token], settings: &mut TestCaseSettings) -> Result<(), Error> {
    let value = tokens[2].value.clone();
    let name = tokens[0].value.trim();

    match name {
        "ignore-case" => {
            if let Some(bool_val) = string_util::get_boolean_value(&value) {
                settings.ignore_case = bool_val;

                Ok(())
            } else {
                Err(Error::InvalidOptionValue("boolean".to_string(), tokens[2].line, tokens[2].column))
            }
        },
        "delay" => {
            if let Some(time) = string_util::get_time_value(&value) {
                settings.delay = Some(time);

                Ok(())
            } else {
                Err(Error::InvalidOptionValue("time".to_string(), tokens[2].line, tokens[2].column))
            }
        },
        "timeout" => {
            if let Some(time) = string_util::get_time_value(&value) {
                settings.timeout = time;

                Ok(())
            } else {
                Err(Error::InvalidOptionValue("time".to_string(), tokens[2].line, tokens[2].column))
            }
        },
        "repeat" => {
            if let Ok(count) = value.parse::<u32>() {
                settings.repeat = count;

                Ok(())
            } else {
                Err(Error::InvalidOptionValue("number".to_string(), tokens[2].line, tokens[2].column))
            }
        },
        _ => Err(Error::UnknownTestOption(name.to_string(), tokens[0].line, tokens[0].column))
    }
}
