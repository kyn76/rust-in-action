use reqwest;
use std::env;
use scraper::{Html, Selector, ElementRef};

#[tokio::main]
async fn main() {
    // Retrieve command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <mot>", args[0]);
        return;
    }
    let word = &args[1];
    let wiki_url = format!("https://fr.wiktionary.org/wiki/{}", word);

    let body = reqwest::get(wiki_url)
        .await
        .unwrap()
        .text()
        .await;
    
    let document = Html::parse_document(body.unwrap().as_str());
    let selector = Selector::parse("h3").unwrap();
    let h3s = document.select(&selector);
    let mut etymological_h3: Option<ElementRef>  = None;
    for h3 in h3s {
        if is_etymological_h3(&h3) {
            etymological_h3 = Some(h3);
        }
    }
    if etymological_h3.is_none() {
        println!("Pas de section étymologique pour le mot '{}'.", word);
        std::process::exit(1);
    }
    let etymological_h3 = etymological_h3.unwrap();
    let mut etym_sections: Vec<String> = Vec::new();
    for next_sibling in etymological_h3.next_siblings() {
        if next_sibling.value().is_element() && next_sibling.value().as_element().unwrap().name() == "h3" {
            break;
        } else if next_sibling.value().is_element() {
            let element_ref = ElementRef::wrap(next_sibling).unwrap();
            let text = element_ref.text().collect::<String>();
            if text != "" {
                etym_sections.push(text);
            }
        }
    }

    if etym_sections.len() == 0 {
        println!("Pas de section étymologique pour le mot '{}'.", word);
        std::process::exit(1);
    } else if etym_sections.len() == 1 {
        println!("\n{}\n", etym_sections[0]);
    } else {
        for (index, etym_section) in etym_sections.iter().enumerate() {
            println!("\n== {} ==", index+1);
            println!("{}\n", etym_section);
        }
    }
}

fn is_etymological_h3(h3: &ElementRef) -> bool {
    for desc in h3.descendent_elements() {
        match desc.attr("class") {
            Some(class) => {
                if class == "titreetym" {
                    return true;
                }
            }
            None => {continue;}
        }
    }
    false
}