pub fn build_proverb(list: &[&str]) -> String {
    let mut words = vec![];
    if list.len() == 0{
        return String::new();
    }
    
    for i in 0.. (list.len()-1){

    words.push(format!("For want of a {} the {} was lost.",list[i] ,list[i+1] ));
        
    }
    words.push(format!("And all for the want of a {}.",list[0] ));

    words.join("\n")
    
}