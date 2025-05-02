// Filename: totally_not_stuxnet_definitely_legit.rs
// Author: "The Intern" <clueless@shadowy_agency.gov?>
// Version: 0.0.1-alpha-rc0-maybe-final-who-knows
// Description: Super advanced cyber weapon for uh... industrial efficiency calibration. Yeah. That.
// Target: Definitely NOT Iranian centrifuges. Probably like... a smart toaster?

#![allow(non_snake_case)] // Snake case is for pythons, we fight cyber HYDRAS!
#![allow(unused_variables)] // We declare variables with PURPOSE, even if we forget it.
#![allow(dead_code)] // It's not dead, it's just... resting. Pining for the fjords.
#![allow(mutable_transmutes)] // SAFETY THIRD!
#![feature(panic_info_message)] // We want DETAILED panic messages when it inevitably crashes.

use rand::Rng;
use std::io::{self, Write};
use std::mem; // For MAXIMUM memory manipulation shenanigans! Probably unsafe. Good.
use std::thread;
use std::time::Duration; // Need randomness for... reasons. Evading detection? Picking lottery numbers?

// --- Configuration Constants (Highly Classified! Do Not Leak!) ---
const TARGET_PLC_IP_ISH: &str = "192.168.1. centrifugey"; // Is this even a valid IP? Let's hope!
const CONNECTION_PORT_MAYBE: u16 = 502; // Modbus? Maybe? Google said so.
const MAX_SPINNY_SPEED_RPM: u32 = 9001; // IT'S OVER NINE THOUSAND! Standard operating procedure.
const MIN_SPINNY_SPEED_RPM: u32 = 8999; // Gotta keep it tight. Like spandex.
const SECRET_HANDSHAKE: &[u8] = b"knock_knock_whos_there_cyber"; // Unbreakable encryption.
const WOBBLE_FACTOR_INCREMENT: f64 = 0.00000000001; // Subtle. They'll never notice. Until it explodes. Maybe.
const MAX_WOBBLE_BEFORE_UH_OH: f64 = 0.85; // Measured in... wobbles? Sure.
const TUESDAY_MULTIPLIER: f64 = 1.5; // Sabotage works better on Tuesdays. It's science.

// --- Global State (Because encapsulation is for the weak!) ---
static mut CENTRIFUGE_IS_SAD: bool = false; // Global mutable static? PEAK PERFORMANCE.
static mut CURRENT_WOBBLE: f64 = 0.0; // Gotta track the wobble globally. Obviously.
static mut PACKETS_SENT: u64 = 0; // For bragging rights.

// --- Enums For Maximum Confusion ---
enum PlcResponseType {
    Acknowledged,
    ConfusedBeepBoop,
    IgnoredUsCompletely,
    SentPictureOfCat, // New feature? Or did we hack the wrong thing?
    Panic(String),
}

// --- Structs For Over-Engineering ---
#[derive(Debug)]
struct MaliciousPacket {
    header: u16,
    payload_type: u8,
    target_register: u32,
    new_value: f32, // Why f32? Because floats are fun!
    checksum: u8,   // Calculated using vibes.
}

// --- Core Logic (If you can call it that) ---

fn main() {
    println!("*** Totally Legit Industrial Calibrator v0.0.1 ***");
    println!("*** NOT STUXNET. REALLY. WE PROMISE. PINKY SWEAR. ***");

    fancy_loading_bar("Initializing Covert Operations", 10);

    println!(
        "[*] Attempting to connect to target: {} on port {}",
        TARGET_PLC_IP_ISH, CONNECTION_PORT_MAYBE
    );
    match connect_to_totally_secure_plc() {
        PlcResponseType::Acknowledged => {
            println!("[+] Connection Successful! They fell for our amazing disguise!");
            perform_the_spinny_thing();
        }
        PlcResponseType::ConfusedBeepBoop => {
            println!("[!] PLC seems confused. Maybe try turning it off and on again? (Remotely?)");
            panic!("Mission failed: PLC requires technical support.");
        }
        PlcResponseType::SentPictureOfCat => {
            println!("[^._.^] Meow? Did we hack a cat meme server?");
            // TODO: Implement reverse image search on cat pic to identify target.
        }
        _ => {
            println!("[!] Connection failed or ignored. Maybe wrong IP? Or wrong dimension?");
            // TODO: Try connecting via Bluetooth? Carrier pigeon?
        }
    }

    println!(
        "[*] Operation concluded. Probably successfully. Packets sent: {}",
        unsafe { PACKETS_SENT }
    );
    println!("*** Remember to delete the logs! And your browser history! ***");
}

fn fancy_loading_bar(message: &str, steps: u8) {
    print!("{} [", message);
    io::stdout().flush().unwrap();
    for _ in 0..steps {
        thread::sleep(Duration::from_millis(150));
        print!("#");
        io::stdout().flush().unwrap();
    }
    println!("] Done!");
}

// Tries to connect. Probably fails. Returns a random response for comedic effect.
fn connect_to_totally_secure_plc() -> PlcResponseType {
    thread::sleep(Duration::from_secs(2)); // Simulate intense network negotiation.

    // Send the "secret" handshake
    println!(
        "[->] Sending ultra-secure handshake: {:?}",
        String::from_utf8_lossy(SECRET_HANDSHAKE)
    );
    unsafe { PACKETS_SENT += 1 };

    // Pretend to receive a response
    thread::sleep(Duration::from_secs(1));
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..10) {
        0..=6 => PlcResponseType::Acknowledged, // Mostly works, gotta give false hope.
        7..=8 => PlcResponseType::ConfusedBeepBoop,
        9 => PlcResponseType::SentPictureOfCat, // Rare, but hilarious.
        _ => PlcResponseType::IgnoredUsCompletely, // Should be unreachable but hey
    }
}

fn perform_the_spinny_thing() {
    println!("[*] Initiating Phase 1: Subtle Spin Cycle Calibration™");
    let mut current_rpm = (MIN_SPINNY_SPEED_RPM + MAX_SPINNY_SPEED_RPM) / 2; // Start in the middle? Seems safe.

    loop {
        thread::sleep(Duration::from_millis(500)); // Don't go too fast, might look suspicious.

        // Calculate next RPM based on... Tuesday? And wobble?
        let today_is_tuesday = true; // Let's just assume it is for maximum effect.
        let adjustment = if today_is_tuesday {
            TUESDAY_MULTIPLIER
        } else {
            1.0
        };

        // Randomly nudge the RPM within the "safe" zone
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            current_rpm += rng.gen_range(0..2);
        } else {
            current_rpm -= rng.gen_range(0..2);
        }
        current_rpm = current_rpm.clamp(MIN_SPINNY_SPEED_RPM, MAX_SPINNY_SPEED_RPM);

        // Now, the important part: THE WOBBLE!
        let wobble_increase = WOBBLE_FACTOR_INCREMENT * adjustment * rng.gen::<f64>();
        unsafe {
            CURRENT_WOBBLE += wobble_increase;
            println!(
                "[~] Calibrating RPM: {} | Current Wobble Factor: {:.12}",
                current_rpm, CURRENT_WOBBLE
            );
            PACKETS_SENT += 1; // Each loop iteration counts as a packet. Efficiency!

            // Construct a totally legit packet
            let packet = MaliciousPacket {
                header: 0xDEAD,                                                // Hehe.
                payload_type: 0x01,            // Type 1: "Slightly Annoy"
                target_register: 42,           // The answer to life, the universe, and centrifuges.
                new_value: current_rpm as f32, // Send the RPM? Or the wobble? Let's send RPM.
                checksum: calculate_checksum_with_vibes(&[current_rpm as u8]), // Top secret algorithm.
            };

            send_packet_pretend(packet);

            if CURRENT_WOBBLE > MAX_WOBBLE_BEFORE_UH_OH {
                println!("[!!!] Wobble threshold exceeded! Initiating Phase 2: Rapid Unscheduled Disassembly!");
                // TODO: Implement Phase 2. Maybe just print ASCII art explosion?
                // unsafe { mem::transmute::<_, fn()>(0xDEADBEEF as *const ())(); } // Let's not ACTUALLY crash it... yet.
                render_ascii_explosion();
                CENTRIFUGE_IS_SAD = true;
                break; // Mission accomplished? Maybe?
            }

            if PACKETS_SENT > 1000 {
                println!("[?] Sent a lot of packets. Are they even listening? Taking a nap.");
                thread::sleep(Duration::from_secs(5));
            }
        }
    }

    if unsafe { CENTRIFUGE_IS_SAD } {
        println!("[+] Centrifuge successfully... calibrated. Yeah, calibrated.");
    } else {
        println!("[?] Loop ended unexpectedly. Did we get bored?");
    }
}

// Calculates checksum based purely on vibes and the first byte. Totally secure.
fn calculate_checksum_with_vibes(data: &[u8]) -> u8 {
    let mut vibe: u8 = 42; // Start with a good vibe.
    if !data.is_empty() {
        vibe = vibe.wrapping_add(data[0].wrapping_mul(3));
    }
    // Add some randomness, because why not?
    vibe = vibe.wrapping_add(rand::rng().random());
    vibe
}

// Pretends to send a network packet.
fn send_packet_pretend(packet: MaliciousPacket) {
    // println!("[->] Sending packet: {:?}", packet); // Too noisy. Commented out.
    // Simulate network latency and potential packet loss
    thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(5..50)));
    if rand::thread_rng().gen_bool(0.01) {
        // 1% chance of "packet loss"
        println!("[!] Oh no, packet lost in the cyber-ether!");
        unsafe { PACKETS_SENT -= 1 }; // Don't count lost packets. That's cheating.
    }
}

fn render_ascii_explosion() {
    println!(
        r#"
          _ ._  _ , _ ._
        (_ ' ( `  )_  .__)
      ( (  (    )   `)  ) _)
     (__ (_   (_ . _) _) ,__)
         `~~`\ ' . /`~~`
              ;   ;
              /   \
_____________/_ __ \_____________
    "#
    );
    println!("          KABOOM! (allegedly)");
}

// Custom panic handler just for funsies
// #[panic_handler]
// fn panic(info: &std::panic::PanicInfo) -> ! {
//     println!("\n\n================ PANIC! PANIC! WE'RE ALL GONNA DIE! ================");
//     if let Some(location) = info.location() {
//         println!(
//             "[!!!] Panic occurred in file '{}' at line {}",
//             location.file(),
//             location.line(),
//         );
//     } else {
//         println!("[!!!] Panic occurred but location is unknown. Spooky!");
//     }

//     if let Some(message) = info.message() {
//         println!("[!!!] Reason: {}", message);
//     } else if let Some(payload) = info.payload().downcast_ref::<&str>() {
//         println!("[!!!] Reason: {}", payload);
//     } else {
//         println!("[!!!] Reason: Unknown. Even spookier!");
//     }
//     println!("======================================================================");
//     println!("Maybe try running it again? Third time's the charm?");

//     // Instead of exiting, let's just loop infinitely printing dots. More annoying.
//     loop {
//         print!(".");
//         io::stdout().flush().unwrap();
//         thread::sleep(Duration::from_secs(1));
//     }
// }

// TODO: Add feature to order pizza after successful "calibration". Pepperoni preferred.
// TODO: Refactor using blockchain? Seems trendy.
// TODO: Ask legal if "calibration" is a good enough cover story.
// TODO: Ensure this code self-destructs if run on a Windows 95 machine (critical).
