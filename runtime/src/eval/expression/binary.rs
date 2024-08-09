use tsr_lexer::token::Operator;
use tsr_parser::ast::BinaryExpression;

use crate::{value::Value, Runtime};
use crate::environment::Scope;
use crate::eval::error::RuntimeError;

impl Runtime {


    // fn resolve_value(&self, value: Value) -> Value {
    //     match value {
    //         Value::Reference(path, scope) => {
    //             match self.resolve_reference(&path, &scope) {
    //                 Ok(resolved) => resolved,
    //                 Err(_) => Value::Null, // 或者你可以选择抛出错误
    //             }
    //         },
    //         _ => value,
    //     }
    // }
    //
    pub(crate) fn resolve_if_reference(&self, value: Value) -> Value {
        match value {
            Value::Reference(path, scope) => self.resolve_reference(&path, scope).unwrap_or(Value::None),
            _ => value,
        }
    }

    pub(crate) fn resolve_reference(&self, path: &[String], scope:Scope) -> Result<Value, RuntimeError> {
        // 这个方法的实现应该与之前讨论的类似
        let var_name = path.last().ok_or(RuntimeError::InvalidReference)?;
        let mut context = self.context.lock().unwrap();

        match context.get_mut(var_name, scope.clone()) {
            Some(variable) => {
                // 如果变量存在，我们返回它的值
                // 注意：这里我们克隆值，以避免借用检查器的问题
                Ok(variable.value.clone())
            },
            None => {
                // 如果变量不存在，返回一个 UndefinedVariable 错误
                Err(RuntimeError::UndefinedVariable(var_name.clone()))
            }
        }

    }

    fn binary_operation<F>(&self, left: Value, right: Value, op: F) -> Value
        where
            F: Fn(f64, f64) -> f64,
    {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Value::Number(op(a as f64, b as f64) as i64),
            // (Value::String(a), Value::String(b)) if matches!(op, |a, b| a + b) => {
            //     Value::String(a + &b)
            // }
            // 可以添加更多类型的组合
            _ => Value::None, // 或者返回一个类型错误
        }
    }
    pub fn eval_binary_expression(&mut self, expression: BinaryExpression) -> Value {
        // let left = self.eval_expression(expression.left);
        // let right = self.eval_expression(expression.right);
        let left = self.eval_expression(expression.left);
        let right = self.eval_expression(expression.right);

        let left = self.resolve_if_reference(left);
        let right = self.resolve_if_reference(right);

        println!("{left} {:?} {right}", expression.operator.value);

        match expression.operator.value {
            Operator::And => todo!(),
            Operator::AndAnd => todo!(),
            Operator::Minus => self.binary_operation(left, right, |a, b| a - b),

            Operator::Plus => match (left, right) {
                (Value::String(first), Value::String(second)) => Value::String(first + &second),
                (Value::Number(first), Value::Number(second)) => Value::Number(first + second),
                (Value::Reference(path, scope), Value::Number(second)) => {
                    match self.resolve_reference(&path, scope) {
                        Ok(value) => match value {
                            Value::Number(first) => Value::Number(first + second),
                            _ => Value::None
                        },
                        Err(e) => Value::None,
                    }
                },
                (Value::Number(first),Value::Reference(path, scope), ) => {
                    match self.resolve_reference(&path, scope) {
                        Ok(value) => match value {
                            Value::Number(second) => Value::Number(first + second),
                            _ => Value::None
                        },
                        Err(e) => Value::None,
                    }
                },
                (Value::Reference(left_path, left_scope), Value::Reference(right_path, right_scope)) => {
                    match (self.resolve_reference(&left_path, left_scope), self.resolve_reference(&right_path, right_scope)) {
                        (Ok(left_value), Ok(right_value)) => match (left_value, right_value) {
                            (Value::Number(first), Value::Number(second)) => Value::Number(first + second),
                            (Value::String(first), Value::String(second)) => Value::String(first + &second),
                            // 可以根据需要添加其他类型的组合
                            _ => Value::None // 或者返回一个错误
                        },
                        _ => Value::None // 如果任何一个引用解析失败，返回 None 或错误
                    }
                },
                // (Value::Reference(path, scope),Value::Reference(path, scope), ) => {
                //     match self.resolve_reference(&path, scope) {
                //         Ok(value) => match value {
                //             Value::Number(second) => Value::Number(first + second),
                //             _ => Value::None
                //         },
                //         Err(e) => Value::None,
                //     }
                // },
                (_, _) => todo!(),
            },
            Operator::Star => match (left, right) {
                (Value::String(data), Value::Number(times)) => {
                    Value::String(data.repeat(times as usize))
                }
                (Value::Number(first), Value::Number(second)) => Value::Number(first * second),
                (Value::Number(first), Value::Number(second)) => Value::Number(first * second),
                (_, _) => todo!(),
            },
            Operator::Slash => match (left, right) {
                (Value::Number(first), Value::Number(second)) => Value::Number(first / second),
                (_, _) => todo!(),
            },
            Operator::Or => todo!(),
            Operator::OrOr => todo!(),
            Operator::PlusPlus => match left {
                Value::Reference(path, scope) => {
                    let mut context = self.context.lock().unwrap();

                    if let Some(Value::Number(value)) =
                        context.get(&path[0], scope.clone()).map(|var| &var.value)
                    {
                        let value = Value::Number(value + 1);

                        context.set(&path, scope, value);
                    }

                    Value::None
                }
                _ => todo!(),
            },
            // Operator::Minus => match (left, right) {
            //     (Value::Number(first), Value::Number(second)) => Value::Number(first - second),
            //     (_, _) => todo!(),
            // },
            Operator::MinusMinus => match left {
                Value::Reference(path, scope) => {
                    let mut context = self.context.lock().unwrap();

                    if let Some(Value::Number(value)) =
                        context.get(&path[0], scope.clone()).map(|var| &var.value)
                    {
                        let value = Value::Number(value - 1);

                        context.set(&path, scope, value);
                    }

                    Value::None
                }
                _ => todo!(),
            },
            Operator::EqEq => Value::Boolean(left == right),
            Operator::Eq => match (left, right) {
                (Value::Reference(path, scope), value) => {
                    let value = match value {
                        Value::Reference(path, scope) => todo!(),
                        value => value,
                    };

                    self.context.lock().unwrap().set(&path, scope, value);

                    Value::None
                }
                (_, _) => todo!(),
            },
            Operator::Ne => Value::Boolean(left != right),
            Operator::Le => todo!(),
            Operator::Ge => todo!(),
            Operator::Lt => todo!(),
            Operator::Gt => todo!(),
            Operator::Not => match left {
                Value::Reference(path, scope) => {
                    if let Some(Value::Boolean(value)) = self
                        .context
                        .lock()
                        .unwrap()
                        .get(&path[0], scope.clone())
                        .map(|var| &var.value)
                    {
                        return Value::Boolean(!value);
                    }

                    Value::None
                }
                Value::Boolean(value) => Value::Boolean(!value),
                _ => todo!(),
            },
        }
    }
}