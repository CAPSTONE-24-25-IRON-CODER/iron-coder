// here we will be build a simulator in rust

// we should be able to run the code in the background

// handle the errors and windows

// handle board and pieces

// two layers

// GUI board and components
    // will use egui

// Handle of business logic


// simulator.rs
use egui::{Context, Window};

pub struct Simulator {
    pub is_open: bool,               // Whether the simulator window is open
    pub simulation_state: String,    // State of the simulation: "Running", "Stopped", etc.
    pub simulation_time: f32,        // Simulation time tracker
    pub simulation_logs: Vec<String>, // Logs from the simulation
}

impl Simulator {
    // Constructor for Simulator
    pub fn new() -> Self {
        Self {
            is_open: false,
            simulation_state: "Stopped".into(),
            simulation_time: 0.0,
            simulation_logs: Vec::new(),
        }
    }

    pub fn show_window(&mut self, ctx: &egui::Context) {
        if self.is_open {
            // Create the simulator window
            Window::new("Simulator")
                .resizable(true)
                .show(ctx, |ui| {
                    // Display simulation state and time
                    ui.label(format!("State: {}", self.simulation_state));
                    ui.label(format!("Time: {:.2} seconds", self.simulation_time));

                    // Display the logs in a scrollable container
                    ui.group(|ui| {
                        for log in &self.simulation_logs {
                            ui.label(log);
                        }
                    });

                    // Control buttons
                    if ui.button("Start Simulation").clicked() {
                        self.start_simulation();
                    }
                    if ui.button("Stop Simulation").clicked() {
                        self.stop_simulation();
                    }
                    if ui.button("Reset Simulation").clicked() {
                        self.reset_simulation();
                    }
                });
        }
    }

    // Start the simulation
    fn start_simulation(&mut self) {
        self.simulation_state = "Running".into();
        self.simulation_logs.push("Simulation started.".into());
        // Logic to begin the simulation
    }

    // Stop the simulation
    fn stop_simulation(&mut self) {
        self.simulation_state = "Stopped".into();
        self.simulation_logs.push("Simulation stopped.".into());
        // Logic to stop the simulation
    }

    // Reset the simulation state and clear logs
    fn reset_simulation(&mut self) {
        self.simulation_state = "Reset".into();
        self.simulation_time = 0.0;
        self.simulation_logs.clear();
        self.simulation_logs.push("Simulation reset.".into());
    }
}
