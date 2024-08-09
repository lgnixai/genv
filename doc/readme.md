[
VariableStatement(
VariableStatement {
mutable: true,
declarations: [
VariableDeclaration {
name: Ident(
"a",
),
ty: None,
nullable: false,
initializer: Some(
Literal(
Number(
10,
),
),
),
},
],
},
),
VariableStatement(
VariableStatement {
mutable: true,
declarations: [
VariableDeclaration {
name: Ident(
"b",
),
ty: None,
nullable: false,
initializer: Some(
Literal(
Number(
5,
),
),
),
},
],
},
),
VariableStatement(
VariableStatement {
mutable: true,
declarations: [
VariableDeclaration {
name: Ident(
"c",
),
ty: None,
nullable: false,
initializer: Some(
BinaryExpression(
BinaryExpression {
left: Ident(
Ident(
"a",
),
),
operator: Plus,
right: Literal(
Number(
3,
),
),
},
),
),
},
],
},
),
FunctionDeclaration(
FunctionDeclaration {
name: Ident(
"add",
),
modifiers: [],
type_parameters: [],
parameters: [
Parameter {
name: Ident(
"x",
),
nullable: false,
ty: UnionOrIntersectionOrPrimaryType(
IntersectionOrPrimaryType(
PrimaryType(
PredefinedType(
Number,
),
),
),
),
default: None,
},
Parameter {
name: Ident(
"y",
),
nullable: false,
ty: UnionOrIntersectionOrPrimaryType(
IntersectionOrPrimaryType(
PrimaryType(
PredefinedType(
Number,
),
),
),
),
default: None,
},
],
ty: UnionOrIntersectionOrPrimaryType(
IntersectionOrPrimaryType(
PrimaryType(
PredefinedType(
Number,
),
),
),
),
body: Some(
[
ReturnStatement(
BinaryExpression(
BinaryExpression {
left: Ident(
Ident(
"x",
),
),
operator: Plus,
right: Ident(
Ident(
"y",
),
),
},
),
),
],
),
},
),
Expression(
FunctionCallExpression(
FunctionCallExpression {
function: Ident(
Ident(
"print",
),
),
arguments: [
Ident(
Ident(
"c",
),
),
],
lambda: None,
},
),
),
]
10 Plus 3
13