use crate::views::renderer::{ActivePage, Page, render_page};
use leptos::prelude::*;

pub fn render_about() -> String {
    let content = view! {
        <section class="page-section">
            <div class="narrow">
                <header class="page-header">
                    <h1 class="page-title">"About"</h1>
                </header>

                <div class="two-column">
                    <article class="article-prose">
                        <p>
                            "I originally studied chemistry, but eventually realized that I enjoyed "
                            "software engineering much more and decided to make it my career."
                        </p>
                        <p>
                            "For the past 12 years, I’ve been building mobile products at a wide"
                            "range of companies, from small startups to large fintech companies."
                        </p>
                        <p>
                            "In my free time, I enjoy exploring programming languages such as"
                            "Scala, Rust, Clojure, and Swift, as well as learning more about"
                            "different programming paradigms and approaches to software development."
                        </p>
                    </article>

                    <aside class="card">
                        <h2>"Contact"</h2>
                        <p class="microcopy">
                            "Found something wrong or have a suggestion? Let me know by opening an issue on "
                            <a
                                href="https://github.com/aivanovski/aivanovski.github.io/issues"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                "GitHub"
                            </a>
                            "."
                        </p>
                    </aside>
                </div>
            </div>
        </section>
    };

    render_page(
        content,
        Page {
            title: String::from("About | Personal Notes"),
            description: String::from(
                "About Aliaksei Ivanouski and this personal software engineering blog.",
            ),
            active_page: ActivePage::About,
        },
    )
}
