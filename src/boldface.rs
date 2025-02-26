use std::collections::HashMap;

/// Credit JDemler from StackOverflow for this macro - you da real MVP
/// https://stackoverflow.com/questions/28392008/how-to-create-a-vector-with-variable-number-of-elements
/// Creates a vector of strings from a list of strings
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

/// Returns a vector of vector of strings that contains the boldface emergency procedures
/// Format:
/// [x][0] = TITLE OF OP
/// [x][1..y] = PROCEDURAL STEP
pub fn init_boldface_db() -> Vec<Vec<String>> {
    let emerg_ops: Vec<Vec<String>> = vec![
        vec_of_strings![
            "Emergency Engine Shutdown on the Ground",
            "PCL - OFF",
            "FIREWALL SHUTOFF HANDLE - PULL"
        ],
        vec_of_strings!["Abort", "PCL - IDLE", "BRAKES - AS REQUIRED"],
        vec_of_strings![
            "Engine Failure Immediately After Takeoff (Sufficient Runway Remaining Straight Ahead)",
            "AIRSPEED - 110 KNOTS (MINIMUM)",
            "PCL - AS REQUIRED",
            "EMER LDG GR HANDLE - PULL (AS REQUIRED)"
        ],
        vec_of_strings![
            "Engine Failure During Flight",
            "ZOOM/GLIDE - 125 KNOTS",
            "PCL - OFF",
            "INTERCEPT ELP"
        ],
        vec_of_strings![
            "Immediate Airstart (PMU NORM)",
            "PCL - OFF",
            "STARTER SWITCH - AUTO/RESET",
            "PCL - IDLE, ABOVE 13% N1"
        ],
        vec_of_strings![
            "Uncommanded Power Changes / Loss of Power / Uncommanded Propeller Feather",
            "PCL - MID RANGE",
            "PMU SWITCH - OFF",
            "PROP SYS CIRCUIT BREAKER (left front console) - PULL, IF Np STABLE BELOW 40%"
        ],
        vec_of_strings![
            "Inadvertent Departure From Controlled Flight",
            "PCL - IDLE",
            "CONTROLS - NEUTRAL",
            "ALTITUDE - CHECK"
        ],
        vec_of_strings![
            "Fire In Flight/If Fire is Confirmed",
            "PCL - OFF",
            "FIREWALL SHUTOFF HANDLE - PULL"
        ],
        vec_of_strings![
            "OBOGS Failure / Overtemp / Physiological Symptoms",
            "GREEN RING - PULL (AS REQUIRED)",
            "DESCENT BELOW 10,000 FEET MSL - INITIATE",
            "OBOGS SUPPLY LEVER - OFF"
        ],
        vec_of_strings!["Eject", "EJECTION HANDLE - PULL"],
    ];
    emerg_ops
}

/// Holds the different boldface procedure types
#[derive(PartialEq, Copy, Clone)]
pub enum BfProcedureEnum {
    EmergencyEngineShutdown,
    Abort,
    EngineFailureTakeoff,
    EngineFailureFlight,
    ImmediateAirstart,
    UncommandedPower,
    InadvertentDeparture,
    FireInFlight,
    OBOGSFailure,
    Eject,
}

/// Holds the different boldface procedure types
impl BfProcedureEnum {
    pub fn iterator() -> std::slice::Iter<'static, BfProcedureEnum> {
        static PROCEDURES: [BfProcedureEnum; 10] = [
            BfProcedureEnum::EmergencyEngineShutdown,
            BfProcedureEnum::Abort,
            BfProcedureEnum::EngineFailureTakeoff,
            BfProcedureEnum::EngineFailureFlight,
            BfProcedureEnum::ImmediateAirstart,
            BfProcedureEnum::UncommandedPower,
            BfProcedureEnum::InadvertentDeparture,
            BfProcedureEnum::FireInFlight,
            BfProcedureEnum::OBOGSFailure,
            BfProcedureEnum::Eject,
        ];
        PROCEDURES.iter()
    }

    //pub fn as_str(&self) -> &'static str {
    //    match self {
    //        BfProcedureEnum::EmergencyEngineShutdown => "Emergency Engine Shutdown on the Ground",
    //        BfProcedureEnum::Abort => "Abort",
    //        BfProcedureEnum::EngineFailureTakeoff => "Engine Failure Immediately After Takeoff (Sufficient Runway Remaining Straight Ahead)",
    //        BfProcedureEnum::EngineFailureFlight => "Engine Failure During Flight",
    //        BfProcedureEnum::ImmediateAirstart => "Immediate Airstart (PMU NORM)",
    //        BfProcedureEnum::UncommandedPower => "Uncommanded Power Changes / Loss of Power / Uncommanded Propeller Feather",
    //        BfProcedureEnum::InadvertentDeparture => "Inadvertent Departure from Controlled Flight",
    //        BfProcedureEnum::FireInFlight => "Fire In Flight/If Fire is Confirmed",
    //        BfProcedureEnum::OBOGSFailure => "OBOGS Failure/Overtemp/Physiological Symptoms",
    //        BfProcedureEnum::Eject => "Eject",
    //    }
    //}

    /// Returns a short string representation of the BfProcedureEnum
    pub fn as_short_str(&self) -> &'static str {
        match self {
            BfProcedureEnum::EmergencyEngineShutdown => "Emergency Engine Ground Shutdown",
            BfProcedureEnum::Abort => "Abort",
            BfProcedureEnum::EngineFailureTakeoff => {
                "Engine Failure Takeoff (Sufficient Runway Ahead)"
            }
            BfProcedureEnum::EngineFailureFlight => "Engine Failure During Flight",
            BfProcedureEnum::ImmediateAirstart => "Immediate Airstart (PMU NORM)",
            BfProcedureEnum::UncommandedPower => "Uncommanded Power Change/Loss / Prop Feather",
            BfProcedureEnum::InadvertentDeparture => "Inadvertent Departure from Controlled Flight",
            BfProcedureEnum::FireInFlight => "Fire In Flight/If Fire is Confirmed",
            BfProcedureEnum::OBOGSFailure => "OBOGS Failure/Overtemp/Physiological Symptoms",
            BfProcedureEnum::Eject => "Eject",
        }
    }

    /// Returns a usize representation of the BfProcedureEnum compared to the Boldface DB
    pub fn get_id(&self) -> usize {
        match self {
            BfProcedureEnum::EmergencyEngineShutdown => 0,
            BfProcedureEnum::Abort => 1,
            BfProcedureEnum::EngineFailureTakeoff => 2,
            BfProcedureEnum::EngineFailureFlight => 3,
            BfProcedureEnum::ImmediateAirstart => 4,
            BfProcedureEnum::UncommandedPower => 5,
            BfProcedureEnum::InadvertentDeparture => 6,
            BfProcedureEnum::FireInFlight => 7,
            BfProcedureEnum::OBOGSFailure => 8,
            BfProcedureEnum::Eject => 9,
        }
    }

    //pub fn next(&mut self) {
    //    *self = match self {
    //        BfProcedureEnum::EmergencyEngineShutdown => BfProcedureEnum::Abort,
    //        BfProcedureEnum::Abort => BfProcedureEnum::EngineFailureTakeoff,
    //        BfProcedureEnum::EngineFailureTakeoff => BfProcedureEnum::EngineFailureFlight,
    //        BfProcedureEnum::EngineFailureFlight => BfProcedureEnum::ImmediateAirstart,
    //        BfProcedureEnum::ImmediateAirstart => BfProcedureEnum::UncommandedPower,
    //        BfProcedureEnum::UncommandedPower => BfProcedureEnum::InadvertentDeparture,
    //        BfProcedureEnum::InadvertentDeparture => BfProcedureEnum::FireInFlight,
    //        BfProcedureEnum::FireInFlight => BfProcedureEnum::OBOGSFailure,
    //        BfProcedureEnum::OBOGSFailure => BfProcedureEnum::Eject,
    //        BfProcedureEnum::Eject => BfProcedureEnum::EmergencyEngineShutdown,
    //    }
    //}

    //pub fn prev(&mut self) {
    //    *self = match self {
    //        BfProcedureEnum::EmergencyEngineShutdown => BfProcedureEnum::Eject,
    //        BfProcedureEnum::Abort => BfProcedureEnum::EmergencyEngineShutdown,
    //        BfProcedureEnum::EngineFailureTakeoff => BfProcedureEnum::Abort,
    //        BfProcedureEnum::EngineFailureFlight => BfProcedureEnum::EngineFailureTakeoff,
    //        BfProcedureEnum::ImmediateAirstart => BfProcedureEnum::EngineFailureFlight,
    //        BfProcedureEnum::UncommandedPower => BfProcedureEnum::ImmediateAirstart,
    //        BfProcedureEnum::InadvertentDeparture => BfProcedureEnum::UncommandedPower,
    //        BfProcedureEnum::FireInFlight => BfProcedureEnum::InadvertentDeparture,
    //        BfProcedureEnum::OBOGSFailure => BfProcedureEnum::FireInFlight,
    //        BfProcedureEnum::Eject => BfProcedureEnum::OBOGSFailure,
    //    }
    //}
}

/// Returns a hashmap of hashmaps of vectors of strings that contains the boldface operational data
pub fn init_bf_opdata_db() -> HashMap<String, HashMap<String, Vec<String>>> {
    let mut opdata_db: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    opdata_db.insert("Engine".to_string(), {
        let mut engine_ops = HashMap::new();
        engine_ops.insert(
            "Maximum Torque".to_string(),
            vec_of_strings![
                "Takeoff / Max: _$100_%",
                "Transient: _$132_% ( _$20_ ) seconds",
                "Torque above _$102_% is indicative of a system malfunction"
            ],
        );
        engine_ops.insert(
            "Maximum ITT".to_string(),
            vec_of_strings![
                "Idle: _$750_°C",
                "Takeoff / Max: _$820_°C",
                "Transient:  _$821_ to _$870_°C ( _$20_ ) seconds"
            ],
        );
        engine_ops.insert(
            "N1".to_string(),
            vec_of_strings!["Idle _$60_ to _$61_% Ground, _$67%_ (Min) Flight"],
        );
        engine_ops.insert(
            "Np".to_string(),
            vec_of_strings![
                "Idle: _$46_ to _$50_%",
                "Takeoff / Max: _$100_%, (_$100_% ± _$2_% PMU Off)",
                "Avoid stabilized ground operations from _$62_ to _$80_% Np"
            ],
        );
        engine_ops.insert(
            "Oil Pressure".to_string(),
            vec_of_strings![
                "Takeoff / Max: _$90_ to _$120_PSI",
                "Aerobatics / Spins: _$40_ to _$130_PSI",
                "Aerobatics / Spins (Idle): _$15_ to _$40_PSI ( _$5_ ) Sec"
            ],
        );
        engine_ops.insert(
            "Oil Temp".to_string(),
            vec_of_strings![
                "Takeoff / Max: _$10_ to _$105_°C",
                "Transient: _$106_ to _$110_°C ( _$10_ ) Min"
            ],
        );
        engine_ops.insert(
            "Maximum Fuel Flow".to_string(),
            vec_of_strings!["All phases of flight _$799_ PPH"],
        );
        engine_ops
    });

    opdata_db.insert("Prohibited Maneuvers".to_string(), {
        let mut prohibited_maneuvers = HashMap::new();
        prohibited_maneuvers.insert("Prohibited Maneuvers".to_string(), vec_of_strings![
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
            "11. _$Tail_ spins."
        ]);
        prohibited_maneuvers
    });

    opdata_db.insert("Airspeed Limitations".to_string(), {
        let mut airspeed_limits = HashMap::new();
        airspeed_limits.insert("Airspeed Limitations".to_string(), vec_of_strings![
            "Max Airspeed Gear and/or Flaps _$150_ KIAS", 
            "Max Operating Speed _$316_ KIAS or _$0.67_ Mach", 
            "Full rudder deflection above _$150_ KIAS will exceed the limits of the rubber control system."
        ]);
        airspeed_limits
    });

    opdata_db.insert("Starting".to_string(), {
        let mut starting_limits = HashMap::new();
        starting_limits.insert(
            "Starting".to_string(),
            vec_of_strings![
                "Starter Limit: _$20_ Seconds",
                "Wait _$30_ Sec, _$2_ Min, _$5_ Min, _$30_ Min after each start/motoring attempt",
                "Maximum ITT _$871_ to _$1000_ °C for _$5_ Sec (Do Not Attempt Restart)",
                "Maximum Oil Pressure _$200_ PSI",
                "Maximum Oil Temperature _$-40_ °C",
                "Minimum Battery Voltage _$23.5_ V"
            ],
        );
        starting_limits
    });

    opdata_db.insert("Pressurization".to_string(), {
        let mut pressurization_limits = HashMap::new();
        pressurization_limits.insert(
            "Pressurization".to_string(),
            vec_of_strings![
                "Normal Above 18,000 Ft MSL _$3.6_ ± _$0.2_ PSI",
                "Overpressurization Safety Valve Opens _$5.0_ PSI"
            ],
        );
        pressurization_limits
    });

    opdata_db.insert("Fuel".to_string(), {
        let mut fuel_limits = HashMap::new();
        fuel_limits.insert(
            "Fuel".to_string(),
            vec_of_strings![
                "Normal Recovery Fuel _$200_ Pounds",
                "Minimum Fuel _$150_ Pounds (_$200_ Pounds Solo",
                "Emergency Fuel _$100_ Pounds",
                "Minimum Fuel for Aerobatics _$150_ Pounds Per Side"
            ],
        );
        fuel_limits
    });

    opdata_db.insert("Runway".to_string(), {
        let mut runway_limits = HashMap::new();
        runway_limits.insert(
            "Runway".to_string(),
            vec_of_strings![
                "Maximum Landing Distance Available (LDA) _$4000_ Feet, or heavy weight flaps _$up_ landing distance plus _$500_ feet, whichever is greater.", 
                "Minimum Runway Width _$75_ Feet"
            ],
        );
        runway_limits
    });

    opdata_db.insert("Maximum Crosswinds".to_string(), {
        let mut crosswind_limits = HashMap::new();
        crosswind_limits.insert(
            "Maximum Crosswinds".to_string(),
            vec_of_strings![
                "Dry Runway _$25_ Knots",
                "Wet Runway _$10_ Knots",
                "Icy Runway _$5_ Knots",
                "Touch-and-Go _$20_ Knots",
                "Formation Takeoff/Landing _$15_ Knots",
                "Maximum Tailwind Component for Takeoff _$10_ Knots",
                "Maximum Wind with Canopy Open _$40_ Knots"
            ],
        );
        crosswind_limits
    });

    opdata_db.insert("Acceleration Limits".to_string(), {
        let mut accel_limits = HashMap::new();
        accel_limits.insert(
            "Acceleration Limits".to_string(),
            vec_of_strings![
                "Symmetric Clean _$-3.5_ to _$7.0_ Gs",
                "Symmetric Gear/Flaps _$0_ to _$2.5_ Gs",
                "Asymmetric Clean _$-1.0_ to _$4.7_ Gs",
                "Asymmetric Gear/Flaps _$0_ to _$2.0_ Gs"
            ],
        );
        accel_limits
    });

    opdata_db.insert("Intentional Spin Entry".to_string(), {
        let mut is_limits = HashMap::new();
        is_limits.insert(
            "Intentional Spin Entry".to_string(),
            vec_of_strings![
                "Minimum Altitude for Entry _$13,500_ Feet MSL",
                "Minimum Cloud Clearance _$7,000_ Feet above clouds"
            ],
        );
        is_limits
    });

    opdata_db.insert("Icing".to_string(), {
        let mut icing_limits = HashMap::new();
        icing_limits.insert(
            "Icing".to_string(),
            vec_of_strings![
                "Maximum Icing Band _$5,000_ Feet",
                "Maximum Icing Type _$Light Rime_"
            ],
        );
        icing_limits
    });

    opdata_db.insert("Temperature".to_string(), {
        let mut temp_limits = HashMap::new();
        temp_limits.insert(
            "Temperature".to_string(),
            vec_of_strings![
                "Ground operation is limited to ambient temperatures of _$-23_ to _$40_ °C"
            ],
        );
        temp_limits
    });

    opdata_db
}

/// Holds the different sections of Opdata
#[derive(PartialEq)]
pub enum BfOpdataEnum {
    Engine,
    ProhibitedManeuvers,
    AirspeedLimitations,
    Starting,
    Pressurization,
    Fuel,
    Runway,
    MaximumCrosswinds,
    AccelerationLimits,
    IntentionalSpinEntry,
    Icing,
    Temperature,
}

/// Implementation of BfOpdataEnum
impl BfOpdataEnum {
    // Returns a string representation of the BfOpdataEnum
    pub fn as_str(&self) -> &'static str {
        match self {
            BfOpdataEnum::Engine => "Engine",
            BfOpdataEnum::ProhibitedManeuvers => "Prohibited Maneuvers",
            BfOpdataEnum::AirspeedLimitations => "Airspeed Limitations",
            BfOpdataEnum::Starting => "Starting",
            BfOpdataEnum::Pressurization => "Pressurization",
            BfOpdataEnum::Fuel => "Fuel",
            BfOpdataEnum::Runway => "Runway",
            BfOpdataEnum::MaximumCrosswinds => "Maximum Crosswinds",
            BfOpdataEnum::AccelerationLimits => "Acceleration Limits",
            BfOpdataEnum::IntentionalSpinEntry => "Intentional Spin Entry",
            BfOpdataEnum::Icing => "Icing",
            BfOpdataEnum::Temperature => "Temperature",
        }
    }

    // Go to the next BfOpdataEnum
    pub fn next(&mut self) {
        *self = match self {
            BfOpdataEnum::Engine => BfOpdataEnum::ProhibitedManeuvers,
            BfOpdataEnum::ProhibitedManeuvers => BfOpdataEnum::AirspeedLimitations,
            BfOpdataEnum::AirspeedLimitations => BfOpdataEnum::Starting,
            BfOpdataEnum::Starting => BfOpdataEnum::Pressurization,
            BfOpdataEnum::Pressurization => BfOpdataEnum::Fuel,
            BfOpdataEnum::Fuel => BfOpdataEnum::Runway,
            BfOpdataEnum::Runway => BfOpdataEnum::MaximumCrosswinds,
            BfOpdataEnum::MaximumCrosswinds => BfOpdataEnum::AccelerationLimits,
            BfOpdataEnum::AccelerationLimits => BfOpdataEnum::IntentionalSpinEntry,
            BfOpdataEnum::IntentionalSpinEntry => BfOpdataEnum::Icing,
            BfOpdataEnum::Icing => BfOpdataEnum::Temperature,
            BfOpdataEnum::Temperature => BfOpdataEnum::Engine,
        }
    }

    // Go to the previous BfOpdataEnum
    pub fn prev(&mut self) {
        *self = match self {
            BfOpdataEnum::Engine => BfOpdataEnum::Temperature,
            BfOpdataEnum::ProhibitedManeuvers => BfOpdataEnum::Engine,
            BfOpdataEnum::AirspeedLimitations => BfOpdataEnum::ProhibitedManeuvers,
            BfOpdataEnum::Starting => BfOpdataEnum::AirspeedLimitations,
            BfOpdataEnum::Pressurization => BfOpdataEnum::Starting,
            BfOpdataEnum::Fuel => BfOpdataEnum::Pressurization,
            BfOpdataEnum::Runway => BfOpdataEnum::Fuel,
            BfOpdataEnum::MaximumCrosswinds => BfOpdataEnum::Runway,
            BfOpdataEnum::AccelerationLimits => BfOpdataEnum::MaximumCrosswinds,
            BfOpdataEnum::IntentionalSpinEntry => BfOpdataEnum::AccelerationLimits,
            BfOpdataEnum::Icing => BfOpdataEnum::IntentionalSpinEntry,
            BfOpdataEnum::Temperature => BfOpdataEnum::Icing,
        }
    }
}
