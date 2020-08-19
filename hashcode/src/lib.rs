pub fn one_is_bigger(one:String,two:String) -> bool {
    let one_as_num = convert_to_num(one);
    let two_as_num = convert_to_num(two);
    if one_as_num.len() > two_as_num.len(){
        return true;
    }
    if one_as_num.len() < two_as_num.len(){
        return false;
    }
    let one_as_bytes = one_as_num.into_bytes();
    let two_as_bytes = two_as_num.into_bytes();
    for i in 0..one_as_bytes.len(){
        if one_as_bytes[i] > two_as_bytes[i]{
            return true;
        }
        if one_as_bytes[i] < two_as_bytes[i]{
            return false;
        }
    }
    return false;
}

fn convert_to_num(s:String) -> String {
    let mut collect = String::new();
    for b in s.into_bytes(){
        collect.push_str(&b.to_string());
    }
    collect
}
