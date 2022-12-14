// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use shared::ratings::Rating;

use crate::error::Error;
use crate::schema::ratings;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = ratings)]
pub struct NewRating {
    pub book: i32,
    pub rating: i32,
}

pub fn add_rating(conn: &mut PgConnection, new_rating: &NewRating) -> Result<(), Error> {
    use crate::schema::ratings::dsl::ratings;
    diesel::insert_into(ratings)
        .values(new_rating)
        .execute(conn)?;
    Ok(())
}

pub fn get_rating(conn: &mut PgConnection, book_id: i32) -> Result<Rating, Error> {
    use crate::schema::ratings::dsl::{book, ratings};
    ratings
        .filter(book.eq(book_id))
        .first::<Rating>(conn)
        .map_err(Into::into)
}

pub fn update_rating(conn: &mut PgConnection, new_rating: &NewRating) -> Result<(), Error> {
    use crate::schema::ratings::dsl::{book, rating, ratings};
    diesel::update(ratings.filter(book.eq(new_rating.book)))
        .set(rating.eq(new_rating.rating))
        .execute(conn)?;
    Ok(())
}

pub fn delete_rating(conn: &mut PgConnection, book_id: i32) -> Result<(), Error> {
    use crate::schema::ratings::dsl::{book, ratings};
    let _rating = get_rating(conn, book_id)?;
    diesel::delete(ratings.filter(book.eq(book_id))).execute(conn)?;
    Ok(())
}
