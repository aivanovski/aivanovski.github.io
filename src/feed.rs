use crate::error::AppError;
use crate::model::Post;
use chrono::{NaiveDate, TimeZone, Utc};
use rss::{ChannelBuilder, GuidBuilder, ItemBuilder};

const SITE_URL: &str = "https://aivanovski.github.io";
const FEED_TITLE: &str = "Aliaksei Ivanouski";
const FEED_DESCRIPTION: &str = "Writing about Kotlin, Android, software engineering, and more.";

pub fn render_rss_feed(posts: &[Post]) -> Result<String, AppError> {
    let items = posts
        .iter()
        .map(|post| {
            let post_url = format!("{SITE_URL}/posts/{}", post.path);
            let pub_date = parse_post_date(&post.posted)?.to_rfc2822();

            Ok(ItemBuilder::default()
                .title(Some(post.title.clone()))
                .link(Some(post_url.clone()))
                .description(Some(post.description.clone()))
                .pub_date(Some(pub_date))
                .guid(Some(
                    GuidBuilder::default()
                        .value(post_url)
                        .permalink(true)
                        .build(),
                ))
                .build())
        })
        .collect::<Result<Vec<_>, AppError>>()?;

    let channel = ChannelBuilder::default()
        .title(FEED_TITLE)
        .link(SITE_URL)
        .description(FEED_DESCRIPTION)
        .last_build_date(
            items
                .first()
                .and_then(|item| item.pub_date().map(str::to_owned)),
        )
        .items(items)
        .build();

    Ok(channel.to_string())
}

fn parse_post_date(date: &str) -> Result<chrono::DateTime<Utc>, AppError> {
    let date =
        NaiveDate::parse_from_str(date, "%B %d, %Y").map_err(|_| AppError::GenericError {
            message: format!("Unable to parse date: {date}"),
        })?;

    Ok(Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap()))
}
