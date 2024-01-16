document.getElementById('file-upload').addEventListener('change', function (e) {
    // Get the selected file name
    var fileName = e.target.files[0].name;

    // Update the file name element
    document.getElementById('file-name').textContent = fileName;
});

var form = document.querySelector('form');
var overlay = document.getElementById('overlay');
var progressBar = document.querySelector('.progress-bar');


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
            progressBar.value =100;
            document.querySelector('.progress-bar-fill').style.width = 100 + '%';
            document.getElementById('progress-text').textContent = 100 + '%';
            isUploading = false;
            setTimeout(function () {
                if ('//' == directory){
                    window.location.href = '/';
                }else {
                    window.location.href = '/?dir=' + encodeURI(directory);
                }
            }, 500);
        } else {
            // If the upload failed, display an error message
            alert('Upload failed: ' + xhr.statusText);
            overlay.style.display = 'none';
            isUploading = false;
        }
    });

    // Send the request
    xhr.send(formData);
    isUploading = true;
});