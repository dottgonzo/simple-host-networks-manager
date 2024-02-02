use simple_host_networks_manager_lib;
use clap::{command, value_parser, Arg, ArgAction};


#[tokio::main]
async fn main(){
    let args = command!()
        .about("update main embedded app")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("debug")
                .short('u')
                .long("debug")
                .long_help("debug values")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("wifiscan")
                .short('s')
                .long("wifiscan")
                .long_help("wifiscan")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("connect")
                .short('c')
                .long("connect")
                .long_help("connect")
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    if args.get_flag("debug") {
        println!("debug args: {:?}", args);
        return
    }

    let _ = simple_host_networks_manager_lib::get_wifi_networks().await;
    let interfaces = simple_host_networks_manager_lib::get_all_networking_phisical_interfaces().await;

    println!("{:?}", interfaces);

}