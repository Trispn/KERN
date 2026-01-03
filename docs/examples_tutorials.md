# KERN Examples & Tutorials

## Getting Started Examples

### Hello World Example

Let's start with the simplest possible KERN program:

```kern
# hello_world.kern
rule HelloWorld:
    if 1 == 1  # This condition is always true
    then log("Hello, World!")

flow GreetingFlow {
    HelloWorld
}
```

This example demonstrates:
- Basic rule structure with `if`/`then`
- A simple flow that executes the rule
- Use of a built-in function (`log`)

### Simple Data Validation

A more practical example that validates user input:

```kern
# user_validation.kern
entity User {
    id
    name
    email
    age
}

rule ValidateUser:
    if user.id > 0 and user.name != "" and user.email != "" and user.age >= 18
    then set_valid(user, true)
    else set_valid(user, false)

constraint ValidId: user.id > 0
constraint ValidName: user.name != ""
constraint ValidEmail: user.email != ""
constraint ValidAge: user.age >= 18

flow ValidateUserFlow {
    ValidateUser
}
```

## Tutorial 1: Building a Farmer Approval System

Let's build a complete system for approving farmers in an agricultural platform. This tutorial will demonstrate all KERN constructs.

### Step 1: Define Data Structures

First, we define the entities that represent our data:

```kern
# farmer_approval.kern
entity Farmer {
    id
    name
    location
    produce
    certification
    approved
    application_date
}

entity Application {
    farmer_id
    status
    submitted_date
    review_notes
}

entity Report {
    farmer_id
    status
    timestamp
    reviewer
}
```

### Step 2: Create Validation Rules

Next, we create rules to validate farmer applications:

```kern
rule ValidateFarmer:
    if farmer.id > 0 and farmer.name != "" and farmer.location != ""
    then mark_valid(farmer)

rule CheckCertification:
    if farmer.certification == "organic" or farmer.certification == "conventional"
    then mark_certification_valid(farmer)

rule CheckApplicationAge:
    if days_since(farmer.application_date) < 365  # Less than 1 year old
    then flag_new_application(farmer)
```

### Step 3: Create Approval Logic

Now we implement the approval logic:

```kern
rule ApproveCertifiedOrganic:
    if farmer.certification == "organic" and farmer.valid == true
    then approve_farmer(farmer)

rule ApproveConventionalWithHistory:
    if farmer.certification == "conventional" and farmer.approved == true
    then confirm_approval(farmer)

rule RequireManualReview:
    if farmer.certification == "new" or farmer.location == "restricted_area"
    then flag_for_manual_review(farmer)
```

### Step 4: Create Reporting Logic

Finally, we add reporting functionality:

```kern
rule GenerateApprovalReport:
    if farmer.approved == true
    then create_report(farmer.id, "APPROVED", current_time(), "SYSTEM")

rule GenerateRejectionReport:
    if farmer.approved == false and farmer.reviewed == true
    then create_report(farmer.id, "REJECTED", current_time(), "SYSTEM")
```

### Step 5: Define Execution Flow

We tie it all together with a flow:

```kern
flow FarmerApprovalProcess {
    ValidateFarmer
    CheckCertification
    CheckApplicationAge
    ApproveCertifiedOrganic
    ApproveConventionalWithHistory
    RequireManualReview
    GenerateApprovalReport
    GenerateRejectionReport
}

constraint ValidFarmer: farmer.id > 0 and farmer.name != ""
```

### Step 6: Complete Program

Here's the complete program:

```kern
# Complete farmer_approval.kern
entity Farmer {
    id
    name
    location
    produce
    certification
    approved
    application_date
    valid
    reviewed
}

entity Report {
    farmer_id
    status
    timestamp
    reviewer
}

rule ValidateFarmer:
    if farmer.id > 0 and farmer.name != "" and farmer.location != ""
    then set_field(farmer, "valid", true)
    else set_field(farmer, "valid", false)

rule CheckCertification:
    if farmer.certification == "organic" or farmer.certification == "conventional"
    then set_field(farmer, "certification_valid", true)
    else set_field(farmer, "certification_valid", false)

rule ApproveCertifiedOrganic:
    if farmer.certification == "organic" and farmer.valid == true
    then set_field(farmer, "approved", true)

rule RequireManualReview:
    if farmer.certification == "new" or farmer.location == "restricted_area"
    then set_field(farmer, "review_required", true)

rule GenerateApprovalReport:
    if farmer.approved == true
    then create_report(farmer.id, "APPROVED", current_time(), "SYSTEM")

flow FarmerApprovalProcess {
    ValidateFarmer
    CheckCertification
    ApproveCertifiedOrganic
    RequireManualReview
    GenerateApprovalReport
}

constraint ValidFarmer: farmer.id > 0 and farmer.name != ""
```

## Tutorial 2: Inventory Management System

Let's create an inventory management system that demonstrates more complex business logic.

### Complete Inventory System

```kern
# inventory_system.kern
entity Product {
    id
    name
    category
    current_stock
    min_stock
    max_stock
    price
    supplier_id
}

entity Order {
    id
    product_id
    quantity
    status
    order_date
    delivery_date
}

entity Supplier {
    id
    name
    reliability_score
    delivery_time_days
}

# Validation rules
rule ValidateProduct:
    if product.id > 0 and product.name != "" and product.min_stock <= product.max_stock
    then mark_product_valid(product)

rule ValidateOrder:
    if order.quantity > 0 and order.product_id > 0
    then mark_order_valid(order)

# Inventory monitoring
rule CheckLowStock:
    if product.current_stock < product.min_stock
    then flag_low_stock(product)

rule CheckOverStock:
    if product.current_stock > product.max_stock
    then flag_over_stock(product)

# Ordering logic
rule GenerateRestockOrder:
    if product.current_stock < product.min_stock and product.restock_order_pending == false
    then create_restock_order(product)

rule PrioritizeUrgentRestocks:
    if product.current_stock == 0
    then mark_urgent_restock(product)

# Order processing
rule ProcessOrder:
    if order.status == "pending" and order.quantity <= product.current_stock
    then process_order_fulfillment(order)

rule CancelImpossibleOrders:
    if order.status == "pending" and order.quantity > product.current_stock
    then cancel_order(order, "INSUFFICIENT_STOCK")

# Supplier selection
rule SelectReliableSupplier:
    if product.supplier_id == 0  # No supplier assigned
    then assign_best_supplier(product)

rule EvaluateSupplierPerformance:
    if supplier.delivery_time_days < 7 and supplier.reliability_score > 80
    then mark_supplier_reliable(supplier)

# Reporting
rule GenerateLowStockReport:
    if product.low_stock_flag == true
    then add_to_low_stock_report(product)

rule GenerateSalesReport:
    if order.status == "fulfilled"
    then update_sales_metrics(order)

# Main execution flow
flow InventoryManagementFlow {
    ValidateProduct
    ValidateOrder
    CheckLowStock
    CheckOverStock
    GenerateRestockOrder
    PrioritizeUrgentRestocks
    ProcessOrder
    CancelImpossibleOrders
    SelectReliableSupplier
    EvaluateSupplierPerformance
    GenerateLowStockReport
    GenerateSalesReport
}

# Constraints
constraint ValidProduct: product.id > 0 and product.min_stock >= 0 and product.max_stock >= product.min_stock
constraint ValidOrder: order.quantity > 0 and order.product_id > 0
constraint ValidSupplier: supplier.id > 0 and supplier.reliability_score >= 0 and supplier.reliability_score <= 100
```

## Tutorial 3: Financial Transaction Processing

A more complex example showing financial transaction validation and processing:

```kern
# financial_system.kern
entity Account {
    id
    owner_id
    balance
    account_type
    status
    daily_limit
    created_date
}

entity Transaction {
    id
    from_account
    to_account
    amount
    transaction_type
    timestamp
    status
    description
}

entity Customer {
    id
    name
    risk_level
    account_count
    total_balance
}

# Account validation
rule ValidateAccount:
    if account.id > 0 and account.status == "active"
    then mark_account_valid(account)

rule CheckAccountBalance:
    if account.balance >= 0
    then mark_balance_valid(account)

# Transaction validation
rule ValidateTransaction:
    if transaction.amount > 0 and transaction.from_account > 0 and transaction.to_account > 0
    then mark_transaction_valid(transaction)

rule CheckSufficientFunds:
    if transaction.amount <= account_balance(transaction.from_account)
    then approve_funds_check(transaction)

rule CheckDailyLimit:
    if daily_spending(transaction.from_account) + transaction.amount <= account_daily_limit(transaction.from_account)
    then approve_daily_limit(transaction)

# Fraud detection
rule DetectLargeTransactions:
    if transaction.amount > 10000
    then flag_large_transaction(transaction)

rule DetectFrequentTransactions:
    if count_transactions_last_hour(transaction.from_account) > 10
    then flag_frequent_activity(transaction)

rule DetectSuspiciousPattern:
    if transaction.amount > 5000 and customer_risk_level(transaction.from_account.owner_id) == "HIGH"
    then flag_suspicious_transaction(transaction)

# Transaction processing
rule ProcessValidTransaction:
    if transaction.valid == true and transaction.funds_approved == true
    then execute_transaction(transaction)

rule UpdateAccountBalances:
    if transaction.status == "executed"
    then update_balances(transaction)

rule LogTransaction:
    if transaction.status != "pending"
    then log_transaction(transaction)

# Customer risk assessment
rule AssessCustomerRisk:
    if customer.account_count > 5 or customer.total_balance > 1000000
    then update_risk_level(customer, "MEDIUM")

rule UpdateHighRisk:
    if customer.risk_level == "MEDIUM" and suspicious_activity(customer.id)
    then update_risk_level(customer, "HIGH")

# Compliance reporting
rule GenerateComplianceReport:
    if transaction.amount > 3000
    then add_to_compliance_report(transaction)

rule FlagForReview:
    if transaction.flagged == true
    then add_to_review_queue(transaction)

# Main processing flow
flow TransactionProcessingFlow {
    ValidateAccount
    CheckAccountBalance
    ValidateTransaction
    CheckSufficientFunds
    CheckDailyLimit
    DetectLargeTransactions
    DetectFrequentTransactions
    DetectSuspiciousPattern
    ProcessValidTransaction
    UpdateAccountBalances
    LogTransaction
    AssessCustomerRisk
    UpdateHighRisk
    GenerateComplianceReport
    FlagForReview
}

# Constraints
constraint ValidAccount: account.id > 0 and account.status in ["active", "inactive", "closed"]
constraint ValidTransaction: transaction.amount > 0 and transaction.from_account != transaction.to_account
constraint ValidTransfer: account_balance(transaction.from_account) >= transaction.amount
```

## Advanced Examples

### Complex Rule with Nested Logic

```kern
# complex_logic.kern
entity Order {
    id
    customer_id
    items
    total
    status
    priority
    region
}

entity Customer {
    id
    tier
    credit_limit
    account_age_days
}

rule ComplexOrderProcessing:
    if order.total <= customer_credit_limit(order.customer_id)
    then if customer_tier(order.customer_id) == "PREMIUM"
         then if order.region == "LOCAL"
              then process_premium_local_order(order)
              else process_premium_remote_order(order)
         else if customer_account_age(order.customer_id) > 365
              then process_trusted_customer_order(order)
              else process_regular_order(order)
    else flag_for_review(order)

flow ComplexProcessingFlow {
    ComplexOrderProcessing
}
```

### Using External Functions

```kern
# external_integration.kern
entity Data {
    id
    value
    processed
}

# This rule calls external functions registered in the host system
rule ProcessWithExternal:
    if data.value > threshold()
    then enhanced_process(data, external_api_call(data.id))

rule ValidateWithDatabase:
    if database_lookup(data.id) == "VALID"
    then mark_valid(data)
    else flag_invalid(data)

flow ExternalIntegrationFlow {
    ProcessWithExternal
    ValidateWithDatabase
}
```

## Integration Examples

### Rust Integration Example

Here's how to integrate the farmer approval system into a Rust application:

```rust
use kern_vm::{VirtualMachine, Context};
use kern_parser::Parser;
use kern_graph_builder::GraphBuilder;
use kern_bytecode::BytecodeCompiler;
use serde_json::json;

fn run_farmer_approval_system() -> Result<(), Box<dyn std::error::Error>> {
    // Farmer approval KERN program
    let kern_code = r#"
        entity Farmer {
            id
            name
            location
            certification
            approved
            valid
        }

        rule ValidateFarmer:
            if farmer.id > 0 and farmer.name != "" and farmer.location != ""
            then set_field(farmer, "valid", true)
            else set_field(farmer, "valid", false)

        rule ApproveCertifiedOrganic:
            if farmer.certification == "organic" and farmer.valid == true
            then set_field(farmer, "approved", true)

        flow FarmerApprovalProcess {
            ValidateFarmer
            ApproveCertifiedOrganic
        }

        constraint ValidFarmer: farmer.id > 0 and farmer.name != ""
    "#;

    // Parse the program
    let mut parser = Parser::new(kern_code);
    let program = parser.parse_program()?;

    // Build execution graph
    let mut graph_builder = GraphBuilder::new();
    let graph = graph_builder.build_execution_graph(&program);

    // Compile to bytecode
    let mut compiler = BytecodeCompiler::new();
    let bytecode = compiler.compile_graph(&graph);

    // Create and configure VM
    let mut vm = VirtualMachine::new();
    vm.load_program(bytecode)?;

    // Set up input data
    let mut context = Context::new();
    context.set_data("farmer", json!({
        "id": 123,
        "name": "John Doe",
        "location": "California",
        "certification": "organic"
    }))?;

    vm.set_context("main", context)?;

    // Execute the program
    vm.execute()?;

    // Get results
    let result = vm.get_context_data("main", "farmer")?;
    println!("Farmer approval result: {}", result);

    Ok(())
}
```

### HTTP API Integration Example

Creating a web service that executes KERN programs:

```rust
use warp::Filter;
use serde::{Deserialize, Serialize};
use kern_vm::{VirtualMachine, Context};
use kern_parser::Parser;
use kern_graph_builder::GraphBuilder;
use kern_bytecode::BytecodeCompiler;

#[derive(Deserialize)]
struct ApprovalRequest {
    farmer: serde_json::Value,
    kern_program: String,
}

#[derive(Serialize)]
struct ApprovalResponse {
    approved: bool,
    reason: String,
    details: serde_json::Value,
}

async fn approve_farmer_handler(request: ApprovalRequest) -> Result<impl warp::Reply, warp::Rejection> {
    match execute_approval_process(&request.kern_program, request.farmer).await {
        Ok(result) => Ok(warp::reply::json(&ApprovalResponse {
            approved: result["approved"].as_bool().unwrap_or(false),
            reason: result["reason"].as_str().unwrap_or("Unknown").to_string(),
            details: result,
        })),
        Err(_) => Ok(warp::reply::json(&ApprovalResponse {
            approved: false,
            reason: "Execution error".to_string(),
            details: serde_json::Value::Null,
        })),
    }
}

async fn execute_approval_process(
    kern_code: &str,
    farmer_data: serde_json::Value
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Parse, compile, and execute as shown in previous example
    let mut parser = Parser::new(kern_code);
    let program = parser.parse_program()?;
    
    let mut graph_builder = GraphBuilder::new();
    let graph = graph_builder.build_execution_graph(&program);
    
    let mut compiler = BytecodeCompiler::new();
    let bytecode = compiler.compile_graph(&graph);
    
    let mut vm = VirtualMachine::new();
    vm.load_program(bytecode)?;
    
    let mut context = Context::new();
    context.set_data("farmer", farmer_data)?;
    vm.set_context("approval", context)?;
    
    vm.execute()?;
    
    let result = vm.get_context_data("approval", "farmer")?;
    Ok(result)
}

#[tokio::main]
async fn main() {
    let approval_route = warp::post()
        .and(warp::path("approve"))
        .and(warp::body::json())
        .and_then(approve_farmer_handler);

    warp::serve(approval_route).run(([127, 0, 0, 1], 3030)).await;
}
```

## Best Practices Examples

### Modular Design

Breaking complex systems into modules:

```kern
# validation_module.kern
entity Validation {
    target_id
    validation_type
    result
    timestamp
}

rule ValidateId:
    if validation.target_id > 0
    then set_validation_result(validation, "ID_VALID", true)

rule ValidateName:
    if validation.name != ""
    then set_validation_result(validation, "NAME_VALID", true)

flow ValidationFlow {
    ValidateId
    ValidateName
}
```

```kern
# business_logic_module.kern
# Import or reference validation results
rule ApplyBusinessLogic:
    if validation_result("ID_VALID") == true and validation_result("NAME_VALID") == true
    then proceed_with_business_logic()
```

### Error Handling Pattern

```kern
# error_handling.kern
entity ProcessResult {
    success
    value
    error_code
    error_message
}

rule SafeOperation:
    if operation_can_execute()
    then result = perform_operation()
         set_field(process_result, "success", true)
         set_field(process_result, "value", result)
    else set_field(process_result, "success", false)
         set_field(process_result, "error_code", "OPERATION_FAILED")
         set_field(process_result, "error_message", "Operation could not be executed")

rule HandleSuccess:
    if process_result.success == true
    then handle_successful_result(process_result.value)

rule HandleError:
    if process_result.success == false
    then handle_error_result(process_result.error_code, process_result.error_message)

flow ErrorHandlingFlow {
    SafeOperation
    HandleSuccess
    HandleError
}

constraint ValidResult: process_result.success == true or (process_result.error_code != "" and process_result.error_message != "")
```

## Testing Examples

### Unit Testing KERN Logic

```rust
#[cfg(test)]
mod kern_tests {
    use kern_vm::{VirtualMachine, Context};
    use kern_parser::Parser;
    use kern_graph_builder::GraphBuilder;
    use kern_bytecode::BytecodeCompiler;
    use serde_json::json;

    #[test]
    fn test_farmer_approval() -> Result<(), Box<dyn std::error::Error>> {
        let kern_code = r#"
            entity Farmer {
                id
                certification
                approved
            }

            rule ApproveOrganic:
                if farmer.certification == "organic"
                then set_field(farmer, "approved", true)

            flow TestFlow {
                ApproveOrganic
            }
        "#;

        // Compile and execute as in previous examples
        let mut parser = Parser::new(kern_code);
        let program = parser.parse_program()?;

        let mut graph_builder = GraphBuilder::new();
        let graph = graph_builder.build_execution_graph(&program);

        let mut compiler = BytecodeCompiler::new();
        let bytecode = compiler.compile_graph(&graph);

        let mut vm = VirtualMachine::new();
        vm.load_program(bytecode)?;

        let mut context = Context::new();
        context.set_data("farmer", json!({
            "id": 1,
            "certification": "organic",
            "approved": false
        }))?;
        vm.set_context("test", context)?;

        vm.execute()?;

        let result: serde_json::Value = vm.get_context_data("test", "farmer")?;
        assert_eq!(result["approved"], true);

        Ok(())
    }
}
```

These examples and tutorials demonstrate the full range of KERN's capabilities, from simple validation rules to complex business logic systems. Each example shows practical applications of KERN's constructs and provides a foundation for building real-world systems.