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
  - Added Arduino Uno as a main board
  - Example code feature finished
  - Added examples to example folder to be loaded within Iron Coder
# Iron Coder Architecture
Iron Coder is split into 3 main crates, one that handels the application itself, one for the boards, and the last for projects. The application uses both the boards and projects to
seemlessly create both the hardware and code editors. Within the project source code the project serves as a container for the path to where the project is stored, the boards that
are being used, functions to load/save projects, and anything else the application would need from it during run time. The boards serve a similar purpose in storing the functions and
parameters that are needed to describe the boards and work within the hardware editor. 
# Known bugs
  - Window stays open after loading example, need research into egui to figure out how to close after clicking example
  - Sometimes drawn wire connections will appear on top of the add board menu
    

