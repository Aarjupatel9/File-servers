<?php

$allowedExts = array("jpg", "jpeg", "gif", "png", "mp4", "mkv");
$extension = pathinfo($_FILES['file']['name'], PATHINFO_EXTENSION);

if ((($_FILES["file"]["type"] == "image/pjpeg")
        || ($_FILES["file"]["type"] == "image/gif")
        || ($_FILES["file"]["type"] == "video/mp4")
        || ($_FILES["file"]["type"] == "video/x-matroska")
        || ($_FILES["file"]["type"] == "image/jpeg"))
    && ($_FILES["file"]["size"] < 900000000000000)
    && in_array($extension, $allowedExts)
) {
    if ($_FILES["file"]["error"] > 0) {
        echo "Return Code: " . $_FILES["file"]["error"] . "<br />";
    } else {
        echo "Upload: " . $_FILES["file"]["name"] . "<br />";
        echo "Type: " . $_FILES["file"]["type"] . "<br />";
        echo "Size: " . ($_FILES["file"]["size"] / 1024) . " Kb<br />";
        echo "Temp file: " . $_FILES["file"]["tmp_name"] . "<br />";

        if (file_exists("uploaded_file/" . $_FILES["file"]["name"])) {
            echo $_FILES["file"]["name"] . " already exists. ";
        } else {
            move_uploaded_file(
                $_FILES["file"]["tmp_name"],
                "uploaded_file/" . $_FILES["file"]["name"]
            );
            echo "Stored at : " . "uploaded_file/" . $_FILES["file"]["name"];
        }
    }
} else {
    echo "File type : " . $_FILES["file"]["type"] . "<br>";
    echo "Invalid file" . "<br>";
    echo "Extention in the file is : " . $extension . "<br>";
    echo "size of the file is " .  $_FILES["file"]["size"] . "<br>";
}
