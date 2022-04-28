use serenity::builder::CreateApplicationCommands;

use super::{
    cmd_add::setup::cmd_add_setup, cmd_del::setup::cmd_del_setup, cmd_list::setup::cmd_list_setup,
};

pub fn custom_cmd_setup(cmds: &mut CreateApplicationCommands) {
    cmd_add_setup(cmds);
    cmd_del_setup(cmds);
    cmd_list_setup(cmds);
}
