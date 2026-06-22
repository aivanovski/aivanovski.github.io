use std::fs;
use std::path::{Path, PathBuf};
use crate::error::AppError;
use crate::posts::load_posts;
use crate::{POSTS_DIRECTORY, STATIC_DIRECTORY};
use crate::views::about::render_about;
use crate::views::home::render_home;
use crate::views::post::render_post;

pub fn export_static_site(output: &Path) -> Result<(), AppError> {
    let posts = load_posts(&PathBuf::from(POSTS_DIRECTORY))?;

    write_file(&output.join("index.html"), render_home(&posts))?;
    write_file(&output.join("home").join("index.html"), render_home(&posts))?;
    write_file(
        &output.join("posts").join("index.html"),
        render_home(&posts),
    )?;
    write_file(&output.join("about").join("index.html"), render_about())?;

    for post in &posts {
        write_file(
            &output.join("posts").join(&post.path).join("index.html"),
            render_post(post),
        )?;
    }

    copy_directory(
        &PathBuf::from(STATIC_DIRECTORY),
        &output.join(STATIC_DIRECTORY),
    )?;

    Ok(())
}

fn write_file(path: &Path, contents: String) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| io_error(parent, error))?;
    }

    fs::write(path, contents).map_err(|error| io_error(path, error))
}

fn copy_directory(source: &Path, destination: &Path) -> Result<(), AppError> {
    fs::create_dir_all(destination).map_err(|error| io_error(destination, error))?;

    for entry in fs::read_dir(source).map_err(|error| io_error(source, error))? {
        let entry = entry.map_err(|error| AppError::IOError {
            message: format!(
                "Unable to read directory entry in {}: {error}",
                source.display()
            ),
        })?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            copy_directory(&source_path, &destination_path)?;
        } else {
            if let Some(parent) = destination_path.parent() {
                fs::create_dir_all(parent).map_err(|error| io_error(parent, error))?;
            }

            fs::copy(&source_path, &destination_path).map_err(|error| AppError::IOError {
                message: format!(
                    "Unable to copy {} to {}: {error}",
                    source_path.display(),
                    destination_path.display(),
                ),
            })?;
        }
    }

    Ok(())
}

fn io_error(path: &Path, error: std::io::Error) -> AppError {
    AppError::IOError {
        message: format!("Unable to write {}: {error}", path.display()),
    }
}