use crate::{
    error::CompilerError,
    frontend::{
        ast::{ASTNode, Block, BlockItem},
        tokens::Token,
    },
};

use super::Parser;

impl Parser {
    pub fn parse_block(&mut self) -> Result<ASTNode<Block>, CompilerError> {
        self.trace("Block");

        let start = self.peek()?.span;

        self.expect(Token::OpenBrace)?;
        let mut block_items = Vec::new();
        while let Some(tok) = self.tokens.peek() {
            if tok.token == Token::CloseBrace {
                break;
            }
            if let Some(next_tok) = self.tokens.peek()
                && next_tok.token == Token::TypeInt
            {
                let decl = self.parse_decl()?;
                let span = decl.span;
                block_items.push(span.build(BlockItem::Declaration(decl))?);
            } else {
                let statement = self.parse_statement()?;
                let span = statement.span;
                block_items.push(span.build(BlockItem::Statement(statement))?);
            }
        }

        let end = self.expect(Token::CloseBrace)?;

        let span = start.merge(&end.span);
        span.build(Block::Block(block_items))
    }
}
