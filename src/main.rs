// link and compile other libs to the project
extern crate hyper;
extern crate rustc_serialize;
extern crate hyper_native_tls;

use hyper::client::Client;
use std::io::Read;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io;




fn main () {

	println!("----------------------------------------------------------------------------
\n --- Welcome to Rust Translator by Marwen Doukh - Mozilla Tunisia ---
\n----------------------------------------------------------------------------");
	println!("Please enter the word you want to translate");
	// create a new variable to save the word typed in the keyboard
        let mut word = String::new();
	// get user input from keyboard
        io::stdin().read_line(&mut word)
            .expect("failed to read the word");

	/// propmt to the user the language choices
	println!("Please choose the language that you want to translate to (eg : choose en if you want English)
\n English : en
\n Arabic : ar
\n French : fr
\n German : de
\n ----------------------
\n Translate to :");

	// create a new variable to save the choice number typed in the keyboard
        let mut language = String::new();
	// get user input from keyboard
        io::stdin().read_line(&mut language)
            .expect("failed to read the language");

	// append string to get the right URL request
   	word = word.to_string();
	let mut link = "https://translate.yandex.net/api/v1.5/tr.json/translate?key=trnsl.1.1.20170114T114459Z.7de5877240d656e7.e5bdc730b5945491203a9b7a0ee0cfd7af579369&text=".to_string();
	link = link +  &word;
	link.push_str("&lang=");
	link= link+&language;
	link.push_str("&format=json");


	// we need that because we're going to request an SSL link
	let ssl = NativeTlsClient::new().unwrap();
   	let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
	// the string that will contain the request result
   	let mut translationResult = String::new();
	// send the request   
 	client.get(&link)
                    .send()
                    .unwrap()
		    // put the result to the string declared above
                    .read_to_string(&mut translationResult)
                    .unwrap();

	// format the result as JSON 
	let data = rustc_serialize::json::Json::from_str(&translationResult).unwrap();
  
	// obj will contain the JSON object
   	let obj = data.as_object().unwrap();
	//from the JSON object created , get only the result of translation
	let translatedWord = obj.get("text").unwrap();
	let translatedFromTo = obj.get("lang").unwrap();
	// display it 
	println!("translation is : {} ",translatedWord[0]);
	println!("translated from-to  : {} ",translatedFromTo);

}
