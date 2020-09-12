use diesel::{self, prelude::*};
use std::net::Ipv4Addr;

use schema::edgenodes;
use schema::edgenodes::dsl::{edgenodes as all_edgenodes};

#[table_name="edgenodes"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Edgenode {
    pub id: Option<i32>,
    pub name: String,
    pub ipaddr: Ipv4Addr,
    pub radio_type: String,
    pub radio_address: String
}

#[derive(FromForm)]
pub struct Node {
    pub name: String,
    pub ipaddr: Ipv4Addr,
    pub radio_type: String,
    pub radio_address: String
}

impl Edgenode {
    pub fn all(conn: &SqliteConnection) -> Vec<Edgenode> {
        all_edgenodes.order(edgenodes::id.desc()).load::<Edgenode>(conn).unwrap()
    }

    pub fn insert(node: Node, conn: &SqliteConnection) -> bool {
        let t = Edgenode { id: None, name: node.name, ipaddr: node.ipaddr, radio_type: node.radio_type, radio_address: node.radio_address };
        diesel::insert_into(edgenodes::table).values(&t).execute(conn).is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_edgenodes.find(id)).execute(conn).is_ok()
    }

    #[cfg(test)]
    pub fn delete_all(conn: &SqliteConnection) -> bool {
        diesel::delete(all_edgenodes).execute(conn).is_ok()
    }
}
