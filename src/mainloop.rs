




pub fn gamelogic(input:String) -> i32{
    if input.starts_with("/"){
        match input {
          
            _ => return 1
        }
    }

    else{
        return -1;
    }
}

pub fn gameinit(Username:String) -> (String,bool){
    let mut namestate:bool = true;
    loop {
        if Username.contains(""){
            namestate = true;
            return (Username, namestate);
        }
        else {
            namestate = false;
            return (Username,namestate);
        }
    }
}