document.getElementById('file-upload').addEventListener('change', function (e) {
    // Get the selected file name
    var fileName = e.target.files[0].name;

    // Update the file name element
    document.getElementById('file-name').textContent = fileName;
});

var form = document.querySelector('form');
var overlay = document.getElementById('overlay');
var progressBar = document.querySelector('.progress-bar');

var uploadProgressInterval;

// Set isUploading to true
var isUploading = false;

window.onbeforeunload = function () {
    if (isUploading) {
        return '文件正在上传，如果你离开或刷新页面，上传将会被取消。你确定要离开吗？';
    }
};

form.addEventListener('submit', function (e) {
    e.preventDefault();

    // Show the overlay
    overlay.style.display = 'block';

    // Create a new FormData object
    var formData = new FormData(form);
    var uuid = generateUUID();
    formData.append("uuid", uuid);
    var directory = document.getElementById('directory').value;
    formData.append("path", directory);

    // Create a new XMLHttpRequest object for file upload
    var xhr = new XMLHttpRequest();

    // Configure the request
    xhr.open('POST', '/upload', true);
    xhr.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

    // Handle the load event
    xhr.addEventListener('load', function (e) {
        if (xhr.status == 200) {
            // If the upload was successful, wait for the progress to reach 100 before redirecting
            waitForProgressCompletion();
            isUploading = false;
        } else {
            // If the upload failed, display an error message
            alert('Upload failed: ' + xhr.statusText);
            clearInterval(uploadProgressInterval);
            overlay.style.display = 'none';
            isUploading = false;
        }
    });

    // Send the request
    xhr.send(formData);
    isUploading = true;

    // Start the interval to get the upload progress
    uploadProgressInterval = setInterval(function () {
        // Create a new XMLHttpRequest object for progress query
        var progressXhr = new XMLHttpRequest();

        // Set the withCredentials flag to true to send session cookies
        progressXhr.withCredentials = true;

        // Configure the request
        progressXhr.open('GET', '/upload-progress/' + uuid, true);

        // Handle the load event
        progressXhr.addEventListener('load', function (e) {
            if (progressXhr.status == 200) {
                // Update the progress bar

                var progress = JSON.parse(progressXhr.responseText);

                // Change the color of the text when the progress reaches 50%
                if (progress >= 50) {
                    document.getElementById('progress-text').style.color = '#fff';
                } else {
                    document.getElementById('progress-text').style.color = '#4CAF50';
                }

                progressBar.value = JSON.parse(progressXhr.responseText);
                document.querySelector('.progress-bar-fill').style.width = progress + '%';
                document.getElementById('progress-text').textContent = progress + '%';

                // Check if the progress has reached 100%
                if (JSON.parse(progressXhr.responseText) === 100) {
                    clearInterval(uploadProgressInterval);
                    setTimeout(function () {
                        console.log(encodeURI(directory));
                        window.location.href = '/upload-result?path=' + encodeURI(directory);
                    }, 500); // Add a small delay before redirecting
                }
            }
        });

        // Send the request
        progressXhr.send();
    }, 100); // Query progress every 1 second (you can adjust this value as needed)
});

function waitForProgressCompletion() {
    setTimeout(function () {
        if (progressBar.value !== 100) {
            waitForProgressCompletion();
        }
    }, 1000); // Check progress every 1 second (you can adjust this value as needed)
}

function generateUUID() {
    // 你可以使用自己的方式生成UUID，这里提供一个简单的示例
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
        var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
    });
}