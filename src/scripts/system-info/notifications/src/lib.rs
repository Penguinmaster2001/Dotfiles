use std::{
    ffi::OsStr,
    process::Command,
};



const NOTIFICATION_BIN: &str = "/bin/notify-send";



pub enum Level
{
    Low,
    Normal,
    Critical,
}



impl ToString for Level
{
    fn to_string(&self) -> String
    {
        match self
        {
            Level::Low => "low".to_string(),
            Level::Normal => "normal".to_string(),
            Level::Critical => "critical".to_string(),
        }
    }
}



pub struct NotificationSettings
{
    pub level: Option<Level>,
    pub duration_ms: Option<u32>,
    pub title: Option<String>,
    pub message: Option<String>,
}



impl NotificationSettings
{
    pub fn push(&self)
    {
        send_notification(self.build_args())
    }



    fn build_args(&self) -> Vec<String>
    {
        let mut args = Vec::new();

        if let Some(level) = &self.level
        {
            args.push(level.to_string());
        }
        else
        {
            args.push(Level::Low.to_string());
        }

        if let Some(duration) = self.duration_ms
        {
            args.push("-t".to_string());
            args.push(duration.to_string());
        }

        if let Some(title) = &self.title
        {
            args.push(title.clone());
        }

        if let Some(message) = &self.message
        {
            args.push(message.clone());
        }

        args
    }
}



fn send_notification<I, S>(args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new(NOTIFICATION_BIN)
        .args(args)
        .output()
        .expect("Should be able to run program.");
}
