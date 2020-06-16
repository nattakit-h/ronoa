/*
 * Copyright (c) 2020 Nattakit Hosapsin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use libronoa::lexer;
use std::{
    env,
    fs,
};

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("incorrect arguments");
    }

    let filepath = &args[1];
    let contents = fs::read_to_string(filepath).expect("unable to read file");
    let mut lexer = lexer::Lexer::new();
    lexer
        .tokens_from(contents)
        .iter()
        .for_each(|token| println!("{}", token));
}
