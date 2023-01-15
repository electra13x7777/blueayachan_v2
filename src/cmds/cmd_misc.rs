
use rand::Rng;

use crate::{helpers::readlines_to_vec, commands::{Runtype, Command}};
use crate::db_ops::*;


pub async fn range(runtype: Runtype, command: Command) -> anyhow::Result<String>
{
    fn arg_is_int(s: &str) -> bool
    {
        for (i, c) in s.chars().enumerate()
        {
            if !c.is_numeric() && c != '-' || s.len() == 1 && !c.is_numeric() || i != 0 && !c.is_numeric()
            {
                return false;
            }
        }
        return true;
    }
    match runtype
    {
        Runtype::Command =>
        {
            const I64LEN: usize = "9223372036854775807".len();

            let argv_s: Vec<&str> = command.args().split(' ').collect();
            // check arg count
            if argv_s.len() != 2{return Ok("Bad argument count! Please make sure your command follows this syntax: !range INT1 INT2".to_string());}
            // check if int
            if !arg_is_int(argv_s[0]) || !arg_is_int(argv_s[1]){return Ok("Bad argument found! Please make sure you are providing INTEGERS as arguments. Ex) 1000, -500, 69, -420".to_string());}
            // check input string length
            if argv_s[0].len() >= I64LEN || argv_s[1].len() >= I64LEN
            {
                return Ok("One or more of the arguments provided are not only above 32 bits, they are also above max signed 64bit integer bounds...".to_string());
            }
            let argv: &mut [i64] = &mut [argv_s[0].parse::<i64>().unwrap(), argv_s[1].parse::<i64>().unwrap()];
            if argv[0] >= i32::MAX.into() || argv[0] <= i32::MIN.into()
            {
                return Ok(format!("Make sure the first argument provided is no greater than {} and no less than {}", i32::MAX, i32::MIN));
            }
            if argv[1] >= i32::MAX.into() || argv[1] <= i32::MIN.into()
            {
                return Ok(format!("Make sure the second argument provided is no greater than {} and no less than {}", i32::MAX, i32::MIN));
            }

            if argv[0] > argv[1]{argv.swap(0, 1);}
            let rand_int: i64 = rand::thread_rng().gen_range(argv[0]..=argv[1]);
            Ok(format!("{} your new integer value is {}!", command.msg.sender.name, rand_int))
        },
        Runtype::Help =>
        {
            Ok("This command picks a random 32 bit integer in a given range. Use whitespace to separate the numbers. | USAGE: !range INT1 INT2 | !range INT2 INT1 -> swaps larger and smaller to make it easy to use. NOTE: Range command is INCLUSIVE of the upperbound".to_string())
        },
        _ => Ok(String::from("")),
    }
}

pub async fn pick(runtype: Runtype, command: Command) -> anyhow::Result<String>
{
    match runtype
    {
        Runtype::Command =>
        {
            // should we ever want to refactor to have whitespace split the 2 tag arguments
            let argv: Vec<&str> = command.args().split(' ').collect();
            //args.split_whitespace().collect();//::<Vec<String>>();//.join("+")
            let index: usize = rand::thread_rng().gen_range(0..argv.len());
            Ok(format!("picks {}", argv[index]))

        },
        Runtype::Help =>
        {
            Ok("This command picks a single argument from input provided via message. Use whitespace to make another argument for the bot to pick from (will be better in the future) | USAGE: !pick, !pick ARG, !pick ARG1 ARG2, !pick ARG1 ... ARGn |".to_string())
        },
        _ => Ok(String::from("")),
    }
}

pub async fn is_hentai(runtype: Runtype, _command: Command) -> anyhow::Result<String>
{
    match runtype
    {
        Runtype::Command =>
        {
            let out: &[&str] = &["This game is hentai DataSweat", "This game is NOT hentai YoumuAngry", "This game could possibly be hentai, but more testing is needed MarisaFace"];
            let index: usize = rand::thread_rng().gen_range(0..out.len());
            Ok(out[index].to_string())

        },
        Runtype::Help =>
        {
            Ok("This command lets the bot decide if any content on the stream contains hentai. NOTE: The author of this command does not guarantee its reliability...".to_string())
        },
        _ => Ok(String::from("")),
    }
}



pub async fn cfb(runtype: Runtype, _command: Command) -> anyhow::Result<String>
{
    match runtype
    {
        Runtype::Command =>
        {
            let c_list: Vec<String> = readlines_to_vec("assets/c.txt").expect("Error reading file!");
            let f_list: Vec<String> = readlines_to_vec("assets/f.txt").expect("Error reading file!");
            let b_list: Vec<String> = readlines_to_vec("assets/b.txt").expect("Error reading file!");
            let index_c: usize = rand::thread_rng().gen_range(0..c_list.len());
            let index_f: usize = rand::thread_rng().gen_range(0..f_list.len());
            let index_b: usize = rand::thread_rng().gen_range(0..b_list.len());
            Ok(format!("{} {} {}", c_list[index_c], f_list[index_f], b_list[index_b]))
        },
        Runtype::Help =>
        {
            Ok("This command generates a string containing words that start with C, F, and B".to_string())
        },
        _ => Ok(String::from("")),
    }
}

pub async fn kinohackers(runtype: Runtype, _command: Command) -> anyhow::Result<String>
{
    match runtype
    {
        Runtype::Command =>
        {
            let id: i32 = rand::thread_rng().gen_range(1..=get_kinohack_count()).try_into().unwrap();
            let queried_link: String = query_kinohackers(id);
            return Ok(queried_link);
        },
        Runtype::Help =>
        {
            return Ok("This command gives you a brand kinohackers meme made by various members of the Claude influencer circle".to_string());
        },
        _ => Ok(String::from("")),
    }
}

pub async fn strive(runtype: Runtype, command: Command) -> anyhow::Result<String>
{
    match runtype
    {
        Runtype::Command =>
        {
            return Ok("😆 👉 STRIVE".to_string());
        },
        Runtype::Help =>
        {
            return Ok("GGSTRIVE4EVER... For '#' runtype: #strive <chatter>".to_string());
        },
        Runtype::Hash =>
        {
            let argv_s: Vec<&str> = command.args().split(' ').collect();
            // check arg count
            if argv_s.len() != 1{return Ok("Bad argument count! Please make sure your command follows this syntax: #strive <chatter>".to_string());}
            return Ok(format!("{} accuses {} of being a Strive player!!!", command.msg.sender.name, argv_s[0]));
        },
        _ => Ok(String::from("")),
    }
}

pub async fn shftnw(runtype: Runtype, command: Command) -> anyhow::Result<String>
{
    match runtype
    {
        Runtype::Command =>
        {
            return Ok(format!("{} loves Shadow Hearts: From the New World!", command.msg.sender.name));
        },
        Runtype::Help =>
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
        pub async fn $fn_name(runtype: Runtype, _command: Command) -> anyhow::Result<String>
        {
            match runtype
            {
                Runtype::Command =>
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