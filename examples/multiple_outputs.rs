// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example multiple_outputs --release
use iota::{Client, MessageId, Seed, Transfers};
use std::{num::NonZeroU64, time::Duration};
use tokio::time::delay_for;
/// In this example, we send 900 tokens to the following 3 locations, respectively
///
/// Address Index 0
///   output 0: 300 tokens iot1q86rlrygq5wcgdwt7fpajaxxppc49tg0jk0xadnp66fsfjtwt8vgc48sse6
///   output 1: 300 tokens iot1qyg7l34etk4sdfrdt46vwt7a964avk9sfrxh8ecq2sgpezaktd55cyc76lc
///   output 2: 300 tokens iot1q9r5hvlppf44gvcxnuue4dwjtjcredrw6yesphqeq7fqm2fyjy6kul4tv5r
///
///
/// These two addresses belong to seed "256a818b2aac458941f7274985a410e57fb750f3a3a67369ece5bd9ae7eef5b0"

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .node("https://api.lb-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.
    // First address from the seed below is iot1qxt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupxgecea4
    let seed = Seed::from_ed25519_bytes(
        &hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2").unwrap(),
    )
    .unwrap();

    let mut transfers = Transfers::new(
        "iot1q86rlrygq5wcgdwt7fpajaxxppc49tg0jk0xadnp66fsfjtwt8vgc48sse6",
        NonZeroU64::new(300).unwrap(),
    )
    .unwrap();
    transfers
        .add(
            "iot1qyg7l34etk4sdfrdt46vwt7a964avk9sfrxh8ecq2sgpezaktd55cyc76lc",
            NonZeroU64::new(300).unwrap(),
        )
        .unwrap();
    transfers
        .add(
            "iot1q9r5hvlppf44gvcxnuue4dwjtjcredrw6yesphqeq7fqm2fyjy6kul4tv5r",
            NonZeroU64::new(300).unwrap(),
        )
        .unwrap();

    let message_id = iota
        .send()
        .transaction(&seed)
        .account_index(0)
        .outputs(transfers)
        .post()
        .await
        .unwrap();

    println!(
        "Transaction sent: https://explorer.iota.org/chrysalis/message/{}",
        message_id
    );
    reattach_promote_until_confirmed(message_id, &iota).await;
}

async fn reattach_promote_until_confirmed(message_id: MessageId, iota: &Client) {
    while let Ok(metadata) = iota.get_message().metadata(&message_id).await {
        if let Some(state) = metadata.ledger_inclusion_state {
            println!("Leder inclusion state: {}", state);
            break;
        } else {
            match iota.reattach(&message_id).await {
                Ok(msg_id) => println!("Reattached or promoted {}", msg_id.0),
                _ => {}
            }
        }
        delay_for(Duration::from_secs(5)).await;
    }
}