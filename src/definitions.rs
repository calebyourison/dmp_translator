use std::collections::HashMap;

// Event Details
pub fn event_qualifier_hashmap() -> HashMap<&'static str, &'static str> {
    let event_qualifier_hashmap = HashMap::from([
        ("AC", "Area Closing, all areas armed"),
        ("DT", "Dirty smoke detector"),
        ("LC", "Local Alarm/Restore"),
        ("NO", "Not encrypted"),
        ("OFF", "Not in Lockdown"),
        ("ON", "In Lockdown"),
        ("RH", "256-bit"),
        ("RI", "Rijndael"),

        ("e ", "Message"),
        ("ee", "Encryption"),
        ("el", "Lockdown State"),
    ]);

    event_qualifier_hashmap
}

pub fn path_information_hashmap() -> HashMap<&'static str, &'static str> {
    let path_information_hashmap = HashMap::from([
        ("B", "Backup"),
        ("C", "CID"),
        ("D", "DD"),
        ("L", "Cell"),
        ("N", "Network"),
        ("P", "Primary"),
        ("W", "Wi-Fi"),

        ("c ", "Message communicated on path"),
        ("cf", "Event occured on path"),
    ]);

    path_information_hashmap
}

pub fn programming_network_hashmap() -> HashMap<&'static str, &'static str> {
    
    let programming_network_hashmap = HashMap::from([
        ("0", "None"),
        ("6", "Cell"),
        ("7", "Net"),
        ("8", "DD"),
        ("B", "D2"),
        ("N", "No"),
        ("Y", "Yes"),

        ("pn", "Programming Network"),
    ]);

    programming_network_hashmap
}

pub fn simple_event_details_hashmap() -> HashMap<&'static str, &'static str> {
    
    let simple_event_details: HashMap<&str, &str> = HashMap::from([
        // Zone
        ("z ", "Zone"),
        // Area
        ("a ", "Area"),
        // Device
        ("v ", "Device"),
        ("vd", "Device"),
        // Schedule Name
        ("n ", "Schedule"),
        // Holiday
        ("h ", "Holiday number"),
        // Time/Day
        ("io", "Open time (24-hour)"),
        ("ic", "Close time (24-hour)"),
        // Date
        ("d ", "Date (mm-dd)"),
        // User Code
        ("u ", "User that performed action"),
        ("um", "Action performed on user"),
        ("ua", "Panel audit, no user"),
        ("us", "Two man rule"),
        // Service Code
        ("s ", "Service code ID (0-65535)"),
        ("sY", "Service code ID (0-65535)"),
        ("sN", "Service code ID (0-65535)"),
        // Equipment ID
        ("g ", "Equipment ID number"),
        // Call Information
        ("na", "ANI/DNIS"),
        ("nc", "Caller ID"),
        // Signal Strength
        ("bc", "Cell Signal Strength"),
        // Event Qualifier
        ("es", "Sequence number (0-250)"),
        ("et", "Automatic recall number of hours (0001-1440)"),
        ("ec", "Traffic Count—number of zone trips"),
        ("ef", "Cellular communication trouble (1-7)"),
        ("en", "Negative acknowledgements (NAKs) from host—number received"),
        ("ep", "Protocol (DS)"),
        ("eo", "Previous MAC (Control panel substitution)"),
        ("em", "Current MAC (Control panel substitution)"),
        ("eu", "Unrecognized Card (Access)"),
        ("er", "Remote initiation"),
        ("ev", "Video alarm"),
    ]);

    simple_event_details
}

pub fn zone_test_event_details_hashmap() -> HashMap<&'static str, &'static str> {
    
    let zone_test_event_details = HashMap::from([
        // Test Zones
        ("tz", "Test Zone Types"),
    ]);

    zone_test_event_details
}

// Event Types
pub fn equipment_event_types_hasmap() -> HashMap<&'static str, &'static str> {
    
    let equipment_event_types = HashMap::from([
        ("RP", "Repair"),
        ("RL", "Replace"),
        ("AD", "Add"),
        ("RM", "Remove"),
        ("AJ", "Adjust"),
        ("TS", "Test"),
        ("SO", "System Options: EEPROM"),
        ("PR", "Printer: EEPROM"),
        ("LC", "Line Card: EEPROM"),
        ("H1", "Host Port 1: EEPROM"),
        ("H2", "Host Port 2: EEPROM"),
        ("SP", "Serial Port: EEPROM"),
        ("LG", "Log"),
        ("EE", "Entire EEPROM"),
        ("CD", "Contact ID"),

    ]);

    equipment_event_types
}

pub fn all_other_event_types_hashmap() -> HashMap<&'static str, &'static str> {
    
    let all_other_event_types = HashMap::from([
    // Zones
    ("BL", "Blank"),
    ("FI", "Fire"),
    ("BU", "Burglary"),
    ("SV", "Supervisory"),
    ("PN", "Panic"),
    ("EM", "Emergency"),
    ("A1", "Auxillary 1"),
    ("A2", "Auxillary 2"),
    ("CO", "Carbon Monoxide"),
    ("VA", "Video Alarm"), 
    // Arming
    ("OP", "Disarmed (Opened) Area"),
    ("CL", "Armed (Closed) Area"),
    ("LA", "Area Late to Arm"),  
    // User Codes
    ("AD", "Added User Code"),
    ("CH", "Changed User Code"),
    ("DE", "Deleted User Code"),
    ("IN", "Inactive User Code"),
    // Schedule
    ("PE", "Permanent Schedule"),
    ("TE", "Temporary Schedule"),
    ("PR", "Primary Schedule"),
    ("SE", "Secondary Schedule"),
    ("S1", "Shift One"),
    ("S2", "Shift Two"),
    ("S3", "Shift Three"),
    ("S4", "Shift Four"), 
    // Numeric Schedule
    ("00", "Numeric Schedule 00"),
    ("01", "Numeric Schedule 01"),
    ("02", "Numeric Schedule 02"),
    ("03", "Numeric Schedule 03"),
    ("04", "Numeric Schedule 04"),
    ("05", "Numeric Schedule 05"),
    ("06", "Numeric Schedule 06"),
    ("07", "Numeric Schedule 07"),
    ("08", "Numeric Schedule 08"),
    ("09", "Numeric Schedule 09"),
    ("10", "Numeric Schedule 10"),
    ("11", "Numeric Schedule 11"),
    ("12", "Numeric Schedule 12"),
    ("13", "Numeric Schedule 13"),
    ("14", "Numeric Schedule 14"),
    ("15", "Numeric Schedule 15"),
    ("16", "Numeric Schedule 16"),
    ("17", "Numeric Schedule 17"),
    ("18", "Numeric Schedule 18"),
    ("19", "Numeric Schedule 19"),
    ("20", "Numeric Schedule 20"),
    ("21", "Numeric Schedule 21"),
    ("22", "Numeric Schedule 22"),
    ("23", "Numeric Schedule 23"),
    ("24", "Numeric Schedule 24"),
    ("25", "Numeric Schedule 25"),
    ("26", "Numeric Schedule 26"),
    ("27", "Numeric Schedule 27"),
    ("28", "Numeric Schedule 28"),
    ("29", "Numeric Schedule 29"),
    ("30", "Numeric Schedule 30"),
    ("31", "Numeric Schedule 31"),
    ("32", "Numeric Schedule 32"),
    ("33", "Numeric Schedule 33"),
    ("34", "Numeric Schedule 34"),
    ("35", "Numeric Schedule 35"),
    ("36", "Numeric Schedule 36"),
    ("37", "Numeric Schedule 37"),
    ("38", "Numeric Schedule 38"),
    ("39", "Numeric Schedule 39"),
    ("40", "Numeric Schedule 40"),
    ("41", "Numeric Schedule 41"),
    ("42", "Numeric Schedule 42"),
    ("43", "Numeric Schedule 43"),
    ("44", "Numeric Schedule 44"),
    ("45", "Numeric Schedule 45"),
    ("46", "Numeric Schedule 46"),
    ("47", "Numeric Schedule 47"),
    ("48", "Numeric Schedule 48"),
    ("49", "Numeric Schedule 49"),
    ("50", "Numeric Schedule 50"),
    ("51", "Numeric Schedule 51"),
    ("52", "Numeric Schedule 52"),
    ("53", "Numeric Schedule 53"),
    ("54", "Numeric Schedule 54"),
    ("55", "Numeric Schedule 55"),
    ("56", "Numeric Schedule 56"),
    ("57", "Numeric Schedule 57"),
    ("58", "Numeric Schedule 58"),
    ("59", "Numeric Schedule 59"),
    ("60", "Numeric Schedule 60"),
    ("61", "Numeric Schedule 61"),
    ("62", "Numeric Schedule 62"),
    ("63", "Numeric Schedule 63"),
    ("64", "Numeric Schedule 64"),
    ("65", "Numeric Schedule 65"),
    ("66", "Numeric Schedule 66"),
    ("67", "Numeric Schedule 67"),
    ("68", "Numeric Schedule 68"),
    ("69", "Numeric Schedule 69"),
    ("70", "Numeric Schedule 70"),
    ("71", "Numeric Schedule 71"),
    ("72", "Numeric Schedule 72"),
    ("73", "Numeric Schedule 73"),
    ("74", "Numeric Schedule 74"),
    ("75", "Numeric Schedule 75"),
    ("76", "Numeric Schedule 76"),
    ("77", "Numeric Schedule 77"),
    ("78", "Numeric Schedule 78"),
    ("79", "Numeric Schedule 79"),
    ("80", "Numeric Schedule 80"),
    ("81", "Numeric Schedule 81"),
    ("82", "Numeric Schedule 82"),
    ("83", "Numeric Schedule 83"),
    ("84", "Numeric Schedule 84"),
    ("85", "Numeric Schedule 85"),
    ("86", "Numeric Schedule 86"),
    ("87", "Numeric Schedule 87"),
    ("88", "Numeric Schedule 88"),
    ("89", "Numeric Schedule 89"),
    ("90", "Numeric Schedule 90"),
    ("91", "Numeric Schedule 91"),
    ("92", "Numeric Schedule 92"),
    ("93", "Numeric Schedule 93"),
    ("94", "Numeric Schedule 94"),
    ("95", "Numeric Schedule 95"),
    ("96", "Numeric Schedule 96"),
    ("97", "Numeric Schedule 97"),
    ("98", "Numeric Schedule 98"),
    ("99", "Numeric Schedule 99"), 
    // Holiday
    ("HA", "Holiday A"),
    ("HB", "Holiday B"),
    ("HC", "Holiday C"), 
    // Access
    ("DA", "Door: Access Granted"),
    ("AA", "Armed Area: Access Denied"),
    ("IA", "Invalid Area: Access Denied"),
    ("IT", "Invalid Time: Access Denied"),
    ("AP", "Anti-passback: Access Denied"),
    ("IC", "Invalid Code: Access Denied"),
    ("IL", "Invalid Level: Access Denied"),
    ("WP", "Wrong Pin: Access Denied"),
    ("IN", "Inactive User: Access Denied"), 
    // Real-Time Status
    ("DO", "Door: Open"),
    ("DC", "Door: Closed"),
    ("HO", "Door: Held Open"),
    ("FO", "Door: Forced Open"),
    ("ON", "Output: On"),
    ("OF", "Output: Off"),
    ("PL", "Output: Pulse"),
    ("TP", "Output: Temporal"),
    ("MO", "Output: Momentary"), 
    // Serivce User
    ("ST", "Start Service User"),
    ("SP", "Stop Service User"), //148
    // System Message
    ("000", "AC power restored"),
    ("001", "Standby battery restored"),
    ("002", "Communication line level restored"),
    ("003", "Panel tamper restored"),
    ("004", "Backup communication line restored"),
    ("005", "Panel ground restored"),
    ("006", "Late to close"),
    ("007", "Automatic recall test OK"),
    ("008", "WARNING: AC power failure"),
    ("009", "WARNING: Low standby battery"),
    ("010", "WARNING: Low communication line level"),
    ("011", "WARNING: Panel tamper"),
    ("012", "WARNING: Panel backup communication fail"),
    ("013", "WARNING: Panel ground fault"),
    ("014", "WARNING: Non-alarm message overflow"),
    ("015", "* * AMBUSH * *"),
    ("016", "WARNING: Panel not responding"),
    ("017", "Panel response restored"),
    ("018", "ALARM: Zone alarm overflow"),
    ("019", "WARNING: New panel on line"),
    ("020", "Call xxxxxxxxxx"),
    ("021", "Automation not responding"),
    ("022", "Automation restored"),
    ("023", "Panel test signal received"),
    ("024", "Undefined"),
    ("025", "Undefined"),
    ("026", "WARNING: Auxiliary fuse trouble"),
    ("027", "Auxiliary fuse restored"),
    ("028", "WARNING: Telephone line 1 trouble"),
    ("029", "Telephone line 1 restored"),
    ("030", "WARNING: Telephone line 2 trouble"),
    ("031", "Telephone line 2 restored"),
    ("032", "ALARM: Supervised wireless interference"),
    ("033", "ALARM: Early morning ambush"),
    ("034", "WARNING: Alarm bell silenced"),
    ("035", "Alarm bell returned to normal"),
    ("036", "Time and date set by operator"),
    ("037", "Undefined"),
    ("038", "WARNING: Bell circuit trouble"),
    ("039", "Bell circuit restored"),
    ("040", "ALARM: Fire zone alarm overflow"),
    ("041", "ALARM: Panic zone alarm overflow"),
    ("042", "ALARM: Burglary zone alarm overflow"),
    ("043", "WARNING: Bell fuse trouble"),
    ("044", "WARNING: Fire-burglary trouble overflow"),
    ("045", "Abort signal received"),
    ("046", "Zone swinger automatically bypassed"),
    ("047", "Zone swinger automatically reset"),
    ("048", "WARNING: Low battery cutoff—LAST MESSAGE"),
    ("049", "CANCEL signal received"),
    ("050", "WARNING: Supervised wireless trouble"),
    ("051", "WARNING: Remote programming"),
    ("052", "Undefined"),
    ("053", "Bell fuse restored"),
    ("054", "WARNING: Unsuccessful remote attempt"),
    ("055", "Request for alarm receiver key"),
    ("056", "Undefined"),
    ("057", "Undefined"),
    ("058", "ALARM: Control panel substitution"),
    ("059", "WARNING: Substitution/Checkin overflow"),
    ("060", "WARNING: Invalid panel message format"),
    ("061", "Undefined"),
    ("062", "Undefined"),
    ("063", "Undefined"),
    ("064", "Undefined"),
    ("065", "Undefined"),
    ("066", "System TEST mode begin"),
    ("067", "System TEST mode end"),
    ("068", "Printer fail"),
    ("069", "Printer restore"),
    ("070", "End of history buffer"),
    ("071", "Date time requested from control"),
    ("072", "WARNING: Network or communication path trouble"),
    ("073", "Network or communication path restored"),
    ("074", "ALARM: Tamper during armed state"),
    ("075", "ALERT: Early to close"),
    ("076", "ALERT: Late to open"),
    ("077", "ALERT: Unauthorized entry"),
    ("078", "ALERT: System recently armed"),
    ("079", "ALERT: Signal during opened period"),
    ("080", "ALERT: Exit error"),
    ("081", "WARNING: Network card trouble—Card c"),
    ("082", "Network card restored—Card c"),
    ("083", "Remote programming complete"),
    ("084", "Remote command received"),
    ("085", "Undefined"),
    ("086", "WARNING: Local programming in progress"),
    ("087", "WARNING: Transmit failed—Messages not sent"),
    ("088", "Automatic recall OK—Unrestored system"),
    ("089", "Supervised wireless restored"),
    ("090", "WARNING: Unrecognized message"),
    ("091", "ALERT: Service request"),
    ("092", "WARNING: No arm/disarm activity"),
    ("093", "ALARM: User activity not detected"),
    ("094", "ALERT: Activity check enabled"),
    ("095", "ALERT: Activity check disabled"),
    ("096", "ALARM: Verify signal received"),
    ("097", "Network communication test OK"),
    ("098", "SCS1 memory full"),
    ("099", "15 minute system check"),
    ("100", "Automation rejected a message"),
    ("101", "Device missing"),
    ("102", "Device restored"),
    ("103", "Undefined"),
    ("104", "Undefined"),
    ("105", "Undefined"),
    ("106", "Undefined"),
    ("107", "Undefined"),
    ("108", "Undefined"),
    ("109", "Panel restart"),
    ("110", "Auxiliary power fail"),
    ("111", "Auxiliary power restore"),
    ("112", "DC power fail"),
    ("113", "DC power restore"),
    ("114", "Undefined"),
    ("115", "Undefined"),
    ("116", "On-demand monitoring started"),
    ("117", "On-demand monitoring stopped"),
    ("118", "System On-Test"),
    ("119", "System Off-Test"),
    ("120", "Undefined"),
    ("121", "ALERT: Cell data communication excessive"),
    ("122", "WARNING: Cell data non-alarm suppress"),
    ("123", "ALARM: Cell data fire alarm suppress"),
    ("124", "ALARM: Cell data non-fire alarm suppress"),
    ("125", "Cell data communication restored"),
    ("126", "ALERT: Cell rate plan exceeded"),
    ("127", "Undefined"),
    ("128", "Undefined"),
    ("129", "Undefined"),
    ("130", "WARNING: Cell communicator bus failed"),
    ("131", "ALARM: Cell communicator bus failed"),
    ("132", "Cell communicator bus restored"),
    ("133", "WARNING: Cell communicator DC failed"),
    ("134", "Cell communicator DC restored"),
    ("135", "WARNING: Cell communicator low battery"),
    ("136", "Cell communicator battery restored"),
    ("137", "WARNING: Cell communicator tamper"),
    ("138", "ALARM: Cell communicator tamper"),
    ("139", "Cell communicator tamper restored"),
    ("140", "TROUBLE OVERRIDE"),
    ("141", "Undefined"),
    ("142", "Undefined"),
    ("143", "Undefined"),
    ("144", "Undefined"),
    ("145", "Undefined"),
    ("146", "Undefined"),
    ("147", "Undefined"),
    ("148", "Undefined"),
    ("149", "Undefined"),
    ("150", "Undefined"),
    ("151", "WARNING: Memory usage"),
    ("152", "Undefined"),
    ("153", "WARNING: Communication trouble"),
    ("154", "WARNING: Database failure (Line card failure for hardware receivers)"),
    ("155", "WARNING: Comm line"),
    ("156", "Undefined"),
    ("157", "Undefined"),
    ("158", "Undefined"),
    ("159", "Undefined"),
    ("160", "Undefined"),
    ("172", "Undefined"),
    ("173", "Undefined"),            
    
    ]);
        
    all_other_event_types

}

// Event Definitions
pub fn event_definitions_hashmap() -> HashMap<&'static str, &'static str> {
    
    let event_definitions_hashmap = HashMap::from([
        ("Za", "Zone Alarm"),
        ("Zb", "Zone Force Alarm"),
        ("Zc", "Real-Time Status"),
        ("Zd", "Wireless Zone Low Battery (LOBAT)"),
        ("Ze", "Equipment (hardware receivers)"),
        ("Zf", "Zone Fail"),
        ("Zg", "Holidays"),
        ("Zh", "Wireless Zone Missing"),
        ("Zi", "Zone Tamper"),
        ("Zj", "Door Access"),
        ("Zk", "Walk Test Zone Verify"),
        ("Zl", "Schedules"),
        ("Zm", "Service Code"),
        ("Zp", "Zone Trip Count"),
        ("Zq", "Arming Status"),
        ("Zr", "Zone Restore"),
        ("Zs", "System Message"),
        ("Zt", "Zone Trouble"),
        ("Zu", "User Codes"),
        ("Zw", "Zone Fault"),
        ("Zx", "Zone Bypass"),
        ("Zy", "Zone Reset"),
        ("Zz", "Reserved Type (System use only)"),
    ]);

    event_definitions_hashmap
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmaps_for_proper_length() {
        // Event Details
        assert_eq!(event_qualifier_hashmap().len(), 11);
        assert_eq!(path_information_hashmap().len(), 9);
        assert_eq!(programming_network_hashmap().len(), 8);
        assert_eq!(simple_event_details_hashmap().len(), 31);
        assert_eq!(zone_test_event_details_hashmap().len(), 1);

        // Event Types
        assert_eq!(equipment_event_types_hasmap().len(), 15);
        assert_eq!(all_other_event_types_hashmap().len(), 310);

        // Event Definitions
        assert_eq!(event_definitions_hashmap().len(), 23);
    }
}