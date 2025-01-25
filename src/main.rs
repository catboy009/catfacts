#![allow(non_snake_case)]

use css_in_rs::{make_styles, use_style_provider_quickstart, Classes, EmptyTheme};
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};
use serde::Deserialize;
use web_sys::window;

fn main() {
    dioxus_sdk::storage::set_dir!();
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    dioxus::launch(App);
}

#[derive(Debug, Deserialize, Clone)]
pub struct Fact {
    pub data: Vec<String>,
}

async fn get_catfacts(lang: String) -> reqwest::Result<Fact> {
    reqwest::get(format!(
        "https://meowfacts.herokuapp.com/?count=5&lang={}",
        lang,
    ))
    .await?
    .json::<Fact>()
    .await
}

make_styles! {
    (_theme: EmptyTheme) -> MyClasses {
        ":root" {
            __rosewater: "#ff8389",
            __flamingo: "#ff8389",
            __red: "#ff8389",
            __maroon: "#ff8389",
            __pink: "#ff7eb6",
            __mauve: "#be95ff",
            __peach: "#d44a1c",
            __yellow: "#ab8600",
            __green: "#08bdba",
            __teal: "#33b1ff",
            __sky: "#33b1ff",
            __sapphire: "#33b1ff",
            __blue: "#78a9ff",
            __lavender: "#78a9ff",
            __text: "#ffffff",
            __subtext1: "#f4f4f4",
            __subtext0: "#e0e0e0",
            __overlay2: "#adadad",
            __overlay1: "#949494",
            __overlay0: "#7a7a7a",
            __surface2: "#4f4f4f",
            __surface1: "#383838",
            __surface0: "#2e2e2e",
            __base: "#161616",
            __mantle: "#0d0d0d",
            __crust: "#000000",
        },
    "@media (prefers-color-scheme: light)" {
        ":root" {
            __rosewater: "#da1e28",
            __flamingo: "#da1e28",
            __red: "#da1e28",
            __maroon: "#da1e28",
            __pink: "#d02670",
            __mauve: "#8a3ffc",
            __peach: "#d44a1c",
            __yellow: "#ab8600",
            __green: "#007d79",
            __teal: "#1192e8",
            __sky: "#1192e8",
            __sapphire: "#1192e8",
            __blue: "#0f62fe",
            __lavender: "#0f62fe",
            __text: "#000000",
            __subtext1: "#404040",
            __subtext0: "#474747",
            __overlay2: "#575757",
            __overlay1: "#595959",
            __overlay0: "#737373",
            __surface2: "#8c8c8c",
            __surface1: "#d1d1d1",
            __surface0: "#e6e6e6",
            __base: "#ffffff",
            __mantle: "#f2f2f2",
            __crust: "#ebebeb",
            }
        },
        ":root" {
            background_color: "var(--base)",
            color: "var(--text)",
            line_height: "1.6",
            font_family: "Cartograph CF",
        },
        "@media (hover: hover) and (pointer: fine)" {
            ".animated_list li" {
                all: "unset",
                transition_property: "all",
                transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
                transition_duration: "300ms",
            },
            ".animated_list:hover li" {
                opacity: "0.5",
            },
            ".animated_list:hover li:hover" {
                opacity: "1",
            }
        },
        ".underlined" {
            all: "unset",
            cursor: "pointer",
            text_decoration_line: "underline",
            text_decoration_thickness: "2px",
            text_underline_offset: "4px",
            transition_property: "color, background-color, border-color, text-decoration-color, fill, stroke",
            transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
            transition_duration: "300ms",
            text_decoration_color: "var(--surface2)",
        },
        ".underlined:hover" {
            text_decoration_color: "var(--overlay2)",
        },
        ".underlined:active" {
            text_decoration_color: "var(--overlay1)",
        },
        ".main" {
            margin_left: "auto",
            margin_right: "auto",
            max_width: "768px",
            padding_left: "24px",
            padding_right: "24px",
            padding_bottom: "80px",
            padding_top: "64px",
        },
        "@media(min-width: 950px)" {
            ".main"{
                padding_top: "320px",
            }
        }
    }
}

#[component]
fn App() -> Element {
    use_style_provider_quickstart(|| EmptyTheme);
    let cls: &MyClasses = &MyClasses::use_style();

    let mut language =
        use_synced_storage::<LocalStorage, String>("language".to_string(), || "eng".to_string());
    let facts = use_resource(move || async move { get_catfacts(language.to_string()).await });

    rsx! {
        style {
            "
            @font-face {{
            font-family: 'Cartograph CF';
            src: url('./assets/fonts/CartographCF-Regular.woff2') format('woff2');
            }}
            @font-face {{
            font-family: 'Cartograph CF';
            src: url('./assets/fonts/CartographCF-RegularItalic.woff2') format('woff2');
            font-style: italic;
            }}
            @font-face {{
            font-family: 'Cartograph CF';
            src: url('./assets/fonts/CartographCF-Bold.woff2') format('woff2');
            font-weight: bold;
            }}"
        }
        main { class: &cls.main as &str,
            div {
                h1 {
                    font_size: "20px",
                    margin_bottom: "32px",
                    font_style: "bold",
                    "cat facts"
                }
                ul {
                    class: &cls.animated_list as &str,
                    display: "flex",
                    flex_direction: "column",
                    list_style_type: "none",
                    padding: "unset",
                    if let Some(Ok(data)) = facts.read().as_ref() {
                        {data.data.iter().map(|fact| rsx! {
                            li { padding_bottom: "32px", font_style: "italic", "\"{fact}\"" }
                        })}
                    }
                }
                div { display: "flex",
                    button {
                        class: &cls.underlined as &str,
                        margin_right: "auto",
                        onclick: move |_| {
                            if let Some(win) = window() {
                                win.location().reload().expect("Failed to reload the page");
                            }
                        },
                        "refresh"
                    }
                    button {
                        color: if language.to_string() != "eng" { "var(--overlay0)" },
                        class: &cls.underlined as &str,
                        margin_right: "14px",
                        onclick: move |_| language.set("eng".to_string()),
                        "eng"
                    }
                    button {
                        color: if language.to_string() != "rus" { "var(--overlay0)" },
                        class: &cls.underlined as &str,
                        onclick: move |_| language.set("rus".to_string()),
                        "rus"
                    }
                }
            }
        }
    }
}
