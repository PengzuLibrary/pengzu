// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use shared::languages::Language;

use crate::error::Error;
use crate::schema::languages;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = languages)]
pub struct NewLanguage {
    pub lang_code: String,
}

pub fn add_language(conn: &mut PgConnection, new_language: &NewLanguage) -> Result<(), Error> {
    use crate::schema::languages::dsl::languages;
    diesel::insert_into(languages)
        .values(new_language)
        .execute(conn)?;
    Ok(())
}

pub fn get_language_by_name(
    conn: &mut PgConnection,
    language_name: &str,
) -> Result<Language, Error> {
    use crate::schema::languages::dsl::{lang_code, languages};
    languages
        .filter(lang_code.eq(language_name))
        .first(conn)
        .map_err(Into::into)
}

pub fn get_all_languages(conn: &mut PgConnection) -> Result<Vec<Language>, Error> {
    use crate::schema::languages::dsl::languages;
    languages.load::<Language>(conn).map_err(Into::into)
}
