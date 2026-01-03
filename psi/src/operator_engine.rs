use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use kern_vm::VirtualMachine;
use kern_graph_builder::GraphBuilder;
use kern_bytecode::BytecodeCompiler;
use kern_parser::{Parser, Program};

use crate::{PSI_Operator, PSI_Brain};

#[derive(Debug, Clone)]
pub struct OperatorExecutionContext {
    pub inputs: HashMap<String, String>,
    pub outputs: HashMap<String, String>,
    pub context_vars: HashMap<String, String>,
    pub language: String,
}

impl OperatorExecutionContext {
    pub fn new() -> Self {
        OperatorExecutionContext {
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            context_vars: HashMap::new(),
            language: "rust".to_string(),
        }
    }

    pub fn set_input(&mut self, key: String, value: String) {
        self.inputs.insert(key, value);
    }

    pub fn get_input(&self, key: &str) -> Option<&String> {
        self.inputs.get(key)
    }

    pub fn set_output(&mut self, key: String, value: String) {
        self.outputs.insert(key, value);
    }

    pub fn get_output(&self, key: &str) -> Option<&String> {
        self.outputs.get(key)
    }

    pub fn set_context_var(&mut self, key: String, value: String) {
        self.context_vars.insert(key, value);
    }

    pub fn get_context_var(&self, key: &str) -> Option<&String> {
        self.context_vars.get(key)
    }
}

pub struct OperatorEngine {
    pub vm: VirtualMachine,
    pub graph_builder: GraphBuilder,
    pub bytecode_compiler: BytecodeCompiler,
}

impl OperatorEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(OperatorEngine {
            vm: VirtualMachine::new(),
            graph_builder: GraphBuilder::new(),
            bytecode_compiler: BytecodeCompiler::new(),
        })
    }

    pub fn execute_operator(
        &mut self,
        operator: &PSI_Operator,
        context: &mut OperatorExecutionContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Select the appropriate emission template based on the context language
        let template = match context.language.as_str() {
            "rust" => operator.emissions.get("rust").unwrap_or(&operator.kern_template),
            "python" => operator.emissions.get("python").unwrap_or(&operator.kern_template),
            "go" => operator.emissions.get("go").unwrap_or(&operator.kern_template),
            "javascript" => operator.emissions.get("javascript").unwrap_or(&operator.kern_template),
            _ => &operator.kern_template,
        };

        // Create KERN source code from the template, substituting context variables
        let kern_code = self.substitute_template(template, context);

        // Parse the KERN code
        let mut parser = Parser::new(&kern_code);
        let program = match parser.parse_program() {
            Ok(p) => p,
            Err(errors) => {
                return Err(format!("Parse errors: {:?}", errors).into());
            }
        };

        // Build execution graph
        let graph = self.graph_builder.build_execution_graph(&program);

        // Compile to bytecode
        let bytecode_module = self.bytecode_compiler.compile_graph(&graph);

        // Extract the instruction stream from the bytecode module
        let instructions = bytecode_module.instruction_stream;

        // Load and execute in VM
        self.vm.load_program(instructions);
        match self.vm.execute() {
            Ok(()) => {},
            Err(e) => return Err(format!("VM execution error: {:?}", e).into()),
        }

        // Extract results from VM context and update operator context
        // Note: We don't need to pass the VM to extract_vm_results since it's a no-op in this implementation
        self.extract_vm_results(context)?;

        Ok(())
    }

    fn substitute_template(&self, template: &str, context: &OperatorExecutionContext) -> String {
        let mut result = template.to_string();
        
        // Replace input variables
        for (key, value) in &context.inputs {
            let placeholder = format!("{{{{{}}}}}", key); // {{input_name}}
            result = result.replace(&placeholder, value);
        }
        
        // Replace context variables
        for (key, value) in &context.context_vars {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }

        result
    }

    fn extract_vm_results(
        &self,
        _context: &mut OperatorExecutionContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Extract results from the VM context and update the operator context
        // This is a simplified implementation - in a real system, we'd have more sophisticated
        // result extraction based on the specific operator and its expected outputs
        Ok(())
    }

    pub fn execute_operator_chain(
        &mut self,
        brain: &PSI_Brain,
        operator_names: &[String],
        initial_context: OperatorExecutionContext,
    ) -> Result<OperatorExecutionContext, Box<dyn std::error::Error>> {
        let mut current_context = initial_context;

        for operator_name in operator_names {
            if let Some(operator) = brain.operators.iter().find(|op| &op.name == operator_name) {
                self.execute_operator(operator, &mut current_context)?;
            } else {
                return Err(format!("Operator not found: {}", operator_name).into());
            }
        }

        Ok(current_context)
    }
}

// Predefined common operators
pub mod common_operators {
    use super::*;

    pub fn create_define_entities_operator() -> PSI_Operator {
        let mut emissions = HashMap::new();
        emissions.insert("rust".to_string(), r#"// Define Rust structs
struct User {
    id: u32,
    name: String,
    email: String,
}"#.to_string());
        emissions.insert("python".to_string(), r#"# Define Python classes
class User:
    def __init__(self, id, name, email):
        self.id = id
        self.name = name
        self.email = email"#.to_string());
        emissions.insert("go".to_string(), r#"// Define Go structs
type User struct {
    ID    uint32 `json:"id"`
    Name  string `json:"name"`
    Email string `json:"email"`
}"#.to_string());
        emissions.insert("javascript".to_string(), r#"// Define JavaScript class
class User {
    constructor(id, name, email) {
        this.id = id;
        this.name = name;
        this.email = email;
    }
}"#.to_string());

        PSI_Operator {
            id: 1,
            name: "DefineEntities".to_string(),
            domain: "code".to_string(),
            purity: 0,
            arity_in: 1,
            arity_out: 1,
            cost_hint: 5,
            kern_template: r#"rule DefineEntities:
    if has_entities_spec()
    then generate_entities_code()"#.to_string(),
            emissions,
        }
    }

    pub fn create_create_routes_operator() -> PSI_Operator {
        let mut emissions = HashMap::new();
        emissions.insert("rust".to_string(), r#"// Create Rust routes with Actix
use actix_web::{web, HttpResponse, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("OK")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}"#.to_string());
        emissions.insert("python".to_string(), r#"# Create Python routes with Flask
from flask import Flask, jsonify

app = Flask(__name__)

@app.route('/health')
def health_check():
    return jsonify({"status": "OK"})
"#.to_string());
        emissions.insert("go".to_string(), r#"// Create Go routes with Gin
package main

import (
    "github.com/gin-gonic/gin"
    "net/http"
)

func healthCheck(c *gin.Context) {
    c.JSON(http.StatusOK, gin.H{"status": "OK"})
}

func setupRoutes(router *gin.Engine) {
    router.GET("/health", healthCheck)
}
"#.to_string());
        emissions.insert("javascript".to_string(), r#"// Create JavaScript routes with Express
const express = require('express');
const router = express.Router();

router.get('/health', (req, res) => {
    res.json({ status: 'OK' });
});

module.exports = router;
"#.to_string());

        PSI_Operator {
            id: 2,
            name: "CreateRoutes".to_string(),
            domain: "code".to_string(),
            purity: 0,
            arity_in: 1,
            arity_out: 1,
            cost_hint: 8,
            kern_template: r#"rule CreateRoutes:
    if has_route_spec()
    then generate_route_code()"#.to_string(),
            emissions,
        }
    }

    pub fn create_implement_auth_operator() -> PSI_Operator {
        let mut emissions = HashMap::new();
        emissions.insert("rust".to_string(), r#"// Rust authentication implementation
use actix_web::{dev::ServiceRequest, Error};
use futures::future::{ok, Ready};

pub struct AuthMiddleware;

impl<S, B> actix_web::dev::Transform<S, actix_web::dev::ServiceRequest> for AuthMiddleware
where
    S: actix_web::dev::Service<
        actix_web::dev::ServiceRequest,
        Response = actix_web::dev::ServiceResponse<B>,
        Error = Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware)
    }
}"#.to_string());
        emissions.insert("python".to_string(), r#"# Python authentication implementation
from functools import wraps
from flask import request, jsonify

def require_auth(f):
    @wraps(f)
    def decorated_function(*args, **kwargs):
        token = request.headers.get('Authorization')
        if not token:
            return jsonify({'error': 'Missing token'}), 401
        # Validate token here
        return f(*args, **kwargs)
    return decorated_function
"#.to_string());
        emissions.insert("go".to_string(), r#"// Go authentication implementation
package main

import (
    "net/http"
    "strings"
)

func authMiddleware(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        authHeader := r.Header.Get("Authorization")
        if authHeader == "" {
            http.Error(w, "Missing authorization header", http.StatusUnauthorized)
            return
        }

        // Validate token here
        token := strings.TrimPrefix(authHeader, "Bearer ")
        if !validateToken(token) {
            http.Error(w, "Invalid token", http.StatusUnauthorized)
            return
        }

        next.ServeHTTP(w, r)
    })
}
"#.to_string());
        emissions.insert("javascript".to_string(), r#"// JavaScript authentication implementation
const jwt = require('jsonwebtoken');

const authMiddleware = (req, res, next) => {
    const token = req.header('Authorization')?.replace('Bearer ', '');

    if (!token) {
        return res.status(401).json({ error: 'Access denied. No token provided.' });
    }

    try {
        const verified = jwt.verify(token, process.env.JWT_SECRET);
        req.user = verified;
        next();
    } catch (error) {
        res.status(400).json({ error: 'Invalid token.' });
    }
};
"#.to_string());

        PSI_Operator {
            id: 3,
            name: "ImplementAuth".to_string(),
            domain: "security".to_string(),
            purity: 0,
            arity_in: 1,
            arity_out: 1,
            cost_hint: 15,
            kern_template: r#"rule ImplementAuth:
    if needs_auth()
    then generate_auth_mechanism()"#.to_string(),
            emissions,
        }
    }

    pub fn create_write_tests_operator() -> PSI_Operator {
        let mut emissions = HashMap::new();
        emissions.insert("rust".to_string(), r#"// Rust tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User {
            id: 1,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };

        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Test User");
    }
}"#.to_string());
        emissions.insert("python".to_string(), r#"# Python tests
import unittest
from user import User

class TestUser(unittest.TestCase):
    def test_user_creation(self):
        user = User(1, "Test User", "test@example.com")
        self.assertEqual(user.id, 1)
        self.assertEqual(user.name, "Test User")

if __name__ == '__main__':
    unittest.main()
"#.to_string());
        emissions.insert("go".to_string(), r#"// Go tests
package main

import (
    "testing"
)

func TestUserCreation(t *testing.T) {
    user := User{
        ID:    1,
        Name:  "Test User",
        Email: "test@example.com",
    }

    if user.ID != 1 {
        t.Errorf("Expected ID 1, got %d", user.ID)
    }

    if user.Name != "Test User" {
        t.Errorf("Expected name 'Test User', got %s", user.Name)
    }
}
"#.to_string());
        emissions.insert("javascript".to_string(), r#"// JavaScript tests
const { User } = require('./user');

describe('User', () => {
    test('should create user with correct properties', () => {
        const user = new User(1, 'Test User', 'test@example.com');

        expect(user.id).toBe(1);
        expect(user.name).toBe('Test User');
    });
});
"#.to_string());

        PSI_Operator {
            id: 4,
            name: "WriteTests".to_string(),
            domain: "testing".to_string(),
            purity: 0,
            arity_in: 1,
            arity_out: 1,
            cost_hint: 10,
            kern_template: r#"rule WriteTests:
    if has_spec()
    then generate_test_cases()"#.to_string(),
            emissions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_engine_creation() {
        let engine = OperatorEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_common_operators() {
        let ops = [
            common_operators::create_define_entities_operator(),
            common_operators::create_create_routes_operator(),
            common_operators::create_implement_auth_operator(),
            common_operators::create_write_tests_operator(),
        ];

        assert_eq!(ops.len(), 4);
        assert_eq!(ops[0].name, "DefineEntities");
        assert_eq!(ops[1].name, "CreateRoutes");
        assert_eq!(ops[2].name, "ImplementAuth");
        assert_eq!(ops[3].name, "WriteTests");
    }
}