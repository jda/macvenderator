extern crate eui48;

use std::io::prelude::*;

use rustop::opts;

use oui::OuiDatabase;

fn main() {
    let (args, _rest) = opts! {
        synopsis "look up vendor info for mac addresses";
        opt manuf:String="manuf.txt".to_string(), desc:"Wireguard OUI db";
        opt summary:bool, desc:"print summary";
    }
    .parse_or_exit();

    let db = OuiDatabase::new_from_file(&args.manuf).unwrap();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let macaddr = match eui48::MacAddress::parse_str(&line.unwrap()) {
            Ok(macaddr) => macaddr,
            Err(_macaddr) => {
                continue;
            }
        };

        let res = match db.query_by_mac(&macaddr) {
            Ok(res) => res,
            Err(_res) => {
                continue;
            }
        };

        let oui = match res {
            Some(oui) => oui,
            None => {
                continue;
            }
        };

        println!("{} {:#?}", macaddr.to_hex_string(), oui.name_short);
    }
}
