// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::publishers as models;

pub async fn add_publisher(
    pool: web::Data<DbPool>,
    new_publisher: web::Json<models::NewPublisher>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        models::add_publisher(&conn, &new_publisher)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_publishers(
    pool: web::Data<DbPool>,
    query: web::Query<models::GetPublishersQuery>,
) -> Result<HttpResponse, Error> {
    let resp_publishers = web::block(move || {
        let conn = pool.get()?;
        models::get_publishers(&conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp_publishers))
}
