use php_parser_rs::parser::ast::{
    MethodCallExpression, NewExpression,
};

use crate::analyse::Rule;

pub struct E0010 {}

impl Rule for E0010 {
    fn validate(
        &self,
        statement: &php_parser_rs::parser::ast::Statement,
    ) -> Vec<crate::project::Suggestion> {
        if let php_parser_rs::parser::ast::Statement::Expression(expression_statement) = statement {
            let _class_name_target = match &expression_statement.expression {
                php_parser_rs::parser::ast::Expression::AssignmentOperation(assign) => {
                    match assign.right() {
                        php_parser_rs::parser::ast::Expression::New(NewExpression {
                            new: _,
                            target,
                            arguments: _,
                        }) => Some(target),
                        _ => None,
                    }
                }
                _ => None,
            };

            if let php_parser_rs::parser::ast::Expression::MethodCall(MethodCallExpression {
                target: _,
                arrow: _,
                method: _,
                arguments: _,
            }) = &expression_statement.expression {}
        };
        vec![]
    }
}
