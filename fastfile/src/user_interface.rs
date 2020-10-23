use ssh2::Session;
use std::fs;
use std::io::stdin;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;

pub fn list_all_remote_addresses(addrs: &[String]) {
    println!("The remote addresses are here: ");
    for (index, addr) in addrs.iter().enumerate() {
        println!("{}-{}", index + 1, addr);
    }
}

pub fn promote_user_input_index(promote: &str) -> usize {
    println!("{}", promote);

    let mut user_choose = String::new();
    stdin()
        .read_line(&mut user_choose)
        .expect("User not enter a corret thing");
    let choose_index: usize = user_choose.trim().parse().unwrap();
    choose_index
}

pub fn search_remote_files_under_some_path(
    remote_addr: &str,
    remote_path: &str,
    file_name: &str,
) -> String {
    let mut exec_cmd = String::new();
    exec_cmd.push_str("find -L ");
    exec_cmd.push_str(remote_path);
    exec_cmd.push_str(" -name *");
    exec_cmd.push_str(file_name);
    exec_cmd.push_str("*");

    println!("I will exec remote command");

    exec_ssh_remote_command(remote_addr, &exec_cmd)
}

pub fn upload_file_to_remote_mechine(addrinfo: &str, local_file_path: &str, remote_file_dir: &str) {
    let (user, hostaddr, passwd) = parse_addr_elements(addrinfo);

    println!("-----------Remote Mechine Info---------------");
    println!("User: {}", user);
    println!("HostAddr: {}", hostaddr);
    println!("PassWord: {}", passwd);
    println!("---------------------------------------------");

    let mut addr_port = String::new();
    addr_port.push_str(&hostaddr);
    addr_port.push_str(":22");

    let tcp = TcpStream::connect(&addr_port).unwrap();
    let mut sess = Session::new().unwrap();

    sess.handshake(&tcp).unwrap();
    sess.userauth_password(&user, &passwd).unwrap();

    let local_path = Path::new(local_file_path);
    let local_file_name = local_path.file_name().unwrap().to_str().unwrap();

    let data = fs::read(local_path).expect("unable to read file");

    let mut remote_file = sess
        .scp_send(
            &Path::new(remote_file_dir).join(local_file_name),
            0o644,
            data.len() as u64,
            None,
        )
        .unwrap();

    remote_file.write(&data).unwrap();
}

pub fn exec_ssh_remote_command(addrinfo: &str, cmd: &str) -> String {
    let (user, hostaddr, passwd) = parse_addr_elements(addrinfo);

    println!("-----------Remote Mechine Info---------------");
    println!("User: {}", user);
    println!("HostAddr: {}", hostaddr);
    println!("PassWord: {}", passwd);
    println!("---------------------------------------------");

    let mut addr_port = String::new();
    addr_port.push_str(&hostaddr);
    addr_port.push_str(":22");

    let tcp = TcpStream::connect(&addr_port).unwrap();
    let mut sess = Session::new().unwrap();

    sess.handshake(&tcp).unwrap();
    sess.userauth_password(&user, &passwd).unwrap();

    println!("{}", cmd);

    let mut channel = sess.channel_session().unwrap();
    channel.exec(cmd).unwrap();

    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    s
}

fn parse_addr_elements(addr: &str) -> (String, String, String) {
    let v: Vec<&str> = addr.split(':').collect();
    (String::from(v[0]), String::from(v[1]), String::from(v[2]))
}
