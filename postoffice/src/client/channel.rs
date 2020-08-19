use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;
use crate::client::{start_connection,send_message_async,get_random_connection_id,Response};
use json::{JsonValue,parse};
use futures::future::{BoxFuture,join_all};

#[derive(Clone, Debug)]
pub struct Member{
    pub name:String,
    pub connection_id:String
}

impl Member{
    #[allow(dead_code)]
    fn new(name:&String,address:&String,key:&String)->Result<Member,&'static str>{
        let connection_id = get_random_connection_id();
        match start_connection(&connection_id, address.to_string(), key.to_string()){
            Ok(_)=>{
                return Ok(Member {
                    name:name.clone(),
                    connection_id:connection_id
                });
            },
            Err(_)=>{
                return Err("failed-start_connection");
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Channel{
    pub last:String,
    pub members:Vec<String>,
    pub map:HashMap<String,Member>
}

impl Channel{
    #[allow(dead_code)]
    fn new() -> Channel{
        Channel{
            last:String::new(),
            members:Vec::new(),
            map:HashMap::new()
        }
    }
}

lazy_static! {
    static ref CHANNELS : Mutex<HashMap<String,Channel>> = Mutex::new(HashMap::new());
}

#[allow(dead_code)]
pub fn add_member(channel_name:&String,member_name:&String,address:&String,key:&String) -> Result<(),&'static str>{
    match CHANNELS.lock(){
        Ok(mut lock)=>{
            if !lock.contains_key(&channel_name.clone()){
                match lock.insert(channel_name.clone(),Channel::new()){
                    Some(_)=>{},
                    None=>{}
                }
            }
            match lock.get_mut(channel_name){
                Some(channel)=>{
                    match Member::new(&member_name,&address,&key){
                        Ok(m)=>{
                            channel.members.push(m.name.clone());
                            match channel.map.insert(m.name.clone(),m){
                                Some(_)=>{},
                                None=>{}
                            }
                            return Ok(());
                        },
                        Err(_)=>{
                            return Err("failed-make-new_member");
                        }
                    }
                },
                None=>{
                    return Err("failed-get_channel_from_hashmap");
                }
            }
        },
        Err(_)=>{
            return Err("failed-lock-memebers");
        }
    }
}

#[allow(dead_code)]
fn get_member(channel_name:&String) -> Result<Member,&'static str>{
    match CHANNELS.lock(){
        Ok(mut lock)=>{
            match lock.get_mut(channel_name){
                Some(mut channel)=>{
                    if channel.members.len() == 0{
                        return Err("failed-no_member-found-get_member");
                    }
                    if channel.members.len() == 1 || !channel.members.contains(&channel.last){
                        match channel.map.get(&channel.members[0]){
                            Some(v)=>{
                                return Ok(v.clone());
                            },
                            None=>{
                                return Err("failed-get_member_from_map-get_member");
                            }
                        }
                    }
                    let mut next_member:String = String::new();
                    let mut select_member = false;
                    let mut member_selected = false;
                    while !member_selected{
                        for member in &channel.members{
                            if select_member{
                                next_member = member.to_string();
                                member_selected = true;
                                break;
                            }
                            if member == &channel.last{
                                select_member = true;
                            }
                        }
                    }
                    channel.last = next_member.clone();
                    match channel.map.get(&next_member){
                        Some(v)=>{
                            return Ok(v.clone());
                        },
                        None=>{
                            return Err("failed-get_member_from_map-get_member");
                        }
                    }
                },
                None=>{
                    return Err("failed-get_channel_from_hashmap");
                }
            }
        },
        Err(_)=>{
            return Err("failed-lock-memebers-get_member");
        }
    }
}

#[allow(dead_code)]
pub struct ChannelResponse{
    pub name:String,
    pub data:JsonValue
}

#[allow(dead_code)]
pub async fn send(channel_name:&String,message:&JsonValue,secure:bool) -> Result<ChannelResponse,&'static str>{
    match get_member(&channel_name){
        Ok(member)=>{
            let run = send_with_member(&member, message, secure).await;
            match run{
                Ok(r)=>{
                    return Ok(r);
                },
                Err(_)=>{
                    return Err("no-error");
                }
            }
        },
        Err(_)=>{
            return Err("failed-get_memeber-send");
        }
    }
}

#[allow(dead_code)]
pub async fn send_with_member(m:&Member,message:&JsonValue,secure:bool) -> Result<ChannelResponse,&'static str>{
    let run = process_message_async(&m.connection_id,&message.dump(),secure).await;
    match run {
        Ok(resp)=>{
            match parse(&resp.message){
                Ok(parsed)=>{
                    return Ok(ChannelResponse {
                        name:m.name.clone(),
                        data:parsed
                    });
                },
                Err(_)=>{
                    return Err("failed-parse_response");
                }
            }
        },
        Err(_)=>{
            return Err("failed-send_message");
        }
    }
}

#[allow(dead_code)]
pub async fn send_to_member(channel_name:&String,member_name:&String,message:&JsonValue,secure:bool) -> Result<ChannelResponse,&'static str>{
    match CHANNELS.lock(){
        Ok(mut lock)=>{
            match lock.get_mut(channel_name){
                Some(channel)=>{
                    if !channel.map.contains_key(member_name){
                        return Err("failed-member-not_found");
                    }
                    match channel.map.get(member_name){
                        Some(member)=>{
                            let run = process_message_async(&member.connection_id,&message.dump(),secure).await;
                            match run {
                                Ok(resp)=>{
                                    match parse(&resp.message){
                                        Ok(parsed)=>{
                                            return Ok(ChannelResponse {
                                                name:member.name.clone(),
                                                data:parsed
                                            });
                                        },
                                        Err(_)=>{
                                            return Err("failed-parse_response");
                                        }
                                    }
                                },
                                Err(_)=>{
                                    return Err("failed-send_message");
                                }
                            }
                        },
                        None=>{
                            return Err("failed-get-member-from_map");
                        }
                    }
                },
                None=>{
                    return Err("failed-get_channel_from_hashmap");
                }
            }
        },
        Err(_)=>{
            return Err("failed-lock-channels");
        }
    }
}

#[allow(dead_code)]
async fn process_message_async(connection_id:&String,message:&String,secure:bool) -> Result<Response,&'static str>{
    let run = send_message_async(&connection_id, message.to_string(), secure).await;
    match run {
        Ok(r)=>{
            return Ok(r);
        },
        Err(_)=>{
            return Err("failed-send_message");
        }
    }
}

#[allow(dead_code)]
pub async fn brodcast(channel_name:&String,message:JsonValue,secure:bool)->Result<Vec<ChannelResponse>,&'static str>{
    let message_as_string = message.dump();
    match CHANNELS.lock(){
        Ok(mut lock)=>{
            match lock.get_mut(channel_name){
                Some(channel)=>{
                    let mut collect:Vec<BoxFuture<Result<ChannelResponse,&'static str>>> = Vec::new();
                    for member in channel.map.values(){
                        collect.push(
                            Box::pin(process_message_async_as_channel_response(&member.name,&member.connection_id,&message_as_string,secure))
                        );
                    }
                    let hold = join_all(collect).await;
                    let mut recollect:Vec<ChannelResponse> = Vec::new();
                    for resp in hold {
                        match resp{
                            Ok(v)=>{
                                recollect.push(v);
                            },
                            Err(_)=>{
                                return Err("failed-send-request");
                            }
                        }
                    }
                    return Ok(recollect);
                },
                None=>{
                    return Err("failed-get_channel_from_hashmap");
                }
            }
        },
        Err(_)=>{
            return Err("failed-lock-members-brodcast");
        }
    }
}

#[allow(dead_code)]
async fn process_message_async_as_channel_response(name:&String,connection_id:&String,message:&String,secure:bool) -> Result<ChannelResponse,&'static str>{
    let run = send_message_async(&connection_id, message.to_string(), secure).await;
    match run {
        Ok(r)=>{
            match parse(&r.message){
                Ok(obj)=>{
                    return Ok(ChannelResponse{
                        name:name.to_string(),
                        data:obj
                    });
                },
                Err(_)=>{
                    return Err("failed-parse_message-to_channelResponse");
                }
            }
        },
        Err(_)=>{
            return Err("failed-send_message");
        }
    }
}
