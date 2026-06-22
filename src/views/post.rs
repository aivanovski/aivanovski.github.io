use crate::model::Post;
use crate::views::renderer::{ActivePage, Page, render_page};
use leptos::prelude::*;
use pulldown_cmark::{Options, Parser, html as markdown_html};

pub fn render_post(post: &Post) -> String {
    let markdown = render_markdown(&post.markdown);
    let content = view! {
        <section class="page-section">
            <div class="narrow article-shell">
                <article aria-labelledby="article-title">
                    <header class="article-header">
                        <h1 id="article-title" class="article-title">{post.title.clone()}</h1>
                        <p class="article-summary">{post.description.clone()}</p>
                        <p class="article-meta">
                            <span>"By " {post.author.clone()}</span>
                            <span aria-hidden="true">"\u{00b7}"</span>
                            <time>{post.posted.clone()}</time>
                            <span aria-hidden="true">"\u{00b7}"</span>
                            <span>{post.estimated_reading_time} " min read"</span>
                        </p>
                    </header>
                    <hr class="article-divider" />
                    <div class="article-prose" inner_html=markdown></div>
                </article>
            </div>
        </section>
    };

    render_page(
        content,
        Page {
            title: String::from(&format!("{} | Personal Notes", post.title)),
            description: post.description.to_string(),
            active_page: ActivePage::Post,
        },
    )
}

fn render_markdown(markdown: &str) -> String {
    let options = Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_SMART_PUNCTUATION;
    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();

    markdown_html::push_html(&mut html_output, parser);
    html_output
}
