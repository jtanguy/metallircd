//! Basic structure representing a message

#![experimental]

use std::cmp::min;
use std::from_str::FromStr;

#[deriving(Show, PartialEq, Clone)]
pub struct IRCMessage {
    pub prefix: Option<String>,
    pub command: String,
    pub args: Vec<String>,
    pub suffix: Option<String>
}

impl IRCMessage {

    /// Generates the String that will be sent over the network.
    #[experimental]
    pub fn to_protocol(&self) -> String {
        let mut output = String::new();
        if let Some(ref text) = self.prefix {
            output.push(':');
            output.push_str(text.as_slice());
            output.push(' ');
        }
        output.push_str(self.command.as_slice());
        for ref arg in self.args.iter() {
            output.push(' ');
            output.push_str(arg.as_slice());
        }
        if let Some(ref text) = self.suffix {
            output.push(' ');
            output.push(':');
            output.push_str(text.as_slice());
        }
        output
    }

    /// Computes the len of the message in protocol form
    #[experimental]
    pub fn protocol_len(&self) -> uint {
        self.command.len()
        + if let Some(ref s) = self.prefix { s.len() } else { 0 }
        + self.args.iter().fold(0, |n, s| n + 1 + s.len())
        + if let Some(ref s) = self.suffix { s.len() } else { 0 }
    }

    /// Attemps to parse parameters of the Message a `needed` necessary
    /// parameters and `optionnal` optionnal parameters. Returning `None`
    /// if there were not enough parmeters to fullfill `needed`.
    #[experimental]
    pub fn as_nparams(&self, needed: uint, optionnal: uint)
            -> Option<Vec<String> > {

        // Skips the first `skip` arguments and merge the rest as one big string
        fn skip_and_fuse(m: &IRCMessage, skip: uint) -> String {
            let mut result = String::new();
            let mut it = m.args.iter().skip(skip);
            match it.next() {
                Some(ref txt) => { result.push_str(txt.as_slice()) },
                _ => {}
            }
            for txt in it {
                result.push_str(" ");
                result.push_str(txt.as_slice());
            }
            match m.suffix {
                Some(ref txt) => {
                    if result.len() > 0 { result.push_str(" "); }
                    result.push_str(txt.as_slice());
                }
                _ => {}
            }
            result
        }

        let available = self.args.len() + self.suffix.is_some() as uint;
        let taking = min(available, needed + optionnal);

        if available < needed {
            return None
        }

        let mut params = Vec::new();

        if taking == 0 { return Some(params); }

        for i in range(0u, taking - 1) {
            params.push(self.args[i].clone());
        }
        params.push(skip_and_fuse(self, taking - 1));
        Some(params)
    }

}

impl FromStr for IRCMessage {

    fn from_str(s: &str) -> Option<IRCMessage> {
        // These chars are forbidden by the protocol
        // Or there is not only one line
        if s.len() == 0 || s.contains_char('\0') ||
           s.contains_char(0x0D as char) || s.contains_char(0x0A as char) {
               return None;
        }
        let mut rest = s;
        // is there a prefix to parse ?
        let mut prefix = None;
        if rest.char_at(0) == ':' {
            let mut split = rest.splitn(1, ' ');
            match split.next() { // parsing prefix
                Some(txt) if txt.len() > 1 => {
                    prefix = Some(String::from_str(txt.slice_from(1)));
                },
                _ => { return None; } // invalid prefix
            }
            match split.next() { // the rest
                Some(txt) => { rest = txt; },
                _ => { return None; } // no command ??
            }
        }
        // is there a suffix string ?
        let mut suffix = None;
        {
            if rest.char_at(0) == ':' { return None; } // no command ??
            match rest.match_indices(" :").next() {
                None => {}, // no suffix
                Some((x, y)) if x > 0 => {
                    suffix = Some(String::from_str(rest.slice_from(y)));
                    rest = rest.slice_to(x);
                },
                _ => { return None; } // only a suffix, without command ??
            }
        }
        // parse command and args :
        let mut args = Vec::new();
        for slice in rest.splitn(14, ' ') {
            if slice.len() > 0 { args.push(String::from_str(slice)); }
        }
        let command = match args.remove(0) {
            Some(txt) => txt,
            None => { return None; } // no command ??
        };
        // The parsing grammar sets 14 args plus 1 trailing
        if  args.len() == 15 {
            suffix = match suffix {
                None => Some(args.pop().unwrap()),
                Some(txt) => {
                    let mut result = args.pop().unwrap();
                    result.push_str(" :");
                    result.push_str(txt.as_slice());
                    Some(result)
                }
            }
        }
        Some(IRCMessage{
            prefix: prefix,
            command: command,
            args: args,
            suffix: suffix
        })
    }
}

#[cfg(test)]
mod tests {
    use super::IRCMessage;

    #[test]
    fn ircmessage_to_protocol() {
        let message = IRCMessage {
            prefix: Some("kitty".to_string()),
            command: "FOO".to_string(),
            args: vec!("bar".to_string(), "baz".to_string()),
            suffix: Some("I am a cake.".to_string())
        };
        assert_eq!(message.to_protocol().as_slice(), ":kitty FOO bar baz :I am a cake.");
    }

    #[test]
    fn ircmessage_from_string() {
        let message = from_str::<IRCMessage>(":bl:ih BLAH blo_uh bl:uh bleh :I love cakes !! ::").unwrap();
        let expected = IRCMessage {
            prefix: Some("bl:ih".to_string()),
            command: "BLAH".to_string(),
            args: vec!("blo_uh".to_string(), "bl:uh".to_string(), "bleh".to_string()),
            suffix: Some("I love cakes !! ::".to_string())
        };
        assert_eq!(message, expected);
    }

}