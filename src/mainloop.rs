




pub fn gamelogic(input:String) -> i32{
    if input.starts_with("/"){
        match input {
            _ => return 1
        }
    }
    else if input == "".to_string() {
        return 1;
    }
    else{
        return 0;
    }
}
