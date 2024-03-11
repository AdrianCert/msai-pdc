#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Protocol {
    UDP,
    TCP,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::UDP => "udp",
            Self::TCP => "tcp",
        };
        s.fmt(f)
    }
}

impl std::str::FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "udp" => Ok(Self::UDP),
            "tcp" => Ok(Self::TCP),
            _ => Err(format!("Unknown protocol: {s}")),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mechanism {
    Stream,
    Sync,
}

impl std::fmt::Display for Mechanism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Stream => "stream",
            Self::Sync => "sync",
        };
        s.fmt(f)
    }
}
impl std::str::FromStr for Mechanism {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stream" => Ok(Self::Stream),
            "sync" => Ok(Self::Sync),
            _ => Err(format!("Unknown mechanism: {s}")),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Role {
    Client,
    Server,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Client => "client",
            Self::Server => "server",
        };
        s.fmt(f)
    }
}
impl std::str::FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "client" => Ok(Self::Client),
            "server" => Ok(Self::Server),
            _ => Err(format!("Unknown role: {s}")),
        }
    }
}
