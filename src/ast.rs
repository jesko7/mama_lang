use std::{collections::HashMap, io::{self, Write}, process::Output};

use crate::{get_PG, interpret, lexer, parser::{EvalVal, Node}};
use once_cell::unsync::Lazy;

pub static mut vars: once_cell::unsync::Lazy<HashMap<String, Option<EvalVal>>> = Lazy::new(HashMap::new);
pub static mut line_index: usize = 0;
pub static mut lines: Vec<String> = vec![];

pub fn Add(node: Node) -> Option<EvalVal> {
    let num1 = node.token_values[0].eval();
    let num2 = node.token_values[2].eval();

    let output = num1.unwrap().num.unwrap() + num2.unwrap().num.unwrap();

    Some(
        EvalVal {  
            num: Some(output),  
            ..Default::default()
        }
    )
}

pub fn Minus(node: Node) -> Option<EvalVal> {
    let num1 = node.token_values[0].eval();
    let num2 = node.token_values[2].eval();
    let output = num1.unwrap().num.unwrap() - num2.unwrap().num.unwrap();

    Some(
        EvalVal {  
            num: Some(output),
            ..Default::default()
        }
    )
}

pub fn Mul(node: Node) -> Option<EvalVal> {
    let num1 = node.token_values[0].eval();
    let num2 = node.token_values[2].eval();

    let output = num1.unwrap().num.unwrap() * num2.unwrap().num.unwrap();

    Some(
        EvalVal {  
            num: Some(output),
            ..Default::default()
        }
    )
}

pub fn Div(node: Node) -> Option<EvalVal> {
    let num1 = node.token_values[0].eval();
    let num2 = node.token_values[2].eval();

    let output = num1.unwrap().num.unwrap() / num2.unwrap().num.unwrap();

    Some(
        EvalVal {  
            num: Some(output),
            ..Default::default()
        }
    )
}

pub fn Pow(node: Node) -> Option<EvalVal> {
    let num1 = node.token_values[0].eval();
    let num2 = node.token_values[2].eval();

    let output = num1.unwrap().num.unwrap().powf(num2.unwrap().num.unwrap());

    Some(
        EvalVal {  
            num: Some(output),
            ..Default::default()
        }
    )
}

pub fn Num(node: Node) -> Option<EvalVal> {
    let output = node.token_values[0].token.as_ref().unwrap().value().parse::<f32>().unwrap();

    Some(
        EvalVal {  
            num: Some(output),
            ..Default::default()
        }
    )
}



pub fn String(node: Node) -> Option<EvalVal> {
    let output = node.token_values[0].token.as_ref().unwrap().value();

    Some(
        EvalVal {  
            string: Some(output),
            ..Default::default()
        }
    )
}

pub fn Var(node: Node) -> Option<EvalVal> {
    let output = node.token_values[0].token.as_ref().unwrap().value();
    let var;

    unsafe {
        var = vars.get(&output);
    }


    if var.is_none() {
        //"erererererererer"
        panic!();
    }

    


    let res = var.unwrap();
        

    if res.is_none() {
        return None;
    }
    else {
        res.clone()
    }
}

pub fn NoneNode(node: Node) -> Option<EvalVal> {
    None
}

pub fn Name(node: Node) -> Option<EvalVal> {
    let output = node.token_values[1].token.as_ref().unwrap().value();

    Some(
        EvalVal {  
            string: Some(output),
            ..Default::default()
        }
    )
}

pub fn Print(node: Node) -> Option<EvalVal> {
    
    let output = node.token_values[2].eval();

    if node.token_values[2].class_name == "While" || node.token_values[2].class_name == "If" {
        unsafe {line_index += 1;}
    }

    if output.is_none() {
        println!("none");
    }
    else {
        let output = output.unwrap();

        if output.num.is_some() {
            println!("{}", output.num.unwrap());
        }
        else if output.string.is_some(){
            println!("{}", output.string.unwrap());
        }
        else {
            println!("{}", output.trueorfalse.unwrap());
        }
    }
    
    None
}



pub fn Input(node: Node) -> Option<EvalVal> {
    let mut output = "".to_string();
    let input_line = node.token_values[2].eval();

    let input_prompt;

    if input_line.is_none() {
        input_prompt = "none".to_string();
    }
    else {
        if let Some(num) = input_line.clone().unwrap().num {
            input_prompt = num.to_string();
        }
        else if let Some(string) = input_line.clone().unwrap().string{
            input_prompt = string;
        }
        else {
            input_prompt = input_line.unwrap().trueorfalse.unwrap().to_string();
        }
    }

    print!("{input_prompt}");

    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut output);

    Some(EvalVal {
        string: Some(output.replace("\n", "")),
        ..Default::default()
    })
}




pub fn AssignVar(node: Node) -> Option<EvalVal> {
    let name = node.token_values[0].eval().unwrap().string.unwrap();
    
    let val = node.token_values[2].clone().eval();


    unsafe {
        vars.insert(name, val);
    }

    None
}


//pub fn If(node: Node) -> Option<EvalVal> {}


pub fn Asstr(node: Node) -> Option<EvalVal> {
    let evalval = node.token_values[0].eval();
    

    if evalval.is_none() {
        //erererererererer
        panic!()
    }

    let val = evalval.clone().unwrap().num;
    if val.is_none() {
        if evalval.clone().unwrap().string.is_some() {
            Some(EvalVal { string: evalval.unwrap().string, ..Default::default() })
        }
        else {
            Some(EvalVal { string: Some(evalval.unwrap().trueorfalse.unwrap().to_string()), ..Default::default() })
        }
    }
    else {
        let res = val.unwrap().to_string();
        Some(EvalVal { string: Some(res), ..Default::default() })
    }
}
pub fn Asint(node: Node) -> Option<EvalVal> {
    let evalval = node.token_values[0].eval();


    if evalval.is_none() {
        //erererererererer
        panic!()
    }

    let val = evalval.clone().unwrap().string;
    if val.is_none() {
        if evalval.clone().unwrap().trueorfalse.is_some() {
            Some(EvalVal { num: match evalval.unwrap().trueorfalse.unwrap() {
                true => Some(1.),
                false => Some(0.)
            }, ..Default::default() })
        }
        else {
            Some(EvalVal { num: evalval.unwrap().num, ..Default::default() })
        }
    }
    else {
        let res = val.unwrap().trim().parse::<f32>();

        if res.is_err() {
            //erererererrererer
            panic!()
        }
        Some(EvalVal { num: Some(res.unwrap()), ..Default::default() })
    }
}


pub fn Paren(node: Node) -> Option<EvalVal> {
    let PG = get_PG();

    let mut tokens = vec![];



    for nod in node.token_values[1..node.token_values.len() - 1].to_vec() {
        tokens.push(nod.token.unwrap())
    }

    //println!("toks: {:?}", tokens);

    PG.parse(&tokens, node.line).eval()
}


pub fn Condition(node: Node) -> Option<EvalVal> {
    let val0 = node.token_values[0].eval();
    let val1 = node.token_values[2].eval();


    if val0.is_none() {
        //erererererer
        panic!()
    }
    if val1.is_none() {
        //erererererer
        panic!()
    }

    let val0 = val0.unwrap();
    let val1 = val1.unwrap();

    let is_true = match node.token_names[1].as_str() {
        "EQEQ" => val0.eq(&val1),
        "NOTEQ" => val0.ne(&val1),
        "LESS" => val0.lt(&val1),
        "LESSEQUAL" => val0.le(&val1),
        "GREATER" => val0.gt(&val1),
        "GREATEREQUAL" => val0.ge(&val1),
        other => panic!()
    };

    Some(EvalVal {trueorfalse: Some(is_true), ..Default::default()})
}


pub fn And(node: Node) -> Option<EvalVal> {
    let val1 = node.token_values[0].eval();
    let val2 =  node.token_values[2].eval();

    let true_val1 = trueorfalse_from_evalval(val1, "".to_string());
    let true_val2 = trueorfalse_from_evalval(val2, "".to_string());


    Some(EvalVal{trueorfalse: Some(true_val1 && true_val2), ..Default::default()})
}

pub fn Or(node: Node) -> Option<EvalVal> {
    let val1 = node.token_values[0].eval();
    let val2 =  node.token_values[2].eval();

    let true_val1 = trueorfalse_from_evalval(val1, "".to_string());
    let true_val2 = trueorfalse_from_evalval(val2, "".to_string());


    Some(EvalVal{trueorfalse: Some(true_val1 || true_val2), ..Default::default()})
}

pub fn Not(node: Node) -> Option<EvalVal> {
    let val = node.token_values[1].eval();

    let true_val = trueorfalse_from_evalval(val, "".to_string());


    Some(EvalVal{trueorfalse: Some(!true_val), ..Default::default()})
}




pub fn Bool(node: Node) -> Option<EvalVal> {
    Some(EvalVal {trueorfalse: Some(match node.token_names[0].as_str() {
        "TRUE" => true,
        "FALSE" => false,
        other => panic!()
    }), ..Default::default()})
}



pub fn If(node: Node) -> Option<EvalVal> {

    let trueorfalse = node.token_values[1].eval();



    let true_val = trueorfalse_from_evalval(trueorfalse, "".to_string());


    let mut lines_to_excute = vec![];
    let mut skipped_lines = 0;

    unsafe {
        let mut paren_indent = 1;
        
        for line in lines[node.line + 1..].iter() {
            skipped_lines += 1;
            if line.contains("{") {
                paren_indent += 1;
            }
            if line.contains("}") {
                paren_indent -= 1;
            }
            if paren_indent == 0 {
                break;
            }
            lines_to_excute.push(line.to_string());
        }

        
    }
    let mut val = None;

    


    let prev_lines = unsafe {
        lines.clone()
    };
    let prev_index = unsafe {
        line_index.clone()
    };
    unsafe {
        line_index = 0;
        lines = lines_to_excute.clone();
    }


    if true_val {    
        //println!("lines to execute: {:?}", lines_to_excute);
        val = interpret(lines_to_excute.clone());
    }


    unsafe {
        lines = prev_lines;
        line_index = prev_index + skipped_lines;
    }


    val
}



pub fn While(node: Node) -> Option<EvalVal> {







    let mut lines_to_excute = vec![];
    let mut skipped_lines = 0;

    unsafe {
        let mut paren_indent = 1;
        
        for line in lines[node.line + 1..].iter() {
            skipped_lines += 1;
            if line.contains("{") {
                paren_indent += 1;
            }
            if line.contains("}") {
                paren_indent -= 1;
            }
            if paren_indent == 0 {
                break;
            }
            lines_to_excute.push(line.to_string());
        }

        
    }
    let mut val = None;

    


    let mut prev_lines = unsafe {
        lines.clone()
    };
    let mut prev_index = unsafe {
        line_index.clone()
    };
    unsafe {
        line_index = 0;
        lines = lines_to_excute.clone();
    }

    let trueorfalse = node.token_values[1].eval();
    let mut true_val = trueorfalse_from_evalval(trueorfalse.clone(), "".to_string());

    //println!("condition: {:?}", node.token_values[1]);

    while true_val {  
        unsafe {
            line_index = 0;
            lines = lines_to_excute.clone();
        }
          
        let return_val = interpret(lines_to_excute.clone());

        if let Some(return_eval) = return_val {
            if let Some(break_eval) = return_eval.should_break {
                val = *break_eval;
                //println!("break eval: {:?}", val);
                break;
            }
            else if return_eval.should_return.is_some() {
                unsafe {
                    lines = prev_lines;
                    line_index = prev_index + skipped_lines;
                }
                return Some(return_eval);
            }
        }

        let trueorfalse = node.token_values[1].eval();
        //println!("condition: {:?}", trueorfalse);
        true_val = trueorfalse_from_evalval(trueorfalse.clone(), "".to_string());
    }


    unsafe {
        lines = prev_lines;
        line_index = prev_index + skipped_lines;
    }

    val
}




pub fn Break(node: Node) -> Option<EvalVal> {
    if node.token_values.len() == 1 {
        return Some(EvalVal {should_break: Some(Box::new(None)), ..Default::default()});
    }
    else {
        return Some(EvalVal {should_break: Some(Box::new(node.token_values[1].eval())), ..Default::default()});
    }
}



pub fn trueorfalse_from_evalval(evalval: Option<EvalVal>, error_message: String) -> bool {
    if evalval.is_none() {
        //erererererer
        panic!("{}", error_message);
    }

    let val = evalval.unwrap();

    let true_val;

    if let Some(num) = val.num {
        if num == 0. {
            true_val = false;
        }
        else {
            true_val = true;
        }
    }
    else if let Some(string) = val.string {
        if string.is_empty() {
            true_val = false;
        }
        else {
            true_val = true;
        }
    }
    else if let Some(trueorfalse) = val.trueorfalse {
        true_val = trueorfalse;
    }
    else {
        //ererererererer
        panic!()
    }

    true_val
}


pub fn Return(node: Node) -> Option<EvalVal> {
    if node.token_values.len() == 1 {
        return Some(EvalVal {should_return: Some(Box::new(None)), ..Default::default()});
    }
    else {
        return Some(EvalVal {should_return: Some(Box::new(node.token_values[1].eval())), ..Default::default()});
    }
}