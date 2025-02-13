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
        engine_ops.insert("N1".to_string(), vec_of_strings![
            "Idle _$60_ to _$61_% Ground, _$67%_ (Min) Flight"]);
        engine_ops.insert("Np".to_string(), vec_of_strings![
            "Idle: _$46_ to _$50_%",
            "Takeoff / Max: _$100_%, (_$100_% ± _$2_% PMU Off)",
            "Avoid stabilized ground operations from _$62_ to _$80_% Np"]);
        engine_ops.insert("Oil Pressure".to_string(), vec_of_strings![
            "Takeoff / Max: _$90_ to _$120_PSI",
            "Aerobatics / Spins: _$40_ to _$130_PSI",
            "Aerobatics / Spins (Idle): _$15_ to _$40_PSI ( _$5_ ) Sec"]);
        engine_ops.insert("Oil Temp".to_string(), vec_of_strings![
            "Takeoff / Max: _$10_ to _$105_°C",
            "Transient: _$106_ to _$110_°C ( _$10_ ) Min"]);
        engine_ops.insert("Maximum Fuel Flow".to_string(), vec_of_strings![
            "All phases of flight _$799_ PPH"]);
        engine_ops
    });

    opdata_db.insert("Prohibited Manuevers".to_string(), {
        let mut prohibited_manuevers = HashMap::new();
        prohibited_manuevers.insert("Prohibited Manuevers".to_string(), vec_of_strings![
            "1. _$Inverted_ Stalls",
            "2. _$Inverted_ Spins",
            "3. Aggravated _$spins past 2 turns_",
            "4. Spins with the PCL _$above idle_",
            "5. Spins with the _$landing gear_, _$flaps_, or _$speed brake_ extended",
            "6. Spins with the _$PMU off",
            "7. Spins below _$10,000_ feet pressure altitude",
            "8. Spins above _$22,000_ feet pressure altitude",
            "9. Abrupt _$cross-controlled (snap)_ maneuvers",
            "10. Aerobatic maneuvers, spins, or stalls with greater than _$50_ pounds fuel imbalance",
            "11. _$Tail_ spins."]);
        prohibited_manuevers
    });

    opdata_db.insert("Airspeed Limitations".to_string(), {
        let mut airspeed_limits = HashMap::new();
        airspeed_limits.insert("Airspeed Limitations".to_string(), vec_of_strings![
            "Max Airspeed Gear and/or Flaps _$150_ KIAS",
            "Max Operating Speed _$316_ KIAS or _$0.67_ Mach",
            "Full rudder deflection above _$150_ KIAS will exceed the limits of the rubber control system."]);
        airspeed_limits
    });

    return opdata_db
}