
use rand::Rng;

use twitch_irc::
{
    message::{PrivmsgMessage},
};

use crate::helpers::readlines_to_vec;
use crate::db_ops::*;


pub async fn range(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            let text = msg_ctx.message_text.as_str();
            let (_name, args) = match text.split_once(' ')
            {
                Some((name, args)) => (name, args),
                None => (text, ""),
            };

            let argv_s: Vec<&str> = args.split_whitespace().collect();
            let Some([arg_1, arg_2]) = argv_s.get(0..2)
            else
            {
                return Ok("Bad argument count! Please make sure your command follows this syntax: !range INT1 INT2".to_string());
            };
            let Ok(mut arg_1) = arg_1.parse::<i32>()
            else
            {
                return Ok(format!("Make sure the first argument provided is a number no greater than {} and no less than {}", i32::MAX, i32::MIN));
            };
            let Ok(mut arg_2) = arg_2.parse::<i32>()
            else
            {
                return Ok(format!("Make sure the second argument provided is a number no greater than {} and no less than {}", i32::MAX, i32::MIN));
            };
            if arg_1 > arg_2 {std::mem::swap(&mut arg_1, &mut arg_2)}
            let rand_int: i32 = rand::thread_rng().gen_range(arg_1..=arg_2);
            Ok(format!("{} your new integer value is {}!", msg_ctx.sender.name, rand_int))
        },
        b'?' =>
        {
            Ok("This command picks a random 32 bit integer in a given range. Use whitespace to separate the numbers. | USAGE: !range INT1 INT2 | !range INT2 INT1 -> swaps larger and smaller to make it easy to use. NOTE: Range command is INCLUSIVE of the upperbound".to_string())
        },
        _ => Ok(String::from("")),
    }
}

pub async fn pick(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            let text = msg_ctx.message_text.as_str(); // get str from msg context
            let (_name, args) = match text.split_once(' ')
            {
                Some((name, args)) => (name, args),
                None => (text, ""),
            };
            // should we ever want to refactor to have whitespace split the 2 tag arguments
            let argv: Vec<&str> = args.split(' ').collect();
            //args.split_whitespace().collect();//::<Vec<String>>();//.join("+")
            let index: usize = rand::thread_rng().gen_range(0..argv.len());
            Ok(format!("picks {}", argv[index]))

        },
        b'?' =>
        {
            Ok("This command picks a single argument from input provided via message. Use whitespace to make another argument for the bot to pick from (will be better in the future) | USAGE: !pick, !pick ARG, !pick ARG1 ARG2, !pick ARG1 ... ARGn |".to_string())
        },
        _ => Ok(String::from("")),
    }
}

pub async fn is_hentai(runtype: u8, _msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            let out: &[&str] = &["This game is hentai DataSweat", "This game is NOT hentai YoumuAngry", "This game could possibly be hentai, but more testing is needed MarisaFace"];
            let index: usize = rand::thread_rng().gen_range(0..out.len());
            Ok(out[index].to_string())

        },
        b'?' =>
        {
            Ok("This command lets the bot decide if any content on the stream contains hentai. NOTE: The author of this command does not guarantee its reliability...".to_string())
        },
        _ => Ok(String::from("")),
    }
}



pub async fn cfb(runtype: u8, _msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            let c_list: Vec<String> = readlines_to_vec("assets/c.txt").expect("Error reading file!");
            let f_list: Vec<String> = readlines_to_vec("assets/f.txt").expect("Error reading file!");
            let b_list: Vec<String> = readlines_to_vec("assets/b.txt").expect("Error reading file!");
            let index_c: usize = rand::thread_rng().gen_range(0..c_list.len());
            let index_f: usize = rand::thread_rng().gen_range(0..f_list.len());
            let index_b: usize = rand::thread_rng().gen_range(0..b_list.len());
            Ok(format!("{} {} {}", c_list[index_c], f_list[index_f], b_list[index_b]))
        },
        b'?' =>
        {
            Ok("This command generates a string containing words that start with C, F, and B".to_string())
        },
        _ => Ok(String::from("")),
    }
}

pub async fn kinohackers(runtype: u8, _msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            let id: i32 = rand::thread_rng().gen_range(1..=get_kinohack_count()).try_into().unwrap();
            let queried_link: String = query_kinohackers(id);
            return Ok(queried_link);
        },
        b'?' =>
        {
            return Ok("This command gives you a brand kinohackers meme made by various members of the Claude influencer circle".to_string());
        },
        _ => Ok(String::from("")),
    }
}

pub async fn strive(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            return Ok("😆 👉 STRIVE".to_string());
        },
        b'?' =>
        {
            return Ok("GGSTRIVE4EVER... For '#' runtype: #strive <chatter>".to_string());
        },
        b'#' =>
        {
            let text = msg_ctx.message_text.as_str(); // get str from msg context
            let (_name, args) = match text.split_once(' ')
            {
                Some((name, args)) => (name, args),
                None => (text, ""),
            };
            let argv_s: Vec<&str> = args.split(' ').collect();
            // check arg count
            if argv_s.len() != 1{return Ok("Bad argument count! Please make sure your command follows this syntax: #strive <chatter>".to_string());}
            return Ok(format!("{} accuses {} of being a Strive player!!!", msg_ctx.sender.name, argv_s[0]));
        },
        _ => Ok(String::from("")),
    }
}

pub async fn shftnw(runtype: u8, msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
{
    match runtype
    {
        b'!' =>
        {
            return Ok(format!("{} loves Shadow Hearts: From the New World!", msg_ctx.sender.name));
        },
        b'?' =>
        {
            return Ok("This is the worlds most useless command".to_string());
        },
        _ => Ok(String::from("")),
    }
}

// SIMPLE STRING COMMANDS
// a simple command is a command that generates the same text string every time
macro_rules! generate_simple_command
{
    ($fn_name:ident, $text:literal) =>
    {
        pub async fn $fn_name(runtype: u8, _msg_ctx: PrivmsgMessage) -> anyhow::Result<String>
        {
            match runtype
            {
                b'!' =>
                {
                    return Ok(format!($text));
                }
                _ => Ok(String::from("")),
            }
        }
    };
}
generate_simple_command!(cmds, "Current Commands: dreamboumtweet, demongacha, savedemon, hornedanimegacha, chen, speedgame, pic, pick, range, hentai, kinohackers, melty, lumina, melee, soku, bbcf, ggxxacplusr, akb, vsav, jojos, millions, cfb, me, help, cmds, repo, weekly");
generate_simple_command!(help, "Blueayachan version 2 supports multiple different \"runtype\" characters : \'!\' is supposed to produce similar functionality to the previous bot. \'?\' should give information and help regarding that command. \'#\' does the standard command with different functionality that is specific to the command itself. For a list of commands type !cmds");
generate_simple_command!(poll, "THERE'S STILL TIME TO VOTE IN THE POLL! http://bombch.us/DYOt CirnoGenius");
generate_simple_command!(repo, "You can find the github repository here: https://github.com/electra13x7777/blueayachan_v2");
generate_simple_command!(fsr, "🌻 ☀️ 🌧️");
generate_simple_command!(weekly, "Last Week's Top 15: https://imgur.com/a/PYmokTp");