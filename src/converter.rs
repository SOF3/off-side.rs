use std::iter::FromIterator;

use indent_stack::IndentStack;
use proc_macro2::{Delimiter, Group, LineColumn, Punct, Spacing, Span, TokenStream, TokenTree};
use proc_macro2::token_stream::IntoIter;

pub struct Converter {
    output: Vec<FlatToken>,
    state: ParseState,
    indent_stack: IndentStack,
    close_stack: Vec<bool>,
    cursor: LineColumn,
}

impl Default for Converter {
    fn default() -> Self {
        Self {
            output: Default::default(),
            state: Default::default(),
            indent_stack: Default::default(),
            close_stack: Default::default(),
            cursor: LineColumn { line: 0, column: 0 },
        }
    }
}

impl Converter {
    pub fn iterate(&mut self, tokens: IntoIter) -> syn::Result<()> {
        for token in tokens {
            self.read_token(token)?;
        }
        Ok(())
    }

    fn read_token(&mut self, token: TokenTree) -> syn::Result<()> {
        let span = token.span();
        println!("Reading token {}", token);
        dbg!(span.start());
        self.read_space(span, span.start())?;

        let mut consumed = false;
        match self.state {
            ParseState::Read => {
                if let TokenTree::Punct(punct) = &token {
                    if punct.as_char() == ':' {
                        self.state = ParseState::Colon;
                        consumed = true;
                    }
                }
            }
            ParseState::Colon => {
                if let TokenTree::Punct(punct) = &token {
                    if punct.as_char() == ';' {
                        self.state = ParseState::ColonSemi;
                        consumed = true;
                    }
                }
                if !consumed {
                    self.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
                    self.state = ParseState::Read;
                }
            }
            ParseState::ColonSemi => {
                self.state = ParseState::Read;
                self.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
                self.push(TokenTree::Punct(Punct::new(';', Spacing::Joint)));
            }
        }

        if !consumed {
            match token {
                TokenTree::Ident(_) | TokenTree::Literal(_) | TokenTree::Punct(_) => {
                    self.push(token);
                }
                TokenTree::Group(group) => { self.read_group(group)?; }
            }
        }

        self.cursor = span.end();

        Ok(())
    }

    fn read_group(&mut self, group: Group) -> syn::Result<()> {
        let delim = group.delimiter();
        self.output.push(FlatToken::OpenDelim(delim));
        self.cursor.column += 1;
        self.iterate(group.stream().into_iter())?;
        self.output.push(FlatToken::CloseDelim(false));
        Ok(())
    }

    fn read_space(&mut self, span: Span, until: LineColumn) -> syn::Result<()> {
        println!("White space from {}:{} to {}:{}", self.cursor.line, self.cursor.column, until.line, until.column);

        if self.state != ParseState::Read && until.line > self.cursor.line {
            self.cursor.line = until.line;
            let indents = until.column;
            let delta = self.indent_stack.accept(" ".repeat(indents).as_str())
                .map_err(|err| syn::Error::new(
                    span, format!("{} at {}:{}", err, until.line, until.column)))?;
            self.read_indent(delta)?;
        }
        self.state = ParseState::Read;
        self.cursor = until;
        Ok(())
    }

    fn read_indent(&mut self, delta: isize) -> syn::Result<()> {
        if delta == 1 {
            self.output.push(FlatToken::OpenDelim(Delimiter::Brace));
            self.close_stack.push(match self.state {
                ParseState::Colon => false,
                ParseState::ColonSemi => true,
                _ => unreachable!(),
            }.into())
        } else if delta < 0 {
            for _ in 0..-delta {
                let semi = self.close_stack.pop().expect("Unexpected dedent");
                self.output.push(FlatToken::CloseDelim(semi));
            }
        }
        Ok(())
    }

    fn push(&mut self, token: TokenTree) { self.output.push(FlatToken::Tree(token)) }

    pub fn collect(mut self) -> syn::Result<TokenStream> {
        // clean up dangling indents
        let dangling = self.indent_stack.accept("").expect("Indent error for empty indent???");
        self.read_indent(dangling)?;

        let (ret, _semi) = build_tree(&mut self.output.into_iter(), false);
        Ok(TokenStream::from_iter(ret.into_iter()))
    }
}

fn build_tree(flat: &mut std::vec::IntoIter<FlatToken>, inner: bool) -> (Vec<TokenTree>, bool) {
    let mut ret = Vec::<TokenTree>::new();
    while let Some(token) = flat.next() {
        match token {
            FlatToken::Tree(tree) => ret.push(tree),
            FlatToken::OpenDelim(delim) => {
                let (vec, semi) = build_tree(flat, true);
                let stream = TokenStream::from_iter(vec.into_iter());
                ret.push(TokenTree::Group(Group::new(delim, stream)));
                if semi {
                    ret.push(TokenTree::Punct(Punct::new(';', Spacing::Alone)));
                }
            }
            FlatToken::CloseDelim(semi) => {
                assert!(inner, "Unmatched CloseDelim");
                return (ret, semi);
            }
        }
    }
    assert!(!inner, "Unmatched OpenDelim");
    (ret, false)
}

#[derive(PartialEq)]
enum ParseState {
    Read,
    Colon,
    ColonSemi,
}

impl Default for ParseState { fn default() -> Self { ParseState::Read } }

enum FlatToken {
    Tree(TokenTree),
    OpenDelim(Delimiter),
    CloseDelim(bool),
}
