use enigo::{Enigo, KeyboardControllable};
use remote_button::{SendableKeys, STARFIELD_TOPIC, show_bytes, NEW_PERSON_TOPIC, NewClient};
use rumqttc::{MqttOptions, Client, QoS};
use std::process::exit;
use std::time::Duration;
use rumqttc::Event::Incoming;
use rumqttc::Packet::Publish;

fn main() {
   
   let mut mqttoptions = MqttOptions::new("reciever", "dstarserver.duckdns.org", 17974);
   mqttoptions.set_keep_alive(Duration::from_secs(5));
   
   let (mut client, mut connection) = Client::new(mqttoptions, 10);
   if let Err(e) = client.subscribe(STARFIELD_TOPIC, QoS::ExactlyOnce) {
      eprintln!("Unable to Subscribe to {}\nError: {}", STARFIELD_TOPIC, e);
      exit(1);
   }

   if let Err(e) = client.subscribe(NEW_PERSON_TOPIC, QoS::ExactlyOnce) {
      eprintln!("Unable to Subscribe to {}\nError: {}",NEW_PERSON_TOPIC, e);
      exit(1);
   }
   
   let mut enigo = Enigo::new();
   
   // Iterate to poll the eventloop for connection progress
   for notification in connection.iter() {

       println!("Notification = {:?}", notification);

       if let Ok(event) = notification {
         if let Incoming(incoming_msg) = event {
            if let Publish(publish_msg) = incoming_msg {

               match publish_msg.topic.as_str() {
                  STARFIELD_TOPIC => {
                     match serde_json::from_slice::<SendableKeys>(&publish_msg.payload) {
                        Ok(key) => {
                           if let Ok(press_key) = key.try_into() {
                              enigo.key_click(press_key);
                           } 
                        }
      
                        Err(e) => {
                           eprintln!("Unable to deserialize message\nMessage: {}\nError: {}\n\n", show_bytes(&publish_msg.payload), e)
                        }
                     }
                  },

                  _ => {

                  }
               }

               
            }
         }
       }
   }
}