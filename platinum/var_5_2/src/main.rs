use actix_web::{web, HttpResponse, HttpServer, App, HttpRequest};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct RegisterData {
    new_username: String,
    new_password: String,
}

async fn login_page() -> HttpResponse {
    HttpResponse::Ok().body(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Login Page</title>
            <link href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css" rel="stylesheet">
        </head>
        <body>
            <div class="container">
                <h1>Login Page</h1>
                <form action="/login" method="post">
                    <div class="form-group">
                        <label for="username">Username:</label>
                        <input type="text" class="form-control" id="username" name="username">
                    </div>
                    <div class="form-group">
                        <label for="password">Password:</label>
                        <input type="password" class="form-control" id="password" name="password">
                    </div>
                    <button type="submit" class="btn btn-primary">Login</button>
                </form>
            </div>
        </body>
        </html>
    "#)
}

async fn login(data: web::Form<LoginData>) -> HttpResponse {
    let user_id = &data.username;
    let password = &data.password;

    if user_id == "ititsam" && password == "12" {
        println!("Login successful for user: {}", user_id);
        let html_response = format!("{} <form action=\"/logout\" method=\"get\">\
                                     <button type=\"submit\">Logout</button></form>",
                                    format!("Welcome, {}! You are now logged in.", user_id));
        return HttpResponse::Ok().body(html_response);
    } else {
        println!("Login failed for user: {}", user_id);
        HttpResponse::Found().header("Location", "/register").finish()
    }
}

async fn register_page() -> HttpResponse {
    HttpResponse::Ok().body(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Register Page</title>
            <link href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css" rel="stylesheet">
        </head>
        <body>
            <div class="container">
                <h1>Register Page</h1>
                <form action="/register" method="post">
                    <div class="form-group">
                        <label for="new_username">New Username:</label>
                        <input type="text" class="form-control" id="new_username" name="new_username">
                    </div>
                    <div class="form-group">
                        <label for="new_password">New Password:</label>
                        <input type="password" class="form-control" id="new_password" name="new_password">
                    </div>
                    <button type="submit" class="btn btn-primary">Register</button>
                </form>
            </div>
        </body>
        </html>
    "#)
}

async fn register(data: web::Form<RegisterData>) -> HttpResponse {
    println!("Register request received: {:?}", data);
    HttpResponse::Found().header("Location", "/login").finish()
}

async fn logout(req: HttpRequest) -> HttpResponse {
    println!("Logout request received: {:?}", req);
    HttpResponse::Ok().body("Logged out successfully")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/login", web::get().to(login_page))
            .route("/login", web::post().to(login))
            .route("/register", web::get().to(register_page))
            .route("/register", web::post().to(register))
            .route("/logout", web::get().to(logout))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
