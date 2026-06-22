use crate::model::Post;
use crate::views::footer::footer;
use crate::views::header::header;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ActivePage {
    Posts,
    Post,
    About,
}

pub struct Page {
    pub title: String,
    pub description: String,
    pub active_page: ActivePage,
}

pub fn render_page(content: impl RenderHtml + Send + 'static, page: Page) -> String {
    let html = view! {
        <html lang="en" class="dark">
            <head>
                <meta charset="UTF-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <title>{page.title}</title>
                <meta name="description" content=page.description/>
                <link
                    id="highlight-theme-dark"
                    rel="stylesheet"
                    href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/tokyo-night-dark.min.css"
                    media="all"
                />
                <link
                    id="highlight-theme-light"
                    rel="stylesheet"
                    href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/tokyo-night-light.min.css"
                    media="not all"
                />
                <link rel="stylesheet" href="/static/index.css"/>
                <script
                    src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/highlight.min.js"
                    defer
                ></script>
                <script
                    src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/scala.min.js"
                    defer
                ></script>
                <script>
                    {r#"(function () {
  var stored = localStorage.getItem("theme");
  var theme = stored === "light" ? "light" : "dark";
  document.documentElement.classList.remove("light", "dark");
  document.documentElement.classList.add(theme);
  document.getElementById("highlight-theme-dark").media = theme === "dark" ? "all" : "not all";
  document.getElementById("highlight-theme-light").media = theme === "light" ? "all" : "not all";
})();"#}
                </script>
                <script src="/static/site.js" defer></script>
            </head>
            <body>
                {header(page.active_page)}
                <main class="site-main">{content}</main>
                {footer()}
            </body>
        </html>
    }
    .to_html();

    format!("<!DOCTYPE html>{html}")
}

pub fn format_post_url(post: &Post) -> String {
    format!("/posts/{}", post.path)
}

pub fn nav_link(href: &str, label: &str, active: bool, base_class: &str) -> impl IntoView {
    let class = match (base_class.is_empty(), active) {
        (true, true) => "is-active".to_string(),
        (true, false) => String::new(),
        (false, true) => format!("{base_class} is-active"),
        (false, false) => base_class.to_string(),
    };

    view! {
        <a class=class href=href aria-current=active.then_some("page")>{label.to_string()}</a>
    }
}
