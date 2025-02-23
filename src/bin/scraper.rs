use trpl::Html;

async fn page_title(url: &str) -> (&str, Option<String>) {
    //let response = trpl::get(url).await;
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());

    (url, title)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::run(async {
        let title_future_1 = page_title(&args[1]);
        let title_future_2 = page_title(&args[2]);

        let (url, maybe_title) = match trpl::race(title_future_1, title_future_2).await {
            trpl::Either::Left(l) => l,
            trpl::Either::Right(r) => r,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could no be parsed.")
        }
    })
}
