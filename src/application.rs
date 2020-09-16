use diesel::{self, prelude::*};

#[path = "./schema.rs"]
mod schema;

use schema::applications;
use schema::applications::dsl::applications as all_applications;

#[table_name = "applications"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Application {
    pub id: Option<i32>,
    pub name: String,
    pub op_mode: String,
    pub mac_mode: String,
    pub lang: String,
    pub matlab_dir: String,
    pub matlab_func: String,
    pub matlab_log: String,
    pub num_samples: i32,
    pub sample_rate: f32,
    pub freq: f32,
    pub bw: f32,
}

#[derive(FromForm)]
pub struct App {
    pub name: String,
    pub op_mode: String,
    pub mac_mode: String,
    pub lang: String,
    pub matlab_dir: String,
    pub matlab_func: String,
    pub matlab_log: String,
    pub num_samples: i32,
    pub sample_rate: f32,
    pub freq: f32,
    pub bw: f32,
}

impl Application {
    pub fn all(conn: &SqliteConnection) -> Vec<Application> {
        all_applications
            .order(applications::id.desc())
            .load::<Application>(conn)
            .unwrap()
    }

    pub fn insert(app: App, conn: &SqliteConnection) -> bool {
        let a = Application {
            id: None,
            name: app.name,
            op_mode: app.op_mode,
            mac_mode: app.mac_mode,
            lang: app.lang,
            matlab_dir: app.matlab_dir,
            matlab_func: app.matlab_func,
            matlab_log: app.matlab_log,
            num_samples: app.num_samples,
            sample_rate: app.sample_rate,
            freq: app.freq,
            bw: app.bw,
        };
        diesel::insert_into(applications::table)
            .values(&a)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_applications.find(id))
            .execute(conn)
            .is_ok()
    }

    pub fn get_with_id(id: i32, conn: &SqliteConnection) -> Option<Application> {
        all_applications
            .find(id)
            .get_result::<Application>(conn)
            .ok()
    }

    #[cfg(test)]
    pub fn delete_all(conn: &SqliteConnection) -> bool {
        diesel::delete(all_applications).execute(conn).is_ok()
    }
}
