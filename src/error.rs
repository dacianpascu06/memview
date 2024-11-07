use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileErr {
    #[error("Process doesn't exist!")]
    ProcessErr,
}

#[derive(Error, Debug, Copy, Clone)]
pub enum InfoErr {
    #[error("Cannot parse line of proc/pid/maps!")]
    LineErr,
    #[error("Cannot format address!")]
    AddrFmtErr,
    #[error("Failed to get page size!")]
    PageErr,
    #[error("Output is empty!")]
    OutputErr,
    #[error("Process has stopped!")]
    StoppedErr,
    #[error("Draw error!")]
    DrawErr,
    #[error("An event error has occured!")]
    EventErr,
    #[error("")]
    None,
}
