use regex::Regex;
use std::env;
fn main() -> Result<(), std::io::Error> {
    //get the first input for the name of the file

    let input = env::args().nth(1).expect("pease enter a name for html");

    assert!(
        input.len() < 15,
        "the name of the file should not be bigger than 15 characters"
    );
    let re = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    assert!(
        re.is_match(input.as_str()),
        "please insert a proper name without special characters and stuff"
    );

    let directory_name = input;
    // simple protection dublication protection
    if let Ok(entries) = std::fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                //everything is good, as long as the html_name doesn't equal the entries
                //else the program terminates
                assert!(entry.file_name() != directory_name.as_str(),
                "the name of the file matches with another one.. please choose another name for your html");
            }
        }
    }

    let html_file_name = format!("{}{}",directory_name,"/index.html");
    let css_file = format!("{}{}",directory_name,"/style.css");
    let js_script = format!("{}{}",directory_name,"/script.js");
    // finally create and write the content into the file
    std::fs::create_dir(&directory_name)?;
    std::fs::write(html_file_name, html_default_content())?;
    std::fs::write(css_file, css_default_content())?;
    std::fs::write(js_script, "")?;
    Ok(())
}

//html template
pub fn html_default_content() -> String { 
    let content = "<!DOCTYPE html>
<html lang=\"en\">
    <head>
       <meta charset=\"utf-8\">
       <title>title</title>
       <link rel=\"stylesheet\" href=\"style.css\">
       <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
       <meta name=\"robots\" content=\"index,follow\"><!--helps so google can follow-->
       <script src=\"script.js\"></script>
    </head>
    <body>
        <header>
        </header>        
        <main>
        </main>        
        <footer>
        </footer>    
    </body>
</html>";
    content.to_string()
}
//css template
pub fn css_default_content()->String{
    "*{ margin: 0; padding: 0; box-sizing: border-box; }".to_string()
}

