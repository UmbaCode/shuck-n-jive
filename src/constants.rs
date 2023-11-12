pub const VERS_STRING: &str = r#"0.02"#;
pub const SOFTWARE_NAME: &str = r#"shuck-n-jive"#;
pub const MAX_HTML_FILENAME_STRING_SIZE: usize = 255; //type usize for declaring size of a string
pub const MAX_FIRST_COMMAND_STRING_SIZE: usize = 16; //type usize for declaring size of a string

pub const APPLICATION_HELP_STRING_BLOCK: &str = r#"
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

pub const INDEX_HTML_FILE_NAME: &str = "index.html"; //name of html file for the index of page in select diretories.

/*
author
search
org


*/
