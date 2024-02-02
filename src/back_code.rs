pub struct BackCode{
    capsuled_tasks: usize,
    capsule_stuff: Vec<String>,
}

impl BackCode{
    // constructor
    pub fn new() -> BackCode{ // initialize capsuled_tasks and capsuled_stuff
        BackCode{
            capsuled_tasks: 0,
            capsule_stuff: Vec::new(),
        }
    }

    // setters 
    pub fn set_capsuled_stuff(&mut self, capsuled_tasks: usize){
        self.capsuled_tasks = capsuled_tasks;
    }

    pub fn set_capsule_stuff(&mut self, capsule_stuff: Vec<String>){
        self.capsule_stuff = capsule_stuff;
    }

    pub fn add_capsule_stuff(&mut self, item: String){
        self.capsule_stuff.push(item);
    }

    pub fn remove_capsule_stuff(&mut self, index: usize){
        if index < self.capsule_stuff.len(){
            self.capsule_stuff.remove(index);
        }
    }

    pub fn edit_capsule_stuff(&mut self, index: usize, item: String){
        if index < self.capsule_stuff.len(){
            self.capsule_stuff[index] = item;
        }
    }


    // getters
    pub fn get_capsuled_tasks(&self) -> usize {
        self.capsuled_tasks
    }

    pub fn get_capsule_stuff(&self) -> &Vec<String> {
        &self.capsule_stuff
    }
}