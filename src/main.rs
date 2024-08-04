use std::{env, os::unix::ffi::OsStrExt};
use std::fs::File;
use std::path::*;
use std::io::{self, BufRead, Read};

use parser::EvalVal;

mod lexer;
mod parser;
mod readfile;
mod ast;

use crate::ast::line_index;


pub fn get_PG() -> parser::Parser {
    let mut PG = parser::Parser::new();

    PG.add_rule(vec!["OP", "..", "CP"], "expr", "Paren", false);

    PG.new_precedence();

    PG.add_rule(vec!["LET", "NAME"], "VAR_NAME", "Name", false);
    PG.add_rule(vec!["NUM"], "expr", "Num", false);
    PG.add_rule(vec!["STRING"], "expr", "String", false);
    PG.add_rule(vec!["NONE"], "expr", "None", false);
    PG.add_rule(vec!["TRUE"], "expr", "Bool", false);
    PG.add_rule(vec!["FALSE"], "expr", "Bool", false);

    PG.new_precedence();

    PG.add_rule(vec!["expr", "AS", "STR"], "expr", "Asstr", false);
    PG.add_rule(vec!["expr", "AS", "INT"], "expr", "Asint", false);
    PG.add_rule(vec!["expr", "AS", "FLOAT"], "expr", "Asfloat", false);

    PG.new_precedence();

    PG.add_rule(vec!["NAME"], "expr", "Var", false);

    PG.new_precedence();

    PG.add_rule(vec!["expr", "CIRCUMFLEX", "expr"], "expr", "Pow", false);

    PG.new_precedence();

    PG.add_rule(vec!["expr", "STAR", "expr"], "expr", "Mul", false);
    PG.add_rule(vec!["expr", "SLASH", "expr"], "expr", "Div", false);

    PG.new_precedence();

    PG.add_rule(vec!["expr", "PLUS", "expr"], "expr", "Add", false);
    PG.add_rule(vec!["expr", "MINUS", "expr"], "expr", "Minus", false);
    
    PG.new_precedence();

    PG.add_rule(vec!["expr", "EQEQ", "expr"], "expr", "Condition", false);
    PG.add_rule(vec!["expr", "NOTEQ", "expr"], "expr", "Condition", false);
    PG.add_rule(vec!["expr", "LESS", "expr"], "expr", "Condition", false);
    PG.add_rule(vec!["expr", "LESSEQUAL", "expr"], "expr", "Condition", false);
    PG.add_rule(vec!["expr", "GREATER", "expr"], "expr", "Condition", false);
    PG.add_rule(vec!["expr", "GREATEREQUAL", "expr"], "expr", "Condition", false);

    PG.new_precedence();

    PG.add_rule(vec!["NOT", "expr"], "expr", "Not", false);

    PG.new_precedence();

    PG.add_rule(vec!["expr", "AND", "expr"], "expr", "And", false);

    PG.new_precedence();

    PG.add_rule(vec!["expr", "OR", "expr"], "expr", "Or", false);

    PG.new_precedence();

    PG.add_rule(vec!["INPUT", "OP", "expr", "CP"], "expr", "Input", false);

    PG.new_precedence();

    PG.add_rule(vec!["IF", "expr", "OCP"], "expr", "If", true);
    PG.add_rule(vec!["WHILE", "expr", "OCP"], "expr", "While", true);

    PG.new_precedence();

    PG.add_rule(vec!["PRINT", "OP", "expr", "CP"], "statement", "Print", true);
    PG.add_rule(vec!["VAR_NAME", "EQ", "expr"], "statement", "AssignVar", true);
   
    PG.add_rule(vec!["BREAK", "expr"], "statement", "Break", true);

    PG.new_precedence();

    PG.add_rule(vec!["PRINT", "OP", "expr"], "statement", "Print", true);
    PG.add_rule(vec!["BREAK"], "statement", "Break", true);


    

    PG
}


pub fn interpret(lines: Vec<String>) -> Option<EvalVal>{
   

    let PG = get_PG();

    let mut last_eval = None;

    unsafe {
        ast::lines = lines;

        while ast::line_index < ast::lines.len() {

            //println!("lines: {:?}", ast::lines.clone());

            let line = ast::lines[ast::line_index].clone();

            let mut line = line.trim().to_string();
            let line_split = line.split_once("#");
            if line_split.is_some() {
                line = line_split.unwrap().0.to_string();
            }
            if line.is_empty() {
                ast::line_index += 1;
                continue;
            }
    
            //println!("{line}");
            let tokens = lexer::lex(&mut line);
            //println!("toks: {:?}", tokens);
    
            
            lexer::AST_PRINT = true;
            
    
            let ast = PG.parse(&tokens, ast::line_index);
            
            //println!("ast: {:?}", ast);
    
            lexer::AST_PRINT = false;
            
    
            last_eval = ast.eval();
            //println!("last eval: {:?}", last_eval);

            if let Some(last_evalval) = last_eval.clone() {
                if last_evalval.should_break.is_some() {
                    //println!("break, lines: {:?}", ast::lines.clone());
                    return Some(last_evalval);
                }
                else if last_evalval.should_return.is_some() {
                    return Some(last_evalval.should_return.unwrap().unwrap());
                }
            }

            //println!("last eval: {:?}", last_eval);

            //println!("-------------------------------------------------------------------------------------------");

            ast::line_index += 1; 
        }
    }

    
    last_eval
}

fn main() {
    let mut lines = vec![];
    let args = env::args().collect::<Vec<String>>();

    if let Some(file_name) = args.get(1) {
        let files = readfile::get_files_with_extension(env::current_dir().unwrap(), "mama").unwrap();
        
        let arg_name;
            if let Some(arg_name_ohne_endung) = file_name.strip_suffix(".mama") {
                arg_name = arg_name_ohne_endung
            }
            else {
                arg_name = file_name
            }
        
        for file in files {
            let name = file.file_name().unwrap().as_bytes().to_vec();

            if name == (arg_name.to_string() + ".mama").bytes().collect::<Vec<u8>>() {
                let path = Path::new(&file);

                let file = File::open(&path).expect("couldnt load file");
            
                let mut reader: io::BufReader<File> = io::BufReader::new(file);
            
                let mut lines_string = String::new();

                let _ = reader.read_to_string(&mut lines_string);
            
                for line in lines_string.split(';') {
                        lines.push(line.trim().to_string().replace("\n", ""));
                }
        

                break;
            }
        }
    }
    else {
        lines = readfile::read("mama"); 
    }

    //println!("lines: {:?}", lines);

    //println!("result: {:?}", interpret(lines));
    interpret(lines);
}
