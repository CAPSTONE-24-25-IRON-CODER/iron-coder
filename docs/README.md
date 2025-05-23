# Iron Coder Work Done (Pre-Alpha Build)
  - Updated version of ra_ap_rust-analyzer to 0.0.237 to allow code to build
  - Fixed flagging error when building project by switching to nightly channel
  - Fixed red box bug when dragging elements in the hardware editor
  - Cloned and built project to a working state
  - Setup GitHub organization for project work
  - Architecture for example code built, can load examples from menu 
  - Architecture for automatically generating board planned out, including where the new menu button needs to be added in source code
  - Create structs to store persistent state for fields needed to automatically generate boards
  - Learning Rust, looking through EGUI documentation to help understand the existing code base
  - Research examples of possible IronCoder structure (Wokwi, Fritzing, Arduino)
  - Design ideation/wireframing for simulator and debugging modes
  - Iron Coder Forum work done is listed in the [forum repository](https://github.com/CAPSTONE-24-25-IRON-CODER/iron-coder-forum)
# Iron Coder Work Done (Prototype)
  - Gained a better understanding of the process for creating new components by adding various components through creating TOML files and SVG files
    - Added Arduino Uno as a main board
    - Added Adafruit's 8x8 NeoPixel as a peripheral board
    - Added an 1000 Ω Resistor
    - Added a Yellow LED
    - Added a push button
    - Added a piezo buzzer
  - Support the ability to run Rust code on the Arduino Uno
  - Example code feature finished
  - Added Rust Examples to example folder to be loaded within Iron Coder
      - Alarm Clock Example (Arduino Uno)
      - LCD Screen Example
      - Blink LED Example
      - LED Array Example
  - Updated board menu
      - Changed name of board menu to component menu
      - Updated headers of each component to display board type (Main, Peripheral, Discrete)
      - Sorted components in the menu by board type and name
  - Updated Board struct to support discrete components as a type
      - Created new BoardType enum
      - Change is_main boolean field in Board struct to a BoardType enum
      - Implemented Ord trait for Board (allows us to sort by board type and name)
      - Change internal storage of components by adding a discrete components vector. When a component is added to the hardware editor, it is either added to the peripheral device vector, discrete components vector, or the main board variable.
      - Update .ironcoder TOML files for Example Code hardware editor state to support new discrete component type and vector
  - Create initial module for serial monitor debugger
      - Created data struct to contain serial data
      - Used egui to start testing with plotter
      - Connected example program to serial monitor console on VSCode
      - Researched how to connect code variables to serial
  - Can run Rust programs on Renode supported boards
      - Created an example program to utilize USART on STM32F4 Discovery board
      - Made a Renode script to load board and program .elf file
      - Researched information on how to support new boards such as the AdaFruit Feather RP2040
      - Planned a path to support the AdaFruit Feather RP2040 by production
# Iron Coder Work Done (Alpha Build)
  - Automate Generation of Boards Vertical Slice
      - Added a generate new boards button to the end of the add components menu
      - Created a TOML information form that allows the user to input information about the new board
        - The generate new boards button navigates to this window
      - Added a file dialog that prompts the user for an SVG file when the user completes the TOML information form
      - Created a new window that displays the selected SVG image or an error message if there was SVG parsing failures
      - Implemented basic error handling for SVG parsing failures
      - Added functionality that creates a new board directory, TOML file, and SVG file based on user input
      - Implemented basic error handling for file operation errors
      - Made a confirmation window pop-up when the user finishes the board generation process
      - Added two structs: BoardTomlInfo and PinoutTomlInfo that are used to save board state from user input
  - Wire clipping bug research
      - Researched how EGUI decides the order in which windows and elements are drawn to the screen
  - Terminal Changes
      - Terminal now editable
      - Terminal accepts commands based on operating system
      - Shell output from commands entered show up in Iron Coder terminal
      - Error handling for invalid commands
      - Basic functions of terminal applications
  - Simulator
      - Added simulator button and window for future use of simulation
      - Support for basic emulation of code through qemu
# Iron Coder Work Done (Beta Build)
- Automate Generation of Boards
    - Add user input validation to the TOML information form
      - Make all fields required, user can’t proceed without providing all fields
      - Ensure user is not attempting to duplicate an existing board
    - Add designate pinout functionality
      - Left-clicking image adds a blue pin circle to the UI with the pin name
      - Right-clicking an existing pin deletes it
      - Double-clicking a pin renames it with the name in the pin name box
      - User can change the size of all pins with the pin radius slider
    - Information about the user’s pinouts are saved as EGUI Rects and a vector of String pin names
    - When the user selects done, the pinout information is translated into SVG circle elements, and these are appended to the end of the SVG file
    - When the user adds the generated board to the editor, they can access their created pins and make connections
    - Improve error messages for user when selecting an SVG file
      - Display the specific error that is thrown
      - Add a new error kind (ImageNotPNG) that specifies that the SVG was not derived from a PNG
SVG parser only support PNG image types
      - Previously, a confusing NoImage error was thrown when the image type was not a PNG
      - Add supporting error messages for the user when the ImageNotPNG error is thrown
        - “SVG Image must be derived from a PNG”
    - Add Unit Tests for generating TOML File and SVG File
- Renode Simulator
    - Added better integration of Renode within Iron Coder with the ability to launch a predetermined script for testing
    - Added the option to open and close Renode
    - Added separate threads to read Renode output
    - Added another thread for state saving every 5 minutes

# Iron Coder Architecture
Iron Coder is split into 3 main crates, one that handels the application itself, one for the boards, and the last for projects. The application uses both the boards and projects to
seemlessly create both the hardware and code editors. Within the project source code the project serves as a container for the path to where the project is stored, the boards that
are being used, functions to load/save projects, and anything else the application would need from it during run time. The boards serve a similar purpose in storing the functions and
parameters that are needed to describe the boards and work within the hardware editor. 
# Known bugs
  - Window stays open after loading example, need research into egui to figure out how to close after clicking example
  - Sometimes drawn wire connections will appear on top of the add board menu and other windows
  - More than one discrete component cannot be added to the hardware editor
  - Tree command in windows OS causes crash, possibly due to string variable not being able to hold data from shell output
  - When an SVG image element is very large, it will take up the entire designate pins window and some of the window elements are not accessible (instructions and cancel button)
  - Renode does not automatically compile code done within the window so will require user to compile before starting the script 

