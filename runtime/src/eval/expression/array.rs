use tsr_lexer::{globals::Positioned, util::BoolExt};
use tsr_parser::ast::{ArraySize, Expression};

use crate::{value::Value, Runtime};

impl Runtime {
    pub fn eval_array(
        &mut self,
        elements: Vec<Positioned<Expression>>,
        is_dynamic: Positioned<bool>,
    ) -> Value {

        //println!("{:?}",elements);
        let elements = elements
            .into_iter()
            .map(|expression| match self.eval_expression(expression) {

                Value::Reference(path, scope) => {

                    //println!("=={:?},{:?}",path,scope);
                    Value::None
                }
                value => value,
            })
            .collect::<Vec<_>>();
        let size = elements.len();
        //println!("{:?},{:?}",elements,size);

        Value::Array(
            elements,
            is_dynamic
                .value
                .map(ArraySize::Dynamic, ArraySize::Fixed(size)),
        )
    }
}
