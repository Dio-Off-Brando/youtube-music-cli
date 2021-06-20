use json;
use isahc;
use isahc::prelude::*;
//gonna hard code the youtube token just for testing, ill remove it and replace it with the token
//found in .config/youtube-music-cli/config.json
//my api key -> AIzaSyDqdQDsS4gRbDI_oxIzdF1zEH7-hwoBgbA
// its probably 
//again just for testing purposes, probably gonna remove it on deployment
static api_key:&str ="AIzaSyDqdQDsS4gRbDI_oxIzdF1zEH7-hwoBgbA";


struct youtube_video {
    name: String,
    url: String

}


// need to turn the response into an array of youtube_videos somehow
//
// iterate for each json object in json array push youtube_video into vector with name and url
//

fn youtube_data_search_req(query: &str) -> Result<String, isahc::Error> {
    //make query url-friendly
    let query:String = String::from("q=")+query;
    let mut query:String = query.replace(" ", "%20");

    let mut url: String = "https://www.googleapis.com/youtube/v3/search?part=snippet&".to_owned(); 
    let mut video_list: Vec<youtube_video> = vec![];
    
    let url:String = String::from(url.as_str()) + query.as_str() + "&key=AIzaSyDqdQDsS4gRbDI_oxIzdF1zEH7-hwoBgbA";
    

    //return the result
    let mut result = isahc::get(url)?.text()?;

    Ok(result)
   
    // finally -_-
}


pub fn youtube_search_json_parser(query: String) Vec<youtube_video> {
   //println!("placeholder")
        

}
