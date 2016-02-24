extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
rustOver.

Pushover client in rust.
Create a pushover application in your Pushover Dashboard, grab the token and user token and send a message.
Take a look at -h for extra options

Usage:
    rustOver <token> <user-token> <message> [options]
    rustOver (-h | --help)

Options
  --title=<title>          Title of the message, otherwise the App name is used.
  --devices=<devices>      List of devices to send the message to, seperated by comma.
  --url=<url>              Supplementary url to show with your message.
  --url-title=<url_title>  A title for your supplementary URL, otherwise just the url is shown.
  --priority=<n>           Priority of the message -2..2 (low..high).
  --timestamp=<unix_t>     Unix timestamp to be added to the message.
  --sound=<sound>	   Sound to be played on receiving the message.
  --use-html               Enable the usage of HTML in Message. 
  -h --help     Show this screen.
  -v            Verbose mode
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_title: Vec<String>,
    flag_devices: Vec<String>,
    flag_url: Vec<String>,
    flag_url_title: Vec<String>,
    flag_priority: Vec<String>,
    flag_timestamp: Option<i64>,
    flag_sound: Vec<String>,
    flag_use_html: bool,
    flag_v: bool,
    arg_token: Vec<String>,
    arg_user_token: Vec<String>,
    arg_message: Vec<String>
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.flag_v
    {
	println!("{:?}", args);
    }
}

