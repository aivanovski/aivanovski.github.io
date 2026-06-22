use chrono::{Datelike, Utc};
use leptos::prelude::*;

pub fn footer() -> impl IntoView {
    let current_year = Utc::now().year();

    view! {
        <footer class="site-footer">
            <div class="narrow footer-inner">
                <nav class="footer-links" aria-label="Social links">
                    <a
                        href="https://github.com/aivanovski"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        "GitHub"
                    </a>
                    <a
                        href="https://www.linkedin.com/in/aliaksei-ivanouski-883ba381"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        "LinkedIn"
                    </a>
                </nav>
                <p class="footer-copyright">
                    {format!("\u{00a9} {current_year} Aliaksei Ivanouski")}
                </p>
            </div>
        </footer>
    }
}
