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

//! Module for LR(k) parsers

use alloc::string::ToString;
use alloc::vec::Vec;

use super::subtree::SubTree;
use super::{
    get_op_code_base, get_op_code_tree_action, read_table, read_u16, ContextProvider, LRAction,
    LRActionCode, LRColumnMap, LRContexts, LRExpected, LRProduction, Parser, Symbol, TreeAction,
    LR_ACTION_CODE_ACCEPT, LR_ACTION_CODE_NONE, LR_ACTION_CODE_REDUCE, LR_ACTION_CODE_SHIFT,
    LR_OP_CODE_BASE_ADD_VIRTUAL, LR_OP_CODE_BASE_SEMANTIC_ACTION, TREE_ACTION_DROP,
    TREE_ACTION_NONE, TREE_ACTION_PROMOTE, TREE_ACTION_REPLACE_BY_CHILDREN,
    TREE_ACTION_REPLACE_BY_EPSILON,
};
use crate::ast::{AstImpl, TableElemRef, TableType};
use crate::errors::ParseErrorUnexpectedToken;
use crate::lexers::{Lexer, TokenKernel, DEFAULT_CONTEXT};
use crate::symbols::{SemanticBody, SemanticElement, SemanticElementTrait};

/// Represents the LR(k) parsing table and productions
#[derive(Clone)]
pub struct LRkAutomaton<'a> {
    /// The number of columns in the LR table
    columns_count: usize,
    /// The number of states in the LR table
    states_count: usize,
    /// Map of symbol ID to column index in the LR table
    columns_map: LRColumnMap,
    /// The contexts information
    contexts: Vec<LRContexts>,
    /// The LR table
    table: &'a [u16],
    /// The table of LR productions
    productions: Vec<LRProduction<'a>>,
}

impl<'a> LRkAutomaton<'a> {
    /// Initializes a new automaton from the given binary data
    #[must_use]
    pub fn new(data: &'a [u8]) -> LRkAutomaton<'a> {
        let columns_count = read_u16(data, 0) as usize;
        let states_count = read_u16(data, 2) as usize;
        let productions_count = read_u16(data, 4) as usize;
        let columns_map = LRColumnMap::new(data, 6, columns_count);
        let mut contexts = Vec::with_capacity(states_count);
        let mut index = 6 + columns_count * 2;
        for _i in 0..states_count {
            let mut context = LRContexts::new();
            let count = read_u16(data, index);
            index += 2;
            for _j in 0..count {
                context.add(read_u16(data, index), read_u16(data, index + 2));
                index += 4;
            }
            contexts.push(context);
        }
        let table = read_table(&data[index..], states_count * columns_count * 2).unwrap();
        index += states_count * columns_count * 4;
        let mut productions = Vec::with_capacity(productions_count);
        for _i in 0..productions_count {
            let production = LRProduction::new(data, &mut index);
            productions.push(production);
        }
        LRkAutomaton {
            columns_count,
            states_count,
            columns_map,
            contexts,
            table,
            productions,
        }
    }

    /// Gets the number of states in this automaton
    #[must_use]
    pub fn get_states_count(&self) -> usize {
        self.states_count
    }

    /// Gets the number of columns in the LR table
    #[must_use]
    pub fn get_columns_count(&self) -> usize {
        self.columns_count
    }

    /// Gets the symbol's identifier for a column
    #[must_use]
    pub fn get_sid_for_column(&self, column: usize) -> u32 {
        self.columns_map.get_id_at(column)
    }

    /// Gets the contexts opened by the specified state
    #[must_use]
    pub fn get_contexts(&self, state: u32) -> &LRContexts {
        &self.contexts[state as usize]
    }

    /// Gets the LR(k) action for the given state and sid
    #[must_use]
    pub fn get_action(&self, state: u32, identifier: u32) -> LRAction {
        let column = self.columns_map.get(identifier) as usize;
        self.get_action_at(state, column)
    }

    /// Gets the LR(k) action for the given state and column
    #[must_use]
    pub fn get_action_at(&self, state: u32, column: usize) -> LRAction {
        LRAction {
            table: &self.table,
            offset: (state as usize * self.columns_count + column) * 2,
        }
    }

    /// Gets the i-th production
    #[must_use]
    pub fn get_production(&self, index: usize) -> &LRProduction {
        &self.productions[index]
    }

    /// Gets the expected terminals for the specified state
    #[must_use]
    pub fn get_expected<'s>(&self, state: u32, terminals: &[Symbol<'s>]) -> LRExpected<'s> {
        let mut expected = LRExpected::new();
        let mut offset = self.columns_count * state as usize * 2;
        for terminal in terminals {
            let action = self.table[offset];
            if action == LR_ACTION_CODE_SHIFT {
                expected.shifts.push(*terminal);
            } else if action == LR_ACTION_CODE_REDUCE {
                expected.reductions.push(*terminal);
            }
            offset += 2;
        }
        expected
    }
}

const ESTIMATION_BIAS: usize = 5;

/// The data about a reduction
struct LRkAstReduction {
    /// The length of the reduction
    length: usize,
    /// The sub-tree build-up cache
    cache: SubTree,
    /// The number of items popped from the stack
    pop_count: usize,
}

impl LRkAstReduction {
    /// Turns this reduction data into a subtree
    pub fn into_subtree(self) -> SubTree {
        self.cache
    }
}

/// Represents the builder of Parse Trees for LR(k) parsers
struct LRkAstBuilder<'s, 't, 'a> {
    /// Lexer associated to this parser
    lexer: &'a mut Lexer<'s, 't, 'a>,
    /// The table of variables
    variables: &'a [Symbol<'s>],
    /// The table of virtuals
    virtuals: &'a [Symbol<'s>],
    /// The stack of semantic objects
    stack: Vec<SubTree>,
    /// The AST being built
    result: &'a mut AstImpl,
    /// The reduction handle represented as the indices of the sub-trees in the cache
    handle: Vec<usize>,
    /// The data of the current reduction
    reduction: Option<LRkAstReduction>,
}

impl<'s, 't, 'a> SemanticBody for LRkAstBuilder<'s, 't, 'a> {
    fn get_element_at(&self, index: usize) -> SemanticElement {
        match &self.reduction {
            None => panic!("Not in a reduction"),
            Some(data) => {
                let label = data.cache.get_label_at(self.handle[index]);
                match label.table_type() {
                    TableType::Token => SemanticElement::Token(
                        self.lexer.get_data().repository.get_token(label.index()),
                    ),
                    TableType::Variable => SemanticElement::Variable(self.variables[label.index()]),
                    TableType::Virtual => SemanticElement::Virtual(self.virtuals[label.index()]),
                    TableType::None => {
                        SemanticElement::Terminal(self.lexer.get_data().repository.terminals[0])
                    }
                }
            }
        }
    }

    fn length(&self) -> usize {
        self.handle.len()
    }
}

impl<'s, 't, 'a> LRkAstBuilder<'s, 't, 'a> {
    /// Initializes the builder with the given stack size
    pub fn new(
        lexer: &'a mut Lexer<'s, 't, 'a>,
        variables: &'a [Symbol<'s>],
        virtuals: &'a [Symbol<'s>],
        result: &'a mut AstImpl,
    ) -> LRkAstBuilder<'s, 't, 'a> {
        LRkAstBuilder {
            lexer,
            variables,
            virtuals,
            stack: Vec::new(),
            result,
            handle: Vec::new(),
            reduction: None,
        }
    }

    /// Push a token onto the stack
    pub fn push_token(&mut self, index: usize) {
        let mut single = SubTree::new(1);
        single.push(TableElemRef::new(TableType::Token, index), TREE_ACTION_NONE);
        self.stack.push(single);
    }

    /// Prepares for the forthcoming reduction operations
    pub fn reduction_prepare(&mut self, variable_index: usize, length: usize, action: TreeAction) {
        let mut estimation = ESTIMATION_BIAS;
        for i in 0..length {
            estimation += self.stack[self.stack.len() - length + i].get_size();
        }
        let mut cache = SubTree::new(estimation);
        cache.setup_root(
            TableElemRef::new(TableType::Variable, variable_index),
            action,
        );
        self.reduction = Some(LRkAstReduction {
            length,
            cache,
            pop_count: 0,
        });
    }

    /// During a reduction, insert the given sub-tree
    fn reduction_add_sub(
        reduction: &mut LRkAstReduction,
        handle: &mut Vec<usize>,
        sub: &SubTree,
        action: TreeAction,
    ) {
        if sub.get_action_at(0) == TREE_ACTION_REPLACE_BY_CHILDREN {
            let children_count = sub.get_children_count_at(0);
            // copy the children to the cache
            let mut cache_index = sub.copy_children_to(&mut reduction.cache);
            // setup the handle
            let mut sub_index = 1;
            for _i in 0..children_count {
                let size = sub.get_children_count_at(sub_index) + 1;
                handle.push(cache_index);
                cache_index += size;
                sub_index += size;
            }
        } else if action == TREE_ACTION_DROP {
            // do nothing
        } else {
            // copy the complete sub-tree to the cache
            let cache_index = sub.copy_to(&mut reduction.cache);
            handle.push(cache_index);
            if action != TREE_ACTION_NONE {
                reduction.cache.set_action_at(cache_index, action);
            }
        }
    }

    /// During a redution, pops the top symbol from the stack and gives it a tree action
    pub fn reduction_pop(&mut self, action: TreeAction) {
        match self.reduction.as_mut() {
            None => panic!("Not in a reduction"),
            Some(reduction) => {
                let sub = &self.stack[self.stack.len() - reduction.length + reduction.pop_count];
                LRkAstBuilder::reduction_add_sub(reduction, &mut self.handle, sub, action);
                reduction.pop_count += 1;
            }
        }
    }

    /// During a reduction, inserts a virtual symbol
    pub fn reduction_add_virtual(&mut self, index: usize, action: TreeAction) {
        if action != TREE_ACTION_DROP {
            match self.reduction.as_mut() {
                None => panic!("Not in a reduction"),
                Some(reduction) => {
                    let cache_index = reduction
                        .cache
                        .push(TableElemRef::new(TableType::Virtual, index), action);
                    self.handle.push(cache_index);
                }
            }
        }
    }

    /// Finalizes the reduction operation
    pub fn reduce(&mut self) {
        let stack_size = self.stack.len();
        match self.reduction.as_mut() {
            None => panic!("Not in a reduction"),
            Some(reduction) => {
                if reduction.cache.get_action_at(0) == TREE_ACTION_REPLACE_BY_CHILDREN {
                    reduction.cache.set_children_count_at(0, self.handle.len());
                } else {
                    LRkAstBuilder::reduce_tree(reduction, &self.handle, self.result);
                }
                // Put it on the stack
                self.stack.truncate(stack_size - reduction.length);
            }
        }
        let result = self.reduction.take().unwrap().into_subtree();
        self.handle.clear();
        self.stack.push(result);
    }

    /// Applies the promotion tree actions to the cache and commits to the final AST
    pub fn reduce_tree(reduction: &mut LRkAstReduction, handle: &[usize], result: &mut AstImpl) {
        // apply the epsilon replace, if any
        if reduction.cache.get_action_at(0) == TREE_ACTION_REPLACE_BY_EPSILON {
            reduction
                .cache
                .set_label_at(0, TableElemRef::new(TableType::None, 0));
            reduction.cache.set_action_at(0, TREE_ACTION_NONE);
        }
        // promotion data
        let mut promotion = false;
        let mut insertion = 1;
        for item in handle {
            let item = *item;
            if reduction.cache.get_action_at(item) == TREE_ACTION_PROMOTE {
                if promotion {
                    // This is not the first promotion
                    // Commit the previously promoted node's children
                    reduction.cache.set_children_count_at(0, insertion - 1);
                    reduction.cache.commit_children_of(0, result);
                    // Re-put the previously promoted node in the cache
                    reduction.cache.move_node(0, 1);
                    insertion = 2;
                }
                promotion = true;
                // Save the new promoted node
                reduction.cache.move_node(item, 0);
                // Repack the children on the left if any
                let nb = reduction.cache.get_children_count_at(0);
                reduction.cache.move_range(item + 1, insertion, nb);
                insertion += nb;
            } else {
                // Commit the children if any
                reduction.cache.commit_children_of(item, result);
                // Repack the sub-root on the left
                if insertion != item {
                    reduction.cache.move_node(item, insertion);
                }
                insertion += 1;
            }
        }
        // finalize the sub-tree data
        reduction.cache.set_children_count_at(0, insertion - 1);
    }

    /// Commits the tree's root
    pub fn commit_root(&mut self) {
        let length = self.stack.len();
        if length > 1 {
            let head = &mut self.stack[length - 2];
            head.commit(self.result);
        }
    }
}

/// The head of a LR(k) parser
#[derive(Copy, Clone)]
struct LRkHead {
    /// The automaton's state
    state: u32,
    /// The symbol identifier
    identifier: u32,
}

struct LRkParserData<'aut, 's, 'a> {
    /// The parser's automaton
    automaton: LRkAutomaton<'aut>,
    /// The parser's stack
    stack: Vec<LRkHead>,
    /// The grammar variables
    variables: &'a [Symbol<'s>],
    /// The semantic actions
    actions: &'a mut dyn FnMut(usize, Symbol, &dyn SemanticBody),
}

impl<'aut, 's, 'a> ContextProvider for LRkParserData<'aut, 's, 'a> {
    /// Gets the priority of the specified context required by the specified terminal
    /// The priority is an unsigned integer. The lesser the value the higher the priority.
    /// The absence of value represents the unavailability of the required context.
    fn get_context_priority(
        &self,
        token_count: usize,
        context: u16,
        terminal_id: u32,
    ) -> Option<usize> {
        // the default context is always active
        if context == DEFAULT_CONTEXT {
            return Some(usize::MAX);
        }
        if token_count == 0 {
            // this is the first token, does it open the context?
            let contexts = self.automaton.get_contexts(0);
            return if contexts.opens(terminal_id, context) {
                Some(0)
            } else {
                None
            };
        }
        // retrieve the action for this terminal
        let state = self.stack[self.stack.len() - 1].state;
        let mut action = self.automaton.get_action(state, terminal_id);
        // if the terminal is unexpected, do not validate
        if action.get_code() == LR_ACTION_CODE_NONE {
            return None;
        }
        // does the context opens with the terminal?
        if action.get_code() == LR_ACTION_CODE_SHIFT
            && self
                .automaton
                .get_contexts(state)
                .opens(terminal_id, context)
        {
            return Some(0);
        }
        let production = if action.get_code() == LR_ACTION_CODE_REDUCE {
            Some(self.automaton.get_production(action.get_data() as usize))
        } else {
            None
        };
        // look into the stack for the opening of the context
        let mut i = self.stack.len() - 2;
        loop {
            let state = self.stack[i].state;
            let id = self.stack[i + 1].identifier;
            if self.automaton.get_contexts(state).opens(id, context) {
                // the context opens here
                // but is it closed by the reduction (if any)?
                match production {
                    None => return Some(self.stack.len() - 1 - i),
                    Some(data) => {
                        if i < self.stack.len() - 1 - data.reduction_length {
                            return Some(self.stack.len() - 1 - i);
                        }
                    }
                }
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        // at this point, the requested context is not yet open or is closed by a reduction
        // now, if the action is something else than a reduction (shift, accept or error),
        // the context can never be produced
        // for the context to open, a new state must be pushed onto the stack
        // this means that the provided terminal must trigger a chain of at least one reduction
        if action.get_code() != LR_ACTION_CODE_REDUCE {
            return None;
        }
        // there is at least one reduction, simulate
        let mut my_stack = self.stack.clone();
        while action.get_code() == LR_ACTION_CODE_REDUCE {
            // execute the reduction
            let production = self.automaton.get_production(action.get_data() as usize);
            let variable = self.variables[production.head];
            let length = my_stack.len();
            my_stack.truncate(length - production.reduction_length);
            // this must be a shift
            action = self
                .automaton
                .get_action(my_stack[my_stack.len() - 1].state, variable.id);
            my_stack.push(LRkHead {
                state: u32::from(action.get_data()),
                identifier: variable.id,
            });
            // now, get the new action for the terminal
            action = self
                .automaton
                .get_action(u32::from(action.get_data()), terminal_id);
        }
        // is this a shift action that opens the context?
        if action.get_code() == LR_ACTION_CODE_SHIFT
            && self
                .automaton
                .get_contexts(my_stack[my_stack.len() - 1].state)
                .opens(terminal_id, context)
        {
            Some(0)
        } else {
            None
        }
    }
}

impl<'aut, 's, 't, 'a> LRkParserData<'aut, 's, 'a> {
    /// Checks whether the specified terminal is indeed expected for a reduction
    /// This check is required because in the case of a base LALR graph,
    /// some terminals expected for reduction in the automaton are coming from other paths.
    fn check_is_expected(&self, terminal: Symbol<'s>) -> bool {
        // copy the stack to use for the simulation
        let mut my_stack = self.stack.clone();
        let mut action = self
            .automaton
            .get_action(my_stack[my_stack.len() - 1].state, terminal.id);
        while action.get_code() != LR_ACTION_CODE_NONE {
            if action.get_code() == LR_ACTION_CODE_SHIFT {
                // yep, the terminal was expected
                return true;
            }
            if action.get_code() == LR_ACTION_CODE_REDUCE {
                // execute the reduction
                let production = self.automaton.get_production(action.get_data() as usize);
                let variable = self.variables[production.head];
                let length = my_stack.len();
                my_stack.truncate(length - production.reduction_length);
                // this must be a shift
                action = self
                    .automaton
                    .get_action(my_stack[my_stack.len() - 1].state, variable.id);
                my_stack.push(LRkHead {
                    state: u32::from(action.get_data()),
                    identifier: variable.id,
                });
                // now, get the new action for the terminal
                action = self
                    .automaton
                    .get_action(u32::from(action.get_data()), terminal.id);
            }
        }
        // nope, that was a pathological case in a LALR graph
        false
    }

    /// Parses on the specified token kernel
    fn parse_on_token(&mut self, kernel: TokenKernel, builder: &mut LRkAstBuilder) -> LRActionCode {
        let stack = &mut self.stack;

        loop {
            let head = stack[stack.len() - 1];
            let action = self.automaton.get_action(head.state, kernel.terminal_id);
            if action.get_code() == LR_ACTION_CODE_SHIFT {
                stack.push(LRkHead {
                    state: u32::from(action.get_data()),
                    identifier: kernel.terminal_id,
                });
                builder.push_token(kernel.index as usize);
                return action.get_code();
            }
            if action.get_code() != LR_ACTION_CODE_REDUCE {
                return action.get_code();
            }
            // now reduce
            let production = self.automaton.get_production(action.get_data() as usize);
            let variable = LRkParserData::reduce(production, builder, &mut self.actions);
            let length = stack.len();
            stack.truncate(length - production.reduction_length);
            let action = self.automaton.get_action(
                stack[stack.len() - 1].state,
                builder.variables[production.head].id,
            );
            stack.push(LRkHead {
                state: u32::from(action.get_data()),
                identifier: variable.id,
            });
        }
    }

    /// Executes the given LR reduction
    fn reduce(
        production: &LRProduction,
        builder: &mut LRkAstBuilder<'s, 't, 'a>,
        actions: &mut dyn FnMut(usize, Symbol, &dyn SemanticBody),
    ) -> Symbol<'s> {
        let variable = builder.variables[production.head];
        builder.reduction_prepare(
            production.head,
            production.reduction_length,
            production.head_action,
        );
        let mut i = 0;
        while i < production.bytecode.len() {
            let op_code = production.bytecode[i];
            i += 1;
            match get_op_code_base(op_code) {
                LR_OP_CODE_BASE_SEMANTIC_ACTION => {
                    let index = production.bytecode[i] as usize;
                    i += 1;
                    actions(index, variable, builder);
                }
                LR_OP_CODE_BASE_ADD_VIRTUAL => {
                    let index = production.bytecode[i] as usize;
                    i += 1;
                    builder.reduction_add_virtual(index, get_op_code_tree_action(op_code));
                }
                _ => {
                    builder.reduction_pop(get_op_code_tree_action(op_code));
                }
            }
        }
        builder.reduce();
        variable
    }
}

/// Represents a base for all LR(k) parsers
pub struct LRkParser<'aut, 's, 't, 'a> {
    /// The parser's data
    data: LRkParserData<'aut, 's, 'a>,
    /// The AST builder
    builder: LRkAstBuilder<'s, 't, 'a>,
}

impl<'aut, 's, 't, 'a> LRkParser<'aut, 's, 't, 'a> {
    /// Initializes a new instance of the parser
    pub fn new(
        lexer: &'a mut Lexer<'s, 't, 'a>,
        variables: &'a [Symbol<'s>],
        virtuals: &'a [Symbol<'s>],
        automaton: LRkAutomaton<'aut>,
        ast: &'a mut AstImpl,
        actions: &'a mut dyn FnMut(usize, Symbol, &dyn SemanticBody),
    ) -> LRkParser<'aut, 's, 't, 'a> {
        LRkParser {
            data: LRkParserData {
                automaton,
                stack: alloc::vec![LRkHead {
                    state: 0,
                    identifier: 0
                }],
                variables,
                actions,
            },
            builder: LRkAstBuilder::<'s, 't, 'a>::new(lexer, variables, virtuals, ast),
        }
    }

    /// Gets the next token in the kernel
    fn get_next_token(&mut self) -> Option<TokenKernel> {
        let data = &self.data;
        self.builder.lexer.get_next_token(data)
    }

    /// Builds the unexpected token error
    fn build_error(&self, kernel: TokenKernel) -> ParseErrorUnexpectedToken<'s> {
        let token = self
            .builder
            .lexer
            .get_data()
            .repository
            .get_token(kernel.index as usize);
        let state = self.data.stack[self.data.stack.len() - 1].state;
        let expected_on_head = self
            .data
            .automaton
            .get_expected(state, self.builder.lexer.get_data().repository.terminals);
        let mut my_expected = Vec::new();
        for x in &expected_on_head.shifts {
            my_expected.push(*x);
        }
        for x in &expected_on_head.reductions {
            if self.data.check_is_expected(*x) {
                my_expected.push(*x);
            }
        }
        ParseErrorUnexpectedToken::new(
            token.get_position().unwrap(),
            token.get_span().unwrap().length,
            token.get_value().unwrap().to_string(),
            token.get_symbol(),
            #[cfg(feature = "debug")]
            alloc::vec![state],
            my_expected,
        )
    }
}

impl<'aut, 's, 't, 'a> Parser for LRkParser<'aut, 's, 't, 'a> {
    fn parse(&mut self) {
        let mut kernel_maybe = self.get_next_token();
        loop {
            match kernel_maybe {
                None => {
                    self.builder.commit_root();
                    return;
                }
                Some(kernel) => {
                    let action = self.data.parse_on_token(kernel, &mut self.builder);
                    match action {
                        LR_ACTION_CODE_ACCEPT => {
                            self.builder.commit_root();
                            return;
                        }
                        LR_ACTION_CODE_SHIFT => {
                            kernel_maybe = self.get_next_token();
                        }
                        _ => {
                            // this is an error
                            let error = self.build_error(kernel);
                            self.builder
                                .lexer
                                .get_data_mut()
                                .errors
                                .push_error_unexpected_token(error);
                            // TODO: try to recover here
                            return;
                        }
                    }
                }
            }
        }
    }
}
