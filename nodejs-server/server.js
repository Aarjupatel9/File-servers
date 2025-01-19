const express = require('express');
const multer = require('multer');
const path = require('path');
const fs = require('fs');
const os = require('os');

const app = express();
const PORT = 3030;

// Function to get the local IP address
function getLocalIp() {
  const interfaces = os.networkInterfaces();
  for (const iface of Object.values(interfaces)) {
    for (const details of iface) {
      if (details.family === 'IPv4' && !details.internal) {
        return details.address;
      }
    }
  }
  return 'localhost';
}

// Set up storage for file uploads
const storage = multer.diskStorage({
  destination: (req, file, cb) => {
    const uploadDir = path.join(__dirname, 'uploads');
    if (!fs.existsSync(uploadDir)) {
      fs.mkdirSync(uploadDir);
    }
    cb(null, uploadDir);
  },
  filename: (req, file, cb) => {
    const uploadDir = path.join(__dirname, 'uploads');
    let fileName = file.originalname;
    const fileExt = path.extname(fileName);
    const baseName = path.basename(fileName, fileExt);
    let counter = 1;

    while (fs.existsSync(path.join(uploadDir, fileName))) {
      fileName = `${baseName}_${counter}${fileExt}`;
      counter++;
    }

    cb(null, fileName);
  }
});

const upload = multer({
  storage,
  limits: { fileSize:20* 1024 * 1024 * 1024 } // Limit file size to 1GB
});

// Serve the upload form
app.get('/', (req, res) => {
  const uploadDir = path.join(__dirname, 'uploads');
  const files = fs.existsSync(uploadDir) ? fs.readdirSync(uploadDir) : [];

  const fileList = files.map(file => `<li><a href="/download/${file}">${file}</a></li>`).join('');

  res.send(`
    <h1>File Transfer Server</h1>
    <form action="/upload" method="post" enctype="multipart/form-data">
      <input type="file" name="file" />
      <progress id="progress" value="0" max="100" style="width: 100%;"></progress>
      <br>
      <button type="submit">Upload</button>
    </form>
    <h2>Available Files</h2>
    <ul>
      ${fileList}
    </ul>
    <script>
      const form = document.querySelector('form');
      const progress = document.getElementById('progress');

      form.addEventListener('submit', (e) => {
        e.preventDefault();
        const xhr = new XMLHttpRequest();
        const formData = new FormData(form);

        xhr.open('POST', '/upload');

        xhr.upload.addEventListener('progress', (e) => {
          if (e.lengthComputable) {
            const percentComplete = (e.loaded / e.total) * 100;
            progress.value = percentComplete;
          }
        });

        xhr.addEventListener('load', () => {
          alert('File uploaded successfully!');
          progress.value = 0;
          location.reload(); // Reload the page to update the file list
        });

        xhr.send(formData);
      });
    </script>
  `);
});

// Handle file upload
app.post('/upload', upload.single('file'), (req, res) => {
  if (req.file) {
    console.log(`Uploading file: ${req.file.originalname}`);
    console.log(`File size: ${req.file.size} bytes`);
    res.send(`File uploaded successfully: ${req.file.filename}`);
  } else {
    console.log('No file uploaded.');
    res.status(400).send('No file uploaded.');
  }
});

// Handle file download
app.get('/download/:filename', (req, res) => {
  const filePath = path.join(__dirname, 'uploads', req.params.filename);
  if (fs.existsSync(filePath)) {
    res.download(filePath);
  } else {
    res.status(404).send('File not found');
  }
});

// Start the server
app.listen(PORT, () => {
  const localIp = getLocalIp();
  console.log(`Server is running on http://${localIp}:${PORT}`);
  console.log(`Or access via http://localhost:${PORT}`);
});
