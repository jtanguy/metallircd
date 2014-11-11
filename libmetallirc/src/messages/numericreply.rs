//! Numeric replys of the protocol according to RCF 2812.

#![experimental]

use super::IRCMessage;

/// Lists all possible numerical answers from the server.
#[allow(non_camel_case_types)]
#[experimental]
pub enum NumericReply<'a> {
    //
    // Connection welcome
    //
    RPL_WELCOME(&'a str, &'a str),
    RPL_YOURHOST(&'a str, &'a str),
    RPL_CREATED(&'a str),
    RPL_MYINFO(&'a str, &'a str, &'a str, &'a str,),
    RPL_BOUNCE(&'a str, u16),
    //
    // Responses
    //
    RPL_USERHOST(Vec<(&'a str, bool, bool, &'a str)>),
    RPL_ISON(Vec<&'a str>),
    // Away related
    RPL_AWAY(&'a str, &'a str),
    RPL_UNAWAY,
    RPL_NOWAWAY,
    // Whois related
    RPL_WHOISUSER(&'a str, &'a str, &'a str, &'a str),
    RPL_WHOISSERVER(&'a str, &'a str, &'a str),
    RPL_WHOISOPERATOR(&'a str),
    RPL_WHOISIDLE(&'a str, u32),
    RPL_ENDOFWHOIS(&'a str),
    RPL_WHOISCHANNELS(&'a str, Vec<(Option<char>, &'a str)>),
    // Whowas related
    RPL_WHOWASUSER(&'a str, &'a str, &'a str, &'a str),
    RPL_ENDOFWHOWAS,
    // List related
    RPL_LIST(&'a str, u32, &'a str),
    RPL_LISTEND,
    RPL_SERVLIST(&'a str, &'a str, &'a str, &'a str, u32, &'a str),
    RPL_SERVLISTEND(&'a str, &'a str),
    RPL_LUSERCLIENT(u32, u32, u32),
    RPL_LUSEROP(u32),
    RPL_LUSERUNKNOWN(u32),
    RPL_LUSERCHANNELS(u32),
    RPL_LUSERME(u32, u32),
    // Chan related
    RPL_UNIQOPIS(&'a str, &'a str),
    RPL_CHANNELMODEIS(&'a str, &'a str),
    RPL_NOTOPIC(&'a str),
    RPL_TOPIC(&'a str, &'a str),
    RPL_INVITING(&'a str, &'a str),
    RPL_SUMMONING(&'a str),
    RPL_INVITELIST(&'a str, &'a str),
    RPL_ENDOFINVITELIST(&'a str),
    RPL_EXCEPTLIST(&'a str, &'a str),
    RPL_ENDOFEXCEPTLIST(&'a str),
    RPL_BANLIST(&'a str, &'a str),
    RPL_ENDOFBANLIST(&'a str),
    RPL_CREATIONTIME(&'a str, i64),
    // Server related
    RPL_VERSION(&'a str, &'a str, &'a str, &'a str),
    RPL_WHOREPLY(&'a str, &'a str, &'a str, &'a str, &'a str, char, bool, Option<char>, u32, &'a str),
    RPL_ENDOFWHO(&'a str),
    RPL_NAMEREPLY(char, &'a str, Vec<(Option<char>, String)>),
    RPL_ENDOFNAMES(&'a str),
    RPL_LINKS(&'a str, &'a str, u32, &'a str),
    RPL_ENDOFLINKS(&'a str),
    RPL_INFO(&'a str),
    RPL_ENDOFINFO,
    RPL_TIME(&'a str, &'a str),
    // MOTD
    RPL_MOTDSTART(&'a str),
    RPL_MOTD(&'a str),
    RPL_ENDOFMOTD,
    // Administration
    RPL_YOUREOPER,
    RPL_REHASHING(&'a str),
    RPL_YOURESERVICE(&'a str),
    RPL_ADMINME(&'a str),
    RPL_ADMINLOC1(&'a str),
    RPL_ADMINLOC2(&'a str),
    RPL_ADMINEMAIL(&'a str),
    // Users related
    RPL_USERSSTART,
    RPL_USERS(&'a str, &'a str, &'a str),
    RPL_ENDOFUSERS,
    RPL_NOUSERS,
    // Trace related
    RPL_TRACELINK(&'a str, &'a str, &'a str, &'a str, &'a str, &'a str, &'a str),
    RPL_TRACECONNECTING(&'a str, &'a str),
    RPL_TRACEHANDSHAKE(&'a str, &'a str),
    RPL_TRACEUNKNOWN(&'a str, &'a str),
    RPL_TRACEOPERATOR(&'a str, &'a str),
    RPL_TRACEUSER(&'a str, &'a str),
    RPL_TRACESERVER(&'a str, &'a str, &'a str, &'a str, &'a str, &'a str, &'a str),
    RPL_TRACESERVICE(&'a str, &'a str, &'a str, &'a str),
    RPL_TRACENEWTYPE(&'a str, &'a str),
    RPL_TRACECLASS(&'a str, &'a str),
    RPL_TRACELOG(&'a str, &'a str),
    RPL_TRACEEND(&'a str, &'a str, &'a str),
    // Stats related
    RPL_STATSLINKINFO(&'a str, u32, u32, u32, u32, u32, u32),
    RPL_STATSCOMMANDS(&'a str, u32, u32, u32),
    RPL_ENDOFSTATS(&'a str),
    RPL_STATSUPTIME(u32, u8, u8, u8),
    RPL_STATSOLINE(&'a str, &'a str),
    // Client mode
    RPL_UMODEIS(&'a str),
    // Misc
    RPL_TRYAGAIN(&'a str),
    //
    // Errors
    //
    ERR_NOSUCHNICK(&'a str),
    ERR_NOSUCHSERVER(&'a str),
    ERR_NOSUCHCHANNEL(&'a str),
    ERR_CANNOTSENDTOCHAN(&'a str),
    ERR_TOOMANYCHANNELS(&'a str),
    ERR_WASNOSUCHNICK(&'a str),
    ERR_TOOMANYTARGETS(&'a str, &'a str, &'a str),
    ERR_NOSUCHSERVICE(&'a str),
    ERR_NOORIGIN,
    ERR_UNKNOWNCOMMAND(&'a str),
    ERR_NOMOTD,
    ERR_NOADMININFO(&'a str),
    ERR_FILEERROR(&'a str, &'a str),
    ERR_UNAVAILRESOURCE(&'a str),
    // PRIVMSG_ERR
    ERR_NORECIPIENT(&'a str),
    ERR_NOTEXTTOSEND,
    ERR_NOTOPLEVEL(&'a str),
    ERR_WILDTOLEVEL(&'a str),
    ERR_BADMASK(&'a str),
    // Nick Related
    ERR_NONICKNAMEGIVEN,
    ERR_ERRONEUSNICKNAME(&'a str),
    ERR_NICKNAMEINUSE(&'a str),
    ERR_NICKCOLLISION(&'a str, &'a str, &'a str),
    // Chan related
    ERR_USERNOTINCHANNEL(&'a str, &'a str),
    ERR_NOTONCHANNEL(&'a str),
    ERR_USERONCHANNEL(&'a str, &'a str),
    ERR_NOLOGIN(&'a str),
    ERR_KEYSET(&'a str),
    ERR_CHANNELISFULL(&'a str),
    ERR_UNKNOWNMODE(char, &'a str),
    ERR_INVITEONLYCHAN(&'a str),
    ERR_BANNEDFROMCHAN(&'a str),
    ERR_BADCHANNELKEY(&'a str),
    ERR_BADCHANMASK(&'a str),
    ERR_NOCHANMODES(&'a str),
    ERR_BANLISTFULL(&'a str, char),
    // NotAlloed related
    ERR_SUMMONDISABLED,
    ERR_USERSDISABLED,
    ERR_NOTREGISTERED,
    ERR_NEEDMOREPARAMS(&'a str),
    ERR_ALREADYREGISTERED,
    ERR_NOPERMFORHOST,
    ERR_PASSWDMISMATCH,
    ERR_YOURBANNEDCREEP,
    ERR_YOUWILLBEBANNED,
    ERR_NOPRIVILIGES,
    ERR_CHANOPRIVSNEEDED(&'a str),
    ERR_CANTKILLSERVER,
    ERR_RESTRICTED,
    ERR_UNIQOPPRIVSNEEDED,
    ERR_NOOPERHOST,
    ERR_UMODEUNKNOWNFLAG,
    ERR_USERSDONTMATCH
}

#[experimental]
impl<'a> NumericReply<'a> {

    pub fn into_prefixed_message(self, usrnick: &str, prefix: &str) -> IRCMessage {
        let mut msg = self.into_ircmessage(usrnick);
        msg.prefix = Some(prefix.into_string());
        msg
    }

    #[experimental]
    pub fn into_ircmessage(self, usrnick: &str) -> IRCMessage {
        match self {
            RPL_WELCOME(msg, fullname) => IRCMessage {
                prefix: None,
                command: "001".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(format!("Welcome to {} {}", msg, fullname))
            },
            RPL_YOURHOST(srvname, version) => IRCMessage {
                prefix: None,
                command: "002".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(format!("Your host is {}, running version {}", srvname, version))
            },
            RPL_CREATED(date) => IRCMessage {
                prefix: None,
                command: "003".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(format!("This server was created {}", date))
            },
            RPL_MYINFO(srvname, version, user_modes, chan_modes) => IRCMessage {
                prefix: None,
                command: "004".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    srvname.into_string(),
                    version.into_string(),
                    user_modes.into_string(),
                    chan_modes.into_string()
                ),
                suffix: None
            },
            RPL_BOUNCE(server, port) => IRCMessage {
                prefix: None,
                command: "005".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(format!("Try server {} port {}", server, port))
            },
            //
            // Responses
            //
            RPL_USERHOST(v) => IRCMessage {
                prefix: None,
                command: "302".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(
                    v.into_iter().fold(String::new(), |mut s, (nick, op, aw, host)|{
                        if s.len() > 0 { s.push(' '); }
                        s.push_str(nick);
                        if op { s.push('*'); }
                        if aw { s.push('-'); } else { s.push('+'); }
                        s.push_str(host);
                        s
                    })
                )
            },
            RPL_ISON(v) => IRCMessage {
                prefix: None,
                command: "302".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(
                    v.into_iter().fold(String::new(), |mut s, nick|{
                        if s.len() > 0 { s.push(' '); }
                        s.push_str(nick);
                        s
                    })
                )
            },
            // Away related
            RPL_AWAY(nick, msg) => IRCMessage {
                prefix: None,
                command: "301".into_string(),
                args: vec!(usrnick.into_string(), nick.into_string()),
                suffix: Some(msg.into_string())
            },
            RPL_UNAWAY => IRCMessage {
                prefix: None,
                command: "305".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("You are no longer marked as away".into_string())
            },
            RPL_NOWAWAY => IRCMessage {
                prefix: None,
                command: "306".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("You have been marked as being away".into_string())
            },
            // Whois related
            RPL_WHOISUSER(nick, user, host, real_name) => IRCMessage {
                prefix: None,
                command: "311".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    nick.into_string(),
                    user.into_string(),
                    host.into_string(),
                    "*".into_string()
                ),
                suffix: Some(real_name.into_string())
            },
            RPL_WHOISSERVER(nick, server, server_info) => IRCMessage {
                prefix: None,
                command: "312".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    nick.into_string(),
                    server.into_string()
                ),
                suffix: Some(server_info.into_string())
            },
            RPL_WHOISOPERATOR(nick) => IRCMessage {
                prefix: None,
                command: "313".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    nick.into_string()
                ),
                suffix: Some("is an IRC operator".into_string())
            },
            RPL_WHOISIDLE(nick, time) => IRCMessage {
                prefix: None,
                command: "317".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    nick.into_string(),
                    time.to_string()
                ),
                suffix: Some("seconds idle".into_string())
            },
            RPL_ENDOFWHOIS(masks) => IRCMessage {
                prefix: None,
                command: "318".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    masks.into_string()
                ),
                suffix: Some("End of WHOIS list".into_string())
            },
            // (&'a str, Vec<(char, &'a str)>)
            RPL_WHOISCHANNELS(nick, v) => IRCMessage {
                prefix: None,
                command: "319".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    nick.into_string()
                ),
                suffix: Some(
                    v.into_iter().fold(String::new(), |mut s, (m, chan)|{
                        if s.len() > 0 { s.push(' '); }
                        if let Some(c) = m { s.push(c) };
                        s.push_str(chan);
                        s
                    })
                )
            },
            // Whowas related
            RPL_WHOWASUSER(nick, user, host, real_name) => IRCMessage {
                prefix: None,
                command: "314".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    nick.into_string(),
                    user.into_string(),
                    host.into_string(),
                    "*".into_string()
                ),
                suffix: Some(real_name.into_string())
            },
            RPL_ENDOFWHOWAS => IRCMessage {
                prefix: None,
                command: "369".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("End of WHOWAS list".into_string())
            },
            // List related
            RPL_LIST(chan, users, topic) => IRCMessage {
                prefix: None,
                command: "322".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string(),
                    users.to_string()
                ),
                suffix: Some(topic.into_string())
            },
            RPL_LISTEND => IRCMessage {
                prefix: None,
                command: "323".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("End of LIST".into_string())
            },
            RPL_SERVLIST(name, server, mask, t_ype, hopcount, info) => IRCMessage {
                prefix: None,
                command: "234".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    name.into_string(),
                    server.into_string(),
                    mask.into_string(),
                    t_ype.into_string(),
                    hopcount.to_string(),
                    info.into_string()
                ),
                suffix: None
            },
            RPL_SERVLISTEND(mask, t_ype) => IRCMessage {
                prefix: None,
                command: "235".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    mask.into_string(),
                    t_ype.into_string()
                ),
                suffix: Some("End of service listing".into_string())
            },
            RPL_LUSERCLIENT(users, services, servers) => IRCMessage {
                prefix: None,
                command: "251".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(
                    format!("There are {} users and {} services on {} servers",
                        users, services, servers)
                )
            },
            RPL_LUSEROP(count) => IRCMessage {
                prefix: None,
                command: "252".into_string(),
                args: vec!(usrnick.into_string(), count.to_string()),
                suffix: Some("operator(s) online".into_string())
            },
            RPL_LUSERUNKNOWN(count) => IRCMessage {
                prefix: None,
                command: "253".into_string(),
                args: vec!(usrnick.into_string(), count.to_string()),
                suffix: Some("unknown connection(s)".into_string())
            },
            RPL_LUSERCHANNELS(count) => IRCMessage {
                prefix: None,
                command: "254".into_string(),
                args: vec!(usrnick.into_string(), count.to_string()),
                suffix: Some("channels formed".into_string())
            },
            RPL_LUSERME(users, servers) => IRCMessage {
                prefix: None,
                command: "255".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(
                    format!("I have {} clients and {} servers",
                        users, servers)
                )
            },
            // Chan related
            RPL_UNIQOPIS(chan, nick) => IRCMessage {
                prefix: None,
                command: "325".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string(),
                    nick.into_string()
                ),
                suffix: None
            },
            RPL_CHANNELMODEIS(chan, modes) => IRCMessage {
                prefix: None,
                command: "324".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string(),
                    modes.into_string()
                ),
                suffix: None
            },
            RPL_NOTOPIC(chan) => IRCMessage {
                prefix: None,
                command: "331".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string()
                ),
                suffix: Some("No topic is set".into_string())
            },
            RPL_TOPIC(chan, topic) => IRCMessage {
                prefix: None,
                command: "331".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string()
                ),
                suffix: Some(topic.into_string())
            },
            RPL_INVITING(nick, chan) => IRCMessage {
                prefix: None,
                command: "341".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    nick.into_string(),
                    chan.into_string()
                ),
                suffix: None
            },
            RPL_SUMMONING(nick) => IRCMessage {
                prefix: None,
                command: "342".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    nick.into_string()
                ),
                suffix: Some("Summoning user to IRC.".into_string())
            },
            RPL_INVITELIST(chan, mask) => IRCMessage {
                prefix: None,
                command: "346".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string(),
                    mask.into_string()
                ),
                suffix: None
            },
            RPL_ENDOFINVITELIST(chan) => IRCMessage {
                prefix: None,
                command: "347".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string()
                ),
                suffix: Some("End of channel invite list".into_string())
            },
            RPL_EXCEPTLIST(chan, mask) => IRCMessage {
                prefix: None,
                command: "348".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string(),
                    mask.into_string()
                ),
                suffix: None
            },
            RPL_ENDOFEXCEPTLIST(chan) => IRCMessage {
                prefix: None,
                command: "349".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string()
                ),
                suffix: Some("End of channel exception list".into_string())
            },
            RPL_BANLIST(chan, mask) => IRCMessage {
                prefix: None,
                command: "367".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string(),
                    mask.into_string()
                ),
                suffix: None
            },
            RPL_ENDOFBANLIST(chan) => IRCMessage {
                prefix: None,
                command: "368".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string()
                ),
                suffix: Some("End of channel exception list".into_string())
            },
            RPL_CREATIONTIME(chan, time) => IRCMessage {
                prefix: None,
                command: "329".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string()
                ),
                suffix: Some(time.to_string())
            },
            // Server related
            RPL_VERSION(version, debuglevel, server, comments) => IRCMessage {
                prefix: None,
                command: "351".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    format!("{}.{}", version, debuglevel),
                    server.into_string()
                ),
                suffix: Some(comments.into_string())
            },
            RPL_WHOREPLY(channel, user, host, server,
                    nick, hg, star, membership, hopcount, real_name) => IRCMessage {
                prefix: None,
                command: "352".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    channel.into_string(),
                    user.into_string(),
                    host.into_string(),
                    server.into_string(),
                    nick.into_string(),
                    {
                        let mut s = hg.to_string();
                        if star { s.push('*'); }
                        if let Some(c) = membership { s.push(c); }
                        s
                    }
                ),
                suffix: Some(hopcount.to_string() + " " + real_name)
            },
            RPL_ENDOFWHO(name) => IRCMessage {
                prefix: None,
                command: "315".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    name.into_string()
                ),
                suffix: Some("End of WHO list".into_string())
            },
            RPL_NAMEREPLY(cprefix, chan, v) => IRCMessage {
                prefix: None,
                command: "353".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    cprefix.to_string(),
                    chan.into_string()
                ),
                suffix: Some(
                    v.into_iter().fold(String::new(), |mut s, (prefix, nick)|{
                        if s.len() > 0 { s.push(' '); }
                        if let Some(c) = prefix { s.push(c) };
                        s.push_str(nick.as_slice());
                        s
                    })
                )
            },
            RPL_ENDOFNAMES(chan) => IRCMessage {
                prefix: None,
                command: "366".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string()
                ),
                suffix: Some("End of NAMES list".into_string())
            },
            RPL_LINKS(mask, server, hopcount, server_info) => IRCMessage {
                prefix: None,
                command: "364".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    mask.into_string(),
                    server.into_string()
                ),
                suffix: Some(
                    format!("{} {}", hopcount, server_info)
                )
            },
            RPL_ENDOFLINKS(mask) => IRCMessage {
                prefix: None,
                command: "366".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    mask.into_string()
                ),
                suffix: Some("End of LINKS list".into_string())
            },
            RPL_INFO(info) => IRCMessage {
                prefix: None,
                command: "371".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(info.into_string())
            },
            RPL_ENDOFINFO => IRCMessage {
                prefix: None,
                command: "374".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("End of INFO list".into_string())
            },
            RPL_TIME(server, time) => IRCMessage {
                prefix: None,
                command: "391".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    server.into_string()
                ),
                suffix: Some(time.into_string())
            },
            // MOTD
            RPL_MOTDSTART(server) => IRCMessage {
                prefix: None,
                command: "375".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(format!("- {} Message of the day - ", server))
            },
            RPL_MOTD(text) => IRCMessage {
                prefix: None,
                command: "372".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(format!("- {}", text))
            },
            RPL_ENDOFMOTD => IRCMessage {
                prefix: None,
                command: "376".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("End of MOTH command".into_string())
            },
            // Administration
            RPL_YOUREOPER => IRCMessage {
                prefix: None,
                command: "381".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("You are now an IRC operator".into_string())
            },
            RPL_REHASHING(file) => IRCMessage {
                prefix: None,
                command: "382".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    file.into_string()
                ),
                suffix: Some("Rehashing".into_string())
            },
            RPL_YOURESERVICE(name) => IRCMessage {
                prefix: None,
                command: "383".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    // RFC. Don't ask.
                    "You".into_string(),
                    "are".into_string(),
                    "service".into_string(),
                    name.into_string()
                ),
                suffix: None
            },
            RPL_ADMINME(server) => IRCMessage {
                prefix: None,
                command: "256".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    server.into_string()
                ),
                suffix: Some("Administrative info".into_string())
            },
            RPL_ADMINLOC1(info) => IRCMessage {
                prefix: None,
                command: "257".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(info.into_string())
            },
            RPL_ADMINLOC2(info) => IRCMessage {
                prefix: None,
                command: "258".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(info.into_string())
            },
            RPL_ADMINEMAIL(info) => IRCMessage {
                prefix: None,
                command: "259".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(info.into_string())
            },
            // Users related
            RPL_USERSSTART => IRCMessage {
                prefix: None,
                command: "392".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("UserID   Terminal  Host".into_string())
            },
            RPL_USERS(username, tty, host) => IRCMessage {
                prefix: None,
                command: "393".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(format!("{} {} {}", username, tty, host))
            },
            RPL_ENDOFUSERS => IRCMessage {
                prefix: None,
                command: "394".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("End of users".into_string())
            },
            RPL_NOUSERS => IRCMessage {
                prefix: None,
                command: "395".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("Nobody logged in".into_string())
            },
            // Trace related
            RPL_TRACELINK(fullversion, dest, next_srv, proto_vers,
                          uptime, back_sendq, up_sendq) => IRCMessage {
                prefix: None,
                command: "200".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    "Link".into_string(),
                    fullversion.into_string(),
                    dest.into_string(),
                    next_srv.into_string(),
                    proto_vers.into_string(),
                    uptime.into_string(),
                    back_sendq.into_string(),
                    up_sendq.into_string()
                ),
                suffix: None
            },
            RPL_TRACECONNECTING(class, server) => IRCMessage {
                prefix: None,
                command: "201".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    "Try.".into_string(),
                    class.into_string(),
                    server.into_string()
                ),
                suffix: None
            },
            RPL_TRACEHANDSHAKE(class, server) => IRCMessage {
                prefix: None,
                command: "202".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    "H.S.".into_string(),
                    class.into_string(),
                    server.into_string()
                ),
                suffix: None
            },
            RPL_TRACEUNKNOWN(class, ip) => IRCMessage {
                prefix: None,
                command: "203".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    "????".into_string(),
                    class.into_string(),
                    ip.into_string()
                ),
                suffix: None
            },
            RPL_TRACEOPERATOR(class, nick) => IRCMessage {
                prefix: None,
                command: "204".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    "Oper".into_string(),
                    class.into_string(),
                    nick.into_string()
                ),
                suffix: None
            },
            RPL_TRACEUSER(class, nick) => IRCMessage {
                prefix: None,
                command: "205".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    "User".into_string(),
                    class.into_string(),
                    nick.into_string()
                ),
                suffix: None
            },
            RPL_TRACESERVER(class, s, c, server, nick_user, host_server, version) => IRCMessage {
                prefix: None,
                command: "206".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    "Serv".into_string(),
                    class.into_string(),
                    { let mut _s = s.into_string(); _s.push('S'); _s },
                    { let mut _s = c.into_string(); _s.push('C'); _s },
                    server.into_string(),
                    { let mut _s = nick_user.into_string(); _s.push('@'); _s + host_server },
                    version.into_string()
                ),
                suffix: None
            },
            RPL_TRACESERVICE(class, name, t_ype, active_type) => IRCMessage {
                prefix: None,
                command: "207".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    "Service".into_string(),
                    class.into_string(),
                    name.into_string(),
                    t_ype.into_string(),
                    active_type.into_string()
                ),
                suffix: None
            },
            RPL_TRACENEWTYPE(newtype, client_name) => IRCMessage {
                prefix: None,
                command: "208".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    newtype.into_string(),
                    "0".into_string(),
                    client_name.into_string()
                ),
                suffix: None
            },
            RPL_TRACECLASS(class, count) => IRCMessage {
                prefix: None,
                command: "209".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    "Class".into_string(),
                    class.into_string(),
                    count.into_string()
                ),
                suffix: None
            },
            RPL_TRACELOG(logfile, debuglevel) => IRCMessage {
                prefix: None,
                command: "261".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    "File".into_string(),
                    logfile.into_string(),
                    debuglevel.into_string()
                ),
                suffix: None
            },
            RPL_TRACEEND(server, version, debuglevel) => IRCMessage {
                prefix: None,
                command: "262".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    server.into_string(),
                    version.into_string() + "--" + debuglevel
                ),
                suffix: None
            },
            // Stats related
            RPL_STATSLINKINFO(linkname, sendq, sentm, sentkb, recvm, recvkb, time) => IRCMessage {
                prefix: None,
                command: "211".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    linkname.into_string(),
                    sendq.to_string(),
                    sentm.to_string(),
                    sentkb.to_string(),
                    recvm.to_string(),
                    recvkb.to_string(),
                    time.to_string()
                ),
                suffix: None
            },
            RPL_STATSCOMMANDS(command, count, bcount, rcount) => IRCMessage {
                prefix: None,
                command: "212".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    command.into_string(),
                    count.to_string(),
                    bcount.to_string(),
                    rcount.to_string()
                ),
                suffix: None
            },
            RPL_ENDOFSTATS(letter) => IRCMessage {
                prefix: None,
                command: "219".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    letter.into_string()
                ),
                suffix: Some("End of STATS report".into_string())
            },
            RPL_STATSUPTIME(d, h, m, s) => IRCMessage {
                prefix: None,
                command: "242".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(format!("Server Up {} days {}:{:02u}:{:02u}", d, h, m, s))
            },
            RPL_STATSOLINE(hostmask, name) => IRCMessage {
                prefix: None,
                command: "243".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    "O".into_string(),
                    hostmask.into_string(),
                    "*".into_string(),
                    name.into_string()
                ),
                suffix: None
            },
            // Client mode
            RPL_UMODEIS(modes) => IRCMessage {
                prefix: None,
                command: "221".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(modes.into_string())
            },
            // Misc
            RPL_TRYAGAIN(command) => IRCMessage {
                prefix: None,
                command: "263".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    command.into_string()
                ),
                suffix: Some("Please wait a while and try again.".into_string())
            },
            //
            // Errors
            //
            ERR_NOSUCHNICK(nick) => IRCMessage {
                prefix: None,
                command: "401".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    nick.into_string()
                ),
                suffix: Some("No such nick/channel".into_string())
            },
            ERR_NOSUCHSERVER(srv) => IRCMessage {
                prefix: None,
                command: "402".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    srv.into_string()
                ),
                suffix: Some("No such server".into_string())
            },
            ERR_NOSUCHCHANNEL(chan) => IRCMessage {
                prefix: None,
                command: "403".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string()
                ),
                suffix: Some("No such channel".into_string())
            },
            ERR_CANNOTSENDTOCHAN(chan) => IRCMessage {
                prefix: None,
                command: "404".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string()
                ),
                suffix: Some("Cannot send to channel".into_string())
            },
            ERR_TOOMANYCHANNELS(chan) => IRCMessage {
                prefix: None,
                command: "405".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    chan.into_string()
                ),
                suffix: Some("You have joined too many channels".into_string())
            },
            ERR_WASNOSUCHNICK(nick) => IRCMessage {
                prefix: None,
                command: "406".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    nick.into_string()
                ),
                suffix: Some("There was no such nickname".into_string())
            },
            ERR_TOOMANYTARGETS(target, err_code, msg) => IRCMessage {
                prefix: None,
                command: "407".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    target.into_string()
                ),
                suffix: Some(format!("{} recipients. {}", err_code, msg))
            },
            ERR_NOSUCHSERVICE(service) => IRCMessage {
                prefix: None,
                command: "408".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    service.into_string()
                ),
                suffix: Some("No such service".into_string())
            },
            ERR_NOORIGIN => IRCMessage {
                prefix: None,
                command: "409".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("No origin specified".into_string())
            },
            ERR_UNKNOWNCOMMAND(command) => IRCMessage {
                prefix: None,
                command: "421".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    command.into_string()
                ),
                suffix: Some("Unknown command".into_string())
            },
            ERR_NOMOTD => IRCMessage {
                prefix: None,
                command: "422".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("MOTD file is missing".into_string())
            },
            ERR_NOADMININFO(server) => IRCMessage {
                prefix: None,
                command: "423".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    server.into_string()
                ),
                suffix: Some("No administrative info available".into_string())
            },
            ERR_FILEERROR(op, file) => IRCMessage {
                prefix: None,
                command: "424".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(format!("File error doing {} on {}", op, file))
            },
            ERR_UNAVAILRESOURCE(name) => IRCMessage {
                prefix: None,
                command: "437".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    name.into_string()
                ),
                suffix: Some("Nick/channel is temporarily unavailable".into_string())
            },
            // PRIVMSG_ERR
            ERR_NORECIPIENT(command) => IRCMessage {
                prefix: None,
                command: "411".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some(format!("No recipient given ({})", command))
            },
            ERR_NOTEXTTOSEND => IRCMessage {
                prefix: None,
                command: "412".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("No text to send".into_string())
            },
            ERR_NOTOPLEVEL(mask) => IRCMessage {
                prefix: None,
                command: "413".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    mask.into_string()
                ),
                suffix: Some("No toplevel specified".into_string())
            },
            ERR_WILDTOLEVEL(mask) => IRCMessage {
                prefix: None,
                command: "414".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    mask.into_string()
                ),
                suffix: Some("Wildcard in toplevel domain".into_string())
            },
            ERR_BADMASK(mask) => IRCMessage {
                prefix: None,
                command: "415".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    mask.into_string()
                ),
                suffix: Some("Bad Server/host mask".into_string())
            },
            // Nick Related
            ERR_NONICKNAMEGIVEN => IRCMessage {
                prefix: None,
                command: "432".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("No nickname given".into_string())
            },
            ERR_ERRONEUSNICKNAME(nick) => IRCMessage {
                prefix: None,
                command: "432".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    nick.into_string()
                ),
                suffix: Some("Erroneous nickname".into_string())
            },
            ERR_NICKNAMEINUSE(nick) => IRCMessage {
                prefix: None,
                command: "433".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    nick.into_string()
                ),
                suffix: Some("Nickname is already in use".into_string())
            },
            ERR_NICKCOLLISION(nick, user, host) => IRCMessage {
                prefix: None,
                command: "436".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    nick.into_string()
                ),
                suffix: Some(format!("Nickname collision KILL from {}@{}", user, host))
            },
            // Chan related
            ERR_USERNOTINCHANNEL(nick, channel) => IRCMessage {
                prefix: None,
                command: "441".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    nick.into_string(),
                    channel.into_string()
                ),
                suffix: Some("They aren't on that channel".into_string())
            },
            ERR_NOTONCHANNEL(channel) => IRCMessage {
                prefix: None,
                command: "442".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    channel.into_string()
                ),
                suffix: Some("You're not on that channel".into_string())
            },
            ERR_USERONCHANNEL(nick, channel) => IRCMessage {
                prefix: None,
                command: "443".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    nick.into_string(),
                    channel.into_string()
                ),
                suffix: Some("is already on channel".into_string())
            },
            ERR_NOLOGIN(user) => IRCMessage {
                prefix: None,
                command: "444".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    user.into_string()
                ),
                suffix: Some("User not logged in".into_string())
            },
            ERR_KEYSET(channel) => IRCMessage {
                prefix: None,
                command: "467".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    channel.into_string()
                ),
                suffix: Some("Key already set".into_string())
            },
            ERR_CHANNELISFULL(channel) => IRCMessage {
                prefix: None,
                command: "471".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    channel.into_string()
                ),
                suffix: Some("Cannot join channel (+l)".into_string())
            },
            ERR_UNKNOWNMODE(mode, channel) => IRCMessage {
                prefix: None,
                command: "471".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    mode.to_string()
                ),
                suffix: Some(format!("is unknown mode char to me for {}", channel))
            },
            ERR_INVITEONLYCHAN(channel) => IRCMessage {
                prefix: None,
                command: "473".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    channel.into_string()
                ),
                suffix: Some("Cannot join channel (+i)".into_string())
            },
            ERR_BANNEDFROMCHAN(channel) => IRCMessage {
                prefix: None,
                command: "474".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    channel.into_string()
                ),
                suffix: Some("Cannot join channel (+b)".into_string())
            },
            ERR_BADCHANNELKEY(channel) => IRCMessage {
                prefix: None,
                command: "475".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    channel.into_string()
                ),
                suffix: Some("Cannot join channel (+b)".into_string())
            },
            ERR_BADCHANMASK(channel) => IRCMessage {
                prefix: None,
                command: "476".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    channel.into_string()
                ),
                suffix: Some("Bad channel mask".into_string())
            },
            ERR_NOCHANMODES(channel) => IRCMessage {
                prefix: None,
                command: "477".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    channel.into_string()
                ),
                suffix: Some("Channel doesn't support modes".into_string())
            },
            ERR_BANLISTFULL(channel, mode) => IRCMessage {
                prefix: None,
                command: "478".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    channel.into_string(),
                    mode.to_string()
                ),
                suffix: Some("Channel list is full".into_string())
            },
            // NotAlloed related
            ERR_SUMMONDISABLED => IRCMessage {
                prefix: None,
                command: "445".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("SUMMON has been disabled".into_string())
            },
            ERR_USERSDISABLED => IRCMessage {
                prefix: None,
                command: "446".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("USERS has been disabled".into_string())
            },
            ERR_NOTREGISTERED => IRCMessage {
                prefix: None,
                command: "451".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("You have not registered".into_string())
            },
            ERR_NEEDMOREPARAMS(command) => IRCMessage {
                prefix: None,
                command: "461".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    command.into_string()
                ),
                suffix: Some("Not enough parameters".into_string())
            },
            ERR_ALREADYREGISTERED => IRCMessage {
                prefix: None,
                command: "462".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("Unauthorized command (already registered)".into_string())
            },
            ERR_NOPERMFORHOST => IRCMessage {
                prefix: None,
                command: "463".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("Your host isn't among the privileged".into_string())
            },
            ERR_PASSWDMISMATCH => IRCMessage {
                prefix: None,
                command: "464".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("Password incorrect".into_string())
            },
            ERR_YOURBANNEDCREEP => IRCMessage {
                prefix: None,
                command: "465".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("You are banned from this server".into_string())
            },
            ERR_YOUWILLBEBANNED => IRCMessage {
                prefix: None,
                command: "466".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: None
            },
            ERR_NOPRIVILIGES => IRCMessage {
                prefix: None,
                command: "481".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("Permission Denied- You're not an IRC operator".into_string())
            },
            ERR_CHANOPRIVSNEEDED(channel) => IRCMessage {
                prefix: None,
                command: "482".into_string(),
                args: vec!(
                    usrnick.into_string(),
                    channel.into_string()
                ),
                suffix: Some("You're not channel operator".into_string())
            },
            ERR_CANTKILLSERVER => IRCMessage {
                prefix: None,
                command: "483".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("You can't kill a server!".into_string())
            },
            ERR_RESTRICTED => IRCMessage {
                prefix: None,
                command: "484".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("Your connection is restricted!".into_string())
            },
            ERR_UNIQOPPRIVSNEEDED => IRCMessage {
                prefix: None,
                command: "485".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("You're not the original channel operator".into_string())
            },
            ERR_NOOPERHOST => IRCMessage {
                prefix: None,
                command: "491".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("No O-lines for your host".into_string())
            },
            ERR_UMODEUNKNOWNFLAG => IRCMessage {
                prefix: None,
                command: "501".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("Unknown MODE flag".into_string())
            },
            ERR_USERSDONTMATCH => IRCMessage {
                prefix: None,
                command: "502".into_string(),
                args: vec!(usrnick.into_string()),
                suffix: Some("Cannot change mode for other users".into_string())
            },
        }
    }

}
