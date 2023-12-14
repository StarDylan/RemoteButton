use rb::get_config_or_exit;
use rb::KEY_TOPIC;
use rdev::Event;
use rdev::Key;
use rumqttc::AsyncClient;
use rumqttc::{MqttOptions, QoS};
use std::collections::HashMap;
use std::process::exit;
use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main]
pub async fn main() -> ! {
    let cfg = get_config_or_exit();

    // Setup Global key listener
    let (key_event_tx, mut key_event_rx) = mpsc::channel(100);

    let static_key_event_tx = Box::leak(Box::new(key_event_tx));
    let sender_set: &mut HashMap<Key, Key> =
        Box::leak(Box::new(cfg.key_map.send_map));

    let global_key_event_callback = |event: Event| {
        // We only care about keyboard keys being pressed down, not mouse
        if let rdev::EventType::KeyPress(key) = event.event_type {
            if let Some(mapped_key) = sender_set.get(&key) {
                let _ = static_key_event_tx.try_send(mapped_key);
            }
        };
    };

    tokio::task::spawn_blocking(move || {
        if let Err(err) = rdev::listen(global_key_event_callback) {
            eprintln!("Rdev listen error: {:?}", err);
            exit(1);
        }
    });

    let mqtt_id = "sender_".to_owned()
        + gethostname::gethostname()
            .to_str()
            .expect("Can get hostname");

    let mut mqttoptions = MqttOptions::new(mqtt_id, cfg.mqtt_server_host, cfg.mqtt_server_port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 1);

    println!("Starting sending!");

    loop {
        tokio::select! {
            key_event = key_event_rx.recv() => {
                if key_event.is_none() {
                    eprint!("Global event MPSC closed");
                    exit(1);
                }
                let key_event = key_event.unwrap();



                let json_to_send = serde_json::to_string(&key_event).expect("Key EventType can be seralized to JSON");

                client.publish(KEY_TOPIC, QoS::ExactlyOnce, false, json_to_send).await.expect("Able to send MQTT");
            }

            notif = eventloop.poll() => {
                match notif {
                    Err(err) =>  {
                        eprintln!("Error when polling MQTT! Err: {}", err);
                        tokio::time::sleep(Duration::from_secs(2)).await;
                    },
                    Ok(evnt) =>
                        if let rumqttc::Event::Outgoing(out) = evnt {
                            match out {
                                rumqttc::Outgoing::Subscribe(sub) => println!("Subscribed to {}", sub),
                                rumqttc::Outgoing::Disconnect => eprintln!("Disconnected"),
                                _ => {}
                            }
                        }
                };
            }
        }
    }
}
