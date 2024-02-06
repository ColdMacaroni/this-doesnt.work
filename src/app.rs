use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <script>
            // Get rid of the no-javascript message, won't need it.
            document.getElementById("no-javascript").remove();
        </script>
        <Router fallback=|| view! { <p>"This page doesn't exist :("</p> }.into_view()>
            <h1>this-doesnt.work</h1>
            <Routes>
                <Route path="/" view=|| view! { <p>"Welcome to my site :))"</p> }/>
            </Routes>
            <Footer/>
        </Router>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer>
            <a href="https://neovim.io"><img src="/assets/nvim-badge.gif" alt="Made with neovim" /></a>
        </footer>
    }
}
