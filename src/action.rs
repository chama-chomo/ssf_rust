use rexpect;
use rexpect::spawn;
use rexpect::session::PtyReplSession;
use rexpect::errors::*;
use rexpect::process::wait::WaitStatus;
use std::io;
use regex::Regex;
use std::io::prelude::*;

pub enum ConnectAction {
    SshConnect,
    IloSshConnect,
    IloWebConnect,
}

impl ConnectAction {
    pub fn invoke(&self, dest: Option<&String>) {
        match &self {
            ConnectAction::SshConnect => &self.connect_ssh(dest),
            ConnectAction::IloSshConnect => &self.connect_ilo_ssh(dest),
            ConnectAction::IloWebConnect => &self.connect_ilo_web(dest),
        };
    }

    fn connect_ssh(&self, dest: Option<&String>) -> Result<()> {
        ConnectAction::do_ssh_repl(&dest.unwrap())
            .unwrap_or_else(|e| panic!("ssh session failed with {}", e));
        Ok(())
    }

    fn connect_ilo_ssh(&self, dest: Option<&String>) -> Result<()> {
        todo!()
    }

    fn connect_ilo_web(&self, dest: Option<&String>) -> Result<()> {
        todo!()
    }

    fn do_ssh_repl(host: &String) -> Result<()> {
        let mut ssh = ConnectAction::create_ssh_session(&host)?;
        ssh.exp_regex("\r\n.*root.*:~# ")?;
        ConnectAction::interact(&mut ssh)?;
        ssh.exp_eof()?;
        Ok(())
    }

    fn create_ssh_session(host: &String) -> Result<PtyReplSession> {
        io::stdout().flush().expect("couldn't flush buffer");
        println!("Connecting to host {}", host);
        let custom_prompt = format!("root@{}'s password: ", host);
        let custom_command = format!("ssh root@{}", host);
        let mut ssh = PtyReplSession {
            echo_on: false,
            prompt: custom_prompt,
            pty_session: spawn(&*custom_command.into_boxed_str(), Some(5000))?,
            quit_command: Some("Q".to_string()),
        };
        ssh.wait_for_prompt()?;
        ssh.send_line("root")?;
        Ok(ssh)
    }

    fn interact(session: &mut PtyReplSession) -> Result<()> {
        while let Some(WaitStatus::StillAlive) = session.process.status() {
            let mut user_input = String::new();
            print!("ssh session :> ");
            io::stdout().flush().expect("couldn't flush buffer");
            io::stdin().read_line(&mut user_input).expect("error: unable to read user input");
            session.writer.write(&user_input.into_bytes()).expect("could not write");

            let re = Regex::new(r".*root@.*:~# ").unwrap();
            let (before, _) = session.reader.read_until(&rexpect::ReadUntil::Regex(re)).unwrap();

            println!("{}", before);
        };
        Ok(())
    }
}
