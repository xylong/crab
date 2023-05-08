pub fn get_level_by_score(score:i32) -> &'static str {
    let level = if score>=90 {
         "A"
    } else if score>=80 && score<90 {
         "B"
    } else if score>=70 && score<80 {
         "C"
    } else if score>=60 && score<70 {
         "D"
    } else {
         "E"
    };

    return level;
}

