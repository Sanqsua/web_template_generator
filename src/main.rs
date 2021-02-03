use regex::Regex;
use std::env;

fn main() -> Result<(), std::io::Error> {
    //get the first input for the name of the file
    let input = env::args().nth(1).expect("pease enter a name for html");

    let re = Regex::new(r"^[a-zA-Z1-9_-]+$").unwrap();
    assert!(
        re.is_match(input.as_str()),
        "please insert a proper name without special characters and stuff"
    );

    let html_file_name = format!("{}{}", input, ".html");
    
    // simple protection dublication protection
    if let Ok(entries) = std::fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                //everything is good, as long as the html_name doesn't equal the entries
                //else the program terminates
                assert!(entry.file_name() != html_file_name.as_str(),
                "the name of the file matches with another one.. please choose another name for your html");
            }
        }
    }
    
    // finally create and write the content into the file
    std::fs::write(html_file_name, content())?;
    Ok(())
}

fn content() -> String {
    //html template
    let content = "<!DOCTYPE html>
<html lang=\"en\">
    <head>
       <meta charset=\"utf-8\">
       <title>title</title>
       <link rel=\"stylesheet\" href=\"style.css\">
       <script src=\"script.js\"></script>
    </head>
    <body>
        <!-- page content -->
    </body>
</html>";
    content.to_string()
}
