/*
AUTHOR: RC
ORG: DataHotep Inc.
USE: main program file for shuck-n-jive
*/

// Static variables
static VERS_STRING: &str = r#"0.01"#;
static SOFTWARE_NAME: &str = r#"shuck-n-jive"#;
static MAX_HTML_FILENAME_STRING_SIZE: usize = 255; //type usize for declaring size of a string
const MAX_FIRST_COMMAND_STRING_SIZE: usize = 16; //type usize for declaring size of a string

static APPLICATION_HELP_STRING_BLOCK: &str = r#"
Shuck-N-Jive
Author: RC
shuck-n-jive creates a static web site for causing trouble.

 Find more information at: https://github.com/UmbaCode/shuck-n-jive

Commands:
  status        Check the status of the site stored in the database.
  init          Initialize a database store for static site.
  generate      Run a particular image on the cluster
  add           Add an: author, article, etc.
  delete        Delete an: author, article, etc.
  modify        Modify an: author, article, etc.
  help          This page.

  Usage:
    shuck-n-jive [flags] [options]
"#;

enum CommandLineFirstState {
    //These establish the First State From the first command given
    FirstStateExit,     //Usually this will mean Break out of the loop
    FirstStateStatus,   //Show the status
    FirstStateInit,     //Initialize the DB
    FirstStateGenerate, //Generate the site
    FirstStateAdd,      //Add
    FirstStateDelete,   //Delete
    FirstStateModify,   //Modify
    FirstStateHelp,     //Help
}

enum CommandLineSecondState {
    //These establish the Second State From the first command given
    SecondStateMoot,
    SecondStateAuthor,
    SecondStateDatabase,
    SecondStateArticle,
    SecondStateHelp,
    SecondStateYamlDocument,
}

enum CommandLineThirdState {
    //These establish the Third State From the first command given
    ThirdStateName,
    ThirdStateId,
    ThirdStateDatabase,
    ThirdStateArticle,
    ThirdStateHelp,
    ThirdStateYamlDocument,
    ThirdStateMarkdownDocument,
}

// Main Libraries
//use dict::{ Dict };

fn open_file_and_return_string(file_path_string: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::Path;

    let file_path = Path::new(file_path_string);
    let display = file_path.display();

    let file_to_open = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file_to_open) => file_to_open,
    };

    let mut buf_reader = BufReader::new(file_to_open);
    let mut internal_string_buffer = String::new();

    match buf_reader.read_to_string(&mut internal_string_buffer) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(internal_string_buffer) => internal_string_buffer,
    };

    return internal_string_buffer;
}

fn sanitize_uri_string(passed_string: &str) -> String {
    //short circuit the function
    //if passed_string is null
    //use voca_rs
    use voca_rs::*;

    //vector to loop through so we can do what is appropriate
    let dashVector = vec!["/", ".", "_", "~", " ", "="];
    let voidVector = vec![
        "!", "$", "&", "'", "(", ")", "*", "+", ",", ";", "[", "]", "@", "?", ":", "#",
    ];

    //string buffer
    let mut internalStringBuffer = String::with_capacity(MAX_HTML_FILENAME_STRING_SIZE);

    internalStringBuffer.insert_str(0, &passed_string);

    //trim the whitespaces on the end before looping
    internalStringBuffer = manipulate::trim(&internalStringBuffer, "");

    //replace with dash
    for character in dashVector {
        internalStringBuffer = manipulate::replace_all(&internalStringBuffer, character, "-");
    }

    //replace with nothing
    for character in voidVector {
        internalStringBuffer = manipulate::replace_all(&internalStringBuffer, character, "");
    }

    //return lowercase
    return case::lower_case(&internalStringBuffer);
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
    internal_string_buffer.insert_str(0, &"passed_string");

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

fn print_markdown_text_to_html_string(passed_string: &str) -> String {
    //this function grabs the markdown document and passes it as html
    use markdown::*;

    let mut internal_string_buffer = String::new(); //internal string buffer

    internal_string_buffer.insert_str(0, &to_html(passed_string));

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
    authorDescription: Option<String>,
}

struct mainPageContentBodyStruct {
    /* This is the structure for the main page */
    imageLocation: Option<String>,
    imageDescription: Option<String>,
    textBody: Option<String>,
    textTitle: Option<String>,
    dateFooter: Option<String>,
    anchorLinkToContent: Option<String>,
}

enum processStackJobState {
    // these are the states for the process queue for the main program
    PROCESS_EXIT,          //first state should be exit
    PROCESS_ERROR,         //show the error screen.. which is the bulk info
    PROCESS_HELP,          // show the help screen.. which is the bulk info pretty much
    PROCESS_VERSION,       //show the version of the software
    PROCESS_AUTHOR_LOOKUP, //show the author lookup from the database
}

fn first_command_state(passed_string: &str) -> CommandLineFirstState {
    const MAX_FIRST_COMMAND_STRING_SIZE: usize = 16; //type usize for declaring size of a string

    use voca_rs::*;

    //let lower_command_state = ;

    let mut internal_string_buffer = String::with_capacity(MAX_FIRST_COMMAND_STRING_SIZE);

    internal_string_buffer.insert_str(0, &case::lower_case(passed_string));

    let mut internel_state_buffer;

    match internal_string_buffer.as_str() {
        "status" => {
            internel_state_buffer = CommandLineFirstState::FirstStateStatus;
        }
        "init" => {
            internel_state_buffer = CommandLineFirstState::FirstStateInit;
        }
        "generate" => {
            internel_state_buffer = CommandLineFirstState::FirstStateGenerate;
        }
        "add" => {
            internel_state_buffer = CommandLineFirstState::FirstStateAdd;
        }
        "delete" => {
            internel_state_buffer = CommandLineFirstState::FirstStateDelete;
        }
        "modify" => {
            internel_state_buffer = CommandLineFirstState::FirstStateModify;
        }
        "help" => {
            internel_state_buffer = CommandLineFirstState::FirstStateHelp;
        }
        _ => {
            internel_state_buffer = CommandLineFirstState::FirstStateHelp;
        }
    };

    return internel_state_buffer;
}

fn main() {
    //use standard collections
    use std::collections::*;
    use std::env;

    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let mut mainProcessStack: Vec<processStackJobState> = Vec::new();

    //push the error
    mainProcessStack.push(processStackJobState::PROCESS_ERROR);

    // main loop
    let loop_defeat = 1;

    match loop_defeat {
        1 => {
            println!("Its Monday my dudes");
        }
        2 => {
            println!("It's Tuesday my dudes");
        }
        3 => {
            println!("It's Wednesday my dudes");
        }
        4 => {
            println!("It's Thursday my dudes");
        }
        5 => {
            println!("It's Friday my dudes");
        }
        6 => {
            println!("It's Saturday my dudes");
        }
        7 => {
            println!("It's Sunday my dudes");
        }
        _ => {
            println!("Default!")
        }
    };

    println!("Still cooking");

    //let mut _dict = Dict::<String>::new();
    //    let mut book_reviews = HashMap::new();
}
