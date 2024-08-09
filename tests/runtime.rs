use std::{fs, io};

use tsr_lexer::{globals::Span, Lexer};
use tsr_parser::{ast::PredefinedType, Parser};
use tsr_runtime::{
    api::{reflection::Reflection, util::Util},
    value::{builders::ObjectBuilder, Value},
    FunctionBuilder, Runtime,
};
use tsr_runtime::api::events::Events;
use tsr_runtime::value::{ClassInstance, Field};

#[test]
fn main() -> io::Result<()> {
    let path = "main.tsx";
    let input = fs::read_to_string(path)?;
    let code = input.as_bytes();

    let (_, tokens) = Lexer::lex_tokens(code.into()).unwrap();
    let (_, ast) = Parser::parse_tokens(&tokens).unwrap();
    let mut runtime = Runtime::default();

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
     println!("{:?}",ast);

    runtime.set_variable(
        "print",
        Span::default().wrap(FunctionBuilder::new("log")
                                 .param("data", PredefinedType::Any)
                                 .returns(PredefinedType::Void)
                                 .build(|args| {
                                     if let Some(data) = args.get("data") {
                                        // println!("{:?}",data);
                                         match data {
                                             Value::String(data) => println!("{data}"),
                                             data => println!("{data:#}"),
                                         }
                                     }
                                 })),
    );
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

    runtime.add_module(&Reflection);
    runtime.add_module(&Util);
    runtime.add_module(&Events);
    //
   println!("{}", runtime.eval_program(ast).format(path, &input));
   // println!("{:#?}", runtime.get_context());

    Ok(())
}
