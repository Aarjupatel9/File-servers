pub fn render() -> String {
    r#"<!DOCTYPE html>
<html>
<head>
    <title>Rust File Server</title>
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
        form {
            margin: 20px 0;
            padding: 20px;
            border: 1px solid #ddd;
            border-radius: 5px;
        }
        input[type="file"] {
            margin: 10px 0;
        }
        button {
            background-color: #4CAF50;
            color: white;
            padding: 10px 15px;
            border: none;
            border-radius: 4px;
            cursor: pointer;
        }
        button:hover {
            background-color: #45a049;
        }
        #progress-container {
            display: none;
            margin-top: 10px;
            width: 100%;
            background: #eee;
            border-radius: 4px;
            overflow: hidden;
        }
        #progress-bar {
            height: 20px;
            width: 0%;
            background: #4CAF50;
            text-align: center;
            line-height: 20px;
            color: white;
        }
    </style>
</head>
<body>
    <h1>Rust File Server</h1>
    
    <form id="upload-form">
        <h2>Upload a file</h2>
        <input type="file" id="file-input" required />
        <br />
        <button type="submit">Upload</button>
    </form>

    <!-- Progress Bar -->
    <div id="progress-container">
        <div id="progress-bar">0%</div>
    </div>

    <h2>Available Files</h2>
    <p>Visit <a href="/files">/files</a> to see and download uploaded files.</p>

    <script>
        document.getElementById("upload-form").addEventListener("submit", function(event) {
            event.preventDefault();

            let fileInput = document.getElementById("file-input");
            if (fileInput.files.length === 0) {
                alert("Please select a file to upload.");
                return;
            }

            let file = fileInput.files[0];
            let formData = new FormData();
            formData.append("file", file);

            let xhr = new XMLHttpRequest();
            xhr.open("POST", "/upload", true);

            xhr.upload.onprogress = function(event) {
                if (event.lengthComputable) {
                    let percentComplete = (event.loaded / event.total) * 100;
                    let progressBar = document.getElementById("progress-bar");
                    progressBar.style.width = percentComplete + "%";
                    progressBar.textContent = Math.round(percentComplete) + "%";
                }
            };

            xhr.onloadstart = function() {
                document.getElementById("progress-container").style.display = "block";
                document.getElementById("progress-bar").style.width = "0%";
                document.getElementById("progress-bar").textContent = "0%";
            };

            xhr.onload = function() {
                if (xhr.status === 200) {
                    alert("File uploaded successfully!");
                    document.getElementById("progress-container").style.display = "none";
                } else {
                    alert("Error uploading file.");
                }
            };

            xhr.onerror = function() {
                alert("Request failed.");
            };

            xhr.send(formData);
        });
    </script>
</body>
</html>
"#
    .to_string()
}
