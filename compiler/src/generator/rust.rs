use crate::parser::{Node, Program};

pub fn gen(ast: &Program) -> String {
    let (funcs, code) = convert_branch(&ast.body);

    format!("fn main(){{{}}} {}", code.join(" "), funcs.join(" "))
}

fn convert_branch(nodes: &Vec<Node>) -> (Vec<String>, Vec<String>) {
    let mut functions: Vec<String> = Vec::new();
    let mut main_loop: Vec<String> = Vec::new();

    let mut current = 0;
    while current < nodes.len() {
        let node = &nodes[current];

        let code: String = match node {
            Node::StringLiteral(val) => format!("\"{val}\""),
            Node::NumberLiteral(val) => val.to_string(),
            Node::CallExpression(call) => match &call.name[..] {
                "📣" => {
                    let (i_func, i_main) = convert_branch(&call.params);
                    functions.extend(i_func);
                    format!(
                        "println!(\"{}\", {});",
                        "{}".repeat(call.params.len()),
                        i_main.join(", ")
                    )
                }
                "➕" => {
                    let (i_func, i_main) = convert_branch(&call.params);
                    functions.extend(i_func);
                    format!("{}", i_main.join(" + "))
                }
                &_ => "".to_string(),
            },
            _ => String::new(),
        };
        main_loop.push(code);

        current += 1;
    }

    (functions, main_loop)
}
