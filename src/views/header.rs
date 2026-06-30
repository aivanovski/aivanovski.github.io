use crate::views::icons::{close_icon, home_icon, menu_icon, moon_icon, rss_icon, sun_icon};
use crate::views::renderer::{ActivePage, nav_link};
use leptos::either::Either;
use leptos::prelude::*;

pub fn header(active_page: ActivePage) -> impl IntoView {
    let posts_active = matches!(active_page, ActivePage::Posts | ActivePage::Post);

    view! {
        <header class="site-header">
            <div class="shell">
                <div class="nav-row">
                    {if active_page == ActivePage::Post {
                        Either::Left(view! {
                            <a class="back-link" href="/" aria-label="Back to posts">
                                "\u{2190} Back"
                            </a>
                        })
                    } else {
                        Either::Right(view! {
                            <a class="brand" href="/" aria-label="Go to posts page">
                                <span class="brand-mark" aria-hidden="true">
                                    {home_icon()}
                                </span>
                            </a>
                        })
                    }}

                    <nav class="nav-pills" aria-label="Primary">
                        {nav_link("/", "Posts", posts_active, "nav-pill")}
                        {nav_link("/about", "About", active_page == ActivePage::About, "nav-pill")}
                    </nav>

                    <div class="nav-actions">
                        <a
                            class="icon-button"
                            href="/rss.xml"
                            aria-label="RSS feed"
                            title="RSS feed"
                        >
                            {rss_icon()}
                        </a>
                        <button
                            class="icon-button"
                            type="button"
                            data-theme-toggle=""
                            aria-label="Toggle color theme"
                            title="Toggle color theme"
                        >
                            {moon_icon()}
                            {sun_icon()}
                        </button>
                        <button
                            class="icon-button nav-toggle"
                            type="button"
                            data-menu-toggle=""
                            aria-label="Open navigation"
                        >
                            {menu_icon()}
                        </button>
                    </div>
                </div>
            </div>

            <div class="mobile-drawer" data-menu="" hidden=true>
                <div class="mobile-backdrop" data-menu-close=""></div>
                <aside class="mobile-panel" aria-label="Mobile navigation">
                    <div class="mobile-panel-header">
                        <div class="mobile-title">"Navigation"</div>
                        <button
                            class="icon-button"
                            type="button"
                            data-menu-close=""
                            aria-label="Close navigation"
                        >
                            {close_icon()}
                        </button>
                    </div>
                    <nav class="mobile-nav">
                        {nav_link("/", "Posts", posts_active, "")}
                        {nav_link("/about", "About", active_page == ActivePage::About, "")}
                        <a href="/rss.xml">"RSS feed"</a>
                    </nav>
                </aside>
            </div>
        </header>
    }
}
