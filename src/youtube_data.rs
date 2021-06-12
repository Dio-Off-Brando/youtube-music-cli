use std::array;

use reqwest::Client;
use json;

//gonna hard code the youtube token just for testing, ill remove it and replace it with the token
//found in .config/youtube-music-cli/config.json
//my api key -> AIzaSyBQiSl_pVzCUsNoxhhiEgNADqE-L-_Ta84


//again just for testing purposes, probably gonna remove it on deployment
fn api_key() -> String{


    "AIzaSyBQiSl_pVzCUsNoxhhiEgNADqE-L-_Ta84".to_string()
}


struct youtube_video {
    name: String,
    url: String

}


// need to turn the response into an array of youtube_videos somehow
//
// iterate for each json object in json array push youtube_video into vector with name and url
//

fn youtube_data_search_req(query: &str) -> Vec<youtube_video> {
  
    let mut url: String = "https://www.googleapis.com/youtube/v3/search?part=".to_owned();
    
    let access_token: &str = "";    
    

}


pub fn youtube_search(query: String) {
   
    

}
