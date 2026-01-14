use std::{
    env::{
        self,
    },
    process::Command,
};
use sway_workspace::workspace_manager::{
    Direction,
    WorkspaceManager,
};
use swayipc::{
    Connection,
    Error,
    Fallible,
};



fn main() -> Fallible<()>
{
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1
    {
        return Err(Error::CommandParse(format!("Expected command.")));
    }

    let mut connection = Connection::new()?;
    let workspaces = WorkspaceManager::new(&mut connection)?;

    match args[1].to_lowercase().as_str()
    {
        "create" => match args[2].to_lowercase().as_str()
        {
            "right" => workspaces.insert_workspace(&mut connection, Direction::Right),
            "left" => workspaces.insert_workspace(&mut connection, Direction::Left),
            unknown => Err(Error::CommandParse(format!(
                "Unknown argument. Command: \"create\", arg: \"{unknown}\""
            ))),
        },
        "move" => match args[2].to_lowercase().as_str()
        {
            "right" => workspaces.move_workspace(&mut connection, Direction::Right),
            "left" => workspaces.move_workspace(&mut connection, Direction::Left),
            unknown => Err(Error::CommandParse(format!(
                "Unknown argument. Command: \"move\", arg: \"{unknown}\""
            ))),
        },
        "move_focused" => match args[2].to_lowercase().as_str()
        {
            "right" => workspaces.move_window(&mut connection, Direction::Right),
            "left" => workspaces.move_window(&mut connection, Direction::Left),
            unknown => Err(Error::CommandParse(format!(
                "Unknown argument. Command: \"move_focused\", arg: \"{unknown}\""
            ))),
        },
        "info" => workspaces.print_info(),
        unknown => Err(Error::CommandParse(format!(
            "Unknown command \"{unknown}\""
        ))),
    }
    .inspect_err(|e| {
        Command::new("/bin/notify-send")
            .args([
                "-u",
                "critical",
                "Workspace Error",
                format!("Error in workspace: {e}").as_str(),
            ])
            .output()
            .expect("Should be able to run program.");
    })
}
