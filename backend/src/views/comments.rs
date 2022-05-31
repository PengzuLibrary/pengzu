// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::comments as models;

pub async fn add_comment(
    pool: web::Data<DbPool>,
    form: web::Json<models::Comment>,
) -> Result<HttpResponse, Error> {
    let comment = web::block(move || {
        let conn = pool.get()?;
        models::add_comment(&conn, &form)
    })
    .await?;

    Ok(HttpResponse::Ok().json(comment))
}

pub async fn get_comments() {
    todo!();
}

pub async fn update_comment() {
    todo!();
}

pub async fn delete_comment() {
    todo!();
}
