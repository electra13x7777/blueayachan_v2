

use rand::Rng;

use crate::{helpers::{readlines_to_vec}, commands::{Command, Runtype}};
use crate::db_ops::*;
use crate::models::*;

pub async fn dreamboumtweet(runtype: Runtype, _command: Command) -> anyhow::Result<String>//Option<String>//(String, String)
{
    //const TOTAL_TWEETS: usize = 6569;
    match runtype
    {
        Runtype::Command =>
        {
            let id: i32 = rand::thread_rng().gen_range(1..=get_dbt_count()).try_into().unwrap();
            let tweet_ctx = query_single_dbtweet(id);
            return Ok(tweet_ctx);
        },
        Runtype::Help =>
        {
            return Ok(format!("This command sends a random tweet made by twitter user @Dreamboum. TOTAL_TWEETS: {}", get_dbt_count()));
        },
        Runtype::Hash =>
        {
            let dbt_vec = readlines_to_vec("assets/dreamboum_tweets_10_05_2022.txt").expect("Could not load lines");
            let index = rand::thread_rng().gen_range(0..dbt_vec.len());
            let _splitpoint: usize = 13;
            let _length = dbt_vec[index].len();
            let tweet_ctx: &str = &dbt_vec[index];
            return Ok(String::from(tweet_ctx));
        },
        Runtype::Tilde =>
        {
            let dbt_vec: Vec<(String, String)> = query_dbtweet_to_vec();
            let index = rand::thread_rng().gen_range(1..=dbt_vec.len());
            let tweet_ctx =  &dbt_vec[index].0;
            //let date_ctx = &dbt_vec[index].1;
            return Ok(String::from(tweet_ctx));
        },
    }
}

// DEMONGACHA
pub async fn demongacha(runtype: Runtype, command: Command) -> anyhow::Result<String>
{
    match runtype
    {
        Runtype::Command =>
        {
            // query random demon
            let id: i32 =rand::thread_rng().gen_range(1..=get_demon_count()).try_into().unwrap();
            //println!("{}", id);
            let demon: NDemon = query_demon(id);
            // get rarity
            let rarity_weight: i8 = rand::thread_rng().gen_range(0..=100);
            let rarity =
            if rarity_weight>=95
            {
                5
            }
            else if rarity_weight >= 80
            {
                4
            }
            else if rarity_weight >= 60
            {
                3
            }
            else if rarity_weight >= 35
            {
                2
            }
            else
            {
                1
            };

            // HANDLE AUX DB STUFF
            // TODO: CHANGE THIS TO QUERY BY ID
            let bacuser: BACUser = query_user_data(&command.msg.sender.name);
            //let name = demon.demon_name;
            handle_user_last_demon(&bacuser, &demon, rarity);
            if &demon.demon_name == "Kusi Mitama"
            {
                return Ok(format!("{} summoned a {}⭐ NAME_CENSORED_BY_TWITCH_POLICE Mitama! {}", command.msg.sender.name, rarity, demon.demon_img_link));

            }
            return Ok(format!("{} summoned a {}⭐ {}! {}", command.msg.sender.name, rarity, demon.demon_name, demon.demon_img_link));
        },
        Runtype::Help =>
        {

            return Ok(format!("This command summons a random demon from Shin Megami Tensei III: Nocturne. Use <!savedemon> to save your last demon. Use <#demongacha> to see your saved demon. TOTAL_DEMONS: {}", get_demon_count()));
        },
        Runtype::Hash =>
        {
            // TODO: need to check for no table entry
            let bacuser: BACUser = query_user_data(&command.msg.sender.name);
            let sud = match query_user_demon(&bacuser)
            {
                // HANDLE SOME (GOOD DATA)
                Some(sud) => sud,
                // HANDLE NONE (NO DATA)
                None =>
                {
                    return Ok(format!("NO SAVED DEMON FOUND!!! {} please run <!demongacha> and <!savedemon> first!!!", command.msg.sender.name));
                },
            };
            let demon: NDemon = query_demon(sud.saved_demon_id);
            return Ok(format!("{} has a {}⭐ {}! {}", bacuser.user_nick, sud.saved_demon_rarity, demon.demon_name, demon.demon_img_link));

        },
        _ =>
        {
            Ok(String::from(""))
        },
    }
}

pub async fn savedemon(runtype: Runtype, command: Command) -> anyhow::Result<String>
{
    match runtype
    {
        Runtype::Command =>
        {
            let bacuser: BACUser = query_user_data(&command.msg.sender.name);
            save_user_demon(&bacuser);
            return Ok(format!("{} saved their last demon", command.msg.sender.name));
        },
        Runtype::Help =>
        {

            return Ok("This command saves the last demon you summoned with !demongacha.".to_string());
        },
        _ =>
        {
            Ok(String::from(""))
        },
    }
}


pub async fn hornedanimegacha(runtype: Runtype, command: Command) -> anyhow::Result<String>
{
    match runtype
    {
        Runtype::Command =>
        {
            // query random demon
            let id: i32 = rand::thread_rng().gen_range(1..=get_hornedanime_count()).try_into().unwrap();
            let ha: String = query_hornedanime(id);
            // get rarity
            let rarity_weight: i8 = rand::thread_rng().gen_range(0..=100);
            let rarity =
            if rarity_weight>=95
            {
                5
            }
            else if rarity_weight >= 80 //&& rarity_weight < 95
            {
                4
            }
            else if rarity_weight >= 60 //&& rarity_weight < 80
            {
                3
            }
            else if rarity_weight >= 35 //&& rarity_weight < 60
            {
                2
            }
            else
            {
                1
            };
            return Ok(format!("{} rolled a {}⭐ {}!", command.msg.sender.name, rarity, ha));
        },
        Runtype::Help =>
        {

            return Ok(format!("This command rolls for a random HornedAnime. TOTAL_HORNEDANIMES: {}", get_hornedanime_count()));
        },
        _ =>
        {
            Ok(String::from(""))
        },
    }
}

pub async fn melty(runtype: Runtype, command: Command) -> anyhow::Result<String>
{
    match runtype
    {
        Runtype::Command =>
        {
            let id: i32 = rand::thread_rng().gen_range(1..=get_melty_count()).try_into().unwrap();
            let queried_string: String = query_melty(id);
            let moonstyle_r: i8 = rand::thread_rng().gen_range(0..3);
            let moon: &str = match moonstyle_r
            {
                0 => "Crescent Moon",
                1 => "Half Moon",
                2 => "Full Moon",
                _ => "",
            };
            return Ok(format!("{} your new main in Melty Blood: Actress Again is {} {}!", command.msg.sender.name, moon, queried_string));
        },
        Runtype::Help =>
        {
            return Ok("This command gives you a brand new main for Melty Blood: Actress Again".to_string());
        },
        _ => Ok(String::from("")),
    }
}

pub async fn chen(runtype: Runtype, command: Command) -> anyhow::Result<String>
{
    match runtype
    {
        Runtype::Command =>
        {
            let chen_str: (String, String) = match command.msg.channel_login.as_str() // will read from a database eventually
            {
                "claude" => (String::from("HONKHONK"), String::from("CirnoGenius")),
                "blueayachan" => (String::from("saHonk"), String::from("CirnoGenius")),
                "darko_rta" => (String::from("saHonk"), String::from("CirnoGenius")),
                "electra_rta" => (String::from("saHonk"), String::from("CirnoGenius")),
                "crypton42" => (String::from("saHonk"), String::from("saHonk")),
                _ => return Ok(String::from("")),
            };
            let chens: usize = rand::thread_rng().gen_range(0..=10);
            match chens
            {
                0 => return Ok(format!("{}... got 0 chens :(", command.msg.sender.name)),
                1 => return Ok(format!("{} got {} chen. {}",  command.msg.sender.name, chens, chen_str.0)),
                _ => {}
            };
            let mut new_chens: String = "".to_owned();
            for i in 1..=chens
            {
                match chens
                {
                    9 => new_chens += &chen_str.1,
                    _ => new_chens += &chen_str.0
                }

                if i != chens
                {
                    new_chens += " ";
                }
            }
            Ok(format!("{} got a {} chen combo! {}",  command.msg.sender.name, chens, new_chens))
        },
        Runtype::Help =>
        {
            Ok("This command gives you chens. Yay!".to_string())
        },
        Runtype::Hash =>
        {
            let chen_str: String = match command.msg.channel_login.as_str() // will read from a database eventually
            {
                "claude" => String::from("HONKHONK"),
                "blueayachan" => String::from("saHonk"),
                "darko_rta" => String::from("saHonk"),
                "electra_rta" => String::from("saHonk"),
                "crypton42" => String::from("saHonk"),
                _ => return Ok(String::from("")),
            };
            let chens: usize = rand::thread_rng().gen_range(0..=10);
            match chens
            {
                0 => return Ok("0 chens :(".to_string()),
                1 => return Ok(chen_str),
                _ => chens
            };
            let mut new_chens: String = "".to_owned();
            for i in 1..=chens
            {
                new_chens += &chen_str;
                if i != chens
                {
                    new_chens += " ";
                }
            }
            Ok(new_chens)
        },
        _ => Ok(String::from("")),
    }
}

// SIMPLE GACHA COMMANDS
macro_rules! generate_simple_gacha
{
    ($fn_name:ident, $game_name:literal, $count:ident, $query_fn:ident) =>
    {
        pub async fn $fn_name(runtype: Runtype, command: Command) -> anyhow::Result<String>
        {
            match runtype
            {
                Runtype::Command =>
                {
                    let id: i32 = rand::thread_rng().gen_range(1..=$count()).try_into().unwrap();
                    let queried_string: String = $query_fn(id);
                    return Ok(format!("{} your new main in {} is {}!", command.msg.sender.name, $game_name, queried_string));
                },
                Runtype::Help =>
                {
                    return Ok(format!("This command gives you a brand new main for {}", $game_name));
                },
                _ => Ok(String::from("")),
            }
        }
    };
}
generate_simple_gacha!(lumina, "Melty Blood: Type Lumina", get_lumina_count, query_lumina);
generate_simple_gacha!(melee, "Super Smash Bros. Melee", get_melee_count, query_melee);
generate_simple_gacha!(soku, "Touhou 12.3: Hisoutensoku", get_soku_count, query_soku);
generate_simple_gacha!(bbcf, "BlazBlue Centralfiction", get_bbcf_count, query_bbcf);
generate_simple_gacha!(ggxxacplusr, "Guilty Gear XX Accent Core Plus R", get_ggxxacplusr_count, query_ggxxacplusr);
generate_simple_gacha!(akb, "Akatsuki Blitzkampf Ausf. Achse", get_akb_count, query_akb);
generate_simple_gacha!(vsav, "Vampire Savior: The Lord of Vampire", get_vsav_count, query_vsav);
generate_simple_gacha!(jojos, "JoJo\'s Bizarre Adventure: Heritage for the Future", get_jojo_count, query_jojo);
generate_simple_gacha!(millions, "Million Arthur: Arcana Blood", get_millions_count, query_millions);