

fn main(){
    let mut runtime = Runtime::new();

    // 设置一些变量
    runtime.set_variable("x", Span::default().wrap(Value::Number(5.0)));
    runtime.set_variable("y", Span::default().wrap(Value::Reference(vec!["x".to_string()], Scope::Global)));

    // 创建一个函数调用表达式
    let call_expr = FunctionCallExpression {
        function: Box::new(Span::default().wrap(Expression::Ident(Span::default().wrap("console.log".to_string())))),
        arguments: vec![Span::default().wrap(Expression::Ident(Span::default().wrap("y".to_string())))],
        lambda: None,
    };

    // 评估函数调用
    let result = runtime.eval_call(Span::default().wrap(call_expr));

    // 验证结果
    assert_eq!(result, Value::Number(5.0));
}
