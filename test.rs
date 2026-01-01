use kern_compiler::{KernCompiler, ast::nodes::AstNode};

fn main() {
    let source = r#"
entity Person {
    name: sym,
    age: num,
    active: bool
}

rule CheckAge {
    person.age >= 18
    then {
        person.adult = true
    }
}

flow ProcessData {
    step1: load_data(),
    step2: transform_data(),
    step3: save_data()
}

constraint ValidateEmail {
    user.email.contains("@")
    then {
        user.valid = true
    }
}
"#;

    let mut compiler = KernCompiler::new();
    match compiler.compile(source) {
        Some(program) => {
            println!("Compilation successful!");
            println!("Parsed {} declarations", program.declarations.len());
            
            for (i, decl) in program.declarations.iter().enumerate() {
                println!("Declaration {}: {:?}", i, decl);
            }
        }
        None => {
            println!("Compilation failed:");
            compiler.diagnostics.print_diagnostics();
        }
    }
}