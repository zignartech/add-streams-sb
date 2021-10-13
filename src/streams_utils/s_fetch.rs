use iota_streams::{
    app::message::HasLink as _,
    app_channels::api::tangle::{
        Author,
        Subscriber,
        Transport,
    },
};
use iota_streams::app::transport::tangle::client::Client;
pub async fn s_fetch_next_messages(subscriber: &mut Subscriber<Client>) {
    let mut exists = true;

    while exists {
        let msgs = subscriber.fetch_next_msgs().await;
        exists = false;

        for msg in msgs {
            println!("Message exists at {}... ", &msg.link.rel());
            exists = true;
        }

        if !exists {
            println!("No more messages in sequence.");
        }
    }
}

pub async fn a_fetch_next_messages(author: &mut Author<Client>) {
    let mut exists = true;

    while exists {
        let msgs = author.fetch_next_msgs().await;
        exists = false;

        for msg in msgs {

            println!("Message exists at {}... ", &msg.link.rel());
            exists = true;
        }

        if !exists {
            println!("No more messages in sequence.");
        }
    }
}
