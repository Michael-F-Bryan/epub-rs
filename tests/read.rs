extern crate epub;

use epub::doc::EpubDoc;

#[test]
fn read_doc() {
    let input_file = "tests/docs/Metamorphosis-jackson.epub";
    let doc = EpubDoc::new(input_file);
    assert!(doc.is_ok());
    let mut doc = doc.unwrap();

    if let Some(title) = doc.metadata.get("title") {
        println!("Book title: {}", title);
    } else {
        println!("Book title not found");
    }
    println!("Num Pages: {}\n", doc.get_num_pages());

    {
        println!("resources:\n");
        for (k, v) in doc.resources.iter() {
            println!("{}: {}\n * {}\n", k, v.1, v.0);
        }
        println!("");

    }

    while let Ok(_) = doc.go_next() {
        println!("ID: {}", doc.get_current_id().unwrap());
        let current = doc.get_current_str();
        match current {
            Ok(v) => println!("Value {:?}\n", v),
            Err(e) => println!("Text Err {:?}\n", e.description())
        }
    }
}