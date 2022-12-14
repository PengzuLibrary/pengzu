// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use shared::authors::Author;
use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::page::BOOKS_EACH_PAGE;

use crate::error::Error;
use crate::models::books::{book_list_to_book_authors, Book};
use crate::models::books_query::sort_books_by_column;
use crate::schema::books_authors_link;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = books_authors_link)]
pub struct NewBookAuthor {
    pub book: i32,
    pub author: i32,
}

#[derive(Debug, Serialize, Queryable)]
pub struct BookAuthor {
    pub id: i32,
    pub book: i32,
    pub author: i32,
    pub created: NaiveDateTime,
}

pub fn add_book_author(
    conn: &mut PgConnection,
    new_book_author: &NewBookAuthor,
) -> Result<(), Error> {
    use crate::schema::books_authors_link::dsl::books_authors_link;
    diesel::insert_into(books_authors_link)
        .values(new_book_author)
        .execute(conn)?;
    Ok(())
}

pub fn delete_book_author(
    conn: &mut PgConnection,
    new_book_author: &NewBookAuthor,
) -> Result<(), Error> {
    use crate::schema::books_authors_link::dsl::{author, book, books_authors_link};
    // TODO(Shaohua): Check exists
    diesel::delete(
        books_authors_link
            .filter(book.eq(new_book_author.book))
            .filter(author.eq(new_book_author.author)),
    )
    .execute(conn)?;
    Ok(())
}

pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result<(), Error> {
    diesel::delete(books_authors_link::table.find(id)).execute(conn)?;
    Ok(())
}

pub fn get_authors_by_book(conn: &mut PgConnection, book_id: i32) -> Result<Vec<Author>, Error> {
    use crate::schema::authors;

    authors::table
        .filter(
            authors::id.eq_any(
                books_authors_link::table
                    .filter(books_authors_link::book.eq(book_id))
                    .select(books_authors_link::author),
            ),
        )
        .load::<Author>(conn)
        .map_err(Into::into)
}

pub fn get_links_by_author(
    conn: &mut PgConnection,
    author_id: i32,
) -> Result<Vec<BookAuthor>, Error> {
    books_authors_link::table
        .filter(books_authors_link::author.eq(author_id))
        .load::<BookAuthor>(conn)
        .map_err(Into::into)
}

pub fn get_books_by_author(
    conn: &mut PgConnection,
    author_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, Error> {
    use crate::schema::books;

    let offset = query.backend_page_id() * BOOKS_EACH_PAGE;
    let total = books_authors_link::table
        .filter(books_authors_link::author.eq(author_id))
        .count()
        .first::<i64>(conn)?;

    // Get book list based on a subquery.
    let book_list = sort_books_by_column(query.order)
        .filter(
            books::id.eq_any(
                books_authors_link::table
                    .filter(books_authors_link::author.eq(author_id))
                    .select(books_authors_link::book),
            ),
        )
        .limit(BOOKS_EACH_PAGE)
        .offset(offset)
        .load::<Book>(conn)?;

    book_list_to_book_authors(conn, book_list, query, total)
}
