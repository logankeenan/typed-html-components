use typed_html::dom::DOMTree;
use typed_html::html;
use typed_html::text;
use typed_html::elements::FlowContent;


fn button(class: &str, button_text: &str) -> Box<dyn FlowContent<String>> {
    html!(
        <button class={class} type="button" > {text!(button_text)} </button>
    )
}

fn primary_button(button_text: &str) -> Box<dyn FlowContent<String>> {
    button("Primary", button_text)
}

fn secondary_button(button_text: &str) -> Box<dyn FlowContent<String>> {
    button("Secondary", button_text)
}


fn layout() -> String {
    let doc: DOMTree<String> = html!(
    <html>
        <head>
            <title>"typed-html-components"</title>
        </head>
        <body>
            {primary_button("Click Primary!")}
            {secondary_button("Click Secondary!")}
        </body>
    </html>
);
    return doc.to_string();
}

fn main() {
    println!("{}", layout());
}
