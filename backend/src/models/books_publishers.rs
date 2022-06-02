// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::books_publishers_link;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "books_publishers_link"]
pub struct NewBookPublisher {
    pub book: i32,
    pub publisher: i32,
}

#[derive(Debug, Serialize, Queryable)]
pub struct BookPublisher {
    pub id: i32,
    pub book: i32,
    pub publisher: i32,
    pub created: NaiveDateTime,
}

pub fn add_book_publisher(
    conn: &PgConnection,
    new_book_publisher: &NewBookPublisher,
) -> Result<(), Error> {
    use crate::schema::books_publishers_link::dsl::books_publishers_link;
    diesel::insert_into(books_publishers_link)
        .values(new_book_publisher)
        .execute(conn)?;
    Ok(())
}

pub fn get_book_publisher(conn: &PgConnection, book_id: i32) -> Result<BookPublisher, Error> {
    use crate::schema::books_publishers_link::dsl::{book, books_publishers_link};
    books_publishers_link
        .filter(book.eq(book_id))
        .first::<BookPublisher>(conn)
        .map_err(Into::into)
}

pub fn delete_book_publisher(conn: &PgConnection, book_id: i32) -> Result<(), Error> {
    use crate::schema::books_publishers_link::dsl::{book, books_publishers_link};
    let _link = get_book_publisher(conn, book_id)?;
    diesel::delete(books_publishers_link)
        .filter(book.eq(book_id))
        .execute(conn)?;
    Ok(())
}