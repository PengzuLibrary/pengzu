// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::not_found::NotFoundComponent;
use crate::views::authors::AuthorsComponent;
use crate::views::book_detail::BookDetailComponent;
use crate::views::books::BooksComponent;
use crate::views::books_of_author::BooksOfAuthorComponent;
use crate::views::books_of_publisher::BooksOfPublisherComponent;
use crate::views::books_of_series::BooksOfSeriesComponent;
use crate::views::books_of_tag::BooksOfTagComponent;
use crate::views::categories::CategoriesComponent;
use crate::views::discover::DiscoverComponent;
use crate::views::file_formats::FileFormatsComponent;
use crate::views::publishers::PublishersComponent;
use crate::views::ratings::RatingsComponent;
use crate::views::series::SeriesComponent;
use crate::views::tags::TagsComponent;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/book/:book_id")]
    BookDetail { book_id: i32 },
    #[at("/book")]
    Book,
    #[at("/author/books/:author_id")]
    BooksOfAuthor { author_id: i32 },
    #[at("/author")]
    Author,
    #[at("/category")]
    Category,
    #[at("/tag/books/:tag_id")]
    BooksOfTag { tag_id: i32 },
    #[at("/tag")]
    Tag,
    #[at("/publisher/books/:publisher_id")]
    BooksOfPublisher { publisher_id: i32 },
    #[at("/publisher")]
    Publisher,
    #[at("/series/books/:series_id")]
    BooksOfSeries { series_id: i32 },
    #[at("/series")]
    Series,
    #[at("/discover")]
    Discover,
    #[at("/format")]
    FileFormat,
    #[at("/rating")]
    Rating,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[must_use]
pub fn switch_route(routes: &Route) -> Html {
    match routes {
        Route::BookDetail { book_id } => {
            html! { <BookDetailComponent book_id={ * book_id } /> }
        }
        Route::Book | Route::Home => html! { <BooksComponent /> },
        Route::BooksOfAuthor { author_id } => {
            html! { <BooksOfAuthorComponent author_id={ *author_id } /> }
        }
        Route::Author => html! { <AuthorsComponent /> },
        Route::Category => html! { <CategoriesComponent /> },
        Route::BooksOfTag { tag_id } => html! { <BooksOfTagComponent tag_id={ *tag_id }/>},
        Route::Tag => html! { <TagsComponent /> },
        Route::BooksOfPublisher { publisher_id } => {
            html! { <BooksOfPublisherComponent publisher_id={ *publisher_id } /> }
        }
        Route::Publisher => html! { <PublishersComponent /> },
        Route::BooksOfSeries { series_id } => {
            html! { <BooksOfSeriesComponent series_id={ * series_id } /> }
        }
        Route::Series => html! { <SeriesComponent /> },
        Route::Discover => html! { <DiscoverComponent /> },
        Route::FileFormat => html! { <FileFormatsComponent /> },
        Route::Rating => html! { <RatingsComponent /> },
        Route::NotFound => html! { <NotFoundComponent /> },
    }
}
