use clap::{command, value_parser, Arg, ArgAction, Command};
use simple_host_networks_manager_lib::{self, structs::NetworkInterface};

#[tokio::main]
async fn main() {
    let matches = command!()
        .about("update main embedded app")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("debug")
                .short('u')
                .long("debug")
                .long_help("debug values")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("connect")
                .about("connect one or all interfaces")
                .arg(Arg::new("intrerface").index(1)),
        )
        .subcommand(Command::new("scan").about("scan wifi networks"))
        .subcommand(
            Command::new("profile").about("manage profiles").subcommand(
                Command::new("action")
                    .about("profile actions")
                    .arg(
                        Arg::new("profile_name")
                            .short('p')
                            .long("prfile_name")
                            .long_help("profile name")
                            .value_parser(value_parser!(String)),
                    )
                    .arg(
                        Arg::new("network_interface")
                            .short('n')
                            .long("network_interface")
                            .long_help("network interface")
                            .value_parser(value_parser!(String)),
                    )
                    .arg(
                        Arg::new("wifi_ssid")
                            .short('s')
                            .long("wifi_ssid")
                            .long_help("wifi ssid")
                            .value_parser(value_parser!(String)),
                    )
                    .arg(
                        Arg::new("wifi_password")
                            .short('p')
                            .long("wifi_password")
                            .long_help("wifi password")
                            .value_parser(value_parser!(String)),
                    )
                    .arg(
                        Arg::new("apn_name")
                            .short('a')
                            .long("apn_name")
                            .long_help("apn name")
                            .value_parser(value_parser!(String)),
                    ),
            ),
        )
        .get_matches();

    if matches.get_flag("debug") {
        println!("debug args: {:?}", matches);
        return;
    }
    match matches.subcommand() {
        Some(("connect", matches)) => {
            if let Some(interface) = matches.get_one::<String>("intrerface") {
                let is_connected = simple_host_networks_manager_lib::connect(
                    None,
                    None,
                    Some(interface.to_owned()),
                    &None,
                    &None,
                )
                .await;
                if is_connected.is_ok() {
                    println!("interface {:?} connected", interface);
                } else {
                    println!("interface {:?} NOT connected", interface);
                }
            } else {
                let is_connected =
                    simple_host_networks_manager_lib::connect(None, None, None, &None, &None).await;
                if is_connected.is_ok() {
                    println!("connected");
                } else {
                    println!("not connected");
                }
            }
        }
        Some(("scan", _)) => {
            let nets = simple_host_networks_manager_lib::get_wifi_networks().await;
            if let Ok(nets) = nets {
                for net in nets {
                    println!("{:?}", net);
                }
            }
        }
        Some(("profile", matches)) => {
            if let Some(action) = matches.get_one::<String>("action") {
                match action.as_str() {
                    "list" => {
                        let list =
                            simple_host_networks_manager_lib::get_connection_profiles(None).await;
                        if let Ok(list) = list {
                            for profile in list {
                                println!("{:?}", profile);
                            }
                        }
                    }
                    "add" => {
                        let interface_name_string: &String;

                        let interface_name = matches.get_one::<String>("network_interface");
                        if let Some(interface_name) = interface_name {
                            interface_name_string = interface_name;
                        } else {
                            return println!("network_interface is required");
                        }

                        let profile_name_string: &String;
                        let profile_name = matches.get_one::<String>("profile_name");
                        if let Some(profile_name) = profile_name {
                            profile_name_string = profile_name;
                        } else {
                            return println!("profile_name is required");
                        }

                        let phisical =
                            simple_host_networks_manager_lib::get_all_networking_phisical_interfaces().await;

                        let mut interface_requested: Option<NetworkInterface> = None;

                        if let Ok(phisical) = phisical {
                            for network_interface in phisical {
                                if &network_interface.name == interface_name_string {
                                    interface_requested = Some(network_interface);
                                    break;
                                }
                            }
                        }
                        if interface_requested.is_none() {
                            return println!("network_interface not found");
                        }
                        let interface_requested = interface_requested.unwrap();

                        let wifi_ssid = matches.get_one::<String>("wifi_ssid").cloned();
                        let wifi_password = matches.get_one::<String>("wifi_password").cloned();
                        let apn_name = matches.get_one::<String>("apn_name").cloned();

                        let newprofile =
                            simple_host_networks_manager_lib::structs::NetworkConnectionProfile {
                                interface: interface_requested.name,
                                ipv4: None,
                                gateway: None,
                                priority: None,
                                name: profile_name_string.to_owned(),
                                network_type:
                                    simple_host_networks_manager_lib::structs::NetworkType::Ethernet,
                                wifi_connection_ssid: wifi_ssid,
                                wifi_connection_password: wifi_password,
                                ppp_apn_connection_name: apn_name,
                            };

                        let result = simple_host_networks_manager_lib::set_connection_profile(
                            None, newprofile,
                        )
                        .await;
                        if let Ok(result) = result {
                            println!("profile added: {:?}", result);
                            _ = simple_host_networks_manager_lib::connect(
                                None,
                                None,
                                Some(interface_name_string.to_owned()),
                                &None,
                                &None,
                            )
                            .await;
                        } else {
                            println!("profile not added: {:?}", result);
                        }
                    }
                    "remove" => {
                        let profile_name = matches.get_one::<String>("profile_name");
                        if let Some(profile_name) = profile_name {
                            let result =
                                simple_host_networks_manager_lib::remove_connection_profile(
                                    None,
                                    profile_name,
                                )
                                .await;
                            if let Ok(result) = result {
                                println!("profile removed: {:?}", result);
                            } else {
                                println!("profile not removed: {:?}", result);
                            }
                        } else {
                            println!("profile_name is required");
                        }
                    }
                    _ => {
                        println!("unknown action: {}", action);
                    }
                }
            }
        }
        _ => {
            println!("unknown command");
        }
    }
}
