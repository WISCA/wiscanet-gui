#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

mod application;
mod edgenode;

use diesel::SqliteConnection;
use rocket::fairing::AdHoc;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket::Rocket;
use rocket_contrib::{json::Json, serve::StaticFiles, templates::Template};

use std::fs::File;
use std::io::prelude::*;

use dirs::home_dir;

use application::{App, Application};
use edgenode::{Edgenode, Node};

// Config Generation
#[derive(Debug, Deserialize)]
pub struct ConfigPair {
    pub node_id: String,
    pub node_name: String,
    pub app_id: String,
    pub app_name: String,
    pub logic_id: i32,
    pub tx_gain: String,
    pub rx_gain: String,
    pub antenna: String,
    pub subdev: String,
    pub num_chans: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigFile {
    pub logic_id: i32,
    pub op_mode: String,
    pub mac_mode: String,
    pub time_slot: i32,
    pub lang: String,
    pub matlab_dir: String,
    pub matlab_func: String,
    pub matlab_log: String,
    pub num_samples: i32,
    pub sample_rate: f32,
    pub subdevice: String,
    pub freq: f32,
    pub tx_gain: f32,
    pub rx_gain: f32,
    pub bandwidth: f32,
    pub device_addr: String,
    pub channels: String,
    pub antennas: String,
}

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

#[derive(Debug, Serialize)]
struct Context<'a, 'b> {
    msg: Option<(&'a str, &'b str)>,
    edgenodes: Vec<Edgenode>,
    applications: Vec<Application>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn err(conn: &DbConn, msg: &'a str) -> Context<'static, 'a> {
        Context {
            msg: Some(("error", msg)),
            edgenodes: Edgenode::all(conn),
            applications: Application::all(conn),
        }
    }

    pub fn raw(conn: &DbConn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context {
            msg: msg,
            edgenodes: Edgenode::all(conn),
            applications: Application::all(conn),
        }
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
        Err(Template::render(
            "index",
            &Context::err(&conn, "Couldn't delete Edge Node."),
        ))
    }
}

#[post("/", data = "<app_form>")]
fn new_app(app_form: Form<App>, conn: DbConn) -> Flash<Redirect> {
    let app = app_form.into_inner();
    if app.name.is_empty() {
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
        Ok(Flash::success(
            Redirect::to("/"),
            "Application was deleted.",
        ))
    } else {
        Err(Template::render(
            "index",
            &Context::err(&conn, "Couldn't delete Application."),
        ))
    }
}

#[get("/")]
fn index(msg: Option<FlashMessage>, conn: DbConn) -> Template {
    Template::render(
        "index",
        &match msg {
            Some(ref msg) => Context::raw(&conn, Some((msg.name(), msg.msg()))),
            None => Context::raw(&conn, None),
        },
    )
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

#[post("/", data = "<gen_config_form>")]
fn gen_configuration(gen_config_form: Json<Vec<ConfigPair>>, conn: DbConn) -> Flash<Redirect> {
    let conf = gen_config_form.into_inner();
    // Let's find the actual objects associated with each of these id's
    // We are going to set up a list of pairs of (Node, Application)
    let node_app_map: Vec<(Edgenode, Application, i32, f32, f32, String, String, i32)> = conf
        .iter()
        .map(|t| {
            (
                Edgenode::get_with_id(t.node_id.parse::<i32>().unwrap(), &conn).unwrap(),
                Application::get_with_id(t.app_id.parse::<i32>().unwrap(), &conn).unwrap(),
                t.logic_id,
                t.tx_gain.parse::<f32>().unwrap(),
                t.rx_gain.parse::<f32>().unwrap(),
                t.subdev.clone(),
                t.antenna.clone(),
                t.num_chans.parse::<i32>().unwrap(),
            )
        })
        .collect();

    // Now that we have a list of pairs, we are going to do some stuff with them
    //println!("Node App Map (Rust): {:#?}", node_app_map);

    // First we will generate the "iplist" file at ~/wdemo/run/usr/cfg/iplist
    // On each line it contains the ip address of the nodes in the configuration
    // Algorithm: foreach in node_app_map, get node ip, println into file

    // Next we will generate the usrconfig_$ipaddr.yml files (also found in ~/wdemo/run/usr/cfg/)
    // Algorithm: foreach in node_app_map, write out configuration combined from the application
    // and node into yml file
    let home_path = home_dir().unwrap();
    // Set up file writes
    let iplist_path = home_path.join("wdemo/run/usr/cfg/iplist");
    let mut file = match File::create(&iplist_path) {
        Err(why) => panic!("Couldn't create {}: {}", iplist_path.display(), why),
        Ok(file) => file,
    };
    let mut iplist_string = "".to_string();
    // We will do these in one loop to save time iterating over the data structure
    for conf_pair in node_app_map {
        let pair_ip = conf_pair.0.ipaddr;
        iplist_string.push_str(&format!("{}\n", pair_ip));
        let mut usrconfig_string = "wdemo/run/usr/cfg/usrconfig_".to_string();
        usrconfig_string.push_str(&format!("{}.yml", pair_ip));
        let usrconfig_path = home_path.join(&usrconfig_string);
        let mut usrconfig_file = match File::create(&usrconfig_path) {
            Err(why) => panic!("Couldn't create {}: {}", usrconfig_path.display(), why),
            Ok(file) => file,
        };
        let usrconfig = ConfigFile {
            logic_id: conf_pair.2,
            op_mode: conf_pair.1.op_mode,
            mac_mode: conf_pair.1.mac_mode,
            time_slot: conf_pair.2,
            lang: conf_pair.1.lang,
            matlab_dir: conf_pair.1.matlab_dir,
            matlab_func: conf_pair.1.matlab_func,
            matlab_log: conf_pair.1.matlab_log,
            num_samples: conf_pair.1.num_samples,
            subdevice: conf_pair.5,
            sample_rate: conf_pair.1.sample_rate,
            freq: conf_pair.1.freq,
            tx_gain: conf_pair.3,
            rx_gain: conf_pair.4,
            bandwidth: conf_pair.1.bw,
            device_addr: conf_pair.0.radio_address,
            channels: vec!["\"0\"", "\"0,1\"", "\"0,1,2\"", "\"0,1,2,3\""]
                .get((conf_pair.7 - 1) as usize)
                .unwrap()
                .to_string(),
            antennas: conf_pair.6,
        };

        let usrconfig_string = serde_yaml::to_string(&usrconfig);
        match usrconfig_file.write_all(usrconfig_string.unwrap().as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", usrconfig_path.display(), why),
            Ok(_) => println!("Succesfully wrote to {}", usrconfig_path.display()),
        }
    }

    // Write out IP List file
    match file.write_all(iplist_string.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", iplist_path.display(), why),
        Ok(_) => println!("Succesfully wrote to {}", iplist_path.display()),
    }

    Flash::success(Redirect::to("/"), "Configuration Successfully Generated.")
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index])
        .mount("/node", routes![new, delete])
        .mount("/app", routes![new_app, delete_app])
        .mount("/genconfig", routes![gen_configuration])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
