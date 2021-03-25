#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use rocket::request::Form;
use rocket::response::Debug;
use rocket::Data;
use std::{env, io};

#[derive(FromForm)]
struct UserLogin<'r> {
	username: &'r RawStr,
	password: &'r RawStr,
}

// curl http://localhost:8000
#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

// curl http://localhost:8000/hello/fritz/47
#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
	format!("Hello, {} year old named {}!", age, name)
}

// curl -X POST -d 'username=JonDoe' -d 'password=secure' http://localhost:8000/login
#[post("/login", data = "<user_form>")]
fn login(user_form: Form<UserLogin>) -> String {
	format!(
		"Hello {}! You use password {}!",
		user_form.username, user_form.password
	)
}

// curl -X POST -H "Content-Type: text/plain" --data "this is raw data" http://localhost:8000/upload
#[post("/upload", format = "plain", data = "<data>")]
fn upload(data: Data) -> Result<String, Debug<io::Error>> {
	data.stream_to_file(env::temp_dir().join("upload.txt"))
		.map(|n| n.to_string())
		.map_err(Debug)
}

fn main() {
	rocket::ignite()
		.mount("/", routes![index, hello, login, upload])
		.launch();
}
