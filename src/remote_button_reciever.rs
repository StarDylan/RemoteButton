use rb::{get_config_or_exit, show_bytes, KEY_TOPIC};
use rdev::Key;
use rumqttc::Event::Incoming;
use rumqttc::Packet::Publish;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::process::exit;
use std::time::Duration;

#[tokio::main]
pub async fn main() -> ! {
    let cfg = get_config_or_exit();

    let mqtt_id = "reciever_".to_owned()
        + gethostname::gethostname()
            .to_str()
            .expect("Can get hostname");
    let mut mqttoptions = MqttOptions::new(mqtt_id, cfg.mqtt_server_host, cfg.mqtt_server_port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    if let Err(e) = client.subscribe(KEY_TOPIC, QoS::ExactlyOnce).await {
        eprintln!("Unable to Subscribe to {}\nError: {}", KEY_TOPIC, e);
        exit(1);
    }

    println!("Starting Receiving!");

    loop {
        match eventloop.poll().await {
            Err(err) => eprintln!("Error Recieving: {}", err),
            Ok(event) => {
                if let Incoming(Publish(publish_msg)) = event {
                    if publish_msg.topic.as_str() == KEY_TOPIC {
                        match serde_json::from_slice::<Key>(&publish_msg.payload) {
                            Ok(key) => {
                                if let Some(mapped_key) = cfg.key_map.recv_map.get(&key) {
                                    let _ = rdev::simulate(&rdev::EventType::KeyPress(*mapped_key));
                                    tokio::time::sleep(Duration::from_millis(10)).await;
                                    let _ = rdev::simulate(&rdev::EventType::KeyRelease(*mapped_key));
                                }
                            }

                            Err(e) => {
                                eprintln!(
                                    "Unable to deserialize message\nMessage: {}\nError: {}\n\n",
                                    show_bytes(&publish_msg.payload),
                                    e
                                )
                            }
                        }
                    }
                } else {
                    // Incorrect topic
                }
            }
        }
    }
}
