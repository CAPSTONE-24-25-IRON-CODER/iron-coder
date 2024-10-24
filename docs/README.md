# Iron Coder Work Done
  - Fixed flagging error when building project by swithcing to nightly channel
  - Fixed red box bug when dragging elements in the hardware editor
  - Cloned and built project to a working state
# Iron Coder Architecture
Iron Coder is split into 3 main crates, one that handels the application itself, one for the boards, and the last for projects. The application uses both the boards and projects to
seemlessly create both the hardware and code editors. Within the project source code the project serves as a container for the path to where the project is stored, the boards that
are being used, functions to load/save projects, and anything else the application would need from it during run time. The boards serve a similar purpose in storing the functions and
parameters that are needed to describe the boards and work within the hardware editor. 
# Known bugs
  - 

