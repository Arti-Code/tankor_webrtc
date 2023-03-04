//mod motor;

//use std::env::args;
use anyhow::Result;
use webrtc::ice_transport::ice_credential_type::RTCIceCredentialType;
use std::sync::Arc;
use std::time::Duration;
use std::env;
use tokio::time::sleep;
use tokio::net::{UdpSocket};
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::{MediaEngine, MIME_TYPE_VP8};
use webrtc::api::APIBuilder;
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::data_channel::RTCDataChannel;
use webrtc::ice_transport::ice_connection_state::RTCIceConnectionState;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::rtp_transceiver::rtp_codec::RTCRtpCodecCapability;
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
use webrtc::track::track_local::{TrackLocal, TrackLocalWriter};
use webrtc::Error;
//use webrtc::rtp_transceiver::rtp_sender;
//use webrtc::api::media_engine::{MediaEngine, MIME_TYPE_H264};
//use webrtc::track::track_local::track_local_static_sample::TrackLocalStaticSample;
use serde::{Serialize, Deserialize};
use firebase_rs::*;
use base64::{Engine as _, engine::{general_purpose}};
//use rppal::gpio::*;
//use crate::motor::*;

//const DB_REF: &str = "https://rtp-to-webrtc-default-rtdb.firebaseio.com";
const DB_REF: &str = "https://my-killer-bot-default-rtdb.europe-west1.firebasedatabase.app";
const DEVICE: &str = "tankor";

#[derive(Serialize, Deserialize, Debug)]
struct Answer {
  answer: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Offer {
    offer: String
}
#[derive(Serialize, Deserialize, Debug)]
struct Device {
    name: String
}

impl Device {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() {
    loop {
        run().unwrap();
    }
}

#[tokio::main]
async fn run() -> Result<()> {
    //let args_list = env::args().collect();
    //let params = Params::new(&args_list);
    prog_intro().await;
    let sock = UdpSocket::bind("0.0.0.0:5004").await.unwrap();
    let listener = Arc::new(sock);

    let device: Device = create_device(DEVICE).await;
    let mut m = MediaEngine::default();
    m.register_default_codecs()?;
    let mut registry = Registry::new();
    registry = register_default_interceptors(registry, &mut m)?;
    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .build();
    restart_info(&device).await; 
    let config = RTCConfiguration {
        /* ice_servers: vec![RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            ..Default::default()
        }], */

        ice_servers: vec![RTCIceServer {
            urls: vec!["stun:fr-turn1.xirsys.com".to_owned()],
            username: "23Xgr3XVCOk2GqoZW5eWhbdXM1EfA8VcC6OVVacJSpFdoljTUOsTcgAoFUvfN4vcAAAAAGNFN29nd296ZHlrMg==".to_owned(),
            credential: "2ed490ce-4947-11ed-bd3d-0242ac120004".to_owned(),
            credential_type: RTCIceCredentialType::default(),
        }],
        ..Default::default()
    };

    let peer_connection = Arc::new(api.new_peer_connection(config).await?);
    let video_track = Arc::new(TrackLocalStaticRTP::new(
        RTCRtpCodecCapability {
            mime_type: MIME_TYPE_VP8.to_owned(),
            ..Default::default()}, 
        "robot_cam".to_owned(), 
        "robot_stream".to_owned(), 
    ));
    
    let rtp_sender = peer_connection
        .add_track(Arc::clone(&video_track) as Arc<dyn TrackLocal + Send + Sync>).await?;
    
    let rtp_sender2 = rtp_sender.clone();
    tokio::spawn(async move {
        let mut rtcp_buf = vec![0u8; 4096];
        while let Ok((_, _)) = rtp_sender.read(&mut rtcp_buf).await {}
        Result::<()>::Ok(())
    });

    let (done_tx, mut done_rx) = tokio::sync::mpsc::channel::<()>(5);
    let done_tx1 = done_tx.clone();
    let (_data_tx, mut _data_rx) = tokio::sync::mpsc::channel::<()>(5);
    let (drive_tx, mut drive_rx) = tokio::sync::mpsc::channel::<&str>(10);
    peer_connection
    .on_data_channel(Box::new(move |d: Arc<RTCDataChannel>| {
        let drive_tx1 = drive_tx.clone();
        let d_label = d.label().to_owned();
        Box::pin(async move {
            let _d2 = Arc::clone(&d);
            let d_label2 = d_label.clone();
            d.on_open(Box::new(move || {
                println!("[DATACHANNEL] OPEN: {}", d_label2);
                Box::pin(async move {})
            }));
            
            d.on_message(Box::new(move |msg: DataChannelMessage| {
                    let msg_str = String::from_utf8(msg.data.to_vec()).unwrap();
                    let drive_tx2 = drive_tx1.clone();
                    println!("[↓]: {}", msg_str);
                    if msg_str=="front" {
                        _ = drive_tx2.try_send("front");
                    }
                    else if msg_str=="back" {
                        //println!("sending");
                        _ = drive_tx2.try_send("back");
                    }
                    else if msg_str=="left" {
                        //println!("sending");
                        _ = drive_tx2.try_send("left");
                    }
                    else if msg_str=="right" {
                        //println!("sending");
                        _ = drive_tx2.try_send("right");
                    }
                    else if msg_str=="stop" {
                        //println!("sending");
                        _ = drive_tx2.try_send("stop");
                    }
                    Box::pin(async {})
                }));
            })
        }));

    tokio::spawn(async move {
        //let mut motors = new_motors();
        //prepare(&mut motors);
        while let Some(cmd) = drive_rx.recv().await {
            println!("{}", cmd);
            //if cmd=="front" {
            //    front(&mut motors);
            //} else if cmd=="stop" {
            //    stop(&mut motors);
            //} else if cmd=="back" {
            //    back(&mut motors);
            //} else if cmd=="left" {
            //    left(&mut motors);
            //} else if cmd=="right" {
            //    right(&mut motors);
            //}
        }
        //finish(&mut motors);
        //println!("FINISH MOTORS");
    });

    peer_connection
        .on_ice_connection_state_change(Box::new(move |connection_state: RTCIceConnectionState| {
            println!("[ICE CONNECTION]: {}", connection_state);
            if connection_state == RTCIceConnectionState::Disconnected {
                let _ = done_tx1.try_send(());
            }
            if connection_state == RTCIceConnectionState::Failed {
                let _ = done_tx1.try_send(());
            }
            Box::pin(async {})
        })
    );
    
        let done_tx2 = done_tx.clone();
    
        peer_connection
            .on_peer_connection_state_change(Box::new(move |s: RTCPeerConnectionState| {
                println!("[PEER CONNECTION]: {}", s);
                if s == RTCPeerConnectionState::Failed {
                    let _ = done_tx2.try_send(());
                }
                Box::pin(async {})
            })
        );
        
        
        let offer_encoded = wait_offer(device.get_name()).await;
        let desc_data = decode(&offer_encoded);
        let offer = serde_json::from_str::<RTCSessionDescription>(&desc_data)?;
        peer_connection.set_remote_description(offer).await?;
        let answer = peer_connection.create_answer(None).await?;
    
        let mut gather_complete = peer_connection.gathering_complete_promise().await;
        peer_connection.set_local_description(answer).await?;
        let _ = gather_complete.recv().await;
    
        if let Some(local_desc) = peer_connection.local_description().await {
            send_answer(&local_desc, device.get_name()).await;
        } else {
            println!("[ANSWER]: failed!");
        }
        let done_tx3 = done_tx.clone();
        
        let l = Arc::clone(&listener);
        tokio::spawn(async move {
            let mut inbound_rtp_packet = vec![0u8; 4096]; // UDP MTU
            while let Ok((n, _)) = l.recv_from(&mut inbound_rtp_packet).await {
                if let Err(err) = video_track.write(&inbound_rtp_packet[..n]).await {
                    if Error::ErrClosedPipe == err {
                    } else {
                        println!("[VIDEO]: error: {}", err);
                    }
                    let _ = done_tx3.try_send(());
                    return;
                }
            }
        });
    
        tokio::select! {
            _ = done_rx.recv() => {
                println!("[SIGNAL]: done");
            }
            _ = tokio::signal::ctrl_c() => {
                println!("");
            }
        };

        //let trans = peer_connection.get_transceivers().await;
        //let mut i = 0;
        //for _ in trans.iter() {
            //println!("transceivers: {}", i);
            //i += 1;
        //}
        peer_connection.remove_track(&rtp_sender2).await.unwrap();
        peer_connection.close().await?;
        //finish(engines);
        println!("[CONNECTION] close");
    Ok(())
}

async fn wait(seconds: u32) {
    sleep(Duration::from_secs(seconds as u64)).await;
}

async fn prog_intro() {
    let app_name = env!("CARGO_PKG_NAME").to_uppercase();
    let author = env!("CARGO_PKG_AUTHORS");
    let version = env!("CARGO_PKG_VERSION");
    println!("*** {} ***", app_name);
    wait(1).await;
    println!("version {}", version);
    wait(1).await;
    println!("made by {} ®2023", author);
}

async fn create_device(name: &str) -> Device {
    let device: Device = Device::new(name);
    device
}

async fn restart_info(device: &Device) {
    println!("[DEVICE]: {}", device.get_name().to_uppercase());
    wait(1).await;
}

async fn wait_offer(device: &str) -> String {
    let firebase = Firebase::new(DB_REF)
        .unwrap().at("signaling").at(&device).at("offer");
    let mut offer_founded: bool=false;
    let mut offer_b64: String=String::new();
    println!("WAITING FOR CONNECTION...");
    sleep(Duration::from_secs(1)).await;
    
    while !offer_founded {
        let encod = firebase.get::<String>().await;
        match encod  {
            Ok(v) if v != "" => {
                offer_b64 = v;
                offer_founded = true;
                let firebase2 = Firebase::new(DB_REF)
                    .unwrap().at("signaling").at(&device);
                let clear_offer: Offer=Offer { offer: "".to_string() };
                firebase2.update(&clear_offer).await.unwrap();
            },
            Ok(_) => {
                sleep(Duration::from_secs(1)).await;
            },
            Err(_) => {
                sleep(Duration::from_secs(3)).await;
            }
        }
    }
    println!("[OFFER]: Ok");
    offer_b64
}

async fn send_answer(answer: &RTCSessionDescription, device: &str) {
    let json_str = serde_json::to_string(answer).unwrap();
    let b64 = encode(&json_str);
    let firebase = Firebase::new(DB_REF)
        .unwrap().at("signaling").at(device);
    let ans: Answer=Answer { answer: b64 };
    firebase.update(&ans).await.unwrap();
}

fn encode(b: &str) -> String {
    let encoded = general_purpose::STANDARD.encode(b);
    encoded
}

fn decode(s: &str) -> String {
    let b64 = general_purpose::STANDARD.decode(s).unwrap();
    let decoded = String::from_utf8(b64).unwrap();
    decoded
}