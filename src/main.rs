use chrono::TimeZone;
use serde::Deserialize;
use std::io::BufRead;

// Color scheme borrowed from ffvrc (except for a darker, less red gray)
const RESET  : &str = "\x1b[0m";
const BOLD   : &str = "\x1b[1m";
const GRAY   : &str = "\x1b[38;2;128;144;144m"; // #809090
const MAGENTA: &str = "\x1b[38;2;255;0;255m"  ; // #f0f
const VIOLET : &str = "\x1b[38;2;204;102;255m"; // #c6f
const BLUE   : &str = "\x1b[38;2;68;170;221m" ; // #4ad
const AQUA   : &str = "\x1b[38;2;26;186;151m" ; // #1aba97
const GREEN  : &str = "\x1b[38;2;68;221;68m"  ; // #4d4
const ORANGE : &str = "\x1b[38;2;236;182;74m" ; // #ecb64a
const RED    : &str = "\x1b[38;2;255;0;0m"    ; // #f00

#[derive(Deserialize)]
struct CaddyLog {
  ts: f64,
  logger: String,
  msg: String,
  request: Request,
  status: u16,
}

#[derive(Deserialize)]
struct Request {
  proto: String,
  method: String,
  uri: String,
  remote_ip: String,
  remote_port: String,
}

fn main() {
  if std::env::args().count() > 1 {
    println!("\
{BOLD}{BLUE}USAGE{RESET}
        {BOLD}{AQUA}calopr{RESET} currently accepts no arguments. It reads Caddy
        JSON logs from stdin and writes colorized human-readable
        lines to stdout.

{BOLD}{BLUE}EXAMPLES{RESET}
        {GRAY}# View entire log{RESET}
        cat /var/lib/caddy/access.log | {BOLD}{AQUA}calopr{RESET}
        
        {GRAY}# View last 10 entries and watch for updates{RESET}
        tail -n 10 /var/lib/caddy/access.log | {BOLD}{AQUA}calopr{RESET}"
    );
    std::process::exit(1);
  }
  
  for line_result in std::io::stdin().lock().lines() {
    match line_result {
      Ok(line) => println!("{}", format_log(line)),
      Err(e) => println!("{BOLD}{RED}Error{RESET} {ORANGE}reading line:{RESET} \
        {}", e),
    }
  }
}

fn format_log(line: String) -> String {
  let log_entry = match serde_json::from_str::<CaddyLog>(&line) {
    Ok(log_entry) => log_entry,
    Err(e) => return format!("{BOLD}{RED}Error{RESET} \
      {ORANGE}parsing JSON:{RESET} {}", e),
  };
  
  if !log_entry.logger.starts_with("http.log.access") {
    return format!("{BOLD}{RED}Error{RESET} {ORANGE}handling log entry:{RESET} \
      Unsupported logger `{}` (only `http.log.access.*` supported)",
      log_entry.logger);
  }
  
  if log_entry.msg != "handled request" {
    return format!("{BOLD}{RED}Error{RESET} {ORANGE}handling log entry:{RESET} \
      Unsupported message type `{}` (only `handled request` supported)",
      log_entry.msg);
  }
  
  //let timestamp = match chrono::Utc.timestamp_opt(
  let timestamp = match chrono::Local.timestamp_opt(
    log_entry.ts.trunc() as i64,
    (log_entry.ts.fract()*1e+9) as u32,
  ) {
    chrono::offset::LocalResult::Single(timestamp)  =>
      // Using consts for the ANSI codes here would be excessively complicated
      timestamp.format("%b %d \x1b[1m%H:%M:%S\x1b[22m.%3f"),
    _ => return format!("{BOLD}{RED}Error{RESET} \
      {ORANGE}parsing timestamp:{RESET} This shouldn't be possible"),
  };
  
  let mut uri = log_entry.request.uri.replace("/",
    &format!("{GRAY}/{VIOLET}"));
  if let Some(i_period) = uri.rfind(".") {
    if i_period >= uri.rfind("/").unwrap_or(0) {
      uri.insert_str(i_period, MAGENTA);
    }
  };
  
  let status = match http::StatusCode::from_u16(log_entry.status) {
    Ok(status) => format!("{}", status),
    Err(_) => format!("{} Unknown HTTP Code", log_entry.status)
  };
  let status_color = match log_entry.status {
    0..300 => GREEN,
    300..400 => ORANGE,
    _ => RED,
  };
  
  // Because inline values in format strings only allow identifiers, not
  // expressions
  let remote_ip   = log_entry.request.remote_ip;
  let remote_port = log_entry.request.remote_port;
  let method      = log_entry.request.method;
  let proto       = log_entry.request.proto;
  
  return format!(
    "{AQUA}{timestamp} \
    {GRAY}{remote_ip:>15}:{remote_port:<5}{RESET} - \
    {BLUE}{BOLD}{method}{RESET} \
    {VIOLET}{uri} \
    {BLUE}{proto} {status_color}{status}{RESET}"
  );
}
