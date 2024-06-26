use ninjabook::{event::Event, orderbook::Orderbook};

fn main() {
    let mut reader = csv::Reader::from_path("./data/norm_book_data_300k.csv").unwrap();
    let tick_size = 0.01;
    let mut ob = Orderbook::new(tick_size);

    let mut rdr = reader.deserialize::<Event>();

    while let Some(Ok(event)) = rdr.next() {
        ob.process(event);

        // or if its a raw event
        // ob.process_raw(
        //     event.timestamp,
        //     event.seq,
        //     event.is_trade,
        //     event.is_buy,
        //     event.price,
        //     event.size,
        // );

        println!(
            "{:?} -- {:?} -- {:?}",
            ob.best_bid(),
            ob.midprice(),
            ob.best_ask()
        );
    }
}
