/*
 * Copyright (c) 2020 Nattakit Hosapsin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

pub struct NodeId
{
    pub num: u64,
}

pub struct Module
{
    pub id: NodeId,
    pub statements: Vec<Statement>,
}

pub struct Statement
{
    pub id: NodeId,
    pub kind: StatementKind,
}

pub enum StatementKind
{
    ModuleStatement(ModuleStatement),
    VariableDecl(VariableDecl),
    Expression(Expression),
}

pub enum ModuleStatement
{
    Function(Function),
}

pub struct Paremeter
{
    pub id: NodeId,
    pub name: String,
    pub param_type: Type,
}

pub struct Function
{
    pub id: NodeId,
    pub name: String,
    pub params: Vec<Paremeter>,
    pub return_type: Type,
    pub block: Block,
}

pub enum VariableConstKind
{
    CompileTime,
    Runtime,
    None,
}

pub struct VariableDecl
{
    pub id: NodeId,
    pub constness: VariableConstKind,
    pub name: String,
    pub var_type: Option<Type>,
    pub expression: Option<Expression>,
}

pub struct Block
{
    pub id: NodeId,
    pub statement: Vec<Statement>,
}

pub struct Expression
{
    pub id: NodeId,
    pub kind: ExpressionKind,
}

pub enum ExpressionKind
{
    Call(Box<Expression>, Vec<Box<Expression>>),
    Return(Option<Box<Expression>>),
    Assign(Box<Expression>, Box<Expression>),
    Binary(BinaryOperation, Box<Expression>, Box<Expression>),
    Literal(Literal),
}

pub enum BinaryOperation
{
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Literal
{
    pub id: NodeId,
    pub kind: LiteralKind,
}

pub enum LiteralKind
{
    String,
    Char(char),
    Int(u128, IntType),
    Float(f64, FloatType),
    Bool(bool),
}

pub enum IntType
{
    Signed(SignedIntKind),
    Unsigned(UnsignedIntKind),
    Default,
}

pub enum SignedIntKind
{
    I8,
    I16,
    I32,
    I64,
}

pub enum UnsignedIntKind
{
    U8,
    U16,
    U32,
    U64,
}

pub enum FloatType
{
    Suffixed(FloatKind),
    Default,
}

pub enum FloatKind
{
    F32,
    F64,
}

pub struct Type
{
    pub id: NodeId,
    pub kind: TypeKind,
}

pub enum TypeKind
{
    Array,
    Tuple(Vec<Box<Type>>),
    Inferred,
}
