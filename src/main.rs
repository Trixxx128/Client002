extern crate serde;
extern crate rmp_serde;

use std::thread;
use std::time::Duration;
use zmq::{Context, Socket, REQ};
use std::str;

use serde::{Serialize, Deserialize};
use rmp_serde::{to_vec, from_slice};



#[derive(Debug, Serialize, Deserialize)]
struct send_command{
    start: String,
    start_command: String,
    pc_name: String,

}




fn main() {
    println!("Client V00.2");

    let context=Context::new();
    let socket_client=context.socket(REQ).unwrap();
    socket_client.connect("tcp://127.0.0.1:5000").unwrap();
    println!("Client connected");

     let mut go=send_command{
        start:String::from("online"),
        start_command:String::from("none"),
        pc_name:String::from("none"),
    };

    loop {
        let mut serial_d =rmp_serde::to_vec(&go).expect("Fehler beim Serialiezisen");
        socket_client.send(&serial_d, 0).unwrap();                                              //Sende Anmeldung

        println!("Angemeldet:..");                                                                          //Erhalte Befehl
        println!("Warte auf die Anfrage vom Server:....");
        let mut get_data=socket_client.recv_bytes(0).unwrap();
        println!("Daten erhatlen vom Server:..");
        let mut re_got_data:send_command=rmp_serde::from_slice(&get_data).expect("Fehler beim Deserialisieren:..");
        println!("Folgenden Befehl erhalten:...{:?}",re_got_data.start_command);



        match re_got_data.start_command.as_str(){                                               //Sende Antwort
            "info"=>{
                println!("Bei Info:...");
                go.pc_name=String::from("PC name lautet: Hans");
                let mut serial_d =rmp_serde::to_vec(&go).expect("Fehler beim Serialiezisen");
                socket_client.send(&serial_d, 0).unwrap();
                println!("Befehl wurde beantwortet und gesendet:...");
                let mut get_data2=socket_client.recv_bytes(0).unwrap();

            }
            _=>println!("Befehl unbekannt:..."),
        }

        







    }



}
