#![allow(unused_imports)]
use crate::definitions::{
    event_qualifier_hashmap, 
    path_information_hashmap, 
    programming_network_hashmap, 
    simple_event_details_hashmap, 
    zone_test_event_details_hashmap,
    equipment_event_types_hasmap,
    all_other_event_types_hashmap,
    event_definitions_hashmap,
};

use crate::utilities::invalid_sample_events;

// Event Details
fn process_event_details(event_details: &str) -> String {
    
    let mut translated_string = String::new();

    let event_details_category = &event_details[..2];

    if event_qualifier_hashmap().contains_key(event_details_category) {
        translated_string += &event_qualifier(event_details)
    } 
    
    else if path_information_hashmap().contains_key(event_details_category) {
        translated_string += &path_information(event_details)
    }

    else if programming_network_hashmap().contains_key(event_details_category) {
        translated_string += &programming_network(event_details)
    }

    else if simple_event_details_hashmap().contains_key(event_details_category) {

        translated_string += &simple_event_details(event_details)
    }

    else if zone_test_event_details_hashmap().contains_key(event_details_category) {
        translated_string += &zone_test_event_details(event_details)
    }

    else {
        translated_string += &("Unknown Section: ".to_string() + event_details + "\n")
    }

    return translated_string
}

fn event_qualifier(event_details: &str) -> String{
    
    let mut translated_string = String::new();

    let h = event_qualifier_hashmap();
    let event_details = &event_details.replace("\"", "").replace("“", " ");
    
    let detail_type = &event_details[..2];
    let details = &event_details[2..event_details.len()];

    translated_string += &(h.get(detail_type).unwrap().to_string() + ": " + h.get(details).unwrap_or(&details) + "\n");

    translated_string
}

fn path_information(event_details: &str) -> String {
    
    let mut translated_string = String::new();

    let h = path_information_hashmap();

    let event_details = &event_details.replace("\"", "").replace("“", " ");

    if event_details.len() == 6 {
        translated_string += &(h.get(&event_details[..2]).unwrap().to_string() + "\n");

        translated_string += &("Path Number: ".to_string() + &event_details[2..4] +"\n");

        translated_string += &("Communication Type: ".to_string() + &h.get(&event_details[4..5]).unwrap_or(&&event_details[4..5]) + "\n");

        translated_string += &("Path Type: ".to_string() + &h.get(&event_details[5..]).unwrap_or(&&event_details[5..]) + "\n");

    } else {
        translated_string += &("Path Information: ".to_string() + event_details + "\n")
    }

    translated_string
}

fn programming_network(event_details: &str) -> String {
    
    let mut translated_string = String::new();
    
    let h = programming_network_hashmap();

    let event_details = &event_details.replace("\"", "").replace("“", "");
    let event_details = &event_details[2..event_details.len()];

    if event_details.len() == 14 {
    
        translated_string += &("Primary Communication Type: ".to_string() + &h.get(&event_details[..1]).unwrap_or(&&event_details[..1]) + "\n");

        translated_string += &("Secondary Communication Type: ".to_string() + &h.get(&event_details[1..2]).unwrap_or(&&event_details[1..2]) + "\n");
    
        translated_string += &("Network Backup: ".to_string() + &h.get(&event_details[2..3]).unwrap_or(&&event_details[2..3]) + "\n");

        translated_string += &("Sub Code: ".to_string() + &h.get(&event_details[3..4]).unwrap_or(&&event_details[3..4]) + "\n");

        translated_string += &("Retry Time: ".to_string() + &event_details[4..6] + "\n");

        translated_string += &("Check In Time: ".to_string() + &event_details[6..9] + "\n");

        translated_string += &("Fail Time: ".to_string() + &event_details[9..12] + "\n");

        translated_string += &("TCP Enable: ".to_string() + &h.get(&event_details[12..13]).unwrap_or(&&event_details[12..13]) + "\n");

        translated_string += &("Supervised Backup: ".to_string() + &h.get(&event_details[13..]).unwrap_or(&&event_details[13..]) + "\n");

    } else {
        translated_string += &("Programming Network: ".to_string() + event_details + "\n"); 
    }

    translated_string

}

fn simple_event_details(event_details: &str) -> String {

    let mut translated_string = String::new();
    
    let h = simple_event_details_hashmap();

    let detail_type = h.get(&event_details[..2]).unwrap_or(&event_details).to_string();

    let details = event_details[2..event_details.len()].replace("“", " ").replace("\"", " ");

    translated_string += &(detail_type + ": " + &details + "\n");

    translated_string
}

fn zone_test_event_details(event_details: &str) -> String{
    
    let mut details = event_details[2..event_details.len()].replace("“", "").replace("\"", "");

    let mut translated_string = String::from("Test Zone Types: ");

    let h = all_other_event_types_hashmap();

    for _i in 0..100 {
        if details.len() > 1 {
            let zone_pattern = &details[0..2];

            let zone_type = h.get(zone_pattern).unwrap_or(&zone_pattern).to_string();
            translated_string += &(zone_type + " ");
            details = details[2..].to_string();
        }
    }

    translated_string += "\n";

    translated_string
}

// Event Types
fn process_event_type(event_details: &str, event_definition: &str) -> String {
    
    let mut translated_string = String::new();

    let h = 
    if event_definition == "Ze" {
        equipment_event_types_hasmap() 
    } else {
        all_other_event_types_hashmap()
    };

    if event_details.contains("t ") {
        let event_type = event_details.replace("“", "").replace("\"", "");

        let event_type_value = h.get(&event_type[2..]).unwrap_or(&event_details);
        translated_string += &("Event type: ".to_string() + &event_type_value + "\n");
    } else {
        translated_string += &process_event_details(event_details);
    };

    translated_string

} 

// Event Definitions
fn process_event_definition(event_definition:&str) -> String{
    let mut translated_string = String::new();

    let h = event_definitions_hashmap();

    translated_string += h.get(event_definition).unwrap_or(&event_definition);

    translated_string
} 

// Main Translation Function
pub fn translate_dmp_event(event: String) -> String {
    // Section deliters "\", items of length greater than 1 will be retained
    let mut event_splits: Vec<&str> = event.split("\\").collect();
    event_splits.retain(|&i| i.len() > 1);

    // Error if unable to produce at least three sections
    if event_splits.len() < 3 {
        return String::from("Error, event might be missing '\\' delimiters or formatted incorrectly\n")
    };

    let mut translated_event_string = String::new();

    // Sections
    let event_def = event_splits[0];
    let message_length = event_splits[1];
    let remaining_sections = &event_splits[2..];

    // Event Definition
    let translated_event_definition = process_event_definition(event_def);
    translated_event_string += &("Event Definition: ".to_string() + &translated_event_definition + "\n");

    // Message Length
    translated_event_string += &("Message Length: ".to_string() + &message_length + "\n");

    // Remaining Sections
    for section in remaining_sections {
        translated_event_string += &process_event_type(&section, &event_def);
    }

    translated_event_string
}

#[cfg(test)]
mod tests {
    use super::*;

    // Event Detail
    #[test]
    fn test_process_event_details() {
        
        let e_qualifier = "e tt";
        assert_eq!(process_event_details(e_qualifier), event_qualifier(e_qualifier));
        
        let path_info = "cf\"123";
        assert_eq!(process_event_details(path_info), path_information(path_info));

        let programming_network_info = "pn\"123";
        assert_eq!(process_event_details(programming_network_info), programming_network(programming_network_info));
        
        let simple_event = "z \"backdoor";
        assert_eq!(process_event_details(simple_event), simple_event_details(simple_event));

        let zone_test_event = "tz\"FIPNBU";
        assert_eq!(process_event_details(zone_test_event), zone_test_event_details(zone_test_event));
    
        let invalid_event_details = "***";
        assert_eq!(process_event_details(invalid_event_details), "Unknown Section: ".to_string() + invalid_event_details + "\n");
    }

    #[test]
    fn test_event_qualifier() {
        let message_event = "e \"AC";
        let encryption = "ee\"RI";
        let lockdown = "el\"ON";

        assert_eq!(event_qualifier(message_event), "Message: Area Closing, all areas armed\n");
        assert_eq!(event_qualifier(encryption), "Encryption: Rijndael\n");
        assert_eq!(event_qualifier(lockdown), "Lockdown State: In Lockdown\n");
    }

    #[test]
    fn test_path_information() {
        let message = "c 03\"DB";
        let event = "cf07\"LB";

        assert_eq!(path_information(message), 
        "Message communicated on path\n\
        Path Number: 03\n\
        Communication Type: DD\n\
        Path Type: Backup\n"
    );
        
        assert_eq!(path_information(event), 
        "Event occured on path\n\
        Path Number: 07\n\
        Communication Type: Cell\n\
        Path Type: Backup\n"
    );
    }

    #[test]
    fn test_programming_network() {
        let programming_net_event = "pn\"78YY05015030YY";
        assert_eq!(programming_network(programming_net_event), 
        "Primary Communication Type: Net\n\
        Secondary Communication Type: DD\n\
        Network Backup: Yes\n\
        Sub Code: Yes\n\
        Retry Time: 05\n\
        Check In Time: 015\n\
        Fail Time: 030\n\
        TCP Enable: Yes\n\
        Supervised Backup: Yes\n"
    );
    }

    #[test]
    fn test_simple_event_details() {
        // Zone
        let zone = "z 001";
        assert_eq!(simple_event_details(zone), "Zone: 001\n");
        // Area
        let area = "a 001";
        assert_eq!(simple_event_details(area), "Area: 001\n");
        // Device
        let device = "v \"thing";
        let video_device = "vd123";
        assert_eq!(simple_event_details(device), "Device:  thing\n");
        assert_eq!(simple_event_details(video_device), "Device: 123\n");
        // Schedule Name
        let schedule = "n \"12";
        assert_eq!(simple_event_details(schedule), "Schedule:  12\n");
        // Holiday
        let holiday = "h \"12";
        assert_eq!(simple_event_details(holiday), "Holiday number:  12\n");
        // Time/Day
        let open_time = "io\"12";
        let close_time = "ic\"12";
        assert_eq!(simple_event_details(open_time), "Open time (24-hour):  12\n");
        assert_eq!(simple_event_details(close_time), "Close time (24-hour):  12\n");
        // Date
        let date = "d 12-25";
        assert_eq!(simple_event_details(date), "Date (mm-dd): 12-25\n");
        // User Code
        let user_performed_action = "u 00001\"WILLIAM SMITH";
        let action_performed_on_user = "um00002“TOM JONES";
        let panel_audit = "ua00002\"BILL JONES";
        let two_man_rule = "us00002\"BILL JONES";
        assert_eq!(simple_event_details(user_performed_action), "User that performed action: 00001 WILLIAM SMITH\n");
        assert_eq!(simple_event_details(action_performed_on_user), "Action performed on user: 00002 TOM JONES\n");
        assert_eq!(simple_event_details(panel_audit), "Panel audit, no user: 00002 BILL JONES\n");
        assert_eq!(simple_event_details(two_man_rule), "Two man rule: 00002 BILL JONES\n");
        // Service Code
        let service_code = "s \"123";
        let service_code_2 = "sY\"123";
        let service_code_3 = "sN\"123";
        assert_eq!(simple_event_details(service_code), "Service code ID (0-65535):  123\n");
        assert_eq!(simple_event_details(service_code_2), "Service code ID (0-65535):  123\n");
        assert_eq!(simple_event_details(service_code_3), "Service code ID (0-65535):  123\n");
        // Equipment ID
        let equipment_id = "g 1212 1 617 01 002";
        assert_eq!(simple_event_details(equipment_id), "Equipment ID number: 1212 1 617 01 002\n");
        // Call Information
        let ani_dnis = "na\"*8319362#321*";
        let caller_id = "nc\"8006414282";
        assert_eq!(simple_event_details(ani_dnis), "ANI/DNIS:  *8319362#321*\n");
        assert_eq!(simple_event_details(caller_id), "Caller ID:  8006414282\n");
        // Signal Strength
        let signal_strength = "bc\"73";
        assert_eq!(simple_event_details(signal_strength), "Cell Signal Strength:  73\n");
        // Event Qualifier
        let sequence_number = "es123";
        let automatic_recell = "et0024";
        let traffic_count_number = "ec12345";
        let cellular_comm_trouble = "ef1";
        let negative_acks = "en010";
        let protocol = "epDS";
        let previous_mac = "eo000000000000";
        let current_mac = "em000000000000";
        let unrecognized_card = "eu12345";
        let remote_initiation = "er12345";
        let video_alarm = "ev333";
        assert_eq!(simple_event_details(sequence_number), "Sequence number (0-250): 123\n");
        assert_eq!(simple_event_details(automatic_recell), "Automatic recall number of hours (0001-1440): 0024\n");
        assert_eq!(simple_event_details(traffic_count_number), "Traffic Count—number of zone trips: 12345\n");
        assert_eq!(simple_event_details(cellular_comm_trouble), "Cellular communication trouble (1-7): 1\n");
        assert_eq!(simple_event_details(negative_acks), "Negative acknowledgements (NAKs) from host—number received: 010\n");
        assert_eq!(simple_event_details(protocol), "Protocol (DS): DS\n");
        assert_eq!(simple_event_details(previous_mac), "Previous MAC (Control panel substitution): 000000000000\n");
        assert_eq!(simple_event_details(current_mac), "Current MAC (Control panel substitution): 000000000000\n");
        assert_eq!(simple_event_details(unrecognized_card), "Unrecognized Card (Access): 12345\n");
        assert_eq!(simple_event_details(remote_initiation), "Remote initiation: 12345\n");
        assert_eq!(simple_event_details(video_alarm), "Video alarm: 333\n");

    }

    #[test]
    fn test_zone_test_event_details() {
        let test_zones = "tz\"BUPNFI";
        assert_eq!(zone_test_event_details(test_zones), "Test Zone Types: Burglary Panic Fire \n");        
    }    

    // Event Types
    #[test]
    fn test_process_event_type() {
        let event_type = "t “AD";
        assert_eq!(process_event_type(event_type, "Ze"), "Event type: Add\n");
        assert_eq!(process_event_type(event_type, "Za"), "Event type: Added User Code\n");
    }

    // Event Definitions
    #[test]
    fn test_process_event_definition() {
        assert_eq!(process_event_definition("Za"),"Zone Alarm");
        assert_eq!(process_event_definition("Zb"),"Zone Force Alarm");
        assert_eq!(process_event_definition("Zc"),"Real-Time Status");
        assert_eq!(process_event_definition("Zd"), "Wireless Zone Low Battery (LOBAT)");
        assert_eq!(process_event_definition("Ze"), "Equipment (hardware receivers)");
        assert_eq!(process_event_definition("Zf"), "Zone Fail");
        assert_eq!(process_event_definition("Zg"), "Holidays");
        assert_eq!(process_event_definition("Zh"), "Wireless Zone Missing");
        assert_eq!(process_event_definition("Zi"), "Zone Tamper");
        assert_eq!(process_event_definition("Zj"), "Door Access");
        assert_eq!(process_event_definition("Zk"), "Walk Test Zone Verify");
        assert_eq!(process_event_definition("Zl"), "Schedules");
        assert_eq!(process_event_definition("Zm"), "Service Code");
        assert_eq!(process_event_definition("Zp"), "Zone Trip Count");
        assert_eq!(process_event_definition("Zq"), "Arming Status");
        assert_eq!(process_event_definition("Zr"), "Zone Restore");
        assert_eq!(process_event_definition("Zs"), "System Message");
        assert_eq!(process_event_definition("Zt"), "Zone Trouble");
        assert_eq!(process_event_definition("Zu"), "User Codes");
        assert_eq!(process_event_definition("Zw"), "Zone Fault");
        assert_eq!(process_event_definition("Zx"), "Zone Bypass");
        assert_eq!(process_event_definition("Zy"), "Zone Reset");
        assert_eq!(process_event_definition("Zz"), "Reserved Type (System use only)"); 
    }

    // Main Translation Function
    #[test]
    fn test_translate_dmp_event_invalid() {
        let invalid_events = invalid_sample_events();

        for event in invalid_events {
            assert_eq!(translate_dmp_event(event), "Error, event might be missing '\\' delimiters or formatted incorrectly\n");
        }
    }

    #[test]
    fn test_translate_dmp_event_zones() {
        // Zones
        let burglary_alarm = String::from("Za\\060\\t \"BU\\z 001\"EAST OFFICE DOOR\\a 001“EAST WAREHOUSE \\");
        let generic_alarm = String::from("Zr\\060\\t \"BL\\z 001\"EAST OFFICE DOOR\\a 001\"EAST WAREHOUSE \\ee\"RI\\");
        let fire_alarm = String::from("Za\\037\\t \"FI\\z 001\"OFFICE SMOKE DET\\ee\"RI\\");
        let service_message = String::from("Zw\\043\\t \"FI\\z 001\"OFFICE SMOKE DET\\e \"DT\\ee\"NO\\");
        let zone_bypass_by_user = String::from("Zx\\085\\t \"BU\\z 001\"EAST OFFICE DOOR\\u 00001\"WILLIAM SMITH \\a 001\"EAST WAREHOUSE \\");
        let zone_reset_from_bypass_by_user = String::from("Zy\\085\\t \"BU\\z 001\"EAST OFFICE DOOR\\u 00001\"WILLIAM SMITH \\a 001\"EAST WAREHOUSE \\");
        let zone_swinger_bypass = String::from("Zx\\085\\t \"BU\\z 001\"EAST OFFICE DOOR\\u 32765\"SWINGER BYPASS \\a 001\"EAST WAREHOUSE \\ee\"NO\\");
        let zone_trip_count = String::from("Zp\\067\\t \"BU\\z 001\"EAST OFFICE DOOR\\a 001\"EAST WAREHOUSE \\e 00123\\");
        let keypad_tamper_zone_touble = String::from("Zt\\037\\t \"A1\\z 202\"WIRELESS KEYPAD \\");

        assert_eq!(translate_dmp_event(burglary_alarm), 
        "Event Definition: Zone Alarm\nMessage Length: 060\n\
        Event type: Burglary\n\
        Zone: 001 EAST OFFICE DOOR\n\
        Area: 001 EAST WAREHOUSE \n");
        
        assert_eq!(translate_dmp_event(generic_alarm), 
        "Event Definition: Zone Restore\n\
        Message Length: 060\n\
        Event type: Blank\n\
        Zone: 001 EAST OFFICE DOOR\n\
        Area: 001 EAST WAREHOUSE \n\
        Encryption: Rijndael\n");
        
        assert_eq!(translate_dmp_event(fire_alarm), 
        "Event Definition: Zone Alarm\n\
        Message Length: 037\n\
        Event type: Fire\n\
        Zone: 001 OFFICE SMOKE DET\n\
        Encryption: Rijndael\n");
        
        assert_eq!(translate_dmp_event(service_message), 
        "Event Definition: Zone Fault\n\
        Message Length: 043\n\
        Event type: Fire\n\
        Zone: 001 OFFICE SMOKE DET\n\
        Message: Dirty smoke detector\n\
        Encryption: Not encrypted\n");
        
        assert_eq!(translate_dmp_event(zone_bypass_by_user), 
        "Event Definition: Zone Bypass\n\
        Message Length: 085\n\
        Event type: Burglary\n\
        Zone: 001 EAST OFFICE DOOR\n\
        User that performed action: 00001 WILLIAM SMITH \n\
        Area: 001 EAST WAREHOUSE \n");
        
        assert_eq!(translate_dmp_event(zone_reset_from_bypass_by_user), 
        "Event Definition: Zone Reset\n\
        Message Length: 085\n\
        Event type: Burglary\n\
        Zone: 001 EAST OFFICE DOOR\n\
        User that performed action: 00001 WILLIAM SMITH \n\
        Area: 001 EAST WAREHOUSE \n");
        
        assert_eq!(translate_dmp_event(zone_swinger_bypass), 
        "Event Definition: Zone Bypass\n\
        Message Length: 085\n\
        Event type: Burglary\n\
        Zone: 001 EAST OFFICE DOOR\n\
        User that performed action: 32765 SWINGER BYPASS \n\
        Area: 001 EAST WAREHOUSE \n\
        Encryption: Not encrypted\n");
        
        assert_eq!(translate_dmp_event(zone_trip_count), 
        "Event Definition: Zone Trip Count\n\
        Message Length: 067\n\
        Event type: Burglary\n\
        Zone: 001 EAST OFFICE DOOR\n\
        Area: 001 EAST WAREHOUSE \n\
        Message: 00123\n");

        assert_eq!(translate_dmp_event(keypad_tamper_zone_touble), 
        "Event Definition: Zone Trouble\n\
        Message Length: 037\n\
        Event type: Auxillary 1\n\
        Zone: 202 WIRELESS KEYPAD \n");
    }
    
    #[test]
    fn test_translate_dmp_event_door_access() {
        // Door Access
        let door_access = String::from("Zj\\045\\t \"DA\\v 001\\u 00001\"WILLIAM SMITH \\");
        let door_access_device_name = String::from("Zj\\062\\t \"DA\\v 001\"FRONT ENTRANCE \\u 00001\"WILLIAM SMITH \\ee\"NO\\");
        let door_access_denied_device_name = String::from("Zj\\087\\t \"AA\\v 001\"FRONT ENTRANCE \\u 00001\"WILLIAM SMITH \\us00002\"BILL JONES \\ee\"RI\\");

        assert_eq!(translate_dmp_event(door_access), 
        "Event Definition: Door Access\n\
        Message Length: 045\n\
        Event type: Door: Access Granted\n\
        Device: 001\n\
        User that performed action: 00001 WILLIAM SMITH \n");
        
        assert_eq!(translate_dmp_event(door_access_device_name), 
        "Event Definition: Door Access\n\
        Message Length: 062\n\
        Event type: Door: Access Granted\n\
        Device: 001 FRONT ENTRANCE \n\
        User that performed action: 00001 WILLIAM SMITH \n\
        Encryption: Not encrypted\n");
        
        assert_eq!(translate_dmp_event(door_access_denied_device_name), 
        "Event Definition: Door Access\n\
        Message Length: 087\n\
        Event type: Armed Area: Access Denied\n\
        Device: 001 FRONT ENTRANCE \n\
        User that performed action: 00001 WILLIAM SMITH \n\
        Two man rule: 00002 BILL JONES \n\
        Encryption: Rijndael\n");
    }    

    #[test]    
    fn test_translate_dmp_event_schedule_change() {    
        // Schedule Change
        let schedule_change_by_user = String::from("Zl\\063\\t \"PE\\io08:00\"MON\\ic02:30\"TUE\\u 00001\"WILLIAM SMITH \\");
        let shift_one_schedule_change_by_area_by_user = String::from("Zl\\086\\t \"S1\\io08:00\"MON\\ic02:30\"TUE\\a 001\"EAST WAREHOUSE \\u 00001\"WILLIAM SMITH \\");
        let shift_two_holiday_schedule_change_by_area_by_user = String::from("Zl\\086\\t \"S2\\io08:00\"HOL\\ic02:30\"HOL\\a 001\"EAST WAREHOUSE \\u 00001\"WILLIAM SMITH \\ee\"NO\\");
        let secondary_holiday_schedule_change_by_user = String::from("Zl\\062\\t \"SE\\io08:00\"HOL\\ic02:30\"HOL\\u 00001\"WILLIAM SMITH \\ee\"RI\\");
        let shift_four_holiday_a_schedule_change_by_area_by_user = String::from("Zl\\086\\t \"S4\\io08:00\"H-A\\ic02:30\"H-A\\a 001\"EAST WAREHOUSE \\u 00001\"WILLIAM SMITH \\");
        let numeric_schedule_change_by_user = String::from("Zl\\081\\t 07\\n \"PLANT WEEKDAY \\io08:00\"MON\\ic02:30\"TUE\\u 00001\"WILLIAM SMITH \\");

        assert_eq!(translate_dmp_event(schedule_change_by_user), 
        "Event Definition: Schedules\n\
        Message Length: 063\n\
        Event type: Permanent Schedule\n\
        Open time (24-hour): 08:00 MON\n\
        Close time (24-hour): 02:30 TUE\n\
        User that performed action: 00001 WILLIAM SMITH \n");
        
        assert_eq!(translate_dmp_event(shift_one_schedule_change_by_area_by_user), 
        "Event Definition: Schedules\n\
        Message Length: 086\n\
        Event type: Shift One\n\
        Open time (24-hour): 08:00 MON\n\
        Close time (24-hour): 02:30 TUE\n\
        Area: 001 EAST WAREHOUSE \n\
        User that performed action: 00001 WILLIAM SMITH \n");
        
        assert_eq!(translate_dmp_event(shift_two_holiday_schedule_change_by_area_by_user), 
        "Event Definition: Schedules\n\
        Message Length: 086\n\
        Event type: Shift Two\n\
        Open time (24-hour): 08:00 HOL\n\
        Close time (24-hour): 02:30 HOL\n\
        Area: 001 EAST WAREHOUSE \n\
        User that performed action: 00001 WILLIAM SMITH \n\
        Encryption: Not encrypted\n");

        assert_eq!(translate_dmp_event(secondary_holiday_schedule_change_by_user), 
        "Event Definition: Schedules\n\
        Message Length: 062\n\
        Event type: Secondary Schedule\n\
        Open time (24-hour): 08:00 HOL\n\
        Close time (24-hour): 02:30 HOL\n\
        User that performed action: 00001 WILLIAM SMITH \n\
        Encryption: Rijndael\n");
        
        assert_eq!(translate_dmp_event(shift_four_holiday_a_schedule_change_by_area_by_user), 
        "Event Definition: Schedules\n\
        Message Length: 086\n\
        Event type: Shift Four\n\
        Open time (24-hour): 08:00 H-A\n\
        Close time (24-hour): 02:30 H-A\n\
        Area: 001 EAST WAREHOUSE \n\
        User that performed action: 00001 WILLIAM SMITH \n");
        
        assert_eq!(translate_dmp_event(numeric_schedule_change_by_user), 
        "Event Definition: Schedules\n\
        Message Length: 081\n\
        Event type: Numeric Schedule 07\n\
        Schedule:  PLANT WEEKDAY \n\
        Open time (24-hour): 08:00 MON\n\
        Close time (24-hour): 02:30 TUE\n\
        User that performed action: 00001 WILLIAM SMITH \n");
    }

    #[test]
    fn test_translate_dmp_event_opening_closing() {
        // Opening/Closing
        let area_open_using_two_man = String::from("Zq\\085\\t \"OP\\u 00001\"WILLIAM SMITH \\a 001\"EAST WAREHOUSE \\us00002\"BILL JONES \\ee\"RI\\");
        let area_close_automatic_arming = String::from("Zq\\059\\t \"CL\\u 32766\"SCHEDULE \\a 001\"EAST WAREHOUSE \\");
        let area_close_service_user = String::from("Zq\\065\\t \"CL\\u 32767\"SERVICE USER \\a 001\"EAST WAREHOUSE \\ee\"NO\\");
        let area_close_remote_access_user = String::from("Zq\\059\\t \"CL\\u 32764\"REMOTE USER \\a 001\"EAST WAREHOUSE \\");
        let area_close_all_areas_armed_qualifier = String::from("Zq\\071\\t \"CL\\u 00001\"WILLIAM SMITH \\a 001\"EAST WAREHOUSE \\e \"AC\\ee\"RI\\");
        let area_close_traffic_count_qualifier = String::from("Zq\\067\\t \"CL\\u 00001\"WILLIAM SMITH \\a 001\"EAST WAREHOUSE \\ec12345\\");

        assert_eq!(translate_dmp_event(area_open_using_two_man), 
        "Event Definition: Arming Status\n\
        Message Length: 085\n\
        Event type: Disarmed (Opened) Area\n\
        User that performed action: 00001 WILLIAM SMITH \n\
        Area: 001 EAST WAREHOUSE \n\
        Two man rule: 00002 BILL JONES \n\
        Encryption: Rijndael\n");
        
        assert_eq!(translate_dmp_event(area_close_automatic_arming), 
        "Event Definition: Arming Status\nMessage Length: 059\n\
        Event type: Armed (Closed) Area\n\
        User that performed action: 32766 SCHEDULE \n\
        Area: 001 EAST WAREHOUSE \n");
        
        assert_eq!(translate_dmp_event(area_close_service_user), 
        "Event Definition: Arming Status\n\
        Message Length: 065\n\
        Event type: Armed (Closed) Area\n\
        User that performed action: 32767 SERVICE USER \n\
        Area: 001 EAST WAREHOUSE \n\
        Encryption: Not encrypted\n");

        assert_eq!(translate_dmp_event(area_close_remote_access_user), 
        "Event Definition: Arming Status\n\
        Message Length: 059\n\
        Event type: Armed (Closed) Area\n\
        User that performed action: 32764 REMOTE USER \n\
        Area: 001 EAST WAREHOUSE \n");
        
        assert_eq!(translate_dmp_event(area_close_all_areas_armed_qualifier), 
        "Event Definition: Arming Status\n\
        Message Length: 071\n\
        Event type: Armed (Closed) Area\n\
        User that performed action: 00001 WILLIAM SMITH \n\
        Area: 001 EAST WAREHOUSE \n\
        Message: Area Closing, all areas armed\nEncryption: Rijndael\n");
        
        assert_eq!(translate_dmp_event(area_close_traffic_count_qualifier), 
        "Event Definition: Arming Status\n\
        Message Length: 067\nEvent type: Armed (Closed) Area\n\
        User that performed action: 00001 WILLIAM SMITH \n\
        Area: 001 EAST WAREHOUSE \n\
        Traffic Count—number of zone trips: 12345\n");
    }

    #[test]
    fn test_translate_dmp_event_user_codes() {
        // User Codes
        let user_added_by_user = String::from("Zu\\064\\t \"AD\\um00002\"BILL JONES \\u 00001\"WILLIAM SMITH \\");
        let user_changed_by_user = String::from("Zu\\064\\t \"CH\\um00002\"BILL JONES \\u 00001\"WILLIAM SMITH \\ee\"RI\\");
        let user_deleted_by_user = String::from("Zu\\064\\t \"DE\\um00002\"BILL JONES \\u 00001\"WILLIAM SMITH \\");
        let user_inactive = String::from("Zu\\038\\t \"IN\\ua00002\"BILL JONES \\");

        assert_eq!(translate_dmp_event(user_added_by_user), 
        "Event Definition: User Codes\n\
        Message Length: 064\n\
        Event type: Added User Code\n\
        Action performed on user: 00002 BILL JONES \n\
        User that performed action: 00001 WILLIAM SMITH \n");

        assert_eq!(translate_dmp_event(user_changed_by_user), 
        "Event Definition: User Codes\n\
        Message Length: 064\n\
        Event type: Changed User Code\n\
        Action performed on user: 00002 BILL JONES \n\
        User that performed action: 00001 WILLIAM SMITH \n\
        Encryption: Rijndael\n");
        
        assert_eq!(translate_dmp_event(user_deleted_by_user), 
        "Event Definition: User Codes\n\
        Message Length: 064\n\
        Event type: Deleted User Code\n\
        Action performed on user: 00002 BILL JONES \n\
        User that performed action: 00001 WILLIAM SMITH \n");
        
        assert_eq!(translate_dmp_event(user_inactive), 
        "Event Definition: User Codes\n\
        Message Length: 038\n\
        Event type: Inactive User: Access Denied\n\
        Panel audit, no user: 00002 BILL JONES \n");
    }

    #[test]
    fn test_translate_dmp_event_holiday_change() {
        // Holiday Date Change
        let holiday_change_by_user = String::from("Zg\\046\\h 20\\d 12-25\\u 00001\"WILLIAM SMITH \\");
        let holiday_b_of_holiday_20_change_by_user = String::from("Zg\\052\\t \"HB\\h 20\\d 12-25\\u 00001\"WILLIAM SMITH \\ee\"RI\\");

        assert_eq!(translate_dmp_event(holiday_change_by_user), 
        "Event Definition: Holidays\n\
        Message Length: 046\n\
        Holiday number: 20\n\
        Date (mm-dd): 12-25\n\
        User that performed action: 00001 WILLIAM SMITH \n");
        
        assert_eq!(translate_dmp_event(holiday_b_of_holiday_20_change_by_user), 
        "Event Definition: Holidays\n\
        Message Length: 052\n\
        Event type: Holiday B\n\
        Holiday number: 20\n\
        Date (mm-dd): 12-25\n\
        User that performed action: 00001 WILLIAM SMITH \n\
        Encryption: Rijndael\n");
    }

    #[test]
    fn test_translate_dmp_event_equipment_service_codes() {
        // Equipment Messages
        let cellcom_host_panel_had_event_part_one_zone_two = String::from("Ze\\040\\t \"CD\\g 1212 1 617 01 002\\ee\"NO\\");

        assert_eq!(translate_dmp_event(cellcom_host_panel_had_event_part_one_zone_two), 
        "Event Definition: Equipment (hardware receivers)\n\
        Message Length: 040\n\
        Event type: Contact ID\n\
        Equipment ID number: 1212 1 617 01 002\n\
        Encryption: Not encrypted\n");

        // Service Code Messages
        let service_code_start = String::from("Zm\\022\\t \"ST\\sY12345\\");
        let service_code_stop = String::from("Zm\\022\\t \"SP\\sY12345\\");

        assert_eq!(translate_dmp_event(service_code_start), 
        "Event Definition: Service Code\n\
        Message Length: 022\n\
        Event type: Start Service User\nService code ID (0-65535): 12345\n");
        
        assert_eq!(translate_dmp_event(service_code_stop), 
        "Event Definition: Service Code\n\
        Message Length: 022\n\
        Event type: Stop Service User\n\
        Service code ID (0-65535): 12345\n");
    }

    #[test]
    fn test_translate_dmp_event_other_system_msgs() {
        // Other System Messages
        let gen_system_msg = String::from("Zs\\013\\t 001\\pn\"78YY05015030YY\\c 01\"NP\\cf07\"LB\\ee\"RI\\et0024\\");
        let gen_msg_with_user = String::from("Zs\\038\\t 066\\u 00001\"WILLIAM SMITH\\tz\"BUPNFI\\ee\"RI\\");
        let device_system_msg = String::from("Zs\\020\\t 101\\v 100\\ee\"RI\\");
        let service_request_system_msg = String::from("Zs\\020\\t 091\\v 001\"FRONT OFFICE KEYPAD\\u 00001\"FRANKLIN WILLIAM SHEPARD III\\");
        let abort_system_msg = String::from("Zs\\039\\t 150\\u 00001\"WILLIAM SMITH \\");
        let dialer_comm_trouble_msg = String::from("Zs\\020\\t 153\\e 054\\");
        let scs_vr_auto_reject_msg = String::from("Zs\\020\\t 100\\en010\\");
        let unrestored_sys_cid = String::from("Zs\\042\\t 007\\et0002\\c 03\"DB\\nc\"8006414282\\");
        let unrestored_sys_ani_dnis = String::from("Zs\\046\\t 088\\et0002\\c 03\"DB\\na\"*8319362#321*\\");

        assert_eq!(translate_dmp_event(gen_system_msg), 
        "Event Definition: System Message\n\
        Message Length: 013\n\
        Event type: Standby battery restored\n\
        Primary Communication Type: Net\n\
        Secondary Communication Type: DD\n\
        Network Backup: Yes\n\
        Sub Code: Yes\n\
        Retry Time: 05\n\
        Check In Time: 015\n\
        Fail Time: 030\n\
        TCP Enable: Yes\n\
        Supervised Backup: Yes\n\
        Message communicated on path\n\
        Path Number: 01\n\
        Communication Type: Network\n\
        Path Type: Primary\n\
        Event occured on path\n\
        Path Number: 07\n\
        Communication Type: Cell\n\
        Path Type: Backup\n\
        Encryption: Rijndael\n\
        Automatic recall number of hours (0001-1440): 0024\n");

        assert_eq!(translate_dmp_event(gen_msg_with_user), 
        "Event Definition: System Message\n\
        Message Length: 038\n\
        Event type: System TEST mode begin\n\
        User that performed action: 00001 WILLIAM SMITH\n\
        Test Zone Types: Burglary Panic Fire \n\
        Encryption: Rijndael\n");

        assert_eq!(translate_dmp_event(device_system_msg), 
        "Event Definition: System Message\n\
        Message Length: 020\n\
        Event type: Device missing\n\
        Device: 100\n\
        Encryption: Rijndael\n");

        assert_eq!(translate_dmp_event(service_request_system_msg), 
        "Event Definition: System Message\n\
        Message Length: 020\n\
        Event type: ALERT: Service request\n\
        Device: 001 FRONT OFFICE KEYPAD\n\
        User that performed action: 00001 FRANKLIN WILLIAM SHEPARD III\n");

        assert_eq!(translate_dmp_event(abort_system_msg), 
        "Event Definition: System Message\n\
        Message Length: 039\n\
        Event type: Undefined\n\
        User that performed action: 00001 WILLIAM SMITH \n");

        assert_eq!(translate_dmp_event(dialer_comm_trouble_msg), 
        "Event Definition: System Message\n\
        Message Length: 020\n\
        Event type: WARNING: Communication trouble\nMessage: 054\n");

        assert_eq!(translate_dmp_event(scs_vr_auto_reject_msg), 
        "Event Definition: System Message\n\
        Message Length: 020\n\
        Event type: Automation rejected a message\n\
        Negative acknowledgements (NAKs) from host—number received: 010\n");

        assert_eq!(translate_dmp_event(unrestored_sys_cid), 
        "Event Definition: System Message\n\
        Message Length: 042\n\
        Event type: Automatic recall test OK\n\
        Automatic recall number of hours (0001-1440): 0002\n\
        Message communicated on path\nPath Number: 03\n\
        Communication Type: DD\n\
        Path Type: Backup\n\
        Caller ID:  8006414282\n");

        assert_eq!(translate_dmp_event(unrestored_sys_ani_dnis), 
        "Event Definition: System Message\n\
        Message Length: 046\n\
        Event type: Automatic recall OK—Unrestored system\n\
        Automatic recall number of hours (0001-1440): 0002\n\
        Message communicated on path\n\
        Path Number: 03\n\
        Communication Type: DD\n\
        Path Type: Backup\n\
        ANI/DNIS:  *8319362#321*\n");
    }

    #[test]
    fn test_translate_dmp_event_real_time_status() {
        // Real Time 
        let rt_door_open = String::from("Zc\\020\\t \"DO\\z 162\\");
        let rt_door_closed = String::from("Zc\\020\\t \"DC\\z 162\\");
        let rt_door_forced = String::from("Zc\\020\\t \"FO\\z 162\\");

        assert_eq!(translate_dmp_event(rt_door_open), 
        "Event Definition: Real-Time Status\n\
        Message Length: 020\n\
        Event type: Door: Open\n\
        Zone: 162\n");

        assert_eq!(translate_dmp_event(rt_door_closed), 
        "Event Definition: Real-Time Status\n\
        Message Length: 020\n\
        Event type: Door: Closed\n\
        Zone: 162\n");

        assert_eq!(translate_dmp_event(rt_door_forced), 
        "Event Definition: Real-Time Status\n\
        Message Length: 020\n\
        Event type: Door: Forced Open\n\
        Zone: 162\n");

        let rt_output_on = String::from("Zc\\020\\t \"ON\\v 999\\");
        let rt_output_off = String::from("Zc\\020\\t \"OF\\v 999\\");
        let rt_output_pulse = String::from("Zc\\020\\t \"PL\\v 999\\");
        let rt_ouput_temporal_three = String::from("Zc\\020\\t \"TP\\v 999\\");
        let rt_output_momentary = String::from("Zc\\020\\t \"MO\\v 999\\");

        assert_eq!(translate_dmp_event(rt_output_on), 
        "Event Definition: Real-Time Status\n\
        Message Length: 020\n\
        Event type: Output: On\n\
        Device: 999\n");

        assert_eq!(translate_dmp_event(rt_output_off), 
        "Event Definition: Real-Time Status\n\
        Message Length: 020\n\
        Event type: Output: Off\n\
        Device: 999\n");

        assert_eq!(translate_dmp_event(rt_output_pulse), 
        "Event Definition: Real-Time Status\n\
        Message Length: 020\n\
        Event type: Output: Pulse\n\
        Device: 999\n");

        assert_eq!(translate_dmp_event(rt_ouput_temporal_three), 
        "Event Definition: Real-Time Status\n\
        Message Length: 020\n\
        Event type: Output: Temporal\n\
        Device: 999\n");

        assert_eq!(translate_dmp_event(rt_output_momentary), 
        "Event Definition: Real-Time Status\n\
        Message Length: 020\n\
        Event type: Output: Momentary\n\
        Device: 999\n");

        let strike_on = String::from("Zc\\020\\t \"ON\\vd016\\");
        let strike_off = String::from("Zc\\020\\t \"OF\\vd016\\");
    
        assert_eq!(translate_dmp_event(strike_on), 
        "Event Definition: Real-Time Status\n\
        Message Length: 020\n\
        Event type: Output: On\n\
        Device: 016\n");
        
        assert_eq!(translate_dmp_event(strike_off), 
        "Event Definition: Real-Time Status\n\
        Message Length: 020\n\
        Event type: Output: Off\n\
        Device: 016\n");
        
    
    }
}