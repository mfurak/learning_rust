/*
STRENGTHS of State pattern:
  - when business requirements change only the code of the specific state object has to be updated or
    more states have to be added
LIMITATIONS of State pattern:
  - adding another state as a transition between 2 existing ones requires a lot of rewrites
  - lot of logic duplication especially for methods like request_review where default implementations
      cannot exist because of object safety (cannot return self because 
      the compiler wouldn't know the concrete type) <= could be solved by macros
*/
pub mod states_as_traits;

/*
STRENGTHS of Encoding States and Behavior as Types:
  - only methods used in the state need to be implemented
  - compile time checks for specific state properties and constraints => invalid states are impossible
LIMITATIONS of Encoding States and Behavior as Types:
  - state transitions are not completely encapsulated within the Post implementation => we reassign
    the post variable on each state change and don't just mutate it in place
*/
pub mod states_as_types;