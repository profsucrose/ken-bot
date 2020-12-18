
extern crate discord;
extern crate dotenv;

use discord::model::{EmojiId, Event, ReactionEmoji, UserId};
use discord::Discord;
use std::env;

fn main() {
    dotenv::dotenv().ok();

	let discord = Discord::from_bot_token(&env::var("DISCORD_TOKEN").expect("Expected token"))
		.expect("login failed");

	let (mut connection, _) = discord.connect().expect("connect failed");
    
    println!("Started Ken Bot");
	loop {
		match connection.recv_event() {
			Ok(Event::MessageCreate(message)) => {
                if message.author.id == UserId(734851161823248525) {
                    discord.add_reaction(
                        message.channel_id, 
                        message.id, 
                        ReactionEmoji::Custom { 
                            name: String::from("downvote"), 
                            id: EmojiId(773668231327121448) 
                        }
                    ).expect("Failed to react to message with :downvote:");    
                }
			}
			Ok(_) => {}
			Err(discord::Error::Closed(code, body)) => {
				println!("Gateway closed with code {:?}: {}", code, body);
				break;
			}
			Err(err) => println!("Receive error: {:?}", err),
		}
	}
}