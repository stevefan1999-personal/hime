/*******************************************************************************
 * Copyright (c) 2020 Association Cénotélie (cenotelie.fr)
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General
 * Public License along with this program.
 * If not, see <http://www.gnu.org/licenses/>.
 ******************************************************************************/

//! Module for generating lexer code in Rust

use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::errors::Error;
use crate::grammars::{Grammar, TerminalRef, TerminalSet, PREFIX_GENERATED_TERMINAL};
use crate::output::get_lexer_bin_name_rust;
use crate::output::helper::{to_upper_camel_case, to_upper_case};
use crate::CRATE_VERSION;

/// Generates code for the specified file
#[allow(
    clippy::too_many_lines,
    clippy::too_many_arguments,
    clippy::fn_params_excessive_bools
)]
pub fn write(
    path: Option<&String>,
    file_name: String,
    grammar: &Grammar,
    expected: &TerminalSet,
    separator: Option<TerminalRef>,
    is_rnglr: bool,
    with_std: bool,
    suppress_module_doc: bool,
    compress_automata: bool,
) -> Result<(), Error> {
    let mut final_path = PathBuf::new();
    if let Some(path) = path {
        final_path.push(path);
    }
    final_path.push(file_name);
    let file = File::create(final_path)?;
    let mut writer = io::BufWriter::new(file);

    let name = to_upper_camel_case(&grammar.name);
    let base_lexer = if grammar.contexts.len() > 1 {
        "ContextSensitive"
    } else {
        "ContextFree"
    };
    let bin_name = get_lexer_bin_name_rust(grammar);
    let separator = match separator {
        None => 0xFFFF,
        Some(terminal_ref) => terminal_ref.sid(),
    };

    if !suppress_module_doc {
        writeln!(
            writer,
            "//! Module for the lexer and parser for `{}`",
            &name
        )?;
        writeln!(writer, "//! WARNING: this file has been generated by")?;
        writeln!(writer, "//! Hime Parser Generator {CRATE_VERSION}")?;
        writeln!(writer)?;
    }

    if !with_std {
        writeln!(writer, "use alloc::string::String;")?;
    }

    writeln!(writer, "use hime_redist::ast::{{AstImpl, AstNode}};")?;
    writeln!(writer, "use hime_redist::errors::ParseErrors;")?;
    writeln!(writer, "use hime_redist::lexers::automaton::Automaton;")?;
    writeln!(writer, "use hime_redist::lexers::impls::{base_lexer}Lexer;")?;
    writeln!(writer, "use hime_redist::lexers::Lexer;")?;
    if is_rnglr {
        writeln!(writer, "use hime_redist::parsers::rnglr::RNGLRAutomaton;")?;
        writeln!(writer, "use hime_redist::parsers::rnglr::RNGLRParser;")?;
    } else {
        writeln!(writer, "use hime_redist::parsers::lrk::LRkAutomaton;")?;
        writeln!(writer, "use hime_redist::parsers::lrk::LRkParser;")?;
    }
    writeln!(writer, "use hime_redist::parsers::Parser;")?;
    writeln!(
        writer,
        "use hime_redist::result::{{ParseResult, ParseResultAst{}}};",
        if is_rnglr { ", ParseResultSppf" } else { "" }
    )?;
    if is_rnglr {
        writeln!(writer, "use hime_redist::sppf::{{SppfImpl, SppfNode, SppfNodeVersion, SppfNodeVersions}};")?;
        writeln!(writer, "use hime_redist::dyn_clone::{{DynClone, clone_box}};")?;
    }
    writeln!(writer, "use hime_redist::symbols::SemanticBody;")?;
    writeln!(writer, "use hime_redist::symbols::SemanticElementTrait;")?;
    writeln!(writer, "use hime_redist::symbols::Symbol;")?;
    writeln!(writer, "use hime_redist::text::Text;")?;
    writeln!(writer, "use hime_redist::tokens::TokenRepository;")?;
    writeln!(writer)?;

    if compress_automata {
        writeln!(
            writer,
            r#"include_flate::flate!(static LEXER_AUTOMATON: [u8] from "{bin_name}" on "OUT_DIR");"#
        )?;
    } else {
        writeln!(
            writer,
            "/// Static resource for the serialized lexer automaton"
        )?;
        writeln!(
            writer,
            "static LEXER_AUTOMATON: &[u8] = include_bytes!(\"{bin_name}\");"
        )?;
    }
    writeln!(writer)?;

    for terminal_ref in expected.content.iter().skip(2) {
        let terminal = grammar.get_terminal(terminal_ref.sid()).unwrap();
        if terminal.name.starts_with(PREFIX_GENERATED_TERMINAL) {
            continue;
        }
        writeln!(
            writer,
            "/// The unique identifier for terminal `{}`",
            &terminal.name
        )?;
        writeln!(
            writer,
            "pub const ID_TERMINAL_{}: u32 = 0x{:04X};",
            to_upper_case(&terminal.name),
            terminal.id
        )?;
    }
    writeln!(writer)?;

    writeln!(writer, "/// The unique identifier for the default context")?;
    writeln!(writer, "pub const CONTEXT_DEFAULT: u16 = 0;")?;
    for (index, context) in grammar.contexts.iter().enumerate().skip(1) {
        writeln!(writer, "/// The unique identifier for context {context}")?;
        writeln!(
            writer,
            "pub const CONTEXT_{}: u16 = 0x{:04X};",
            to_upper_case(context),
            index
        )?;
    }
    writeln!(writer)?;

    writeln!(
        writer,
        "/// The collection of terminals matched by this lexer"
    )?;
    writeln!(
        writer,
        "/// The terminals are in an order consistent with the automaton,"
    )?;
    writeln!(writer, "/// so that terminal indices in the automaton can be used to retrieve the terminals in this table")?;
    writeln!(writer, "pub const TERMINALS: &[Symbol] = &[")?;
    writeln!(writer, "    Symbol {{")?;
    writeln!(writer, "        id: 0x0001,")?;
    writeln!(writer, "        name: \"ε\"")?;
    writeln!(writer, "    }},")?;
    writeln!(writer, "    Symbol {{")?;
    writeln!(writer, "        id: 0x0002,")?;
    writeln!(writer, "        name: \"$\"")?;
    write!(writer, "    }}")?;
    for terminal_ref in expected.content.iter().skip(2) {
        let terminal = grammar.get_terminal(terminal_ref.sid()).unwrap();
        writeln!(writer, ",")?;
        writeln!(writer, "    Symbol {{")?;
        writeln!(writer, "        id: 0x{:04X},", terminal.id)?;
        writeln!(
            writer,
            "        name: \"{}\"",
            terminal.value.replace('"', "\\\"")
        )?;
        write!(writer, "    }}")?;
    }
    writeln!(writer)?;
    writeln!(writer, "];")?;
    writeln!(writer)?;

    writeln!(writer, "/// Creates a new lexer")?;
    writeln!(writer, "fn new_lexer<'a: 'b, 'b, 'c>(")?;
    writeln!(writer, "    repository: TokenRepository<'a, 'b, 'c>,")?;
    writeln!(writer, "    errors: &'c mut ParseErrors<'a>")?;
    writeln!(writer, ") -> Lexer<'a, 'b, 'c> {{")?;
    writeln!(
        writer,
        "    let automaton = Automaton::new(LEXER_AUTOMATON{});",
        if compress_automata { ".as_ref()" } else { "" }
    )?;
    writeln!(
        writer,
        "    Lexer::{base_lexer}({base_lexer}Lexer::new(repository, errors, automaton, 0x{separator:04X}))"
    )?;
    writeln!(writer, "}}")?;
    writeln!(writer)?;
    Ok(())
}
