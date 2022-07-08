// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::books as models;

pub async fn add_book(
    pool: web::Data<DbPool>,
    new_book: web::Json<models::NewBook>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        models::add_book(&conn, &new_book)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_books(
    pool: web::Data<DbPool>,
    query: web::Query<models::GetBooksQuery>,
) -> Result<HttpResponse, Error> {
    log::info!("query: {:?}", query);
    let resp = web::block(move || {
        let conn = pool.get()?;
        models::get_books(&conn, &query)
    })
    .await??;

    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_book_detail(
    pool: web::Data<DbPool>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp_book = web::block(move || {
        let conn = pool.get()?;
        models::get_book_detail(&conn, book_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp_book))
}

pub async fn get_books_by_author(
    pool: web::Data<DbPool>,
    author_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let conn = pool.get()?;
        models::get_books_by_author(&conn, author_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_books_by_format(
    pool: web::Data<DbPool>,
    format_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let conn = pool.get()?;
        models::get_books_by_format(&conn, format_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_books_by_publisher(
    pool: web::Data<DbPool>,
    publisher_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let conn = pool.get()?;
        models::get_books_by_publisher(&conn, publisher_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_books_by_tag(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let conn = pool.get()?;
        models::get_books_by_tag(&conn, tag_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}
