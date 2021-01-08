use diesel::{prelude::*, sqlite::SqliteConnection};
use crate::db::models;
use crate::db::schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./ichneos.db";
    SqliteConnection::establish(db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn save_activity(connection: &SqliteConnection,
    uuid: &str, header: &str, title: &str,
    title_url: &str, time: &str) { 
    
    let task = models::MyActivityEntity { 
        uuid      : uuid.to_string(),
        header    : header.to_string(),
        title     : title.to_string(),
        title_url : title_url.to_string(),
        time      : time.to_string()
     };

    diesel::insert_into(schema::google_my_activity::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new google_my_activity");
}

pub fn save_sub_title(connection: &SqliteConnection,
    a_uuid: &str, name: &str, url: &str ) { 
    
    let task = models::SubTitlesEntity { 
        a_uuid  : a_uuid.to_string(),
        name    : name.to_string(),
        url     : url.to_string()
     };

    diesel::insert_into(schema::activity_sub_title::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new activity_sub_title");
}

pub fn save_location_info(connection: &SqliteConnection,
    a_uuid: &str, name: &str, url: &str, source: &str ) { 
    
    let task = models::LocationInfoEntity { 
        a_uuid  : a_uuid.to_string(),
        name    : name.to_string(),
        url     : url.to_string(),
        source  : source.to_string()
     };

    diesel::insert_into(schema::activity_location_info::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new activity_location_info");
}

pub fn save_products(connection: &SqliteConnection,
    a_uuid: &str, name: &str ) { 
    
    let task = models::ProductsEntity { 
        a_uuid  : a_uuid.to_string(),
        name    : name.to_string()
     };

    diesel::insert_into(schema::activity_products::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new activity_products");
}

pub fn save_details(connection: &SqliteConnection,
    a_uuid: &str, name: &str ) { 
    
    let task = models::DetailsEntity { 
        a_uuid  : a_uuid.to_string(),
        name    : name.to_string()
     };

    diesel::insert_into(schema::activity_details::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new activity_details");
}

pub fn save_location_history(connection: &SqliteConnection,
    activity: &str, timestamp_msec: i64, 
    accuracy: i32, verticalaccuracy: i32,
    altitude: i32, lat: f32, lng: f32 ) { 
    
    let task = models::LocationHistoryEntity { 
        activity         : activity.to_string(),
        timestamp_msec   : timestamp_msec,
        accuracy         : accuracy,
        verticalaccuracy : verticalaccuracy,
        altitude         : altitude,
        lat              : lat,
        lng              : lng
     };

    diesel::insert_into(schema::location_history::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new activity_details");
}
