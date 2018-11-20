extern crate thruster; 
extern crate futures; 

use std::boxed::Box; 
use futures::future; 
use thruster::{App, 
	BasicContext as Ctx, 
	MiddlewareChain, 
	MiddlewareReturnValue, 
	Request
	}; 
use thruster::builtins::server::Server; 
use thruster::server::ThrusterServer; 

fn hello(mut context: Ctx, 
		 _chain: &MiddlewareChain<Ctx>) 
		 -> MiddlewareReturnValue<Ctx> { 
	let val = "Hello, World!".to_owned(); 
	context.body = val; 

	Box::new(future::ok(context)) 
} 

fn main() { 
	println!("Starting server..."); 

	let mut app = App::<Request, Ctx>::new(); 

	app.get("/hello", vec![hello]); 

	let server = Server::new(app); 
	server.start("0.0.0.0", 4321); 
}