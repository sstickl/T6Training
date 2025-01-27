// STRUCTS
/*pub struct EmergencyProcedure {
    pub title: String,
    pub steps: Vec<String>,
}*/ // DONT NEED IT

//Credit JDemler from StackOverflow for this macro
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