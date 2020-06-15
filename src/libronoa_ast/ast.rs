struct Statement {
}

enum StatementKind {
}

struct Expression {
}

enum ExpressionKind {
    Call(Box<Expression>, Vec<Box<Expression>>),
    Assign(Box<Expression>, Box<Expression>),
    Binary(Box<BinaryOperation>, Box<Expression>, Box<Expression>),
    Literal(Literal)
}

struct BinaryOperation;

struct Literal;

/*

trait Visitor<T> {
    fn visit(&mut self, node: &T);
}

trait Node: Sized {
    fn accept<T>(&self, visitor: &mut T) where T: Visitor<Self> {
        visitor.visit(self);
    }
}

struct Walker;

impl Visitor<Statement> for Walker {
    fn visit(&mut self, statement: &Statement) {
        self.visit(&statement.left);
    }
}

impl Visitor<Expression> for Walker {
    fn visit(&mut self, expression: &Expression) {
    }
}

*/
