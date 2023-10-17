/*******************************************************************************
 * Copyright (c) 2017 Association Cénotélie (cenotelie.fr)
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

//! Module for lexers' automata

use crate::text::{Text, Utf16C};
use crate::utils::bin::{read_table, read_u32};

/// Identifier of an invalid state in an automaton
pub const DEAD_STATE: u32 = 0xFFFF;

/// Represents the information of a terminal matched at the state of a lexer's automaton
pub struct MatchedTerminal {
    /// The context
    pub context: u16,
    /// The terminal's index
    pub index: u16,
}

/// Represents a transition in the automaton of a lexer
/// A transition is matched by a range of UTF-16 code points
/// Its target is a state in the automaton
#[derive(Copy, Clone)]
pub struct AutomatonTransition {
    /// Start of the range
    pub start: Utf16C,
    /// End of the range
    pub end: Utf16C,
    /// The transition's target
    pub target: u32,
}

impl AutomatonTransition {
    /// Get whether this transition matches the specified character
    #[must_use]
    pub fn matches(self, c: Utf16C) -> bool {
        c >= self.start && c <= self.end
    }
}

/// Represents a state in the automaton of a lexer
/// Binary data structure:
/// u16: number of matched terminals
/// u16: total number of transitions
/// u16: number of non-cached transitions
/// -- matched terminals
/// u16: context identifier
/// u16: index of the matched terminal
/// -- cache: 256 entries
/// u16: next state's index for index of the entry
/// -- transitions
/// each transition is of the form:
/// u16: start of the range
/// u16: end of the range
/// u16: next state's index
#[derive(Copy, Clone)]
pub struct AutomatonState<'a> {
    /// The automaton table
    table: &'a [u16],
    /// The offset of this state within the table
    offset: usize,
}

impl<'a> AutomatonState<'a> {
    /// Gets the number of matched terminals in this state
    #[must_use]
    pub fn get_terminals_count(&self) -> usize {
        self.table[self.offset] as usize
    }

    /// Gets the i-th matched terminal in this state
    #[must_use]
    pub fn get_terminal(&self, index: usize) -> MatchedTerminal {
        MatchedTerminal {
            context: self.table[self.offset + index * 2 + 3],
            index: self.table[self.offset + index * 2 + 4],
        }
    }

    /// Gets an iterator over all the matched terminals
    pub fn get_terminals(&self) -> impl Iterator<Item = MatchedTerminal> + '_ {
        let count = self.get_terminals_count();
        (0..count).map(|i| self.get_terminal(i))
    }

    /// Gets whether this state is a dead end (no more transition)
    #[must_use]
    pub fn is_dead_end(&self) -> bool {
        self.table[self.offset + 1] == 0
    }

    /// Gets the number of non-cached transitions in this state
    #[must_use]
    pub fn get_bulk_transitions_count(&self) -> usize {
        self.table[self.offset + 2] as usize
    }

    /// Gets the target of the cached transition for the specified value
    #[must_use]
    pub fn get_cached_transition(&self, value: Utf16C) -> u32 {
        u32::from(
            self.table[self.offset + 3 + self.table[self.offset] as usize * 2 + value as usize],
        )
    }

    /// Gets an iterator over all the cached transitions
    pub fn get_cached_transitions(&self) -> impl Iterator<Item = (u16, u32)> + '_ {
        (0..256)
            .map(|c| (c, self.get_cached_transition(c)))
            .filter(|(_, s)| *s != DEAD_STATE)
    }

    /// Gets the i-th non-cached transition in this state
    #[must_use]
    pub fn get_bulk_transition(&self, index: usize) -> AutomatonTransition {
        let offset = self.offset + 3 + self.table[self.offset] as usize * 2 + 256 + index * 3;
        AutomatonTransition {
            start: self.table[offset],
            end: self.table[offset + 1],
            target: u32::from(self.table[offset + 2]),
        }
    }

    /// Gets an iterator over all the bulk transitions
    pub fn get_bulk_transitions(&self) -> impl Iterator<Item = AutomatonTransition> + '_ {
        let count = self.get_bulk_transitions_count();
        (0..count).map(|i| self.get_bulk_transition(i))
    }

    /// Gets the target of a transition from this state on the specified value
    #[must_use]
    pub fn get_target_by(&self, value: Utf16C) -> u32 {
        if value <= 255 {
            return self.get_cached_transition(value);
        }
        for i in 0..self.get_bulk_transitions_count() {
            let transition = self.get_bulk_transition(i);
            if transition.matches(value) {
                return transition.target;
            }
        }
        DEAD_STATE
    }
}

/// Represents the automaton of a lexer
/// Binary data structure of lexers:
/// u32: number of entries in the states index table
/// -- states offset table
/// each entry is of the form:
/// u32: offset of the state from the beginning of the states table in number of u16
/// -- states table
#[derive(Clone, Default)]
pub struct Automaton<'a> {
    /// Table of indices in the states table
    table: &'a [u32],
    /// Lexer's DFA table of states
    states: &'a [u16],
    /// The number of states in the automaton
    states_count: usize,
}

impl<'a> Automaton<'a> {
    /// Initializes a new automaton from the given binary data
    #[must_use]
    pub fn new(data: &'a [u8]) -> Automaton<'a> {
        let states_count = read_u32(data, 0) as usize;
        let table = read_table(&data[4..], states_count).unwrap();
        let rest = (data.len() - 4 - states_count * 4) / 2;
        let states = read_table(&data[4 + states_count * 4..], rest).unwrap();
        Automaton {
            table,
            states,
            states_count,
        }
    }

    /// Gets the number of states in the automaton
    #[must_use]
    pub fn get_states_count(&self) -> usize {
        self.states_count
    }

    /// Get the data of the specified state
    #[must_use]
    pub fn get_state(&self, state: u32) -> AutomatonState {
        AutomatonState {
            table: &self.states,
            offset: self.table[state as usize] as usize,
        }
    }

    /// Gets an iterator over the states
    pub fn get_states(&self) -> impl Iterator<Item = AutomatonState<'_>> + '_ {
        (0..self.states_count).map(|i| self.get_state(i as u32))
    }
}

/// Represents a match in the input
pub struct TokenMatch {
    /// The matching DFA state
    pub state: u32,
    /// Length of the matched input
    pub length: u32,
}

/// Runs the lexer's DFA to match a terminal in the input ahead
#[must_use]
pub fn run_dfa(automaton: &Automaton, input: &Text, index: usize) -> Option<TokenMatch> {
    if input.is_end(index) {
        return Some(TokenMatch {
            state: 0,
            length: 0,
        });
    }

    let mut result = None;
    let mut state = 0;
    let mut position = index;
    let mut input_iter = input.iter_utf16_from(index);

    while state != DEAD_STATE {
        let state_data = automaton.get_state(state);
        // Is this state a matching state ?
        if state_data.get_terminals_count() > 0 {
            result = Some(TokenMatch {
                state,
                length: (position - index) as u32,
            });
        }
        // No further transition => exit
        if state_data.is_dead_end() {
            break;
        }
        match input_iter.next() {
            None => {
                // at end
                break;
            }
            Some((current, l)) => {
                position += l;
                state = state_data.get_target_by(current);
            }
        }
    }
    result
}
