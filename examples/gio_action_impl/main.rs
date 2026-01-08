mod action;

use gio::prelude::*;

fn main() {
    let action = gio::SimpleAction::new("bark", Some(glib::VariantTy::STRING));
    action.connect_activate(|_, p| {
        let target = p.unwrap().str().unwrap();
        println!("Woof, {}!", target);
    });

    let renamed_action = action::RenamedAction::new("meow", &action);

    let group = gio::SimpleActionGroup::new();
    group.add_action(&action);
    group.add_action(&renamed_action);

    println!("actions = {:?}", group.list_actions());

    group.activate_action("bark", Some(&"postman".to_variant()));
    group.activate_action("meow", Some(&"milkman".to_variant()));
}
