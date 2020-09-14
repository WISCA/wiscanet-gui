#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

mod edgenode;
mod application;

use rocket::Rocket;
use rocket::fairing::AdHoc;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::{templates::Template, serve::StaticFiles};
use diesel::SqliteConnection;

use edgenode::{Edgenode, Node};
use application::{Application, App};

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

#[derive(Debug, Serialize)]
struct Context<'a, 'b>{ msg: Option<(&'a str, &'b str)>, edgenodes: Vec<Edgenode>, applications: Vec<Application>}

impl<'a, 'b> Context<'a, 'b> {
    pub fn err(conn: &DbConn, msg: &'a str) -> Context<'static, 'a> {
        Context{msg: Some(("error", msg)), edgenodes: Edgenode::all(conn), applications: Application::all(conn)}
    }

    pub fn raw(conn: &DbConn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context{msg: msg, edgenodes: Edgenode::all(conn), applications: Application::all(conn)}
    }
}

#[post("/", data = "<node_form>")]
fn new(node_form: Form<Node>, conn: DbConn) -> Flash<Redirect> {
    let node = node_form.into_inner();
    if node.name.is_empty() {
        Flash::error(Redirect::to("/"), "Name cannot be empty.")
    } else if node.ipaddr.is_empty() {
        Flash::error(Redirect::to("/"), "IP Address cannot be empty.")
    } else if node.radio_type.is_empty() {
        Flash::error(Redirect::to("/"), "Radio Type cannot be empty.")
    } else if Edgenode::insert(node, &conn) {
        Flash::success(Redirect::to("/"), "Edge Node successfully added.")
    } else {
        Flash::error(Redirect::to("/"), "Whoops! The server failed.")
    }
}

#[delete("/<id>")]
fn delete(id: i32, conn: DbConn) -> Result<Flash<Redirect>, Template> {
    if Edgenode::delete_with_id(id, &conn) {
        Ok(Flash::success(Redirect::to("/"), "Edge Node was deleted."))
    } else {
        Err(Template::render("index", &Context::err(&conn, "Couldn't delete Edge Node.")))
    }
}

#[post("/", data = "<app_form>")]
fn new_app(app_form: Form<App>, conn: DbConn) -> Flash<Redirect> {
    let app = app_form.into_inner();
    if app.name.is_empty(){
        Flash::error(Redirect::to("/"), "Name cannot be empty.")
    } else if Application::insert(app, &conn) {
        Flash::success(Redirect::to("/"), "Application successfully added.")
    } else {
        Flash::error(Redirect::to("/"), "Whoops! The server failed.")
    }
}

#[delete("/<id>")]
fn delete_app(id: i32, conn: DbConn) -> Result<Flash<Redirect>, Template> {
    if Application::delete_with_id(id, &conn) {
        Ok(Flash::success(Redirect::to("/"), "Application was deleted."))
    } else {
        Err(Template::render("index", &Context::err(&conn, "Couldn't delete Application.")))
    }
}

#[get("/")]
fn index(msg: Option<FlashMessage>, conn: DbConn) -> Template {
    Template::render("index", &match msg {
        Some(ref msg) => Context::raw(&conn, Some((msg.name(), msg.msg()))),
        None => Context::raw(&conn, None),
    })
}

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = DbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index])
        .mount("/node", routes![new, delete])
        .mount("/app", routes![new_app, delete_app])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
