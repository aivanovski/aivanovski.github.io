use crate::error::AppError;
use crate::error::AppError::IOError;
use crate::model::{Post, PostDescription};
use chrono::{NaiveDate, TimeZone, Utc};
use markdown_readtime::minutes;
use std::fs;
use std::path::{Path, PathBuf};

pub struct PostsRepository {
    directory: PathBuf,
}

impl PostsRepository {
    pub fn new(directory: PathBuf) -> PostsRepository {
        PostsRepository { directory }
    }

    pub fn list_posts(&self) -> Result<Vec<Post>, AppError> {
        load_posts(&self.directory)
    }

    pub fn find_post(&self, path: &str) -> Result<Option<Post>, AppError> {
        Ok(self
            .list_posts()?
            .into_iter()
            .find(|post| post.path == path))
    }
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn load_posts(dir: &Path) -> Result<Vec<Post>, AppError> {
    let root_dir = project_root().join(dir.to_str().unwrap());
    let entries = fs::read_dir(&root_dir).map_err(|error| IOError {
        message: format!(
            "Unable to read posts directory {}: {error}",
            root_dir.display()
        ),
    })?;

    let mut posts = entries
        .filter_map(|entry| match entry {
            Ok(entry)
                if entry.path().is_file()
                    && entry
                        .path()
                        .extension()
                        .and_then(|extension| extension.to_str())
                        == Some("md") =>
            {
                Some(read_post(&entry.path()))
            }
            Ok(_) => None,
            Err(error) => Some(Err(IOError {
                message: format!("Unable to read post directory entry: {error}"),
            })),
        })
        .collect::<Result<Vec<_>, AppError>>()?;

    posts.sort_by_key(|(order, _)| *order);
    posts.reverse();
    Ok(posts.into_iter().map(|(_, post)| post).collect())
}

fn read_post(markdown_path: &Path) -> Result<(i64, Post), AppError> {
    let post = fs::read_to_string(markdown_path).map_err(|error| IOError {
        message: format!("Unable to read {}: {error}", markdown_path.display()),
    })?;
    let (front_matter, markdown) = parse_front_matter(&post, markdown_path)?;
    let description: PostDescription = toml::from_str(front_matter).map_err(|error| IOError {
        message: format!(
            "Unable to parse front matter in {}: {error}",
            markdown_path.display()
        ),
    })?;

    let path = markdown_path
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| IOError {
            message: format!("Invalid post file name: {}", markdown_path.display()),
        })?
        .to_owned();

    let date =
        NaiveDate::parse_from_str(description.posted.as_str(), "%B %d, %Y").map_err(|_| {
            AppError::GenericError {
                message: format!("Unable to parse date: {}", description.posted),
            }
        })?;

    let datetime = Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap());

    Ok((
        datetime.timestamp(),
        Post {
            title: description.title,
            posted: description.posted,
            author: description.author,
            path,
            description: description.description,
            estimated_reading_time: minutes(markdown),
            markdown: markdown.to_owned(),
        },
    ))
}

fn parse_front_matter<'a>(post: &'a str, path: &Path) -> Result<(&'a str, &'a str), AppError> {
    let post = post.strip_prefix("+++\n").ok_or_else(|| IOError {
        message: format!("Missing TOML front matter in {}", path.display()),
    })?;

    post.split_once("\n+++\n").ok_or_else(|| IOError {
        message: format!("Unclosed TOML front matter in {}", path.display()),
    })
}
