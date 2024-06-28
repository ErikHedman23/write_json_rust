use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}
//function for reading each line in a json object:
fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}

fn main() {
    //Creating JSON

    let article: Article = Article {
        article: String::from("How to work with JSON in Rust"),
        author: String::from("Erik Hedman"),
        paragraph: vec![
            Paragraph {
                name: String::from("first sentence"),
            },
            Paragraph {
                name: String::from("second paragraph"),
            },
            Paragraph {
                name: String::from("third paragraph"),
            },
        ],
    };

    //Parsing the json object to a string and printing the Json as a string to the console:

    let json = serde_json::to_string(&article).unwrap();
    println!("the json is : {}", json);

    //Parsing the JSON back to a string and printing each line and the first line to the console:

    let parsed: Article = read_json_typed(&json);

    for line in &parsed.paragraph {
        println!("\n\nThe name of this line is : {}", line.name)
    }

    println!(
        "\n\n The name of the first paragraph is : {}",
        parsed.paragraph[0].name
    );
}
