/* 
AUTHOR: RC 
ORG: DataHotep Inc. 
USE: main program file for shuck-n-jive
*/

// Static variables
static VERS_STRING: &str = r#"0.01"#;
static SOFTWARE_NAME: &str = r#"shuck-n-jive"#;
static MAX_HTML_FILENAME_STRING_SIZE: usize = 255; //type usize for declaring size of a string




// Main Libraries
//use dict::{ Dict };


fn sanitizeString(passedString: &str) -> String
{
    //short circuit the function
    //if passedString is null    
    //use voca_rs
    use voca_rs::*;
    
    //vector to loop through so we can do what is appropriate
    let dashVector = vec!["/", ".", "_", "~", " ","="];
    let voidVector = vec!["!", "$", "&", "'", "(", ")", "*", "+", ",", ";","[","]","@","?", ":","#"];
     
    //string buffer 
    let mut internalStringBuffer = String::with_capacity(MAX_HTML_FILENAME_STRING_SIZE);
    
    internalStringBuffer.insert_str(0,&passedString);

    //trim the whitespaces on the end before looping 
    internalStringBuffer = manipulate::trim(&internalStringBuffer,"");


    //replace with dash 
    for character in dashVector {
        
        internalStringBuffer = manipulate::replace_all(&internalStringBuffer, character, "-");

    }

    //replace with nothing
    for character in voidVector {
        
        internalStringBuffer = manipulate::replace_all(&internalStringBuffer, character, "");

    }
    

    //return lowercase
    return( case::lower_case(&internalStringBuffer));
}






fn pull_yaml_from_markdown(passed_string: &str) -> String {
    //libraries at the top
    use gray_matter::engine::YAML;
    use gray_matter::Matter;
    use serde::Deserialize;

    //its just easy right now to take the pod data and return a new String object
    let matter_lib_plugin_w_pod_return = Matter::<YAML>::new();

    let parsed_font_yaml_results = matter_lib_plugin_w_pod_return.parse(passed_string);

    let mut internal_string_buffer = String::new();
    internal_string_buffer.insert_str(0, &"passedString");

    struct YamlBlockStruct {
        title: String,
        descriptions: String,
        tags: Vec<String>,
        keywords: Vec<String>,
        publish_date: String,
        publish_time: String,
        author_id: String,
    }

    println!(
        "{:?}",
        parsed_font_yaml_results.data.as_ref().unwrap()["title"].as_string()
    );

    return internal_string_buffer;
}



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


enum processStackJobState {
// these are the states for the process queue for the main program
    PROCESS_EXIT, //first state should be exit
    PROCESS_ERROR, //show the error screen.. which is the bulk info
    PROCESS_HELP, // show the help screen.. which is the bulk info pretty much
    PROCESS_VERSION, //show the version of the software
    PROCESS_AUTHOR_LOOKUP, //show the author lookup from the database
}



fn main() 
{
    //use standard collections
    use std::collections::*; 

    let mut mainProcessStack: Vec<processStackJobState> = Vec::new();

    //push the error
    mainProcessStack.push(processStackJobState::PROCESS_ERROR);






    // main loop
    let loop_defeat = 1;

    match loop_defeat {
        1 => {
          println!("Its Monday my dudes");
        },
        2 => {
          println!("It's Tuesday my dudes");
        },
        3 => {
          println!("It's Wednesday my dudes");
        },
        4 => {
          println!("It's Thursday my dudes");
        },
        5 => {
          println!("It's Friday my dudes");
        },
        6 => {
          println!("It's Saturday my dudes");
        },
        7 => {
          println!("It's Sunday my dudes");
        },
        _ => {
          println!("Default!")
        }
      };
    
    
    println!("Still cooking");

    //let mut _dict = Dict::<String>::new();
    //    let mut book_reviews = HashMap::new();


}
