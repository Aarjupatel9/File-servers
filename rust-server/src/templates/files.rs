pub fn render(files: Vec<String>) -> String {
    let mut html = String::from(r#"<!DOCTYPE html>
<html>
<head>
    <title>Available Files</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
        }
        h1 {
            color: #333;
        }
        ul {
            list-style-type: none;
            padding: 0;
        }
        li {
            margin: 10px 0;
            padding: 10px;
            border: 1px solid #eee;
            border-radius: 4px;
        }
        a {
            color: #2196F3;
            text-decoration: none;
        }
        a:hover {
            text-decoration: underline;
        }
        .back-link {
            display: inline-block;
            margin-top: 20px;
            padding: 10px 15px;
            background-color: #f1f1f1;
            border-radius: 4px;
            color: #333;
        }
    </style>
</head>
<body>
    <h1>Available Files</h1>
    <ul>
"#);

    if files.is_empty() {
        html.push_str("<li>No files uploaded yet.</li>");
    } else {
        for file in files {
            html.push_str(&format!(
                r#"<li><a href="/download/{}" download>{}</a></li>"#,
                file, file
            ));
        }
    }

    html.push_str(r#"
    </ul>
    <a href="/" class="back-link">Back to Upload</a>
</body>
</html>"#);

    html
}
