/* 
AUTHOR: RC 
ORG: DataHotep Inc. 
USE: main program file for shuck-n-jive
*/


static VERS_STRING: &str = r#"0.01"#;
static SOFTWARE_NAME: &str = r#"shuck-n-jive"#;


// Main Libraries
use dict::{ Dict };









//#[derive(Debug, PartialEq, Eq)]
struct authorStruct {
   /* // for working with author data */ 
    authorId: i32,
    authorFirstName: Option<String>,
    authorIdHash: Option<String>,
    authorMiddleName: Option<String>,
    authorLastName: Option<String>,
    authorUri: Option<String>,
    authorEmail: Option<String>,
    authorDescription: Option<String>
}


struct mainPageContentBodyStruct {
    /* This is the structure for the main page */
    imageLocation: Option<String>,
    imageDescription: Option<String>,
    textBody: Option<String>,
    textTitle: Option<String>,
    dateFooter: Option<String>,
    anchorLinkToContent: Option<String>

}





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

    let mut _dict = Dict::<String>::new();
    //    let mut book_reviews = HashMap::new();


}
