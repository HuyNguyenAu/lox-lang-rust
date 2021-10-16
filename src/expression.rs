use std::collections::HashMap;

use crate::Lox;

mod scanner;

use scanner::Token;

pub trait Expression {
    fn accept<T: Visitor>(&self, visitor: &mut T) -> T;
}

trait Visitor {
    fn visit_assign_expression(&mut self, expression: Assign) -> T;
    fn visit_binary_expression(&mut self, expression: Binary) -> T;
    fn visit_call_expression(&mut self, expression: Call) -> T;
    fn visit_get_expression(&mut self, expression: Get) -> T;
    fn visit_grouping_expression(&mut self, expression: Grouping) -> T;
    fn visit_literal_expression(&mut self, expression: Literal) -> T;
    fn visit_logical_expression(&mut self, expression: Logical) -> T;
    fn visit_set_expression(&mut self, expression: Set) -> T;
    fn visit_super_expression(&mut self, expression: Super) -> T;
    fn visit_this_expression(&mut self, expression: This) -> T;
    fn visit_unary_expression(&mut self, expression: Unary) -> T;
    fn visit_variable_expression(&mut self, expression: Variable) -> T;
}

// Assign.

pub struct Assign {
    name: Token,
    value: Expression,
}

impl Expression for Assign {
    fn new(name: Token, value: Expression) -> Self {
        Assign { name, value }
    }

    fn accept<T: Visitor>(&self, visitor: &mut T) -> T {
        visitor.visit_assign_expression(self)
    }
}

// Binary.

pub struct Binary {
    left: Expression,
    operator: Token,
    right: Expression,
}

impl Expression for Binary {
    fn new(left: Expression, operator: Token, right: Expression) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }

    fn accept<T: Visitor>(&self, visitor: &mut T) -> T {
        visitor.visit_binary_expression(self)
    }
}

// Call.

pub struct Call {
    callee: Expression,
    paren: Token,
    arguments: Vec<Expression>,
}

impl Expression for Call {
    fn new(callee: Expression, paren: Token, arguments: Vec<Expression>) -> Self {
        Call {
            callee,
            paren,
            arguments,
        }
    }

    fn accept<T: Visitor>(&self, visitor: &mut T) -> T {
        visitor.visit_call_expression(self)
    }
}

// Get.

pub struct Get {
    object: Expression,
    name: Token,
}

impl Expression for Get {
    fn new(object: Expression, name: Token) -> Self {
        Get { object, name }
    }

    fn accept<T: Visitor>(&self, visitor: &mut T) -> T {
        visitor.visit_get_expression(self)
    }
}

// Grouping.

pub struct Grouping {
    expression: Expression,
}

impl Expression for Grouping {
    fn new(expression: Expression) -> Self {
        Grouping { expression }
    }

    fn accept<T: Visitor>(&self, visitor: &mut T) -> T {
        visitor.visit_grouping_expression(self)
    }
}

// Literal.

pub struct Literal<G> {
    expression: G,
}

impl Expression for Literal<G> {
    fn new(value: G) -> Self {
        Literal { value }
    }

    fn accept<T: Visitor>(&self, visitor: &mut T) -> T {
        visitor.visit_literal_expression(self)
    }
}

// Logical.

pub struct Logical {
    left: Expression,
    operator: Token,
    right: Expression,
}

impl Expression for Logical {
    fn new(left: Expression, operator: Token, right: Expression) -> Self {
        Logical {
            left,
            operator,
            right,
        }
    }

    fn accept<T: Visitor>(&self, visitor: &mut T) -> T {
        visitor.visit_logical_expression(self)
    }
}

// Set.

pub struct Set {
    object: Expression,
    name: Token,
    value: Expression,
}

impl Expression for Set {
    fn new(object: Expression, name: Token, value: Expression) -> Self {
        Set {
            object,
            name,
            value,
        }
    }

    fn accept<T: Visitor>(&self, visitor: &mut T) -> T {
        visitor.visit_set_expression(self)
    }
}

// Super.

pub struct Super {
    keyword: Token,
    method: Token,
}

impl Expression for Super {
    fn new(keyword: Token, method: Token) -> Self {
        Super { keyword, method }
    }

    fn accept<T: Visitor>(&self, visitor: &mut T) -> T {
        visitor.visit_super_expression(self)
    }
}

// This.

pub struct This {
    keyword: Token,
}

impl Expression for This {
    fn new(keyword: Token) -> Self {
        This { keyword }
    }

    fn accept<T: Visitor>(&self, visitor: &mut T) -> T {
        visitor.visit_this_expression(self)
    }
}

// Unary.

pub struct Unary {
    operator: Token,
    right: Expression,
}

impl Expression for Unary {
    fn new(operator: Token, right: Expression) -> Self {
        Unary { operator, right }
    }

    fn accept<T: Visitor>(&self, visitor: &mut T) -> T {
        visitor.visit_unary_expression(self)
    }
}

// Variable.

pub struct Variable {
    name: Token,
}

impl Expression for Variable {
    fn new(name: Token) -> Self {
        Variable { name }
    }

    fn accept<T: Visitor>(&self, visitor: &mut T) -> T {
        visitor.visit_variable_expression(self)
    }
}
