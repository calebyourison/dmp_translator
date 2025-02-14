
pub fn valid_sample_events() -> Vec<String> {

    let valid_sample_events = vec![
        
        // Sample Events
        String::from("Za\\060\\t \"BU\\z 001“OFFICE DOOR \\a 001“WAREHOUSE \\"),
        String::from("Za\\032\\t “VA\\z 001\\a 002\\ev333\\"),
        String::from("Zq\\059\\t “OP\\u 00001“LINDA SANCHEZ \\a 001“OFFICE \\"),
        String::from("Zu\\064\\t “AD\\um00002“TOM JONES \\u 00001“JORDAN JONES \\"),
        String::from("Zs\\014\\t 017"),
        
        // Zones
        String::from("Za\\060\\t \"BU\\z 001\"EAST OFFICE DOOR\\a 001“EAST WAREHOUSE \\"),
        String::from("Zr\\060\\t \"BL\\z 001\"EAST OFFICE DOOR\\a 001\"EAST WAREHOUSE \\ee\"RI\\"),
        String::from("Za\\037\\t \"FI\\z 001\"OFFICE SMOKE DET\\ee\"RI\\"),
        String::from("Zw\\043\\t \"FI\\z 001\"OFFICE SMOKE DET\\e \"DT\\ee\"NO\\"),
        String::from("Zx\\085\\t \"BU\\z 001\"EAST OFFICE DOOR\\u 00001\"WILLIAM SMITH \\a 001\"EAST WAREHOUSE \\"),
        String::from("Zy\\085\\t \"BU\\z 001\"EAST OFFICE DOOR\\u 00001\"WILLIAM SMITH \\a 001\"EAST WAREHOUSE \\"),
        String::from("Zx\\085\\t \"BU\\z 001\"EAST OFFICE DOOR\\u 32765\"SWINGER BYPASS \\a 001\"EAST WAREHOUSE \\ee\"NO\\"),
        String::from("Zp\\067\\t \"BU\\z 001\"EAST OFFICE DOOR\\a 001\"EAST WAREHOUSE \\e 00123\\"),
        String::from("Zt\\037\\t \"A1\\z 202\"WIRELESS KEYPAD \\"),
        
        // Door Access
        String::from("Zj\\045\\t \"DA\\v 001\\u 00001\"WILLIAM SMITH \\"),
        String::from("Zj\\062\\t \"DA\\v 001\"FRONT ENTRANCE \\u 00001\"WILLIAM SMITH \\ee\"NO\\"),
        String::from("Zj\\087\\t \"AA\\v 001\"FRONT ENTRANCE \\u 00001\"WILLIAM SMITH \\us00002\"BILL JONES \\ee\"RI\\"),
        
        // Schedule Change
        String::from("Zl\\063\\t \"PE\\io08:00\"MON\\ic02:30\"TUE\\u 00001\"WILLIAM SMITH \\"),
        String::from("Zl\\086\\t \"S1\\io08:00\"MON\\ic02:30\"TUE\\a 001\"EAST WAREHOUSE \\u 00001\"WILLIAM SMITH \\"),
        String::from("Zl\\086\\t \"S2\\io08:00\"HOL\\ic02:30\"HOL\\a 001\"EAST WAREHOUSE \\u 00001\"WILLIAM SMITH \\ee\"NO\\"),
        String::from("Zl\\062\\t \"SE\\io08:00\"HOL\\ic02:30\"HOL\\u 00001\"WILLIAM SMITH \\ee\"RI\\"),
        String::from("Zl\\086\\t \"S4\\io08:00\"H-A\\ic02:30\"H-A\\a 001\"EAST WAREHOUSE \\u 00001\"WILLIAM SMITH \\"),
        String::from("Zl\\081\\t 07\\n \"PLANT WEEKDAY \\io08:00\"MON\\ic02:30\"TUE\\u 00001\"WILLIAM SMITH \\"),
        
        // Opening/Closing
        String::from("Zq\\085\\t \"OP\\u 00001\"WILLIAM SMITH \\a 001\"EAST WAREHOUSE \\us00002\"BILL JONES \\ee\"RI\\"),
        String::from("Zq\\059\\t \"CL\\u 32766\"SCHEDULE \\a 001\"EAST WAREHOUSE \\"),
        String::from("Zq\\065\\t \"CL\\u 32767\"SERVICE USER \\a 001\"EAST WAREHOUSE \\ee\"NO\\"),
        String::from("Zq\\059\\t \"CL\\u 32764\"REMOTE USER \\a 001\"EAST WAREHOUSE \\"),
        String::from("Zq\\071\\t \"CL\\u 00001\"WILLIAM SMITH \\a 001\"EAST WAREHOUSE \\e \"AC\\ee\"RI\\"),
        String::from("Zq\\067\\t \"CL\\u 00001\"WILLIAM SMITH \\a 001\"EAST WAREHOUSE \\ec12345\\"),
        
        // User Codes
        String::from("Zu\\064\\t \"AD\\um00002\"BILL JONES \\u 00001\"WILLIAM SMITH \\"),
        String::from("Zu\\064\\t \"CH\\um00002\"BILL JONES \\u 00001\"WILLIAM SMITH \\ee\"RI\\"),
        String::from("Zu\\064\\t \"DE\\um00002\"BILL JONES \\u 00001\"WILLIAM SMITH \\"),
        String::from("Zu\\038\\t \"IN\\ua00002\"BILL JONES \\"),
        
        // Holiday Date Change
        String::from("Zg\\046\\h 20\\d 12-25\\u 00001\"WILLIAM SMITH \\"),
        String::from("Zg\\052\\t \"HB\\h 20\\d 12-25\\u 00001\"WILLIAM SMITH \\ee\"RI\\"),
        
        // Equipment Messages
        String::from("Ze\\040\\t \"CD\\g 1212 1 617 01 002\\ee\"NO\\"),
        
        // Service Code Messages
        String::from("Zm\\022\\t \"ST\\sY12345\\"),
        String::from("Zm\\022\\t \"SP\\sY12345\\"),
        
        // Other System Messages
        String::from("Zs\\013\\t 001\\pn\"78YY05015030YY\\c 01\"NP\\cf07\"LB\\ee\"RI\\et0024\\"),
        String::from("Zs\\038\\t 066\\u 00001\"WILLIAM SMITH\\tz\"BUPNFI\\ee\"RI\\"),
        String::from("Zs\\020\\t 101\\v 100\\ee\"RI\\"),
        String::from("Zs\\020\\t 091\\v 001\"FRONT OFFICE KEYPAD\\u 00001\"FRANKLIN WILLIAM SHEPARD III\\"),
        String::from("Zs\\039\\t 150\\u 00001\"WILLIAM SMITH \\"),
        String::from("Zs\\020\\t 153\\e 054\\"),
        String::from("Zs\\020\\t 100\\en010\\"),
        String::from("Zs\\042\\t 007\\et0002\\c 03\"DB\\nc\"8006414282\\"),
        String::from("Zs\\046\\t 088\\et0002\\c 03\"DB\\na\"*8319362#321*\\"),
        
        // Real Time 
        String::from("Zc\\020\\t \"DO\\z 162\\"),
        String::from("Zc\\020\\t \"DC\\z 162\\"),
        String::from("Zc\\020\\t \"FO\\z 162\\"),
        
        String::from("Zc\\020\\t \"ON\\v 999\\"),
        String::from("Zc\\020\\t \"OF\\v 999\\"),
        String::from("Zc\\020\\t \"PL\\v 999\\"),
        String::from("Zc\\020\\t \"TP\\v 999\\"),
        String::from("Zc\\020\\t \"MO\\v 999\\"),
        
        String::from("Zc\\020\\t \"ON\\vd016\\"),
        String::from("Zc\\020\\t \"OF\\vd016\\"),
    ];

    valid_sample_events
}

pub fn invalid_sample_events() -> Vec<String> {
    let invalid_sample_events = vec![
        String::from("Zab\\"),
        String::from("\\pt\\t  a\\e"),
        String::from("\\a\\aa\\a\\"),
    ];

    invalid_sample_events
}
