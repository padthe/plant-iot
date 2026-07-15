use rumqttc::{Client, Event, MqttOptions, Packet, QoS};
use std::time::Duration;

fn main() {
    let mut mqtt_options = MqttOptions::new("plant-controller", "localhost", 1883);

    mqtt_options.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut connection) = Client::new(mqtt_options, 10);

    let arguments: Vec<String> = std::env::args().collect();

    let command = match arguments.get(1) {
        Some(value) => match value.as_str() {
            "on" => "on",
            "off" => "off",
            _ => {
                println!("Unknown Command: {value}");
                return;
            }
        },
        None => {
            println!("Usage: controller <on|off>");
            return;
        }
    };

    client
        .publish("plant/led/set", QoS::AtLeastOnce, false, command.as_bytes())
        .expect("Failed to queue MQTT Message");

    for notification in connection.iter() {
        match notification {
            Ok(Event::Incoming(Packet::PubAck(_))) => {
                println!("Command set: {command}");
                break;
            }
            Ok(event) => println!("MQTT event: {event:?}"),
            Err(error) => {
                eprintln!("MQTT error: {error}");
                break;
            }
        }
    }
}
