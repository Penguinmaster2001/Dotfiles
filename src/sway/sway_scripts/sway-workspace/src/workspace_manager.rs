use std::ops::Index;
use swayipc::{
    Connection,
    Error,
    Fallible,
    Workspace,
};



#[derive(PartialEq)]
pub enum Direction
{
    Left,
    Right,
}



pub struct WorkspaceManager
{
    workspaces: Vec<Workspace>,
}



impl WorkspaceManager
{
    pub fn new(connection: &mut Connection) -> Fallible<Self>
    {
        let mut workspaces = connection.get_workspaces()?;
        workspaces.sort_by_key(|w| w.num);

        Ok(Self { workspaces })
    }



    pub fn insert_workspace(self, connection: &mut Connection, direction: Direction)
    -> Fallible<()>
    {
        let mut past_focused = true;
        let mut target_num = -1;
        for workspace in self.workspaces.iter().rev()
        {
            if workspace.focused
            {
                past_focused = false;

                if direction == Direction::Left
                {
                    target_num = workspace.num;
                    if let Err(err) = connection.run_command(format!(
                        "rename workspace {} to {}",
                        workspace.num,
                        workspace.num + 1
                    ))
                    {
                        return Err(err);
                    }
                }
                else
                {
                    target_num = workspace.num + 1;
                }
            }
            else if past_focused
                && let Err(err) = connection.run_command(format!(
                    "rename workspace {} to {}",
                    workspace.num,
                    workspace.num + 1
                ))
            {
                return Err(err);
            }
        }

        connection.run_command(format!("workspace {target_num}"))?;

        Ok(())
    }



    pub fn move_workspace(&self, connection: &mut Connection, direction: Direction)
    -> Fallible<()>
    {
        let target = self.next_workspace(direction)?;
        connection.run_command(format!("workspace {target}"))?;

        Ok(())
    }



    pub fn move_window(&self, connection: &mut Connection, direction: Direction) -> Fallible<()>
    {
        let target = self.next_workspace(direction)?;
        connection.run_command(format!("move container to workspace number {target}"))?;
        connection.run_command(format!("workspace {target}"))?;

        Ok(())
    }



    pub fn print_info(&self) -> Fallible<()>
    {
        for workspace in &self.workspaces
        {
            println!("{:?}", workspace);
        }

        Ok(())
    }



    fn next_workspace(&self, direction: Direction) -> Fallible<i32>
    {
        let current =
            self.workspaces
                .iter()
                .position(|w| w.focused)
                .ok_or(Error::CommandFailed(format!(
                    "Could not find current workspace."
                )))?;

        let next = (self.workspaces.len()
            + match direction
            {
                Direction::Left => current - 1,
                Direction::Right => current + 1,
            })
            % self.workspaces.len();

        Ok(self.workspaces.index(next).num)
    }
}
