fn main() {
    let article_1 = Article {
        title: "Rust is great".to_string(),
        body: "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.".to_string(),
    };
    notify(&article_1);
    let tweet_1 = Tweet {
        username: "rustacean".to_string(),
        content: "Rust is a great".to_string(),
    };
    notify(&tweet_1);
}

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    body: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Title {}: {}", self.title, self.body)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Tweet by {}: {}", self.username, self.content)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}