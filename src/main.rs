use tonic::{Request, Response, Status, transport::Server};
// New imports for SQLx and Serde
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool, sqlite::SqlitePoolOptions};

// 1. Import the generated gRPC code
pub mod todo {
    tonic::include_proto!("todo");
}
use todo::todo_service_server::{TodoService, TodoServiceServer};
use todo::{GetTodosRequest, GetTodosResponse, Todo};

// 2. Define a struct for our database representation of a Todo
//    We derive `FromRow` to map database rows to this struct.
//    We derive `Serialize` and `Deserialize` for potential future use.
#[derive(Debug, FromRow, Serialize, Deserialize)]
struct DbTodo {
    id: i64,
    title: String,
    description: String,
    completed: bool,
}

// Helper to convert our database struct into the gRPC struct
impl From<DbTodo> for Todo {
    fn from(item: DbTodo) -> Self {
        Todo {
            id: item.id.to_string(), // Convert i64 to String
            title: item.title,
            description: item.description,
            completed: item.completed,
        }
    }
}

// 3. Update our service to hold a database connection pool
pub struct MyTodoService {
    pool: SqlitePool,
}

// 4. Implement the `TodoService` trait for our service
#[tonic::async_trait]
impl TodoService for MyTodoService {
    async fn get_todos(
        &self,
        request: Request<GetTodosRequest>,
    ) -> Result<Response<GetTodosResponse>, Status> {
        println!("Received a request: {:?}", request);

        // Query the database for all todos
        let db_todos =
            sqlx::query_as::<_, DbTodo>("SELECT id, title, description, completed FROM todos")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| {
                    eprintln!("Database error: {}", e);
                    Status::internal("Failed to fetch todos")
                })?;

        // Convert our database todos into gRPC todos
        let todos: Vec<Todo> = db_todos.into_iter().map(Todo::from).collect();

        let reply = GetTodosResponse { todos };
        Ok(Response::new(reply))
    }

    // --- Unimplemented methods remain the same ---
    async fn create_todo(
        &self,
        _request: Request<todo::CreateTodoRequest>,
    ) -> Result<Response<Todo>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
    // ... (update_todo and delete_todo are also unimplemented)
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

// 5. Update `main` to establish the database connection
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().expect("Failed to load .env file");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let addr = "0.0.0.0:50051".parse()?;
    // Pass the connection pool to our service
    let todo_service = MyTodoService { pool };

    println!("TodoService listening on {}", addr);

    Server::builder()
        .add_service(TodoServiceServer::new(todo_service))
        .serve(addr)
        .await?;

    Ok(())
}
