// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::user_tags;
use crate::models::user_tags::{GetUserTagsReq, NewUserTag};

pub async fn add_tag(
    pool: web::Data<DbPool>,
    new_tag: web::Json<NewUserTag>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        user_tags::add_tag(&conn, &new_tag)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_tags(
    pool: web::Data<DbPool>,
    query: web::Query<GetUserTagsReq>,
) -> Result<HttpResponse, Error> {
    let tags_resp = web::block(move || {
        let conn = pool.get()?;
        user_tags::get_tags(&conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(tags_resp))
}

pub async fn update_tag(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
    new_tag: web::Json<NewUserTag>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        user_tags::update_tag(&conn, tag_id.into_inner(), &new_tag)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}