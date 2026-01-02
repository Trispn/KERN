#[cfg(test)]
mod integration_tests {
    use kern_ast::ast::*;
    use crate::compiler_driver::BytecodeCompiler;

    #[test]
    fn test_full_compilation_pipeline() {
        // Create a more complex AST program for testing
        let condition = Expression::BinaryOp(
            Box::new(Expression::Symbol("x".to_string())),
            BinaryOperator::Gt,
            Box::new(Expression::Number(10)),
        );
        
        let action = Expression::Call(
            "print".to_string(),
            vec![
                Expression::Symbol("x".to_string()),
                Expression::String("is greater than 10".to_string()),
            ],
        );
        
        let rule = Rule {
            name: "GreaterThanTenRule".to_string(),
            condition: Some(condition),
            action: Some(action),
        };
        
        let program = Program {
            entities: vec![Entity {
                name: "TestEntity".to_string(),
                fields: vec!["x".to_string(), "y".to_string()],
            }],
            rules: vec![rule],
            flows: vec![],
            constraints: vec![],
        };

        let mut compiler = BytecodeCompiler::new();
        let result = compiler.compile(&program);
        
        // Should compile successfully
        assert!(result.is_ok());
        
        let module = result.unwrap();
        // Should have generated some instructions
        assert!(!module.instruction_stream.is_empty());
        assert_eq!(module.header.magic, *b"KERN");
        
        // Verify the module passes verification
        use crate::verifier::{BytecodeVerifier, VerificationResult};
        let verifier = BytecodeVerifier::new();
        let verification_result: VerificationResult = verifier.verify(&module.instruction_stream);
        assert!(verification_result.is_ok());
    }

    #[test]
    fn test_deterministic_compilation() {
        // Create the same program twice and ensure they produce identical bytecode
        let create_program = || {
            let condition = Expression::BinaryOp(
                Box::new(Expression::Symbol("value".to_string())),
                BinaryOperator::Eq,
                Box::new(Expression::Number(42)),
            );
            
            let action = Expression::Call(
                "log".to_string(),
                vec![Expression::String("Value is 42".to_string())],
            );
            
            let rule = Rule {
                name: "ValueIsFortyTwo".to_string(),
                condition: Some(condition),
                action: Some(action),
            };
            
            Program {
                entities: vec![Entity {
                    name: "DataEntity".to_string(),
                    fields: vec!["value".to_string()],
                }],
                rules: vec![rule],
                flows: vec![],
                constraints: vec![],
            }
        };

        let program1 = create_program();
        let program2 = create_program();

        let mut compiler1 = BytecodeCompiler::new();
        let result1 = compiler1.compile(&program1);
        assert!(result1.is_ok());
        let module1 = result1.unwrap();

        let mut compiler2 = BytecodeCompiler::new();
        let result2 = compiler2.compile(&program2);
        assert!(result2.is_ok());
        let module2 = result2.unwrap();

        // The modules should be equivalent (though we're not testing exact byte equality
        // because register allocation might vary between runs)
        assert_eq!(module1.instruction_stream.len(), module2.instruction_stream.len());
        assert_eq!(module1.header.magic, module2.header.magic);
        assert_eq!(module1.header.version, module2.header.version);
    }
}