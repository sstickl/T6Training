// STRUCTS
/*pub struct EmergencyProcedure {
    pub title: String,
    pub steps: Vec<String>,
}*/ // DONT NEED IT
use std::collections::HashMap;

//Credit JDemler from StackOverflow for this macro - you da real MVP
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

pub fn init_boldface_db() -> Vec<Vec<String>> {
    /* FORMAT FOR EMERGENCY OPS
    [x][0] = TITLE OF OP
    [x][1..y] = PROCEDURAL STEP */
    let mut emerg_ops: Vec<Vec<String>> = Vec::new();

    emerg_ops.push(vec_of_strings![
        "Emergency Engine Shutdown on the Ground",
        "PCL - OFF",
        "FIREWALL SHUTOFF HANDLE - PULL"]);

    emerg_ops.push(vec_of_strings![
        "Abort",
        "PCL - IDLE",
        "BRAKES - AS REQUIRED"]);

    emerg_ops.push(vec_of_strings![
        "Engine Failure Immediately After Takeoff (Sufficient Runway Remaining Straight Ahead)",
        "AIRSPEED - 110 KNOTS (MINIMUM)",
        "PCL - AS REQUIRED",
        "EMER LDG GR HANDLE - PULL (AS REQUIRED)"]);

    emerg_ops.push(vec_of_strings![
        "Engine Failure During Flight",
        "ZOOM/GLIDE - 125 KNOTS",
        "PCL - OFF",
        "INTERCEPT ELP"]);

    emerg_ops.push(vec_of_strings![
        "Immediate Airstart (PMU NORM)",
        "PCL - OFF",
        "STARTER SWITCH - AUTO/RESET",
        "PCL - IDLE, ABOVE 13% N1"]);

    emerg_ops.push(vec_of_strings![
        "Uncommanded Power Changes / Loss of Power / Uncommanded Propeller Feather",
        "PCL - MID RANGE",
        "PMU SWITCH - OFF",
        "PROP SYS CIRCUIT BREAKER (left front console) - PULL, IF Np STABLE BELOW 40%"]);

    emerg_ops.push(vec_of_strings![
        "Inadvertent Departure From Controlled Flight",
        "PCL - IDLE",
        "CONTROLS - NEUTRAL",
        "ALTITUDE - CHECK"]);

    emerg_ops.push(vec_of_strings![
        "Fire In Flight/If Fire is Confirmed:",
        "PCL - OFF",
        "FIREWALL SHUTOFF HANDLE - PULL"]);

    emerg_ops.push(vec_of_strings![
        "OBOGS Failure / Overtemp / Physiological Symptoms",
        "GREEN RING - PULL (AS REQUIRED)",
        "DESCENT BELOW 10,000 FEET MSL - INITIATE",
        "OBOGS SUPPLY LEVER - OFF"]);
    
    emerg_ops.push(vec_of_strings![
        "Eject",
        "EJECTION HANDLE - PULL"]);

    return emerg_ops
}

pub fn init_bf_opdata_db() -> HashMap<String, HashMap<String, Vec<String>>> {
    let mut opdata_db: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    opdata_db.insert("Engine".to_string(), {
        let mut engine_ops = HashMap::new();
        engine_ops.insert("Maximum Torque".to_string(), vec_of_strings![
            "Takeoff / Max: _$100_%",
            "Transient: _$132_% ( _$20_ ) seconds",
            "Torque above _$102_% is indicative of a system malfunction"]);
        engine_ops.insert("Maximum ITT".to_string(), vec_of_strings![
            "Idle: _$750_°C",
            "Takeoff / Max: _$820_°C",
            "Transient:  _$821_ to _$870_°C ( _$20_ ) seconds"]);
        engine_ops
    });
    return opdata_db
}