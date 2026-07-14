use crate::model::Post;
use crate::views::renderer::{ActivePage, Page, render_page};
use leptos::prelude::*;
use pulldown_cmark::html::push_html;
use pulldown_cmark::{CowStr, Event, HeadingLevel, Options, Parser, Tag};
use std::collections::HashMap;

pub fn render_post(post: &Post) -> String {
    // Markdown rendering also returns the section list used by the TOC, so
    // article content and sidebar navigation stay in sync automatically.
    let rendered_article = render_markdown(&post.markdown);
    let markdown = rendered_article.html;
    let article_sections = rendered_article.sections;
    let has_sections = !article_sections.is_empty();

    let content = view! {
        <section class="page-section">
            <div class="narrow article-shell">
                <article class="article-main" aria-labelledby="article-title">
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
                {has_sections.then(|| {
                    view! {
                        <aside class="article-contents" aria-labelledby="article-contents-title">
                            <div class="article-contents-card">
                                <p id="article-contents-title" class="article-contents-title">
                                    "On this page"
                                </p>
                                <nav class="article-contents-nav" aria-label="Article sections">
                                    {article_sections
                                        .into_iter()
                                        .enumerate()
                                        .map(|(index, section)| {
                                            let href = format!("#{}", section.id);
                                            // H3 items are rendered as children of the previous
                                            // top-level section, while the first item starts active
                                            // before JS updates the highlight based on scroll.
                                            let class = match (section.level, index == 0) {
                                                (HeadingLevel::H3, true) => {
                                                    "article-contents-link article-contents-link-level-3 is-active"
                                                }
                                                (HeadingLevel::H3, false) => {
                                                    "article-contents-link article-contents-link-level-3"
                                                }
                                                (_, true) => "article-contents-link is-active",
                                                (_, false) => "article-contents-link",
                                            };

                                            view! {
                                                <a class=class href=href data-toc-link>
                                                    {section.title}
                                                </a>
                                            }
                                        })
                                        .collect_view()}
                                </nav>
                            </div>
                        </aside>
                    }
                })}
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

struct RenderedArticle {
    html: String,
    sections: Vec<ArticleSection>,
}

#[derive(Clone)]
struct ArticleSection {
    id: String,
    title: String,
    level: HeadingLevel,
}

fn render_markdown(markdown: &str) -> RenderedArticle {
    let options = Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_SMART_PUNCTUATION;
    let parser = Parser::new_ext(markdown, options);
    let (events, sections) = add_heading_anchors(parser);
    let mut html_output = String::new();

    push_html(&mut html_output, events.into_iter());

    RenderedArticle {
        html: html_output,
        sections,
    }
}

fn add_heading_anchors<'input, 'callback>(
    parser: Parser<'input, 'callback>,
) -> (Vec<Event<'input>>, Vec<ArticleSection>) {
    let mut events = Vec::new();
    let mut sections = Vec::new();
    let mut slugs = HashMap::new();
    let mut parser = parser.peekable();

    while let Some(event) = parser.next() {
        match event {
            Event::Start(Tag::Heading(level, existing_id, classes)) => {
                // A heading is rewritten into raw HTML with an explicit id so:
                // 1. the page can deep-link to it
                // 2. the TOC can point to it
                // 3. the browser JS can detect the active section while scrolling
                let (heading_events, raw_title) = collect_heading_events(&mut parser, level);
                let title = normalize_heading_title(&raw_title);
                let id = existing_id
                    .map(str::to_owned)
                    .unwrap_or_else(|| next_slug(slugify(&title), &mut slugs));

                // Only h2/h3 headings are included in the TOC. That matches the
                // compact "On this page" navigation used on article pages.
                if matches!(level, HeadingLevel::H2 | HeadingLevel::H3) {
                    sections.push(ArticleSection {
                        id: id.clone(),
                        title: title.clone(),
                        level,
                    });
                }

                events.push(Event::Html(CowStr::Boxed(
                    render_heading_start(level, &id, &classes).into_boxed_str(),
                )));
                events.extend(heading_events);
                events.push(Event::Html(CowStr::Boxed(
                    format!("</{level}>").into_boxed_str(),
                )));
            }
            _ => events.push(event),
        }
    }

    (events, sections)
}

fn collect_heading_events<'a, I>(parser: &mut I, level: HeadingLevel) -> (Vec<Event<'a>>, String)
where
    I: Iterator<Item = Event<'a>>,
{
    let mut events = Vec::new();
    let mut title = String::new();

    // We keep the original heading child events so inline formatting still
    // renders correctly, but we also collect a plain-text title for the TOC
    // label and generated slug.
    for event in parser.by_ref() {
        match &event {
            Event::End(Tag::Heading(end_level, _, _)) if *end_level == level => break,
            Event::Text(text) => title.push_str(text.as_ref()),
            Event::Code(text) => title.push_str(text.as_ref()),
            Event::FootnoteReference(label) => title.push_str(label.as_ref()),
            Event::SoftBreak | Event::HardBreak => title.push(' '),
            _ => {}
        }

        events.push(event);
    }

    (events, title)
}

fn render_heading_start(level: HeadingLevel, id: &str, classes: &[&str]) -> String {
    let class_attr = if classes.is_empty() {
        String::new()
    } else {
        format!(" class=\"{}\"", classes.join(" "))
    };

    format!("<{level} id=\"{id}\" data-toc-anchor{class_attr}>")
}

fn normalize_heading_title(title: &str) -> String {
    title.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn slugify(title: &str) -> String {
    // Turn a heading into a URL-friendly fragment id, e.g.
    // "What about C/C++?" -> "what-about-c-c".
    let mut slug = String::new();
    let mut last_was_separator = false;

    for ch in title.chars() {
        if ch.is_alphanumeric() {
            for lower in ch.to_lowercase() {
                slug.push(lower);
            }
            last_was_separator = false;
        } else if !slug.is_empty() && !last_was_separator {
            slug.push('-');
            last_was_separator = true;
        }
    }

    slug.trim_matches('-').to_string()
}

fn next_slug(base_slug: String, slugs: &mut HashMap<String, u32>) -> String {
    // Duplicate headings need unique ids so clicking a TOC entry never lands on
    // the wrong section.
    let base_slug = if base_slug.is_empty() {
        String::from("section")
    } else {
        base_slug
    };
    let count = slugs.entry(base_slug.clone()).or_insert(0);
    let slug = if *count == 0 {
        base_slug.clone()
    } else {
        format!("{base_slug}-{}", *count + 1)
    };

    *count += 1;
    slug
}
