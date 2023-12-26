use rtrend::{Client, Country, Keywords, SearchInterest};

fn main() {
    let keywords = Keywords::new(vec!["Ethereum", "Solana", "Bitcoin", ]);
    let country = Country::ALL;

    let client = Client::new(keywords, country).build();

    let search_interest = SearchInterest::new(client).get();
    println!("{}", search_interest);
}
