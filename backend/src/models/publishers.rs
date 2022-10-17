// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use super::page::{Page, PageQuery};
use crate::error::Error;
use crate::schema::publishers;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = publishers)]
pub struct NewPublisher {
    pub name: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Publisher {
    pub id: i32,
    pub name: String,
    pub crated: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_publisher(conn: &mut PgConnection, new_publisher: &NewPublisher) -> Result<(), Error> {
    use crate::schema::publishers::dsl::publishers;
    diesel::insert_into(publishers)
        .values(new_publisher)
        .execute(conn)?;
    Ok(())
}

#[derive(Debug, Serialize, Queryable)]
pub struct PublisherAndBook {
    pub id: i32,
    pub name: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct GetPublishersResp {
    pub page: Page,
    pub list: Vec<PublisherAndBook>,
}

pub fn get_publishers(
    conn: &mut PgConnection,
    query: &PageQuery,
) -> Result<GetPublishersResp, Error> {
    use crate::schema::books_publishers_link;

    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let each_page = 50;
    let offset = page_id * each_page;

    let list = publishers::table
        .left_join(
            books_publishers_link::table.on(books_publishers_link::publisher.eq(publishers::id)),
        )
        .group_by(publishers::id)
        .select((
            publishers::id,
            publishers::name,
            diesel::dsl::sql::<diesel::sql_types::BigInt>("count(books_publishers_link.id)"),
        ))
        .limit(each_page)
        .offset(offset)
        .load::<PublisherAndBook>(conn)?;

    let total = publishers::dsl::publishers.count().first(conn)?;

    Ok(GetPublishersResp {
        page: Page {
            page_num: page_id + 1,
            each_page,
            total,
        },
        list,
    })
}

pub fn get_publisher_by_id(conn: &mut PgConnection, publisher_id: i32) -> Result<Publisher, Error> {
    publishers::table
        .find(publisher_id)
        .first(conn)
        .map_err(Into::into)
}

pub fn get_publisher_by_name(
    conn: &mut PgConnection,
    publisher_name: &str,
) -> Result<Publisher, Error> {
    use crate::schema::publishers::dsl::{name, publishers};
    publishers
        .filter(name.eq(publisher_name))
        .first(conn)
        .map_err(Into::into)
}

pub fn update_publisher(
    conn: &mut PgConnection,
    pub_id: i32,
    new_publisher: &NewPublisher,
) -> Result<(), Error> {
    use crate::schema::publishers::dsl::{name, publishers};
    diesel::update(publishers.find(pub_id))
        .set(name.eq(new_publisher.name.as_str()))
        .execute(conn)?;
    Ok(())
}
