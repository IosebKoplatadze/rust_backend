use tonic::{Request, Response, Status, transport::Server};

// 1. Import the generated code
// The name `todo` comes from the `package todo;` line in our .proto file
pub mod todo {
    tonic::include_proto!("todo");
}
use todo::todo_service_server::{TodoService, TodoServiceServer};
use todo::{GetTodosRequest, GetTodosResponse, Todo};

// 2. Define our service structure
// This is where we would hold state, like a database connection pool.
#[derive(Debug, Default)]
pub struct MyTodoService {}

// 3. Implement the `TodoService` trait for our structure
#[tonic::async_trait]
impl TodoService for MyTodoService {
    async fn get_todos(
        &self,
        request: Request<GetTodosRequest>,
    ) -> Result<Response<GetTodosResponse>, Status> {
        println!("Received a request: {:?}", request);

        // For now, we'll return a hard-coded list of todos.
        let todos = vec![
            Todo {
                id: "1".to_string(),
                title: "Learn Rust".to_string(),
                description: "Read the Rust book.".to_string(),
                completed: false,
            },
            Todo {
                id: "2".to_string(),
                title: "Learn gRPC".to_string(),
                description: "Build a cool backend.".to_string(),
                completed: false,
            },
        ];

        let reply = GetTodosResponse { todos };

        Ok(Response::new(reply))
    }

    // We'll leave the other methods as `unimplemented` for now.
    async fn create_todo(
        &self,
        _request: Request<todo::CreateTodoRequest>,
    ) -> Result<Response<Todo>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn update_todo(
        &self,
        _request: Request<todo::UpdateTodoRequest>,
    ) -> Result<Response<Todo>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn delete_todo(
        &self,
        _request: Request<todo::DeleteTodoRequest>,
    ) -> Result<Response<todo::DeleteTodoResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}

// 4. Update our `main` function to run the server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the address for our server to listen on.
    let addr = "0.0.0.0:50051".parse()?;
    // Create a new instance of our service.
    let todo_service = MyTodoService::default();

    println!("TodoService listening on {}", addr);

    // Build the server.
    Server::builder()
        .add_service(TodoServiceServer::new(todo_service))
        .serve(addr)
        .await?;

    Ok(())
}
