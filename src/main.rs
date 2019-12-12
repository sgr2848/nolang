mod cli;
mod interpreter;
mod lexer;
mod mapper;
mod node;
// mod interpreter;

use std::collections::{HashMap, VecDeque};
use std::io::{stdin, stdout, Write};
use std::string::String;
fn main() {
    /*TODO Main
            []make hmap for id and mapstruct
            []]make interpreter id validation
    */
    //2-3+88*90/2

    // dbg!(new_tree);

    match cli::start_nano() {
        Ok(a) => println!("{}", a),
        Err(e) => println!("{}", e),
    }

    let mut new_f = interpreter::Interpret {
        id_map: mapper::MapVec::init(),
    };
    &mut new_f.id_map.insert_val(
        String::from("Hello"),
        vec!["2".to_string()].into_iter().collect(),
    );
    loop {
        print!(">> ");
        let mut input_string = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input_string);
        if let Some('\n') = input_string.chars().next_back() {
            input_string.pop();
            input_string.pop();

            if input_string == "exit" || input_string == ":q" {
                break;
            } else if input_string.is_empty() {
                print!("");
            } else {
                let _a_token = lexer::Lexer {
                    a_value: &input_string,
                };
                let mut token_vector: VecDeque<String> = _a_token.get_token_queue();
                let rar = &mut new_f.check_validity(&mut token_vector);
                for (keys, value) in new_f.clone().id_map.data {
                    println!(" the keys-->{:?}", keys.get_value())
                }
                if *rar {
                    if lexer::no_id(&token_vector) {
                        let mut red = lexer::ConvertInfix {
                            top: -1,
                            some_string: token_vector.clone(),
                            stack: vec!["$".to_string()],
                        }
                        .to_postfix();
                        let mut pt = interpreter::PTVec::new_tree();
                        let mut new_tree = interpreter::build_pt(pt, red);
                        let length: usize = new_tree.clone().nodes.len();
                        interpreter::evaluate_pt(&mut new_tree, node::NodeId { index: length - 1 });
                        loop {
                            token_vector.pop_back();
                            if new_tree.nodes.len() > 1 {
                                new_tree.nodes.pop_front();
                            } else {
                                for i in new_tree.clone().iter() {
                                    println!("the value is {:}", i.clone().get_data().unwrap());
                                }
                                break;
                            }
                        }
                    }
                } else {
                    println!("***********invalid or unintialized syntax somewhere********");
                }
            }
            println!("");
        }
    }
}
