use std::collections::HashMap;
use std::hash::Hash;
use std::mem;

pub struct Controller {
    pub text: String,
}

pub struct VisualBlock {
    text: String,
}

pub struct Color {
    hex_color: u32,
}

pub struct Tag {
    name: String,
    color: Color,
}

impl Controller {
    pub fn parse_text(&self) -> Vec<Tree> {
        let mut token_vector = Vec::new();
        let mut lexer = Lexer::new(&self.text, &mut token_vector);
        let token_vector = lexer.lex_input();

        let mut parser = Parser::new();
        let tree_vector = parser.parse_tokens_properly(token_vector.to_vec());

        tree_vector
    }

    pub fn token_amount(token_vector: &Vec<Token>) -> HashMap<&Token, u32> {
        let mut token_map = HashMap::new();

        for t in token_vector {
            let entry = token_map.entry(t).or_insert(0);
            *entry += 1;
        }

        token_map
    }
}

struct Lexer <'a> {
    pub input: String,
    input_chars: Vec<char>,
    
    position: usize, 
    read_position: usize,
    
    current_char: char,
    pub tokens: &'a mut Vec<Token>,
}

impl Lexer <'_> {
    pub fn new<'a>(input: &'a String, token_vector: &'a mut Vec<Token>) -> Lexer<'a> {
        Lexer {
            input: input.to_string(),
            input_chars: input.to_string().chars().collect(),
            position: 0,
            read_position: 1,

            current_char: 'h',
            tokens: token_vector,
        }
    }

    pub fn lex_input(&mut self) -> &Vec<Token> {
        for token_id in 0..(self.input_chars.len()-1) {
            self.next_char();
            
            
            self.tokens.push(match_char_to_token(self.current_char));
        }

        &self.tokens
    }

    fn read_current_char(&mut self) {
        self.current_char = *self.input_chars.get(self.position).unwrap();
    }

    fn next_char(&mut self) {
        self.read_current_char();
        self.position += 1;
        self.read_position += 1;
    }
}

struct Parser<'a> {
    parse_tree: Vec<Tree>,
    position: usize,
    current_token: &'a Token,
}

struct RecursionStorage {
    parse_tree: Vec<Tree>,
    position: usize,
}

impl Parser<'_> {
    fn new() -> Parser<'static> {
        Parser{
            parse_tree: Vec::<Tree>::new(),
            position: 0,
            current_token: &Token::Undefined,
        }
    }

    fn parse_tokens(&mut self, token_vector: Vec<Token>) -> Vec<Tree> {
        self.position = 0;

        while &self.position < &(token_vector.len()) {
            println!("{}, {}", &token_vector.len(), self.position);
            let mut token = &token_vector[self.position];

            if token == &Token::SquareBracketOpen {
                let mut start_position: usize = self.position;
                let mut close_position: usize = token_vector.len();
                let mut depth = 1;

                while depth != 0 {
                    self.peek();
                    token = &token_vector[self.position];

                    if token == &Token::SquareBracketOpen {  
                        if depth == 1 && self.position < close_position {
                            start_position = self.position;
                        }    
                        depth += 1;
                    }

                    else if token == &Token::SquareBracketClosed {
                        depth -= 1;
                        if depth == 1 && self.position < close_position {
                            close_position = self.position;
                        }
                    }
                }                 
                
                let branch_slice = &token_vector[start_position+1..close_position-2];
                println!("{:?}", branch_slice);

                let leaf_vec = self.parse_tokens(branch_slice.to_vec());
                println!(" Hello \n\n{:?}", leaf_vec);

                let branch = Tree::Branch(Branch::SquareBracket(leaf_vec));
                self.parse_tree.push(branch);
            }

            else if token == &Token::Letter {
                let leaf = Tree::Leaf(Leaf::Expression(*token));
                self.parse_tree.push(leaf);
            }

            self.peek();
        }

        let mut parse_tree_swap = Vec::<Tree>::new();
        mem::swap(&mut parse_tree_swap, &mut self.parse_tree);
        parse_tree_swap
    }

    fn parse_tokens_properly(&mut self, token_vector: Vec<Token>) -> Vec<Tree> {
        for pos in 0..token_vector.len() {
            let token = token_vector[pos];

            if token == Token::SquareBracketOpen {
                token_vector.push(token);
            }
            else if token == Token::SquareBracketClosed {

            }
        }

        storage.parse_tree
    }

    fn parse_loops(&mut self, token_vector: ) {
        let mut start_pos = 0;
        let mut end_pos = token_vector.len();

        for token in token_vector {
            token_pos += 1;


            if token == Token::SquareBracketOpen {
                token_vector.push(token);
            }
            else if token == Token::SquareBracketClosed {

            }
        }
    }

    fn peek(&mut self) {
        self.position += 1;
    }
}

#[derive(PartialEq, Clone, Debug, Hash, Eq, Copy)]
pub enum Token {
    Space,
    BackSlash,
    ForwardSlash,
    SquareBracketOpen,
    SquareBracketClosed,

    Letter,
    Undefined,
}

#[derive(Debug)]
pub enum Tree {
    Branch(Branch),
    Leaf(Leaf), 
}

#[derive(Debug)]
pub enum Branch {
    SquareBracket(Vec<Tree>),
}

#[derive(Debug)]
pub enum Leaf { 
    Expression(Token),
}

fn match_token_to_char(t: Token) -> char {
    match t {
        Token::Space                => ' ',
        Token::BackSlash            => '\\',
        Token::ForwardSlash         => '/',
        Token::SquareBracketOpen    => '[',
        Token::SquareBracketClosed  => ']',
        _                           => 'r',
    }
}

fn match_char_to_token(c: char) -> Token {
    match c {
        ' '                     => Token::Space                ,
        '\\'                    => Token::BackSlash            ,
        '/'                     => Token::ForwardSlash         ,
        '['                     => Token::SquareBracketOpen    ,
        ']'                     => Token::SquareBracketClosed  ,
        _                       => {
            if c.is_alphabetic() {return Token::Letter}
            else { return Token::Undefined}
        },  
    }
}
