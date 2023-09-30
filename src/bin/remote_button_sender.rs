use device_query::DeviceState;
use device_query::DeviceEvents;
use remote_button::STARFIELD_TOPIC;
use remote_button::SendableKeys;
use rumqttc::{MqttOptions, Client, QoS};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::thread;

fn main() -> ! {

    
    let mut mqttoptions = MqttOptions::new("sender", "dstarserver.duckdns.org", 17974);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mut client, mut connection) = Client::new(mqttoptions, 1);
    
    
    let device_state = DeviceState::new();

    let things_to_send: Arc<Mutex<Vec<SendableKeys>>> = Arc::new(Mutex::new(Vec::new()));
    let key_down_mutex = Arc::clone(&things_to_send);

    let _guard = device_state.on_key_down(move |key| {
        if let Ok(key) = key.try_into() {
            let mut guard = key_down_mutex.lock().expect("Another thread did not panic while holding mutex");
            guard.push(key);
        }
    });

    thread::spawn(move || loop {
        match (&things_to_send).try_lock() {
            Ok(mut guard) => {
                guard.drain(0..).for_each(|key| {
                    let json_to_send = serde_json::to_string(&key).expect("Sendable Key can be serialized to json");
                    println!("Publish {:?}", json_to_send);
                    client.publish(STARFIELD_TOPIC, QoS::ExactlyOnce, false, json_to_send).expect("Able to send MQTT");
                });
            },
            Err(_) => {}
        }
    });


    // Iterate through the connection, which will run the Event Loop for MQTT
    loop {
        connection.iter().next();
    }
}