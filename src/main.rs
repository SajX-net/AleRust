use std::io::BufReader;
use iron::prelude::*;
use router::Router;
// use std::process::Command;


fn play(_: &mut Request) -> IronResult<Response> {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("alarm.mp3").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
   
    // let mut command = Command::new("/Users/sajjad/workspace/alerting/gif/target/debug/gif"); // On Unix-like systems, use ls

    // Execute the command
    // let output = command.output().expect("Failed to execute command");

    // Check if the command was successful
    // if output.status.success() {
    //     // Convert the output bytes to a string (assuming UTF-8 encoding)
    //     let result = String::from_utf8_lossy(&output.stdout);
    //     println!("Command output:\n{}", result);
    // } else {
    //     // Command failed, print the error message
    //     let error_msg = String::from_utf8_lossy(&output.stderr);
    //     eprintln!("Command failed:\n{}", error_msg);
    // }

    Ok(Response::with((iron::status::Ok, "Playing alarm sound")))
}

fn main() {
    // Create a new Router
    let mut router = Router::new();
    router.get("/alert", play, "alert"); // Pass the `play` function as a closure

    // Create the Iron instance with the router and start the server
    let iron = Iron::new(router);
    println!("Server started on port 8080");
    iron.http("127.0.0.1:8080").unwrap();
}
