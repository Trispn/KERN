use kern_parser::{Parser, Definition};

fn main() {
    // Test with a comprehensive KERN program that includes all grammar productions
    let input = r#"
entity Farmer {
    id
    name
    location
    produce
}

entity Crop {
    id
    name
    season
    value
}

rule ValidateFarmer:
    if farmer.id != 0 and farmer.name != ""
    then mark_valid(farmer)

rule CheckLocation:
    if farmer.location == "valid"
    then approve_farmer(farmer)

rule ComplexCondition:
    if farmer.id != 0 and farmer.name != "" or farmer.location != "unknown"
    then process_farmer(farmer)

flow ProcessFarmers {
    load_farmers(),
    validate_farmers(),
    approve_valid_farmers(),
    generate_reports()
}

flow ComplexFlow {
    if value > 0 then positive_action() else negative_action(),
    loop { process_item() },
    halt
}

constraint ValidFarmerId: farmer.id > 0
constraint ValidCropValue: crop.value >= 0
constraint ValidName: farmer.name != ""
"#;

    println!("Parsing comprehensive KERN code:");
    println!("{}", input);

    let mut parser = Parser::new(input);
    match parser.parse_program() {
        Ok(program) => {
            println!("\nSuccessfully parsed {} definitions:", program.definitions.len());

            for (i, definition) in program.definitions.iter().enumerate() {
                match definition {
                    Definition::Entity(entity) => {
                        println!("  {}. Entity: {} ({} fields)", i + 1, entity.name, entity.fields.len());
                    },
                    Definition::Rule(rule) => {
                        println!("  {}. Rule: {}", i + 1, rule.name);
                    },
                    Definition::Flow(flow) => {
                        println!("  {}. Flow: {}", i + 1, flow.name);
                    },
                    Definition::Constraint(constraint) => {
                        println!("  {}. Constraint: {}", i + 1, constraint.name);
                    },
                }
            }

            if parser.get_errors().is_empty() {
                println!("\nNo parsing errors detected.");
            } else {
                println!("\nParsing completed with {} errors:", parser.get_errors().len());
                for error in parser.get_errors() {
                    println!("  - {}", error);
                }
            }
        },
        Err(errors) => {
            println!("\nParsing failed with {} errors:", errors.len());
            for error in errors {
                println!("  - {}", error);
            }
        }
    }

    // Test error handling with malformed input
    println!("\n{}", "=".repeat(50));
    println!("Testing error handling with malformed input:");
    
    let malformed_input = r#"
entity IncompleteEntity {
    id
    name
    "#;  // Missing closing brace
    
    let mut error_parser = Parser::new(malformed_input);
    match error_parser.parse_program() {
        Ok(program) => {
            println!("Parsed malformed input (unexpected): {} definitions", program.definitions.len());
            if !error_parser.get_errors().is_empty() {
                println!("But found {} errors:", error_parser.get_errors().len());
                for error in error_parser.get_errors() {
                    println!("  - {}", error);
                }
            }
        },
        Err(errors) => {
            println!("Correctly detected {} errors in malformed input:", errors.len());
            for error in errors {
                println!("  - {}", error);
            }
        }
    }
}