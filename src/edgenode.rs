use diesel::{self, prelude::*};

#[path = "./schema.rs"] mod schema;

use schema::edgenodes;
use schema::edgenodes::dsl::{edgenodes as all_edgenodes};

#[table_name="edgenodes"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Edgenode {
    pub id: Option<i32>,
    pub name: String,
    pub ipaddr: String, // Handle conversions when going to and from display and from form
    pub radio_type: String,
    pub radio_address: String
}

#[derive(FromForm)]
pub struct Node {
    pub name: String,
    pub ipaddr: String, // Handle conversion going to and from i32 and String
    pub radio_type: String,
    pub radio_address: String
}

impl Edgenode {
    pub fn all(conn: &SqliteConnection) -> Vec<Edgenode> {
        all_edgenodes.order(edgenodes::id.desc()).load::<Edgenode>(conn).unwrap()
    }

    pub fn insert(node: Node, conn: &SqliteConnection) -> bool {
        // Check if IP Address is valid
        let t = Edgenode { id: None, name: node.name, ipaddr: node.ipaddr, radio_type: node.radio_type, radio_address: node.radio_address };
        diesel::insert_into(edgenodes::table).values(&t).execute(conn).is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_edgenodes.find(id)).execute(conn).is_ok()
    }

    pub fn get_with_id(id: i32, conn: &SqliteConnection) -> Option<Edgenode> {
        all_edgenodes.find(id).get_result::<Edgenode>(conn).ok()
    }

    #[cfg(test)]
    pub fn delete_all(conn: &SqliteConnection) -> bool {
        diesel::delete(all_edgenodes).execute(conn).is_ok()
    }
}
