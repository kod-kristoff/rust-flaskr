extern crate thruster;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate smallvec;
extern crate tokio;
extern crate dotenv;
extern crate chrono;

#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;

mod context;
mod schema;
mod models;

use futures::future;

use thruster::{App, MiddlewareChain, MiddlewareReturnValue};
use thruster::builtins::server::Server;
use thruster::server::ThrusterServer;
use context::{Ctx, generate_context};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use schema::posts::dsl::*;
use dotenv::dotenv;
use std::env;

lazy_static! {
    static ref db: Pool<ConnectionManager<SqliteConnection>> = {
      dotenv().ok();

      let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

      let manager = ConnectionManager::<SqliteConnection>::new(database_url);
      Pool::new(manager).expect("Error creating db pool")
    };
}

fn fetch_value(mut context: Ctx,
               _chain: &MiddlewareChain<Ctx>)
              -> MiddlewareReturnValue<Ctx> {
  let conn = db.get().unwrap();

  let results = posts
    .limit(1)
    .load::<models::Post>(&conn)
    .unwrap();

  let result = results.get(0).unwrap();
  context.body = result.title.clone();

  Box::new(future::ok(context))
}

fn register(mut context: Ctx,
            _chain: &MiddlewareChain<Ctx>)
            -> MiddlewareReturnValue<Ctx> {
    context.body = "<html>register stub</html>".to_owned();
    context.set_header("Content-Type", "text/html");

    Box::new(future::ok(context))
}

fn not_found_404(mut context: Ctx,
                 _chain: &MiddlewareChain<Ctx>)
                 -> MiddlewareReturnValue<Ctx> {
  context.body = "<html>
  ( ͡° ͜ʖ ͡°) What're you looking for here?
</html>".to_owned();
  context.set_header("Content-Type", "text/html");
  context.status_code = 404;

  Box::new(future::ok(context))
}

fn main() {
  println!("Starting server...");

  let mut app = App::create(generate_context);

  let mut auth_app = App::create(generate_context);
  auth_app.get("/register", vec![register]);
  // auth_app.post("/register", vec![do_register]);

  app.get("/plaintext", vec![fetch_value]);
  app.use_sub_app("/auth", auth_app);
  app.get("/*", vec![not_found_404]);

  let server = Server::new(app);
  server.start("0.0.0.0", 4321);
}
