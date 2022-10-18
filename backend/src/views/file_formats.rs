// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::file_formats;
use crate::models::page::PageQuery;

pub async fn get_formats(
    pool: web::Data<DbPool>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        file_formats::get_formats(&mut conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_format(
    pool: web::Data<DbPool>,
    format_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        file_formats::get_file_format_by_id(&mut conn, format_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}
