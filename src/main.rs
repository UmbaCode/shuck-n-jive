/* 
AUTHOR: RC 
ORG: DataHotep Inc. 
USE: main program file for shuck-n-jive
*/


let VERS_STRING = r#"0.01"#;
let SOFTWARE_NAME = r#"shuck-n-jive"#;


// Main Libraries
use dict::{ Dict, DictIface };
use std::collections::HashMap;
use mysql::*;
use mysql::prelude::*;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;
use tera::Tera;



//#[derive(Debug, PartialEq, Eq)]
struct authorStruct {
    // for working with author data
    authorId: i32,
    authorFirstName: Option<String>,
    authorIdHash: Option<String>,
    authorMiddleName: Option<String>,
    authorLastName: Option<String>,
    authorUri: Option<String>,
    authorEmail: Option<String>,
    authorDescription: Option<String>
};



let mut _dict = Dict::<String>::new();
let mut book_reviews = HashMap::new();


fn filterCharactersForFileName()
{
    // This function will remove the illegal characters from a file name for a website and replace it 
    // ":" / "/" / "?" / "#" / "[" / "]" / "@"
    //"!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
    // "." / "_" / "~"
}

fn main() 
{
    println!("Sup World");


}
