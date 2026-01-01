use kern_parser::{Parser, Definition};

fn main() {
    // Test with a comprehensive KERN program
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

flow ProcessFarmers {
    load_farmers()
    validate_farmers()
    approve_valid_farmers()
}

constraint ValidFarmerId: farmer.id > 0
constraint ValidCropValue: crop.value >= 0
"#;

    println!("Parsing KERN code:\n{}", input);
    
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
            for error in &errors {
                println!("  - {}", error);
            }
        }
    }
    
    // Test with a simple entity
    println!("\n{}", "=".repeat(50));
    println!("Testing simple entity parsing:");
    
    let simple_input = "entity Test { field1 field2 }";
    let mut simple_parser = Parser::new(simple_input);
    match simple_parser.parse_program() {
        Ok(program) => {
            println!("Successfully parsed: {} definition(s)", program.definitions.len());
            if let Some(Definition::Entity(entity)) = program.definitions.get(0) {
                println!("Entity name: {}", entity.name);
                println!("Fields: {:?}", entity.fields.iter().map(|f| &f.name).collect::<Vec<_>>());
            }
        },
        Err(errors) => {
            println!("Error parsing simple entity:");
            for error in errors {
                println!("  - {}", error);
            }
        }
    }
}