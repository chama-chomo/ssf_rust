use cursive::traits::{Identifiable, Resizable};
use cursive::views::{
    Button, Dialog, DummyView, EditView, LinearLayout, SelectView, TextArea, TextView,
};
use cursive::Cursive;
use itertools::Itertools;
use ssf_rust::action::ConnectAction;
use ssf_rust::foreman_api::Foreman;
use ssf_rust::machine::{HostDetails, HostsArray, Machine};
use std::process::Command;

fn main() {
    let foreman = Foreman::new("ssf.conf").expect("Failed to initialize Foreman");
    let res_foreman_hosts: Box<str> = foreman.get_machines_list().expect("problem");

    let mut siv = cursive::default();
    // quit on pressing q
    siv.add_global_callback('q', |s| s.quit());

    // Title Layer
    let text_area =
        TextView::new("SSF - Foreman client that connects you to the selected machine...")
            .with_name("hello")
            .fixed_size((120, 30));
    siv.add_layer(text_area);

    // Create Group selection layer as a starting point
    group_creation(&mut siv, res_foreman_hosts);

    siv.run();

}

fn group_creation(siv: &mut Cursive, res_foreman_hosts: Box<str>) {
    // Layer for group selection
    let hosts_data = HostsArray::create_array(&res_foreman_hosts)
        .expect("Could not initialize Hosts list.")
        .results;

    // groups sorted and deduplicated
    let groups = hosts_data.iter().map(Machine::display_group).unique();

    let mut select_view = SelectView::new();
    select_view.add_all_str(groups);

    // Common buttons
    let main_buttons = LinearLayout::vertical()
        .child(Button::new("Help", help))
        .child(Button::new("Quit", Cursive::quit));

    let hosts_data = hosts_data.clone();

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(
                    select_view.on_submit(move |mut s, item| {
                        hosts_list_creation(&mut s, &item, &hosts_data)
                    }),
                )
                .child(DummyView.fixed_width(20))
                .child(main_buttons),
        )
        .title("Select a group.."),
    );
}

fn hosts_list_creation(siv: &mut Cursive, selection: &str, hosts_data: &Vec<Machine>) {
    // create an iterator from hosts and return names of the machines belonging to a specific group

    let selected_hosts_groupfilter = hosts_data
        .iter()
        .filter(|&host| host.hostgroup_name.as_deref() == Some(selection))
        .map(Machine::display_name);

    let mut select_view: SelectView = SelectView::new();
    select_view.add_all_str(selected_hosts_groupfilter);

    let text_view = TextView::new("").with_name("host_details");

    // move hosts list to on_select callback
    select_view.set_on_select({
        let _hosts_data = hosts_data.clone();
        move |mut s, item| see_host_selection(&mut s, &item, &_hosts_data)
    });

    // move hosts list to on_submit callback
    select_view.set_on_submit({
        {
            let _hosts_data = hosts_data.clone();
            move |mut s, item| action_host_selection(&mut s, &item, &_hosts_data)
        }
    });

    // hosts layer
    siv.add_layer(Dialog::around(
        LinearLayout::vertical()
            .child(text_view.fixed_size((80, 15)))
            .child(DummyView)
            .child(DummyView)
            .child(select_view)
            .child(
                LinearLayout::horizontal()
                    .child(Button::new("Back", |s| {
                        s.pop_layer();
                    }))
                    .child(DummyView)
                    .child(Button::new("Quit", Cursive::quit)),
            ),
    ));
}

fn action_choose(siv: &mut Cursive, selection: &String, host: &Machine) {
    // Assign action for the selection

    siv.quit();
    // reset the terminal otherwise console part of the program will not be working nice
    print!("\x1B[2J");
    Command::new("stty").arg("sane").spawn().unwrap();

    if selection == &"ssh".as_ref() {
        let action = ConnectAction::SshConnect;
        action.invoke(host.ip.as_ref());
    } else if selection == &"ssh_ilo".as_ref() {
        let action = ConnectAction::IloSshConnect;
        action.invoke(host.sp_ip.as_ref())
    } else if selection == &"web_ilo".as_ref() {
        let action = ConnectAction::IloWebConnect;
        action.invoke(host.sp_ip.as_ref())
    } else {
        panic!("Something bad happened while selection item action! Sorry, cannot continue.")
    }
}

fn action_host_selection(siv: &mut Cursive, hostname: &String, hosts_data: &Vec<Machine>) {
    // Action layer
    let mut action_view: SelectView = SelectView::new();

    // Create a small menu
    action_view.add_item(
        format!("SSH:     Connect to {} using ssh client", &hostname),
        "ssh".to_string(),
    );
    action_view.add_item(
        format!("ILO SSH: Connect to {}'s ILO using ssh client", &hostname),
        "ssh_ilo".to_string(),
    );
    action_view.add_item(
        format!(
            "WEB SSH: Connect to {}'s ILO using a default web browser",
            &hostname
        ),
        "web_ilo".to_string(),
    );

    // need to choose hostname details from hosts_data and pass it further
    action_view.set_on_submit({
        let host: &Machine = hosts_data
            .iter()
            .find(|&s| s.name.as_deref() == Some(hostname))
            .unwrap();
        let host = host.clone();
        move |s, item| action_choose(s, &item, &host)
    });

    siv.add_layer(Dialog::around(
        LinearLayout::vertical()
            .child(DummyView)
            .child(action_view.fixed_size((100, 5)))
            .child(
                LinearLayout::horizontal()
                    .child(DummyView)
                    .child(Button::new("Back", |s| {
                        s.pop_layer();
                    }))
                    .child(DummyView)
                    .child(Button::new("Quit", Cursive::quit)),
            ),
    ));
}

fn see_host_selection(siv: &mut Cursive, selection: &String, hosts_data: &Vec<Machine>) {
    // creates live view for the selected host item
    let host_detail: String = hosts_data
        .iter()
        .filter(|&host| host.name.as_deref() == Some(selection))
        .map(Machine::display_host_details)
        .collect();

    siv.call_on_name("host_details", |v: &mut TextView| {
        v.set_content(&host_detail);
    })
    .unwrap();
}

// Help layer
fn help(s: &mut Cursive) {
    s.add_layer(
        Dialog::text(format!(
            "Shortcuts:
                pressing q - quits ssf
                pressing h - shows help

Configuration:
    Please create a configuration file before using SSF, it is expected to be located in \"~/.config/ssf.conf\".
    Form of the config file is like below:

    user = \"my.name\"
    password = \"7uhaIVD4MqMW0Rm\"
    url = \"https://10.10.30.201/api/\"
"
        ))
        .title(format!("SSF - How to use instructions"))
        .button("Back", |s| {
            s.pop_layer();
        }),
    )
}
