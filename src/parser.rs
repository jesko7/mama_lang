use std::fmt::Debug;
use crate::ast;
use crate::lexer::Token;
//use crate::ast::*
//rule.class_name, next_token_values, next_token_names, index, line


#[derive(Clone)]
pub struct Node {
    pub class_name: String,
    pub token_values: Vec<Node>,
    pub token_names: Vec<String>,
    pub index: usize,
    pub line: usize,
    pub token: Option<Token>
}


#[derive(Debug, Clone)]
pub struct EvalVal {
    pub num: Option<f32>,
    pub string: Option<String>,
    pub trueorfalse: Option<bool>,
    pub should_break: Option<Box<Option<EvalVal>>>,
    pub should_return: Option<Box<Option<EvalVal>>>,
}

impl PartialEq for EvalVal {
    fn eq(&self, other: &Self) -> bool {
        if self.num.is_some() && other.num.is_some() {
            return self.num.unwrap() == other.num.unwrap();
        }
        else if self.string.is_some() && other.string.is_some() {
            return self.string.clone().unwrap() == other.string.clone().unwrap();
        }
        else if self.trueorfalse.is_some() && other.trueorfalse.is_some() {
            return self.trueorfalse.unwrap() == other.trueorfalse.unwrap();
        }
        else {
            //erererererer
            panic!()
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for EvalVal {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        None
    }

    fn gt(&self, other: &Self) -> bool {
        if self.num.is_some() && other.num.is_some() {
            return self.num.unwrap() > other.num.unwrap();
        }
        else if self.string.is_some() && other.string.is_some() {
            return self.string.clone().unwrap() > other.string.clone().unwrap();
        }
        else if self.trueorfalse.is_some() && other.trueorfalse.is_some() {
            return self.trueorfalse.unwrap() > other.trueorfalse.unwrap();
        }
        else {
            //erererererer
            panic!()
        }
    }
    fn ge(&self, other: &Self) -> bool {
        if self.num.is_some() && other.num.is_some() {
            return self.num.unwrap() >= other.num.unwrap();
        }
        else if self.string.is_some() && other.string.is_some() {
            return self.string.clone().unwrap() >= other.string.clone().unwrap();
        }
        else if self.trueorfalse.is_some() && other.trueorfalse.is_some() {
            return self.trueorfalse.unwrap() >= other.trueorfalse.unwrap();
        }
        else {
            //erererererer
            panic!()
        }
    }
    fn lt(&self, other: &Self) -> bool {
        if self.num.is_some() && other.num.is_some() {
            return self.num.unwrap() < other.num.unwrap();
        }
        else if self.string.is_some() && other.string.is_some() {
            return self.string.clone().unwrap() < other.string.clone().unwrap();
        }
        else if self.trueorfalse.is_some() && other.trueorfalse.is_some() {
            return self.trueorfalse.unwrap() < other.trueorfalse.unwrap();
        }
        else {
            //erererererer
            panic!()
        }
    }
    fn le(&self, other: &Self) -> bool {
        if self.num.is_some() && other.num.is_some() {
            return self.num.unwrap() <= other.num.unwrap();
        }
        else if self.string.is_some() && other.string.is_some() {
            return self.string.clone().unwrap() <= other.string.clone().unwrap();
        }
        else if self.trueorfalse.is_some() && other.trueorfalse.is_some() {
            return self.trueorfalse.unwrap() <= other.trueorfalse.unwrap();
        }
        else {
            //erererererer
            panic!()
        }
    }
}

impl Default for EvalVal {
    fn default() -> Self {
        EvalVal { num: None, string: None, trueorfalse: None, should_break: None, should_return: None}
    }
}



impl Node {
    pub fn eval(&self) -> Option<EvalVal> {

        match self.class_name.as_str() {
            "Num" => ast::Num(self.clone()),
            "String" => ast::String(self.clone()),
            "Div" => ast::Div(self.clone()),
            "Mul" => ast::Mul(self.clone()),
            "Add" => ast::Add(self.clone()),
            "Pow" => ast::Pow(self.clone()),
            "Var" => ast::Var(self.clone()),
            "None" => ast::NoneNode(self.clone()),
            "Name" => ast::Name(self.clone()),
            "AssignVar" => ast::AssignVar(self.clone()),
            "Print" => ast::Print(self.clone()),
            "Input" => ast::Input(self.clone()),
            "Asstr" => ast::Asstr(self.clone()),
            "Asint" => ast::Asint(self.clone()),
            "Asfloat" => ast::Asint(self.clone()),
            "Paren" => ast::Paren(self.clone()),
            "Condition" => ast::Condition(self.clone()),
            "Or" => ast::Or(self.clone()),
            "And" => ast::And(self.clone()),
            "Not" => ast::Not(self.clone()),
            "Bool" => ast::Bool(self.clone()),
            "If" => ast::If(self.clone()),
            "While" => ast::While(self.clone()),
            "Break" => ast::Break(self.clone()),
            "Return" => ast::Return(self.clone()),
            other => panic!("node eval not implemented: {}", self.class_name)
        }
    } 
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.class_name.as_str() {
            "Add"   => write!(f, "Add({:?} + {:?})", self.token_values[0], self.token_values[2]),
            "Minus" => write!(f, "Minus({:?} - {:?})", self.token_values[0], self.token_values[2]),
            "Mul"   => write!(f, "Mul({:?} * {:?})", self.token_values[0], self.token_values[2]),
            "Div"   => write!(f, "Div({:?} / {:?})", self.token_values[0], self.token_values[2]),
            "Pow"   => write!(f, "Pow({:?} ^ {:?})", self.token_values[0], self.token_values[2]),
            "Paren" => write!(f, "Paren({:?})", self.token_values[1..self.token_values.len() - 1].to_vec()),

            "And" => write!(f, "And({:?} && {:?})", self.token_values[0], self.token_values[2]),
            "Or" => write!(f, "Or({:?} && {:?})", self.token_values[0], self.token_values[2]),
            "Not" => write!(f, "Not(!{:?})", self.token_values[1]),
            "Bool" => write!(f, "Bool({})", match self.token_names[0].as_str() {
                "TRUE" => true,
                "FALSE" => false,
                other => panic!()
            }),
            

            "Var" => write!(f, "Var({:?})", self.token_values[0]),
            "Print" => write!(f, "Print({:?})", self.token_values[2]),
            "Input" => write!(f, "Input({:?})", self.token_values[2]),
            "AssignVar" => write!(f, "AssignVar({} = {:?})", self.token_values[0].eval().unwrap().string.unwrap(), self.token_values[2]),
            
            "Asstr" => write!(f, "Asstr({:?} as str)", self.token_values[0]),
            "Asint" => write!(f, "Asint({:?} as int)", self.token_values[0]),
            "Asfloat" => write!(f, "Asfloat({:?} as float)", self.token_values[0]),

            "Condition" => write!(f, "Condition({:?} {} {:?})",
                self.token_values[0],
                match self.token_names[1].as_str() {
                    "EQEQ" => "==",
                    "GREATER" => ">",
                    "GREATEREQUAL" => ">=",
                    "LESS" => "<",
                    "LESSEQUAL" => "<=",
                    "NOTEQ" => "!=",
                    other => panic!()
                },
                self.token_values[2]
            ),
            "If" => write!(f, "IF(if {:?} then execute some code)", self.token_values[1]),
            "While" => write!(f, "While(while {:?} execute some code)", self.token_values[1]),
            "Break" => write!(f, "Break({:?})", 
            if self.token_values.len() == 1 {
                None
            }
            else {
                Some(self.token_values[1].clone())
            }
            ),

            "Return" => write!(f, "Return({:?})", 
            if self.token_values.len() == 1 {
                None
            }
            else {
                Some(self.token_values[1].clone())
            }
            ),

            "TOKEN" => write!(f, "{}", self.token.as_ref().unwrap().value()),
            
            "None" => write!(f, "None"),
            "Num"    => write!(f, "{:?}", self.token_values[0].token.as_ref().unwrap()),
            "String" => write!(f, "{:?}", self.token_values[0].token.as_ref().unwrap()),
            "Name"   => write!(f, "{:?}", self.token_values[0].token.as_ref().unwrap()),
            
            other => todo!("{other}")
        }
        
    }
}


#[derive(Debug)]
pub struct Rule
{
    pub pattern: Vec<String>,
    pub class_name: String,
    pub name: String,
    pub dotdot: bool
}

pub struct Parser
{
    pub rules: Vec<Vec<Rule>>,
    pub rules_statement: Vec<Vec<Rule>>,
    pub precedence: usize
}

impl Parser
{
    pub fn new() -> Parser {
        Parser {
            rules: vec![vec![]],
            rules_statement: vec![vec![]],
            precedence: 0
        }
    }

    pub fn add_rule(&mut self, pattern: Vec<&str>, name: &str, class_name: &str, statement: bool)
    {
        let mut pattern2 = vec![];

        for value in pattern {
            pattern2.push(value.to_string());
        }

        let name2 = name.to_string();

        let class_name2 = class_name.to_string();

        let rule = Rule {
            pattern: pattern2.clone(),
            class_name: class_name2,
            name: name2,
            dotdot: if pattern2.contains(&"..".to_string()) {
                true
            }
            else {
                false
            }
        };

        if !statement {
            self.rules[self.precedence].push(rule);
        }
        else {
            self.rules_statement[self.precedence].push(rule);
        }
        
    }

    pub fn new_precedence(&mut self) {
        self.rules.push(vec![]);
        self.rules_statement.push(vec![]);
        self.precedence += 1;
    }

    fn parse_rules(&self, mut names: Vec<String>, mut values: Vec<Node>, rules: &Vec<Rule>, line_index: usize) -> (Vec<Node>, Vec<String>){
        for rule in rules {
            let mut index = 0;
            

            if !rule.dotdot {
                if names.len() < rule.pattern.len() {
                    continue;
                }                
    
                while index <= names.len() - rule.pattern.len() {

                    let next_token_names: Vec<String> = names[index..index + rule.pattern.len()].to_vec();

                    if next_token_names == rule.pattern {
                        let next_token_values: Vec<Node> = values[index..index + rule.pattern.len()].to_vec();
    
                        let node = Node {
                            class_name: rule.class_name.clone(),
                            token_values: next_token_values,
                            token_names: next_token_names,
                            index: index,
                            line: line_index,
                            token: None
                        };
                        
                        for _ in 0..rule.pattern.len() {
                            values.remove(index);
                            names.remove(index);
                        }
                        
                        values.insert(index, node);
                        names.insert(index, rule.name.clone());
                        
                        index += 1;

                    } else {
                        index += 1;
                    }
    
    
                    if names.len() < rule.pattern.len() {
                        break;
                    }
                }
            }

            else {
                let mut dotdots = rule.pattern.split(|string: &String| string == &"..".to_string());

                let mut beforedotdot = dotdots.next().unwrap().iter().map(|x| x.clone()).collect::<Vec<String>>();
                let mut afterdotdot = dotdots.next().unwrap().iter().map(|x| x.clone()).collect::<Vec<String>>();

                beforedotdot.retain(|x| x != "..");
                afterdotdot.retain(|x| x != "..");

                if names.len() < beforedotdot.len() {
                    continue;
                }        

                while index <= names.len() - afterdotdot.len() {
                    let next_token_names: Vec<String> = names[index..index + afterdotdot.len()].to_vec();
                    let next_token_values: Vec<Node> = values[index..index + afterdotdot.len()].to_vec();

                    let index_before = index;
                    if next_token_names == beforedotdot {

                        let mut found = 1;

                        index += beforedotdot.len();
                        
                        let mut accumilatted_token_names = next_token_names;
                        let mut accumilatted_token_values = next_token_values;

                        let is_matching = loop {
                            if index >= names.len() - afterdotdot.len() {
                                break false;
                            }
                            
                            

                            let mut next_token_names: Vec<String> = names[index..index + afterdotdot.len()].to_vec();
                            let mut next_token_values: Vec<Node> = values[index..index + afterdotdot.len()].to_vec();

                            
                            let mut found_minus = false;

                            if index < names.len() - afterdotdot.len() {
                                let mut next_token_names_before: Vec<String> = names[index..index + beforedotdot.len()].to_vec();
                                let mut next_token_values_before: Vec<Node> = values[index..index + beforedotdot.len()].to_vec();
                                
                                if next_token_names_before == beforedotdot {
                                    accumilatted_token_names.append(&mut next_token_names_before);
                                    accumilatted_token_values.append(&mut next_token_values_before);
                                    found += 1;

                                    found_minus = true
                                }
                            } 
                            

                            if next_token_names == afterdotdot {
                                accumilatted_token_names.append(&mut next_token_names);
                                accumilatted_token_values.append(&mut next_token_values);
                                found -= 1;
                            }
                            else if !found_minus{
                                
                                accumilatted_token_names.push(names[index].clone());
                                accumilatted_token_values.push(values[index].clone());
                            }
                            index += 1;

                            if found == 0 {
                                break true;
                            }
                        };

                        index = index_before;
                        
                        //println!("before: {:?}, after: {:?}", beforedotdot, afterdotdot);
                        //println!("accumulated tokens: {:?}", accumilatted_token_names);

                        if is_matching {
                            let node = Node {
                                class_name: rule.class_name.clone(),
                                token_values: accumilatted_token_values,
                                token_names: accumilatted_token_names.clone(),
                                index: index,
                                line: line_index,
                                token: None
                            };

                            
                            
                            for _ in 0..accumilatted_token_names.len() {
                                values.remove(index);
                                names.remove(index);
                            }
                            
                            values.insert(index, node);
                            names.insert(index, rule.name.clone());
                        }
                    }

                    index += 1;
                }        
            }
        }


        (values, names)
    }   

    pub fn parse(&self, tokens: &Vec<Token>, line_index: usize) -> Node {
        let mut names: Vec<String> = vec![];
        let mut values: Vec<Node> = vec![];

        //put the tokens in the ast tree
        for token in tokens {
            values.push(
                Node {
                    class_name: "TOKEN".to_string(),
                    token_values: vec![],
                    token_names: vec![],
                    index: 0,
                    line: usize::MAX,
                    token: Some(token.clone())
                }
            );
            names.push(token.name());
        }

        //aply rules until it hasnt changed
        loop {
            let prev_ast = names.clone();
            
            //println!("names: {:?}", names);
            

            for rules_in_precedence in self.rules.iter() {
                
                (values, names) = self.parse_rules(names, values, rules_in_precedence, line_index);
            }


            //hasnt changed
            if names == prev_ast {
                break;
            }
        }

       

        loop {
            let prev_ast: Vec<String> = names.clone();

            //println!("names: {:?}", names);

            for rules_in_precedence in self.rules_statement.iter() {
                (values, names) = self.parse_rules(names, values, rules_in_precedence, line_index);
            }


            //hasnt changed
            if names == prev_ast {
                break;
            }
        }

        //println!("names after: {:?}", names);

        if values.len() != 1 {
            unsafe {
                panic!("syntaxt error in line: {}", ast::lines[line_index])
            }
        }

        return values[0].clone();
    }
}