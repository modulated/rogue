#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct PrefabRoom {
    pub template: &'static str,
    pub width: usize,
    pub height: usize,
    pub first_depth: i32,
    pub last_depth: i32,
}

#[allow(dead_code)]
pub const TRAP_ROOM: PrefabRoom = PrefabRoom {
    template: TRAP_MAP,
    width: 5,
    height: 5,
    first_depth: 0,
    last_depth: 100,
};

#[allow(dead_code)]
const TRAP_MAP: &str = "
     
 ^^^ 
 ^!^ 
 ^^^ 
     
";

#[allow(dead_code)]
pub const SILLY_SMILE: PrefabRoom = PrefabRoom {
    template: SILLY_SMILE_MAP,
    width: 6,
    height: 6,
    first_depth: 0,
    last_depth: 100,
};

#[allow(dead_code)]
const SILLY_SMILE_MAP: &str = "
      
 ^  ^ 
  #   
      
 ###  
      
";

#[allow(dead_code)]
pub const CHECKERBOARD: PrefabRoom = PrefabRoom {
    template: CHECKERBOARD_MAP,
    width: 6,
    height: 5,
    first_depth: 0,
    last_depth: 100,
};

#[allow(dead_code)]
const CHECKERBOARD_MAP: &str = "
      
 g# # 
 #!#  
 ^# # 
      
";
