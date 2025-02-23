use std::time::Duration;

fn main () {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx_future = async {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
    
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }    
        };

        let rx_future = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_future, rx_future).await;
    });
}