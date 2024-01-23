extern crate serde;
extern crate rmp_serde;

use std::thread;
use std::time::Duration;
use zmq::{Context, Socket, REQ};
use std::str;
use std::ffi::OsString;
use std::env;
use std::fs;

use serde::{Serialize, Deserialize};
use rmp_serde::{to_vec, from_slice};


use hostname;
use whoami::lang;
use whoami::Platform;


use local_ip_address::local_ip;         //ip staff
use get_if_addrs::{get_if_addrs, Interface};
use ifcfg::{self, InterfaceAddress};


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
    path: String,

}
#[derive(Debug, Serialize, Deserialize)]
struct info_network{
    interface_name:Vec<String>,
    int_mac: Vec<String>,
    int_description: Vec<String>,   
}

#[derive(Debug, Serialize, Deserialize)]
struct dir_info{
    dir_name: Vec<String>,
    Ordner_File: Vec<String>,
    file_size: Vec<String>,
    f_d_rights: Vec<String>,

}

trait dir_op{
    fn get_dir_info(&mut self, path: String);
 }
trait InfoOp{
    fn get_sys_info(&mut self);
}

trait IpOp{
    fn get_network_info(&mut self);
}

impl IpOp for info_network{
    fn get_network_info(&mut self) {
        let ifaces=ifcfg::IfCfg::get().expect("Fehler");
        for netw in ifaces{
            self.interface_name.push(netw.name);
            self.int_mac.push(netw.mac);
            self.int_description.push(netw.description);

            
        }
    }
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


impl dir_op for dir_info{

    fn get_dir_info(&mut self, path: String) {

       self.dir_name.clear();
       self.Ordner_File.clear();
       self.f_d_rights.clear();
       self.file_size.clear();
       println!("Folgender Pfad wurde gesendet: {:?}",path);

        if let Ok(entries)=fs::read_dir(path){
            println!("Pfad Ok");
            for entry_result in entries{
               if let Ok(entry)=entry_result{
                if let Some(file_name)=entry.file_name().to_str(){
                    //println!("{:?}",file_name);
                    self.dir_name.push(file_name.to_string());
    
                     match fs::metadata(entry.path()){
                        Ok(metadata)=>{
                            if metadata.is_file(){
                                self.Ordner_File.push("File".to_string());
                                self.file_size.push(metadata.len().to_string());
                            }else if metadata.is_dir(){
                                    self.Ordner_File.push("Verzeichnss".to_string());
                                    self.file_size.push("0".to_string());
                            }else{
                                self.Ordner_File.push("Unbekannt".to_string());
                                self.file_size.push("0".to_string());
                            }
    /* 
                            let mut permissions = metadata.permissions();
                            if permissions.readonly(){
                                dir.f_d_rights.push("read only".to_string());
                            }
                            */
                           // println!("Erfolgreich dir ermittelt: ");
                        }
                        Err(err)=>eprintln!("Fehler: {}",err),
                    }
                }
    
               }else {
                   println!("Fehler beim for schleife:..");
               }
            }
        }else{
            println!("Pfad existiert nicht:..");
            //println!("{:?}",path);
        }

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
        path:String::from("non"),
    };

    let mut network_info=info_network { 
        interface_name:Vec::new(),
        int_mac:Vec::new(),
        int_description:Vec::new(),
        
    };

    let mut dir=dir_info{
        dir_name: Vec::new(),
        Ordner_File:Vec::new(),
        file_size:Vec::new(),
        f_d_rights:Vec::new(),

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
               // go.pc_name=String::from(hostname::get().unwrap().to_str().unwrap());
                go.get_sys_info();
                let mut serial_d =rmp_serde::to_vec(&go).expect("Fehler beim Serialiezisen");
                socket_client.send(&serial_d, 0).unwrap();                  //send 2 Anwort wird gesendet
                println!("Befehl wurde beantwortet und gesendet:...");                  // recv 2 geht danach zu send 1
                let mut get_data2=socket_client.recv_bytes(0).unwrap();

            }
            "info network"=>{
                println!("Bei Network Info:...");
                network_info.get_network_info();
                let mut serial_d =rmp_serde::to_vec(&network_info).expect("Fehler beim Serialiezisen");
                socket_client.send(&serial_d, 0).unwrap(); 
                println!("Befehl wurde beantwortet und gesendet:..."); 
                let mut get_data2=socket_client.recv_bytes(0).unwrap();

            }
            "dir"=>{
                println!("Bei dir");
               println!("Pfad lautet: {:?}",re_got_data.path.trim());
               dir.get_dir_info(re_got_data.path);
               let mut serial_d =rmp_serde::to_vec(&dir).expect("Fehler beim Serialiezisen");
               socket_client.send(&serial_d, 0).unwrap(); 
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
