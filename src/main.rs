mod ui_elements;
mod youtube_data;


fn main() {
    ui_elements::highlight_text("test".to_string());
    let test:String = String::from("test"); 

    println!("{}", test.as_str())
}
