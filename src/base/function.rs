pub fn get_level_by_score(score:i32) -> &'static str {
    if score>=90 {
        return "A";
    } else if score>=80 && score<90 {
        return "B";
    } else if score>=70 && score<80 {
        return "C";
    } else if score>=60 && score<70 {
        return "D";
    } else {
        return "E";
    }
}