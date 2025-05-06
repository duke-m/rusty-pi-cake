use rusty_pi_cake::App;

fn main() {
    // Set the locale to English for the demo, will look for the locale in the `locales` folder. You could also try using
    // version 2 of the locale system, which will make it possible to put all locales in one file.
    rust_i18n::set_locale("en");
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
