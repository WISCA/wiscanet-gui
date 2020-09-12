use diesel::{self, prelude::*};
use std::net::Ipv4Addr;
use std::str::FromStr;

mod schema {
    table! {
        edgenodes (id) {
            id -> Nullable<Integer>,
            name -> Text,
            ipaddr -> Integer,
            radio_type -> Text,
            radio_address -> Text,
        }
    }
}

use self::schema::edgenodes;
use self::schema::edgenodes::dsl::{edgenodes as all_edgenodes};

#[table_name="edgenodes"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Edgenode {
    pub id: Option<i32>,
    pub name: String,
    pub ipaddr: i32, // Handle conversions when going to and from display and from form
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
        let ipaddr = Ipv4Addr::from_str(&node.ipaddr);
        let numeric_ipaddr = u32::from(ipaddr.unwrap()) as i32;
        let t = Edgenode { id: None, name: node.name, ipaddr: numeric_ipaddr, radio_type: node.radio_type, radio_address: node.radio_address };
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
