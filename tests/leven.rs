use std::{fs, io};

use tsr_lexer::{globals::Span, Lexer};
use tsr_lexer::globals::Positioned;
use tsr_lexer::token::Token::Ident;
use tsr_parser::{ast::PredefinedType, Parser};
use tsr_parser::ast::FunctionCallExpression;
use tsr_parser::ast::Statement::Expression;
use tsr_runtime::{
    api::{reflection::Reflection, util::Util},
    value::{builders::ObjectBuilder, Value},
    FunctionBuilder, Runtime,
};
use tsr_runtime::environment::Scope;

#[test]
fn test_eval_call_with_references() {
     let mut runtime = Runtime::default();

    // 设置一些变量
    runtime.set_variable("x", Span::default().wrap(Value::Number(5.0 as i64)));
    runtime.set_variable("y", Span::default().wrap(Value::Reference(vec!["x".to_string()], Scope::Global)));
// 创建一个模拟的 console.log 函数

    runtime.set_variable(
        "console",
        Span::default().wrap(
            ObjectBuilder::default()
                .prop(
                    "log",
                    FunctionBuilder::new("log")
                        .param("data", PredefinedType::Any)
                        .returns(PredefinedType::Void)
                        .build(|args| {
                            if let Some(data) = args.get("data") {
                                match data {
                                    Value::String(data) => println!("{data}"),
                                    data => println!("{data:#}"),
                                }
                            }
                        }),
                )
                .build(),
        ),
    );
    // runtime.set_variable(
    //     "console",
    //     Span::default().wrap(Value::Object(
    //         vec![("log".to_string(), Value::NativeFunction(Box::new(|args: Vec<Value>| {
    //             println!("Logged: {:?}", args);
    //             Value::Undefined
    //         })))]
    //             .into_iter()
    //             .collect()
    //     ))
    // );

    // 创建一个函数调用表达式
    // let call_expr = FunctionCallExpression {
    //     function: Box::new(Span::default().wrap(Expression::Ident(Positioned::new("console.log".to_string(), Span::default())))),
    //     arguments: vec![Span::default().wrap(Expression::Ident(Positioned::new("y".to_string(), Span::default())))],
    //     lambda: None,
    // };
    // 创建一个函数调用表达式
    // let call_expr = FunctionCallExpression {
    //     function: Box::new(Span::default().wrap(Expression::Ident(Positioned::new(
    //         Ident("console.log".to_string()),
    //         Span::default()
    //     )))),
    //     arguments: vec![Span::default().wrap(Expression::Ident(Positioned::new(
    //         Ident("y".to_string()),
    //         Span::default()
    //     )))],
    //     lambda: None,
    // };
    // span.wrap(tsr_parser::ast::Expression::FunctionCallExpression(Box::new(span.wrap(
    //     FunctionCallExpression {
    //         function: Box::new(fn_handle.clone()),
    //         arguments,
    //         lambda,
    //     },
    // )))
    fn log(a: Box<str>){
        println!("{:?}",a);
    }
    let call_expr = FunctionCallExpression {
        function: Box::new(  Span::default().wrap(Expression::Ident(
            Positioned::new(Ident("console.log".to_string()), Span::default())
        ))),
        arguments: vec![Span::default().wrap(Expression::Ident(
            Positioned::new(Ident("y".to_string()), Span::default())
        ))],
        lambda: None,
    };
    // 评估函数调用
    let result = runtime.eval_call(Span::default().wrap(call_expr));

    // // 创建一个函数调用表达式
    // let call_expr = FunctionCallExpression {
    //     function: Box::new(Span::default().wrap(Expression::Ident(Span::default().wrap("console.log".to_string())))),
    //     arguments: vec![Span::default().wrap(Expression::Ident(Span::default().wrap("y".to_string())))],
    //     lambda: None,
    // };
    //
    // // 评估函数调用
    // let result = runtime.eval_call(Span::default().wrap(call_expr));

    // 验证结果
    assert_eq!(result, Value::Number(5.0 as i64));
}