extern crate serde;
extern crate rmp_serde;

use std::thread;
use std::time::Duration;
use zmq::{Context, Socket, REQ};
use std::str;
use std::ffi::OsString;

use serde::{Serialize, Deserialize};
use rmp_serde::{to_vec, from_slice};


use hostname;
use whoami::lang;
use whoami::Platform;



#[derive(Debug, Serialize, Deserialize)]
struct send_command{
    start: String,
    start_command: String,
    pc_name: String,
    status: String,
    user_real_name: String,
    user: String,
    language: Vec<String>,
    platform: String,
    desktop:String,
    cpu:String,

}

trait InfoOp{
    fn get_sys_info(&mut self);
}


impl InfoOp for send_command {
    fn get_sys_info(&mut self) {
        self.pc_name=String::from(hostname::get().unwrap().to_str().unwrap());
        self.user_real_name=String::from(whoami::realname());
        self.user=String::from(whoami::username());
        self.language=whoami::lang().collect();
        self.platform= format!("{:?}", whoami::platform());
        self.desktop= format!("{:?}", whoami::desktop_env());
        self.cpu= format!("{:?}", whoami::arch());

    }
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
        status:String::from("none"),
        user_real_name:String::from("none"),
        user:String::from("none"),
        language: Vec::new(),
        platform:String::from("none"),
        desktop:String::from("none"),
        cpu:String::from("none"),
    };

    loop {
        let mut serial_d =rmp_serde::to_vec(&go).expect("Fehler beim Serialiezisen");
        socket_client.send(&serial_d, 0).unwrap();                                              //Sende Anmeldung SEND 1

        println!("Angemeldet:..");                                                                          //Erhalte Befehl
        println!("Warte auf die Anfrage vom Server:....");
        let mut get_data=socket_client.recv_bytes(0).unwrap();                              // recv 1
        println!("Daten erhatlen vom Server:..");
        let mut re_got_data:send_command=rmp_serde::from_slice(&get_data).expect("Fehler beim Deserialisieren:..");
        println!("Folgenden Befehl erhalten:...{:?}",re_got_data.start_command);



        match re_got_data.start_command.as_str(){                                               //Sende Antwort
            "info"=>{


                println!("Bei Info:...");
                go.pc_name=String::from(hostname::get().unwrap().to_str().unwrap());
                let mut serial_d =rmp_serde::to_vec(&go).expect("Fehler beim Serialiezisen");
                socket_client.send(&serial_d, 0).unwrap();                  //send 2 Anwort wird gesendet
                println!("Befehl wurde beantwortet und gesendet:...");                  // recv 2 geht danach zu send 1
                let mut get_data2=socket_client.recv_bytes(0).unwrap();

            }
            _=>{

                println!("Befehl unbekannt:...");
                                                                 //send 1
            }
            
        }

        


    }



}
