use crate::model::Post;
use crate::views::renderer::{format_post_url, render_page, ActivePage, Page};
use leptos::prelude::*;

pub fn render_home(posts: &[Post]) -> String {
    let posts = posts.to_vec();
    let content = view! {
        <section class="page-section">
            <div class="narrow">
                <header class="page-header home-intro">
                    <h1 class="page-title home-title">"Hi there, I’m Aliaksei!"</h1>
                    <p class="home-description">
                        "Welcome to my website, where I write about Kotlin, Android, "
                        "software engineering and more!"
                    </p>
                </header>

                <div class="timeline">
                    {posts
                        .into_iter()
                        .map(|post| {
                            let url = format_post_url(&post);

                            view! {
                                <article class="post-row">
                                    <time class="post-date">{post.posted}</time>
                                    <div class="post-copy">
                                        <h2>
                                            <a href=url.clone()>{post.title}</a>
                                        </h2>
                                        <p>{post.description}</p>
                                        <a class="read-link" href=url>
                                            "Read article"
                                            <span aria-hidden="true">"\u{2192}"</span>
                                        </a>
                                    </div>
                                </article>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    };

    render_page(
        content,
        Page {
            title: String::from("Posts | Aliaksei Ivanouski"),
            description: String::from(
                "Writing about Kotlin, Android, software engineering, and more.",
            ),
            active_page: ActivePage::Posts,
        },
    )
}
