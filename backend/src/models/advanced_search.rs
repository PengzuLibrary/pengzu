// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{
    ExpressionMethods, JoinOnDsl, PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
};
use shared::advanced_search::AdvancedSearchQuery;
use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;

use crate::error::Error;
use crate::models::books::get_books_by_ids;

#[allow(unused_assignments)]
pub fn get_books_by_advanced_search(
    conn: &mut PgConnection,
    query: &AdvancedSearchQuery,
) -> Result<BookAndAuthorsList, Error> {
    use crate::schema::authors;
    use crate::schema::books;
    use crate::schema::books_authors_link;
    use crate::schema::books_publishers_link;
    use crate::schema::identifier_types;
    use crate::schema::identifiers;
    use crate::schema::publishers;

    let books_query = GetBooksQuery {
        page: query.page,
        order: query.order,
    };

    // TODO(Shaohua): Tuning query, replace with a subquery.

    let mut book_ids = Vec::new();
    let mut book_id_nil = true;
    if let Some(author_name) = &query.author {
        let author_pattern = format!("%{author_name}%");
        let author_ids = authors::table
            .filter(authors::name.ilike(author_pattern))
            .select(authors::id)
            .load::<i32>(conn)?;

        book_ids = books_authors_link::table
            .filter(books_authors_link::author.eq_any(author_ids))
            .select(books_authors_link::book)
            .load::<i32>(conn)?;
        book_id_nil = false;
    }

    if let Some(publisher_name) = &query.publisher {
        let publisher_pattern = format!("%{publisher_name}%");
        let publisher_ids = publishers::table
            .filter(publishers::name.ilike(publisher_pattern))
            .select(publishers::id)
            .load::<i32>(conn)?;

        if book_id_nil {
            book_id_nil = false;
            book_ids = books_publishers_link::table
                .filter(books_publishers_link::publisher.eq_any(publisher_ids))
                .select(books_publishers_link::book)
                .load::<i32>(conn)?;
        } else {
            book_ids = books_publishers_link::table
                .filter(books_publishers_link::publisher.eq_any(publisher_ids))
                .filter(books_publishers_link::book.eq_any(book_ids))
                .select(books_publishers_link::book)
                .load::<i32>(conn)?;
        }
    }

    if let Some(title) = &query.title {
        let title_pattern = format!("%{title}%");
        if book_id_nil {
            book_id_nil = false;
            book_ids = books::table
                .filter(books::title.ilike(&title_pattern))
                .select(books::id)
                .load::<i32>(conn)?;
        } else {
            book_ids = books::table
                .filter(books::title.ilike(&title_pattern))
                .filter(books::id.eq_any(book_ids))
                .select(books::id)
                .load::<i32>(conn)?;
        }
    }

    // SELECT book_id FROM identifiers
    // INNER JOIN identifier_types ON identifiers.scheme = identifier_types.id
    // WHERE identifier_types.name == 'isbn' AND identifier.value = "USER-ISBN"
    if let Some(isbn) = &query.isbn {
        book_id_nil = false;
        let isbn_name = "isbn";
        book_ids = identifiers::table
            .inner_join(identifier_types::table.on(identifiers::scheme.eq(identifier_types::id)))
            .filter(identifier_types::name.eq(isbn_name))
            .filter(identifiers::value.eq(isbn))
            .select(identifiers::book)
            .load::<i32>(conn)?;
    }

    get_books_by_ids(conn, &books_query, &book_ids)
}
