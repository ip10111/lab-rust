use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use tera::{Tera, Context};
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryParam {
    greet: Option<String>,
    name: Option<String>,
}

async fn greet(tmpl: web::Data<Tera>, query: web::Query<QueryParam>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("name", "World Sama");

    let greet = query.greet.clone().unwrap_or_else(|| "Hello".to_string());
    let name = query.name.clone().unwrap_or_else(|| "World".to_string());
    ctx.insert("greet", &format!("I say, {}", &greet));
    ctx.insert("name", &name);           
    let html = tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // DEBUG PATH
    // Dynamically set the templates directory path
    // let templates_path = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*");
    // Print the path to the console
    // println!("Templates path: {}", templates_path);

    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
 
    // Print the server URL
    println!("🚀 Server is running at http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(greet))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

