//! Numeric replys of the protocol according to RCF 2812.

#![experimental]

use super::IRCMessage;

/// Lists all possible numerical answers from the server.
#[allow(non_camel_case_types)]
#[deriving(Show,PartialEq,FromPrimitive)]
#[repr(uint)]
#[experimental]
pub enum NumericReply {
    //
    // Connection welcome
    //
    RPL_WELCOME         = 001,
    RPL_YOURHOST        = 002,
    RPL_CREATED         = 003,
    RPL_MYINFO          = 004,
    RPL_BOUNCE          = 005,
    //
    // Responses
    //
    RPL_USERHOST        = 302,
    RPL_ISON            = 303,
    // Away related
    RPL_AWAY            = 301,
    RPL_UNAWAY          = 305,
    RPL_NOWAWAY         = 306,
    // Whois related
    RPL_WHOISUSER       = 311,
    RPL_WHOISSERVER     = 312,
    RPL_WHOISOPERAtOR   = 313,
    RPL_WHOISIDLE       = 317,
    RPL_ENDOFWHOIS      = 318,
    RPL_WHOISCHANNELS   = 319,
    // Whowas related
    RPL_WHOWASUSER      = 314,
    RPL_ENDOFWHOWAS     = 369,
    // List related
    RPL_LIST            = 322,
    RPL_LISTEND         = 323,
    RPL_SERVLIST        = 234,
    RPL_SERVLISTEND     = 235,
    RPL_LUSERCLIENT     = 251,
    RPL_LUSEROP         = 252,
    RPL_LUSERUNKNOWN    = 253,
    RPL_LUSERCHANNELS   = 254,
    RPL_LUSERME         = 255,
    // Chan related
    RPL_UNIQOPIS        = 325,
    RPL_CHANNELMODEIS   = 324,
    RPL_NOTOPIC         = 331,
    RPL_TOPIC           = 332,
    RPL_INVITING        = 341,
    RPL_SUMMONING       = 342,
    RPL_INVITELIST      = 346,
    RPL_ENDOFINVITELIST = 347,
    RPL_EXCEPTLIST      = 348,
    RPL_ENDOFEXCEPTLIST = 349,
    RPL_BANLIST         = 367,
    RPL_ENDOFBANLIST    = 368,
    // Server related
    RPL_VERSION         = 351,
    RPL_WHOREPLY        = 352,
    RPL_ENDOFWHO        = 315,
    RPL_NAMEREPLY       = 353,
    RPL_ENDOFNAMES      = 366,
    RPL_LINKS           = 364,
    RPL_ENDOFLINKS      = 365,
    RPL_INFO            = 371,
    RPL_ENDOFINFO       = 374,
    RPL_TIME            = 391,
    // MOTD
    RPL_MOTDSTART       = 375,
    RPL_MOTD            = 372,
    RPL_ENDOFMOTD       = 376,
    // Administration
    RPL_YOUREOPER       = 381,
    RPL_REHASHING       = 382,
    RPL_YOURESERVICE    = 383,
    RPL_ADMINME         = 256,
    RPL_ADMINLOC1       = 257,
    RPL_ADMINLOC2       = 258,
    RPL_ADMINEMAIL      = 259,
    // Users related
    RPL_USERSSTART      = 392,
    RPL_USERS           = 393,
    RPL_ENDOFUSERS      = 394,
    RPL_NOUSERS         = 395,
    // Trace related
    RPL_TRACELINK       = 200,
    RPL_TRACECONNECTING = 201,
    RPL_TRACEHANDSHAKE  = 202,
    RPL_TRACEUNKNOWN    = 203,
    RPL_TRACEOPERATOR   = 204,
    RPL_TRACEUSER       = 205,
    RPL_TRACESERVER     = 206,
    RPL_TRACESERVICE    = 207,
    RPL_TRACENEWTYPE    = 208,
    RPL_TRACECLASS      = 209,
    RPL_TRACELOG        = 261,
    RPL_TRACEEND        = 262,
    // Stats related
    RPL_STATSLINKINFO   = 211,
    RPL_STATSCOMMANDS   = 212,
    RPL_ENDOFSTATS      = 219,
    RPL_STATSUPTIME     = 242,
    RPL_STATSOLINE      = 243,
    // Client mode
    RPL_UMODEIS        = 221,
    // Misc
    RPL_TRYAGAIN        = 263,
    //
    // Errors
    //
    ERR_NOSUCHNICK      = 401,
    ERR_NOSUCHSERVER    = 402,
    ERR_NOSUCHCHANNEL   = 403,
    ERR_CANNOTSENDTOCHAN= 404,
    ERR_TOOMANYCHANNELS = 405,
    ERR_WASNOSUCHNICK   = 406,
    ERR_TOOMANYTARGETS  = 407,
    ERR_NOSUCHSERVICE   = 408,
    ERR_NOORIGIN        = 409,
    ERR_UNKNOWNCOMMAND  = 421,
    ERR_NOMOTD          = 422,
    ERR_NOADMININFO     = 423,
    ERR_FILEERROR       = 424,
    ERR_UNAVAILRESOURCE = 437,
    // PRIVMSG_ERR
    ERR_NORECIPIENT     = 411,
    ERR_NOTEXTTOSEND    = 412,
    ERR_NOTOPLEVEL      = 413,
    ERR_WILDTOLEVEL     = 414,
    ERR_BADMASK         = 415,
    // Nick Related
    ERR_NONICKNAMEGIVEN = 431,
    ERR_ERRONEUSNICKNAME= 432,
    ERR_NICKNAMEINUSE   = 433,
    ERR_NICKCOLLISION   = 436,
    // Chan related
    ERR_USERNOTINCHANNEL= 441,
    ERR_NOTONCHANNEL    = 442,
    ERR_USERONCHANNEL   = 443,
    ERR_NOLOGIN         = 444,
    ERR_KEYSET          = 467,
    ERR_CHANNELISFULL   = 471,
    ERR_UNKNOWNMODE     = 472,
    ERR_INVITEONLYCHAN  = 473,
    ERR_BANNEDFROMCHAN  = 474,
    ERR_BADCHANNELKEY   = 475,
    ERR_BADCHANMASK     = 476,
    ERR_NOCHANMODES     = 477,
    ERR_BANLISTFULL     = 478,
    // NotAlloed related
    ERR_SUMMONDISABLED  = 445,
    ERR_USERSDISABLED   = 446,
    ERR_NOTREGISTERED   = 451,
    ERR_NEEDMOREPARAMS  = 461,
    ERR_ALREADYREGISTERED=462,
    ERR_NOPERMFORHOST   = 463,
    ERR_PASSWDMISMATCH  = 464,
    ERR_YOURBANNEDCREEP = 465,
    ERR_YOUWILLBEBANNED = 466,
    ERR_NOPRIVILIGES    = 481,
    ERR_CHANOPRIVSNEEDED= 482,
    ERR_CANTKILLSERVER  = 483,
    ERR_RESTRICTED      = 484,
    ERR_UNIQOPPRIVSNEEDED=485,
    ERR_NOOPERHOST      = 491,
    ERR_UMODEUNKNOWNFLAG= 501,
    ERR_USERSDONTMATCH  = 502
}

#[experimental]
impl NumericReply {

    /// Attemps to extract the numeric reply from an `IRCMessage`.
    #[experimental]
    pub fn from_ircmessage(msg: &IRCMessage) -> Option<NumericReply> {
        match from_str::<uint>(msg.command.as_slice()) {
            Some(num) => FromPrimitive::from_uint(num),
            None => None
        }
    }

    /// 3 digit string representing the numeric reply.
    #[experimental]
    pub fn to_text(self) -> String {
        format!("{:03u}", self as uint)
    }

}
