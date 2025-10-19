use tonic::Request;

// Import the generated gRPC code
pub mod todo {
    tonic::include_proto!("todo");
}
use todo::todo_service_client::TodoServiceClient;
use todo::{CreateTodoRequest, DeleteTodoRequest, GetTodosRequest, UpdateTodoRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the gRPC server
    let server_addr = "http://127.0.0.1:50051";
    println!("🔌 Connecting to TodoService at {}", server_addr);

    let mut client = TodoServiceClient::connect(server_addr).await?;
    println!("✅ Connected successfully!\n");

    // Test 1: Get all todos
    println!("📋 Test 1: Getting all todos...");
    println!("{}", "=".repeat(60));
    let request = Request::new(GetTodosRequest {});
    match client.get_todos(request).await {
        Ok(response) => {
            let todos = response.into_inner().todos;
            println!("✅ Response received:");
            println!("   Total todos: {}", todos.len());
            for (i, todo) in todos.iter().enumerate() {
                println!("\n   Todo #{}:", i + 1);
                println!("     ID: {}", todo.id);
                println!("     Title: {}", todo.title);
                println!("     Description: {}", todo.description);
                println!("     Completed: {}", todo.completed);
            }
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    println!("\n{}\n", "=".repeat(60));

    // Test 2: Create a new todo
    println!("➕ Test 2: Creating a new todo...");
    println!("{}", "=".repeat(60));
    let request = Request::new(CreateTodoRequest {
        title: "Test Todo from Client".to_string(),
        description: "This is a test todo created by the gRPC client".to_string(),
    });
    match client.create_todo(request).await {
        Ok(response) => {
            let todo = response.into_inner();
            println!("✅ Todo created successfully:");
            println!("   ID: {}", todo.id);
            println!("   Title: {}", todo.title);
            println!("   Description: {}", todo.description);
            println!("   Completed: {}", todo.completed);
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    println!("\n{}\n", "=".repeat(60));

    // Test 3: Update a todo
    println!("✏️  Test 3: Updating a todo...");
    println!("{}", "=".repeat(60));
    let request = Request::new(UpdateTodoRequest {
        id: "1".to_string(),
        title: "Updated Todo Title".to_string(),
        description: "Updated description".to_string(),
        completed: true,
    });
    match client.update_todo(request).await {
        Ok(response) => {
            let todo = response.into_inner();
            println!("✅ Todo updated successfully:");
            println!("   ID: {}", todo.id);
            println!("   Title: {}", todo.title);
            println!("   Description: {}", todo.description);
            println!("   Completed: {}", todo.completed);
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    println!("\n{}\n", "=".repeat(60));

    // Test 4: Delete a todo
    println!("🗑️  Test 4: Deleting a todo...");
    println!("{}", "=".repeat(60));
    let request = Request::new(DeleteTodoRequest {
        id: "1".to_string(),
    });
    match client.delete_todo(request).await {
        Ok(response) => {
            let result = response.into_inner();
            println!("✅ Delete operation completed:");
            println!("   Success: {}", result.success);
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    println!("\n{}\n", "=".repeat(60));

    // Test 5: Get all todos again to see changes
    println!("📋 Test 5: Getting all todos again...");
    println!("{}", "=".repeat(60));
    let request = Request::new(GetTodosRequest {});
    match client.get_todos(request).await {
        Ok(response) => {
            let todos = response.into_inner().todos;
            println!("✅ Response received:");
            println!("   Total todos: {}", todos.len());
            for (i, todo) in todos.iter().enumerate() {
                println!("\n   Todo #{}:", i + 1);
                println!("     ID: {}", todo.id);
                println!("     Title: {}", todo.title);
                println!("     Description: {}", todo.description);
                println!("     Completed: {}", todo.completed);
            }
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    println!("\n{}\n", "=".repeat(60));

    println!("🎉 All tests completed!");

    Ok(())
}
